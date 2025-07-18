// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateOauth2CredentialProviderInput {
    /// <p>The name of the OAuth2 credential provider. The name must be unique within your account.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The vendor of the OAuth2 credential provider. This specifies which OAuth2 implementation to use.</p>
    pub credential_provider_vendor: ::std::option::Option<crate::types::CredentialProviderVendorType>,
    /// <p>The configuration settings for the OAuth2 provider, including client ID, client secret, and other vendor-specific settings.</p>
    pub oauth2_provider_config_input: ::std::option::Option<crate::types::Oauth2ProviderConfigInput>,
}
impl CreateOauth2CredentialProviderInput {
    /// <p>The name of the OAuth2 credential provider. The name must be unique within your account.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The vendor of the OAuth2 credential provider. This specifies which OAuth2 implementation to use.</p>
    pub fn credential_provider_vendor(&self) -> ::std::option::Option<&crate::types::CredentialProviderVendorType> {
        self.credential_provider_vendor.as_ref()
    }
    /// <p>The configuration settings for the OAuth2 provider, including client ID, client secret, and other vendor-specific settings.</p>
    pub fn oauth2_provider_config_input(&self) -> ::std::option::Option<&crate::types::Oauth2ProviderConfigInput> {
        self.oauth2_provider_config_input.as_ref()
    }
}
impl CreateOauth2CredentialProviderInput {
    /// Creates a new builder-style object to manufacture [`CreateOauth2CredentialProviderInput`](crate::operation::create_oauth2_credential_provider::CreateOauth2CredentialProviderInput).
    pub fn builder() -> crate::operation::create_oauth2_credential_provider::builders::CreateOauth2CredentialProviderInputBuilder {
        crate::operation::create_oauth2_credential_provider::builders::CreateOauth2CredentialProviderInputBuilder::default()
    }
}

/// A builder for [`CreateOauth2CredentialProviderInput`](crate::operation::create_oauth2_credential_provider::CreateOauth2CredentialProviderInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateOauth2CredentialProviderInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) credential_provider_vendor: ::std::option::Option<crate::types::CredentialProviderVendorType>,
    pub(crate) oauth2_provider_config_input: ::std::option::Option<crate::types::Oauth2ProviderConfigInput>,
}
impl CreateOauth2CredentialProviderInputBuilder {
    /// <p>The name of the OAuth2 credential provider. The name must be unique within your account.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the OAuth2 credential provider. The name must be unique within your account.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the OAuth2 credential provider. The name must be unique within your account.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The vendor of the OAuth2 credential provider. This specifies which OAuth2 implementation to use.</p>
    /// This field is required.
    pub fn credential_provider_vendor(mut self, input: crate::types::CredentialProviderVendorType) -> Self {
        self.credential_provider_vendor = ::std::option::Option::Some(input);
        self
    }
    /// <p>The vendor of the OAuth2 credential provider. This specifies which OAuth2 implementation to use.</p>
    pub fn set_credential_provider_vendor(mut self, input: ::std::option::Option<crate::types::CredentialProviderVendorType>) -> Self {
        self.credential_provider_vendor = input;
        self
    }
    /// <p>The vendor of the OAuth2 credential provider. This specifies which OAuth2 implementation to use.</p>
    pub fn get_credential_provider_vendor(&self) -> &::std::option::Option<crate::types::CredentialProviderVendorType> {
        &self.credential_provider_vendor
    }
    /// <p>The configuration settings for the OAuth2 provider, including client ID, client secret, and other vendor-specific settings.</p>
    /// This field is required.
    pub fn oauth2_provider_config_input(mut self, input: crate::types::Oauth2ProviderConfigInput) -> Self {
        self.oauth2_provider_config_input = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration settings for the OAuth2 provider, including client ID, client secret, and other vendor-specific settings.</p>
    pub fn set_oauth2_provider_config_input(mut self, input: ::std::option::Option<crate::types::Oauth2ProviderConfigInput>) -> Self {
        self.oauth2_provider_config_input = input;
        self
    }
    /// <p>The configuration settings for the OAuth2 provider, including client ID, client secret, and other vendor-specific settings.</p>
    pub fn get_oauth2_provider_config_input(&self) -> &::std::option::Option<crate::types::Oauth2ProviderConfigInput> {
        &self.oauth2_provider_config_input
    }
    /// Consumes the builder and constructs a [`CreateOauth2CredentialProviderInput`](crate::operation::create_oauth2_credential_provider::CreateOauth2CredentialProviderInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_oauth2_credential_provider::CreateOauth2CredentialProviderInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_oauth2_credential_provider::CreateOauth2CredentialProviderInput {
            name: self.name,
            credential_provider_vendor: self.credential_provider_vendor,
            oauth2_provider_config_input: self.oauth2_provider_config_input,
        })
    }
}
