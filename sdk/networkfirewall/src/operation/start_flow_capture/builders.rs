// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_flow_capture::_start_flow_capture_output::StartFlowCaptureOutputBuilder;

pub use crate::operation::start_flow_capture::_start_flow_capture_input::StartFlowCaptureInputBuilder;

impl crate::operation::start_flow_capture::builders::StartFlowCaptureInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_flow_capture::StartFlowCaptureOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_flow_capture::StartFlowCaptureError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_flow_capture();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartFlowCapture`.
///
/// <p>Begins capturing the flows in a firewall, according to the filters you define. Captures are similar, but not identical to snapshots. Capture operations provide visibility into flows that are not closed and are tracked by a firewall's flow table. Unlike snapshots, captures are a time-boxed view.</p>
/// <p>A flow is network traffic that is monitored by a firewall, either by stateful or stateless rules. For traffic to be considered part of a flow, it must share Destination, DestinationPort, Direction, Protocol, Source, and SourcePort.</p><note>
/// <p>To avoid encountering operation limits, you should avoid starting captures with broad filters, like wide IP ranges. Instead, we recommend you define more specific criteria with <code>FlowFilters</code>, like narrow IP ranges, ports, or protocols.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartFlowCaptureFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_flow_capture::builders::StartFlowCaptureInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_flow_capture::StartFlowCaptureOutput,
        crate::operation::start_flow_capture::StartFlowCaptureError,
    > for StartFlowCaptureFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_flow_capture::StartFlowCaptureOutput,
            crate::operation::start_flow_capture::StartFlowCaptureError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartFlowCaptureFluentBuilder {
    /// Creates a new `StartFlowCaptureFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartFlowCapture as a reference.
    pub fn as_input(&self) -> &crate::operation::start_flow_capture::builders::StartFlowCaptureInputBuilder {
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
        crate::operation::start_flow_capture::StartFlowCaptureOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_flow_capture::StartFlowCaptureError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_flow_capture::StartFlowCapture::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_flow_capture::StartFlowCapture::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_flow_capture::StartFlowCaptureOutput,
        crate::operation::start_flow_capture::StartFlowCaptureError,
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
    /// <p>The Amazon Resource Name (ARN) of the firewall.</p>
    pub fn firewall_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.firewall_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the firewall.</p>
    pub fn set_firewall_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_firewall_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the firewall.</p>
    pub fn get_firewall_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_firewall_arn()
    }
    /// <p>The ID of the Availability Zone where the firewall is located. For example, <code>us-east-2a</code>.</p>
    /// <p>Defines the scope a flow operation. You can use up to 20 filters to configure a single flow operation.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The ID of the Availability Zone where the firewall is located. For example, <code>us-east-2a</code>.</p>
    /// <p>Defines the scope a flow operation. You can use up to 20 filters to configure a single flow operation.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The ID of the Availability Zone where the firewall is located. For example, <code>us-east-2a</code>.</p>
    /// <p>Defines the scope a flow operation. You can use up to 20 filters to configure a single flow operation.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_availability_zone()
    }
    /// <p>The Amazon Resource Name (ARN) of a VPC endpoint association.</p>
    pub fn vpc_endpoint_association_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_endpoint_association_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a VPC endpoint association.</p>
    pub fn set_vpc_endpoint_association_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_endpoint_association_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a VPC endpoint association.</p>
    pub fn get_vpc_endpoint_association_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc_endpoint_association_arn()
    }
    /// <p>A unique identifier for the primary endpoint associated with a firewall.</p>
    pub fn vpc_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_endpoint_id(input.into());
        self
    }
    /// <p>A unique identifier for the primary endpoint associated with a firewall.</p>
    pub fn set_vpc_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_endpoint_id(input);
        self
    }
    /// <p>A unique identifier for the primary endpoint associated with a firewall.</p>
    pub fn get_vpc_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc_endpoint_id()
    }
    /// <p>The reqested <code>FlowOperation</code> ignores flows with an age (in seconds) lower than <code>MinimumFlowAgeInSeconds</code>. You provide this for start commands.</p><note>
    /// <p>We recommend setting this value to at least 1 minute (60 seconds) to reduce chance of capturing flows that are not yet established.</p>
    /// </note>
    pub fn minimum_flow_age_in_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.minimum_flow_age_in_seconds(input);
        self
    }
    /// <p>The reqested <code>FlowOperation</code> ignores flows with an age (in seconds) lower than <code>MinimumFlowAgeInSeconds</code>. You provide this for start commands.</p><note>
    /// <p>We recommend setting this value to at least 1 minute (60 seconds) to reduce chance of capturing flows that are not yet established.</p>
    /// </note>
    pub fn set_minimum_flow_age_in_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_minimum_flow_age_in_seconds(input);
        self
    }
    /// <p>The reqested <code>FlowOperation</code> ignores flows with an age (in seconds) lower than <code>MinimumFlowAgeInSeconds</code>. You provide this for start commands.</p><note>
    /// <p>We recommend setting this value to at least 1 minute (60 seconds) to reduce chance of capturing flows that are not yet established.</p>
    /// </note>
    pub fn get_minimum_flow_age_in_seconds(&self) -> &::std::option::Option<i32> {
        self.inner.get_minimum_flow_age_in_seconds()
    }
    ///
    /// Appends an item to `FlowFilters`.
    ///
    /// To override the contents of this collection use [`set_flow_filters`](Self::set_flow_filters).
    ///
    /// <p>Defines the scope a flow operation. You can use up to 20 filters to configure a single flow operation.</p>
    pub fn flow_filters(mut self, input: crate::types::FlowFilter) -> Self {
        self.inner = self.inner.flow_filters(input);
        self
    }
    /// <p>Defines the scope a flow operation. You can use up to 20 filters to configure a single flow operation.</p>
    pub fn set_flow_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::FlowFilter>>) -> Self {
        self.inner = self.inner.set_flow_filters(input);
        self
    }
    /// <p>Defines the scope a flow operation. You can use up to 20 filters to configure a single flow operation.</p>
    pub fn get_flow_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FlowFilter>> {
        self.inner.get_flow_filters()
    }
}
