// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_capacity_reservation_by_splitting::_create_capacity_reservation_by_splitting_output::CreateCapacityReservationBySplittingOutputBuilder;

pub use crate::operation::create_capacity_reservation_by_splitting::_create_capacity_reservation_by_splitting_input::CreateCapacityReservationBySplittingInputBuilder;

impl crate::operation::create_capacity_reservation_by_splitting::builders::CreateCapacityReservationBySplittingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplittingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplittingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_capacity_reservation_by_splitting();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateCapacityReservationBySplitting`.
///
/// <p>Create a new Capacity Reservation by splitting the capacity of the source Capacity Reservation. The new Capacity Reservation will have the same attributes as the source Capacity Reservation except for tags. The source Capacity Reservation must be <code>active</code> and owned by your Amazon Web Services account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCapacityReservationBySplittingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_capacity_reservation_by_splitting::builders::CreateCapacityReservationBySplittingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplittingOutput,
        crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplittingError,
    > for CreateCapacityReservationBySplittingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplittingOutput,
            crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplittingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateCapacityReservationBySplittingFluentBuilder {
    /// Creates a new `CreateCapacityReservationBySplittingFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateCapacityReservationBySplitting as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::create_capacity_reservation_by_splitting::builders::CreateCapacityReservationBySplittingInputBuilder {
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
        crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplittingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplittingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplitting::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplitting::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplittingOutput,
        crate::operation::create_capacity_reservation_by_splitting::CreateCapacityReservationBySplittingError,
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The ID of the Capacity Reservation from which you want to split the capacity.</p>
    pub fn source_capacity_reservation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_capacity_reservation_id(input.into());
        self
    }
    /// <p>The ID of the Capacity Reservation from which you want to split the capacity.</p>
    pub fn set_source_capacity_reservation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_capacity_reservation_id(input);
        self
    }
    /// <p>The ID of the Capacity Reservation from which you want to split the capacity.</p>
    pub fn get_source_capacity_reservation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_capacity_reservation_id()
    }
    /// <p>The number of instances to split from the source Capacity Reservation.</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.inner = self.inner.instance_count(input);
        self
    }
    /// <p>The number of instances to split from the source Capacity Reservation.</p>
    pub fn set_instance_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_instance_count(input);
        self
    }
    /// <p>The number of instances to split from the source Capacity Reservation.</p>
    pub fn get_instance_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_instance_count()
    }
    ///
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the new Capacity Reservation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the new Capacity Reservation.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the new Capacity Reservation.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
    }
}
