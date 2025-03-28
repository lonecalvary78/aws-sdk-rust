// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_send_workflow_step_action_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::send_workflow_step_action::SendWorkflowStepActionOutput,
    crate::operation::send_workflow_step_action::SendWorkflowStepActionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CallRateLimitExceededException" => {
            crate::operation::send_workflow_step_action::SendWorkflowStepActionError::CallRateLimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CallRateLimitExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_call_rate_limit_exceeded_exception::de_call_rate_limit_exceeded_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ClientException" => crate::operation::send_workflow_step_action::SendWorkflowStepActionError::ClientException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClientExceptionBuilder::default();
                output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(_response_body, output)
                    .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ForbiddenException" => crate::operation::send_workflow_step_action::SendWorkflowStepActionError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ForbiddenExceptionBuilder::default();
                output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(_response_body, output)
                    .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "IdempotentParameterMismatchException" => {
            crate::operation::send_workflow_step_action::SendWorkflowStepActionError::IdempotentParameterMismatchException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::IdempotentParameterMismatchExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_idempotent_parameter_mismatch_exception::de_idempotent_parameter_mismatch_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterValueException" => {
            crate::operation::send_workflow_step_action::SendWorkflowStepActionError::InvalidParameterValueException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRequestException" => crate::operation::send_workflow_step_action::SendWorkflowStepActionError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceInUseException" => crate::operation::send_workflow_step_action::SendWorkflowStepActionError::ResourceInUseException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceInUseExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
                    .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::send_workflow_step_action::SendWorkflowStepActionError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceException" => crate::operation::send_workflow_step_action::SendWorkflowStepActionError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceUnavailableException" => crate::operation::send_workflow_step_action::SendWorkflowStepActionError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(_response_body, output)
                        .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::send_workflow_step_action::SendWorkflowStepActionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_send_workflow_step_action_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::send_workflow_step_action::SendWorkflowStepActionOutput,
    crate::operation::send_workflow_step_action::SendWorkflowStepActionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::send_workflow_step_action::builders::SendWorkflowStepActionOutputBuilder::default();
        output = crate::protocol_serde::shape_send_workflow_step_action::de_send_workflow_step_action(_response_body, output)
            .map_err(crate::operation::send_workflow_step_action::SendWorkflowStepActionError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_send_workflow_step_action_input(
    input: &crate::operation::send_workflow_step_action::SendWorkflowStepActionInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_send_workflow_step_action_input::ser_send_workflow_step_action_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_send_workflow_step_action(
    value: &[u8],
    mut builder: crate::operation::send_workflow_step_action::builders::SendWorkflowStepActionOutputBuilder,
) -> ::std::result::Result<
    crate::operation::send_workflow_step_action::builders::SendWorkflowStepActionOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "clientToken" => {
                    builder = builder.set_client_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "imageBuildVersionArn" => {
                    builder = builder.set_image_build_version_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "stepExecutionId" => {
                    builder = builder.set_step_execution_id(
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
