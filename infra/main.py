# @see: https://www.pulumi.com/blog/lambda-urls-launch/

import json

import pulumi
import pulumi_aws as aws
import pulumi_aws_native as aws_native
from pulumi_command import local

testbed_api_lambda_role = aws_native.iam.Role(
    "testbed_api_lambda_role",
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
    "testbed_api_lambda_role_attachment",
    role=pulumi.Output.concat(testbed_api_lambda_role.role_name),
    policy_arn="arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
)

testbed_api_function = aws.lambda_.Function(
    "testbed-api",
    runtime="provided.al2",  # amazonlinux2
    role=testbed_api_lambda_role.arn,
    handler="bootstrap",  # contents of the zip file
    code=pulumi.FileArchive("./build/rust.zip"),
)

lambda_url = aws_native.lambda_.Url(
    "testbed-api", target_function_arn=testbed_api_function.arn, auth_type=aws_native.lambda_.UrlAuthType.NONE
)

add_permissions = local.Command(
    "add_permissions",
    create=pulumi.Output.concat(
        "aws lambda add-permission --function-name ",
        testbed_api_function.name,
        " --action lambda:InvokeFunctionUrl --principal '*' --function-url-auth-type NONE --statement-id FunctionURLAllowPublicAccess",
    ),
    opts=pulumi.ResourceOptions(delete_before_replace=True),
)

pulumi.export("url", lambda_url.function_url)
