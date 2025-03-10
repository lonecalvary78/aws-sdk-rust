// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_batch_job_execution::_cancel_batch_job_execution_output::CancelBatchJobExecutionOutputBuilder;

pub use crate::operation::cancel_batch_job_execution::_cancel_batch_job_execution_input::CancelBatchJobExecutionInputBuilder;

impl crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.cancel_batch_job_execution();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CancelBatchJobExecution`.
///
/// <p>Cancels the running of a specific batch job execution.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelBatchJobExecutionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionOutput,
        crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionError,
    > for CancelBatchJobExecutionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionOutput,
            crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CancelBatchJobExecutionFluentBuilder {
    /// Creates a new `CancelBatchJobExecutionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CancelBatchJobExecution as a reference.
    pub fn as_input(&self) -> &crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionInputBuilder {
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
        crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::cancel_batch_job_execution::CancelBatchJobExecution::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::cancel_batch_job_execution::CancelBatchJobExecution::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionOutput,
        crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionError,
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
    /// <p>The unique identifier of the application.</p>
    pub fn application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique identifier of the application.</p>
    pub fn set_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The unique identifier of the application.</p>
    pub fn get_application_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_id()
    }
    /// <p>The unique identifier of the batch job execution.</p>
    pub fn execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.execution_id(input.into());
        self
    }
    /// <p>The unique identifier of the batch job execution.</p>
    pub fn set_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_execution_id(input);
        self
    }
    /// <p>The unique identifier of the batch job execution.</p>
    pub fn get_execution_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_execution_id()
    }
    /// <p>The Amazon Web Services Secrets Manager containing user's credentials for authentication and authorization for Cancel Batch Job Execution operation.</p>
    pub fn auth_secrets_manager_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.auth_secrets_manager_arn(input.into());
        self
    }
    /// <p>The Amazon Web Services Secrets Manager containing user's credentials for authentication and authorization for Cancel Batch Job Execution operation.</p>
    pub fn set_auth_secrets_manager_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_auth_secrets_manager_arn(input);
        self
    }
    /// <p>The Amazon Web Services Secrets Manager containing user's credentials for authentication and authorization for Cancel Batch Job Execution operation.</p>
    pub fn get_auth_secrets_manager_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_auth_secrets_manager_arn()
    }
}
