// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_supplemental_tax_registration::_put_supplemental_tax_registration_output::PutSupplementalTaxRegistrationOutputBuilder;

pub use crate::operation::put_supplemental_tax_registration::_put_supplemental_tax_registration_input::PutSupplementalTaxRegistrationInputBuilder;

impl crate::operation::put_supplemental_tax_registration::builders::PutSupplementalTaxRegistrationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistrationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistrationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_supplemental_tax_registration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutSupplementalTaxRegistration`.
///
/// <p>Stores supplemental tax registration for a single account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutSupplementalTaxRegistrationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_supplemental_tax_registration::builders::PutSupplementalTaxRegistrationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistrationOutput,
        crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistrationError,
    > for PutSupplementalTaxRegistrationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistrationOutput,
            crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistrationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutSupplementalTaxRegistrationFluentBuilder {
    /// Creates a new `PutSupplementalTaxRegistrationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutSupplementalTaxRegistration as a reference.
    pub fn as_input(&self) -> &crate::operation::put_supplemental_tax_registration::builders::PutSupplementalTaxRegistrationInputBuilder {
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
        crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistrationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistrationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistrationOutput,
        crate::operation::put_supplemental_tax_registration::PutSupplementalTaxRegistrationError,
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
    /// <p>The supplemental TRN information that will be stored for the caller account ID.</p>
    pub fn tax_registration_entry(mut self, input: crate::types::SupplementalTaxRegistrationEntry) -> Self {
        self.inner = self.inner.tax_registration_entry(input);
        self
    }
    /// <p>The supplemental TRN information that will be stored for the caller account ID.</p>
    pub fn set_tax_registration_entry(mut self, input: ::std::option::Option<crate::types::SupplementalTaxRegistrationEntry>) -> Self {
        self.inner = self.inner.set_tax_registration_entry(input);
        self
    }
    /// <p>The supplemental TRN information that will be stored for the caller account ID.</p>
    pub fn get_tax_registration_entry(&self) -> &::std::option::Option<crate::types::SupplementalTaxRegistrationEntry> {
        self.inner.get_tax_registration_entry()
    }
}
