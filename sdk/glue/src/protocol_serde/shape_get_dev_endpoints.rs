// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_dev_endpoints_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_dev_endpoints::GetDevEndpointsOutput, crate::operation::get_dev_endpoints::GetDevEndpointsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_dev_endpoints::GetDevEndpointsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_dev_endpoints::GetDevEndpointsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "EntityNotFoundException" => crate::operation::get_dev_endpoints::GetDevEndpointsError::EntityNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EntityNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_entity_not_found_exception::de_entity_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_dev_endpoints::GetDevEndpointsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServiceException" => crate::operation::get_dev_endpoints::GetDevEndpointsError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_dev_endpoints::GetDevEndpointsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidInputException" => crate::operation::get_dev_endpoints::GetDevEndpointsError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_dev_endpoints::GetDevEndpointsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "OperationTimeoutException" => crate::operation::get_dev_endpoints::GetDevEndpointsError::OperationTimeoutException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::OperationTimeoutExceptionBuilder::default();
                output = crate::protocol_serde::shape_operation_timeout_exception::de_operation_timeout_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_dev_endpoints::GetDevEndpointsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_dev_endpoints::GetDevEndpointsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_dev_endpoints_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_dev_endpoints::GetDevEndpointsOutput, crate::operation::get_dev_endpoints::GetDevEndpointsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_dev_endpoints::builders::GetDevEndpointsOutputBuilder::default();
        output = crate::protocol_serde::shape_get_dev_endpoints::de_get_dev_endpoints(_response_body, output)
            .map_err(crate::operation::get_dev_endpoints::GetDevEndpointsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_dev_endpoints_input(
    input: &crate::operation::get_dev_endpoints::GetDevEndpointsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_dev_endpoints_input::ser_get_dev_endpoints_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_dev_endpoints(
    value: &[u8],
    mut builder: crate::operation::get_dev_endpoints::builders::GetDevEndpointsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_dev_endpoints::builders::GetDevEndpointsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "DevEndpoints" => {
                    builder = builder.set_dev_endpoints(crate::protocol_serde::shape_dev_endpoint_list::de_dev_endpoint_list(tokens)?);
                }
                "NextToken" => {
                    builder = builder.set_next_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
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
