#![allow(deprecated)]
#![allow(unknown_lints)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::disallowed_names)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::result_large_err)]
#![allow(clippy::unnecessary_map_on_constructor)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::invalid_html_tags)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! Amazon Web Services Control Tower offers application programming interface (API) operations that support programmatic interaction with these types of resources:
//!   - [_Controls_](https://docs.aws.amazon.com/controltower/latest/userguide/controls.html)
//!     - [DisableControl](https://docs.aws.amazon.com/controltower/latest/APIReference/API_DisableControl.html)
//!     - [EnableControl](https://docs.aws.amazon.com/controltower/latest/APIReference/API_EnableControl.html)
//!     - [GetEnabledControl](https://docs.aws.amazon.com/controltower/latest/APIReference/API_GetEnabledControl.html)
//!     - [GetControlOperation](https://docs.aws.amazon.com/controltower/latest/APIReference/API_GetControlOperation.html)
//!     - [ListControlOperations](https://docs.aws.amazon.com/controltower/latest/APIReference/API_ListControlOperations.html)
//!     - [ListEnabledControls](https://docs.aws.amazon.com/controltower/latest/APIReference/API_ListEnabledControls.html)
//!     - [ResetEnabledControl](https://docs.aws.amazon.com/controltower/latest/APIReference/API_ResetEnabledControl.html)
//!     - [UpdateEnabledControl](https://docs.aws.amazon.com/controltower/latest/APIReference/API_UpdateEnabledControl.html)
//!
//!   - [_Landing zones_](https://docs.aws.amazon.com/controltower/latest/userguide/lz-api-launch.html)
//!     - [CreateLandingZone](https://docs.aws.amazon.com/controltower/latest/APIReference/API_CreateLandingZone.html)
//!     - [DeleteLandingZone](https://docs.aws.amazon.com/controltower/latest/APIReference/API_DeleteLandingZone.html)
//!     - [GetLandingZone](https://docs.aws.amazon.com/controltower/latest/APIReference/API_GetLandingZone.html)
//!     - [GetLandingZoneOperation](https://docs.aws.amazon.com/controltower/latest/APIReference/API_GetLandingZoneOperation.html)
//!     - [ListLandingZones](https://docs.aws.amazon.com/controltower/latest/APIReference/API_ListLandingZones.html)
//!     - [ListLandingZoneOperations](https://docs.aws.amazon.com/controltower/latest/APIReference/API_ListLandingZoneOperations.html)
//!     - [ResetLandingZone](https://docs.aws.amazon.com/controltower/latest/APIReference/API_ResetLandingZone.html)
//!     - [UpdateLandingZone](https://docs.aws.amazon.com/controltower/latest/APIReference/API_UpdateLandingZone.html)
//!
//!   - [_Baselines_](https://docs.aws.amazon.com/controltower/latest/userguide/types-of-baselines.html)
//!     - [DisableBaseline](https://docs.aws.amazon.com/controltower/latest/APIReference/API_DisableBaseline.html)
//!     - [EnableBaseline](https://docs.aws.amazon.com/controltower/latest/APIReference/API_EnableBaseline.html)
//!     - [GetBaseline](https://docs.aws.amazon.com/controltower/latest/APIReference/API_GetBaseline.html)
//!     - [GetBaselineOperation](https://docs.aws.amazon.com/controltower/latest/APIReference/API_GetBaselineOperation.html)
//!     - [GetEnabledBaseline](https://docs.aws.amazon.com/controltower/latest/APIReference/API_GetEnabledBaseline.html)
//!     - [ListBaselines](https://docs.aws.amazon.com/controltower/latest/APIReference/API_ListBaselines.html)
//!     - [ListEnabledBaselines](https://docs.aws.amazon.com/controltower/latest/APIReference/API_ListEnabledBaselines.html)
//!     - [ResetEnabledBaseline](https://docs.aws.amazon.com/controltower/latest/APIReference/API_ResetEnabledBaseline.html)
//!     - [UpdateEnabledBaseline](https://docs.aws.amazon.com/controltower/latest/APIReference/API_UpdateEnabledBaseline.html)
//!
//!   - [_Tagging_](https://docs.aws.amazon.com/controltower/latest/controlreference/tagging.html)
//!     - [ListTagsForResource](https://docs.aws.amazon.com/controltower/latest/APIReference/API_ListTagsForResource.html)
//!     - [TagResource](https://docs.aws.amazon.com/controltower/latest/APIReference/API_TagResource.html)
//!     - [UntagResource](https://docs.aws.amazon.com/controltower/latest/APIReference/API_UntagResource.html)
//!
//! For more information about these types of resources, see the [_Amazon Web Services Control Tower User Guide_](https://docs.aws.amazon.com/controltower/latest/userguide/what-is-control-tower.html).
//!
//! __About control APIs__
//!
//! These interfaces allow you to apply the Amazon Web Services library of pre-defined _controls_ to your organizational units, programmatically. In Amazon Web Services Control Tower, the terms "control" and "guardrail" are synonyms.
//!
//! To call these APIs, you'll need to know:
//!   - the controlIdentifier for the control--or guardrail--you are targeting.
//!   - the ARN associated with the target organizational unit (OU), which we call the targetIdentifier.
//!   - the ARN associated with a resource that you wish to tag or untag.
//!
//! __To get the controlIdentifier for your Amazon Web Services Control Tower control:__
//!
//! The controlIdentifier is an ARN that is specified for each control. You can view the controlIdentifier in the console on the __Control details__ page, as well as in the documentation.
//!
//! __About identifiers for Amazon Web Services Control Tower__
//!
//! The Amazon Web Services Control Tower controlIdentifier is unique in each Amazon Web Services Region for each control. You can find the controlIdentifier for each Region and control in the [Tables of control metadata](https://docs.aws.amazon.com/controltower/latest/controlreference/control-metadata-tables.html) or the [Control availability by Region tables](https://docs.aws.amazon.com/controltower/latest/controlreference/control-region-tables.html) in the _Amazon Web Services Control Tower Controls Reference Guide_.
//!
//! A quick-reference list of control identifers for the Amazon Web Services Control Tower legacy _Strongly recommended_ and _Elective_ controls is given in [Resource identifiers for APIs and controls](https://docs.aws.amazon.com/controltower/latest/controlreference/control-identifiers.html.html) in the [_Amazon Web Services Control Tower Controls Reference Guide_](https://docs.aws.amazon.com/controltower/latest/controlreference/control-identifiers.html). Remember that _Mandatory_ controls cannot be added or removed.
//!
//! __To get the targetIdentifier:__
//!
//! The targetIdentifier is the ARN for an OU.
//!
//! In the Amazon Web Services Organizations console, you can find the ARN for the OU on the __Organizational unit details__ page associated with that OU.
//!
//! __ About landing zone APIs__
//!
//! You can configure and launch an Amazon Web Services Control Tower landing zone with APIs. For an introduction and steps, see [Getting started with Amazon Web Services Control Tower using APIs](https://docs.aws.amazon.com/controltower/latest/userguide/getting-started-apis.html).
//!
//! For an overview of landing zone API operations, see [Amazon Web Services Control Tower supports landing zone APIs](https://docs.aws.amazon.com/controltower/latest/userguide/2023-all.html#landing-zone-apis). The individual API operations for landing zones are detailed in this document, the [API reference manual](https://docs.aws.amazon.com/controltower/latest/APIReference/API_Operations.html), in the "Actions" section.
//!
//! __About baseline APIs__
//!
//! You can apply the AWSControlTowerBaseline baseline to an organizational unit (OU) as a way to register the OU with Amazon Web Services Control Tower, programmatically. For a general overview of this capability, see [Amazon Web Services Control Tower supports APIs for OU registration and configuration with baselines](https://docs.aws.amazon.com/controltower/latest/userguide/2024-all.html#baseline-apis).
//!
//! You can call the baseline API operations to view the baselines that Amazon Web Services Control Tower enables for your landing zone, on your behalf, when setting up the landing zone. These baselines are read-only baselines.
//!
//! The individual API operations for baselines are detailed in this document, the [API reference manual](https://docs.aws.amazon.com/controltower/latest/APIReference/API_Operations.html), in the "Actions" section. For usage examples, see [Baseline API input and output examples with CLI](https://docs.aws.amazon.com/controltower/latest/userguide/baseline-api-examples.html).
//!
//! __ About Amazon Web Services Control Catalog identifiers__
//!   - The EnableControl and DisableControl API operations can be called by specifying either the Amazon Web Services Control Tower identifer or the Amazon Web Services Control Catalog identifier. The API response returns the same type of identifier that you specified when calling the API.
//!   - If you use an Amazon Web Services Control Tower identifier to call the EnableControl API, and then call EnableControl again with an Amazon Web Services Control Catalog identifier, Amazon Web Services Control Tower returns an error message stating that the control is already enabled. Similar behavior applies to the DisableControl API operation.
//!   - Mandatory controls and the landing-zone-level Region deny control have Amazon Web Services Control Tower identifiers only.
//!
//! __Details and examples__
//!   - [Control API input and output examples with CLI](https://docs.aws.amazon.com/controltower/latest/controlreference/control-api-examples-short.html)
//!   - [Baseline API input and output examples with CLI](https://docs.aws.amazon.com/controltower/latest/userguide/baseline-api-examples.html)
//!   - [Enable controls with CloudFormation](https://docs.aws.amazon.com/controltower/latest/controlreference/enable-controls.html)
//!   - [Launch a landing zone with CloudFormation](https://docs.aws.amazon.com/controltower/latest/userguide/lz-apis-cfn-setup.html)
//!   - [Control metadata tables (large page)](https://docs.aws.amazon.com/controltower/latest/controlreference/control-metadata-tables.html)
//!   - [Control availability by Region tables (large page)](https://docs.aws.amazon.com/controltower/latest/controlreference/control-region-tables.html)
//!   - [List of identifiers for legacy controls](https://docs.aws.amazon.com/controltower/latest/controlreference/control-identifiers.html)
//!   - [Controls reference guide](https://docs.aws.amazon.com/controltower/latest/controlreference/controls.html)
//!   - [Controls library groupings](https://docs.aws.amazon.com/controltower/latest/controlreference/controls-reference.html)
//!   - [Creating Amazon Web Services Control Tower resources with Amazon Web Services CloudFormation](https://docs.aws.amazon.com/controltower/latest/userguide/creating-resources-with-cloudformation.html)
//!
//! To view the open source resource repository on GitHub, see [aws-cloudformation/aws-cloudformation-resource-providers-controltower](https://github.com/aws-cloudformation/aws-cloudformation-resource-providers-controltower)
//!
//! __Recording API Requests__
//!
//! Amazon Web Services Control Tower supports Amazon Web Services CloudTrail, a service that records Amazon Web Services API calls for your Amazon Web Services account and delivers log files to an Amazon S3 bucket. By using information collected by CloudTrail, you can determine which requests the Amazon Web Services Control Tower service received, who made the request and when, and so on. For more about Amazon Web Services Control Tower and its support for CloudTrail, see [Logging Amazon Web Services Control Tower Actions with Amazon Web Services CloudTrail](https://docs.aws.amazon.com/controltower/latest/userguide/logging-using-cloudtrail.html) in the Amazon Web Services Control Tower User Guide. To learn more about CloudTrail, including how to turn it on and find your log files, see the Amazon Web Services CloudTrail User Guide.
//!
//! ## Getting Started
//!
//! > Examples are available for many services and operations, check out the
//! > [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).
//!
//! The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
//! as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-controltower` to
//! your project, add the following to your **Cargo.toml** file:
//!
//! ```toml
//! [dependencies]
//! aws-config = { version = "1.1.7", features = ["behavior-version-latest"] }
//! aws-sdk-controltower = "1.83.0"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! Then in code, a client can be created with the following:
//!
//! ```rust,no_run
//! use aws_sdk_controltower as controltower;
//!
//! #[::tokio::main]
//! async fn main() -> Result<(), controltower::Error> {
//!     let config = aws_config::load_from_env().await;
//!     let client = aws_sdk_controltower::Client::new(&config);
//!
//!     // ... make some calls with the client
//!
//!     Ok(())
//! }
//! ```
//!
//! See the [client documentation](https://docs.rs/aws-sdk-controltower/latest/aws_sdk_controltower/client/struct.Client.html)
//! for information on what calls can be made, and the inputs and outputs for each of those calls.
//!
//! ## Using the SDK
//!
//! Until the SDK is released, we will be adding information about using the SDK to the
//! [Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
//! additional sections for the guide by opening an issue and describing what you are trying to do.
//!
//! ## Getting Help
//!
//! * [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
//! * [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) - For bug reports & feature requests
//! * [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
//! * [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples)
//!
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`], which exposes one method for each API
//! offered by AWS Control Tower. The return value of each of these methods is a "fluent builder",
//! where the different inputs for that API are added by builder-style function call chaining,
//! followed by calling `send()` to get a [`Future`](std::future::Future) that will result in
//! either a successful output or a [`SdkError`](crate::error::SdkError).
//!
//! Some of these API inputs may be structs or enums to provide more complex structured information.
//! These structs and enums live in [`types`](crate::types). There are some simpler types for
//! representing data such as date times or binary blobs that live in [`primitives`](crate::primitives).
//!
//! All types required to configure a client via the [`Config`](crate::Config) struct live
//! in [`config`](crate::config).
//!
//! The [`operation`](crate::operation) module has a submodule for every API, and in each submodule
//! is the input, output, and error type for that API, as well as builders to construct each of those.
//!
//! There is a top-level [`Error`](crate::Error) type that encompasses all the errors that the
//! client can return. Any other error type can be converted to this `Error` type via the
//! [`From`](std::convert::From) trait.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

/// Client for calling AWS Control Tower.
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
/// let client = aws_sdk_controltower::Client::new(&config);
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
/// let config = aws_sdk_controltower::config::Builder::from(&sdk_config)
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
/// For example, the [`CreateLandingZone`](crate::operation::create_landing_zone) operation has
/// a [`Client::create_landing_zone`], function which returns a builder for that operation.
/// The fluent builder ultimately has a `send()` function that returns an async future that
/// returns a result, as illustrated below:
///
/// ```rust,ignore
/// let result = client.create_landing_zone()
///     .version("example")
///     .send()
///     .await;
/// ```
///
/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
/// information.
pub mod client;

/// Configuration for AWS Control Tower.
pub mod config;

/// Common errors and error handling utilities.
pub mod error;

mod error_meta;

/// Information about this crate.
pub mod meta;

/// All operations that this crate can perform.
pub mod operation;

/// Primitives such as `Blob` or `DateTime` used by other types.
pub mod primitives;

/// Data structures used by operation inputs/outputs.
pub mod types;

pub(crate) mod protocol_serde;

mod sdk_feature_tracker;

mod serialization_settings;

mod endpoint_lib;

mod lens;

mod serde_util;

mod json_errors;

#[doc(inline)]
pub use client::Client;
