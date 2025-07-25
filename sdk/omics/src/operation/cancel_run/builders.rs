// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_run::_cancel_run_output::CancelRunOutputBuilder;

pub use crate::operation::cancel_run::_cancel_run_input::CancelRunInputBuilder;

impl crate::operation::cancel_run::builders::CancelRunInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::cancel_run::CancelRunOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_run::CancelRunError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.cancel_run();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CancelRun`.
///
/// <p>Cancels a run using its ID and returns a response with no body if the operation is successful. To confirm that the run has been cancelled, use the <code>ListRuns</code> API operation to check that it is no longer listed.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelRunFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::cancel_run::builders::CancelRunInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::cancel_run::CancelRunOutput, crate::operation::cancel_run::CancelRunError>
    for CancelRunFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::cancel_run::CancelRunOutput, crate::operation::cancel_run::CancelRunError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CancelRunFluentBuilder {
    /// Creates a new `CancelRunFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CancelRun as a reference.
    pub fn as_input(&self) -> &crate::operation::cancel_run::builders::CancelRunInputBuilder {
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
        crate::operation::cancel_run::CancelRunOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_run::CancelRunError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::cancel_run::CancelRun::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::cancel_run::CancelRun::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::cancel_run::CancelRunOutput,
        crate::operation::cancel_run::CancelRunError,
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
    /// <p>The run's ID.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The run's ID.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The run's ID.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
}
