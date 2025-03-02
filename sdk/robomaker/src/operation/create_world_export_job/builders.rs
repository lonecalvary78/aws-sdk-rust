// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_world_export_job::_create_world_export_job_output::CreateWorldExportJobOutputBuilder;

pub use crate::operation::create_world_export_job::_create_world_export_job_input::CreateWorldExportJobInputBuilder;

impl crate::operation::create_world_export_job::builders::CreateWorldExportJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_world_export_job::CreateWorldExportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_world_export_job::CreateWorldExportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_world_export_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateWorldExportJob`.
///
/// <important>
/// <p>End of support notice: On September 10, 2025, Amazon Web Services will discontinue support for Amazon Web Services RoboMaker. After September 10, 2025, you will no longer be able to access the Amazon Web Services RoboMaker console or Amazon Web Services RoboMaker resources. For more information on transitioning to Batch to help run containerized simulations, visit <a href="https://aws.amazon.com/blogs/hpc/run-simulations-using-multiple-containers-in-a-single-aws-batch-job/">https://aws.amazon.com/blogs/hpc/run-simulations-using-multiple-containers-in-a-single-aws-batch-job/</a>.</p>
/// </important>
/// <p>Creates a world export job.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateWorldExportJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_world_export_job::builders::CreateWorldExportJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_world_export_job::CreateWorldExportJobOutput,
        crate::operation::create_world_export_job::CreateWorldExportJobError,
    > for CreateWorldExportJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_world_export_job::CreateWorldExportJobOutput,
            crate::operation::create_world_export_job::CreateWorldExportJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateWorldExportJobFluentBuilder {
    /// Creates a new `CreateWorldExportJobFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateWorldExportJob as a reference.
    pub fn as_input(&self) -> &crate::operation::create_world_export_job::builders::CreateWorldExportJobInputBuilder {
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
        crate::operation::create_world_export_job::CreateWorldExportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_world_export_job::CreateWorldExportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_world_export_job::CreateWorldExportJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_world_export_job::CreateWorldExportJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_world_export_job::CreateWorldExportJobOutput,
        crate::operation::create_world_export_job::CreateWorldExportJobError,
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    ///
    /// Appends an item to `worlds`.
    ///
    /// To override the contents of this collection use [`set_worlds`](Self::set_worlds).
    ///
    /// <p>A list of Amazon Resource Names (arns) that correspond to worlds to export.</p>
    pub fn worlds(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.worlds(input.into());
        self
    }
    /// <p>A list of Amazon Resource Names (arns) that correspond to worlds to export.</p>
    pub fn set_worlds(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_worlds(input);
        self
    }
    /// <p>A list of Amazon Resource Names (arns) that correspond to worlds to export.</p>
    pub fn get_worlds(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_worlds()
    }
    /// <p>The output location.</p>
    pub fn output_location(mut self, input: crate::types::OutputLocation) -> Self {
        self.inner = self.inner.output_location(input);
        self
    }
    /// <p>The output location.</p>
    pub fn set_output_location(mut self, input: ::std::option::Option<crate::types::OutputLocation>) -> Self {
        self.inner = self.inner.set_output_location(input);
        self
    }
    /// <p>The output location.</p>
    pub fn get_output_location(&self) -> &::std::option::Option<crate::types::OutputLocation> {
        self.inner.get_output_location()
    }
    /// <p>The IAM role that the world export process uses to access the Amazon S3 bucket and put the export.</p>
    pub fn iam_role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.iam_role(input.into());
        self
    }
    /// <p>The IAM role that the world export process uses to access the Amazon S3 bucket and put the export.</p>
    pub fn set_iam_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_iam_role(input);
        self
    }
    /// <p>The IAM role that the world export process uses to access the Amazon S3 bucket and put the export.</p>
    pub fn get_iam_role(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_iam_role()
    }
    ///
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A map that contains tag keys and tag values that are attached to the world export job.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A map that contains tag keys and tag values that are attached to the world export job.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A map that contains tag keys and tag values that are attached to the world export job.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
