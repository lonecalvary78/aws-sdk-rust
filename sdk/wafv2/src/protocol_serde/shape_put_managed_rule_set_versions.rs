// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_managed_rule_set_versions_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsOutput,
    crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "WAFInternalErrorException" => crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::WafInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::WafInternalErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_waf_internal_error_exception::de_waf_internal_error_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "WAFInvalidOperationException" => {
            crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::WafInvalidOperationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::WafInvalidOperationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_waf_invalid_operation_exception::de_waf_invalid_operation_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "WAFInvalidParameterException" => {
            crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::WafInvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::WafInvalidParameterExceptionBuilder::default();
                    output = crate::protocol_serde::shape_waf_invalid_parameter_exception::de_waf_invalid_parameter_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "WAFNonexistentItemException" => {
            crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::WafNonexistentItemException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::WafNonexistentItemExceptionBuilder::default();
                    output = crate::protocol_serde::shape_waf_nonexistent_item_exception::de_waf_nonexistent_item_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "WAFOptimisticLockException" => {
            crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::WafOptimisticLockException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::WafOptimisticLockExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_waf_optimistic_lock_exception::de_waf_optimistic_lock_exception_json_err(_response_body, output)
                            .map_err(crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_managed_rule_set_versions_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsOutput,
    crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_managed_rule_set_versions::builders::PutManagedRuleSetVersionsOutputBuilder::default();
        output = crate::protocol_serde::shape_put_managed_rule_set_versions::de_put_managed_rule_set_versions(_response_body, output)
            .map_err(crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_put_managed_rule_set_versions_input(
    input: &crate::operation::put_managed_rule_set_versions::PutManagedRuleSetVersionsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_managed_rule_set_versions_input::ser_put_managed_rule_set_versions_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_put_managed_rule_set_versions(
    value: &[u8],
    mut builder: crate::operation::put_managed_rule_set_versions::builders::PutManagedRuleSetVersionsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::put_managed_rule_set_versions::builders::PutManagedRuleSetVersionsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "NextLockToken" => {
                    builder = builder.set_next_lock_token(
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
