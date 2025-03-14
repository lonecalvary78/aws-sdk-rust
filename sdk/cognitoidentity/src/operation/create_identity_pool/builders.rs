// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_identity_pool::_create_identity_pool_output::CreateIdentityPoolOutputBuilder;

pub use crate::operation::create_identity_pool::_create_identity_pool_input::CreateIdentityPoolInputBuilder;

impl crate::operation::create_identity_pool::builders::CreateIdentityPoolInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_identity_pool::CreateIdentityPoolOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_identity_pool::CreateIdentityPoolError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_identity_pool();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateIdentityPool`.
///
/// <p>Creates a new identity pool. The identity pool is a store of user identity information that is specific to your Amazon Web Services account. The keys for <code>SupportedLoginProviders</code> are as follows:</p>
/// <ul>
/// <li>
/// <p>Facebook: <code>graph.facebook.com</code></p></li>
/// <li>
/// <p>Google: <code>accounts.google.com</code></p></li>
/// <li>
/// <p>Sign in With Apple: <code>appleid.apple.com</code></p></li>
/// <li>
/// <p>Amazon: <code>www.amazon.com</code></p></li>
/// <li>
/// <p>Twitter: <code>api.twitter.com</code></p></li>
/// <li>
/// <p>Digits: <code>www.digits.com</code></p></li>
/// </ul><important>
/// <p>If you don't provide a value for a parameter, Amazon Cognito sets it to its default value.</p>
/// </important>
/// <p>You must use Amazon Web Services developer credentials to call this operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateIdentityPoolFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_identity_pool::builders::CreateIdentityPoolInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_identity_pool::CreateIdentityPoolOutput,
        crate::operation::create_identity_pool::CreateIdentityPoolError,
    > for CreateIdentityPoolFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_identity_pool::CreateIdentityPoolOutput,
            crate::operation::create_identity_pool::CreateIdentityPoolError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateIdentityPoolFluentBuilder {
    /// Creates a new `CreateIdentityPoolFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateIdentityPool as a reference.
    pub fn as_input(&self) -> &crate::operation::create_identity_pool::builders::CreateIdentityPoolInputBuilder {
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
        crate::operation::create_identity_pool::CreateIdentityPoolOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_identity_pool::CreateIdentityPoolError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_identity_pool::CreateIdentityPool::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_identity_pool::CreateIdentityPool::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_identity_pool::CreateIdentityPoolOutput,
        crate::operation::create_identity_pool::CreateIdentityPoolError,
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
    /// <p>A string that you provide.</p>
    pub fn identity_pool_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identity_pool_name(input.into());
        self
    }
    /// <p>A string that you provide.</p>
    pub fn set_identity_pool_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identity_pool_name(input);
        self
    }
    /// <p>A string that you provide.</p>
    pub fn get_identity_pool_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identity_pool_name()
    }
    /// <p>TRUE if the identity pool supports unauthenticated logins.</p>
    pub fn allow_unauthenticated_identities(mut self, input: bool) -> Self {
        self.inner = self.inner.allow_unauthenticated_identities(input);
        self
    }
    /// <p>TRUE if the identity pool supports unauthenticated logins.</p>
    pub fn set_allow_unauthenticated_identities(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_allow_unauthenticated_identities(input);
        self
    }
    /// <p>TRUE if the identity pool supports unauthenticated logins.</p>
    pub fn get_allow_unauthenticated_identities(&self) -> &::std::option::Option<bool> {
        self.inner.get_allow_unauthenticated_identities()
    }
    /// <p>Enables or disables the Basic (Classic) authentication flow. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flow.html">Identity Pools (Federated Identities) Authentication Flow</a> in the <i>Amazon Cognito Developer Guide</i>.</p>
    pub fn allow_classic_flow(mut self, input: bool) -> Self {
        self.inner = self.inner.allow_classic_flow(input);
        self
    }
    /// <p>Enables or disables the Basic (Classic) authentication flow. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flow.html">Identity Pools (Federated Identities) Authentication Flow</a> in the <i>Amazon Cognito Developer Guide</i>.</p>
    pub fn set_allow_classic_flow(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_allow_classic_flow(input);
        self
    }
    /// <p>Enables or disables the Basic (Classic) authentication flow. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/authentication-flow.html">Identity Pools (Federated Identities) Authentication Flow</a> in the <i>Amazon Cognito Developer Guide</i>.</p>
    pub fn get_allow_classic_flow(&self) -> &::std::option::Option<bool> {
        self.inner.get_allow_classic_flow()
    }
    ///
    /// Adds a key-value pair to `SupportedLoginProviders`.
    ///
    /// To override the contents of this collection use [`set_supported_login_providers`](Self::set_supported_login_providers).
    ///
    /// <p>Optional key:value pairs mapping provider names to provider app IDs.</p>
    pub fn supported_login_providers(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.supported_login_providers(k.into(), v.into());
        self
    }
    /// <p>Optional key:value pairs mapping provider names to provider app IDs.</p>
    pub fn set_supported_login_providers(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_supported_login_providers(input);
        self
    }
    /// <p>Optional key:value pairs mapping provider names to provider app IDs.</p>
    pub fn get_supported_login_providers(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_supported_login_providers()
    }
    /// <p>The "domain" by which Cognito will refer to your users. This name acts as a placeholder that allows your backend and the Cognito service to communicate about the developer provider. For the <code>DeveloperProviderName</code>, you can use letters as well as period (<code>.</code>), underscore (<code>_</code>), and dash (<code>-</code>).</p>
    /// <p>Once you have set a developer provider name, you cannot change it. Please take care in setting this parameter.</p>
    pub fn developer_provider_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.developer_provider_name(input.into());
        self
    }
    /// <p>The "domain" by which Cognito will refer to your users. This name acts as a placeholder that allows your backend and the Cognito service to communicate about the developer provider. For the <code>DeveloperProviderName</code>, you can use letters as well as period (<code>.</code>), underscore (<code>_</code>), and dash (<code>-</code>).</p>
    /// <p>Once you have set a developer provider name, you cannot change it. Please take care in setting this parameter.</p>
    pub fn set_developer_provider_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_developer_provider_name(input);
        self
    }
    /// <p>The "domain" by which Cognito will refer to your users. This name acts as a placeholder that allows your backend and the Cognito service to communicate about the developer provider. For the <code>DeveloperProviderName</code>, you can use letters as well as period (<code>.</code>), underscore (<code>_</code>), and dash (<code>-</code>).</p>
    /// <p>Once you have set a developer provider name, you cannot change it. Please take care in setting this parameter.</p>
    pub fn get_developer_provider_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_developer_provider_name()
    }
    ///
    /// Appends an item to `OpenIdConnectProviderARNs`.
    ///
    /// To override the contents of this collection use [`set_open_id_connect_provider_arns`](Self::set_open_id_connect_provider_arns).
    ///
    /// <p>The Amazon Resource Names (ARN) of the OpenID Connect providers.</p>
    pub fn open_id_connect_provider_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.open_id_connect_provider_arns(input.into());
        self
    }
    /// <p>The Amazon Resource Names (ARN) of the OpenID Connect providers.</p>
    pub fn set_open_id_connect_provider_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_open_id_connect_provider_arns(input);
        self
    }
    /// <p>The Amazon Resource Names (ARN) of the OpenID Connect providers.</p>
    pub fn get_open_id_connect_provider_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_open_id_connect_provider_arns()
    }
    ///
    /// Appends an item to `CognitoIdentityProviders`.
    ///
    /// To override the contents of this collection use [`set_cognito_identity_providers`](Self::set_cognito_identity_providers).
    ///
    /// <p>An array of Amazon Cognito user pools and their client IDs.</p>
    pub fn cognito_identity_providers(mut self, input: crate::types::CognitoIdentityProvider) -> Self {
        self.inner = self.inner.cognito_identity_providers(input);
        self
    }
    /// <p>An array of Amazon Cognito user pools and their client IDs.</p>
    pub fn set_cognito_identity_providers(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CognitoIdentityProvider>>) -> Self {
        self.inner = self.inner.set_cognito_identity_providers(input);
        self
    }
    /// <p>An array of Amazon Cognito user pools and their client IDs.</p>
    pub fn get_cognito_identity_providers(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CognitoIdentityProvider>> {
        self.inner.get_cognito_identity_providers()
    }
    ///
    /// Appends an item to `SamlProviderARNs`.
    ///
    /// To override the contents of this collection use [`set_saml_provider_arns`](Self::set_saml_provider_arns).
    ///
    /// <p>An array of Amazon Resource Names (ARNs) of the SAML provider for your identity pool.</p>
    pub fn saml_provider_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.saml_provider_arns(input.into());
        self
    }
    /// <p>An array of Amazon Resource Names (ARNs) of the SAML provider for your identity pool.</p>
    pub fn set_saml_provider_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_saml_provider_arns(input);
        self
    }
    /// <p>An array of Amazon Resource Names (ARNs) of the SAML provider for your identity pool.</p>
    pub fn get_saml_provider_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_saml_provider_arns()
    }
    ///
    /// Adds a key-value pair to `IdentityPoolTags`.
    ///
    /// To override the contents of this collection use [`set_identity_pool_tags`](Self::set_identity_pool_tags).
    ///
    /// <p>Tags to assign to the identity pool. A tag is a label that you can apply to identity pools to categorize and manage them in different ways, such as by purpose, owner, environment, or other criteria.</p>
    pub fn identity_pool_tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.identity_pool_tags(k.into(), v.into());
        self
    }
    /// <p>Tags to assign to the identity pool. A tag is a label that you can apply to identity pools to categorize and manage them in different ways, such as by purpose, owner, environment, or other criteria.</p>
    pub fn set_identity_pool_tags(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_identity_pool_tags(input);
        self
    }
    /// <p>Tags to assign to the identity pool. A tag is a label that you can apply to identity pools to categorize and manage them in different ways, such as by purpose, owner, environment, or other criteria.</p>
    pub fn get_identity_pool_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_identity_pool_tags()
    }
}
