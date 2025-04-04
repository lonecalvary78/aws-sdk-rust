// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_regex_pattern_set::_get_regex_pattern_set_output::GetRegexPatternSetOutputBuilder;

pub use crate::operation::get_regex_pattern_set::_get_regex_pattern_set_input::GetRegexPatternSetInputBuilder;

impl crate::operation::get_regex_pattern_set::builders::GetRegexPatternSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_regex_pattern_set::GetRegexPatternSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_regex_pattern_set::GetRegexPatternSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_regex_pattern_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetRegexPatternSet`.
///
/// <p>Retrieves the specified <code>RegexPatternSet</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetRegexPatternSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_regex_pattern_set::builders::GetRegexPatternSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_regex_pattern_set::GetRegexPatternSetOutput,
        crate::operation::get_regex_pattern_set::GetRegexPatternSetError,
    > for GetRegexPatternSetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_regex_pattern_set::GetRegexPatternSetOutput,
            crate::operation::get_regex_pattern_set::GetRegexPatternSetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetRegexPatternSetFluentBuilder {
    /// Creates a new `GetRegexPatternSetFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetRegexPatternSet as a reference.
    pub fn as_input(&self) -> &crate::operation::get_regex_pattern_set::builders::GetRegexPatternSetInputBuilder {
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
        crate::operation::get_regex_pattern_set::GetRegexPatternSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_regex_pattern_set::GetRegexPatternSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_regex_pattern_set::GetRegexPatternSet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_regex_pattern_set::GetRegexPatternSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_regex_pattern_set::GetRegexPatternSetOutput,
        crate::operation::get_regex_pattern_set::GetRegexPatternSetError,
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
    /// <p>The name of the set. You cannot change the name after you create the set.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the set. You cannot change the name after you create the set.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the set. You cannot change the name after you create the set.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
    /// <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows:</p>
    /// <ul>
    /// <li>
    /// <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>.</p></li>
    /// <li>
    /// <p>API and SDKs - For all calls, use the Region endpoint us-east-1.</p></li>
    /// </ul>
    pub fn scope(mut self, input: crate::types::Scope) -> Self {
        self.inner = self.inner.scope(input);
        self
    }
    /// <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
    /// <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows:</p>
    /// <ul>
    /// <li>
    /// <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>.</p></li>
    /// <li>
    /// <p>API and SDKs - For all calls, use the Region endpoint us-east-1.</p></li>
    /// </ul>
    pub fn set_scope(mut self, input: ::std::option::Option<crate::types::Scope>) -> Self {
        self.inner = self.inner.set_scope(input);
        self
    }
    /// <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
    /// <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows:</p>
    /// <ul>
    /// <li>
    /// <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>.</p></li>
    /// <li>
    /// <p>API and SDKs - For all calls, use the Region endpoint us-east-1.</p></li>
    /// </ul>
    pub fn get_scope(&self) -> &::std::option::Option<crate::types::Scope> {
        self.inner.get_scope()
    }
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>A unique identifier for the set. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
}
