// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Output Mixin
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetClusterOutput {
    /// <p>The ID of the retrieved cluster.</p>
    pub identifier: ::std::string::String,
    /// <p>The ARN of the retrieved cluster.</p>
    pub arn: ::std::string::String,
    /// <p>The status of the retrieved cluster.</p>
    pub status: crate::types::ClusterStatus,
    /// <p>The time of when the cluster was created.</p>
    pub creation_time: ::aws_smithy_types::DateTime,
    /// <p>Whether deletion protection is enabled in this cluster.</p>
    pub deletion_protection_enabled: bool,
    /// <p>The witness Region of the cluster. Applicable only for multi-Region clusters.</p>
    pub witness_region: ::std::option::Option<::std::string::String>,
    /// <p>The ARNs of the clusters linked to the retrieved cluster.</p>
    pub linked_cluster_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl GetClusterOutput {
    /// <p>The ID of the retrieved cluster.</p>
    pub fn identifier(&self) -> &str {
        use std::ops::Deref;
        self.identifier.deref()
    }
    /// <p>The ARN of the retrieved cluster.</p>
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }
    /// <p>The status of the retrieved cluster.</p>
    pub fn status(&self) -> &crate::types::ClusterStatus {
        &self.status
    }
    /// <p>The time of when the cluster was created.</p>
    pub fn creation_time(&self) -> &::aws_smithy_types::DateTime {
        &self.creation_time
    }
    /// <p>Whether deletion protection is enabled in this cluster.</p>
    pub fn deletion_protection_enabled(&self) -> bool {
        self.deletion_protection_enabled
    }
    /// <p>The witness Region of the cluster. Applicable only for multi-Region clusters.</p>
    pub fn witness_region(&self) -> ::std::option::Option<&str> {
        self.witness_region.as_deref()
    }
    /// <p>The ARNs of the clusters linked to the retrieved cluster.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.linked_cluster_arns.is_none()`.
    pub fn linked_cluster_arns(&self) -> &[::std::string::String] {
        self.linked_cluster_arns.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for GetClusterOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetClusterOutput {
    /// Creates a new builder-style object to manufacture [`GetClusterOutput`](crate::operation::get_cluster::GetClusterOutput).
    pub fn builder() -> crate::operation::get_cluster::builders::GetClusterOutputBuilder {
        crate::operation::get_cluster::builders::GetClusterOutputBuilder::default()
    }
}

/// A builder for [`GetClusterOutput`](crate::operation::get_cluster::GetClusterOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetClusterOutputBuilder {
    pub(crate) identifier: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ClusterStatus>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) deletion_protection_enabled: ::std::option::Option<bool>,
    pub(crate) witness_region: ::std::option::Option<::std::string::String>,
    pub(crate) linked_cluster_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl GetClusterOutputBuilder {
    /// <p>The ID of the retrieved cluster.</p>
    /// This field is required.
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the retrieved cluster.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.identifier = input;
        self
    }
    /// <p>The ID of the retrieved cluster.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.identifier
    }
    /// <p>The ARN of the retrieved cluster.</p>
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the retrieved cluster.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The ARN of the retrieved cluster.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The status of the retrieved cluster.</p>
    /// This field is required.
    pub fn status(mut self, input: crate::types::ClusterStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the retrieved cluster.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ClusterStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status of the retrieved cluster.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ClusterStatus> {
        &self.status
    }
    /// <p>The time of when the cluster was created.</p>
    /// This field is required.
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time of when the cluster was created.</p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The time of when the cluster was created.</p>
    pub fn get_creation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_time
    }
    /// <p>Whether deletion protection is enabled in this cluster.</p>
    /// This field is required.
    pub fn deletion_protection_enabled(mut self, input: bool) -> Self {
        self.deletion_protection_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether deletion protection is enabled in this cluster.</p>
    pub fn set_deletion_protection_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.deletion_protection_enabled = input;
        self
    }
    /// <p>Whether deletion protection is enabled in this cluster.</p>
    pub fn get_deletion_protection_enabled(&self) -> &::std::option::Option<bool> {
        &self.deletion_protection_enabled
    }
    /// <p>The witness Region of the cluster. Applicable only for multi-Region clusters.</p>
    pub fn witness_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.witness_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The witness Region of the cluster. Applicable only for multi-Region clusters.</p>
    pub fn set_witness_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.witness_region = input;
        self
    }
    /// <p>The witness Region of the cluster. Applicable only for multi-Region clusters.</p>
    pub fn get_witness_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.witness_region
    }
    /// Appends an item to `linked_cluster_arns`.
    ///
    /// To override the contents of this collection use [`set_linked_cluster_arns`](Self::set_linked_cluster_arns).
    ///
    /// <p>The ARNs of the clusters linked to the retrieved cluster.</p>
    pub fn linked_cluster_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.linked_cluster_arns.unwrap_or_default();
        v.push(input.into());
        self.linked_cluster_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The ARNs of the clusters linked to the retrieved cluster.</p>
    pub fn set_linked_cluster_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.linked_cluster_arns = input;
        self
    }
    /// <p>The ARNs of the clusters linked to the retrieved cluster.</p>
    pub fn get_linked_cluster_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.linked_cluster_arns
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetClusterOutput`](crate::operation::get_cluster::GetClusterOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`identifier`](crate::operation::get_cluster::builders::GetClusterOutputBuilder::identifier)
    /// - [`arn`](crate::operation::get_cluster::builders::GetClusterOutputBuilder::arn)
    /// - [`status`](crate::operation::get_cluster::builders::GetClusterOutputBuilder::status)
    /// - [`creation_time`](crate::operation::get_cluster::builders::GetClusterOutputBuilder::creation_time)
    /// - [`deletion_protection_enabled`](crate::operation::get_cluster::builders::GetClusterOutputBuilder::deletion_protection_enabled)
    pub fn build(self) -> ::std::result::Result<crate::operation::get_cluster::GetClusterOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_cluster::GetClusterOutput {
            identifier: self.identifier.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "identifier",
                    "identifier was not specified but it is required when building GetClusterOutput",
                )
            })?,
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building GetClusterOutput",
                )
            })?,
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building GetClusterOutput",
                )
            })?,
            creation_time: self.creation_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "creation_time",
                    "creation_time was not specified but it is required when building GetClusterOutput",
                )
            })?,
            deletion_protection_enabled: self.deletion_protection_enabled.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "deletion_protection_enabled",
                    "deletion_protection_enabled was not specified but it is required when building GetClusterOutput",
                )
            })?,
            witness_region: self.witness_region,
            linked_cluster_arns: self.linked_cluster_arns,
            _request_id: self._request_id,
        })
    }
}
