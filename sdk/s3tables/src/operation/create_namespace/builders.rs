// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_namespace::_create_namespace_output::CreateNamespaceOutputBuilder;

pub use crate::operation::create_namespace::_create_namespace_input::CreateNamespaceInputBuilder;

impl crate::operation::create_namespace::builders::CreateNamespaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_namespace::CreateNamespaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_namespace::CreateNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_namespace();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateNamespace`.
///
/// <p>Creates a namespace. A namespace is a logical grouping of tables within your table bucket, which you can use to organize tables. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-tables-namespace-create.html">Create a namespace</a> in the <i>Amazon Simple Storage Service User Guide</i>.</p>
/// <dl>
/// <dt>
/// Permissions
/// </dt>
/// <dd>
/// <p>You must have the <code>s3tables:CreateNamespace</code> permission to use this operation.</p>
/// </dd>
/// </dl>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateNamespaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_namespace::builders::CreateNamespaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_namespace::CreateNamespaceOutput,
        crate::operation::create_namespace::CreateNamespaceError,
    > for CreateNamespaceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_namespace::CreateNamespaceOutput,
            crate::operation::create_namespace::CreateNamespaceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateNamespaceFluentBuilder {
    /// Creates a new `CreateNamespaceFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateNamespace as a reference.
    pub fn as_input(&self) -> &crate::operation::create_namespace::builders::CreateNamespaceInputBuilder {
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
        crate::operation::create_namespace::CreateNamespaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_namespace::CreateNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_namespace::CreateNamespace::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_namespace::CreateNamespace::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_namespace::CreateNamespaceOutput,
        crate::operation::create_namespace::CreateNamespaceError,
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
    /// <p>The Amazon Resource Name (ARN) of the table bucket to create the namespace in.</p>
    pub fn table_bucket_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_bucket_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the table bucket to create the namespace in.</p>
    pub fn set_table_bucket_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_bucket_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the table bucket to create the namespace in.</p>
    pub fn get_table_bucket_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_bucket_arn()
    }
    ///
    /// Appends an item to `namespace`.
    ///
    /// To override the contents of this collection use [`set_namespace`](Self::set_namespace).
    ///
    /// <p>A name for the namespace.</p>
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace(input.into());
        self
    }
    /// <p>A name for the namespace.</p>
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_namespace(input);
        self
    }
    /// <p>A name for the namespace.</p>
    pub fn get_namespace(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_namespace()
    }
}
