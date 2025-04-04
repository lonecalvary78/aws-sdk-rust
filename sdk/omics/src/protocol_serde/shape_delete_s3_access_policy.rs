// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_s3_access_policy_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyOutput,
    crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?
            };
            tmp
        }),
        "NotSupportedOperationException" => crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::NotSupportedOperationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotSupportedOperationExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_supported_operation_exception::de_not_supported_operation_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::not_supported_operation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?
            };
            tmp
        }),
        "RequestTimeoutException" => crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::RequestTimeoutException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RequestTimeoutExceptionBuilder::default();
                output = crate::protocol_serde::shape_request_timeout_exception::de_request_timeout_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::request_timeout_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_s3_access_policy_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyOutput,
    crate::operation::delete_s3_access_policy::DeleteS3AccessPolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_s3_access_policy::builders::DeleteS3AccessPolicyOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
