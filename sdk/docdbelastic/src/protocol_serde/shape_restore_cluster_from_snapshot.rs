// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_restore_cluster_from_snapshot_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotOutput,
    crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?
            };
            tmp
        }),
        "ConflictException" => crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::conflict_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?
            };
            tmp
        }),
        "ServiceQuotaExceededException" => {
            crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::ServiceQuotaExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceQuotaExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_service_quota_exceeded_exception::de_service_quota_exceeded_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::service_quota_exceeded_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?
                };
                tmp
            })
        }
        "ThrottlingException" => crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled(
                            "Failed to parse retryAfterSeconds from header `Retry-After",
                        )
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_restore_cluster_from_snapshot_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotOutput,
    crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::restore_cluster_from_snapshot::builders::RestoreClusterFromSnapshotOutputBuilder::default();
        output = crate::protocol_serde::shape_restore_cluster_from_snapshot::de_restore_cluster_from_snapshot(_response_body, output)
            .map_err(crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::restore_cluster_from_snapshot_output_output_correct_errors(output).build()
    })
}

pub fn ser_restore_cluster_from_snapshot_input(
    input: &crate::operation::restore_cluster_from_snapshot::RestoreClusterFromSnapshotInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_restore_cluster_from_snapshot_input::ser_restore_cluster_from_snapshot_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_restore_cluster_from_snapshot(
    value: &[u8],
    mut builder: crate::operation::restore_cluster_from_snapshot::builders::RestoreClusterFromSnapshotOutputBuilder,
) -> ::std::result::Result<
    crate::operation::restore_cluster_from_snapshot::builders::RestoreClusterFromSnapshotOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "cluster" => {
                    builder = builder.set_cluster(crate::protocol_serde::shape_cluster::de_cluster(tokens)?);
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
