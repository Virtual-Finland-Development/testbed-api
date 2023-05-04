# @see: https://www.pulumi.com/blog/lambda-urls-launch/

import importlib.util
import json

import pulumi
import pulumi_aws as aws
import pulumi_aws_native as aws_native
from pulumi_command import local

# Import utils.py by force
# @see: https://github.com/pulumi/pulumi/issues/1641
# @see: https://github.com/pulumi/pulumi/issues/7360
spec = importlib.util.spec_from_file_location("utils", "utils.py")
utils = importlib.util.module_from_spec(spec)  # type: ignore
spec.loader.exec_module(utils)  # type: ignore

name = "testbed-api"
stage = pulumi.get_stack()
organization = pulumi.get_organization()

tags = {
    "vfd:stack": stage,
    "vfd:project": pulumi.get_project(),
}

#
# External API references
#
authenticationGWLambdaEndpoint = pulumi.StackReference(
    f"{organization}/authentication-gw/{stage}"
).get_output("endpoint")
usersApiLambdaEndpoint = pulumi.StackReference(
    f"{organization}/users-api/{stage}"
).get_output("ApplicationUrl")
tmtProductizerLambdaEndpoint = pulumi.StackReference(
    f"{organization}/tmt-productizer/dev"
).get_output("ApplicationUrl")
jobsInFinlandProductizerLambdaEndpoint = pulumi.StackReference(
    f"{organization}/jobs-in-finland-productizer/dev"
).get_output("ApplicationUrl")
codesetsBaseUrl = pulumi.StackReference(f"{organization}/codesets/{stage}").get_output(
    "url"
)

#
# Lambda function
#
testbed_api_lambda_role = aws_native.iam.Role(
    f"{name}-lambda-role-{stage}",
    assume_role_policy_document=json.dumps(
        {
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Action": "sts:AssumeRole",
                    "Principal": {
                        "Service": "lambda.amazonaws.com",
                    },
                    "Effect": "Allow",
                    "Sid": "",
                },
            ],
        }
    ),
)

testbed_api_lambda_role_attachment = aws.iam.RolePolicyAttachment(
    f"{name}-lambda-role-{stage}",
    role=pulumi.Output.concat(testbed_api_lambda_role.role_name),  # type: ignore
    policy_arn="arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
)

testbed_api_function = aws.lambda_.Function(
    f"{name}-{stage}",
    runtime="provided.al2",  # amazonlinux2
    role=testbed_api_lambda_role.arn,
    handler="bootstrap",  # contents of the zip file
    code=pulumi.FileArchive("./build/rust.zip"),
    timeout=30,
    memory_size=512,
    tags=tags,
    environment=aws.lambda_.FunctionEnvironmentArgs(
        variables={
            "LOGGING_LEVEL": "info",
            **utils.get_dotenv_configuration(stage),  # type: ignore
            "AUTHENTICATION_GW_ENDPOINT_ORIGIN": authenticationGWLambdaEndpoint,
            "USERS_API_ENDPOINT_ORIGIN": usersApiLambdaEndpoint,
            "TMT_PRODUCTIZER_ENDPOINT_ORIGIN": tmtProductizerLambdaEndpoint,
            "JOBS_IN_FINLAND_PRODUCTIZER_ENDPOINT_ORIGIN": jobsInFinlandProductizerLambdaEndpoint,
            "CODESETS_BASE_URL": codesetsBaseUrl,
            "STAGE": stage,
        }
    ),
)

#
# Function URL
#
lambda_url = aws_native.lambda_.Url(
    f"{name}-function-url-{stage}",
    target_function_arn=testbed_api_function.arn,
    auth_type=aws_native.lambda_.UrlAuthType.NONE,
)

add_permissions = local.Command(
    f"{name}-add-permission-{stage}",
    create=pulumi.Output.concat(
        "aws lambda add-permission --function-name ",
        testbed_api_function.name,
        " --action lambda:InvokeFunctionUrl --principal '*' --function-url-auth-type NONE --statement-id FunctionURLAllowPublicAccess",
    ),
    opts=pulumi.ResourceOptions(delete_before_replace=True),
)

pulumi.export("url", lambda_url.function_url)
pulumi.export("LambdaId", testbed_api_function.name)
