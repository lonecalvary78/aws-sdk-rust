// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_flow::_update_flow_output::UpdateFlowOutputBuilder;

pub use crate::operation::update_flow::_update_flow_input::UpdateFlowInputBuilder;

impl crate::operation::update_flow::builders::UpdateFlowInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_flow::UpdateFlowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_flow::UpdateFlowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_flow();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateFlow`.
///
/// <p>Modifies a flow. Include both fields that you want to keep and fields that you want to change. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/flows-how-it-works.html">How it works</a> and <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/flows-create.html">Create a flow in Amazon Bedrock</a> in the Amazon Bedrock User Guide.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateFlowFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_flow::builders::UpdateFlowInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_flow::UpdateFlowOutput,
        crate::operation::update_flow::UpdateFlowError,
    > for UpdateFlowFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_flow::UpdateFlowOutput,
            crate::operation::update_flow::UpdateFlowError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateFlowFluentBuilder {
    /// Creates a new `UpdateFlowFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateFlow as a reference.
    pub fn as_input(&self) -> &crate::operation::update_flow::builders::UpdateFlowInputBuilder {
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
        crate::operation::update_flow::UpdateFlowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_flow::UpdateFlowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_flow::UpdateFlow::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_flow::UpdateFlow::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_flow::UpdateFlowOutput,
        crate::operation::update_flow::UpdateFlowError,
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
    /// <p>A name for the flow.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A name for the flow.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A name for the flow.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A description for the flow.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the flow.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for the flow.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The Amazon Resource Name (ARN) of the service role with permissions to create and manage a flow. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/flows-permissions.html">Create a service role for flows in Amazon Bedrock</a> in the Amazon Bedrock User Guide.</p>
    pub fn execution_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.execution_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service role with permissions to create and manage a flow. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/flows-permissions.html">Create a service role for flows in Amazon Bedrock</a> in the Amazon Bedrock User Guide.</p>
    pub fn set_execution_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_execution_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service role with permissions to create and manage a flow. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/flows-permissions.html">Create a service role for flows in Amazon Bedrock</a> in the Amazon Bedrock User Guide.</p>
    pub fn get_execution_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_execution_role_arn()
    }
    /// <p>The Amazon Resource Name (ARN) of the KMS key to encrypt the flow.</p>
    pub fn customer_encryption_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.customer_encryption_key_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the KMS key to encrypt the flow.</p>
    pub fn set_customer_encryption_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_customer_encryption_key_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the KMS key to encrypt the flow.</p>
    pub fn get_customer_encryption_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_customer_encryption_key_arn()
    }
    /// <p>A definition of the nodes and the connections between the nodes in the flow.</p>
    pub fn definition(mut self, input: crate::types::FlowDefinition) -> Self {
        self.inner = self.inner.definition(input);
        self
    }
    /// <p>A definition of the nodes and the connections between the nodes in the flow.</p>
    pub fn set_definition(mut self, input: ::std::option::Option<crate::types::FlowDefinition>) -> Self {
        self.inner = self.inner.set_definition(input);
        self
    }
    /// <p>A definition of the nodes and the connections between the nodes in the flow.</p>
    pub fn get_definition(&self) -> &::std::option::Option<crate::types::FlowDefinition> {
        self.inner.get_definition()
    }
    /// <p>The unique identifier of the flow.</p>
    pub fn flow_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.flow_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the flow.</p>
    pub fn set_flow_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_flow_identifier(input);
        self
    }
    /// <p>The unique identifier of the flow.</p>
    pub fn get_flow_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_flow_identifier()
    }
}
