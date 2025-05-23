// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_matchmaking_configuration_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationOutput,
    crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => {
            crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServiceExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(_response_body, output)
                        .map_err(crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRequestException" => {
            crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::InvalidRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                        .map_err(crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotFoundException" => crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TaggingFailedException" => {
            crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::TaggingFailedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TaggingFailedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_tagging_failed_exception::de_tagging_failed_exception_json_err(_response_body, output)
                        .map_err(crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnsupportedRegionException" => {
            crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::UnsupportedRegionException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedRegionExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_unsupported_region_exception::de_unsupported_region_exception_json_err(_response_body, output)
                            .map_err(crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_matchmaking_configuration_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationOutput,
    crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_matchmaking_configuration::builders::DeleteMatchmakingConfigurationOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_delete_matchmaking_configuration_input(
    input: &crate::operation::delete_matchmaking_configuration::DeleteMatchmakingConfigurationInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_matchmaking_configuration_input::ser_delete_matchmaking_configuration_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
