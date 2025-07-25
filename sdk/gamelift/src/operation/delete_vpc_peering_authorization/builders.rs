// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_vpc_peering_authorization::_delete_vpc_peering_authorization_output::DeleteVpcPeeringAuthorizationOutputBuilder;

pub use crate::operation::delete_vpc_peering_authorization::_delete_vpc_peering_authorization_input::DeleteVpcPeeringAuthorizationInputBuilder;

impl crate::operation::delete_vpc_peering_authorization::builders::DeleteVpcPeeringAuthorizationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_vpc_peering_authorization();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteVpcPeeringAuthorization`.
///
/// <p>Cancels a pending VPC peering authorization for the specified VPC. If you need to delete an existing VPC peering connection, use <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_DeleteVpcPeeringConnection.html">DeleteVpcPeeringConnection</a>.</p>
/// <p><b>Related actions</b></p>
/// <p><a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/reference-awssdk.html#reference-awssdk-resources-fleets">All APIs by task</a></p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteVpcPeeringAuthorizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_vpc_peering_authorization::builders::DeleteVpcPeeringAuthorizationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorizationOutput,
        crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorizationError,
    > for DeleteVpcPeeringAuthorizationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorizationOutput,
            crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorizationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteVpcPeeringAuthorizationFluentBuilder {
    /// Creates a new `DeleteVpcPeeringAuthorizationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteVpcPeeringAuthorization as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_vpc_peering_authorization::builders::DeleteVpcPeeringAuthorizationInputBuilder {
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
        crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorization::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorization::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorizationOutput,
        crate::operation::delete_vpc_peering_authorization::DeleteVpcPeeringAuthorizationError,
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
    /// <p>A unique identifier for the Amazon Web Services account that you use to manage your Amazon GameLift Servers fleet. You can find your Account ID in the Amazon Web Services Management Console under account settings.</p>
    pub fn game_lift_aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.game_lift_aws_account_id(input.into());
        self
    }
    /// <p>A unique identifier for the Amazon Web Services account that you use to manage your Amazon GameLift Servers fleet. You can find your Account ID in the Amazon Web Services Management Console under account settings.</p>
    pub fn set_game_lift_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_game_lift_aws_account_id(input);
        self
    }
    /// <p>A unique identifier for the Amazon Web Services account that you use to manage your Amazon GameLift Servers fleet. You can find your Account ID in the Amazon Web Services Management Console under account settings.</p>
    pub fn get_game_lift_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_game_lift_aws_account_id()
    }
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p>
    pub fn peer_vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.peer_vpc_id(input.into());
        self
    }
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p>
    pub fn set_peer_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_peer_vpc_id(input);
        self
    }
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift Servers fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the Amazon Web Services Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Servers Fleets</a>.</p>
    pub fn get_peer_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_peer_vpc_id()
    }
}
