// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_system_versions::_list_system_versions_output::ListSystemVersionsOutputBuilder;

pub use crate::operation::list_system_versions::_list_system_versions_input::ListSystemVersionsInputBuilder;

impl crate::operation::list_system_versions::builders::ListSystemVersionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_system_versions::ListSystemVersionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_system_versions::ListSystemVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_system_versions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListSystemVersions`.
///
/// <p>Returns information about the system versions that are available for a VM cluster for the specified <code>giVersion</code> and <code>shape</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListSystemVersionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_system_versions::builders::ListSystemVersionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_system_versions::ListSystemVersionsOutput,
        crate::operation::list_system_versions::ListSystemVersionsError,
    > for ListSystemVersionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_system_versions::ListSystemVersionsOutput,
            crate::operation::list_system_versions::ListSystemVersionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListSystemVersionsFluentBuilder {
    /// Creates a new `ListSystemVersionsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListSystemVersions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_system_versions::builders::ListSystemVersionsInputBuilder {
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
        crate::operation::list_system_versions::ListSystemVersionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_system_versions::ListSystemVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_system_versions::ListSystemVersions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_system_versions::ListSystemVersions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_system_versions::ListSystemVersionsOutput,
        crate::operation::list_system_versions::ListSystemVersionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_system_versions::paginator::ListSystemVersionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_system_versions::paginator::ListSystemVersionsPaginator {
        crate::operation::list_system_versions::paginator::ListSystemVersionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output.</p>
    /// <p>Default: <code>10</code></p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output.</p>
    /// <p>Default: <code>10</code></p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output.</p>
    /// <p>Default: <code>10</code></p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The software version of the Exadata Grid Infrastructure (GI).</p>
    pub fn gi_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.gi_version(input.into());
        self
    }
    /// <p>The software version of the Exadata Grid Infrastructure (GI).</p>
    pub fn set_gi_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_gi_version(input);
        self
    }
    /// <p>The software version of the Exadata Grid Infrastructure (GI).</p>
    pub fn get_gi_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_gi_version()
    }
    /// <p>The Exadata hardware system model.</p>
    pub fn shape(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.shape(input.into());
        self
    }
    /// <p>The Exadata hardware system model.</p>
    pub fn set_shape(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_shape(input);
        self
    }
    /// <p>The Exadata hardware system model.</p>
    pub fn get_shape(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_shape()
    }
}
