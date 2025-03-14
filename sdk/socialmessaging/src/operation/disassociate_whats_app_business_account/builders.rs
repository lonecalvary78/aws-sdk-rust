// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_whats_app_business_account::_disassociate_whats_app_business_account_output::DisassociateWhatsAppBusinessAccountOutputBuilder;

pub use crate::operation::disassociate_whats_app_business_account::_disassociate_whats_app_business_account_input::DisassociateWhatsAppBusinessAccountInputBuilder;

impl crate::operation::disassociate_whats_app_business_account::builders::DisassociateWhatsAppBusinessAccountInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccountOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccountError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_whats_app_business_account();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateWhatsAppBusinessAccount`.
///
/// <p>Disassociate a WhatsApp Business Account (WABA) from your Amazon Web Services account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateWhatsAppBusinessAccountFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_whats_app_business_account::builders::DisassociateWhatsAppBusinessAccountInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccountOutput,
        crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccountError,
    > for DisassociateWhatsAppBusinessAccountFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccountOutput,
            crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccountError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociateWhatsAppBusinessAccountFluentBuilder {
    /// Creates a new `DisassociateWhatsAppBusinessAccountFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateWhatsAppBusinessAccount as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_whats_app_business_account::builders::DisassociateWhatsAppBusinessAccountInputBuilder {
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
        crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccountOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccountError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccount::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccount::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccountOutput,
        crate::operation::disassociate_whats_app_business_account::DisassociateWhatsAppBusinessAccountError,
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
    /// <p>The unique identifier of your WhatsApp Business Account. WABA identifiers are formatted as <code>waba-01234567890123456789012345678901</code>. Use <a href="https://docs.aws.amazon.com/social-messaging/latest/APIReference/API_ListLinkedWhatsAppBusinessAccounts.html">ListLinkedWhatsAppBusinessAccounts</a> to list all WABAs and their details.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The unique identifier of your WhatsApp Business Account. WABA identifiers are formatted as <code>waba-01234567890123456789012345678901</code>. Use <a href="https://docs.aws.amazon.com/social-messaging/latest/APIReference/API_ListLinkedWhatsAppBusinessAccounts.html">ListLinkedWhatsAppBusinessAccounts</a> to list all WABAs and their details.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The unique identifier of your WhatsApp Business Account. WABA identifiers are formatted as <code>waba-01234567890123456789012345678901</code>. Use <a href="https://docs.aws.amazon.com/social-messaging/latest/APIReference/API_ListLinkedWhatsAppBusinessAccounts.html">ListLinkedWhatsAppBusinessAccounts</a> to list all WABAs and their details.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
}
