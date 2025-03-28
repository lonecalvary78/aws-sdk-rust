// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_ml_model_transform_job_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_ml_model_transform_job::GetMlModelTransformJobOutput,
    crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::bad_request_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ClientTimeoutException" => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::ClientTimeoutException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClientTimeoutExceptionBuilder::default();
                output = crate::protocol_serde::shape_client_timeout_exception::de_client_timeout_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::client_timeout_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ConstraintViolationException" => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::ConstraintViolationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConstraintViolationExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_constraint_violation_exception::de_constraint_violation_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::constraint_violation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "IllegalArgumentException" => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::IllegalArgumentException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::IllegalArgumentExceptionBuilder::default();
                output = crate::protocol_serde::shape_illegal_argument_exception::de_illegal_argument_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::illegal_argument_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArgumentException" => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::InvalidArgumentException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArgumentExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::invalid_argument_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::invalid_parameter_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "MissingParameterException" => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::MissingParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::MissingParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_missing_parameter_exception::de_missing_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::missing_parameter_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "MLResourceNotFoundException" => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::MlResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::MlResourceNotFoundExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_ml_resource_not_found_exception::de_ml_resource_not_found_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::ml_resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "PreconditionsFailedException" => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::PreconditionsFailedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::PreconditionsFailedExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_preconditions_failed_exception::de_preconditions_failed_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::preconditions_failed_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::too_many_requests_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedOperationException" => {
            crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::UnsupportedOperationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedOperationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::unsupported_operation_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_ml_model_transform_job_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_ml_model_transform_job::GetMlModelTransformJobOutput,
    crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_ml_model_transform_job::builders::GetMlModelTransformJobOutputBuilder::default();
        output = crate::protocol_serde::shape_get_ml_model_transform_job::de_get_ml_model_transform_job(_response_body, output)
            .map_err(crate::operation::get_ml_model_transform_job::GetMLModelTransformJobError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_ml_model_transform_job(
    value: &[u8],
    mut builder: crate::operation::get_ml_model_transform_job::builders::GetMlModelTransformJobOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_ml_model_transform_job::builders::GetMlModelTransformJobOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "baseProcessingJob" => {
                    builder =
                        builder.set_base_processing_job(crate::protocol_serde::shape_ml_resource_definition::de_ml_resource_definition(tokens)?);
                }
                "id" => {
                    builder = builder.set_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "models" => {
                    builder = builder.set_models(crate::protocol_serde::shape_models::de_models(tokens)?);
                }
                "remoteModelTransformJob" => {
                    builder = builder
                        .set_remote_model_transform_job(crate::protocol_serde::shape_ml_resource_definition::de_ml_resource_definition(tokens)?);
                }
                "status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
