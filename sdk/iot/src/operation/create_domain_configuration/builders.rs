// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_domain_configuration::_create_domain_configuration_output::CreateDomainConfigurationOutputBuilder;

pub use crate::operation::create_domain_configuration::_create_domain_configuration_input::CreateDomainConfigurationInputBuilder;

impl crate::operation::create_domain_configuration::builders::CreateDomainConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_domain_configuration::CreateDomainConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_domain_configuration::CreateDomainConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_domain_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDomainConfiguration`.
///
/// <p>Creates a domain configuration.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">CreateDomainConfiguration</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDomainConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_domain_configuration::builders::CreateDomainConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_domain_configuration::CreateDomainConfigurationOutput,
        crate::operation::create_domain_configuration::CreateDomainConfigurationError,
    > for CreateDomainConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_domain_configuration::CreateDomainConfigurationOutput,
            crate::operation::create_domain_configuration::CreateDomainConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDomainConfigurationFluentBuilder {
    /// Creates a new `CreateDomainConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDomainConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::create_domain_configuration::builders::CreateDomainConfigurationInputBuilder {
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
        crate::operation::create_domain_configuration::CreateDomainConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_domain_configuration::CreateDomainConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_domain_configuration::CreateDomainConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_domain_configuration::CreateDomainConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_domain_configuration::CreateDomainConfigurationOutput,
        crate::operation::create_domain_configuration::CreateDomainConfigurationError,
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
    /// <p>The name of the domain configuration. This value must be unique to a region.</p>
    pub fn domain_configuration_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_configuration_name(input.into());
        self
    }
    /// <p>The name of the domain configuration. This value must be unique to a region.</p>
    pub fn set_domain_configuration_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_configuration_name(input);
        self
    }
    /// <p>The name of the domain configuration. This value must be unique to a region.</p>
    pub fn get_domain_configuration_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_configuration_name()
    }
    /// <p>The name of the domain.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the domain.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the domain.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    ///
    /// Appends an item to `serverCertificateArns`.
    ///
    /// To override the contents of this collection use [`set_server_certificate_arns`](Self::set_server_certificate_arns).
    ///
    /// <p>The ARNs of the certificates that IoT passes to the device during the TLS handshake. Currently you can specify only one certificate ARN. This value is not required for Amazon Web Services-managed domains.</p>
    pub fn server_certificate_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.server_certificate_arns(input.into());
        self
    }
    /// <p>The ARNs of the certificates that IoT passes to the device during the TLS handshake. Currently you can specify only one certificate ARN. This value is not required for Amazon Web Services-managed domains.</p>
    pub fn set_server_certificate_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_server_certificate_arns(input);
        self
    }
    /// <p>The ARNs of the certificates that IoT passes to the device during the TLS handshake. Currently you can specify only one certificate ARN. This value is not required for Amazon Web Services-managed domains.</p>
    pub fn get_server_certificate_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_server_certificate_arns()
    }
    /// <p>The certificate used to validate the server certificate and prove domain name ownership. This certificate must be signed by a public certificate authority. This value is not required for Amazon Web Services-managed domains.</p>
    pub fn validation_certificate_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.validation_certificate_arn(input.into());
        self
    }
    /// <p>The certificate used to validate the server certificate and prove domain name ownership. This certificate must be signed by a public certificate authority. This value is not required for Amazon Web Services-managed domains.</p>
    pub fn set_validation_certificate_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_validation_certificate_arn(input);
        self
    }
    /// <p>The certificate used to validate the server certificate and prove domain name ownership. This certificate must be signed by a public certificate authority. This value is not required for Amazon Web Services-managed domains.</p>
    pub fn get_validation_certificate_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_validation_certificate_arn()
    }
    /// <p>An object that specifies the authorization service for a domain.</p>
    pub fn authorizer_config(mut self, input: crate::types::AuthorizerConfig) -> Self {
        self.inner = self.inner.authorizer_config(input);
        self
    }
    /// <p>An object that specifies the authorization service for a domain.</p>
    pub fn set_authorizer_config(mut self, input: ::std::option::Option<crate::types::AuthorizerConfig>) -> Self {
        self.inner = self.inner.set_authorizer_config(input);
        self
    }
    /// <p>An object that specifies the authorization service for a domain.</p>
    pub fn get_authorizer_config(&self) -> &::std::option::Option<crate::types::AuthorizerConfig> {
        self.inner.get_authorizer_config()
    }
    /// <p>The type of service delivered by the endpoint.</p><note>
    /// <p>Amazon Web Services IoT Core currently supports only the <code>DATA</code> service type.</p>
    /// </note>
    pub fn service_type(mut self, input: crate::types::ServiceType) -> Self {
        self.inner = self.inner.service_type(input);
        self
    }
    /// <p>The type of service delivered by the endpoint.</p><note>
    /// <p>Amazon Web Services IoT Core currently supports only the <code>DATA</code> service type.</p>
    /// </note>
    pub fn set_service_type(mut self, input: ::std::option::Option<crate::types::ServiceType>) -> Self {
        self.inner = self.inner.set_service_type(input);
        self
    }
    /// <p>The type of service delivered by the endpoint.</p><note>
    /// <p>Amazon Web Services IoT Core currently supports only the <code>DATA</code> service type.</p>
    /// </note>
    pub fn get_service_type(&self) -> &::std::option::Option<crate::types::ServiceType> {
        self.inner.get_service_type()
    }
    ///
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Metadata which can be used to manage the domain configuration.</p><note>
    /// <p>For URI Request parameters use format: ...key1=value1&amp;key2=value2...</p>
    /// <p>For the CLI command-line parameter use format: &amp;&amp;tags "key1=value1&amp;key2=value2..."</p>
    /// <p>For the cli-input-json file use format: "tags": "key1=value1&amp;key2=value2..."</p>
    /// </note>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Metadata which can be used to manage the domain configuration.</p><note>
    /// <p>For URI Request parameters use format: ...key1=value1&amp;key2=value2...</p>
    /// <p>For the CLI command-line parameter use format: &amp;&amp;tags "key1=value1&amp;key2=value2..."</p>
    /// <p>For the cli-input-json file use format: "tags": "key1=value1&amp;key2=value2..."</p>
    /// </note>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Metadata which can be used to manage the domain configuration.</p><note>
    /// <p>For URI Request parameters use format: ...key1=value1&amp;key2=value2...</p>
    /// <p>For the CLI command-line parameter use format: &amp;&amp;tags "key1=value1&amp;key2=value2..."</p>
    /// <p>For the cli-input-json file use format: "tags": "key1=value1&amp;key2=value2..."</p>
    /// </note>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>An object that specifies the TLS configuration for a domain.</p>
    pub fn tls_config(mut self, input: crate::types::TlsConfig) -> Self {
        self.inner = self.inner.tls_config(input);
        self
    }
    /// <p>An object that specifies the TLS configuration for a domain.</p>
    pub fn set_tls_config(mut self, input: ::std::option::Option<crate::types::TlsConfig>) -> Self {
        self.inner = self.inner.set_tls_config(input);
        self
    }
    /// <p>An object that specifies the TLS configuration for a domain.</p>
    pub fn get_tls_config(&self) -> &::std::option::Option<crate::types::TlsConfig> {
        self.inner.get_tls_config()
    }
    /// <p>The server certificate configuration.</p>
    pub fn server_certificate_config(mut self, input: crate::types::ServerCertificateConfig) -> Self {
        self.inner = self.inner.server_certificate_config(input);
        self
    }
    /// <p>The server certificate configuration.</p>
    pub fn set_server_certificate_config(mut self, input: ::std::option::Option<crate::types::ServerCertificateConfig>) -> Self {
        self.inner = self.inner.set_server_certificate_config(input);
        self
    }
    /// <p>The server certificate configuration.</p>
    pub fn get_server_certificate_config(&self) -> &::std::option::Option<crate::types::ServerCertificateConfig> {
        self.inner.get_server_certificate_config()
    }
    /// <p>An enumerated string that speciﬁes the authentication type.</p>
    /// <ul>
    /// <li>
    /// <p><code>CUSTOM_AUTH_X509</code> - Use custom authentication and authorization with additional details from the X.509 client certificate.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>CUSTOM_AUTH</code> - Use custom authentication and authorization. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/custom-authentication.html">Custom authentication and authorization</a>.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>AWS_X509</code> - Use X.509 client certificates without custom authentication and authorization. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/x509-client-certs.html">X.509 client certificates</a>.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>AWS_SIGV4</code> - Use Amazon Web Services Signature Version 4. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/custom-authentication.html">IAM users, groups, and roles</a>.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT</code> - Use a combination of port and Application Layer Protocol Negotiation (ALPN) to specify authentication type. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/protocols.html">Device communication protocols</a>.</p></li>
    /// </ul>
    pub fn authentication_type(mut self, input: crate::types::AuthenticationType) -> Self {
        self.inner = self.inner.authentication_type(input);
        self
    }
    /// <p>An enumerated string that speciﬁes the authentication type.</p>
    /// <ul>
    /// <li>
    /// <p><code>CUSTOM_AUTH_X509</code> - Use custom authentication and authorization with additional details from the X.509 client certificate.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>CUSTOM_AUTH</code> - Use custom authentication and authorization. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/custom-authentication.html">Custom authentication and authorization</a>.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>AWS_X509</code> - Use X.509 client certificates without custom authentication and authorization. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/x509-client-certs.html">X.509 client certificates</a>.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>AWS_SIGV4</code> - Use Amazon Web Services Signature Version 4. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/custom-authentication.html">IAM users, groups, and roles</a>.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT</code> - Use a combination of port and Application Layer Protocol Negotiation (ALPN) to specify authentication type. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/protocols.html">Device communication protocols</a>.</p></li>
    /// </ul>
    pub fn set_authentication_type(mut self, input: ::std::option::Option<crate::types::AuthenticationType>) -> Self {
        self.inner = self.inner.set_authentication_type(input);
        self
    }
    /// <p>An enumerated string that speciﬁes the authentication type.</p>
    /// <ul>
    /// <li>
    /// <p><code>CUSTOM_AUTH_X509</code> - Use custom authentication and authorization with additional details from the X.509 client certificate.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>CUSTOM_AUTH</code> - Use custom authentication and authorization. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/custom-authentication.html">Custom authentication and authorization</a>.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>AWS_X509</code> - Use X.509 client certificates without custom authentication and authorization. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/x509-client-certs.html">X.509 client certificates</a>.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>AWS_SIGV4</code> - Use Amazon Web Services Signature Version 4. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/custom-authentication.html">IAM users, groups, and roles</a>.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT</code> - Use a combination of port and Application Layer Protocol Negotiation (ALPN) to specify authentication type. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/protocols.html">Device communication protocols</a>.</p></li>
    /// </ul>
    pub fn get_authentication_type(&self) -> &::std::option::Option<crate::types::AuthenticationType> {
        self.inner.get_authentication_type()
    }
    /// <p>An enumerated string that speciﬁes the application-layer protocol.</p>
    /// <ul>
    /// <li>
    /// <p><code>SECURE_MQTT</code> - MQTT over TLS.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>MQTT_WSS</code> - MQTT over WebSocket.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>HTTPS</code> - HTTP over TLS.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT</code> - Use a combination of port and Application Layer Protocol Negotiation (ALPN) to specify application_layer protocol. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/protocols.html">Device communication protocols</a>.</p></li>
    /// </ul>
    pub fn application_protocol(mut self, input: crate::types::ApplicationProtocol) -> Self {
        self.inner = self.inner.application_protocol(input);
        self
    }
    /// <p>An enumerated string that speciﬁes the application-layer protocol.</p>
    /// <ul>
    /// <li>
    /// <p><code>SECURE_MQTT</code> - MQTT over TLS.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>MQTT_WSS</code> - MQTT over WebSocket.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>HTTPS</code> - HTTP over TLS.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT</code> - Use a combination of port and Application Layer Protocol Negotiation (ALPN) to specify application_layer protocol. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/protocols.html">Device communication protocols</a>.</p></li>
    /// </ul>
    pub fn set_application_protocol(mut self, input: ::std::option::Option<crate::types::ApplicationProtocol>) -> Self {
        self.inner = self.inner.set_application_protocol(input);
        self
    }
    /// <p>An enumerated string that speciﬁes the application-layer protocol.</p>
    /// <ul>
    /// <li>
    /// <p><code>SECURE_MQTT</code> - MQTT over TLS.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>MQTT_WSS</code> - MQTT over WebSocket.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>HTTPS</code> - HTTP over TLS.</p></li>
    /// </ul>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT</code> - Use a combination of port and Application Layer Protocol Negotiation (ALPN) to specify application_layer protocol. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/protocols.html">Device communication protocols</a>.</p></li>
    /// </ul>
    pub fn get_application_protocol(&self) -> &::std::option::Option<crate::types::ApplicationProtocol> {
        self.inner.get_application_protocol()
    }
    /// <p>An object that speciﬁes the client certificate conﬁguration for a domain.</p>
    pub fn client_certificate_config(mut self, input: crate::types::ClientCertificateConfig) -> Self {
        self.inner = self.inner.client_certificate_config(input);
        self
    }
    /// <p>An object that speciﬁes the client certificate conﬁguration for a domain.</p>
    pub fn set_client_certificate_config(mut self, input: ::std::option::Option<crate::types::ClientCertificateConfig>) -> Self {
        self.inner = self.inner.set_client_certificate_config(input);
        self
    }
    /// <p>An object that speciﬁes the client certificate conﬁguration for a domain.</p>
    pub fn get_client_certificate_config(&self) -> &::std::option::Option<crate::types::ClientCertificateConfig> {
        self.inner.get_client_certificate_config()
    }
}
