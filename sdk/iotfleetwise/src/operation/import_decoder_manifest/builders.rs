// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_decoder_manifest::_import_decoder_manifest_output::ImportDecoderManifestOutputBuilder;

pub use crate::operation::import_decoder_manifest::_import_decoder_manifest_input::ImportDecoderManifestInputBuilder;

impl crate::operation::import_decoder_manifest::builders::ImportDecoderManifestInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::import_decoder_manifest::ImportDecoderManifestOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_decoder_manifest::ImportDecoderManifestError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.import_decoder_manifest();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ImportDecoderManifest`.
///
/// <p>Creates a decoder manifest using your existing CAN DBC file from your local device.</p>
/// <p>The CAN signal name must be unique and not repeated across CAN message definitions in a .dbc file.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportDecoderManifestFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_decoder_manifest::builders::ImportDecoderManifestInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::import_decoder_manifest::ImportDecoderManifestOutput,
        crate::operation::import_decoder_manifest::ImportDecoderManifestError,
    > for ImportDecoderManifestFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::import_decoder_manifest::ImportDecoderManifestOutput,
            crate::operation::import_decoder_manifest::ImportDecoderManifestError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ImportDecoderManifestFluentBuilder {
    /// Creates a new `ImportDecoderManifestFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ImportDecoderManifest as a reference.
    pub fn as_input(&self) -> &crate::operation::import_decoder_manifest::builders::ImportDecoderManifestInputBuilder {
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
        crate::operation::import_decoder_manifest::ImportDecoderManifestOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_decoder_manifest::ImportDecoderManifestError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::import_decoder_manifest::ImportDecoderManifest::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::import_decoder_manifest::ImportDecoderManifest::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::import_decoder_manifest::ImportDecoderManifestOutput,
        crate::operation::import_decoder_manifest::ImportDecoderManifestError,
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
    /// <p>The name of the decoder manifest to import.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the decoder manifest to import.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the decoder manifest to import.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    ///
    /// Appends an item to `networkFileDefinitions`.
    ///
    /// To override the contents of this collection use [`set_network_file_definitions`](Self::set_network_file_definitions).
    ///
    /// <p>The file to load into an Amazon Web Services account.</p>
    pub fn network_file_definitions(mut self, input: crate::types::NetworkFileDefinition) -> Self {
        self.inner = self.inner.network_file_definitions(input);
        self
    }
    /// <p>The file to load into an Amazon Web Services account.</p>
    pub fn set_network_file_definitions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::NetworkFileDefinition>>) -> Self {
        self.inner = self.inner.set_network_file_definitions(input);
        self
    }
    /// <p>The file to load into an Amazon Web Services account.</p>
    pub fn get_network_file_definitions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::NetworkFileDefinition>> {
        self.inner.get_network_file_definitions()
    }
}
