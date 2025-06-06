// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_tokens_from_refresh_token::_get_tokens_from_refresh_token_output::GetTokensFromRefreshTokenOutputBuilder;

pub use crate::operation::get_tokens_from_refresh_token::_get_tokens_from_refresh_token_input::GetTokensFromRefreshTokenInputBuilder;

impl crate::operation::get_tokens_from_refresh_token::builders::GetTokensFromRefreshTokenInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshTokenOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshTokenError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_tokens_from_refresh_token();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetTokensFromRefreshToken`.
///
/// <p>Given a refresh token, issues new ID, access, and optionally refresh tokens for the user who owns the submitted token. This operation issues a new refresh token and invalidates the original refresh token after an optional grace period when refresh token rotation is enabled. If refresh token rotation is disabled, issues new ID and access tokens only.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetTokensFromRefreshTokenFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_tokens_from_refresh_token::builders::GetTokensFromRefreshTokenInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshTokenOutput,
        crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshTokenError,
    > for GetTokensFromRefreshTokenFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshTokenOutput,
            crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshTokenError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetTokensFromRefreshTokenFluentBuilder {
    /// Creates a new `GetTokensFromRefreshTokenFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetTokensFromRefreshToken as a reference.
    pub fn as_input(&self) -> &crate::operation::get_tokens_from_refresh_token::builders::GetTokensFromRefreshTokenInputBuilder {
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
        crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshTokenOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshTokenError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshToken::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshToken::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshTokenOutput,
        crate::operation::get_tokens_from_refresh_token::GetTokensFromRefreshTokenError,
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
    /// <p>A valid refresh token that can authorize the request for new tokens. When refresh token rotation is active in the requested app client, this token is invalidated after the request is complete.</p>
    pub fn refresh_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.refresh_token(input.into());
        self
    }
    /// <p>A valid refresh token that can authorize the request for new tokens. When refresh token rotation is active in the requested app client, this token is invalidated after the request is complete.</p>
    pub fn set_refresh_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_refresh_token(input);
        self
    }
    /// <p>A valid refresh token that can authorize the request for new tokens. When refresh token rotation is active in the requested app client, this token is invalidated after the request is complete.</p>
    pub fn get_refresh_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_refresh_token()
    }
    /// <p>The app client that issued the refresh token to the user who wants to request new tokens.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_id(input.into());
        self
    }
    /// <p>The app client that issued the refresh token to the user who wants to request new tokens.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_id(input);
        self
    }
    /// <p>The app client that issued the refresh token to the user who wants to request new tokens.</p>
    pub fn get_client_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_id()
    }
    /// <p>The client secret of the requested app client, if the client has a secret.</p>
    pub fn client_secret(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_secret(input.into());
        self
    }
    /// <p>The client secret of the requested app client, if the client has a secret.</p>
    pub fn set_client_secret(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_secret(input);
        self
    }
    /// <p>The client secret of the requested app client, if the client has a secret.</p>
    pub fn get_client_secret(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_secret()
    }
    /// <p>When you enable device remembering, Amazon Cognito issues a device key that you can use for device authentication that bypasses multi-factor authentication (MFA). To implement <code>GetTokensFromRefreshToken</code> in a user pool with device remembering, you must capture the device key from the initial authentication request. If your application doesn't provide the key of a registered device, Amazon Cognito issues a new one. You must provide the confirmed device key in this request if device remembering is enabled in your user pool.</p>
    /// <p>For more information about device remembering, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with devices</a>.</p>
    pub fn device_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_key(input.into());
        self
    }
    /// <p>When you enable device remembering, Amazon Cognito issues a device key that you can use for device authentication that bypasses multi-factor authentication (MFA). To implement <code>GetTokensFromRefreshToken</code> in a user pool with device remembering, you must capture the device key from the initial authentication request. If your application doesn't provide the key of a registered device, Amazon Cognito issues a new one. You must provide the confirmed device key in this request if device remembering is enabled in your user pool.</p>
    /// <p>For more information about device remembering, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with devices</a>.</p>
    pub fn set_device_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_key(input);
        self
    }
    /// <p>When you enable device remembering, Amazon Cognito issues a device key that you can use for device authentication that bypasses multi-factor authentication (MFA). To implement <code>GetTokensFromRefreshToken</code> in a user pool with device remembering, you must capture the device key from the initial authentication request. If your application doesn't provide the key of a registered device, Amazon Cognito issues a new one. You must provide the confirmed device key in this request if device remembering is enabled in your user pool.</p>
    /// <p>For more information about device remembering, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/amazon-cognito-user-pools-device-tracking.html">Working with devices</a>.</p>
    pub fn get_device_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_device_key()
    }
    ///
    /// Adds a key-value pair to `ClientMetadata`.
    ///
    /// To override the contents of this collection use [`set_client_metadata`](Self::set_client_metadata).
    ///
    /// <p>A map of custom key-value pairs that you can provide as input for certain custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the <code>GetTokensFromRefreshToken</code> API action, Amazon Cognito invokes the Lambda function the pre token generation trigger.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Using Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>
    /// <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>
    /// <li>
    /// <p>Validate the <code>ClientMetadata</code> value.</p></li>
    /// <li>
    /// <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>
    /// </ul>
    /// </note>
    pub fn client_metadata(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_metadata(k.into(), v.into());
        self
    }
    /// <p>A map of custom key-value pairs that you can provide as input for certain custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the <code>GetTokensFromRefreshToken</code> API action, Amazon Cognito invokes the Lambda function the pre token generation trigger.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Using Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>
    /// <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>
    /// <li>
    /// <p>Validate the <code>ClientMetadata</code> value.</p></li>
    /// <li>
    /// <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>
    /// </ul>
    /// </note>
    pub fn set_client_metadata(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_client_metadata(input);
        self
    }
    /// <p>A map of custom key-value pairs that you can provide as input for certain custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the <code>GetTokensFromRefreshToken</code> API action, Amazon Cognito invokes the Lambda function the pre token generation trigger.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Using Lambda triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p><note>
    /// <p>When you use the <code>ClientMetadata</code> parameter, note that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Store the <code>ClientMetadata</code> value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the <code>ClientMetadata</code> parameter serves no purpose.</p></li>
    /// <li>
    /// <p>Validate the <code>ClientMetadata</code> value.</p></li>
    /// <li>
    /// <p>Encrypt the <code>ClientMetadata</code> value. Don't send sensitive information in this parameter.</p></li>
    /// </ul>
    /// </note>
    pub fn get_client_metadata(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_client_metadata()
    }
}
