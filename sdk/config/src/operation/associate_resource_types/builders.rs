// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_resource_types::_associate_resource_types_output::AssociateResourceTypesOutputBuilder;

pub use crate::operation::associate_resource_types::_associate_resource_types_input::AssociateResourceTypesInputBuilder;

impl crate::operation::associate_resource_types::builders::AssociateResourceTypesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_resource_types::AssociateResourceTypesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_resource_types::AssociateResourceTypesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_resource_types();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateResourceTypes`.
///
/// <p>Adds all resource types specified in the <code>ResourceTypes</code> list to the <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_RecordingGroup.html">RecordingGroup</a> of specified configuration recorder and includes those resource types when recording.</p>
/// <p>For this operation, the specified configuration recorder must use a <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_RecordingStrategy.html">RecordingStrategy</a> that is either <code>INCLUSION_BY_RESOURCE_TYPES</code> or <code>EXCLUSION_BY_RESOURCE_TYPES</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateResourceTypesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_resource_types::builders::AssociateResourceTypesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_resource_types::AssociateResourceTypesOutput,
        crate::operation::associate_resource_types::AssociateResourceTypesError,
    > for AssociateResourceTypesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_resource_types::AssociateResourceTypesOutput,
            crate::operation::associate_resource_types::AssociateResourceTypesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateResourceTypesFluentBuilder {
    /// Creates a new `AssociateResourceTypesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateResourceTypes as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_resource_types::builders::AssociateResourceTypesInputBuilder {
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
        crate::operation::associate_resource_types::AssociateResourceTypesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_resource_types::AssociateResourceTypesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_resource_types::AssociateResourceTypes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_resource_types::AssociateResourceTypes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_resource_types::AssociateResourceTypesOutput,
        crate::operation::associate_resource_types::AssociateResourceTypesError,
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
    /// <p>The Amazon Resource Name (ARN) of the specified configuration recorder.</p>
    pub fn configuration_recorder_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.configuration_recorder_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the specified configuration recorder.</p>
    pub fn set_configuration_recorder_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_configuration_recorder_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the specified configuration recorder.</p>
    pub fn get_configuration_recorder_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_configuration_recorder_arn()
    }
    ///
    /// Appends an item to `ResourceTypes`.
    ///
    /// To override the contents of this collection use [`set_resource_types`](Self::set_resource_types).
    ///
    /// <p>The list of resource types you want to add to the recording group of the specified configuration recorder.</p>
    pub fn resource_types(mut self, input: crate::types::ResourceType) -> Self {
        self.inner = self.inner.resource_types(input);
        self
    }
    /// <p>The list of resource types you want to add to the recording group of the specified configuration recorder.</p>
    pub fn set_resource_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ResourceType>>) -> Self {
        self.inner = self.inner.set_resource_types(input);
        self
    }
    /// <p>The list of resource types you want to add to the recording group of the specified configuration recorder.</p>
    pub fn get_resource_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ResourceType>> {
        self.inner.get_resource_types()
    }
}
