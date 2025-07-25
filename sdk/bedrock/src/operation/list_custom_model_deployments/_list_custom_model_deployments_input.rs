// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCustomModelDeploymentsInput {
    /// <p>Filters deployments created before the specified date and time.</p>
    pub created_before: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Filters deployments created after the specified date and time.</p>
    pub created_after: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Filters deployments whose names contain the specified string.</p>
    pub name_contains: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return in a single call.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token for the next set of results. Use this token to retrieve additional results when the response is truncated.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The field to sort the results by. The only supported value is <code>CreationTime</code>.</p>
    pub sort_by: ::std::option::Option<crate::types::SortModelsBy>,
    /// <p>The sort order for the results. Valid values are <code>Ascending</code> and <code>Descending</code>. Default is <code>Descending</code>.</p>
    pub sort_order: ::std::option::Option<crate::types::SortOrder>,
    /// <p>Filters deployments by status. Valid values are <code>CREATING</code>, <code>ACTIVE</code>, and <code>FAILED</code>.</p>
    pub status_equals: ::std::option::Option<crate::types::CustomModelDeploymentStatus>,
    /// <p>Filters deployments by the Amazon Resource Name (ARN) of the associated custom model.</p>
    pub model_arn_equals: ::std::option::Option<::std::string::String>,
}
impl ListCustomModelDeploymentsInput {
    /// <p>Filters deployments created before the specified date and time.</p>
    pub fn created_before(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_before.as_ref()
    }
    /// <p>Filters deployments created after the specified date and time.</p>
    pub fn created_after(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_after.as_ref()
    }
    /// <p>Filters deployments whose names contain the specified string.</p>
    pub fn name_contains(&self) -> ::std::option::Option<&str> {
        self.name_contains.as_deref()
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next set of results. Use this token to retrieve additional results when the response is truncated.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The field to sort the results by. The only supported value is <code>CreationTime</code>.</p>
    pub fn sort_by(&self) -> ::std::option::Option<&crate::types::SortModelsBy> {
        self.sort_by.as_ref()
    }
    /// <p>The sort order for the results. Valid values are <code>Ascending</code> and <code>Descending</code>. Default is <code>Descending</code>.</p>
    pub fn sort_order(&self) -> ::std::option::Option<&crate::types::SortOrder> {
        self.sort_order.as_ref()
    }
    /// <p>Filters deployments by status. Valid values are <code>CREATING</code>, <code>ACTIVE</code>, and <code>FAILED</code>.</p>
    pub fn status_equals(&self) -> ::std::option::Option<&crate::types::CustomModelDeploymentStatus> {
        self.status_equals.as_ref()
    }
    /// <p>Filters deployments by the Amazon Resource Name (ARN) of the associated custom model.</p>
    pub fn model_arn_equals(&self) -> ::std::option::Option<&str> {
        self.model_arn_equals.as_deref()
    }
}
impl ListCustomModelDeploymentsInput {
    /// Creates a new builder-style object to manufacture [`ListCustomModelDeploymentsInput`](crate::operation::list_custom_model_deployments::ListCustomModelDeploymentsInput).
    pub fn builder() -> crate::operation::list_custom_model_deployments::builders::ListCustomModelDeploymentsInputBuilder {
        crate::operation::list_custom_model_deployments::builders::ListCustomModelDeploymentsInputBuilder::default()
    }
}

/// A builder for [`ListCustomModelDeploymentsInput`](crate::operation::list_custom_model_deployments::ListCustomModelDeploymentsInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListCustomModelDeploymentsInputBuilder {
    pub(crate) created_before: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) created_after: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) name_contains: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) sort_by: ::std::option::Option<crate::types::SortModelsBy>,
    pub(crate) sort_order: ::std::option::Option<crate::types::SortOrder>,
    pub(crate) status_equals: ::std::option::Option<crate::types::CustomModelDeploymentStatus>,
    pub(crate) model_arn_equals: ::std::option::Option<::std::string::String>,
}
impl ListCustomModelDeploymentsInputBuilder {
    /// <p>Filters deployments created before the specified date and time.</p>
    pub fn created_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_before = ::std::option::Option::Some(input);
        self
    }
    /// <p>Filters deployments created before the specified date and time.</p>
    pub fn set_created_before(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_before = input;
        self
    }
    /// <p>Filters deployments created before the specified date and time.</p>
    pub fn get_created_before(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_before
    }
    /// <p>Filters deployments created after the specified date and time.</p>
    pub fn created_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_after = ::std::option::Option::Some(input);
        self
    }
    /// <p>Filters deployments created after the specified date and time.</p>
    pub fn set_created_after(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_after = input;
        self
    }
    /// <p>Filters deployments created after the specified date and time.</p>
    pub fn get_created_after(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_after
    }
    /// <p>Filters deployments whose names contain the specified string.</p>
    pub fn name_contains(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name_contains = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Filters deployments whose names contain the specified string.</p>
    pub fn set_name_contains(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name_contains = input;
        self
    }
    /// <p>Filters deployments whose names contain the specified string.</p>
    pub fn get_name_contains(&self) -> &::std::option::Option<::std::string::String> {
        &self.name_contains
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The token for the next set of results. Use this token to retrieve additional results when the response is truncated.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of results. Use this token to retrieve additional results when the response is truncated.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token for the next set of results. Use this token to retrieve additional results when the response is truncated.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The field to sort the results by. The only supported value is <code>CreationTime</code>.</p>
    pub fn sort_by(mut self, input: crate::types::SortModelsBy) -> Self {
        self.sort_by = ::std::option::Option::Some(input);
        self
    }
    /// <p>The field to sort the results by. The only supported value is <code>CreationTime</code>.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::SortModelsBy>) -> Self {
        self.sort_by = input;
        self
    }
    /// <p>The field to sort the results by. The only supported value is <code>CreationTime</code>.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<crate::types::SortModelsBy> {
        &self.sort_by
    }
    /// <p>The sort order for the results. Valid values are <code>Ascending</code> and <code>Descending</code>. Default is <code>Descending</code>.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.sort_order = ::std::option::Option::Some(input);
        self
    }
    /// <p>The sort order for the results. Valid values are <code>Ascending</code> and <code>Descending</code>. Default is <code>Descending</code>.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.sort_order = input;
        self
    }
    /// <p>The sort order for the results. Valid values are <code>Ascending</code> and <code>Descending</code>. Default is <code>Descending</code>.</p>
    pub fn get_sort_order(&self) -> &::std::option::Option<crate::types::SortOrder> {
        &self.sort_order
    }
    /// <p>Filters deployments by status. Valid values are <code>CREATING</code>, <code>ACTIVE</code>, and <code>FAILED</code>.</p>
    pub fn status_equals(mut self, input: crate::types::CustomModelDeploymentStatus) -> Self {
        self.status_equals = ::std::option::Option::Some(input);
        self
    }
    /// <p>Filters deployments by status. Valid values are <code>CREATING</code>, <code>ACTIVE</code>, and <code>FAILED</code>.</p>
    pub fn set_status_equals(mut self, input: ::std::option::Option<crate::types::CustomModelDeploymentStatus>) -> Self {
        self.status_equals = input;
        self
    }
    /// <p>Filters deployments by status. Valid values are <code>CREATING</code>, <code>ACTIVE</code>, and <code>FAILED</code>.</p>
    pub fn get_status_equals(&self) -> &::std::option::Option<crate::types::CustomModelDeploymentStatus> {
        &self.status_equals
    }
    /// <p>Filters deployments by the Amazon Resource Name (ARN) of the associated custom model.</p>
    pub fn model_arn_equals(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_arn_equals = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Filters deployments by the Amazon Resource Name (ARN) of the associated custom model.</p>
    pub fn set_model_arn_equals(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_arn_equals = input;
        self
    }
    /// <p>Filters deployments by the Amazon Resource Name (ARN) of the associated custom model.</p>
    pub fn get_model_arn_equals(&self) -> &::std::option::Option<::std::string::String> {
        &self.model_arn_equals
    }
    /// Consumes the builder and constructs a [`ListCustomModelDeploymentsInput`](crate::operation::list_custom_model_deployments::ListCustomModelDeploymentsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_custom_model_deployments::ListCustomModelDeploymentsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_custom_model_deployments::ListCustomModelDeploymentsInput {
            created_before: self.created_before,
            created_after: self.created_after,
            name_contains: self.name_contains,
            max_results: self.max_results,
            next_token: self.next_token,
            sort_by: self.sort_by,
            sort_order: self.sort_order,
            status_equals: self.status_equals,
            model_arn_equals: self.model_arn_equals,
        })
    }
}
