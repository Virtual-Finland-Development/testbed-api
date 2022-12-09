# @see: https://www.pulumi.com/blog/lambda-urls-launch/

import json

import pulumi
import pulumi_aws as aws
import pulumi_aws_native as aws_native
from pulumi_command import local

name = "testbed-api"
stage = pulumi.get_stack()

tags = {
    "Name": name,
    "Environment": stage,
    "Project": "Virtual Finland",
}

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
    publish=True,  # needed for provisioned concurrency
    timeout=30,
    memory_size=512,
    tags=tags,
)

#
# Scheduled provisioned concurrency setup
#
lambda_id_for_provisioning = pulumi.Output.concat(
    "function:", testbed_api_function.name, ":", testbed_api_function.version
)

provision_autoscaling_target = aws.appautoscaling.Target(
    f"{name}-provisioned-concurrency-target-{stage}",
    resource_id=lambda_id_for_provisioning,
    service_namespace="lambda",
    scalable_dimension="lambda:function:ProvisionedConcurrency",
    min_capacity=1,
    max_capacity=10,
)

aws.appautoscaling.ScheduledAction(
    f"{name}-provisioned-concurrency-by-day-{stage}",
    service_namespace="lambda",
    resource_id=provision_autoscaling_target.resource_id,
    scalable_dimension="lambda:function:ProvisionedConcurrency",
    schedule="cron(0 6 * * ? *)",
    scalable_target_action=aws.appautoscaling.ScheduledActionScalableTargetActionArgs(
        min_capacity=1,
        max_capacity=10,
    ),
)
aws.appautoscaling.ScheduledAction(
    f"{name}-provisioned-concurrency-by-night-{stage}",
    service_namespace="lambda",
    resource_id=provision_autoscaling_target.resource_id,
    scalable_dimension="lambda:function:ProvisionedConcurrency",
    schedule="cron(0 16 * * ? *)",
    scalable_target_action=aws.appautoscaling.ScheduledActionScalableTargetActionArgs(
        min_capacity=0,
        max_capacity=5,
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
