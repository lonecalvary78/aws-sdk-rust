// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_engagement_invitation::_get_engagement_invitation_output::GetEngagementInvitationOutputBuilder;

pub use crate::operation::get_engagement_invitation::_get_engagement_invitation_input::GetEngagementInvitationInputBuilder;

impl crate::operation::get_engagement_invitation::builders::GetEngagementInvitationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_engagement_invitation::GetEngagementInvitationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_engagement_invitation::GetEngagementInvitationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_engagement_invitation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetEngagementInvitation`.
///
/// <p>Retrieves the details of an engagement invitation shared by AWS with a partner. The information includes aspects such as customer, project details, and lifecycle information. To connect an engagement invitation with an opportunity, match the invitation’s <code>Payload.Project.Title</code> with opportunity <code>Project.Title</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetEngagementInvitationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_engagement_invitation::builders::GetEngagementInvitationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_engagement_invitation::GetEngagementInvitationOutput,
        crate::operation::get_engagement_invitation::GetEngagementInvitationError,
    > for GetEngagementInvitationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_engagement_invitation::GetEngagementInvitationOutput,
            crate::operation::get_engagement_invitation::GetEngagementInvitationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetEngagementInvitationFluentBuilder {
    /// Creates a new `GetEngagementInvitationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetEngagementInvitation as a reference.
    pub fn as_input(&self) -> &crate::operation::get_engagement_invitation::builders::GetEngagementInvitationInputBuilder {
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
        crate::operation::get_engagement_invitation::GetEngagementInvitationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_engagement_invitation::GetEngagementInvitationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_engagement_invitation::GetEngagementInvitation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_engagement_invitation::GetEngagementInvitation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_engagement_invitation::GetEngagementInvitationOutput,
        crate::operation::get_engagement_invitation::GetEngagementInvitationError,
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
    /// <p>Specifies the catalog associated with the request. The field accepts values from the predefined set: <code>AWS</code> for live operations or <code>Sandbox</code> for testing environments.</p>
    pub fn catalog(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog(input.into());
        self
    }
    /// <p>Specifies the catalog associated with the request. The field accepts values from the predefined set: <code>AWS</code> for live operations or <code>Sandbox</code> for testing environments.</p>
    pub fn set_catalog(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog(input);
        self
    }
    /// <p>Specifies the catalog associated with the request. The field accepts values from the predefined set: <code>AWS</code> for live operations or <code>Sandbox</code> for testing environments.</p>
    pub fn get_catalog(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_catalog()
    }
    /// <p>Specifies the unique identifier for the retrieved engagement invitation.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>Specifies the unique identifier for the retrieved engagement invitation.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
    /// <p>Specifies the unique identifier for the retrieved engagement invitation.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }
}
