// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutResourcePolicyInput {
    /// <p>Name of the new policy. This parameter is required.</p>
    pub policy_name: ::std::option::Option<::std::string::String>,
    /// <p>Details of the new policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. This parameter is required.</p>
    /// <p>The following example creates a resource policy enabling the Route 53 service to put DNS query logs in to the specified log group. Replace <code>"logArn"</code> with the ARN of your CloudWatch Logs resource, such as a log group or log stream.</p>
    /// <p>CloudWatch Logs also supports <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourcearn">aws:SourceArn</a> and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourceaccount">aws:SourceAccount</a> condition context keys.</p>
    /// <p>In the example resource policy, you would replace the value of <code>SourceArn</code> with the resource making the call from Route&nbsp;53 to CloudWatch Logs. You would also replace the value of <code>SourceAccount</code> with the Amazon Web Services account ID making that call.</p>
    /// <p></p>
    /// <p><code>{ "Version": "2012-10-17", "Statement": \[ { "Sid": "Route53LogsToCloudWatchLogs", "Effect": "Allow", "Principal": { "Service": \[ "route53.amazonaws.com" \] }, "Action": "logs:PutLogEvents", "Resource": "logArn", "Condition": { "ArnLike": { "aws:SourceArn": "myRoute53ResourceArn" }, "StringEquals": { "aws:SourceAccount": "myAwsAccountId" } } } \] }</code></p>
    pub policy_document: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the CloudWatch Logs resource to which the resource policy needs to be added or attached. Currently only supports LogGroup ARN.</p>
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>The expected revision ID of the resource policy. Required when <code>resourceArn</code> is provided to prevent concurrent modifications. Use <code>null</code> when creating a resource policy for the first time.</p>
    pub expected_revision_id: ::std::option::Option<::std::string::String>,
}
impl PutResourcePolicyInput {
    /// <p>Name of the new policy. This parameter is required.</p>
    pub fn policy_name(&self) -> ::std::option::Option<&str> {
        self.policy_name.as_deref()
    }
    /// <p>Details of the new policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. This parameter is required.</p>
    /// <p>The following example creates a resource policy enabling the Route 53 service to put DNS query logs in to the specified log group. Replace <code>"logArn"</code> with the ARN of your CloudWatch Logs resource, such as a log group or log stream.</p>
    /// <p>CloudWatch Logs also supports <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourcearn">aws:SourceArn</a> and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourceaccount">aws:SourceAccount</a> condition context keys.</p>
    /// <p>In the example resource policy, you would replace the value of <code>SourceArn</code> with the resource making the call from Route&nbsp;53 to CloudWatch Logs. You would also replace the value of <code>SourceAccount</code> with the Amazon Web Services account ID making that call.</p>
    /// <p></p>
    /// <p><code>{ "Version": "2012-10-17", "Statement": \[ { "Sid": "Route53LogsToCloudWatchLogs", "Effect": "Allow", "Principal": { "Service": \[ "route53.amazonaws.com" \] }, "Action": "logs:PutLogEvents", "Resource": "logArn", "Condition": { "ArnLike": { "aws:SourceArn": "myRoute53ResourceArn" }, "StringEquals": { "aws:SourceAccount": "myAwsAccountId" } } } \] }</code></p>
    pub fn policy_document(&self) -> ::std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>The ARN of the CloudWatch Logs resource to which the resource policy needs to be added or attached. Currently only supports LogGroup ARN.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>The expected revision ID of the resource policy. Required when <code>resourceArn</code> is provided to prevent concurrent modifications. Use <code>null</code> when creating a resource policy for the first time.</p>
    pub fn expected_revision_id(&self) -> ::std::option::Option<&str> {
        self.expected_revision_id.as_deref()
    }
}
impl PutResourcePolicyInput {
    /// Creates a new builder-style object to manufacture [`PutResourcePolicyInput`](crate::operation::put_resource_policy::PutResourcePolicyInput).
    pub fn builder() -> crate::operation::put_resource_policy::builders::PutResourcePolicyInputBuilder {
        crate::operation::put_resource_policy::builders::PutResourcePolicyInputBuilder::default()
    }
}

/// A builder for [`PutResourcePolicyInput`](crate::operation::put_resource_policy::PutResourcePolicyInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutResourcePolicyInputBuilder {
    pub(crate) policy_name: ::std::option::Option<::std::string::String>,
    pub(crate) policy_document: ::std::option::Option<::std::string::String>,
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) expected_revision_id: ::std::option::Option<::std::string::String>,
}
impl PutResourcePolicyInputBuilder {
    /// <p>Name of the new policy. This parameter is required.</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the new policy. This parameter is required.</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// <p>Name of the new policy. This parameter is required.</p>
    pub fn get_policy_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_name
    }
    /// <p>Details of the new policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. This parameter is required.</p>
    /// <p>The following example creates a resource policy enabling the Route 53 service to put DNS query logs in to the specified log group. Replace <code>"logArn"</code> with the ARN of your CloudWatch Logs resource, such as a log group or log stream.</p>
    /// <p>CloudWatch Logs also supports <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourcearn">aws:SourceArn</a> and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourceaccount">aws:SourceAccount</a> condition context keys.</p>
    /// <p>In the example resource policy, you would replace the value of <code>SourceArn</code> with the resource making the call from Route&nbsp;53 to CloudWatch Logs. You would also replace the value of <code>SourceAccount</code> with the Amazon Web Services account ID making that call.</p>
    /// <p></p>
    /// <p><code>{ "Version": "2012-10-17", "Statement": \[ { "Sid": "Route53LogsToCloudWatchLogs", "Effect": "Allow", "Principal": { "Service": \[ "route53.amazonaws.com" \] }, "Action": "logs:PutLogEvents", "Resource": "logArn", "Condition": { "ArnLike": { "aws:SourceArn": "myRoute53ResourceArn" }, "StringEquals": { "aws:SourceAccount": "myAwsAccountId" } } } \] }</code></p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Details of the new policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. This parameter is required.</p>
    /// <p>The following example creates a resource policy enabling the Route 53 service to put DNS query logs in to the specified log group. Replace <code>"logArn"</code> with the ARN of your CloudWatch Logs resource, such as a log group or log stream.</p>
    /// <p>CloudWatch Logs also supports <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourcearn">aws:SourceArn</a> and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourceaccount">aws:SourceAccount</a> condition context keys.</p>
    /// <p>In the example resource policy, you would replace the value of <code>SourceArn</code> with the resource making the call from Route&nbsp;53 to CloudWatch Logs. You would also replace the value of <code>SourceAccount</code> with the Amazon Web Services account ID making that call.</p>
    /// <p></p>
    /// <p><code>{ "Version": "2012-10-17", "Statement": \[ { "Sid": "Route53LogsToCloudWatchLogs", "Effect": "Allow", "Principal": { "Service": \[ "route53.amazonaws.com" \] }, "Action": "logs:PutLogEvents", "Resource": "logArn", "Condition": { "ArnLike": { "aws:SourceArn": "myRoute53ResourceArn" }, "StringEquals": { "aws:SourceAccount": "myAwsAccountId" } } } \] }</code></p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// <p>Details of the new policy, including the identity of the principal that is enabled to put logs to this account. This is formatted as a JSON string. This parameter is required.</p>
    /// <p>The following example creates a resource policy enabling the Route 53 service to put DNS query logs in to the specified log group. Replace <code>"logArn"</code> with the ARN of your CloudWatch Logs resource, such as a log group or log stream.</p>
    /// <p>CloudWatch Logs also supports <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourcearn">aws:SourceArn</a> and <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html#condition-keys-sourceaccount">aws:SourceAccount</a> condition context keys.</p>
    /// <p>In the example resource policy, you would replace the value of <code>SourceArn</code> with the resource making the call from Route&nbsp;53 to CloudWatch Logs. You would also replace the value of <code>SourceAccount</code> with the Amazon Web Services account ID making that call.</p>
    /// <p></p>
    /// <p><code>{ "Version": "2012-10-17", "Statement": \[ { "Sid": "Route53LogsToCloudWatchLogs", "Effect": "Allow", "Principal": { "Service": \[ "route53.amazonaws.com" \] }, "Action": "logs:PutLogEvents", "Resource": "logArn", "Condition": { "ArnLike": { "aws:SourceArn": "myRoute53ResourceArn" }, "StringEquals": { "aws:SourceAccount": "myAwsAccountId" } } } \] }</code></p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_document
    }
    /// <p>The ARN of the CloudWatch Logs resource to which the resource policy needs to be added or attached. Currently only supports LogGroup ARN.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the CloudWatch Logs resource to which the resource policy needs to be added or attached. Currently only supports LogGroup ARN.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>The ARN of the CloudWatch Logs resource to which the resource policy needs to be added or attached. Currently only supports LogGroup ARN.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_arn
    }
    /// <p>The expected revision ID of the resource policy. Required when <code>resourceArn</code> is provided to prevent concurrent modifications. Use <code>null</code> when creating a resource policy for the first time.</p>
    pub fn expected_revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expected_revision_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The expected revision ID of the resource policy. Required when <code>resourceArn</code> is provided to prevent concurrent modifications. Use <code>null</code> when creating a resource policy for the first time.</p>
    pub fn set_expected_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expected_revision_id = input;
        self
    }
    /// <p>The expected revision ID of the resource policy. Required when <code>resourceArn</code> is provided to prevent concurrent modifications. Use <code>null</code> when creating a resource policy for the first time.</p>
    pub fn get_expected_revision_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.expected_revision_id
    }
    /// Consumes the builder and constructs a [`PutResourcePolicyInput`](crate::operation::put_resource_policy::PutResourcePolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::put_resource_policy::PutResourcePolicyInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::put_resource_policy::PutResourcePolicyInput {
            policy_name: self.policy_name,
            policy_document: self.policy_document,
            resource_arn: self.resource_arn,
            expected_revision_id: self.expected_revision_id,
        })
    }
}
