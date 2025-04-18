// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_security_control_definition_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_security_control_definition::GetSecurityControlDefinitionOutput,
    crate::operation::get_security_control_definition::GetSecurityControlDefinitionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalException" => crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::InternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidAccessException" => crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::InvalidAccessException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidAccessExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_access_exception::de_invalid_access_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidInputException" => crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "LimitExceededException" => crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_security_control_definition_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_security_control_definition::GetSecurityControlDefinitionOutput,
    crate::operation::get_security_control_definition::GetSecurityControlDefinitionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_security_control_definition::builders::GetSecurityControlDefinitionOutputBuilder::default();
        output = crate::protocol_serde::shape_get_security_control_definition::de_get_security_control_definition(_response_body, output)
            .map_err(crate::operation::get_security_control_definition::GetSecurityControlDefinitionError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_security_control_definition_output_output_correct_errors(output).build()
    })
}

pub(crate) fn de_get_security_control_definition(
    value: &[u8],
    mut builder: crate::operation::get_security_control_definition::builders::GetSecurityControlDefinitionOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_security_control_definition::builders::GetSecurityControlDefinitionOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "SecurityControlDefinition" => {
                    builder = builder.set_security_control_definition(
                        crate::protocol_serde::shape_security_control_definition::de_security_control_definition(tokens)?,
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
