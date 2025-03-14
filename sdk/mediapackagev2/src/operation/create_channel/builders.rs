// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_channel::_create_channel_output::CreateChannelOutputBuilder;

pub use crate::operation::create_channel::_create_channel_input::CreateChannelInputBuilder;

impl crate::operation::create_channel::builders::CreateChannelInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_channel::CreateChannelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_channel::CreateChannelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_channel();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateChannel`.
///
/// <p>Create a channel to start receiving content streams. The channel represents the input to MediaPackage for incoming live content from an encoder such as AWS Elemental MediaLive. The channel receives content, and after packaging it, outputs it through an origin endpoint to downstream devices (such as video players or CDNs) that request the content. You can create only one channel with each request. We recommend that you spread out channels between channel groups, such as putting redundant channels in the same AWS Region in different channel groups.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateChannelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_channel::builders::CreateChannelInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_channel::CreateChannelOutput,
        crate::operation::create_channel::CreateChannelError,
    > for CreateChannelFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_channel::CreateChannelOutput,
            crate::operation::create_channel::CreateChannelError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateChannelFluentBuilder {
    /// Creates a new `CreateChannelFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateChannel as a reference.
    pub fn as_input(&self) -> &crate::operation::create_channel::builders::CreateChannelInputBuilder {
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
        crate::operation::create_channel::CreateChannelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_channel::CreateChannelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_channel::CreateChannel::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_channel::CreateChannel::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_channel::CreateChannelOutput,
        crate::operation::create_channel::CreateChannelError,
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
    /// <p>The name that describes the channel group. The name is the primary identifier for the channel group, and must be unique for your account in the AWS Region.</p>
    pub fn channel_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.channel_group_name(input.into());
        self
    }
    /// <p>The name that describes the channel group. The name is the primary identifier for the channel group, and must be unique for your account in the AWS Region.</p>
    pub fn set_channel_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_channel_group_name(input);
        self
    }
    /// <p>The name that describes the channel group. The name is the primary identifier for the channel group, and must be unique for your account in the AWS Region.</p>
    pub fn get_channel_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_channel_group_name()
    }
    /// <p>The name that describes the channel. The name is the primary identifier for the channel, and must be unique for your account in the AWS Region and channel group. You can't change the name after you create the channel.</p>
    pub fn channel_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.channel_name(input.into());
        self
    }
    /// <p>The name that describes the channel. The name is the primary identifier for the channel, and must be unique for your account in the AWS Region and channel group. You can't change the name after you create the channel.</p>
    pub fn set_channel_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_channel_name(input);
        self
    }
    /// <p>The name that describes the channel. The name is the primary identifier for the channel, and must be unique for your account in the AWS Region and channel group. You can't change the name after you create the channel.</p>
    pub fn get_channel_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_channel_name()
    }
    /// <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The input type will be an immutable field which will be used to define whether the channel will allow CMAF ingest or HLS ingest. If unprovided, it will default to HLS to preserve current behavior.</p>
    /// <p>The allowed values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>HLS</code> - The HLS streaming specification (which defines M3U8 manifests and TS segments).</p></li>
    /// <li>
    /// <p><code>CMAF</code> - The DASH-IF CMAF Ingest specification (which defines CMAF segments with optional DASH manifests).</p></li>
    /// </ul>
    pub fn input_type(mut self, input: crate::types::InputType) -> Self {
        self.inner = self.inner.input_type(input);
        self
    }
    /// <p>The input type will be an immutable field which will be used to define whether the channel will allow CMAF ingest or HLS ingest. If unprovided, it will default to HLS to preserve current behavior.</p>
    /// <p>The allowed values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>HLS</code> - The HLS streaming specification (which defines M3U8 manifests and TS segments).</p></li>
    /// <li>
    /// <p><code>CMAF</code> - The DASH-IF CMAF Ingest specification (which defines CMAF segments with optional DASH manifests).</p></li>
    /// </ul>
    pub fn set_input_type(mut self, input: ::std::option::Option<crate::types::InputType>) -> Self {
        self.inner = self.inner.set_input_type(input);
        self
    }
    /// <p>The input type will be an immutable field which will be used to define whether the channel will allow CMAF ingest or HLS ingest. If unprovided, it will default to HLS to preserve current behavior.</p>
    /// <p>The allowed values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>HLS</code> - The HLS streaming specification (which defines M3U8 manifests and TS segments).</p></li>
    /// <li>
    /// <p><code>CMAF</code> - The DASH-IF CMAF Ingest specification (which defines CMAF segments with optional DASH manifests).</p></li>
    /// </ul>
    pub fn get_input_type(&self) -> &::std::option::Option<crate::types::InputType> {
        self.inner.get_input_type()
    }
    /// <p>Enter any descriptive text that helps you to identify the channel.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Enter any descriptive text that helps you to identify the channel.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Enter any descriptive text that helps you to identify the channel.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The configuration for input switching based on the media quality confidence score (MQCS) as provided from AWS Elemental MediaLive. This setting is valid only when <code>InputType</code> is <code>CMAF</code>.</p>
    pub fn input_switch_configuration(mut self, input: crate::types::InputSwitchConfiguration) -> Self {
        self.inner = self.inner.input_switch_configuration(input);
        self
    }
    /// <p>The configuration for input switching based on the media quality confidence score (MQCS) as provided from AWS Elemental MediaLive. This setting is valid only when <code>InputType</code> is <code>CMAF</code>.</p>
    pub fn set_input_switch_configuration(mut self, input: ::std::option::Option<crate::types::InputSwitchConfiguration>) -> Self {
        self.inner = self.inner.set_input_switch_configuration(input);
        self
    }
    /// <p>The configuration for input switching based on the media quality confidence score (MQCS) as provided from AWS Elemental MediaLive. This setting is valid only when <code>InputType</code> is <code>CMAF</code>.</p>
    pub fn get_input_switch_configuration(&self) -> &::std::option::Option<crate::types::InputSwitchConfiguration> {
        self.inner.get_input_switch_configuration()
    }
    /// <p>The settings for what common media server data (CMSD) headers AWS Elemental MediaPackage includes in responses to the CDN. This setting is valid only when <code>InputType</code> is <code>CMAF</code>.</p>
    pub fn output_header_configuration(mut self, input: crate::types::OutputHeaderConfiguration) -> Self {
        self.inner = self.inner.output_header_configuration(input);
        self
    }
    /// <p>The settings for what common media server data (CMSD) headers AWS Elemental MediaPackage includes in responses to the CDN. This setting is valid only when <code>InputType</code> is <code>CMAF</code>.</p>
    pub fn set_output_header_configuration(mut self, input: ::std::option::Option<crate::types::OutputHeaderConfiguration>) -> Self {
        self.inner = self.inner.set_output_header_configuration(input);
        self
    }
    /// <p>The settings for what common media server data (CMSD) headers AWS Elemental MediaPackage includes in responses to the CDN. This setting is valid only when <code>InputType</code> is <code>CMAF</code>.</p>
    pub fn get_output_header_configuration(&self) -> &::std::option::Option<crate::types::OutputHeaderConfiguration> {
        self.inner.get_output_header_configuration()
    }
    ///
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A comma-separated list of tag key:value pairs that you define. For example:</p>
    /// <p><code>"Key1": "Value1",</code></p>
    /// <p><code>"Key2": "Value2"</code></p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A comma-separated list of tag key:value pairs that you define. For example:</p>
    /// <p><code>"Key1": "Value1",</code></p>
    /// <p><code>"Key2": "Value2"</code></p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A comma-separated list of tag key:value pairs that you define. For example:</p>
    /// <p><code>"Key1": "Value1",</code></p>
    /// <p><code>"Key2": "Value2"</code></p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
