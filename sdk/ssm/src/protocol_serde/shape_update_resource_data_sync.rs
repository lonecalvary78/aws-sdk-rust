// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_resource_data_sync_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_resource_data_sync::UpdateResourceDataSyncOutput,
    crate::operation::update_resource_data_sync::UpdateResourceDataSyncError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerError" => crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
                    .map_err(crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceDataSyncConflictException" => {
            crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::ResourceDataSyncConflictException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceDataSyncConflictExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_data_sync_conflict_exception::de_resource_data_sync_conflict_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceDataSyncInvalidConfigurationException" => {
            crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::ResourceDataSyncInvalidConfigurationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceDataSyncInvalidConfigurationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_data_sync_invalid_configuration_exception::de_resource_data_sync_invalid_configuration_exception_json_err(_response_body, output).map_err(crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceDataSyncNotFoundException" => {
            crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::ResourceDataSyncNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceDataSyncNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_data_sync_not_found_exception::de_resource_data_sync_not_found_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::update_resource_data_sync::UpdateResourceDataSyncError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_resource_data_sync_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_resource_data_sync::UpdateResourceDataSyncOutput,
    crate::operation::update_resource_data_sync::UpdateResourceDataSyncError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_resource_data_sync::builders::UpdateResourceDataSyncOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_update_resource_data_sync_input(
    input: &crate::operation::update_resource_data_sync::UpdateResourceDataSyncInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_resource_data_sync_input::ser_update_resource_data_sync_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
