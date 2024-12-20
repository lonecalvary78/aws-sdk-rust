// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_ai_guardrail_versions::_list_ai_guardrail_versions_output::ListAiGuardrailVersionsOutputBuilder;

pub use crate::operation::list_ai_guardrail_versions::_list_ai_guardrail_versions_input::ListAiGuardrailVersionsInputBuilder;

impl crate::operation::list_ai_guardrail_versions::builders::ListAiGuardrailVersionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_ai_guardrail_versions::ListAiGuardrailVersionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_ai_guardrail_versions::ListAIGuardrailVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_ai_guardrail_versions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListAIGuardrailVersions`.
///
/// <p>Lists AI Guardrail versions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAIGuardrailVersionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_ai_guardrail_versions::builders::ListAiGuardrailVersionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_ai_guardrail_versions::ListAiGuardrailVersionsOutput,
        crate::operation::list_ai_guardrail_versions::ListAIGuardrailVersionsError,
    > for ListAIGuardrailVersionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_ai_guardrail_versions::ListAiGuardrailVersionsOutput,
            crate::operation::list_ai_guardrail_versions::ListAIGuardrailVersionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListAIGuardrailVersionsFluentBuilder {
    /// Creates a new `ListAIGuardrailVersionsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListAIGuardrailVersions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_ai_guardrail_versions::builders::ListAiGuardrailVersionsInputBuilder {
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
        crate::operation::list_ai_guardrail_versions::ListAiGuardrailVersionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_ai_guardrail_versions::ListAIGuardrailVersionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_ai_guardrail_versions::ListAIGuardrailVersions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_ai_guardrail_versions::ListAIGuardrailVersions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_ai_guardrail_versions::ListAiGuardrailVersionsOutput,
        crate::operation::list_ai_guardrail_versions::ListAIGuardrailVersionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_ai_guardrail_versions::paginator::ListAiGuardrailVersionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_ai_guardrail_versions::paginator::ListAiGuardrailVersionsPaginator {
        crate::operation::list_ai_guardrail_versions::paginator::ListAiGuardrailVersionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier of the Amazon Q in Connect assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn assistant_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.assistant_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Q in Connect assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn set_assistant_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_assistant_id(input);
        self
    }
    /// <p>The identifier of the Amazon Q in Connect assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn get_assistant_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_assistant_id()
    }
    /// <p>The identifier of the Amazon Q in Connect AI Guardrail for which versions are to be listed.</p>
    pub fn ai_guardrail_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ai_guardrail_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Q in Connect AI Guardrail for which versions are to be listed.</p>
    pub fn set_ai_guardrail_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ai_guardrail_id(input);
        self
    }
    /// <p>The identifier of the Amazon Q in Connect AI Guardrail for which versions are to be listed.</p>
    pub fn get_ai_guardrail_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ai_guardrail_id()
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
