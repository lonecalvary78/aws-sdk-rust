// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_signing_profile_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_signing_profile::GetSigningProfileOutput, crate::operation::get_signing_profile::GetSigningProfileError>
{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_signing_profile::GetSigningProfileError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_signing_profile::GetSigningProfileError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_signing_profile::GetSigningProfileError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_signing_profile::GetSigningProfileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServiceErrorException" => crate::operation::get_signing_profile::GetSigningProfileError::InternalServiceErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_service_error_exception::de_internal_service_error_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_signing_profile::GetSigningProfileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_signing_profile::GetSigningProfileError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_signing_profile::GetSigningProfileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::get_signing_profile::GetSigningProfileError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_signing_profile::GetSigningProfileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_signing_profile::GetSigningProfileError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_signing_profile_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_signing_profile::GetSigningProfileOutput, crate::operation::get_signing_profile::GetSigningProfileError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_signing_profile::builders::GetSigningProfileOutputBuilder::default();
        output = crate::protocol_serde::shape_get_signing_profile::de_get_signing_profile(_response_body, output)
            .map_err(crate::operation::get_signing_profile::GetSigningProfileError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_signing_profile(
    value: &[u8],
    mut builder: crate::operation::get_signing_profile::builders::GetSigningProfileOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_signing_profile::builders::GetSigningProfileOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "arn" => {
                    builder = builder.set_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "overrides" => {
                    builder = builder.set_overrides(crate::protocol_serde::shape_signing_platform_overrides::de_signing_platform_overrides(
                        tokens,
                    )?);
                }
                "platformDisplayName" => {
                    builder = builder.set_platform_display_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "platformId" => {
                    builder = builder.set_platform_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "profileName" => {
                    builder = builder.set_profile_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "profileVersion" => {
                    builder = builder.set_profile_version(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "profileVersionArn" => {
                    builder = builder.set_profile_version_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "revocationRecord" => {
                    builder = builder.set_revocation_record(
                        crate::protocol_serde::shape_signing_profile_revocation_record::de_signing_profile_revocation_record(tokens)?,
                    );
                }
                "signatureValidityPeriod" => {
                    builder = builder.set_signature_validity_period(
                        crate::protocol_serde::shape_signature_validity_period::de_signature_validity_period(tokens)?,
                    );
                }
                "signingMaterial" => {
                    builder = builder.set_signing_material(crate::protocol_serde::shape_signing_material::de_signing_material(tokens)?);
                }
                "signingParameters" => {
                    builder = builder.set_signing_parameters(crate::protocol_serde::shape_signing_parameters::de_signing_parameters(tokens)?);
                }
                "status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::SigningProfileStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "statusReason" => {
                    builder = builder.set_status_reason(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "tags" => {
                    builder = builder.set_tags(crate::protocol_serde::shape_tag_map::de_tag_map(tokens)?);
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
