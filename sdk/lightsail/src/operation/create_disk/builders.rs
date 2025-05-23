// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_disk::_create_disk_output::CreateDiskOutputBuilder;

pub use crate::operation::create_disk::_create_disk_input::CreateDiskInputBuilder;

impl crate::operation::create_disk::builders::CreateDiskInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_disk::CreateDiskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_disk::CreateDiskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_disk();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateDisk`.
///
/// <p>Creates a block storage disk that can be attached to an Amazon Lightsail instance in the same Availability Zone (<code>us-east-2a</code>).</p>
/// <p>The <code>create disk</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-controlling-access-using-tags">Amazon Lightsail Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDiskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_disk::builders::CreateDiskInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_disk::CreateDiskOutput,
        crate::operation::create_disk::CreateDiskError,
    > for CreateDiskFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_disk::CreateDiskOutput,
            crate::operation::create_disk::CreateDiskError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateDiskFluentBuilder {
    /// Creates a new `CreateDiskFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateDisk as a reference.
    pub fn as_input(&self) -> &crate::operation::create_disk::builders::CreateDiskInputBuilder {
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
        crate::operation::create_disk::CreateDiskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_disk::CreateDiskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_disk::CreateDisk::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_disk::CreateDisk::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_disk::CreateDiskOutput,
        crate::operation::create_disk::CreateDiskError,
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
    /// <p>The unique Lightsail disk name (<code>my-disk</code>).</p>
    pub fn disk_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.disk_name(input.into());
        self
    }
    /// <p>The unique Lightsail disk name (<code>my-disk</code>).</p>
    pub fn set_disk_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_disk_name(input);
        self
    }
    /// <p>The unique Lightsail disk name (<code>my-disk</code>).</p>
    pub fn get_disk_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_disk_name()
    }
    /// <p>The Availability Zone where you want to create the disk (<code>us-east-2a</code>). Use the same Availability Zone as the Lightsail instance to which you want to attach the disk.</p>
    /// <p>Use the <code>get regions</code> operation to list the Availability Zones where Lightsail is currently available.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone where you want to create the disk (<code>us-east-2a</code>). Use the same Availability Zone as the Lightsail instance to which you want to attach the disk.</p>
    /// <p>Use the <code>get regions</code> operation to list the Availability Zones where Lightsail is currently available.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The Availability Zone where you want to create the disk (<code>us-east-2a</code>). Use the same Availability Zone as the Lightsail instance to which you want to attach the disk.</p>
    /// <p>Use the <code>get regions</code> operation to list the Availability Zones where Lightsail is currently available.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_availability_zone()
    }
    /// <p>The size of the disk in GB (<code>32</code>).</p>
    pub fn size_in_gb(mut self, input: i32) -> Self {
        self.inner = self.inner.size_in_gb(input);
        self
    }
    /// <p>The size of the disk in GB (<code>32</code>).</p>
    pub fn set_size_in_gb(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_size_in_gb(input);
        self
    }
    /// <p>The size of the disk in GB (<code>32</code>).</p>
    pub fn get_size_in_gb(&self) -> &::std::option::Option<i32> {
        self.inner.get_size_in_gb()
    }
    ///
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tag keys and optional values to add to the resource during create.</p>
    /// <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tag keys and optional values to add to the resource during create.</p>
    /// <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tag keys and optional values to add to the resource during create.</p>
    /// <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    ///
    /// Appends an item to `addOns`.
    ///
    /// To override the contents of this collection use [`set_add_ons`](Self::set_add_ons).
    ///
    /// <p>An array of objects that represent the add-ons to enable for the new disk.</p>
    pub fn add_ons(mut self, input: crate::types::AddOnRequest) -> Self {
        self.inner = self.inner.add_ons(input);
        self
    }
    /// <p>An array of objects that represent the add-ons to enable for the new disk.</p>
    pub fn set_add_ons(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AddOnRequest>>) -> Self {
        self.inner = self.inner.set_add_ons(input);
        self
    }
    /// <p>An array of objects that represent the add-ons to enable for the new disk.</p>
    pub fn get_add_ons(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AddOnRequest>> {
        self.inner.get_add_ons()
    }
}
