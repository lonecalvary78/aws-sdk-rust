// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_start_viewer_session_revocation_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationOutput,
    crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServerException" => crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "PendingVerification" => crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::PendingVerification({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::PendingVerificationBuilder::default();
                output = crate::protocol_serde::shape_pending_verification::de_pending_verification_json_err(_response_body, output)
                    .map_err(crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ValidationException" => crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_viewer_session_revocation_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationOutput,
    crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::start_viewer_session_revocation::builders::StartViewerSessionRevocationOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_start_viewer_session_revocation_input(
    input: &crate::operation::start_viewer_session_revocation::StartViewerSessionRevocationInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_viewer_session_revocation_input::ser_start_viewer_session_revocation_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
