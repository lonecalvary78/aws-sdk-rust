// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_browser::_get_browser_output::GetBrowserOutputBuilder;

pub use crate::operation::get_browser::_get_browser_input::GetBrowserInputBuilder;

impl crate::operation::get_browser::builders::GetBrowserInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_browser::GetBrowserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_browser::GetBrowserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_browser();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetBrowser`.
///
/// <p>Gets information about a custom browser.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetBrowserFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_browser::builders::GetBrowserInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_browser::GetBrowserOutput,
        crate::operation::get_browser::GetBrowserError,
    > for GetBrowserFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_browser::GetBrowserOutput,
            crate::operation::get_browser::GetBrowserError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetBrowserFluentBuilder {
    /// Creates a new `GetBrowserFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetBrowser as a reference.
    pub fn as_input(&self) -> &crate::operation::get_browser::builders::GetBrowserInputBuilder {
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
        crate::operation::get_browser::GetBrowserOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_browser::GetBrowserError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_browser::GetBrowser::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_browser::GetBrowser::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_browser::GetBrowserOutput,
        crate::operation::get_browser::GetBrowserError,
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
    /// <p>The unique identifier of the browser to retrieve.</p>
    pub fn browser_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.browser_id(input.into());
        self
    }
    /// <p>The unique identifier of the browser to retrieve.</p>
    pub fn set_browser_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_browser_id(input);
        self
    }
    /// <p>The unique identifier of the browser to retrieve.</p>
    pub fn get_browser_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_browser_id()
    }
}
