// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `instance_profile_exists` waiter.
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
pub struct InstanceProfileExistsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_instance_profile::builders::GetInstanceProfileInputBuilder,
}
impl InstanceProfileExistsFluentBuilder {
    /// Creates a new `InstanceProfileExistsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetInstanceProfile as a reference.
    pub fn as_input(&self) -> &crate::operation::get_instance_profile::builders::GetInstanceProfileInputBuilder {
        &self.inner
    }
    /// Wait for `instance_profile_exists`
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::instance_profile_exists::InstanceProfileExistsFinalPoll,
        crate::waiters::instance_profile_exists::WaitUntilInstanceProfileExistsError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::get_instance_profile::GetInstanceProfile::operation_runtime_plugins(
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
            &crate::operation::get_instance_profile::GetInstanceProfileOutput,
            &crate::operation::get_instance_profile::GetInstanceProfileError,
        >| {
            // Matches: {"success":true}
            if crate::waiters::matchers::match_get_instance_profile_c955e57777ec0d736(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            // Matches: {"errorType":"NoSuchEntityException"}
            if crate::waiters::matchers::match_get_instance_profile_73f4bff11904ca908(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Retry;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::get_instance_profile::GetInstanceProfile::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(1))
            .max_delay(::std::time::Duration::from_secs(120))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>The name of the instance profile to get information about.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn instance_profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_profile_name(input.into());
        self
    }
    /// <p>The name of the instance profile to get information about.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_instance_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_profile_name(input);
        self
    }
    /// <p>The name of the instance profile to get information about.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn get_instance_profile_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_profile_name()
    }
}

/// Successful return type for the `instance_profile_exists` waiter.
pub type InstanceProfileExistsFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::get_instance_profile::GetInstanceProfileOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::get_instance_profile::GetInstanceProfileError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `instance_profile_exists` waiter.
pub type WaitUntilInstanceProfileExistsError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::get_instance_profile::GetInstanceProfileOutput,
    crate::operation::get_instance_profile::GetInstanceProfileError,
>;
