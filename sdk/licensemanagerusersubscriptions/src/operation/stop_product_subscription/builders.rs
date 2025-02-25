// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_product_subscription::_stop_product_subscription_output::StopProductSubscriptionOutputBuilder;

pub use crate::operation::stop_product_subscription::_stop_product_subscription_input::StopProductSubscriptionInputBuilder;

impl crate::operation::stop_product_subscription::builders::StopProductSubscriptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::stop_product_subscription::StopProductSubscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_product_subscription::StopProductSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.stop_product_subscription();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StopProductSubscription`.
///
/// <p>Stops a product subscription for a user with the specified identity provider.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StopProductSubscriptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::stop_product_subscription::builders::StopProductSubscriptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::stop_product_subscription::StopProductSubscriptionOutput,
        crate::operation::stop_product_subscription::StopProductSubscriptionError,
    > for StopProductSubscriptionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::stop_product_subscription::StopProductSubscriptionOutput,
            crate::operation::stop_product_subscription::StopProductSubscriptionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StopProductSubscriptionFluentBuilder {
    /// Creates a new `StopProductSubscriptionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StopProductSubscription as a reference.
    pub fn as_input(&self) -> &crate::operation::stop_product_subscription::builders::StopProductSubscriptionInputBuilder {
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
        crate::operation::stop_product_subscription::StopProductSubscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_product_subscription::StopProductSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::stop_product_subscription::StopProductSubscription::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::stop_product_subscription::StopProductSubscription::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::stop_product_subscription::StopProductSubscriptionOutput,
        crate::operation::stop_product_subscription::StopProductSubscriptionError,
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
    /// <p>The user name from the identity provider for the user.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The user name from the identity provider for the user.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
    /// <p>The user name from the identity provider for the user.</p>
    pub fn get_username(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_username()
    }
    /// <p>An object that specifies details for the identity provider.</p>
    pub fn identity_provider(mut self, input: crate::types::IdentityProvider) -> Self {
        self.inner = self.inner.identity_provider(input);
        self
    }
    /// <p>An object that specifies details for the identity provider.</p>
    pub fn set_identity_provider(mut self, input: ::std::option::Option<crate::types::IdentityProvider>) -> Self {
        self.inner = self.inner.set_identity_provider(input);
        self
    }
    /// <p>An object that specifies details for the identity provider.</p>
    pub fn get_identity_provider(&self) -> &::std::option::Option<crate::types::IdentityProvider> {
        self.inner.get_identity_provider()
    }
    /// <p>The name of the user-based subscription product.</p>
    /// <p>Valid values: <code>VISUAL_STUDIO_ENTERPRISE</code> | <code>VISUAL_STUDIO_PROFESSIONAL</code> | <code>OFFICE_PROFESSIONAL_PLUS</code> | <code>REMOTE_DESKTOP_SERVICES</code></p>
    pub fn product(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.product(input.into());
        self
    }
    /// <p>The name of the user-based subscription product.</p>
    /// <p>Valid values: <code>VISUAL_STUDIO_ENTERPRISE</code> | <code>VISUAL_STUDIO_PROFESSIONAL</code> | <code>OFFICE_PROFESSIONAL_PLUS</code> | <code>REMOTE_DESKTOP_SERVICES</code></p>
    pub fn set_product(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_product(input);
        self
    }
    /// <p>The name of the user-based subscription product.</p>
    /// <p>Valid values: <code>VISUAL_STUDIO_ENTERPRISE</code> | <code>VISUAL_STUDIO_PROFESSIONAL</code> | <code>OFFICE_PROFESSIONAL_PLUS</code> | <code>REMOTE_DESKTOP_SERVICES</code></p>
    pub fn get_product(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_product()
    }
    /// <p>The Amazon Resource Name (ARN) of the product user.</p>
    pub fn product_user_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.product_user_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the product user.</p>
    pub fn set_product_user_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_product_user_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the product user.</p>
    pub fn get_product_user_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_product_user_arn()
    }
    /// <p>The domain name of the Active Directory that contains the user for whom to stop the product subscription.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p>The domain name of the Active Directory that contains the user for whom to stop the product subscription.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// <p>The domain name of the Active Directory that contains the user for whom to stop the product subscription.</p>
    pub fn get_domain(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain()
    }
}
