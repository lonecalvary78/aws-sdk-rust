// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_start_logging_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::start_logging::StartLoggingOutput, crate::operation::start_logging::StartLoggingError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::start_logging::StartLoggingError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CloudTrailARNInvalidException" => crate::operation::start_logging::StartLoggingError::CloudTrailArnInvalidException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CloudTrailArnInvalidExceptionBuilder::default();
                output = crate::protocol_serde::shape_cloud_trail_arn_invalid_exception::de_cloud_trail_arn_invalid_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ConflictException" => crate::operation::start_logging::StartLoggingError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InsufficientDependencyServiceAccessPermissionException" => {
            crate::operation::start_logging::StartLoggingError::InsufficientDependencyServiceAccessPermissionException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InsufficientDependencyServiceAccessPermissionExceptionBuilder::default();
                    output = crate::protocol_serde::shape_insufficient_dependency_service_access_permission_exception::de_insufficient_dependency_service_access_permission_exception_json_err(_response_body, output).map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidHomeRegionException" => crate::operation::start_logging::StartLoggingError::InvalidHomeRegionException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidHomeRegionExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_home_region_exception::de_invalid_home_region_exception_json_err(_response_body, output)
                        .map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidTrailNameException" => crate::operation::start_logging::StartLoggingError::InvalidTrailNameException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidTrailNameExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_trail_name_exception::de_invalid_trail_name_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoManagementAccountSLRExistsException" => {
            crate::operation::start_logging::StartLoggingError::NoManagementAccountSlrExistsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoManagementAccountSlrExistsExceptionBuilder::default();
                    output = crate::protocol_serde::shape_no_management_account_slr_exists_exception::de_no_management_account_slr_exists_exception_json_err(_response_body, output).map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotOrganizationMasterAccountException" => {
            crate::operation::start_logging::StartLoggingError::NotOrganizationMasterAccountException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotOrganizationMasterAccountExceptionBuilder::default();
                    output = crate::protocol_serde::shape_not_organization_master_account_exception::de_not_organization_master_account_exception_json_err(_response_body, output).map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "OperationNotPermittedException" => crate::operation::start_logging::StartLoggingError::OperationNotPermittedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::OperationNotPermittedExceptionBuilder::default();
                output = crate::protocol_serde::shape_operation_not_permitted_exception::de_operation_not_permitted_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottlingException" => crate::operation::start_logging::StartLoggingError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TrailNotFoundException" => crate::operation::start_logging::StartLoggingError::TrailNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TrailNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_trail_not_found_exception::de_trail_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedOperationException" => crate::operation::start_logging::StartLoggingError::UnsupportedOperationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnsupportedOperationExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
                        .map_err(crate::operation::start_logging::StartLoggingError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::start_logging::StartLoggingError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_logging_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::start_logging::StartLoggingOutput, crate::operation::start_logging::StartLoggingError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::start_logging::builders::StartLoggingOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_start_logging_input(
    input: &crate::operation::start_logging::StartLoggingInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_logging_input::ser_start_logging_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
