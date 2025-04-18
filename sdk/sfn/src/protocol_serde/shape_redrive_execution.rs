// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_redrive_execution_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::redrive_execution::RedriveExecutionOutput, crate::operation::redrive_execution::RedriveExecutionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::redrive_execution::RedriveExecutionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::redrive_execution::RedriveExecutionError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ExecutionDoesNotExist" => crate::operation::redrive_execution::RedriveExecutionError::ExecutionDoesNotExist({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ExecutionDoesNotExistBuilder::default();
                output = crate::protocol_serde::shape_execution_does_not_exist::de_execution_does_not_exist_json_err(_response_body, output)
                    .map_err(crate::operation::redrive_execution::RedriveExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ExecutionLimitExceeded" => crate::operation::redrive_execution::RedriveExecutionError::ExecutionLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ExecutionLimitExceededBuilder::default();
                output = crate::protocol_serde::shape_execution_limit_exceeded::de_execution_limit_exceeded_json_err(_response_body, output)
                    .map_err(crate::operation::redrive_execution::RedriveExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ExecutionNotRedrivable" => crate::operation::redrive_execution::RedriveExecutionError::ExecutionNotRedrivable({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ExecutionNotRedrivableBuilder::default();
                output = crate::protocol_serde::shape_execution_not_redrivable::de_execution_not_redrivable_json_err(_response_body, output)
                    .map_err(crate::operation::redrive_execution::RedriveExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArn" => crate::operation::redrive_execution::RedriveExecutionError::InvalidArn({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArnBuilder::default();
                output = crate::protocol_serde::shape_invalid_arn::de_invalid_arn_json_err(_response_body, output)
                    .map_err(crate::operation::redrive_execution::RedriveExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ValidationException" => crate::operation::redrive_execution::RedriveExecutionError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::redrive_execution::RedriveExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::redrive_execution::RedriveExecutionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_redrive_execution_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::redrive_execution::RedriveExecutionOutput, crate::operation::redrive_execution::RedriveExecutionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::redrive_execution::builders::RedriveExecutionOutputBuilder::default();
        output = crate::protocol_serde::shape_redrive_execution::de_redrive_execution(_response_body, output)
            .map_err(crate::operation::redrive_execution::RedriveExecutionError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::redrive_execution_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::redrive_execution::RedriveExecutionError::unhandled)?
    })
}

pub fn ser_redrive_execution_input(
    input: &crate::operation::redrive_execution::RedriveExecutionInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_redrive_execution_input::ser_redrive_execution_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_redrive_execution(
    value: &[u8],
    mut builder: crate::operation::redrive_execution::builders::RedriveExecutionOutputBuilder,
) -> ::std::result::Result<
    crate::operation::redrive_execution::builders::RedriveExecutionOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "redriveDate" => {
                    builder = builder.set_redrive_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
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
