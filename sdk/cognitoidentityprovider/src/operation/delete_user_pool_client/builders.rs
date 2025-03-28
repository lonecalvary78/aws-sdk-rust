// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_user_pool_client::_delete_user_pool_client_output::DeleteUserPoolClientOutputBuilder;

pub use crate::operation::delete_user_pool_client::_delete_user_pool_client_input::DeleteUserPoolClientInputBuilder;

impl crate::operation::delete_user_pool_client::builders::DeleteUserPoolClientInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_user_pool_client::DeleteUserPoolClientOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_user_pool_client::DeleteUserPoolClientError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_user_pool_client();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteUserPoolClient`.
///
/// <p>Deletes a user pool app client. After you delete an app client, users can no longer sign in to the associated application.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteUserPoolClientFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_user_pool_client::builders::DeleteUserPoolClientInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_user_pool_client::DeleteUserPoolClientOutput,
        crate::operation::delete_user_pool_client::DeleteUserPoolClientError,
    > for DeleteUserPoolClientFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_user_pool_client::DeleteUserPoolClientOutput,
            crate::operation::delete_user_pool_client::DeleteUserPoolClientError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteUserPoolClientFluentBuilder {
    /// Creates a new `DeleteUserPoolClientFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteUserPoolClient as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_user_pool_client::builders::DeleteUserPoolClientInputBuilder {
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
        crate::operation::delete_user_pool_client::DeleteUserPoolClientOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_user_pool_client::DeleteUserPoolClientError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_user_pool_client::DeleteUserPoolClient::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_user_pool_client::DeleteUserPoolClient::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_user_pool_client::DeleteUserPoolClientOutput,
        crate::operation::delete_user_pool_client::DeleteUserPoolClientError,
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
    /// <p>The ID of the user pool where you want to delete the client.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The ID of the user pool where you want to delete the client.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The ID of the user pool where you want to delete the client.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_pool_id()
    }
    /// <p>The ID of the user pool app client that you want to delete.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_id(input.into());
        self
    }
    /// <p>The ID of the user pool app client that you want to delete.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_id(input);
        self
    }
    /// <p>The ID of the user pool app client that you want to delete.</p>
    pub fn get_client_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_id()
    }
}
