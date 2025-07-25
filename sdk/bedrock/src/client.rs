// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) conf: crate::Config,
    #[allow(dead_code)] // unused when a service does not provide any operations
    pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
}

/// Client for Amazon Bedrock
///
/// Client for invoking operations on Amazon Bedrock. Each operation on Amazon Bedrock is a method on this
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
/// let client = aws_sdk_bedrock::Client::new(&config);
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
/// let config = aws_sdk_bedrock::config::Builder::from(&sdk_config)
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
/// For example, the [`CreateCustomModel`](crate::operation::create_custom_model) operation has
/// a [`Client::create_custom_model`], function which returns a builder for that operation.
/// The fluent builder ultimately has a `send()` function that returns an async future that
/// returns a result, as illustrated below:
///
/// ```rust,ignore
/// let result = client.create_custom_model()
///     .model_name("example")
///     .send()
///     .await;
/// ```
///
/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
/// information.
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

mod batch_delete_evaluation_job;

mod create_custom_model;

mod create_custom_model_deployment;

mod create_evaluation_job;

mod create_foundation_model_agreement;

mod create_guardrail;

mod create_guardrail_version;

mod create_inference_profile;

mod create_marketplace_model_endpoint;

mod create_model_copy_job;

mod create_model_customization_job;

mod create_model_import_job;

mod create_model_invocation_job;

mod create_prompt_router;

mod create_provisioned_model_throughput;

/// Operation customization and supporting types.
///
/// The underlying HTTP requests made during an operation can be customized
/// by calling the `customize()` method on the builder returned from a client
/// operation call. For example, this can be used to add an additional HTTP header:
///
/// ```ignore
/// # async fn wrapper() -> ::std::result::Result<(), aws_sdk_bedrock::Error> {
/// # let client: aws_sdk_bedrock::Client = unimplemented!();
/// use ::http::header::{HeaderName, HeaderValue};
///
/// let result = client.batch_delete_evaluation_job()
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

mod delete_custom_model;

mod delete_custom_model_deployment;

mod delete_foundation_model_agreement;

mod delete_guardrail;

mod delete_imported_model;

mod delete_inference_profile;

mod delete_marketplace_model_endpoint;

mod delete_model_invocation_logging_configuration;

mod delete_prompt_router;

mod delete_provisioned_model_throughput;

mod deregister_marketplace_model_endpoint;

mod get_custom_model;

mod get_custom_model_deployment;

mod get_evaluation_job;

mod get_foundation_model;

mod get_foundation_model_availability;

mod get_guardrail;

mod get_imported_model;

mod get_inference_profile;

mod get_marketplace_model_endpoint;

mod get_model_copy_job;

mod get_model_customization_job;

mod get_model_import_job;

mod get_model_invocation_job;

mod get_model_invocation_logging_configuration;

mod get_prompt_router;

mod get_provisioned_model_throughput;

mod get_use_case_for_model_access;

mod list_custom_model_deployments;

mod list_custom_models;

mod list_evaluation_jobs;

mod list_foundation_model_agreement_offers;

mod list_foundation_models;

mod list_guardrails;

mod list_imported_models;

mod list_inference_profiles;

mod list_marketplace_model_endpoints;

mod list_model_copy_jobs;

mod list_model_customization_jobs;

mod list_model_import_jobs;

mod list_model_invocation_jobs;

mod list_prompt_routers;

mod list_provisioned_model_throughputs;

mod list_tags_for_resource;

mod put_model_invocation_logging_configuration;

mod put_use_case_for_model_access;

mod register_marketplace_model_endpoint;

mod stop_evaluation_job;

mod stop_model_customization_job;

mod stop_model_invocation_job;

mod tag_resource;

mod untag_resource;

mod update_guardrail;

mod update_marketplace_model_endpoint;

mod update_provisioned_model_throughput;
