// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_policy_grants::_list_policy_grants_output::ListPolicyGrantsOutputBuilder;

pub use crate::operation::list_policy_grants::_list_policy_grants_input::ListPolicyGrantsInputBuilder;

impl crate::operation::list_policy_grants::builders::ListPolicyGrantsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_policy_grants::ListPolicyGrantsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_policy_grants::ListPolicyGrantsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_policy_grants();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListPolicyGrants`.
///
/// <p>Lists policy grants.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPolicyGrantsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_policy_grants::builders::ListPolicyGrantsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_policy_grants::ListPolicyGrantsOutput,
        crate::operation::list_policy_grants::ListPolicyGrantsError,
    > for ListPolicyGrantsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_policy_grants::ListPolicyGrantsOutput,
            crate::operation::list_policy_grants::ListPolicyGrantsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListPolicyGrantsFluentBuilder {
    /// Creates a new `ListPolicyGrantsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListPolicyGrants as a reference.
    pub fn as_input(&self) -> &crate::operation::list_policy_grants::builders::ListPolicyGrantsInputBuilder {
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
        crate::operation::list_policy_grants::ListPolicyGrantsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_policy_grants::ListPolicyGrantsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_policy_grants::ListPolicyGrants::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_policy_grants::ListPolicyGrants::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_policy_grants::ListPolicyGrantsOutput,
        crate::operation::list_policy_grants::ListPolicyGrantsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_policy_grants::paginator::ListPolicyGrantsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_policy_grants::paginator::ListPolicyGrantsPaginator {
        crate::operation::list_policy_grants::paginator::ListPolicyGrantsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the domain where you want to list policy grants.</p>
    pub fn domain_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_identifier(input.into());
        self
    }
    /// <p>The ID of the domain where you want to list policy grants.</p>
    pub fn set_domain_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_identifier(input);
        self
    }
    /// <p>The ID of the domain where you want to list policy grants.</p>
    pub fn get_domain_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_identifier()
    }
    /// <p>The type of entity for which you want to list policy grants.</p>
    pub fn entity_type(mut self, input: crate::types::TargetEntityType) -> Self {
        self.inner = self.inner.entity_type(input);
        self
    }
    /// <p>The type of entity for which you want to list policy grants.</p>
    pub fn set_entity_type(mut self, input: ::std::option::Option<crate::types::TargetEntityType>) -> Self {
        self.inner = self.inner.set_entity_type(input);
        self
    }
    /// <p>The type of entity for which you want to list policy grants.</p>
    pub fn get_entity_type(&self) -> &::std::option::Option<crate::types::TargetEntityType> {
        self.inner.get_entity_type()
    }
    /// <p>The ID of the entity for which you want to list policy grants.</p>
    pub fn entity_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.entity_identifier(input.into());
        self
    }
    /// <p>The ID of the entity for which you want to list policy grants.</p>
    pub fn set_entity_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_entity_identifier(input);
        self
    }
    /// <p>The ID of the entity for which you want to list policy grants.</p>
    pub fn get_entity_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_entity_identifier()
    }
    /// <p>The type of policy that you want to list.</p>
    pub fn policy_type(mut self, input: crate::types::ManagedPolicyType) -> Self {
        self.inner = self.inner.policy_type(input);
        self
    }
    /// <p>The type of policy that you want to list.</p>
    pub fn set_policy_type(mut self, input: ::std::option::Option<crate::types::ManagedPolicyType>) -> Self {
        self.inner = self.inner.set_policy_type(input);
        self
    }
    /// <p>The type of policy that you want to list.</p>
    pub fn get_policy_type(&self) -> &::std::option::Option<crate::types::ManagedPolicyType> {
        self.inner.get_policy_type()
    }
    /// <p>The maximum number of grants to return in a single call to <code>ListPolicyGrants</code>. When the number of grants to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListPolicyGrants</code> to list the next set of grants.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of grants to return in a single call to <code>ListPolicyGrants</code>. When the number of grants to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListPolicyGrants</code> to list the next set of grants.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of grants to return in a single call to <code>ListPolicyGrants</code>. When the number of grants to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListPolicyGrants</code> to list the next set of grants.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>When the number of grants is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of grants, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListPolicyGrants</code> to list the next set of grants.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>When the number of grants is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of grants, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListPolicyGrants</code> to list the next set of grants.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>When the number of grants is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of grants, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListPolicyGrants</code> to list the next set of grants.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
