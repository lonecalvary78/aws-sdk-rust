// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_collaboration_privacy_budgets::_list_collaboration_privacy_budgets_output::ListCollaborationPrivacyBudgetsOutputBuilder;

pub use crate::operation::list_collaboration_privacy_budgets::_list_collaboration_privacy_budgets_input::ListCollaborationPrivacyBudgetsInputBuilder;

impl crate::operation::list_collaboration_privacy_budgets::builders::ListCollaborationPrivacyBudgetsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_collaboration_privacy_budgets();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListCollaborationPrivacyBudgets`.
///
/// <p>Returns an array that summarizes each privacy budget in a specified collaboration. The summary includes the collaboration ARN, creation time, creating account, and privacy budget details.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListCollaborationPrivacyBudgetsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_collaboration_privacy_budgets::builders::ListCollaborationPrivacyBudgetsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgetsOutput,
        crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgetsError,
    > for ListCollaborationPrivacyBudgetsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgetsOutput,
            crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgetsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListCollaborationPrivacyBudgetsFluentBuilder {
    /// Creates a new `ListCollaborationPrivacyBudgetsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListCollaborationPrivacyBudgets as a reference.
    pub fn as_input(&self) -> &crate::operation::list_collaboration_privacy_budgets::builders::ListCollaborationPrivacyBudgetsInputBuilder {
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
        crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgets::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgets::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgetsOutput,
        crate::operation::list_collaboration_privacy_budgets::ListCollaborationPrivacyBudgetsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_collaboration_privacy_budgets::paginator::ListCollaborationPrivacyBudgetsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_collaboration_privacy_budgets::paginator::ListCollaborationPrivacyBudgetsPaginator {
        crate::operation::list_collaboration_privacy_budgets::paginator::ListCollaborationPrivacyBudgetsPaginator::new(self.handle, self.inner)
    }
    /// <p>A unique identifier for one of your collaborations.</p>
    pub fn collaboration_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.collaboration_identifier(input.into());
        self
    }
    /// <p>A unique identifier for one of your collaborations.</p>
    pub fn set_collaboration_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_collaboration_identifier(input);
        self
    }
    /// <p>A unique identifier for one of your collaborations.</p>
    pub fn get_collaboration_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_collaboration_identifier()
    }
    /// <p>Specifies the type of the privacy budget.</p>
    pub fn privacy_budget_type(mut self, input: crate::types::PrivacyBudgetType) -> Self {
        self.inner = self.inner.privacy_budget_type(input);
        self
    }
    /// <p>Specifies the type of the privacy budget.</p>
    pub fn set_privacy_budget_type(mut self, input: ::std::option::Option<crate::types::PrivacyBudgetType>) -> Self {
        self.inner = self.inner.set_privacy_budget_type(input);
        self
    }
    /// <p>Specifies the type of the privacy budget.</p>
    pub fn get_privacy_budget_type(&self) -> &::std::option::Option<crate::types::PrivacyBudgetType> {
        self.inner.get_privacy_budget_type()
    }
    /// <p>The maximum number of results that are returned for an API request call. The service chooses a default number if you don't set one. The service might return a `nextToken` even if the `maxResults` value has not been met.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results that are returned for an API request call. The service chooses a default number if you don't set one. The service might return a `nextToken` even if the `maxResults` value has not been met.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results that are returned for an API request call. The service chooses a default number if you don't set one. The service might return a `nextToken` even if the `maxResults` value has not been met.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The pagination token that's used to fetch the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token that's used to fetch the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token that's used to fetch the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
