// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::send_activation_code::_send_activation_code_output::SendActivationCodeOutputBuilder;

pub use crate::operation::send_activation_code::_send_activation_code_input::SendActivationCodeInputBuilder;

impl crate::operation::send_activation_code::builders::SendActivationCodeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::send_activation_code::SendActivationCodeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_activation_code::SendActivationCodeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.send_activation_code();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SendActivationCode`.
///
/// <p>Sends an activation email to the email address associated with the specified email contact.</p><note>
/// <p>It might take a few minutes for the activation email to arrive. If it doesn't arrive, check in your spam folder or try sending another activation email.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SendActivationCodeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::send_activation_code::builders::SendActivationCodeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::send_activation_code::SendActivationCodeOutput,
        crate::operation::send_activation_code::SendActivationCodeError,
    > for SendActivationCodeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::send_activation_code::SendActivationCodeOutput,
            crate::operation::send_activation_code::SendActivationCodeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SendActivationCodeFluentBuilder {
    /// Creates a new `SendActivationCodeFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SendActivationCode as a reference.
    pub fn as_input(&self) -> &crate::operation::send_activation_code::builders::SendActivationCodeInputBuilder {
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
        crate::operation::send_activation_code::SendActivationCodeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_activation_code::SendActivationCodeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::send_activation_code::SendActivationCode::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::send_activation_code::SendActivationCode::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::send_activation_code::SendActivationCodeOutput,
        crate::operation::send_activation_code::SendActivationCodeError,
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
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_arn()
    }
}
