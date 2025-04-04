// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_ip_address_type::_set_ip_address_type_output::SetIpAddressTypeOutputBuilder;

pub use crate::operation::set_ip_address_type::_set_ip_address_type_input::SetIpAddressTypeInputBuilder;

impl crate::operation::set_ip_address_type::builders::SetIpAddressTypeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_ip_address_type::SetIpAddressTypeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_ip_address_type::SetIpAddressTypeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_ip_address_type();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetIpAddressType`.
///
/// <p>Sets the type of IP addresses used by the subnets of the specified load balancer.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetIpAddressTypeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_ip_address_type::builders::SetIpAddressTypeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_ip_address_type::SetIpAddressTypeOutput,
        crate::operation::set_ip_address_type::SetIpAddressTypeError,
    > for SetIpAddressTypeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_ip_address_type::SetIpAddressTypeOutput,
            crate::operation::set_ip_address_type::SetIpAddressTypeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetIpAddressTypeFluentBuilder {
    /// Creates a new `SetIpAddressTypeFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetIpAddressType as a reference.
    pub fn as_input(&self) -> &crate::operation::set_ip_address_type::builders::SetIpAddressTypeInputBuilder {
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
        crate::operation::set_ip_address_type::SetIpAddressTypeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_ip_address_type::SetIpAddressTypeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::set_ip_address_type::SetIpAddressType::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::set_ip_address_type::SetIpAddressType::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::set_ip_address_type::SetIpAddressTypeOutput,
        crate::operation::set_ip_address_type::SetIpAddressTypeError,
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
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub fn load_balancer_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.load_balancer_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub fn set_load_balancer_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_load_balancer_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    pub fn get_load_balancer_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_load_balancer_arn()
    }
    /// <p>The IP address type. Internal load balancers must use <code>ipv4</code>.</p>
    /// <p>\[Application Load Balancers\] The possible values are ipv4 (IPv4 addresses), dualstack (IPv4 and IPv6 addresses), and dualstack-without-public-ipv4 (public IPv6 addresses and private IPv4 and IPv6 addresses).</p>
    /// <p>Application Load Balancer authentication supports IPv4 addresses only when connecting to an Identity Provider (IdP) or Amazon Cognito endpoint. Without a public IPv4 address the load balancer can't complete the authentication process, resulting in HTTP 500 errors.</p>
    /// <p>\[Network Load Balancers and Gateway Load Balancers\] The possible values are ipv4 (IPv4 addresses) and dualstack (IPv4 and IPv6 addresses).</p>
    pub fn ip_address_type(mut self, input: crate::types::IpAddressType) -> Self {
        self.inner = self.inner.ip_address_type(input);
        self
    }
    /// <p>The IP address type. Internal load balancers must use <code>ipv4</code>.</p>
    /// <p>\[Application Load Balancers\] The possible values are ipv4 (IPv4 addresses), dualstack (IPv4 and IPv6 addresses), and dualstack-without-public-ipv4 (public IPv6 addresses and private IPv4 and IPv6 addresses).</p>
    /// <p>Application Load Balancer authentication supports IPv4 addresses only when connecting to an Identity Provider (IdP) or Amazon Cognito endpoint. Without a public IPv4 address the load balancer can't complete the authentication process, resulting in HTTP 500 errors.</p>
    /// <p>\[Network Load Balancers and Gateway Load Balancers\] The possible values are ipv4 (IPv4 addresses) and dualstack (IPv4 and IPv6 addresses).</p>
    pub fn set_ip_address_type(mut self, input: ::std::option::Option<crate::types::IpAddressType>) -> Self {
        self.inner = self.inner.set_ip_address_type(input);
        self
    }
    /// <p>The IP address type. Internal load balancers must use <code>ipv4</code>.</p>
    /// <p>\[Application Load Balancers\] The possible values are ipv4 (IPv4 addresses), dualstack (IPv4 and IPv6 addresses), and dualstack-without-public-ipv4 (public IPv6 addresses and private IPv4 and IPv6 addresses).</p>
    /// <p>Application Load Balancer authentication supports IPv4 addresses only when connecting to an Identity Provider (IdP) or Amazon Cognito endpoint. Without a public IPv4 address the load balancer can't complete the authentication process, resulting in HTTP 500 errors.</p>
    /// <p>\[Network Load Balancers and Gateway Load Balancers\] The possible values are ipv4 (IPv4 addresses) and dualstack (IPv4 and IPv6 addresses).</p>
    pub fn get_ip_address_type(&self) -> &::std::option::Option<crate::types::IpAddressType> {
        self.inner.get_ip_address_type()
    }
}
