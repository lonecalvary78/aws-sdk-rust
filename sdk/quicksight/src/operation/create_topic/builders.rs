// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_topic::_create_topic_output::CreateTopicOutputBuilder;

pub use crate::operation::create_topic::_create_topic_input::CreateTopicInputBuilder;

impl crate::operation::create_topic::builders::CreateTopicInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_topic::CreateTopicOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_topic::CreateTopicError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_topic();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateTopic`.
///
/// <p>Creates a new Q topic.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTopicFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_topic::builders::CreateTopicInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_topic::CreateTopicOutput,
        crate::operation::create_topic::CreateTopicError,
    > for CreateTopicFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_topic::CreateTopicOutput,
            crate::operation::create_topic::CreateTopicError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateTopicFluentBuilder {
    /// Creates a new `CreateTopicFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateTopic as a reference.
    pub fn as_input(&self) -> &crate::operation::create_topic::builders::CreateTopicInputBuilder {
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
        crate::operation::create_topic::CreateTopicOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_topic::CreateTopicError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_topic::CreateTopic::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_topic::CreateTopic::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_topic::CreateTopicOutput,
        crate::operation::create_topic::CreateTopicError,
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
    /// <p>The ID of the Amazon Web Services account that you want to create a topic in.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that you want to create a topic in.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that you want to create a topic in.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The ID for the topic that you want to create. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn topic_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.topic_id(input.into());
        self
    }
    /// <p>The ID for the topic that you want to create. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn set_topic_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_topic_id(input);
        self
    }
    /// <p>The ID for the topic that you want to create. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn get_topic_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_topic_id()
    }
    /// <p>The definition of a topic to create.</p>
    pub fn topic(mut self, input: crate::types::TopicDetails) -> Self {
        self.inner = self.inner.topic(input);
        self
    }
    /// <p>The definition of a topic to create.</p>
    pub fn set_topic(mut self, input: ::std::option::Option<crate::types::TopicDetails>) -> Self {
        self.inner = self.inner.set_topic(input);
        self
    }
    /// <p>The definition of a topic to create.</p>
    pub fn get_topic(&self) -> &::std::option::Option<crate::types::TopicDetails> {
        self.inner.get_topic()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Contains a map of the key-value pairs for the resource tag or tags that are assigned to the dataset.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Contains a map of the key-value pairs for the resource tag or tags that are assigned to the dataset.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Contains a map of the key-value pairs for the resource tag or tags that are assigned to the dataset.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    ///
    /// Appends an item to `FolderArns`.
    ///
    /// To override the contents of this collection use [`set_folder_arns`](Self::set_folder_arns).
    ///
    /// <p>The Folder ARN of the folder that you want the topic to reside in.</p>
    pub fn folder_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.folder_arns(input.into());
        self
    }
    /// <p>The Folder ARN of the folder that you want the topic to reside in.</p>
    pub fn set_folder_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_folder_arns(input);
        self
    }
    /// <p>The Folder ARN of the folder that you want the topic to reside in.</p>
    pub fn get_folder_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_folder_arns()
    }
    /// <p>Custom instructions for the topic.</p>
    pub fn custom_instructions(mut self, input: crate::types::CustomInstructions) -> Self {
        self.inner = self.inner.custom_instructions(input);
        self
    }
    /// <p>Custom instructions for the topic.</p>
    pub fn set_custom_instructions(mut self, input: ::std::option::Option<crate::types::CustomInstructions>) -> Self {
        self.inner = self.inner.set_custom_instructions(input);
        self
    }
    /// <p>Custom instructions for the topic.</p>
    pub fn get_custom_instructions(&self) -> &::std::option::Option<crate::types::CustomInstructions> {
        self.inner.get_custom_instructions()
    }
}
