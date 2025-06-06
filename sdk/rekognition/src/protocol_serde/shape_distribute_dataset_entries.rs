// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_distribute_dataset_entries_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::distribute_dataset_entries::DistributeDatasetEntriesOutput,
    crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServerError" => crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
                    .map_err(crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ProvisionedThroughputExceededException" => {
            crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::ProvisionedThroughputExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotReadyException" => crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::ResourceNotReadyException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotReadyExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_ready_exception::de_resource_not_ready_exception_json_err(_response_body, output)
                    .map_err(crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottlingException" => crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_distribute_dataset_entries_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::distribute_dataset_entries::DistributeDatasetEntriesOutput,
    crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::distribute_dataset_entries::builders::DistributeDatasetEntriesOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_distribute_dataset_entries_input(
    input: &crate::operation::distribute_dataset_entries::DistributeDatasetEntriesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_distribute_dataset_entries_input::ser_distribute_dataset_entries_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
