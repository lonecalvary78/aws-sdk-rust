// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_protect_configuration::_delete_protect_configuration_output::DeleteProtectConfigurationOutputBuilder;

pub use crate::operation::delete_protect_configuration::_delete_protect_configuration_input::DeleteProtectConfigurationInputBuilder;

impl crate::operation::delete_protect_configuration::builders::DeleteProtectConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_protect_configuration::DeleteProtectConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_protect_configuration::DeleteProtectConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_protect_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteProtectConfiguration`.
///
/// <p>Permanently delete the protect configuration. The protect configuration must have deletion protection disabled and must not be associated as the account default protect configuration or associated with a configuration set.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteProtectConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_protect_configuration::builders::DeleteProtectConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_protect_configuration::DeleteProtectConfigurationOutput,
        crate::operation::delete_protect_configuration::DeleteProtectConfigurationError,
    > for DeleteProtectConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_protect_configuration::DeleteProtectConfigurationOutput,
            crate::operation::delete_protect_configuration::DeleteProtectConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteProtectConfigurationFluentBuilder {
    /// Creates a new `DeleteProtectConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteProtectConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_protect_configuration::builders::DeleteProtectConfigurationInputBuilder {
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
        crate::operation::delete_protect_configuration::DeleteProtectConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_protect_configuration::DeleteProtectConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_protect_configuration::DeleteProtectConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_protect_configuration::DeleteProtectConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_protect_configuration::DeleteProtectConfigurationOutput,
        crate::operation::delete_protect_configuration::DeleteProtectConfigurationError,
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
    /// <p>The unique identifier for the protect configuration.</p>
    pub fn protect_configuration_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.protect_configuration_id(input.into());
        self
    }
    /// <p>The unique identifier for the protect configuration.</p>
    pub fn set_protect_configuration_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_protect_configuration_id(input);
        self
    }
    /// <p>The unique identifier for the protect configuration.</p>
    pub fn get_protect_configuration_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_protect_configuration_id()
    }
}
