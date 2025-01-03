// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_invoice_unit::_get_invoice_unit_output::GetInvoiceUnitOutputBuilder;

pub use crate::operation::get_invoice_unit::_get_invoice_unit_input::GetInvoiceUnitInputBuilder;

impl crate::operation::get_invoice_unit::builders::GetInvoiceUnitInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_invoice_unit::GetInvoiceUnitOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_invoice_unit::GetInvoiceUnitError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_invoice_unit();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetInvoiceUnit`.
///
/// <p>This retrieves the invoice unit definition.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetInvoiceUnitFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_invoice_unit::builders::GetInvoiceUnitInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_invoice_unit::GetInvoiceUnitOutput,
        crate::operation::get_invoice_unit::GetInvoiceUnitError,
    > for GetInvoiceUnitFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_invoice_unit::GetInvoiceUnitOutput,
            crate::operation::get_invoice_unit::GetInvoiceUnitError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetInvoiceUnitFluentBuilder {
    /// Creates a new `GetInvoiceUnitFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetInvoiceUnit as a reference.
    pub fn as_input(&self) -> &crate::operation::get_invoice_unit::builders::GetInvoiceUnitInputBuilder {
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
        crate::operation::get_invoice_unit::GetInvoiceUnitOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_invoice_unit::GetInvoiceUnitError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_invoice_unit::GetInvoiceUnit::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_invoice_unit::GetInvoiceUnit::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_invoice_unit::GetInvoiceUnitOutput,
        crate::operation::get_invoice_unit::GetInvoiceUnitError,
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
    /// <p>The ARN to identify an invoice unit. This information can't be modified or deleted.</p>
    pub fn invoice_unit_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.invoice_unit_arn(input.into());
        self
    }
    /// <p>The ARN to identify an invoice unit. This information can't be modified or deleted.</p>
    pub fn set_invoice_unit_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_invoice_unit_arn(input);
        self
    }
    /// <p>The ARN to identify an invoice unit. This information can't be modified or deleted.</p>
    pub fn get_invoice_unit_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_invoice_unit_arn()
    }
    /// <p>The state of an invoice unit at a specified time. You can see legacy invoice units that are currently deleted if the <code>AsOf</code> time is set to before it was deleted. If an <code>AsOf</code> is not provided, the default value is the current time.</p>
    pub fn as_of(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.as_of(input);
        self
    }
    /// <p>The state of an invoice unit at a specified time. You can see legacy invoice units that are currently deleted if the <code>AsOf</code> time is set to before it was deleted. If an <code>AsOf</code> is not provided, the default value is the current time.</p>
    pub fn set_as_of(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_as_of(input);
        self
    }
    /// <p>The state of an invoice unit at a specified time. You can see legacy invoice units that are currently deleted if the <code>AsOf</code> time is set to before it was deleted. If an <code>AsOf</code> is not provided, the default value is the current time.</p>
    pub fn get_as_of(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_as_of()
    }
}
