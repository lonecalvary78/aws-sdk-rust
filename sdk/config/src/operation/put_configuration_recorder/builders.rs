// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_configuration_recorder::_put_configuration_recorder_output::PutConfigurationRecorderOutputBuilder;

pub use crate::operation::put_configuration_recorder::_put_configuration_recorder_input::PutConfigurationRecorderInputBuilder;

impl crate::operation::put_configuration_recorder::builders::PutConfigurationRecorderInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_configuration_recorder::PutConfigurationRecorderOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_configuration_recorder::PutConfigurationRecorderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_configuration_recorder();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutConfigurationRecorder`.
///
/// <p>Creates or updates the customer managed configuration recorder.</p>
/// <p>You can use this operation to create a new customer managed configuration recorder or to update the <code>roleARN</code> and the <code>recordingGroup</code> for an existing customer managed configuration recorder.</p>
/// <p>To start the customer managed configuration recorder and begin recording configuration changes for the resource types you specify, use the <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_StartConfigurationRecorder.html">StartConfigurationRecorder</a> operation.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/stop-start-recorder.html"> <b>Working with the Configuration Recorder</b> </a> in the <i>Config Developer Guide</i>.</p><note>
/// <p><b>One customer managed configuration recorder per account per Region</b></p>
/// <p>You can create only one customer managed configuration recorder for each account for each Amazon Web Services Region.</p>
/// <p><b>Default is to record all supported resource types, excluding the global IAM resource types</b></p>
/// <p>If you have not specified values for the <code>recordingGroup</code> field, the default for the customer managed configuration recorder is to record all supported resource types, excluding the global IAM resource types: <code>AWS::IAM::Group</code>, <code>AWS::IAM::Policy</code>, <code>AWS::IAM::Role</code>, and <code>AWS::IAM::User</code>.</p>
/// <p><b>Tags are added at creation and cannot be updated</b></p>
/// <p><code>PutConfigurationRecorder</code> is an idempotent API. Subsequent requests won’t create a duplicate resource if one was already created. If a following request has different tags values, Config will ignore these differences and treat it as an idempotent request of the previous. In this case, tags will not be updated, even if they are different.</p>
/// <p>Use <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_TagResource.html">TagResource</a> and <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_UntagResource.html">UntagResource</a> to update tags after creation.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutConfigurationRecorderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_configuration_recorder::builders::PutConfigurationRecorderInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_configuration_recorder::PutConfigurationRecorderOutput,
        crate::operation::put_configuration_recorder::PutConfigurationRecorderError,
    > for PutConfigurationRecorderFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_configuration_recorder::PutConfigurationRecorderOutput,
            crate::operation::put_configuration_recorder::PutConfigurationRecorderError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutConfigurationRecorderFluentBuilder {
    /// Creates a new `PutConfigurationRecorderFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutConfigurationRecorder as a reference.
    pub fn as_input(&self) -> &crate::operation::put_configuration_recorder::builders::PutConfigurationRecorderInputBuilder {
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
        crate::operation::put_configuration_recorder::PutConfigurationRecorderOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_configuration_recorder::PutConfigurationRecorderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_configuration_recorder::PutConfigurationRecorder::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_configuration_recorder::PutConfigurationRecorder::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_configuration_recorder::PutConfigurationRecorderOutput,
        crate::operation::put_configuration_recorder::PutConfigurationRecorderError,
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
    /// <p>An object for the configuration recorder. A configuration recorder records configuration changes for the resource types in scope.</p>
    pub fn configuration_recorder(mut self, input: crate::types::ConfigurationRecorder) -> Self {
        self.inner = self.inner.configuration_recorder(input);
        self
    }
    /// <p>An object for the configuration recorder. A configuration recorder records configuration changes for the resource types in scope.</p>
    pub fn set_configuration_recorder(mut self, input: ::std::option::Option<crate::types::ConfigurationRecorder>) -> Self {
        self.inner = self.inner.set_configuration_recorder(input);
        self
    }
    /// <p>An object for the configuration recorder. A configuration recorder records configuration changes for the resource types in scope.</p>
    pub fn get_configuration_recorder(&self) -> &::std::option::Option<crate::types::ConfigurationRecorder> {
        self.inner.get_configuration_recorder()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the customer managed configuration recorder. Each tag consists of a key and an optional value, both of which you define.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags for the customer managed configuration recorder. Each tag consists of a key and an optional value, both of which you define.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags for the customer managed configuration recorder. Each tag consists of a key and an optional value, both of which you define.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
