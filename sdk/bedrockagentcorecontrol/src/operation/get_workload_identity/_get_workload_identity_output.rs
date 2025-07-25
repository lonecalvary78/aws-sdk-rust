// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetWorkloadIdentityOutput {
    /// <p>The name of the workload identity.</p>
    pub name: ::std::string::String,
    /// <p>The Amazon Resource Name (ARN) of the workload identity.</p>
    pub workload_identity_arn: ::std::string::String,
    /// <p>The list of allowed OAuth2 return URLs for resources associated with this workload identity.</p>
    pub allowed_resource_oauth2_return_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The timestamp when the workload identity was created.</p>
    pub created_time: ::aws_smithy_types::DateTime,
    /// <p>The timestamp when the workload identity was last updated.</p>
    pub last_updated_time: ::aws_smithy_types::DateTime,
    _request_id: Option<String>,
}
impl GetWorkloadIdentityOutput {
    /// <p>The name of the workload identity.</p>
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the workload identity.</p>
    pub fn workload_identity_arn(&self) -> &str {
        use std::ops::Deref;
        self.workload_identity_arn.deref()
    }
    /// <p>The list of allowed OAuth2 return URLs for resources associated with this workload identity.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.allowed_resource_oauth2_return_urls.is_none()`.
    pub fn allowed_resource_oauth2_return_urls(&self) -> &[::std::string::String] {
        self.allowed_resource_oauth2_return_urls.as_deref().unwrap_or_default()
    }
    /// <p>The timestamp when the workload identity was created.</p>
    pub fn created_time(&self) -> &::aws_smithy_types::DateTime {
        &self.created_time
    }
    /// <p>The timestamp when the workload identity was last updated.</p>
    pub fn last_updated_time(&self) -> &::aws_smithy_types::DateTime {
        &self.last_updated_time
    }
}
impl ::aws_types::request_id::RequestId for GetWorkloadIdentityOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetWorkloadIdentityOutput {
    /// Creates a new builder-style object to manufacture [`GetWorkloadIdentityOutput`](crate::operation::get_workload_identity::GetWorkloadIdentityOutput).
    pub fn builder() -> crate::operation::get_workload_identity::builders::GetWorkloadIdentityOutputBuilder {
        crate::operation::get_workload_identity::builders::GetWorkloadIdentityOutputBuilder::default()
    }
}

/// A builder for [`GetWorkloadIdentityOutput`](crate::operation::get_workload_identity::GetWorkloadIdentityOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetWorkloadIdentityOutputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) workload_identity_arn: ::std::option::Option<::std::string::String>,
    pub(crate) allowed_resource_oauth2_return_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetWorkloadIdentityOutputBuilder {
    /// <p>The name of the workload identity.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the workload identity.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the workload identity.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The Amazon Resource Name (ARN) of the workload identity.</p>
    /// This field is required.
    pub fn workload_identity_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workload_identity_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the workload identity.</p>
    pub fn set_workload_identity_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workload_identity_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the workload identity.</p>
    pub fn get_workload_identity_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.workload_identity_arn
    }
    /// Appends an item to `allowed_resource_oauth2_return_urls`.
    ///
    /// To override the contents of this collection use [`set_allowed_resource_oauth2_return_urls`](Self::set_allowed_resource_oauth2_return_urls).
    ///
    /// <p>The list of allowed OAuth2 return URLs for resources associated with this workload identity.</p>
    pub fn allowed_resource_oauth2_return_urls(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.allowed_resource_oauth2_return_urls.unwrap_or_default();
        v.push(input.into());
        self.allowed_resource_oauth2_return_urls = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of allowed OAuth2 return URLs for resources associated with this workload identity.</p>
    pub fn set_allowed_resource_oauth2_return_urls(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.allowed_resource_oauth2_return_urls = input;
        self
    }
    /// <p>The list of allowed OAuth2 return URLs for resources associated with this workload identity.</p>
    pub fn get_allowed_resource_oauth2_return_urls(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.allowed_resource_oauth2_return_urls
    }
    /// <p>The timestamp when the workload identity was created.</p>
    /// This field is required.
    pub fn created_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the workload identity was created.</p>
    pub fn set_created_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_time = input;
        self
    }
    /// <p>The timestamp when the workload identity was created.</p>
    pub fn get_created_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_time
    }
    /// <p>The timestamp when the workload identity was last updated.</p>
    /// This field is required.
    pub fn last_updated_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the workload identity was last updated.</p>
    pub fn set_last_updated_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_updated_time = input;
        self
    }
    /// <p>The timestamp when the workload identity was last updated.</p>
    pub fn get_last_updated_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_updated_time
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetWorkloadIdentityOutput`](crate::operation::get_workload_identity::GetWorkloadIdentityOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](crate::operation::get_workload_identity::builders::GetWorkloadIdentityOutputBuilder::name)
    /// - [`workload_identity_arn`](crate::operation::get_workload_identity::builders::GetWorkloadIdentityOutputBuilder::workload_identity_arn)
    /// - [`created_time`](crate::operation::get_workload_identity::builders::GetWorkloadIdentityOutputBuilder::created_time)
    /// - [`last_updated_time`](crate::operation::get_workload_identity::builders::GetWorkloadIdentityOutputBuilder::last_updated_time)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_workload_identity::GetWorkloadIdentityOutput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::get_workload_identity::GetWorkloadIdentityOutput {
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building GetWorkloadIdentityOutput",
                )
            })?,
            workload_identity_arn: self.workload_identity_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "workload_identity_arn",
                    "workload_identity_arn was not specified but it is required when building GetWorkloadIdentityOutput",
                )
            })?,
            allowed_resource_oauth2_return_urls: self.allowed_resource_oauth2_return_urls,
            created_time: self.created_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "created_time",
                    "created_time was not specified but it is required when building GetWorkloadIdentityOutput",
                )
            })?,
            last_updated_time: self.last_updated_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "last_updated_time",
                    "last_updated_time was not specified but it is required when building GetWorkloadIdentityOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
