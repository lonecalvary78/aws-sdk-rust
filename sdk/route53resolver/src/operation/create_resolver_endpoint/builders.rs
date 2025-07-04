// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_resolver_endpoint::_create_resolver_endpoint_output::CreateResolverEndpointOutputBuilder;

pub use crate::operation::create_resolver_endpoint::_create_resolver_endpoint_input::CreateResolverEndpointInputBuilder;

impl crate::operation::create_resolver_endpoint::builders::CreateResolverEndpointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_resolver_endpoint::CreateResolverEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_resolver_endpoint::CreateResolverEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_resolver_endpoint();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateResolverEndpoint`.
///
/// <p>Creates a Resolver endpoint. There are two types of Resolver endpoints, inbound and outbound:</p>
/// <ul>
/// <li>
/// <p>An <i>inbound Resolver endpoint</i> forwards DNS queries to the DNS service for a VPC from your network.</p></li>
/// <li>
/// <p>An <i>outbound Resolver endpoint</i> forwards DNS queries from the DNS service for a VPC to your network.</p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateResolverEndpointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_resolver_endpoint::builders::CreateResolverEndpointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_resolver_endpoint::CreateResolverEndpointOutput,
        crate::operation::create_resolver_endpoint::CreateResolverEndpointError,
    > for CreateResolverEndpointFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_resolver_endpoint::CreateResolverEndpointOutput,
            crate::operation::create_resolver_endpoint::CreateResolverEndpointError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateResolverEndpointFluentBuilder {
    /// Creates a new `CreateResolverEndpointFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateResolverEndpoint as a reference.
    pub fn as_input(&self) -> &crate::operation::create_resolver_endpoint::builders::CreateResolverEndpointInputBuilder {
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
        crate::operation::create_resolver_endpoint::CreateResolverEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_resolver_endpoint::CreateResolverEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_resolver_endpoint::CreateResolverEndpoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_resolver_endpoint::CreateResolverEndpoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_resolver_endpoint::CreateResolverEndpointOutput,
        crate::operation::create_resolver_endpoint::CreateResolverEndpointError,
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
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p>
    pub fn creator_request_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.creator_request_id(input.into());
        self
    }
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p>
    pub fn set_creator_request_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_creator_request_id(input);
        self
    }
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string, for example, a date/time stamp.</p>
    pub fn get_creator_request_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_creator_request_id()
    }
    /// <p>A friendly name that lets you easily find a configuration in the Resolver dashboard in the Route 53 console.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A friendly name that lets you easily find a configuration in the Resolver dashboard in the Route 53 console.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A friendly name that lets you easily find a configuration in the Resolver dashboard in the Route 53 console.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    ///
    /// Appends an item to `SecurityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>The ID of one or more security groups that you want to use to control access to this VPC. The security group that you specify must include one or more inbound rules (for inbound Resolver endpoints) or outbound rules (for outbound Resolver endpoints). Inbound and outbound rules must allow TCP and UDP access. For inbound access, open port 53. For outbound access, open the port that you're using for DNS queries on your network.</p>
    /// <p>Some security group rules will cause your connection to be tracked. For outbound resolver endpoint, it can potentially impact the maximum queries per second from outbound endpoint to your target name server. For inbound resolver endpoint, it can bring down the overall maximum queries per second per IP address to as low as 1500. To avoid connection tracking caused by security group, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/security-group-connection-tracking.html#untracked-connectionsl">Untracked connections</a>.</p>
    pub fn security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.security_group_ids(input.into());
        self
    }
    /// <p>The ID of one or more security groups that you want to use to control access to this VPC. The security group that you specify must include one or more inbound rules (for inbound Resolver endpoints) or outbound rules (for outbound Resolver endpoints). Inbound and outbound rules must allow TCP and UDP access. For inbound access, open port 53. For outbound access, open the port that you're using for DNS queries on your network.</p>
    /// <p>Some security group rules will cause your connection to be tracked. For outbound resolver endpoint, it can potentially impact the maximum queries per second from outbound endpoint to your target name server. For inbound resolver endpoint, it can bring down the overall maximum queries per second per IP address to as low as 1500. To avoid connection tracking caused by security group, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/security-group-connection-tracking.html#untracked-connectionsl">Untracked connections</a>.</p>
    pub fn set_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_security_group_ids(input);
        self
    }
    /// <p>The ID of one or more security groups that you want to use to control access to this VPC. The security group that you specify must include one or more inbound rules (for inbound Resolver endpoints) or outbound rules (for outbound Resolver endpoints). Inbound and outbound rules must allow TCP and UDP access. For inbound access, open port 53. For outbound access, open the port that you're using for DNS queries on your network.</p>
    /// <p>Some security group rules will cause your connection to be tracked. For outbound resolver endpoint, it can potentially impact the maximum queries per second from outbound endpoint to your target name server. For inbound resolver endpoint, it can bring down the overall maximum queries per second per IP address to as low as 1500. To avoid connection tracking caused by security group, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/security-group-connection-tracking.html#untracked-connectionsl">Untracked connections</a>.</p>
    pub fn get_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_security_group_ids()
    }
    /// <p>Specify the applicable value:</p>
    /// <ul>
    /// <li>
    /// <p><code>INBOUND</code>: Resolver forwards DNS queries to the DNS service for a VPC from your network.</p></li>
    /// <li>
    /// <p><code>OUTBOUND</code>: Resolver forwards DNS queries from the DNS service for a VPC to your network.</p></li>
    /// <li>
    /// <p><code>INBOUND_DELEGATION</code>: Resolver delegates queries to Route 53 private hosted zones from your network.</p></li>
    /// </ul>
    pub fn direction(mut self, input: crate::types::ResolverEndpointDirection) -> Self {
        self.inner = self.inner.direction(input);
        self
    }
    /// <p>Specify the applicable value:</p>
    /// <ul>
    /// <li>
    /// <p><code>INBOUND</code>: Resolver forwards DNS queries to the DNS service for a VPC from your network.</p></li>
    /// <li>
    /// <p><code>OUTBOUND</code>: Resolver forwards DNS queries from the DNS service for a VPC to your network.</p></li>
    /// <li>
    /// <p><code>INBOUND_DELEGATION</code>: Resolver delegates queries to Route 53 private hosted zones from your network.</p></li>
    /// </ul>
    pub fn set_direction(mut self, input: ::std::option::Option<crate::types::ResolverEndpointDirection>) -> Self {
        self.inner = self.inner.set_direction(input);
        self
    }
    /// <p>Specify the applicable value:</p>
    /// <ul>
    /// <li>
    /// <p><code>INBOUND</code>: Resolver forwards DNS queries to the DNS service for a VPC from your network.</p></li>
    /// <li>
    /// <p><code>OUTBOUND</code>: Resolver forwards DNS queries from the DNS service for a VPC to your network.</p></li>
    /// <li>
    /// <p><code>INBOUND_DELEGATION</code>: Resolver delegates queries to Route 53 private hosted zones from your network.</p></li>
    /// </ul>
    pub fn get_direction(&self) -> &::std::option::Option<crate::types::ResolverEndpointDirection> {
        self.inner.get_direction()
    }
    ///
    /// Appends an item to `IpAddresses`.
    ///
    /// To override the contents of this collection use [`set_ip_addresses`](Self::set_ip_addresses).
    ///
    /// <p>The subnets and IP addresses in your VPC that DNS queries originate from (for outbound endpoints) or that you forward DNS queries to (for inbound endpoints). The subnet ID uniquely identifies a VPC.</p><note>
    /// <p>Even though the minimum is 1, Route&nbsp;53 requires that you create at least two.</p>
    /// </note>
    pub fn ip_addresses(mut self, input: crate::types::IpAddressRequest) -> Self {
        self.inner = self.inner.ip_addresses(input);
        self
    }
    /// <p>The subnets and IP addresses in your VPC that DNS queries originate from (for outbound endpoints) or that you forward DNS queries to (for inbound endpoints). The subnet ID uniquely identifies a VPC.</p><note>
    /// <p>Even though the minimum is 1, Route&nbsp;53 requires that you create at least two.</p>
    /// </note>
    pub fn set_ip_addresses(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::IpAddressRequest>>) -> Self {
        self.inner = self.inner.set_ip_addresses(input);
        self
    }
    /// <p>The subnets and IP addresses in your VPC that DNS queries originate from (for outbound endpoints) or that you forward DNS queries to (for inbound endpoints). The subnet ID uniquely identifies a VPC.</p><note>
    /// <p>Even though the minimum is 1, Route&nbsp;53 requires that you create at least two.</p>
    /// </note>
    pub fn get_ip_addresses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::IpAddressRequest>> {
        self.inner.get_ip_addresses()
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost. If you specify this, you must also specify a value for the <code>PreferredInstanceType</code>.</p>
    pub fn outpost_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.outpost_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost. If you specify this, you must also specify a value for the <code>PreferredInstanceType</code>.</p>
    pub fn set_outpost_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_outpost_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost. If you specify this, you must also specify a value for the <code>PreferredInstanceType</code>.</p>
    pub fn get_outpost_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_outpost_arn()
    }
    /// <p>The instance type. If you specify this, you must also specify a value for the <code>OutpostArn</code>.</p>
    pub fn preferred_instance_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.preferred_instance_type(input.into());
        self
    }
    /// <p>The instance type. If you specify this, you must also specify a value for the <code>OutpostArn</code>.</p>
    pub fn set_preferred_instance_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_preferred_instance_type(input);
        self
    }
    /// <p>The instance type. If you specify this, you must also specify a value for the <code>OutpostArn</code>.</p>
    pub fn get_preferred_instance_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_preferred_instance_type()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of the tag keys and values that you want to associate with the endpoint.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of the tag keys and values that you want to associate with the endpoint.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of the tag keys and values that you want to associate with the endpoint.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>For the endpoint type you can choose either IPv4, IPv6, or dual-stack. A dual-stack endpoint means that it will resolve via both IPv4 and IPv6. This endpoint type is applied to all IP addresses.</p>
    pub fn resolver_endpoint_type(mut self, input: crate::types::ResolverEndpointType) -> Self {
        self.inner = self.inner.resolver_endpoint_type(input);
        self
    }
    /// <p>For the endpoint type you can choose either IPv4, IPv6, or dual-stack. A dual-stack endpoint means that it will resolve via both IPv4 and IPv6. This endpoint type is applied to all IP addresses.</p>
    pub fn set_resolver_endpoint_type(mut self, input: ::std::option::Option<crate::types::ResolverEndpointType>) -> Self {
        self.inner = self.inner.set_resolver_endpoint_type(input);
        self
    }
    /// <p>For the endpoint type you can choose either IPv4, IPv6, or dual-stack. A dual-stack endpoint means that it will resolve via both IPv4 and IPv6. This endpoint type is applied to all IP addresses.</p>
    pub fn get_resolver_endpoint_type(&self) -> &::std::option::Option<crate::types::ResolverEndpointType> {
        self.inner.get_resolver_endpoint_type()
    }
    ///
    /// Appends an item to `Protocols`.
    ///
    /// To override the contents of this collection use [`set_protocols`](Self::set_protocols).
    ///
    /// <p>The protocols you want to use for the endpoint. DoH-FIPS is applicable for default inbound endpoints only.</p>
    /// <p>For a default inbound endpoint you can apply the protocols as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Do53 and DoH in combination.</p></li>
    /// <li>
    /// <p>Do53 and DoH-FIPS in combination.</p></li>
    /// <li>
    /// <p>Do53 alone.</p></li>
    /// <li>
    /// <p>DoH alone.</p></li>
    /// <li>
    /// <p>DoH-FIPS alone.</p></li>
    /// <li>
    /// <p>None, which is treated as Do53.</p></li>
    /// </ul>
    /// <p>For a delegation inbound endpoint you can use Do53 only.</p>
    /// <p>For an outbound endpoint you can apply the protocols as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Do53 and DoH in combination.</p></li>
    /// <li>
    /// <p>Do53 alone.</p></li>
    /// <li>
    /// <p>DoH alone.</p></li>
    /// <li>
    /// <p>None, which is treated as Do53.</p></li>
    /// </ul>
    pub fn protocols(mut self, input: crate::types::Protocol) -> Self {
        self.inner = self.inner.protocols(input);
        self
    }
    /// <p>The protocols you want to use for the endpoint. DoH-FIPS is applicable for default inbound endpoints only.</p>
    /// <p>For a default inbound endpoint you can apply the protocols as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Do53 and DoH in combination.</p></li>
    /// <li>
    /// <p>Do53 and DoH-FIPS in combination.</p></li>
    /// <li>
    /// <p>Do53 alone.</p></li>
    /// <li>
    /// <p>DoH alone.</p></li>
    /// <li>
    /// <p>DoH-FIPS alone.</p></li>
    /// <li>
    /// <p>None, which is treated as Do53.</p></li>
    /// </ul>
    /// <p>For a delegation inbound endpoint you can use Do53 only.</p>
    /// <p>For an outbound endpoint you can apply the protocols as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Do53 and DoH in combination.</p></li>
    /// <li>
    /// <p>Do53 alone.</p></li>
    /// <li>
    /// <p>DoH alone.</p></li>
    /// <li>
    /// <p>None, which is treated as Do53.</p></li>
    /// </ul>
    pub fn set_protocols(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Protocol>>) -> Self {
        self.inner = self.inner.set_protocols(input);
        self
    }
    /// <p>The protocols you want to use for the endpoint. DoH-FIPS is applicable for default inbound endpoints only.</p>
    /// <p>For a default inbound endpoint you can apply the protocols as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Do53 and DoH in combination.</p></li>
    /// <li>
    /// <p>Do53 and DoH-FIPS in combination.</p></li>
    /// <li>
    /// <p>Do53 alone.</p></li>
    /// <li>
    /// <p>DoH alone.</p></li>
    /// <li>
    /// <p>DoH-FIPS alone.</p></li>
    /// <li>
    /// <p>None, which is treated as Do53.</p></li>
    /// </ul>
    /// <p>For a delegation inbound endpoint you can use Do53 only.</p>
    /// <p>For an outbound endpoint you can apply the protocols as follows:</p>
    /// <ul>
    /// <li>
    /// <p>Do53 and DoH in combination.</p></li>
    /// <li>
    /// <p>Do53 alone.</p></li>
    /// <li>
    /// <p>DoH alone.</p></li>
    /// <li>
    /// <p>None, which is treated as Do53.</p></li>
    /// </ul>
    pub fn get_protocols(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Protocol>> {
        self.inner.get_protocols()
    }
}
