// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateCloudExadataInfrastructureOutput {
    /// <p>The user-friendly name for the Exadata infrastructure.</p>
    pub display_name: ::std::option::Option<::std::string::String>,
    /// <p>The current status of the Exadata infrastructure.</p>
    pub status: ::std::option::Option<crate::types::ResourceStatus>,
    /// <p>Additional information about the status of the Exadata infrastructure.</p>
    pub status_reason: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the Exadata infrastructure.</p>
    pub cloud_exadata_infrastructure_id: ::std::string::String,
    _request_id: Option<String>,
}
impl CreateCloudExadataInfrastructureOutput {
    /// <p>The user-friendly name for the Exadata infrastructure.</p>
    pub fn display_name(&self) -> ::std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>The current status of the Exadata infrastructure.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ResourceStatus> {
        self.status.as_ref()
    }
    /// <p>Additional information about the status of the Exadata infrastructure.</p>
    pub fn status_reason(&self) -> ::std::option::Option<&str> {
        self.status_reason.as_deref()
    }
    /// <p>The unique identifier of the Exadata infrastructure.</p>
    pub fn cloud_exadata_infrastructure_id(&self) -> &str {
        use std::ops::Deref;
        self.cloud_exadata_infrastructure_id.deref()
    }
}
impl ::aws_types::request_id::RequestId for CreateCloudExadataInfrastructureOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateCloudExadataInfrastructureOutput {
    /// Creates a new builder-style object to manufacture [`CreateCloudExadataInfrastructureOutput`](crate::operation::create_cloud_exadata_infrastructure::CreateCloudExadataInfrastructureOutput).
    pub fn builder() -> crate::operation::create_cloud_exadata_infrastructure::builders::CreateCloudExadataInfrastructureOutputBuilder {
        crate::operation::create_cloud_exadata_infrastructure::builders::CreateCloudExadataInfrastructureOutputBuilder::default()
    }
}

/// A builder for [`CreateCloudExadataInfrastructureOutput`](crate::operation::create_cloud_exadata_infrastructure::CreateCloudExadataInfrastructureOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateCloudExadataInfrastructureOutputBuilder {
    pub(crate) display_name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ResourceStatus>,
    pub(crate) status_reason: ::std::option::Option<::std::string::String>,
    pub(crate) cloud_exadata_infrastructure_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateCloudExadataInfrastructureOutputBuilder {
    /// <p>The user-friendly name for the Exadata infrastructure.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user-friendly name for the Exadata infrastructure.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// <p>The user-friendly name for the Exadata infrastructure.</p>
    pub fn get_display_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.display_name
    }
    /// <p>The current status of the Exadata infrastructure.</p>
    pub fn status(mut self, input: crate::types::ResourceStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the Exadata infrastructure.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ResourceStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The current status of the Exadata infrastructure.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ResourceStatus> {
        &self.status
    }
    /// <p>Additional information about the status of the Exadata infrastructure.</p>
    pub fn status_reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Additional information about the status of the Exadata infrastructure.</p>
    pub fn set_status_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_reason = input;
        self
    }
    /// <p>Additional information about the status of the Exadata infrastructure.</p>
    pub fn get_status_reason(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_reason
    }
    /// <p>The unique identifier of the Exadata infrastructure.</p>
    /// This field is required.
    pub fn cloud_exadata_infrastructure_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cloud_exadata_infrastructure_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the Exadata infrastructure.</p>
    pub fn set_cloud_exadata_infrastructure_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cloud_exadata_infrastructure_id = input;
        self
    }
    /// <p>The unique identifier of the Exadata infrastructure.</p>
    pub fn get_cloud_exadata_infrastructure_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.cloud_exadata_infrastructure_id
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateCloudExadataInfrastructureOutput`](crate::operation::create_cloud_exadata_infrastructure::CreateCloudExadataInfrastructureOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`cloud_exadata_infrastructure_id`](crate::operation::create_cloud_exadata_infrastructure::builders::CreateCloudExadataInfrastructureOutputBuilder::cloud_exadata_infrastructure_id)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_cloud_exadata_infrastructure::CreateCloudExadataInfrastructureOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_cloud_exadata_infrastructure::CreateCloudExadataInfrastructureOutput {
                display_name: self.display_name,
                status: self.status,
                status_reason: self.status_reason,
                cloud_exadata_infrastructure_id: self.cloud_exadata_infrastructure_id.ok_or_else(|| {
                    ::aws_smithy_types::error::operation::BuildError::missing_field(
                        "cloud_exadata_infrastructure_id",
                        "cloud_exadata_infrastructure_id was not specified but it is required when building CreateCloudExadataInfrastructureOutput",
                    )
                })?,
                _request_id: self._request_id,
            },
        )
    }
}
