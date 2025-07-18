// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `CreateHostedZone`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateHostedZone;
impl CreateHostedZone {
    /// Creates a new `CreateHostedZone`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::create_hosted_zone::CreateHostedZoneInput,
    ) -> ::std::result::Result<
        crate::operation::create_hosted_zone::CreateHostedZoneOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_hosted_zone::CreateHostedZoneError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::create_hosted_zone::CreateHostedZoneError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::create_hosted_zone::CreateHostedZoneOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::create_hosted_zone::CreateHostedZoneInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        use ::tracing::Instrument;
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("Route 53", "CreateHostedZone", input, runtime_plugins, stop_point)
            // Create a parent span for the entire operation. Includes a random, internal-only,
            // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
            .instrument(::tracing::debug_span!(
                "Route 53.CreateHostedZone",
                "rpc.service" = "Route 53",
                "rpc.method" = "CreateHostedZone",
                "sdk_invocation_id" = ::fastrand::u32(1_000_000..10_000_000),
                "rpc.system" = "aws-api",
            ))
            .await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());

        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for CreateHostedZone {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("CreateHostedZone");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            CreateHostedZoneRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            CreateHostedZoneResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            crate::config::auth::Params::builder()
                .operation_name("CreateHostedZone")
                .build()
                .expect("required fields set"),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "CreateHostedZone",
            "Route 53",
        ));
        let mut signing_options = ::aws_runtime::auth::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::SigV4OperationSigningConfig {
            signing_options,
            ..::std::default::Default::default()
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(
        &self,
        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        #[allow(unused_mut)]
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("CreateHostedZone")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(CreateHostedZoneEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::create_hosted_zone::CreateHostedZoneError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::create_hosted_zone::CreateHostedZoneError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::create_hosted_zone::CreateHostedZoneError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct CreateHostedZoneResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for CreateHostedZoneResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        #[allow(unused_mut)]
        let mut force_error = false;
        ::tracing::debug!(request_id = ?::aws_types::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 201 || force_error {
            crate::protocol_serde::shape_create_hosted_zone::de_create_hosted_zone_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_create_hosted_zone::de_create_hosted_zone_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct CreateHostedZoneRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for CreateHostedZoneRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::create_hosted_zone::CreateHostedZoneInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::create_hosted_zone::CreateHostedZoneInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/2013-04-01/hostedzone").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_hosted_zone::CreateHostedZoneInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/xml");
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(crate::protocol_serde::shape_create_hosted_zone::ser_create_hosted_zone_op_input(&input)?);
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct CreateHostedZoneEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for CreateHostedZoneEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "CreateHostedZoneEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context
            .input()
            .downcast_ref::<CreateHostedZoneInput>()
            .ok_or("failed to downcast to CreateHostedZoneInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}

// The get_* functions below are generated from JMESPath expressions in the
// operationContextParams trait. They target the operation's input shape.

/// Error type for the `CreateHostedZoneError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum CreateHostedZoneError {
    /// <p>The cause of this error depends on the operation that you're performing:</p>
    /// <ul>
    /// <li>
    /// <p><b>Create a public hosted zone:</b> Two hosted zones that have the same name or that have a parent/child relationship (example.com and test.example.com) can't have any common name servers. You tried to create a hosted zone that has the same name as an existing hosted zone or that's the parent or child of an existing hosted zone, and you specified a delegation set that shares one or more name servers with the existing hosted zone. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateReusableDelegationSet.html">CreateReusableDelegationSet</a>.</p></li>
    /// <li>
    /// <p><b>Create a private hosted zone:</b> A hosted zone with the specified name already exists and is already associated with the Amazon VPC that you specified.</p></li>
    /// <li>
    /// <p><b>Associate VPCs with a private hosted zone:</b> The VPC that you specified is already associated with another hosted zone that has the same name.</p></li>
    /// </ul>
    ConflictingDomainExists(crate::types::error::ConflictingDomainExists),
    /// <p>You can create a hosted zone that has the same name as an existing hosted zone (example.com is common), but there is a limit to the number of hosted zones that have the same name. If you get this error, Amazon Route 53 has reached that limit. If you own the domain name and Route 53 generates this error, contact Customer Support.</p>
    DelegationSetNotAvailable(crate::types::error::DelegationSetNotAvailable),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    DelegationSetNotReusable(crate::types::error::DelegationSetNotReusable),
    /// <p>The hosted zone you're trying to create already exists. Amazon Route 53 returns this error when a hosted zone has already been created with the specified <code>CallerReference</code>.</p>
    HostedZoneAlreadyExists(crate::types::error::HostedZoneAlreadyExists),
    /// <p>The specified domain name is not valid.</p>
    InvalidDomainName(crate::types::error::InvalidDomainName),
    /// <p>The input is not valid.</p>
    InvalidInput(crate::types::error::InvalidInput),
    /// <p>The VPC ID that you specified either isn't a valid ID or the current account is not authorized to access this VPC.</p>
    InvalidVpcId(crate::types::error::InvalidVpcId),
    /// <p>A reusable delegation set with the specified ID does not exist.</p>
    NoSuchDelegationSet(crate::types::error::NoSuchDelegationSet),
    /// <p>This operation can't be completed either because the current account has reached the limit on the number of hosted zones or because you've reached the limit on the number of hosted zones that can be associated with a reusable delegation set.</p>
    /// <p>For information about default limits, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/DNSLimitations.html">Limits</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
    /// <p>To get the current limit on hosted zones that can be created by an account, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetAccountLimit.html">GetAccountLimit</a>.</p>
    /// <p>To get the current limit on hosted zones that can be associated with a reusable delegation set, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetReusableDelegationSetLimit.html">GetReusableDelegationSetLimit</a>.</p>
    /// <p>To request a higher limit, <a href="http://aws.amazon.com/route53-request">create a case</a> with the Amazon Web Services Support Center.</p>
    TooManyHostedZones(crate::types::error::TooManyHostedZones),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-CreateHostedZoneError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl CreateHostedZoneError {
    /// Creates the `CreateHostedZoneError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `CreateHostedZoneError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.clone().into(),
            meta: err,
        })
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ConflictingDomainExists(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DelegationSetNotAvailable(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::DelegationSetNotReusable(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::HostedZoneAlreadyExists(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidDomainName(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidInput(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidVpcId(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::NoSuchDelegationSet(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::TooManyHostedZones(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `CreateHostedZoneError::ConflictingDomainExists`.
    pub fn is_conflicting_domain_exists(&self) -> bool {
        matches!(self, Self::ConflictingDomainExists(_))
    }
    /// Returns `true` if the error kind is `CreateHostedZoneError::DelegationSetNotAvailable`.
    pub fn is_delegation_set_not_available(&self) -> bool {
        matches!(self, Self::DelegationSetNotAvailable(_))
    }
    /// Returns `true` if the error kind is `CreateHostedZoneError::DelegationSetNotReusable`.
    pub fn is_delegation_set_not_reusable(&self) -> bool {
        matches!(self, Self::DelegationSetNotReusable(_))
    }
    /// Returns `true` if the error kind is `CreateHostedZoneError::HostedZoneAlreadyExists`.
    pub fn is_hosted_zone_already_exists(&self) -> bool {
        matches!(self, Self::HostedZoneAlreadyExists(_))
    }
    /// Returns `true` if the error kind is `CreateHostedZoneError::InvalidDomainName`.
    pub fn is_invalid_domain_name(&self) -> bool {
        matches!(self, Self::InvalidDomainName(_))
    }
    /// Returns `true` if the error kind is `CreateHostedZoneError::InvalidInput`.
    pub fn is_invalid_input(&self) -> bool {
        matches!(self, Self::InvalidInput(_))
    }
    /// Returns `true` if the error kind is `CreateHostedZoneError::InvalidVpcId`.
    pub fn is_invalid_vpc_id(&self) -> bool {
        matches!(self, Self::InvalidVpcId(_))
    }
    /// Returns `true` if the error kind is `CreateHostedZoneError::NoSuchDelegationSet`.
    pub fn is_no_such_delegation_set(&self) -> bool {
        matches!(self, Self::NoSuchDelegationSet(_))
    }
    /// Returns `true` if the error kind is `CreateHostedZoneError::TooManyHostedZones`.
    pub fn is_too_many_hosted_zones(&self) -> bool {
        matches!(self, Self::TooManyHostedZones(_))
    }
}
impl ::std::error::Error for CreateHostedZoneError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::ConflictingDomainExists(_inner) => ::std::option::Option::Some(_inner),
            Self::DelegationSetNotAvailable(_inner) => ::std::option::Option::Some(_inner),
            Self::DelegationSetNotReusable(_inner) => ::std::option::Option::Some(_inner),
            Self::HostedZoneAlreadyExists(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidDomainName(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidInput(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidVpcId(_inner) => ::std::option::Option::Some(_inner),
            Self::NoSuchDelegationSet(_inner) => ::std::option::Option::Some(_inner),
            Self::TooManyHostedZones(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for CreateHostedZoneError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::ConflictingDomainExists(_inner) => _inner.fmt(f),
            Self::DelegationSetNotAvailable(_inner) => _inner.fmt(f),
            Self::DelegationSetNotReusable(_inner) => _inner.fmt(f),
            Self::HostedZoneAlreadyExists(_inner) => _inner.fmt(f),
            Self::InvalidDomainName(_inner) => _inner.fmt(f),
            Self::InvalidInput(_inner) => _inner.fmt(f),
            Self::InvalidVpcId(_inner) => _inner.fmt(f),
            Self::NoSuchDelegationSet(_inner) => _inner.fmt(f),
            Self::TooManyHostedZones(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for CreateHostedZoneError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateHostedZoneError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ConflictingDomainExists(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DelegationSetNotAvailable(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::DelegationSetNotReusable(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::HostedZoneAlreadyExists(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidDomainName(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidInput(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidVpcId(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::NoSuchDelegationSet(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::TooManyHostedZones(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for CreateHostedZoneError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source,
            meta: meta.unwrap_or_default(),
        })
    }
}
impl ::aws_types::request_id::RequestId for crate::operation::create_hosted_zone::CreateHostedZoneError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::create_hosted_zone::_create_hosted_zone_output::CreateHostedZoneOutput;

pub use crate::operation::create_hosted_zone::_create_hosted_zone_input::CreateHostedZoneInput;

mod _create_hosted_zone_input;

mod _create_hosted_zone_output;

/// Builders
pub mod builders;
