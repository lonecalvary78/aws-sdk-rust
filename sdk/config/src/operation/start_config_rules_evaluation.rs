// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `StartConfigRulesEvaluation`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartConfigRulesEvaluation;
impl StartConfigRulesEvaluation {
    /// Creates a new `StartConfigRulesEvaluation`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput,
    ) -> ::std::result::Result<
        crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput,
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
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
            "Config Service",
            "StartConfigRulesEvaluation",
            input,
            runtime_plugins,
            stop_point,
        )
        // Create a parent span for the entire operation. Includes a random, internal-only,
        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
        .instrument(::tracing::debug_span!(
            "Config Service.StartConfigRulesEvaluation",
            "rpc.service" = "Config Service",
            "rpc.method" = "StartConfigRulesEvaluation",
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for StartConfigRulesEvaluation {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("StartConfigRulesEvaluation");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            StartConfigRulesEvaluationRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            StartConfigRulesEvaluationResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            crate::config::auth::Params::builder()
                .operation_name("StartConfigRulesEvaluation")
                .build()
                .expect("required fields set"),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "StartConfigRulesEvaluation",
            "Config Service",
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
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("StartConfigRulesEvaluation")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(StartConfigRulesEvaluationEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct StartConfigRulesEvaluationResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for StartConfigRulesEvaluationResponseDeserializer {
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
        let parse_result = if !success && status != 200 || force_error {
            crate::protocol_serde::shape_start_config_rules_evaluation::de_start_config_rules_evaluation_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_start_config_rules_evaluation::de_start_config_rules_evaluation_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct StartConfigRulesEvaluationRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for StartConfigRulesEvaluationRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/x-amz-json-1.1");
            builder = _header_serialization_settings.set_default_header(
                builder,
                ::http::header::HeaderName::from_static("x-amz-target"),
                "StarlingDoveService.StartConfigRulesEvaluation",
            );
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(
            crate::protocol_serde::shape_start_config_rules_evaluation::ser_start_config_rules_evaluation_input(&input)?,
        );
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct StartConfigRulesEvaluationEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for StartConfigRulesEvaluationEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "StartConfigRulesEvaluationEndpointParamsInterceptor"
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
            .downcast_ref::<StartConfigRulesEvaluationInput>()
            .ok_or("failed to downcast to StartConfigRulesEvaluationInput")?;

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

/// Error type for the `StartConfigRulesEvaluationError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum StartConfigRulesEvaluationError {
    /// <p>One or more of the specified parameters are not valid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValueException(crate::types::error::InvalidParameterValueException),
    /// <p>For <code>PutServiceLinkedConfigurationRecorder</code> API, this exception is thrown if the number of service-linked roles in the account exceeds the limit.</p>
    /// <p>For <code>StartConfigRulesEvaluation</code> API, this exception is thrown if an evaluation is in progress or if you call the <code>StartConfigRulesEvaluation</code> API more than once per minute.</p>
    /// <p>For <code>PutConfigurationAggregator</code> API, this exception is thrown if the number of accounts and aggregators exceeds the limit.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// <p>The Config rule in the request is not valid. Verify that the rule is an Config Process Check rule, that the rule name is correct, and that valid Amazon Resouce Names (ARNs) are used before trying again.</p>
    NoSuchConfigRuleException(crate::types::error::NoSuchConfigRuleException),
    /// <p>You see this exception in the following cases:</p>
    /// <ul>
    /// <li>
    /// <p>For DeleteConfigRule, Config is deleting this rule. Try your request again later.</p></li>
    /// <li>
    /// <p>For DeleteConfigRule, the rule is deleting your evaluation results. Try your request again later.</p></li>
    /// <li>
    /// <p>For DeleteConfigRule, a remediation action is associated with the rule and Config cannot delete this rule. Delete the remediation action associated with the rule before deleting the rule and try your request again later.</p></li>
    /// <li>
    /// <p>For PutConfigOrganizationRule, organization Config rule deletion is in progress. Try your request again later.</p></li>
    /// <li>
    /// <p>For DeleteOrganizationConfigRule, organization Config rule creation is in progress. Try your request again later.</p></li>
    /// <li>
    /// <p>For PutConformancePack and PutOrganizationConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p></li>
    /// <li>
    /// <p>For DeleteConformancePack, a conformance pack creation, update, and deletion is in progress. Try your request again later.</p></li>
    /// </ul>
    ResourceInUseException(crate::types::error::ResourceInUseException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-StartConfigRulesEvaluationError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl StartConfigRulesEvaluationError {
    /// Creates the `StartConfigRulesEvaluationError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `StartConfigRulesEvaluationError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
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
            Self::InvalidParameterValueException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::LimitExceededException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::NoSuchConfigRuleException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ResourceInUseException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `StartConfigRulesEvaluationError::InvalidParameterValueException`.
    pub fn is_invalid_parameter_value_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterValueException(_))
    }
    /// Returns `true` if the error kind is `StartConfigRulesEvaluationError::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::LimitExceededException(_))
    }
    /// Returns `true` if the error kind is `StartConfigRulesEvaluationError::NoSuchConfigRuleException`.
    pub fn is_no_such_config_rule_exception(&self) -> bool {
        matches!(self, Self::NoSuchConfigRuleException(_))
    }
    /// Returns `true` if the error kind is `StartConfigRulesEvaluationError::ResourceInUseException`.
    pub fn is_resource_in_use_exception(&self) -> bool {
        matches!(self, Self::ResourceInUseException(_))
    }
}
impl ::std::error::Error for StartConfigRulesEvaluationError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InvalidParameterValueException(_inner) => ::std::option::Option::Some(_inner),
            Self::LimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::NoSuchConfigRuleException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceInUseException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for StartConfigRulesEvaluationError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InvalidParameterValueException(_inner) => _inner.fmt(f),
            Self::LimitExceededException(_inner) => _inner.fmt(f),
            Self::NoSuchConfigRuleException(_inner) => _inner.fmt(f),
            Self::ResourceInUseException(_inner) => _inner.fmt(f),
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
impl ::aws_smithy_types::retry::ProvideErrorKind for StartConfigRulesEvaluationError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for StartConfigRulesEvaluationError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InvalidParameterValueException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::LimitExceededException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::NoSuchConfigRuleException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ResourceInUseException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for StartConfigRulesEvaluationError {
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
impl ::aws_types::request_id::RequestId for crate::operation::start_config_rules_evaluation::StartConfigRulesEvaluationError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::start_config_rules_evaluation::_start_config_rules_evaluation_output::StartConfigRulesEvaluationOutput;

pub use crate::operation::start_config_rules_evaluation::_start_config_rules_evaluation_input::StartConfigRulesEvaluationInput;

mod _start_config_rules_evaluation_input;

mod _start_config_rules_evaluation_output;

/// Builders
pub mod builders;
