// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_test_configurations::_list_test_configurations_output::ListTestConfigurationsOutputBuilder;

pub use crate::operation::list_test_configurations::_list_test_configurations_input::ListTestConfigurationsInputBuilder;

impl crate::operation::list_test_configurations::builders::ListTestConfigurationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_test_configurations::ListTestConfigurationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_test_configurations::ListTestConfigurationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_test_configurations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListTestConfigurations`.
///
/// <p>Lists test configurations.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTestConfigurationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_test_configurations::builders::ListTestConfigurationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_test_configurations::ListTestConfigurationsOutput,
        crate::operation::list_test_configurations::ListTestConfigurationsError,
    > for ListTestConfigurationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_test_configurations::ListTestConfigurationsOutput,
            crate::operation::list_test_configurations::ListTestConfigurationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListTestConfigurationsFluentBuilder {
    /// Creates a new `ListTestConfigurationsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListTestConfigurations as a reference.
    pub fn as_input(&self) -> &crate::operation::list_test_configurations::builders::ListTestConfigurationsInputBuilder {
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
        crate::operation::list_test_configurations::ListTestConfigurationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_test_configurations::ListTestConfigurationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_test_configurations::ListTestConfigurations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_test_configurations::ListTestConfigurations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_test_configurations::ListTestConfigurationsOutput,
        crate::operation::list_test_configurations::ListTestConfigurationsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_test_configurations::paginator::ListTestConfigurationsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_test_configurations::paginator::ListTestConfigurationsPaginator {
        crate::operation::list_test_configurations::paginator::ListTestConfigurationsPaginator::new(self.handle, self.inner)
    }
    ///
    /// Appends an item to `testConfigurationIds`.
    ///
    /// To override the contents of this collection use [`set_test_configuration_ids`](Self::set_test_configuration_ids).
    ///
    /// <p>The configuration IDs of the test configurations.</p>
    pub fn test_configuration_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.test_configuration_ids(input.into());
        self
    }
    /// <p>The configuration IDs of the test configurations.</p>
    pub fn set_test_configuration_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_test_configuration_ids(input);
        self
    }
    /// <p>The configuration IDs of the test configurations.</p>
    pub fn get_test_configuration_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_test_configuration_ids()
    }
    /// <p>The next token for the test configurations.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The next token for the test configurations.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The next token for the test configurations.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum results of the test configuration.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum results of the test configuration.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum results of the test configuration.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
