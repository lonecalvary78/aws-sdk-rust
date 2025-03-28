// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_campaign::_create_campaign_output::CreateCampaignOutputBuilder;

pub use crate::operation::create_campaign::_create_campaign_input::CreateCampaignInputBuilder;

impl crate::operation::create_campaign::builders::CreateCampaignInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_campaign::CreateCampaignOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_campaign::CreateCampaignError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_campaign();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateCampaign`.
///
/// <p>Creates an orchestration of data collection rules. The Amazon Web Services IoT FleetWise Edge Agent software running in vehicles uses campaigns to decide how to collect and transfer data to the cloud. You create campaigns in the cloud. After you or your team approve campaigns, Amazon Web Services IoT FleetWise automatically deploys them to vehicles.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/iot-fleetwise/latest/developerguide/campaigns.html">Collect and transfer data with campaigns</a> in the <i>Amazon Web Services IoT FleetWise Developer Guide</i>.</p><important>
/// <p>Access to certain Amazon Web Services IoT FleetWise features is currently gated. For more information, see <a href="https://docs.aws.amazon.com/iot-fleetwise/latest/developerguide/fleetwise-regions.html">Amazon Web Services Region and feature availability</a> in the <i>Amazon Web Services IoT FleetWise Developer Guide</i>.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCampaignFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_campaign::builders::CreateCampaignInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_campaign::CreateCampaignOutput,
        crate::operation::create_campaign::CreateCampaignError,
    > for CreateCampaignFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_campaign::CreateCampaignOutput,
            crate::operation::create_campaign::CreateCampaignError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateCampaignFluentBuilder {
    /// Creates a new `CreateCampaignFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateCampaign as a reference.
    pub fn as_input(&self) -> &crate::operation::create_campaign::builders::CreateCampaignInputBuilder {
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
        crate::operation::create_campaign::CreateCampaignOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_campaign::CreateCampaignError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_campaign::CreateCampaign::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_campaign::CreateCampaign::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_campaign::CreateCampaignOutput,
        crate::operation::create_campaign::CreateCampaignError,
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
    /// <p>The name of the campaign to create.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the campaign to create.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the campaign to create.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>An optional description of the campaign to help identify its purpose.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional description of the campaign to help identify its purpose.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>An optional description of the campaign to help identify its purpose.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The Amazon Resource Name (ARN) of the signal catalog to associate with the campaign.</p>
    pub fn signal_catalog_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.signal_catalog_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the signal catalog to associate with the campaign.</p>
    pub fn set_signal_catalog_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_signal_catalog_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the signal catalog to associate with the campaign.</p>
    pub fn get_signal_catalog_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_signal_catalog_arn()
    }
    /// <p>The ARN of the vehicle or fleet to deploy a campaign to.</p>
    pub fn target_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_arn(input.into());
        self
    }
    /// <p>The ARN of the vehicle or fleet to deploy a campaign to.</p>
    pub fn set_target_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_arn(input);
        self
    }
    /// <p>The ARN of the vehicle or fleet to deploy a campaign to.</p>
    pub fn get_target_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_arn()
    }
    /// <p>The time, in milliseconds, to deliver a campaign after it was approved. If it's not specified, <code>0</code> is used.</p>
    /// <p>Default: <code>0</code></p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The time, in milliseconds, to deliver a campaign after it was approved. If it's not specified, <code>0</code> is used.</p>
    /// <p>Default: <code>0</code></p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The time, in milliseconds, to deliver a campaign after it was approved. If it's not specified, <code>0</code> is used.</p>
    /// <p>Default: <code>0</code></p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The time the campaign expires, in seconds since epoch (January 1, 1970 at midnight UTC time). Vehicle data isn't collected after the campaign expires.</p>
    /// <p>Default: 253402214400 (December 31, 9999, 00:00:00 UTC)</p>
    pub fn expiry_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.expiry_time(input);
        self
    }
    /// <p>The time the campaign expires, in seconds since epoch (January 1, 1970 at midnight UTC time). Vehicle data isn't collected after the campaign expires.</p>
    /// <p>Default: 253402214400 (December 31, 9999, 00:00:00 UTC)</p>
    pub fn set_expiry_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_expiry_time(input);
        self
    }
    /// <p>The time the campaign expires, in seconds since epoch (January 1, 1970 at midnight UTC time). Vehicle data isn't collected after the campaign expires.</p>
    /// <p>Default: 253402214400 (December 31, 9999, 00:00:00 UTC)</p>
    pub fn get_expiry_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_expiry_time()
    }
    /// <p>How long (in milliseconds) to collect raw data after a triggering event initiates the collection. If it's not specified, <code>0</code> is used.</p>
    /// <p>Default: <code>0</code></p>
    pub fn post_trigger_collection_duration(mut self, input: i64) -> Self {
        self.inner = self.inner.post_trigger_collection_duration(input);
        self
    }
    /// <p>How long (in milliseconds) to collect raw data after a triggering event initiates the collection. If it's not specified, <code>0</code> is used.</p>
    /// <p>Default: <code>0</code></p>
    pub fn set_post_trigger_collection_duration(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_post_trigger_collection_duration(input);
        self
    }
    /// <p>How long (in milliseconds) to collect raw data after a triggering event initiates the collection. If it's not specified, <code>0</code> is used.</p>
    /// <p>Default: <code>0</code></p>
    pub fn get_post_trigger_collection_duration(&self) -> &::std::option::Option<i64> {
        self.inner.get_post_trigger_collection_duration()
    }
    /// <p>Option for a vehicle to send diagnostic trouble codes to Amazon Web Services IoT FleetWise. If you want to send diagnostic trouble codes, use <code>SEND_ACTIVE_DTCS</code>. If it's not specified, <code>OFF</code> is used.</p>
    /// <p>Default: <code>OFF</code></p>
    pub fn diagnostics_mode(mut self, input: crate::types::DiagnosticsMode) -> Self {
        self.inner = self.inner.diagnostics_mode(input);
        self
    }
    /// <p>Option for a vehicle to send diagnostic trouble codes to Amazon Web Services IoT FleetWise. If you want to send diagnostic trouble codes, use <code>SEND_ACTIVE_DTCS</code>. If it's not specified, <code>OFF</code> is used.</p>
    /// <p>Default: <code>OFF</code></p>
    pub fn set_diagnostics_mode(mut self, input: ::std::option::Option<crate::types::DiagnosticsMode>) -> Self {
        self.inner = self.inner.set_diagnostics_mode(input);
        self
    }
    /// <p>Option for a vehicle to send diagnostic trouble codes to Amazon Web Services IoT FleetWise. If you want to send diagnostic trouble codes, use <code>SEND_ACTIVE_DTCS</code>. If it's not specified, <code>OFF</code> is used.</p>
    /// <p>Default: <code>OFF</code></p>
    pub fn get_diagnostics_mode(&self) -> &::std::option::Option<crate::types::DiagnosticsMode> {
        self.inner.get_diagnostics_mode()
    }
    /// <p>Determines whether to store collected data after a vehicle lost a connection with the cloud. After a connection is re-established, the data is automatically forwarded to Amazon Web Services IoT FleetWise. If you want to store collected data when a vehicle loses connection with the cloud, use <code>TO_DISK</code>. If it's not specified, <code>OFF</code> is used.</p>
    /// <p>Default: <code>OFF</code></p>
    pub fn spooling_mode(mut self, input: crate::types::SpoolingMode) -> Self {
        self.inner = self.inner.spooling_mode(input);
        self
    }
    /// <p>Determines whether to store collected data after a vehicle lost a connection with the cloud. After a connection is re-established, the data is automatically forwarded to Amazon Web Services IoT FleetWise. If you want to store collected data when a vehicle loses connection with the cloud, use <code>TO_DISK</code>. If it's not specified, <code>OFF</code> is used.</p>
    /// <p>Default: <code>OFF</code></p>
    pub fn set_spooling_mode(mut self, input: ::std::option::Option<crate::types::SpoolingMode>) -> Self {
        self.inner = self.inner.set_spooling_mode(input);
        self
    }
    /// <p>Determines whether to store collected data after a vehicle lost a connection with the cloud. After a connection is re-established, the data is automatically forwarded to Amazon Web Services IoT FleetWise. If you want to store collected data when a vehicle loses connection with the cloud, use <code>TO_DISK</code>. If it's not specified, <code>OFF</code> is used.</p>
    /// <p>Default: <code>OFF</code></p>
    pub fn get_spooling_mode(&self) -> &::std::option::Option<crate::types::SpoolingMode> {
        self.inner.get_spooling_mode()
    }
    /// <p>Determines whether to compress signals before transmitting data to Amazon Web Services IoT FleetWise. If you don't want to compress the signals, use <code>OFF</code>. If it's not specified, <code>SNAPPY</code> is used.</p>
    /// <p>Default: <code>SNAPPY</code></p>
    pub fn compression(mut self, input: crate::types::Compression) -> Self {
        self.inner = self.inner.compression(input);
        self
    }
    /// <p>Determines whether to compress signals before transmitting data to Amazon Web Services IoT FleetWise. If you don't want to compress the signals, use <code>OFF</code>. If it's not specified, <code>SNAPPY</code> is used.</p>
    /// <p>Default: <code>SNAPPY</code></p>
    pub fn set_compression(mut self, input: ::std::option::Option<crate::types::Compression>) -> Self {
        self.inner = self.inner.set_compression(input);
        self
    }
    /// <p>Determines whether to compress signals before transmitting data to Amazon Web Services IoT FleetWise. If you don't want to compress the signals, use <code>OFF</code>. If it's not specified, <code>SNAPPY</code> is used.</p>
    /// <p>Default: <code>SNAPPY</code></p>
    pub fn get_compression(&self) -> &::std::option::Option<crate::types::Compression> {
        self.inner.get_compression()
    }
    /// <p>A number indicating the priority of one campaign over another campaign for a certain vehicle or fleet. A campaign with the lowest value is deployed to vehicles before any other campaigns. If it's not specified, <code>0</code> is used.</p>
    /// <p>Default: <code>0</code></p>
    #[deprecated(note = "priority is no longer used or needed as input")]
    pub fn priority(mut self, input: i32) -> Self {
        self.inner = self.inner.priority(input);
        self
    }
    /// <p>A number indicating the priority of one campaign over another campaign for a certain vehicle or fleet. A campaign with the lowest value is deployed to vehicles before any other campaigns. If it's not specified, <code>0</code> is used.</p>
    /// <p>Default: <code>0</code></p>
    #[deprecated(note = "priority is no longer used or needed as input")]
    pub fn set_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_priority(input);
        self
    }
    /// <p>A number indicating the priority of one campaign over another campaign for a certain vehicle or fleet. A campaign with the lowest value is deployed to vehicles before any other campaigns. If it's not specified, <code>0</code> is used.</p>
    /// <p>Default: <code>0</code></p>
    #[deprecated(note = "priority is no longer used or needed as input")]
    pub fn get_priority(&self) -> &::std::option::Option<i32> {
        self.inner.get_priority()
    }
    ///
    /// Appends an item to `signalsToCollect`.
    ///
    /// To override the contents of this collection use [`set_signals_to_collect`](Self::set_signals_to_collect).
    ///
    /// <p>A list of information about signals to collect.</p><note>
    /// <p>If you upload a signal as a condition in a data partition for a campaign, then those same signals must be included in <code>signalsToCollect</code>.</p>
    /// </note>
    pub fn signals_to_collect(mut self, input: crate::types::SignalInformation) -> Self {
        self.inner = self.inner.signals_to_collect(input);
        self
    }
    /// <p>A list of information about signals to collect.</p><note>
    /// <p>If you upload a signal as a condition in a data partition for a campaign, then those same signals must be included in <code>signalsToCollect</code>.</p>
    /// </note>
    pub fn set_signals_to_collect(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SignalInformation>>) -> Self {
        self.inner = self.inner.set_signals_to_collect(input);
        self
    }
    /// <p>A list of information about signals to collect.</p><note>
    /// <p>If you upload a signal as a condition in a data partition for a campaign, then those same signals must be included in <code>signalsToCollect</code>.</p>
    /// </note>
    pub fn get_signals_to_collect(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SignalInformation>> {
        self.inner.get_signals_to_collect()
    }
    /// <p>The data collection scheme associated with the campaign. You can specify a scheme that collects data based on time or an event.</p>
    pub fn collection_scheme(mut self, input: crate::types::CollectionScheme) -> Self {
        self.inner = self.inner.collection_scheme(input);
        self
    }
    /// <p>The data collection scheme associated with the campaign. You can specify a scheme that collects data based on time or an event.</p>
    pub fn set_collection_scheme(mut self, input: ::std::option::Option<crate::types::CollectionScheme>) -> Self {
        self.inner = self.inner.set_collection_scheme(input);
        self
    }
    /// <p>The data collection scheme associated with the campaign. You can specify a scheme that collects data based on time or an event.</p>
    pub fn get_collection_scheme(&self) -> &::std::option::Option<crate::types::CollectionScheme> {
        self.inner.get_collection_scheme()
    }
    ///
    /// Appends an item to `dataExtraDimensions`.
    ///
    /// To override the contents of this collection use [`set_data_extra_dimensions`](Self::set_data_extra_dimensions).
    ///
    /// <p>A list of vehicle attributes to associate with a campaign.</p>
    /// <p>Enrich the data with specified vehicle attributes. For example, add <code>make</code> and <code>model</code> to the campaign, and Amazon Web Services IoT FleetWise will associate the data with those attributes as dimensions in Amazon Timestream. You can then query the data against <code>make</code> and <code>model</code>.</p>
    /// <p>Default: An empty array</p>
    pub fn data_extra_dimensions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data_extra_dimensions(input.into());
        self
    }
    /// <p>A list of vehicle attributes to associate with a campaign.</p>
    /// <p>Enrich the data with specified vehicle attributes. For example, add <code>make</code> and <code>model</code> to the campaign, and Amazon Web Services IoT FleetWise will associate the data with those attributes as dimensions in Amazon Timestream. You can then query the data against <code>make</code> and <code>model</code>.</p>
    /// <p>Default: An empty array</p>
    pub fn set_data_extra_dimensions(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_data_extra_dimensions(input);
        self
    }
    /// <p>A list of vehicle attributes to associate with a campaign.</p>
    /// <p>Enrich the data with specified vehicle attributes. For example, add <code>make</code> and <code>model</code> to the campaign, and Amazon Web Services IoT FleetWise will associate the data with those attributes as dimensions in Amazon Timestream. You can then query the data against <code>make</code> and <code>model</code>.</p>
    /// <p>Default: An empty array</p>
    pub fn get_data_extra_dimensions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_data_extra_dimensions()
    }
    ///
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Metadata that can be used to manage the campaign.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Metadata that can be used to manage the campaign.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Metadata that can be used to manage the campaign.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    ///
    /// Appends an item to `dataDestinationConfigs`.
    ///
    /// To override the contents of this collection use [`set_data_destination_configs`](Self::set_data_destination_configs).
    ///
    /// <p>The destination where the campaign sends data. You can send data to an MQTT topic, or store it in Amazon S3 or Amazon Timestream.</p>
    /// <p>MQTT is the publish/subscribe messaging protocol used by Amazon Web Services IoT to communicate with your devices.</p>
    /// <p>Amazon S3 optimizes the cost of data storage and provides additional mechanisms to use vehicle data, such as data lakes, centralized data storage, data processing pipelines, and analytics. Amazon Web Services IoT FleetWise supports at-least-once file delivery to S3. Your vehicle data is stored on multiple Amazon Web Services IoT FleetWise servers for redundancy and high availability.</p>
    /// <p>You can use Amazon Timestream to access and analyze time series data, and Timestream to query vehicle data so that you can identify trends and patterns.</p>
    pub fn data_destination_configs(mut self, input: crate::types::DataDestinationConfig) -> Self {
        self.inner = self.inner.data_destination_configs(input);
        self
    }
    /// <p>The destination where the campaign sends data. You can send data to an MQTT topic, or store it in Amazon S3 or Amazon Timestream.</p>
    /// <p>MQTT is the publish/subscribe messaging protocol used by Amazon Web Services IoT to communicate with your devices.</p>
    /// <p>Amazon S3 optimizes the cost of data storage and provides additional mechanisms to use vehicle data, such as data lakes, centralized data storage, data processing pipelines, and analytics. Amazon Web Services IoT FleetWise supports at-least-once file delivery to S3. Your vehicle data is stored on multiple Amazon Web Services IoT FleetWise servers for redundancy and high availability.</p>
    /// <p>You can use Amazon Timestream to access and analyze time series data, and Timestream to query vehicle data so that you can identify trends and patterns.</p>
    pub fn set_data_destination_configs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DataDestinationConfig>>) -> Self {
        self.inner = self.inner.set_data_destination_configs(input);
        self
    }
    /// <p>The destination where the campaign sends data. You can send data to an MQTT topic, or store it in Amazon S3 or Amazon Timestream.</p>
    /// <p>MQTT is the publish/subscribe messaging protocol used by Amazon Web Services IoT to communicate with your devices.</p>
    /// <p>Amazon S3 optimizes the cost of data storage and provides additional mechanisms to use vehicle data, such as data lakes, centralized data storage, data processing pipelines, and analytics. Amazon Web Services IoT FleetWise supports at-least-once file delivery to S3. Your vehicle data is stored on multiple Amazon Web Services IoT FleetWise servers for redundancy and high availability.</p>
    /// <p>You can use Amazon Timestream to access and analyze time series data, and Timestream to query vehicle data so that you can identify trends and patterns.</p>
    pub fn get_data_destination_configs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DataDestinationConfig>> {
        self.inner.get_data_destination_configs()
    }
    ///
    /// Appends an item to `dataPartitions`.
    ///
    /// To override the contents of this collection use [`set_data_partitions`](Self::set_data_partitions).
    ///
    /// <p>The data partitions associated with the signals collected from the vehicle.</p>
    pub fn data_partitions(mut self, input: crate::types::DataPartition) -> Self {
        self.inner = self.inner.data_partitions(input);
        self
    }
    /// <p>The data partitions associated with the signals collected from the vehicle.</p>
    pub fn set_data_partitions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DataPartition>>) -> Self {
        self.inner = self.inner.set_data_partitions(input);
        self
    }
    /// <p>The data partitions associated with the signals collected from the vehicle.</p>
    pub fn get_data_partitions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DataPartition>> {
        self.inner.get_data_partitions()
    }
    ///
    /// Appends an item to `signalsToFetch`.
    ///
    /// To override the contents of this collection use [`set_signals_to_fetch`](Self::set_signals_to_fetch).
    ///
    /// <p>A list of information about signals to fetch.</p>
    pub fn signals_to_fetch(mut self, input: crate::types::SignalFetchInformation) -> Self {
        self.inner = self.inner.signals_to_fetch(input);
        self
    }
    /// <p>A list of information about signals to fetch.</p>
    pub fn set_signals_to_fetch(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SignalFetchInformation>>) -> Self {
        self.inner = self.inner.set_signals_to_fetch(input);
        self
    }
    /// <p>A list of information about signals to fetch.</p>
    pub fn get_signals_to_fetch(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SignalFetchInformation>> {
        self.inner.get_signals_to_fetch()
    }
}
