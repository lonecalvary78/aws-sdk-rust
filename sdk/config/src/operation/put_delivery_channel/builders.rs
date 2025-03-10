// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_delivery_channel::_put_delivery_channel_output::PutDeliveryChannelOutputBuilder;

pub use crate::operation::put_delivery_channel::_put_delivery_channel_input::PutDeliveryChannelInputBuilder;

impl crate::operation::put_delivery_channel::builders::PutDeliveryChannelInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_delivery_channel::PutDeliveryChannelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_delivery_channel::PutDeliveryChannelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_delivery_channel();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutDeliveryChannel`.
///
/// <p>Creates or updates a delivery channel to deliver configuration information and other compliance information.</p>
/// <p>You can use this operation to create a new delivery channel or to update the Amazon S3 bucket and the Amazon SNS topic of an existing delivery channel.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/manage-delivery-channel.html"> <b>Working with the Delivery Channel</b> </a> in the <i>Config Developer Guide.</i></p><note>
/// <p><b>One delivery channel per account per Region</b></p>
/// <p>You can have only one delivery channel for each account for each Amazon Web Services Region.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutDeliveryChannelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_delivery_channel::builders::PutDeliveryChannelInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_delivery_channel::PutDeliveryChannelOutput,
        crate::operation::put_delivery_channel::PutDeliveryChannelError,
    > for PutDeliveryChannelFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_delivery_channel::PutDeliveryChannelOutput,
            crate::operation::put_delivery_channel::PutDeliveryChannelError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutDeliveryChannelFluentBuilder {
    /// Creates a new `PutDeliveryChannelFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutDeliveryChannel as a reference.
    pub fn as_input(&self) -> &crate::operation::put_delivery_channel::builders::PutDeliveryChannelInputBuilder {
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
        crate::operation::put_delivery_channel::PutDeliveryChannelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_delivery_channel::PutDeliveryChannelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_delivery_channel::PutDeliveryChannel::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_delivery_channel::PutDeliveryChannel::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_delivery_channel::PutDeliveryChannelOutput,
        crate::operation::put_delivery_channel::PutDeliveryChannelError,
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
    /// <p>An object for the delivery channel. A delivery channel sends notifications and updated configuration states.</p>
    pub fn delivery_channel(mut self, input: crate::types::DeliveryChannel) -> Self {
        self.inner = self.inner.delivery_channel(input);
        self
    }
    /// <p>An object for the delivery channel. A delivery channel sends notifications and updated configuration states.</p>
    pub fn set_delivery_channel(mut self, input: ::std::option::Option<crate::types::DeliveryChannel>) -> Self {
        self.inner = self.inner.set_delivery_channel(input);
        self
    }
    /// <p>An object for the delivery channel. A delivery channel sends notifications and updated configuration states.</p>
    pub fn get_delivery_channel(&self) -> &::std::option::Option<crate::types::DeliveryChannel> {
        self.inner.get_delivery_channel()
    }
}
