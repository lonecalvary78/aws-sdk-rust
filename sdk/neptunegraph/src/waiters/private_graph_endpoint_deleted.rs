// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `private_graph_endpoint_deleted` waiter.
///
/// This builder is intended to be used similar to the other fluent builders for
/// normal operations on the client. However, instead of a `send` method, it has
/// a `wait` method that takes a maximum amount of time to wait.
///
/// Construct this fluent builder using the client by importing the
/// [`Waiters`](crate::client::Waiters) trait and calling the methods
/// prefixed with `wait_until`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PrivateGraphEndpointDeletedFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_private_graph_endpoint::builders::GetPrivateGraphEndpointInputBuilder,
}
impl PrivateGraphEndpointDeletedFluentBuilder {
    /// Creates a new `PrivateGraphEndpointDeletedFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetPrivateGraphEndpoint as a reference.
    pub fn as_input(&self) -> &crate::operation::get_private_graph_endpoint::builders::GetPrivateGraphEndpointInputBuilder {
        &self.inner
    }
    /// Wait until PrivateGraphEndpoint is Deleted
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::private_graph_endpoint_deleted::PrivateGraphEndpointDeletedFinalPoll,
        crate::waiters::private_graph_endpoint_deleted::WaitUntilPrivateGraphEndpointDeletedError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::get_private_graph_endpoint::GetPrivateGraphEndpoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            ::std::option::Option::None,
        )
        .with_operation_plugin(crate::sdk_feature_tracker::waiter::WaiterFeatureTrackerRuntimePlugin::new());
        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
        let runtime_components_builder = runtime_plugins
            .apply_client_configuration(&mut cfg)
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let time_components = runtime_components_builder.into_time_components();
        let sleep_impl = time_components.sleep_impl().expect("a sleep impl is required by waiters");
        let time_source = time_components.time_source().expect("a time source is required by waiters");

        let acceptor = move |result: ::std::result::Result<
            &crate::operation::get_private_graph_endpoint::GetPrivateGraphEndpointOutput,
            &crate::operation::get_private_graph_endpoint::GetPrivateGraphEndpointError,
        >| {
            // Matches: {"output":{"path":"status != 'DELETING'","expected":"true","comparator":"booleanEquals"}}
            if crate::waiters::matchers::match_get_private_graph_endpoint_36cde75d8dc473dbc(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"errorType":"ResourceNotFoundException"}
            if crate::waiters::matchers::match_get_private_graph_endpoint_1cce2c05524fb92d4(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::get_private_graph_endpoint::GetPrivateGraphEndpoint::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(10))
            .max_delay(::std::time::Duration::from_secs(1800))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>The unique identifier of the Neptune Analytics graph.</p>
    pub fn graph_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.graph_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the Neptune Analytics graph.</p>
    pub fn set_graph_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_graph_identifier(input);
        self
    }
    /// <p>The unique identifier of the Neptune Analytics graph.</p>
    pub fn get_graph_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_graph_identifier()
    }
    /// <p>The ID of the VPC where the private endpoint is located.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_id(input.into());
        self
    }
    /// <p>The ID of the VPC where the private endpoint is located.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_id(input);
        self
    }
    /// <p>The ID of the VPC where the private endpoint is located.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc_id()
    }
}

/// Successful return type for the `private_graph_endpoint_deleted` waiter.
pub type PrivateGraphEndpointDeletedFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::get_private_graph_endpoint::GetPrivateGraphEndpointOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::get_private_graph_endpoint::GetPrivateGraphEndpointError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `private_graph_endpoint_deleted` waiter.
pub type WaitUntilPrivateGraphEndpointDeletedError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::get_private_graph_endpoint::GetPrivateGraphEndpointOutput,
    crate::operation::get_private_graph_endpoint::GetPrivateGraphEndpointError,
>;
