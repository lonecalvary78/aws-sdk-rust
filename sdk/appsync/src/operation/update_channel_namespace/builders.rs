// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_channel_namespace::_update_channel_namespace_output::UpdateChannelNamespaceOutputBuilder;

pub use crate::operation::update_channel_namespace::_update_channel_namespace_input::UpdateChannelNamespaceInputBuilder;

impl crate::operation::update_channel_namespace::builders::UpdateChannelNamespaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_channel_namespace::UpdateChannelNamespaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_channel_namespace::UpdateChannelNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_channel_namespace();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateChannelNamespace`.
///
/// <p>Updates a <code>ChannelNamespace</code> associated with an <code>Api</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateChannelNamespaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_channel_namespace::builders::UpdateChannelNamespaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_channel_namespace::UpdateChannelNamespaceOutput,
        crate::operation::update_channel_namespace::UpdateChannelNamespaceError,
    > for UpdateChannelNamespaceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_channel_namespace::UpdateChannelNamespaceOutput,
            crate::operation::update_channel_namespace::UpdateChannelNamespaceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateChannelNamespaceFluentBuilder {
    /// Creates a new `UpdateChannelNamespaceFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateChannelNamespace as a reference.
    pub fn as_input(&self) -> &crate::operation::update_channel_namespace::builders::UpdateChannelNamespaceInputBuilder {
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
        crate::operation::update_channel_namespace::UpdateChannelNamespaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_channel_namespace::UpdateChannelNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_channel_namespace::UpdateChannelNamespace::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_channel_namespace::UpdateChannelNamespace::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_channel_namespace::UpdateChannelNamespaceOutput,
        crate::operation::update_channel_namespace::UpdateChannelNamespaceError,
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
    /// <p>The <code>Api</code> ID.</p>
    pub fn api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.api_id(input.into());
        self
    }
    /// <p>The <code>Api</code> ID.</p>
    pub fn set_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_api_id(input);
        self
    }
    /// <p>The <code>Api</code> ID.</p>
    pub fn get_api_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_api_id()
    }
    /// <p>The name of the <code>ChannelNamespace</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the <code>ChannelNamespace</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the <code>ChannelNamespace</code>.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    ///
    /// Appends an item to `subscribeAuthModes`.
    ///
    /// To override the contents of this collection use [`set_subscribe_auth_modes`](Self::set_subscribe_auth_modes).
    ///
    /// <p>The authorization mode to use for subscribing to messages on the channel namespace. This configuration overrides the default <code>Api</code> authorization configuration.</p>
    pub fn subscribe_auth_modes(mut self, input: crate::types::AuthMode) -> Self {
        self.inner = self.inner.subscribe_auth_modes(input);
        self
    }
    /// <p>The authorization mode to use for subscribing to messages on the channel namespace. This configuration overrides the default <code>Api</code> authorization configuration.</p>
    pub fn set_subscribe_auth_modes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AuthMode>>) -> Self {
        self.inner = self.inner.set_subscribe_auth_modes(input);
        self
    }
    /// <p>The authorization mode to use for subscribing to messages on the channel namespace. This configuration overrides the default <code>Api</code> authorization configuration.</p>
    pub fn get_subscribe_auth_modes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AuthMode>> {
        self.inner.get_subscribe_auth_modes()
    }
    ///
    /// Appends an item to `publishAuthModes`.
    ///
    /// To override the contents of this collection use [`set_publish_auth_modes`](Self::set_publish_auth_modes).
    ///
    /// <p>The authorization mode to use for publishing messages on the channel namespace. This configuration overrides the default <code>Api</code> authorization configuration.</p>
    pub fn publish_auth_modes(mut self, input: crate::types::AuthMode) -> Self {
        self.inner = self.inner.publish_auth_modes(input);
        self
    }
    /// <p>The authorization mode to use for publishing messages on the channel namespace. This configuration overrides the default <code>Api</code> authorization configuration.</p>
    pub fn set_publish_auth_modes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AuthMode>>) -> Self {
        self.inner = self.inner.set_publish_auth_modes(input);
        self
    }
    /// <p>The authorization mode to use for publishing messages on the channel namespace. This configuration overrides the default <code>Api</code> authorization configuration.</p>
    pub fn get_publish_auth_modes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AuthMode>> {
        self.inner.get_publish_auth_modes()
    }
    /// <p>The event handler functions that run custom business logic to process published events and subscribe requests.</p>
    pub fn code_handlers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.code_handlers(input.into());
        self
    }
    /// <p>The event handler functions that run custom business logic to process published events and subscribe requests.</p>
    pub fn set_code_handlers(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_code_handlers(input);
        self
    }
    /// <p>The event handler functions that run custom business logic to process published events and subscribe requests.</p>
    pub fn get_code_handlers(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_code_handlers()
    }
    /// <p>The configuration for the <code>OnPublish</code> and <code>OnSubscribe</code> handlers.</p>
    pub fn handler_configs(mut self, input: crate::types::HandlerConfigs) -> Self {
        self.inner = self.inner.handler_configs(input);
        self
    }
    /// <p>The configuration for the <code>OnPublish</code> and <code>OnSubscribe</code> handlers.</p>
    pub fn set_handler_configs(mut self, input: ::std::option::Option<crate::types::HandlerConfigs>) -> Self {
        self.inner = self.inner.set_handler_configs(input);
        self
    }
    /// <p>The configuration for the <code>OnPublish</code> and <code>OnSubscribe</code> handlers.</p>
    pub fn get_handler_configs(&self) -> &::std::option::Option<crate::types::HandlerConfigs> {
        self.inner.get_handler_configs()
    }
}
