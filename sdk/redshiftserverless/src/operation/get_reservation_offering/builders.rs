// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_reservation_offering::_get_reservation_offering_output::GetReservationOfferingOutputBuilder;

pub use crate::operation::get_reservation_offering::_get_reservation_offering_input::GetReservationOfferingInputBuilder;

impl crate::operation::get_reservation_offering::builders::GetReservationOfferingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_reservation_offering::GetReservationOfferingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_reservation_offering::GetReservationOfferingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_reservation_offering();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetReservationOffering`.
///
/// <p>Returns the reservation offering. The offering determines the payment schedule for the reservation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetReservationOfferingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_reservation_offering::builders::GetReservationOfferingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_reservation_offering::GetReservationOfferingOutput,
        crate::operation::get_reservation_offering::GetReservationOfferingError,
    > for GetReservationOfferingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_reservation_offering::GetReservationOfferingOutput,
            crate::operation::get_reservation_offering::GetReservationOfferingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetReservationOfferingFluentBuilder {
    /// Creates a new `GetReservationOfferingFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetReservationOffering as a reference.
    pub fn as_input(&self) -> &crate::operation::get_reservation_offering::builders::GetReservationOfferingInputBuilder {
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
        crate::operation::get_reservation_offering::GetReservationOfferingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_reservation_offering::GetReservationOfferingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_reservation_offering::GetReservationOffering::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_reservation_offering::GetReservationOffering::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_reservation_offering::GetReservationOfferingOutput,
        crate::operation::get_reservation_offering::GetReservationOfferingError,
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
    /// <p>The identifier for the offering..</p>
    pub fn offering_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.offering_id(input.into());
        self
    }
    /// <p>The identifier for the offering..</p>
    pub fn set_offering_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_offering_id(input);
        self
    }
    /// <p>The identifier for the offering..</p>
    pub fn get_offering_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_offering_id()
    }
}
