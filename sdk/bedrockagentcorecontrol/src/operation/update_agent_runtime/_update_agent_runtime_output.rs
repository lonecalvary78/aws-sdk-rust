// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAgentRuntimeOutput {
    /// <p>The Amazon Resource Name (ARN) of the updated agent runtime.</p>
    pub agent_runtime_arn: ::std::string::String,
    /// <p>The unique identifier of the updated agent runtime.</p>
    pub agent_runtime_id: ::std::string::String,
    /// <p>The workload identity details for the updated agent runtime.</p>
    pub workload_identity_details: ::std::option::Option<crate::types::WorkloadIdentityDetails>,
    /// <p>The version of the updated agent runtime.</p>
    pub agent_runtime_version: ::std::string::String,
    /// <p>The timestamp when the agent runtime was created.</p>
    pub created_at: ::aws_smithy_types::DateTime,
    /// <p>The timestamp when the agent runtime was last updated.</p>
    pub last_updated_at: ::aws_smithy_types::DateTime,
    /// <p>The current status of the updated agent runtime.</p>
    pub status: crate::types::AgentStatus,
    _request_id: Option<String>,
}
impl UpdateAgentRuntimeOutput {
    /// <p>The Amazon Resource Name (ARN) of the updated agent runtime.</p>
    pub fn agent_runtime_arn(&self) -> &str {
        use std::ops::Deref;
        self.agent_runtime_arn.deref()
    }
    /// <p>The unique identifier of the updated agent runtime.</p>
    pub fn agent_runtime_id(&self) -> &str {
        use std::ops::Deref;
        self.agent_runtime_id.deref()
    }
    /// <p>The workload identity details for the updated agent runtime.</p>
    pub fn workload_identity_details(&self) -> ::std::option::Option<&crate::types::WorkloadIdentityDetails> {
        self.workload_identity_details.as_ref()
    }
    /// <p>The version of the updated agent runtime.</p>
    pub fn agent_runtime_version(&self) -> &str {
        use std::ops::Deref;
        self.agent_runtime_version.deref()
    }
    /// <p>The timestamp when the agent runtime was created.</p>
    pub fn created_at(&self) -> &::aws_smithy_types::DateTime {
        &self.created_at
    }
    /// <p>The timestamp when the agent runtime was last updated.</p>
    pub fn last_updated_at(&self) -> &::aws_smithy_types::DateTime {
        &self.last_updated_at
    }
    /// <p>The current status of the updated agent runtime.</p>
    pub fn status(&self) -> &crate::types::AgentStatus {
        &self.status
    }
}
impl ::aws_types::request_id::RequestId for UpdateAgentRuntimeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateAgentRuntimeOutput {
    /// Creates a new builder-style object to manufacture [`UpdateAgentRuntimeOutput`](crate::operation::update_agent_runtime::UpdateAgentRuntimeOutput).
    pub fn builder() -> crate::operation::update_agent_runtime::builders::UpdateAgentRuntimeOutputBuilder {
        crate::operation::update_agent_runtime::builders::UpdateAgentRuntimeOutputBuilder::default()
    }
}

/// A builder for [`UpdateAgentRuntimeOutput`](crate::operation::update_agent_runtime::UpdateAgentRuntimeOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateAgentRuntimeOutputBuilder {
    pub(crate) agent_runtime_arn: ::std::option::Option<::std::string::String>,
    pub(crate) agent_runtime_id: ::std::option::Option<::std::string::String>,
    pub(crate) workload_identity_details: ::std::option::Option<crate::types::WorkloadIdentityDetails>,
    pub(crate) agent_runtime_version: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) status: ::std::option::Option<crate::types::AgentStatus>,
    _request_id: Option<String>,
}
impl UpdateAgentRuntimeOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the updated agent runtime.</p>
    /// This field is required.
    pub fn agent_runtime_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_runtime_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the updated agent runtime.</p>
    pub fn set_agent_runtime_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_runtime_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the updated agent runtime.</p>
    pub fn get_agent_runtime_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.agent_runtime_arn
    }
    /// <p>The unique identifier of the updated agent runtime.</p>
    /// This field is required.
    pub fn agent_runtime_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_runtime_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the updated agent runtime.</p>
    pub fn set_agent_runtime_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_runtime_id = input;
        self
    }
    /// <p>The unique identifier of the updated agent runtime.</p>
    pub fn get_agent_runtime_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.agent_runtime_id
    }
    /// <p>The workload identity details for the updated agent runtime.</p>
    pub fn workload_identity_details(mut self, input: crate::types::WorkloadIdentityDetails) -> Self {
        self.workload_identity_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>The workload identity details for the updated agent runtime.</p>
    pub fn set_workload_identity_details(mut self, input: ::std::option::Option<crate::types::WorkloadIdentityDetails>) -> Self {
        self.workload_identity_details = input;
        self
    }
    /// <p>The workload identity details for the updated agent runtime.</p>
    pub fn get_workload_identity_details(&self) -> &::std::option::Option<crate::types::WorkloadIdentityDetails> {
        &self.workload_identity_details
    }
    /// <p>The version of the updated agent runtime.</p>
    /// This field is required.
    pub fn agent_runtime_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.agent_runtime_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the updated agent runtime.</p>
    pub fn set_agent_runtime_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.agent_runtime_version = input;
        self
    }
    /// <p>The version of the updated agent runtime.</p>
    pub fn get_agent_runtime_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.agent_runtime_version
    }
    /// <p>The timestamp when the agent runtime was created.</p>
    /// This field is required.
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the agent runtime was created.</p>
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The timestamp when the agent runtime was created.</p>
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// <p>The timestamp when the agent runtime was last updated.</p>
    /// This field is required.
    pub fn last_updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the agent runtime was last updated.</p>
    pub fn set_last_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_updated_at = input;
        self
    }
    /// <p>The timestamp when the agent runtime was last updated.</p>
    pub fn get_last_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_updated_at
    }
    /// <p>The current status of the updated agent runtime.</p>
    /// This field is required.
    pub fn status(mut self, input: crate::types::AgentStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the updated agent runtime.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::AgentStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The current status of the updated agent runtime.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::AgentStatus> {
        &self.status
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateAgentRuntimeOutput`](crate::operation::update_agent_runtime::UpdateAgentRuntimeOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_runtime_arn`](crate::operation::update_agent_runtime::builders::UpdateAgentRuntimeOutputBuilder::agent_runtime_arn)
    /// - [`agent_runtime_id`](crate::operation::update_agent_runtime::builders::UpdateAgentRuntimeOutputBuilder::agent_runtime_id)
    /// - [`agent_runtime_version`](crate::operation::update_agent_runtime::builders::UpdateAgentRuntimeOutputBuilder::agent_runtime_version)
    /// - [`created_at`](crate::operation::update_agent_runtime::builders::UpdateAgentRuntimeOutputBuilder::created_at)
    /// - [`last_updated_at`](crate::operation::update_agent_runtime::builders::UpdateAgentRuntimeOutputBuilder::last_updated_at)
    /// - [`status`](crate::operation::update_agent_runtime::builders::UpdateAgentRuntimeOutputBuilder::status)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_agent_runtime::UpdateAgentRuntimeOutput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::update_agent_runtime::UpdateAgentRuntimeOutput {
            agent_runtime_arn: self.agent_runtime_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_runtime_arn",
                    "agent_runtime_arn was not specified but it is required when building UpdateAgentRuntimeOutput",
                )
            })?,
            agent_runtime_id: self.agent_runtime_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_runtime_id",
                    "agent_runtime_id was not specified but it is required when building UpdateAgentRuntimeOutput",
                )
            })?,
            workload_identity_details: self.workload_identity_details,
            agent_runtime_version: self.agent_runtime_version.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "agent_runtime_version",
                    "agent_runtime_version was not specified but it is required when building UpdateAgentRuntimeOutput",
                )
            })?,
            created_at: self.created_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "created_at",
                    "created_at was not specified but it is required when building UpdateAgentRuntimeOutput",
                )
            })?,
            last_updated_at: self.last_updated_at.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "last_updated_at",
                    "last_updated_at was not specified but it is required when building UpdateAgentRuntimeOutput",
                )
            })?,
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building UpdateAgentRuntimeOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
