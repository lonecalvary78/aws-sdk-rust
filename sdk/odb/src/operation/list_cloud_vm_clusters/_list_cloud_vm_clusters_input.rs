// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCloudVmClustersInput {
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output.</p>
    /// <p>Default: <code>10</code></p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the Oracle Exadata infrastructure.</p>
    pub cloud_exadata_infrastructure_id: ::std::option::Option<::std::string::String>,
}
impl ListCloudVmClustersInput {
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output.</p>
    /// <p>Default: <code>10</code></p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The unique identifier of the Oracle Exadata infrastructure.</p>
    pub fn cloud_exadata_infrastructure_id(&self) -> ::std::option::Option<&str> {
        self.cloud_exadata_infrastructure_id.as_deref()
    }
}
impl ListCloudVmClustersInput {
    /// Creates a new builder-style object to manufacture [`ListCloudVmClustersInput`](crate::operation::list_cloud_vm_clusters::ListCloudVmClustersInput).
    pub fn builder() -> crate::operation::list_cloud_vm_clusters::builders::ListCloudVmClustersInputBuilder {
        crate::operation::list_cloud_vm_clusters::builders::ListCloudVmClustersInputBuilder::default()
    }
}

/// A builder for [`ListCloudVmClustersInput`](crate::operation::list_cloud_vm_clusters::ListCloudVmClustersInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListCloudVmClustersInputBuilder {
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) cloud_exadata_infrastructure_id: ::std::option::Option<::std::string::String>,
}
impl ListCloudVmClustersInputBuilder {
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output.</p>
    /// <p>Default: <code>10</code></p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output.</p>
    /// <p>Default: <code>10</code></p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output.</p>
    /// <p>Default: <code>10</code></p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The unique identifier of the Oracle Exadata infrastructure.</p>
    pub fn cloud_exadata_infrastructure_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cloud_exadata_infrastructure_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the Oracle Exadata infrastructure.</p>
    pub fn set_cloud_exadata_infrastructure_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cloud_exadata_infrastructure_id = input;
        self
    }
    /// <p>The unique identifier of the Oracle Exadata infrastructure.</p>
    pub fn get_cloud_exadata_infrastructure_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.cloud_exadata_infrastructure_id
    }
    /// Consumes the builder and constructs a [`ListCloudVmClustersInput`](crate::operation::list_cloud_vm_clusters::ListCloudVmClustersInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::list_cloud_vm_clusters::ListCloudVmClustersInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::list_cloud_vm_clusters::ListCloudVmClustersInput {
            max_results: self.max_results,
            next_token: self.next_token,
            cloud_exadata_infrastructure_id: self.cloud_exadata_infrastructure_id,
        })
    }
}
