// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_instance_capacity_reservation_attributes::_modify_instance_capacity_reservation_attributes_output::ModifyInstanceCapacityReservationAttributesOutputBuilder;

pub use crate::operation::modify_instance_capacity_reservation_attributes::_modify_instance_capacity_reservation_attributes_input::ModifyInstanceCapacityReservationAttributesInputBuilder;

impl crate::operation::modify_instance_capacity_reservation_attributes::builders::ModifyInstanceCapacityReservationAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_instance_capacity_reservation_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyInstanceCapacityReservationAttributes`.
///
/// <p>Modifies the Capacity Reservation settings for a stopped instance. Use this action to configure an instance to target a specific Capacity Reservation, run in any <code>open</code> Capacity Reservation with matching attributes, run in On-Demand Instance capacity, or only run in a Capacity Reservation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyInstanceCapacityReservationAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_instance_capacity_reservation_attributes::builders::ModifyInstanceCapacityReservationAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributesOutput,
        crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributesError,
    > for ModifyInstanceCapacityReservationAttributesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributesOutput,
            crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyInstanceCapacityReservationAttributesFluentBuilder {
    /// Creates a new `ModifyInstanceCapacityReservationAttributesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyInstanceCapacityReservationAttributes as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::modify_instance_capacity_reservation_attributes::builders::ModifyInstanceCapacityReservationAttributesInputBuilder {
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
        crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributes::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributes::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributesOutput,
        crate::operation::modify_instance_capacity_reservation_attributes::ModifyInstanceCapacityReservationAttributesError,
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
    /// <p>The ID of the instance to be modified.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the instance to be modified.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The ID of the instance to be modified.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>Information about the Capacity Reservation targeting option.</p>
    pub fn capacity_reservation_specification(mut self, input: crate::types::CapacityReservationSpecification) -> Self {
        self.inner = self.inner.capacity_reservation_specification(input);
        self
    }
    /// <p>Information about the Capacity Reservation targeting option.</p>
    pub fn set_capacity_reservation_specification(mut self, input: ::std::option::Option<crate::types::CapacityReservationSpecification>) -> Self {
        self.inner = self.inner.set_capacity_reservation_specification(input);
        self
    }
    /// <p>Information about the Capacity Reservation targeting option.</p>
    pub fn get_capacity_reservation_specification(&self) -> &::std::option::Option<crate::types::CapacityReservationSpecification> {
        self.inner.get_capacity_reservation_specification()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
