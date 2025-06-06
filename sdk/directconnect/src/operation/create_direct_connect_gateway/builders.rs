// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_direct_connect_gateway::_create_direct_connect_gateway_output::CreateDirectConnectGatewayOutputBuilder;

pub use crate::operation::create_direct_connect_gateway::_create_direct_connect_gateway_input::CreateDirectConnectGatewayInputBuilder;

impl crate::operation::create_direct_connect_gateway::builders::CreateDirectConnectGatewayInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_direct_connect_gateway::CreateDirectConnectGatewayOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_direct_connect_gateway::CreateDirectConnectGatewayError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_direct_connect_gateway();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDirectConnectGateway`.
///
/// <p>Creates a Direct Connect gateway, which is an intermediate object that enables you to connect a set of virtual interfaces and virtual private gateways. A Direct Connect gateway is global and visible in any Amazon Web Services Region after it is created. The virtual interfaces and virtual private gateways that are connected through a Direct Connect gateway can be in different Amazon Web Services Regions. This enables you to connect to a VPC in any Region, regardless of the Region in which the virtual interfaces are located, and pass traffic between them.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDirectConnectGatewayFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_direct_connect_gateway::builders::CreateDirectConnectGatewayInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_direct_connect_gateway::CreateDirectConnectGatewayOutput,
        crate::operation::create_direct_connect_gateway::CreateDirectConnectGatewayError,
    > for CreateDirectConnectGatewayFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_direct_connect_gateway::CreateDirectConnectGatewayOutput,
            crate::operation::create_direct_connect_gateway::CreateDirectConnectGatewayError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDirectConnectGatewayFluentBuilder {
    /// Creates a new `CreateDirectConnectGatewayFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDirectConnectGateway as a reference.
    pub fn as_input(&self) -> &crate::operation::create_direct_connect_gateway::builders::CreateDirectConnectGatewayInputBuilder {
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
        crate::operation::create_direct_connect_gateway::CreateDirectConnectGatewayOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_direct_connect_gateway::CreateDirectConnectGatewayError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_direct_connect_gateway::CreateDirectConnectGateway::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_direct_connect_gateway::CreateDirectConnectGateway::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_direct_connect_gateway::CreateDirectConnectGatewayOutput,
        crate::operation::create_direct_connect_gateway::CreateDirectConnectGatewayError,
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
    /// <p>The name of the Direct Connect gateway.</p>
    pub fn direct_connect_gateway_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.direct_connect_gateway_name(input.into());
        self
    }
    /// <p>The name of the Direct Connect gateway.</p>
    pub fn set_direct_connect_gateway_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_direct_connect_gateway_name(input);
        self
    }
    /// <p>The name of the Direct Connect gateway.</p>
    pub fn get_direct_connect_gateway_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_direct_connect_gateway_name()
    }
    ///
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The key-value pair tags associated with the request.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The key-value pair tags associated with the request.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The key-value pair tags associated with the request.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>The autonomous system number (ASN) for Border Gateway Protocol (BGP) to be configured on the Amazon side of the connection. The ASN must be in the private range of 64,512 to 65,534 or 4,200,000,000 to 4,294,967,294. The default is 64512.</p>
    pub fn amazon_side_asn(mut self, input: i64) -> Self {
        self.inner = self.inner.amazon_side_asn(input);
        self
    }
    /// <p>The autonomous system number (ASN) for Border Gateway Protocol (BGP) to be configured on the Amazon side of the connection. The ASN must be in the private range of 64,512 to 65,534 or 4,200,000,000 to 4,294,967,294. The default is 64512.</p>
    pub fn set_amazon_side_asn(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_amazon_side_asn(input);
        self
    }
    /// <p>The autonomous system number (ASN) for Border Gateway Protocol (BGP) to be configured on the Amazon side of the connection. The ASN must be in the private range of 64,512 to 65,534 or 4,200,000,000 to 4,294,967,294. The default is 64512.</p>
    pub fn get_amazon_side_asn(&self) -> &::std::option::Option<i64> {
        self.inner.get_amazon_side_asn()
    }
}
