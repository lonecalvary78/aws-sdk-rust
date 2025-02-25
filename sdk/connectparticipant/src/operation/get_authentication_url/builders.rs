// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_authentication_url::_get_authentication_url_output::GetAuthenticationUrlOutputBuilder;

pub use crate::operation::get_authentication_url::_get_authentication_url_input::GetAuthenticationUrlInputBuilder;

impl crate::operation::get_authentication_url::builders::GetAuthenticationUrlInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_authentication_url::GetAuthenticationUrlOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_authentication_url::GetAuthenticationUrlError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_authentication_url();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetAuthenticationUrl`.
///
/// <p>Retrieves the AuthenticationUrl for the current authentication session for the AuthenticateCustomer flow block.</p>
/// <p>For security recommendations, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/security-best-practices.html#bp-security-chat">Amazon Connect Chat security best practices</a>.</p><note>
/// <ul>
/// <li>
/// <p>This API can only be called within one minute of receiving the authenticationInitiated event.</p></li>
/// <li>
/// <p>The current supported channel is chat. This API is not supported for Apple Messages for Business, WhatsApp, or SMS chats.</p></li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetAuthenticationUrlFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_authentication_url::builders::GetAuthenticationUrlInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_authentication_url::GetAuthenticationUrlOutput,
        crate::operation::get_authentication_url::GetAuthenticationUrlError,
    > for GetAuthenticationUrlFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_authentication_url::GetAuthenticationUrlOutput,
            crate::operation::get_authentication_url::GetAuthenticationUrlError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetAuthenticationUrlFluentBuilder {
    /// Creates a new `GetAuthenticationUrlFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetAuthenticationUrl as a reference.
    pub fn as_input(&self) -> &crate::operation::get_authentication_url::builders::GetAuthenticationUrlInputBuilder {
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
        crate::operation::get_authentication_url::GetAuthenticationUrlOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_authentication_url::GetAuthenticationUrlError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_authentication_url::GetAuthenticationUrl::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_authentication_url::GetAuthenticationUrl::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_authentication_url::GetAuthenticationUrlOutput,
        crate::operation::get_authentication_url::GetAuthenticationUrlError,
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
    /// <p>The sessionId provided in the authenticationInitiated event.</p>
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.session_id(input.into());
        self
    }
    /// <p>The sessionId provided in the authenticationInitiated event.</p>
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_session_id(input);
        self
    }
    /// <p>The sessionId provided in the authenticationInitiated event.</p>
    pub fn get_session_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_session_id()
    }
    /// <p>The URL where the customer will be redirected after Amazon Cognito authorizes the user.</p>
    pub fn redirect_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.redirect_uri(input.into());
        self
    }
    /// <p>The URL where the customer will be redirected after Amazon Cognito authorizes the user.</p>
    pub fn set_redirect_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_redirect_uri(input);
        self
    }
    /// <p>The URL where the customer will be redirected after Amazon Cognito authorizes the user.</p>
    pub fn get_redirect_uri(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_redirect_uri()
    }
    /// <p>The authentication token associated with the participant's connection.</p>
    pub fn connection_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_token(input.into());
        self
    }
    /// <p>The authentication token associated with the participant's connection.</p>
    pub fn set_connection_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connection_token(input);
        self
    }
    /// <p>The authentication token associated with the participant's connection.</p>
    pub fn get_connection_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connection_token()
    }
}
