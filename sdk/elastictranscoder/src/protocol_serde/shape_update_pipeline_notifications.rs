// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_pipeline_notifications_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsOutput,
    crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?
            };
            tmp
        }),
        "IncompatibleVersionException" => {
            crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::IncompatibleVersionException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::IncompatibleVersionExceptionBuilder::default();
                    output = crate::protocol_serde::shape_incompatible_version_exception::de_incompatible_version_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::incompatible_version_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?
                };
                tmp
            })
        }
        "InternalServiceException" => crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_service_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?
            };
            tmp
        }),
        "ResourceInUseException" => crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::ResourceInUseException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceInUseExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_in_use_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?
                };
                tmp
            })
        }
        "ValidationException" => crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_pipeline_notifications_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsOutput,
    crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_pipeline_notifications::builders::UpdatePipelineNotificationsOutputBuilder::default();
        output = crate::protocol_serde::shape_update_pipeline_notifications::de_update_pipeline_notifications(_response_body, output)
            .map_err(crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_update_pipeline_notifications_input(
    input: &crate::operation::update_pipeline_notifications::UpdatePipelineNotificationsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_pipeline_notifications_input::ser_update_pipeline_notifications_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_update_pipeline_notifications(
    value: &[u8],
    mut builder: crate::operation::update_pipeline_notifications::builders::UpdatePipelineNotificationsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::update_pipeline_notifications::builders::UpdatePipelineNotificationsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "Pipeline" => {
                    builder = builder.set_pipeline(crate::protocol_serde::shape_pipeline::de_pipeline(tokens)?);
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
