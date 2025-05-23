// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_security_profiles_for_target_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetOutput,
    crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalFailureException" => {
            crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::InternalFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalFailureExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(_response_body, output)
                        .map_err(crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRequestException" => {
            crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::InvalidRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                        .map_err(crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_security_profiles_for_target_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetOutput,
    crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_security_profiles_for_target::builders::ListSecurityProfilesForTargetOutputBuilder::default();
        output = crate::protocol_serde::shape_list_security_profiles_for_target::de_list_security_profiles_for_target(_response_body, output)
            .map_err(crate::operation::list_security_profiles_for_target::ListSecurityProfilesForTargetError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_security_profiles_for_target(
    value: &[u8],
    mut builder: crate::operation::list_security_profiles_for_target::builders::ListSecurityProfilesForTargetOutputBuilder,
) -> ::std::result::Result<
    crate::operation::list_security_profiles_for_target::builders::ListSecurityProfilesForTargetOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "nextToken" => {
                    builder = builder.set_next_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "securityProfileTargetMappings" => {
                    builder = builder.set_security_profile_target_mappings(
                        crate::protocol_serde::shape_security_profile_target_mappings::de_security_profile_target_mappings(tokens)?,
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
