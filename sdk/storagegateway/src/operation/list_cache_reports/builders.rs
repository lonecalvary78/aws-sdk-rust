// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_cache_reports::_list_cache_reports_output::ListCacheReportsOutputBuilder;

pub use crate::operation::list_cache_reports::_list_cache_reports_input::ListCacheReportsInputBuilder;

impl crate::operation::list_cache_reports::builders::ListCacheReportsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_cache_reports::ListCacheReportsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_cache_reports::ListCacheReportsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_cache_reports();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListCacheReports`.
///
/// <p>Returns a list of existing cache reports for all file shares associated with your Amazon Web Services account. This list includes all information provided by the <code>DescribeCacheReport</code> action, such as report name, status, completion progress, start time, end time, filters, and tags.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListCacheReportsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_cache_reports::builders::ListCacheReportsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_cache_reports::ListCacheReportsOutput,
        crate::operation::list_cache_reports::ListCacheReportsError,
    > for ListCacheReportsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_cache_reports::ListCacheReportsOutput,
            crate::operation::list_cache_reports::ListCacheReportsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListCacheReportsFluentBuilder {
    /// Creates a new `ListCacheReportsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListCacheReports as a reference.
    pub fn as_input(&self) -> &crate::operation::list_cache_reports::builders::ListCacheReportsInputBuilder {
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
        crate::operation::list_cache_reports::ListCacheReportsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_cache_reports::ListCacheReportsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_cache_reports::ListCacheReports::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_cache_reports::ListCacheReports::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_cache_reports::ListCacheReportsOutput,
        crate::operation::list_cache_reports::ListCacheReportsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_cache_reports::paginator::ListCacheReportsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_cache_reports::paginator::ListCacheReportsPaginator {
        crate::operation::list_cache_reports::paginator::ListCacheReportsPaginator::new(self.handle, self.inner)
    }
    /// <p>Opaque pagination token returned from a previous <code>ListCacheReports</code> operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to <code>ListCacheReports</code>. Optional.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Opaque pagination token returned from a previous <code>ListCacheReports</code> operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to <code>ListCacheReports</code>. Optional.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>Opaque pagination token returned from a previous <code>ListCacheReports</code> operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to <code>ListCacheReports</code>. Optional.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
}
