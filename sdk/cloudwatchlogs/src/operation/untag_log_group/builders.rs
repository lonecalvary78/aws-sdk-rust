// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::untag_log_group::_untag_log_group_output::UntagLogGroupOutputBuilder;

pub use crate::operation::untag_log_group::_untag_log_group_input::UntagLogGroupInputBuilder;

impl crate::operation::untag_log_group::builders::UntagLogGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::untag_log_group::UntagLogGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::untag_log_group::UntagLogGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.untag_log_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UntagLogGroup`.
///
/// <important>
/// <p>The UntagLogGroup operation is on the path to deprecation. We recommend that you use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_UntagResource.html">UntagResource</a> instead.</p>
/// </important>
/// <p>Removes the specified tags from the specified log group.</p>
/// <p>To list the tags for a log group, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListTagsForResource.html">ListTagsForResource</a>. To add tags, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_TagResource.html">TagResource</a>.</p>
/// <p>When using IAM policies to control tag management for CloudWatch Logs log groups, the condition keys <code>aws:Resource/key-name</code> and <code>aws:TagKeys</code> cannot be used to restrict which tags users can assign.</p>
#[deprecated(note = "Please use the generic tagging API UntagResource")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UntagLogGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::untag_log_group::builders::UntagLogGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::untag_log_group::UntagLogGroupOutput,
        crate::operation::untag_log_group::UntagLogGroupError,
    > for UntagLogGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::untag_log_group::UntagLogGroupOutput,
            crate::operation::untag_log_group::UntagLogGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UntagLogGroupFluentBuilder {
    /// Creates a new `UntagLogGroupFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UntagLogGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::untag_log_group::builders::UntagLogGroupInputBuilder {
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
        crate::operation::untag_log_group::UntagLogGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::untag_log_group::UntagLogGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::untag_log_group::UntagLogGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::untag_log_group::UntagLogGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::untag_log_group::UntagLogGroupOutput,
        crate::operation::untag_log_group::UntagLogGroupError,
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
    /// <p>The name of the log group.</p>
    pub fn log_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.log_group_name(input.into());
        self
    }
    /// <p>The name of the log group.</p>
    pub fn set_log_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_log_group_name(input);
        self
    }
    /// <p>The name of the log group.</p>
    pub fn get_log_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_log_group_name()
    }
    ///
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tag keys. The corresponding tags are removed from the log group.</p>
    pub fn tags(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(input.into());
        self
    }
    /// <p>The tag keys. The corresponding tags are removed from the log group.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tag keys. The corresponding tags are removed from the log group.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_tags()
    }
}
