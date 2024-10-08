// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `signal_map_created` waiter.
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
pub struct SignalMapCreatedFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_signal_map::builders::GetSignalMapInputBuilder,
}
impl SignalMapCreatedFluentBuilder {
    /// Creates a new `SignalMapCreatedFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetSignalMap as a reference.
    pub fn as_input(&self) -> &crate::operation::get_signal_map::builders::GetSignalMapInputBuilder {
        &self.inner
    }
    /// Wait until a signal map has been created
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::signal_map_created::SignalMapCreatedFinalPoll,
        crate::waiters::signal_map_created::WaitUntilSignalMapCreatedError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::get_signal_map::GetSignalMap::operation_runtime_plugins(
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
            &crate::operation::get_signal_map::GetSignalMapOutput,
            &crate::operation::get_signal_map::GetSignalMapError,
        >| {
            // Matches: {"output":{"path":"Status","expected":"CREATE_COMPLETE","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_signal_map_ee24601e746b68869(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            // Matches: {"output":{"path":"Status","expected":"CREATE_IN_PROGRESS","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_signal_map_fd38fc0610d7a1d7e(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Retry;
            }
            // Matches: {"output":{"path":"Status","expected":"CREATE_FAILED","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_signal_map_22c66a56e806df25e(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::get_signal_map::GetSignalMap::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(5))
            .max_delay(::std::time::Duration::from_secs(120))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// A signal map's identifier. Can be either be its id or current name.
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// A signal map's identifier. Can be either be its id or current name.
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
    /// A signal map's identifier. Can be either be its id or current name.
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }
}

/// Successful return type for the `signal_map_created` waiter.
pub type SignalMapCreatedFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::get_signal_map::GetSignalMapOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::get_signal_map::GetSignalMapError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `signal_map_created` waiter.
pub type WaitUntilSignalMapCreatedError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::get_signal_map::GetSignalMapOutput,
    crate::operation::get_signal_map::GetSignalMapError,
>;
