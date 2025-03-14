// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_notification_channel_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::remove_notification_channel::RemoveNotificationChannelOutput,
    crate::operation::remove_notification_channel::RemoveNotificationChannelError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::operation::remove_notification_channel::RemoveNotificationChannelError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::remove_notification_channel::RemoveNotificationChannelError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::remove_notification_channel::RemoveNotificationChannelError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::remove_notification_channel::RemoveNotificationChannelError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::remove_notification_channel::RemoveNotificationChannelError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_notification_channel_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::remove_notification_channel::RemoveNotificationChannelOutput,
    crate::operation::remove_notification_channel::RemoveNotificationChannelError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::remove_notification_channel::builders::RemoveNotificationChannelOutputBuilder::default();
        output = crate::protocol_serde::shape_remove_notification_channel::de_remove_notification_channel(_response_body, output)
            .map_err(crate::operation::remove_notification_channel::RemoveNotificationChannelError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_remove_notification_channel(
    value: &[u8],
    mut builder: crate::operation::remove_notification_channel::builders::RemoveNotificationChannelOutputBuilder,
) -> ::std::result::Result<
    crate::operation::remove_notification_channel::builders::RemoveNotificationChannelOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "notificationConfiguration" => {
                    builder = builder.set_notification_configuration(
                        crate::protocol_serde::shape_notification_configuration::de_notification_configuration(tokens)?,
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
