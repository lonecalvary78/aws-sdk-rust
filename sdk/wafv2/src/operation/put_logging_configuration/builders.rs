// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_logging_configuration::_put_logging_configuration_output::PutLoggingConfigurationOutputBuilder;

pub use crate::operation::put_logging_configuration::_put_logging_configuration_input::PutLoggingConfigurationInputBuilder;

impl crate::operation::put_logging_configuration::builders::PutLoggingConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_logging_configuration::PutLoggingConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_logging_configuration::PutLoggingConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_logging_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutLoggingConfiguration`.
///
/// <p>Enables the specified <code>LoggingConfiguration</code>, to start logging from a web ACL, according to the configuration provided.</p>
/// <p>If you configure data protection for the web ACL, the protection applies to the data that WAF sends to the logs.</p><note>
/// <p>This operation completely replaces any mutable specifications that you already have for a logging configuration with the ones that you provide to this call.</p>
/// <p>To modify an existing logging configuration, do the following:</p>
/// <ol>
/// <li>
/// <p>Retrieve it by calling <code>GetLoggingConfiguration</code></p></li>
/// <li>
/// <p>Update its settings as needed</p></li>
/// <li>
/// <p>Provide the complete logging configuration specification to this call</p></li>
/// </ol>
/// </note> <note>
/// <p>You can define one logging destination per web ACL.</p>
/// </note>
/// <p>You can access information about the traffic that WAF inspects using the following steps:</p>
/// <ol>
/// <li>
/// <p>Create your logging destination. You can use an Amazon CloudWatch Logs log group, an Amazon Simple Storage Service (Amazon S3) bucket, or an Amazon Kinesis Data Firehose.</p>
/// <p>The name that you give the destination must start with <code>aws-waf-logs-</code>. Depending on the type of destination, you might need to configure additional settings or permissions.</p>
/// <p>For configuration requirements and pricing information for each destination type, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging web ACL traffic</a> in the <i>WAF Developer Guide</i>.</p></li>
/// <li>
/// <p>Associate your logging destination to your web ACL using a <code>PutLoggingConfiguration</code> request.</p></li>
/// </ol>
/// <p>When you successfully enable logging using a <code>PutLoggingConfiguration</code> request, WAF creates an additional role or policy that is required to write logs to the logging destination. For an Amazon CloudWatch Logs log group, WAF creates a resource policy on the log group. For an Amazon S3 bucket, WAF creates a bucket policy. For an Amazon Kinesis Data Firehose, WAF creates a service-linked role.</p>
/// <p>For additional information about web ACL logging, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging web ACL traffic information</a> in the <i>WAF Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutLoggingConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_logging_configuration::builders::PutLoggingConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_logging_configuration::PutLoggingConfigurationOutput,
        crate::operation::put_logging_configuration::PutLoggingConfigurationError,
    > for PutLoggingConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_logging_configuration::PutLoggingConfigurationOutput,
            crate::operation::put_logging_configuration::PutLoggingConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutLoggingConfigurationFluentBuilder {
    /// Creates a new `PutLoggingConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutLoggingConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::put_logging_configuration::builders::PutLoggingConfigurationInputBuilder {
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
        crate::operation::put_logging_configuration::PutLoggingConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_logging_configuration::PutLoggingConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_logging_configuration::PutLoggingConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_logging_configuration::PutLoggingConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_logging_configuration::PutLoggingConfigurationOutput,
        crate::operation::put_logging_configuration::PutLoggingConfigurationError,
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
    /// <p></p>
    pub fn logging_configuration(mut self, input: crate::types::LoggingConfiguration) -> Self {
        self.inner = self.inner.logging_configuration(input);
        self
    }
    /// <p></p>
    pub fn set_logging_configuration(mut self, input: ::std::option::Option<crate::types::LoggingConfiguration>) -> Self {
        self.inner = self.inner.set_logging_configuration(input);
        self
    }
    /// <p></p>
    pub fn get_logging_configuration(&self) -> &::std::option::Option<crate::types::LoggingConfiguration> {
        self.inner.get_logging_configuration()
    }
}
