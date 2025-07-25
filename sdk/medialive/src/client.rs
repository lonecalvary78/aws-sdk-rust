// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) conf: crate::Config,
    #[allow(dead_code)] // unused when a service does not provide any operations
    pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
}

/// Client for AWS Elemental MediaLive
///
/// Client for invoking operations on AWS Elemental MediaLive. Each operation on AWS Elemental MediaLive is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
/// ## Constructing a `Client`
///
/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]
/// crate should be used to automatically resolve this config using
/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared
/// across multiple different AWS SDK clients. This config resolution process can be customized
/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses
/// the [builder pattern] to customize the default config.
///
/// In the simplest case, creating a client looks as follows:
/// ```rust,no_run
/// # async fn wrapper() {
/// let config = aws_config::load_from_env().await;
/// let client = aws_sdk_medialive::Client::new(&config);
/// # }
/// ```
///
/// Occasionally, SDKs may have additional service-specific values that can be set on the [`Config`] that
/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.
/// The [`Builder`](crate::config::Builder) struct implements `From<&SdkConfig>`, so setting these specific settings can be
/// done as follows:
///
/// ```rust,no_run
/// # async fn wrapper() {
/// let sdk_config = ::aws_config::load_from_env().await;
/// let config = aws_sdk_medialive::config::Builder::from(&sdk_config)
/// # /*
///     .some_service_specific_setting("value")
/// # */
///     .build();
/// # }
/// ```
///
/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.
///
/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should
/// be done once at application start-up.
///
/// [`Config`]: crate::Config
/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html
/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html
/// [`aws-config` docs]: https://docs.rs/aws-config/*
/// [`aws-config`]: https://crates.io/crates/aws-config
/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html
/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
/// # Using the `Client`
///
/// A client has a function for every operation that can be performed by the service.
/// For example, the [`AcceptInputDeviceTransfer`](crate::operation::accept_input_device_transfer) operation has
/// a [`Client::accept_input_device_transfer`], function which returns a builder for that operation.
/// The fluent builder ultimately has a `send()` function that returns an async future that
/// returns a result, as illustrated below:
///
/// ```rust,ignore
/// let result = client.accept_input_device_transfer()
///     .input_device_id("example")
///     .send()
///     .await;
/// ```
///
/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
/// information.
/// # Waiters
///
/// This client provides `wait_until` methods behind the [`Waiters`](crate::client::Waiters) trait.
/// To use them, simply import the trait, and then call one of the `wait_until` methods. This will
/// return a waiter fluent builder that takes various parameters, which are documented on the builder
/// type. Once parameters have been provided, the `wait` method can be called to initiate waiting.
///
/// For example, if there was a `wait_until_thing` method, it could look like:
/// ```rust,ignore
/// let result = client.wait_until_thing()
///     .thing_id("someId")
///     .wait(Duration::from_secs(120))
///     .await;
/// ```
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Client {
    handle: ::std::sync::Arc<Handle>,
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    ///
    /// # Panics
    ///
    /// This method will panic in the following cases:
    ///
    /// - Retries or timeouts are enabled without a `sleep_impl` configured.
    /// - Identity caching is enabled without a `sleep_impl` and `time_source` configured.
    /// - No `behavior_version` is provided.
    ///
    /// The panic message for each of these will have instructions on how to resolve them.
    #[track_caller]
    pub fn from_conf(conf: crate::Config) -> Self {
        let handle = Handle {
            conf: conf.clone(),
            runtime_plugins: crate::config::base_client_runtime_plugins(conf),
        };
        if let Err(err) = Self::validate_config(&handle) {
            panic!("Invalid client configuration: {err}");
        }
        Self {
            handle: ::std::sync::Arc::new(handle),
        }
    }

    /// Returns the client's configuration.
    pub fn config(&self) -> &crate::Config {
        &self.handle.conf
    }

    fn validate_config(handle: &Handle) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
        handle
            .runtime_plugins
            .apply_client_configuration(&mut cfg)?
            .validate_base_client_config(&cfg)?;
        Ok(())
    }
}

///
/// Waiter functions for the client.
///
/// Import this trait to get `wait_until` methods on the client.
///
pub trait Waiters {
    /// Wait until a channel has been created
    fn wait_until_channel_created(&self) -> crate::waiters::channel_created::ChannelCreatedFluentBuilder;
    /// Wait until a channel has been deleted
    fn wait_until_channel_deleted(&self) -> crate::waiters::channel_deleted::ChannelDeletedFluentBuilder;
    /// Wait until a channel is running
    fn wait_until_channel_running(&self) -> crate::waiters::channel_running::ChannelRunningFluentBuilder;
    /// Wait until a channel has is stopped
    fn wait_until_channel_stopped(&self) -> crate::waiters::channel_stopped::ChannelStoppedFluentBuilder;
    /// Wait until the channel placement group has been assigned
    fn wait_until_channel_placement_group_assigned(
        &self,
    ) -> crate::waiters::channel_placement_group_assigned::ChannelPlacementGroupAssignedFluentBuilder;
    /// Wait until the channel placement group has been deleted
    fn wait_until_channel_placement_group_deleted(
        &self,
    ) -> crate::waiters::channel_placement_group_deleted::ChannelPlacementGroupDeletedFluentBuilder;
    /// Wait until the channel placement group has been unassigned
    fn wait_until_channel_placement_group_unassigned(
        &self,
    ) -> crate::waiters::channel_placement_group_unassigned::ChannelPlacementGroupUnassignedFluentBuilder;
    /// Wait until a cluster has been created
    fn wait_until_cluster_created(&self) -> crate::waiters::cluster_created::ClusterCreatedFluentBuilder;
    /// Wait until a cluster has been deleted
    fn wait_until_cluster_deleted(&self) -> crate::waiters::cluster_deleted::ClusterDeletedFluentBuilder;
    /// Wait until an input has been attached
    fn wait_until_input_attached(&self) -> crate::waiters::input_attached::InputAttachedFluentBuilder;
    /// Wait until an input has been deleted
    fn wait_until_input_deleted(&self) -> crate::waiters::input_deleted::InputDeletedFluentBuilder;
    /// Wait until an input has been detached
    fn wait_until_input_detached(&self) -> crate::waiters::input_detached::InputDetachedFluentBuilder;
    /// Wait until a multiplex has been created
    fn wait_until_multiplex_created(&self) -> crate::waiters::multiplex_created::MultiplexCreatedFluentBuilder;
    /// Wait until a multiplex has been deleted
    fn wait_until_multiplex_deleted(&self) -> crate::waiters::multiplex_deleted::MultiplexDeletedFluentBuilder;
    /// Wait until a multiplex is running
    fn wait_until_multiplex_running(&self) -> crate::waiters::multiplex_running::MultiplexRunningFluentBuilder;
    /// Wait until a multiplex has is stopped
    fn wait_until_multiplex_stopped(&self) -> crate::waiters::multiplex_stopped::MultiplexStoppedFluentBuilder;
    /// Wait until a node has been deregistered
    fn wait_until_node_deregistered(&self) -> crate::waiters::node_deregistered::NodeDeregisteredFluentBuilder;
    /// Wait until a node has been registered
    fn wait_until_node_registered(&self) -> crate::waiters::node_registered::NodeRegisteredFluentBuilder;
    /// Wait until a signal map has been created
    fn wait_until_signal_map_created(&self) -> crate::waiters::signal_map_created::SignalMapCreatedFluentBuilder;
    /// Wait until a signal map's monitor has been deleted
    fn wait_until_signal_map_monitor_deleted(&self) -> crate::waiters::signal_map_monitor_deleted::SignalMapMonitorDeletedFluentBuilder;
    /// Wait until a signal map's monitor has been deployed
    fn wait_until_signal_map_monitor_deployed(&self) -> crate::waiters::signal_map_monitor_deployed::SignalMapMonitorDeployedFluentBuilder;
    /// Wait until a signal map has been updated
    fn wait_until_signal_map_updated(&self) -> crate::waiters::signal_map_updated::SignalMapUpdatedFluentBuilder;
}
impl Waiters for Client {
    fn wait_until_channel_created(&self) -> crate::waiters::channel_created::ChannelCreatedFluentBuilder {
        crate::waiters::channel_created::ChannelCreatedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_channel_deleted(&self) -> crate::waiters::channel_deleted::ChannelDeletedFluentBuilder {
        crate::waiters::channel_deleted::ChannelDeletedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_channel_running(&self) -> crate::waiters::channel_running::ChannelRunningFluentBuilder {
        crate::waiters::channel_running::ChannelRunningFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_channel_stopped(&self) -> crate::waiters::channel_stopped::ChannelStoppedFluentBuilder {
        crate::waiters::channel_stopped::ChannelStoppedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_channel_placement_group_assigned(
        &self,
    ) -> crate::waiters::channel_placement_group_assigned::ChannelPlacementGroupAssignedFluentBuilder {
        crate::waiters::channel_placement_group_assigned::ChannelPlacementGroupAssignedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_channel_placement_group_deleted(
        &self,
    ) -> crate::waiters::channel_placement_group_deleted::ChannelPlacementGroupDeletedFluentBuilder {
        crate::waiters::channel_placement_group_deleted::ChannelPlacementGroupDeletedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_channel_placement_group_unassigned(
        &self,
    ) -> crate::waiters::channel_placement_group_unassigned::ChannelPlacementGroupUnassignedFluentBuilder {
        crate::waiters::channel_placement_group_unassigned::ChannelPlacementGroupUnassignedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_cluster_created(&self) -> crate::waiters::cluster_created::ClusterCreatedFluentBuilder {
        crate::waiters::cluster_created::ClusterCreatedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_cluster_deleted(&self) -> crate::waiters::cluster_deleted::ClusterDeletedFluentBuilder {
        crate::waiters::cluster_deleted::ClusterDeletedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_input_attached(&self) -> crate::waiters::input_attached::InputAttachedFluentBuilder {
        crate::waiters::input_attached::InputAttachedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_input_deleted(&self) -> crate::waiters::input_deleted::InputDeletedFluentBuilder {
        crate::waiters::input_deleted::InputDeletedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_input_detached(&self) -> crate::waiters::input_detached::InputDetachedFluentBuilder {
        crate::waiters::input_detached::InputDetachedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_multiplex_created(&self) -> crate::waiters::multiplex_created::MultiplexCreatedFluentBuilder {
        crate::waiters::multiplex_created::MultiplexCreatedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_multiplex_deleted(&self) -> crate::waiters::multiplex_deleted::MultiplexDeletedFluentBuilder {
        crate::waiters::multiplex_deleted::MultiplexDeletedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_multiplex_running(&self) -> crate::waiters::multiplex_running::MultiplexRunningFluentBuilder {
        crate::waiters::multiplex_running::MultiplexRunningFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_multiplex_stopped(&self) -> crate::waiters::multiplex_stopped::MultiplexStoppedFluentBuilder {
        crate::waiters::multiplex_stopped::MultiplexStoppedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_node_deregistered(&self) -> crate::waiters::node_deregistered::NodeDeregisteredFluentBuilder {
        crate::waiters::node_deregistered::NodeDeregisteredFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_node_registered(&self) -> crate::waiters::node_registered::NodeRegisteredFluentBuilder {
        crate::waiters::node_registered::NodeRegisteredFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_signal_map_created(&self) -> crate::waiters::signal_map_created::SignalMapCreatedFluentBuilder {
        crate::waiters::signal_map_created::SignalMapCreatedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_signal_map_monitor_deleted(&self) -> crate::waiters::signal_map_monitor_deleted::SignalMapMonitorDeletedFluentBuilder {
        crate::waiters::signal_map_monitor_deleted::SignalMapMonitorDeletedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_signal_map_monitor_deployed(&self) -> crate::waiters::signal_map_monitor_deployed::SignalMapMonitorDeployedFluentBuilder {
        crate::waiters::signal_map_monitor_deployed::SignalMapMonitorDeployedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_signal_map_updated(&self) -> crate::waiters::signal_map_updated::SignalMapUpdatedFluentBuilder {
        crate::waiters::signal_map_updated::SignalMapUpdatedFluentBuilder::new(self.handle.clone())
    }
}

impl Client {
    /// Creates a new client from an [SDK Config](::aws_types::sdk_config::SdkConfig).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
    ///   the `sleep_impl` on the Config passed into this function to fix it.
    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
    ///   `http_connector` on the Config passed into this function to fix it.
    /// - This method will panic if no `BehaviorVersion` is provided. If you experience this panic, set `behavior_version` on the Config or enable the `behavior-version-latest` Cargo feature.
    #[track_caller]
    pub fn new(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }
}

mod accept_input_device_transfer;

mod batch_delete;

mod batch_start;

mod batch_stop;

mod batch_update_schedule;

mod cancel_input_device_transfer;

mod claim_device;

mod create_channel;

mod create_channel_placement_group;

mod create_cloud_watch_alarm_template;

mod create_cloud_watch_alarm_template_group;

mod create_cluster;

mod create_event_bridge_rule_template;

mod create_event_bridge_rule_template_group;

mod create_input;

mod create_input_security_group;

mod create_multiplex;

mod create_multiplex_program;

mod create_network;

mod create_node;

mod create_node_registration_script;

mod create_partner_input;

mod create_sdi_source;

mod create_signal_map;

mod create_tags;

/// Operation customization and supporting types.
///
/// The underlying HTTP requests made during an operation can be customized
/// by calling the `customize()` method on the builder returned from a client
/// operation call. For example, this can be used to add an additional HTTP header:
///
/// ```ignore
/// # async fn wrapper() -> ::std::result::Result<(), aws_sdk_medialive::Error> {
/// # let client: aws_sdk_medialive::Client = unimplemented!();
/// use ::http::header::{HeaderName, HeaderValue};
///
/// let result = client.accept_input_device_transfer()
///     .customize()
///     .mutate_request(|req| {
///         // Add `x-example-header` with value
///         req.headers_mut()
///             .insert(
///                 HeaderName::from_static("x-example-header"),
///                 HeaderValue::from_static("1"),
///             );
///     })
///     .send()
///     .await;
/// # }
/// ```
pub mod customize;

mod delete_channel;

mod delete_channel_placement_group;

mod delete_cloud_watch_alarm_template;

mod delete_cloud_watch_alarm_template_group;

mod delete_cluster;

mod delete_event_bridge_rule_template;

mod delete_event_bridge_rule_template_group;

mod delete_input;

mod delete_input_security_group;

mod delete_multiplex;

mod delete_multiplex_program;

mod delete_network;

mod delete_node;

mod delete_reservation;

mod delete_schedule;

mod delete_sdi_source;

mod delete_signal_map;

mod delete_tags;

mod describe_account_configuration;

mod describe_channel;

mod describe_channel_placement_group;

mod describe_cluster;

mod describe_input;

mod describe_input_device;

mod describe_input_device_thumbnail;

mod describe_input_security_group;

mod describe_multiplex;

mod describe_multiplex_program;

mod describe_network;

mod describe_node;

mod describe_offering;

mod describe_reservation;

mod describe_schedule;

mod describe_sdi_source;

mod describe_thumbnails;

mod get_cloud_watch_alarm_template;

mod get_cloud_watch_alarm_template_group;

mod get_event_bridge_rule_template;

mod get_event_bridge_rule_template_group;

mod get_signal_map;

mod list_channel_placement_groups;

mod list_channels;

mod list_cloud_watch_alarm_template_groups;

mod list_cloud_watch_alarm_templates;

mod list_clusters;

mod list_event_bridge_rule_template_groups;

mod list_event_bridge_rule_templates;

mod list_input_device_transfers;

mod list_input_devices;

mod list_input_security_groups;

mod list_inputs;

mod list_multiplex_programs;

mod list_multiplexes;

mod list_networks;

mod list_nodes;

mod list_offerings;

mod list_reservations;

mod list_sdi_sources;

mod list_signal_maps;

mod list_tags_for_resource;

mod list_versions;

mod purchase_offering;

mod reboot_input_device;

mod reject_input_device_transfer;

mod restart_channel_pipelines;

mod start_channel;

mod start_delete_monitor_deployment;

mod start_input_device;

mod start_input_device_maintenance_window;

mod start_monitor_deployment;

mod start_multiplex;

mod start_update_signal_map;

mod stop_channel;

mod stop_input_device;

mod stop_multiplex;

mod transfer_input_device;

mod update_account_configuration;

mod update_channel;

mod update_channel_class;

mod update_channel_placement_group;

mod update_cloud_watch_alarm_template;

mod update_cloud_watch_alarm_template_group;

mod update_cluster;

mod update_event_bridge_rule_template;

mod update_event_bridge_rule_template_group;

mod update_input;

mod update_input_device;

mod update_input_security_group;

mod update_multiplex;

mod update_multiplex_program;

mod update_network;

mod update_node;

mod update_node_state;

mod update_reservation;

mod update_sdi_source;
