// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `image_version_created` waiter.
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
pub struct ImageVersionCreatedFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_image_version::builders::DescribeImageVersionInputBuilder,
}
impl ImageVersionCreatedFluentBuilder {
    /// Creates a new `ImageVersionCreatedFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the DescribeImageVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_image_version::builders::DescribeImageVersionInputBuilder {
        &self.inner
    }
    /// Wait for `image_version_created`
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::image_version_created::ImageVersionCreatedFinalPoll,
        crate::waiters::image_version_created::WaitUntilImageVersionCreatedError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_image_version::DescribeImageVersion::operation_runtime_plugins(
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
            &crate::operation::describe_image_version::DescribeImageVersionOutput,
            &crate::operation::describe_image_version::DescribeImageVersionError,
        >| {
            // Matches: {"output":{"path":"ImageVersionStatus","expected":"CREATED","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_describe_image_version_3e437bf5fdf17cfc6(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            // Matches: {"output":{"path":"ImageVersionStatus","expected":"CREATE_FAILED","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_describe_image_version_6f23d4602c6e621d4(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"errorType":"ValidationException"}
            if crate::waiters::matchers::match_describe_image_version_19c82d26244f04729(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::describe_image_version::DescribeImageVersion::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(60))
            .max_delay(::std::time::Duration::from_secs(3600))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>The name of the image.</p>
    pub fn image_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.image_name(input.into());
        self
    }
    /// <p>The name of the image.</p>
    pub fn set_image_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_image_name(input);
        self
    }
    /// <p>The name of the image.</p>
    pub fn get_image_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_image_name()
    }
    /// <p>The version of the image. If not specified, the latest version is described.</p>
    pub fn version(mut self, input: i32) -> Self {
        self.inner = self.inner.version(input);
        self
    }
    /// <p>The version of the image. If not specified, the latest version is described.</p>
    pub fn set_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_version(input);
        self
    }
    /// <p>The version of the image. If not specified, the latest version is described.</p>
    pub fn get_version(&self) -> &::std::option::Option<i32> {
        self.inner.get_version()
    }
    /// <p>The alias of the image version.</p>
    pub fn alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alias(input.into());
        self
    }
    /// <p>The alias of the image version.</p>
    pub fn set_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alias(input);
        self
    }
    /// <p>The alias of the image version.</p>
    pub fn get_alias(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_alias()
    }
}

/// Successful return type for the `image_version_created` waiter.
pub type ImageVersionCreatedFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::describe_image_version::DescribeImageVersionOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::describe_image_version::DescribeImageVersionError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `image_version_created` waiter.
pub type WaitUntilImageVersionCreatedError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::describe_image_version::DescribeImageVersionOutput,
    crate::operation::describe_image_version::DescribeImageVersionError,
>;
