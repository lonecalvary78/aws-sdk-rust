// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `UpdateAuthEventFeedback`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateAuthEventFeedback;
impl UpdateAuthEventFeedback {
    /// Creates a new `UpdateAuthEventFeedback`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackInput,
    ) -> ::std::result::Result<
        crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackInput,
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
            "Cognito Identity Provider",
            "UpdateAuthEventFeedback",
            input,
            runtime_plugins,
            stop_point,
        )
        // Create a parent span for the entire operation. Includes a random, internal-only,
        // seven-digit ID for the operation orchestration so that it can be correlated in the logs.
        .instrument(::tracing::debug_span!(
            "Cognito Identity Provider.UpdateAuthEventFeedback",
            "rpc.service" = "Cognito Identity Provider",
            "rpc.method" = "UpdateAuthEventFeedback",
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for UpdateAuthEventFeedback {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("UpdateAuthEventFeedback");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            UpdateAuthEventFeedbackRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            UpdateAuthEventFeedbackResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            crate::config::auth::Params::builder()
                .operation_name("UpdateAuthEventFeedback")
                .build()
                .expect("required fields set"),
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::orchestrator::Metadata::new(
            "UpdateAuthEventFeedback",
            "Cognito Identity Provider",
        ));

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(
        &self,
        _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder,
    ) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        #[allow(unused_mut)]
        let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("UpdateAuthEventFeedback")
            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::default())
            .with_interceptor(UpdateAuthEventFeedbackEndpointParamsInterceptor)
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<
                crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackError,
            >::new())
            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<
                crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackError,
            >::new())
            .with_retry_classifier(::aws_runtime::retries::classifiers::AwsErrorCodeClassifier::<
                crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackError,
            >::new());

        ::std::borrow::Cow::Owned(rcb)
    }
}

#[derive(Debug)]
struct UpdateAuthEventFeedbackResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for UpdateAuthEventFeedbackResponseDeserializer {
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
            crate::protocol_serde::shape_update_auth_event_feedback::de_update_auth_event_feedback_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_update_auth_event_feedback::de_update_auth_event_feedback_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct UpdateAuthEventFeedbackRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for UpdateAuthEventFeedbackRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackInput,
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
                "AWSCognitoIdentityProviderService.UpdateAuthEventFeedback",
            );
            builder
        };
        let body = ::aws_smithy_types::body::SdkBody::from(
            crate::protocol_serde::shape_update_auth_event_feedback::ser_update_auth_event_feedback_input(&input)?,
        );
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
    }
}
#[derive(Debug)]
struct UpdateAuthEventFeedbackEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Intercept for UpdateAuthEventFeedbackEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "UpdateAuthEventFeedbackEndpointParamsInterceptor"
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
            .downcast_ref::<UpdateAuthEventFeedbackInput>()
            .ok_or("failed to downcast to UpdateAuthEventFeedbackInput")?;

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

/// Error type for the `UpdateAuthEventFeedbackError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum UpdateAuthEventFeedbackError {
    /// <p>This exception is thrown when Amazon Cognito encounters an internal error.</p>
    InternalErrorException(crate::types::error::InternalErrorException),
    /// <p>This exception is thrown when the Amazon Cognito service encounters an invalid parameter.</p>
    InvalidParameterException(crate::types::error::InvalidParameterException),
    /// <p>This exception is thrown when a user isn't authorized.</p>
    NotAuthorizedException(crate::types::error::NotAuthorizedException),
    /// <p>This exception is thrown when the Amazon Cognito service can't find the requested resource.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>This exception is thrown when the user has made too many requests for a given operation.</p>
    TooManyRequestsException(crate::types::error::TooManyRequestsException),
    /// <p>This exception is thrown when a user isn't found.</p>
    UserNotFoundException(crate::types::error::UserNotFoundException),
    /// <p>This exception is thrown when user pool add-ons aren't enabled.</p>
    UserPoolAddOnNotEnabledException(crate::types::error::UserPoolAddOnNotEnabledException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-UpdateAuthEventFeedbackError) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl UpdateAuthEventFeedbackError {
    /// Creates the `UpdateAuthEventFeedbackError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: err.into(),
            meta: ::std::default::Default::default(),
        })
    }

    /// Creates the `UpdateAuthEventFeedbackError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
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
            Self::InternalErrorException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::InvalidParameterException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::NotAuthorizedException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::ResourceNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::TooManyRequestsException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::UserNotFoundException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::UserPoolAddOnNotEnabledException(e) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(e),
            Self::Unhandled(e) => &e.meta,
        }
    }
    /// Returns `true` if the error kind is `UpdateAuthEventFeedbackError::InternalErrorException`.
    pub fn is_internal_error_exception(&self) -> bool {
        matches!(self, Self::InternalErrorException(_))
    }
    /// Returns `true` if the error kind is `UpdateAuthEventFeedbackError::InvalidParameterException`.
    pub fn is_invalid_parameter_exception(&self) -> bool {
        matches!(self, Self::InvalidParameterException(_))
    }
    /// Returns `true` if the error kind is `UpdateAuthEventFeedbackError::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(self, Self::NotAuthorizedException(_))
    }
    /// Returns `true` if the error kind is `UpdateAuthEventFeedbackError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `UpdateAuthEventFeedbackError::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(self, Self::TooManyRequestsException(_))
    }
    /// Returns `true` if the error kind is `UpdateAuthEventFeedbackError::UserNotFoundException`.
    pub fn is_user_not_found_exception(&self) -> bool {
        matches!(self, Self::UserNotFoundException(_))
    }
    /// Returns `true` if the error kind is `UpdateAuthEventFeedbackError::UserPoolAddOnNotEnabledException`.
    pub fn is_user_pool_add_on_not_enabled_exception(&self) -> bool {
        matches!(self, Self::UserPoolAddOnNotEnabledException(_))
    }
}
impl ::std::error::Error for UpdateAuthEventFeedbackError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InternalErrorException(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidParameterException(_inner) => ::std::option::Option::Some(_inner),
            Self::NotAuthorizedException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::TooManyRequestsException(_inner) => ::std::option::Option::Some(_inner),
            Self::UserNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::UserPoolAddOnNotEnabledException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(&*_inner.source),
        }
    }
}
impl ::std::fmt::Display for UpdateAuthEventFeedbackError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InternalErrorException(_inner) => _inner.fmt(f),
            Self::InvalidParameterException(_inner) => _inner.fmt(f),
            Self::NotAuthorizedException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::TooManyRequestsException(_inner) => _inner.fmt(f),
            Self::UserNotFoundException(_inner) => _inner.fmt(f),
            Self::UserPoolAddOnNotEnabledException(_inner) => _inner.fmt(f),
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
impl ::aws_smithy_types::retry::ProvideErrorKind for UpdateAuthEventFeedbackError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for UpdateAuthEventFeedbackError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InternalErrorException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidParameterException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::NotAuthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::TooManyRequestsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::UserNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::UserPoolAddOnNotEnabledException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => &_inner.meta,
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for UpdateAuthEventFeedbackError {
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
impl ::aws_types::request_id::RequestId for crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}

pub use crate::operation::update_auth_event_feedback::_update_auth_event_feedback_output::UpdateAuthEventFeedbackOutput;

pub use crate::operation::update_auth_event_feedback::_update_auth_event_feedback_input::UpdateAuthEventFeedbackInput;

mod _update_auth_event_feedback_input;

mod _update_auth_event_feedback_output;

/// Builders
pub mod builders;
