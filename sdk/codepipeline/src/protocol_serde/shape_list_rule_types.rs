// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_rule_types_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::list_rule_types::ListRuleTypesOutput, crate::operation::list_rule_types::ListRuleTypesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_rule_types::ListRuleTypesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_rule_types::ListRuleTypesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidNextTokenException" => crate::operation::list_rule_types::ListRuleTypesError::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidNextTokenExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_rule_types::ListRuleTypesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ValidationException" => crate::operation::list_rule_types::ListRuleTypesError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_rule_types::ListRuleTypesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::list_rule_types::ListRuleTypesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_rule_types_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::list_rule_types::ListRuleTypesOutput, crate::operation::list_rule_types::ListRuleTypesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_rule_types::builders::ListRuleTypesOutputBuilder::default();
        output = crate::protocol_serde::shape_list_rule_types::de_list_rule_types(_response_body, output)
            .map_err(crate::operation::list_rule_types::ListRuleTypesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::list_rule_types_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::list_rule_types::ListRuleTypesError::unhandled)?
    })
}

pub fn ser_list_rule_types_input(
    input: &crate::operation::list_rule_types::ListRuleTypesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_rule_types_input::ser_list_rule_types_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_list_rule_types(
    value: &[u8],
    mut builder: crate::operation::list_rule_types::builders::ListRuleTypesOutputBuilder,
) -> ::std::result::Result<
    crate::operation::list_rule_types::builders::ListRuleTypesOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "ruleTypes" => {
                    builder = builder.set_rule_types(crate::protocol_serde::shape_rule_type_list::de_rule_type_list(tokens)?);
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
