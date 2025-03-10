// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_continue_deployment_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::continue_deployment::ContinueDeploymentOutput,
    crate::operation::continue_deployment::ContinueDeploymentError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::continue_deployment::ContinueDeploymentError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::continue_deployment::ContinueDeploymentError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DeploymentAlreadyCompletedException" => {
            crate::operation::continue_deployment::ContinueDeploymentError::DeploymentAlreadyCompletedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DeploymentAlreadyCompletedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_deployment_already_completed_exception::de_deployment_already_completed_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::continue_deployment::ContinueDeploymentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "DeploymentDoesNotExistException" => crate::operation::continue_deployment::ContinueDeploymentError::DeploymentDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DeploymentDoesNotExistExceptionBuilder::default();
                output = crate::protocol_serde::shape_deployment_does_not_exist_exception::de_deployment_does_not_exist_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::continue_deployment::ContinueDeploymentError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DeploymentIdRequiredException" => crate::operation::continue_deployment::ContinueDeploymentError::DeploymentIdRequiredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DeploymentIdRequiredExceptionBuilder::default();
                output = crate::protocol_serde::shape_deployment_id_required_exception::de_deployment_id_required_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::continue_deployment::ContinueDeploymentError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DeploymentIsNotInReadyStateException" => {
            crate::operation::continue_deployment::ContinueDeploymentError::DeploymentIsNotInReadyStateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DeploymentIsNotInReadyStateExceptionBuilder::default();
                    output = crate::protocol_serde::shape_deployment_is_not_in_ready_state_exception::de_deployment_is_not_in_ready_state_exception_json_err(_response_body, output).map_err(crate::operation::continue_deployment::ContinueDeploymentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidDeploymentIdException" => crate::operation::continue_deployment::ContinueDeploymentError::InvalidDeploymentIdException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidDeploymentIdExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_deployment_id_exception::de_invalid_deployment_id_exception_json_err(_response_body, output)
                        .map_err(crate::operation::continue_deployment::ContinueDeploymentError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidDeploymentStatusException" => crate::operation::continue_deployment::ContinueDeploymentError::InvalidDeploymentStatusException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidDeploymentStatusExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_deployment_status_exception::de_invalid_deployment_status_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::continue_deployment::ContinueDeploymentError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidDeploymentWaitTypeException" => crate::operation::continue_deployment::ContinueDeploymentError::InvalidDeploymentWaitTypeException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidDeploymentWaitTypeExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_deployment_wait_type_exception::de_invalid_deployment_wait_type_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::continue_deployment::ContinueDeploymentError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedActionForDeploymentTypeException" => {
            crate::operation::continue_deployment::ContinueDeploymentError::UnsupportedActionForDeploymentTypeException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedActionForDeploymentTypeExceptionBuilder::default();
                    output = crate::protocol_serde::shape_unsupported_action_for_deployment_type_exception::de_unsupported_action_for_deployment_type_exception_json_err(_response_body, output).map_err(crate::operation::continue_deployment::ContinueDeploymentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::continue_deployment::ContinueDeploymentError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_continue_deployment_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::continue_deployment::ContinueDeploymentOutput,
    crate::operation::continue_deployment::ContinueDeploymentError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::continue_deployment::builders::ContinueDeploymentOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_continue_deployment_input(
    input: &crate::operation::continue_deployment::ContinueDeploymentInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_continue_deployment_input::ser_continue_deployment_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
