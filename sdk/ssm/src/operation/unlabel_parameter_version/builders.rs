// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::unlabel_parameter_version::_unlabel_parameter_version_output::UnlabelParameterVersionOutputBuilder;

pub use crate::operation::unlabel_parameter_version::_unlabel_parameter_version_input::UnlabelParameterVersionInputBuilder;

impl crate::operation::unlabel_parameter_version::builders::UnlabelParameterVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::unlabel_parameter_version::UnlabelParameterVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::unlabel_parameter_version::UnlabelParameterVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.unlabel_parameter_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UnlabelParameterVersion`.
///
/// <p>Remove a label or labels from a parameter.</p>
/// <p>Parameter names can't contain spaces. The service removes any spaces specified for the beginning or end of a parameter name. If the specified name for a parameter contains spaces between characters, the request fails with a <code>ValidationException</code> error.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UnlabelParameterVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::unlabel_parameter_version::builders::UnlabelParameterVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::unlabel_parameter_version::UnlabelParameterVersionOutput,
        crate::operation::unlabel_parameter_version::UnlabelParameterVersionError,
    > for UnlabelParameterVersionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::unlabel_parameter_version::UnlabelParameterVersionOutput,
            crate::operation::unlabel_parameter_version::UnlabelParameterVersionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UnlabelParameterVersionFluentBuilder {
    /// Creates a new `UnlabelParameterVersionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UnlabelParameterVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::unlabel_parameter_version::builders::UnlabelParameterVersionInputBuilder {
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
        crate::operation::unlabel_parameter_version::UnlabelParameterVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::unlabel_parameter_version::UnlabelParameterVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::unlabel_parameter_version::UnlabelParameterVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::unlabel_parameter_version::UnlabelParameterVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::unlabel_parameter_version::UnlabelParameterVersionOutput,
        crate::operation::unlabel_parameter_version::UnlabelParameterVersionError,
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
    /// <p>The name of the parameter from which you want to delete one or more labels.</p><note>
    /// <p>You can't enter the Amazon Resource Name (ARN) for a parameter, only the parameter name itself.</p>
    /// </note>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the parameter from which you want to delete one or more labels.</p><note>
    /// <p>You can't enter the Amazon Resource Name (ARN) for a parameter, only the parameter name itself.</p>
    /// </note>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the parameter from which you want to delete one or more labels.</p><note>
    /// <p>You can't enter the Amazon Resource Name (ARN) for a parameter, only the parameter name itself.</p>
    /// </note>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The specific version of the parameter which you want to delete one or more labels from. If it isn't present, the call will fail.</p>
    pub fn parameter_version(mut self, input: i64) -> Self {
        self.inner = self.inner.parameter_version(input);
        self
    }
    /// <p>The specific version of the parameter which you want to delete one or more labels from. If it isn't present, the call will fail.</p>
    pub fn set_parameter_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_parameter_version(input);
        self
    }
    /// <p>The specific version of the parameter which you want to delete one or more labels from. If it isn't present, the call will fail.</p>
    pub fn get_parameter_version(&self) -> &::std::option::Option<i64> {
        self.inner.get_parameter_version()
    }
    ///
    /// Appends an item to `Labels`.
    ///
    /// To override the contents of this collection use [`set_labels`](Self::set_labels).
    ///
    /// <p>One or more labels to delete from the specified parameter version.</p>
    pub fn labels(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.labels(input.into());
        self
    }
    /// <p>One or more labels to delete from the specified parameter version.</p>
    pub fn set_labels(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_labels(input);
        self
    }
    /// <p>One or more labels to delete from the specified parameter version.</p>
    pub fn get_labels(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_labels()
    }
}
