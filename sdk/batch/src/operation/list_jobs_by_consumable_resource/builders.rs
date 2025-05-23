// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_jobs_by_consumable_resource::_list_jobs_by_consumable_resource_output::ListJobsByConsumableResourceOutputBuilder;

pub use crate::operation::list_jobs_by_consumable_resource::_list_jobs_by_consumable_resource_input::ListJobsByConsumableResourceInputBuilder;

impl crate::operation::list_jobs_by_consumable_resource::builders::ListJobsByConsumableResourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_jobs_by_consumable_resource();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListJobsByConsumableResource`.
///
/// <p>Returns a list of Batch jobs that require a specific consumable resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListJobsByConsumableResourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_jobs_by_consumable_resource::builders::ListJobsByConsumableResourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput,
        crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceError,
    > for ListJobsByConsumableResourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput,
            crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListJobsByConsumableResourceFluentBuilder {
    /// Creates a new `ListJobsByConsumableResourceFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListJobsByConsumableResource as a reference.
    pub fn as_input(&self) -> &crate::operation::list_jobs_by_consumable_resource::builders::ListJobsByConsumableResourceInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceOutput,
        crate::operation::list_jobs_by_consumable_resource::ListJobsByConsumableResourceError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_jobs_by_consumable_resource::paginator::ListJobsByConsumableResourcePaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_jobs_by_consumable_resource::paginator::ListJobsByConsumableResourcePaginator {
        crate::operation::list_jobs_by_consumable_resource::paginator::ListJobsByConsumableResourcePaginator::new(self.handle, self.inner)
    }
    /// <p>The name or ARN of the consumable resource.</p>
    pub fn consumable_resource(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.consumable_resource(input.into());
        self
    }
    /// <p>The name or ARN of the consumable resource.</p>
    pub fn set_consumable_resource(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_consumable_resource(input);
        self
    }
    /// <p>The name or ARN of the consumable resource.</p>
    pub fn get_consumable_resource(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_consumable_resource()
    }
    ///
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters to apply to the job list query. If used, only those jobs requiring the specified consumable resource (<code>consumableResource</code>) and that match the value of the filters are listed. The filter names and values can be:</p>
    /// <ul>
    /// <li>
    /// <p>name: <code>JOB_STATUS</code></p>
    /// <p>values: <code>SUBMITTED | PENDING | RUNNABLE | STARTING | RUNNING | SUCCEEDED | FAILED</code></p></li>
    /// <li>
    /// <p>name: <code>JOB_NAME </code></p>
    /// <p>The values are case-insensitive matches for the job name. If a filter value ends with an asterisk (*), it matches any job name that begins with the string before the '*'.</p></li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::KeyValuesPair) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filters to apply to the job list query. If used, only those jobs requiring the specified consumable resource (<code>consumableResource</code>) and that match the value of the filters are listed. The filter names and values can be:</p>
    /// <ul>
    /// <li>
    /// <p>name: <code>JOB_STATUS</code></p>
    /// <p>values: <code>SUBMITTED | PENDING | RUNNABLE | STARTING | RUNNING | SUCCEEDED | FAILED</code></p></li>
    /// <li>
    /// <p>name: <code>JOB_NAME </code></p>
    /// <p>The values are case-insensitive matches for the job name. If a filter value ends with an asterisk (*), it matches any job name that begins with the string before the '*'.</p></li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::KeyValuesPair>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The filters to apply to the job list query. If used, only those jobs requiring the specified consumable resource (<code>consumableResource</code>) and that match the value of the filters are listed. The filter names and values can be:</p>
    /// <ul>
    /// <li>
    /// <p>name: <code>JOB_STATUS</code></p>
    /// <p>values: <code>SUBMITTED | PENDING | RUNNABLE | STARTING | RUNNING | SUCCEEDED | FAILED</code></p></li>
    /// <li>
    /// <p>name: <code>JOB_NAME </code></p>
    /// <p>The values are case-insensitive matches for the job name. If a filter value ends with an asterisk (*), it matches any job name that begins with the string before the '*'.</p></li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::KeyValuesPair>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of results returned by <code>ListJobsByConsumableResource</code> in paginated output. When this parameter is used, <code>ListJobsByConsumableResource</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobsByConsumableResource</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListJobsByConsumableResource</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results returned by <code>ListJobsByConsumableResource</code> in paginated output. When this parameter is used, <code>ListJobsByConsumableResource</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobsByConsumableResource</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListJobsByConsumableResource</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results returned by <code>ListJobsByConsumableResource</code> in paginated output. When this parameter is used, <code>ListJobsByConsumableResource</code> only returns <code>maxResults</code> results in a single page and a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListJobsByConsumableResource</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListJobsByConsumableResource</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobsByConsumableResource</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p><note>
    /// <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobsByConsumableResource</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p><note>
    /// <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>ListJobsByConsumableResource</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is <code>null</code> when there are no more results to return.</p><note>
    /// <p>Treat this token as an opaque identifier that's only used to retrieve the next items in a list and not for other programmatic purposes.</p>
    /// </note>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
