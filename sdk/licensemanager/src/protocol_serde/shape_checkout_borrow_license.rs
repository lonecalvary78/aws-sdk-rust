// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_checkout_borrow_license_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::checkout_borrow_license::CheckoutBorrowLicenseOutput,
    crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "AuthorizationException" => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::AuthorizationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AuthorizationExceptionBuilder::default();
                output = crate::protocol_serde::shape_authorization_exception::de_authorization_exception_json_err(_response_body, output)
                    .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EntitlementNotAllowedException" => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::EntitlementNotAllowedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EntitlementNotAllowedExceptionBuilder::default();
                output = crate::protocol_serde::shape_entitlement_not_allowed_exception::de_entitlement_not_allowed_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoEntitlementsAllowedException" => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::NoEntitlementsAllowedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoEntitlementsAllowedExceptionBuilder::default();
                output = crate::protocol_serde::shape_no_entitlements_allowed_exception::de_no_entitlements_allowed_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RateLimitExceededException" => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::RateLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RateLimitExceededExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_rate_limit_exceeded_exception::de_rate_limit_exceeded_exception_json_err(_response_body, output)
                        .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RedirectException" => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::RedirectException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RedirectExceptionBuilder::default();
                output = crate::protocol_serde::shape_redirect_exception::de_redirect_exception_json_err(_response_body, output)
                    .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServerInternalException" => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::ServerInternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServerInternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_server_internal_exception::de_server_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedDigitalSignatureMethodException" => {
            crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::UnsupportedDigitalSignatureMethodException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedDigitalSignatureMethodExceptionBuilder::default();
                    output = crate::protocol_serde::shape_unsupported_digital_signature_method_exception::de_unsupported_digital_signature_method_exception_json_err(_response_body, output).map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ValidationException" => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_checkout_borrow_license_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::checkout_borrow_license::CheckoutBorrowLicenseOutput,
    crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::checkout_borrow_license::builders::CheckoutBorrowLicenseOutputBuilder::default();
        output = crate::protocol_serde::shape_checkout_borrow_license::de_checkout_borrow_license(_response_body, output)
            .map_err(crate::operation::checkout_borrow_license::CheckoutBorrowLicenseError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_checkout_borrow_license_input(
    input: &crate::operation::checkout_borrow_license::CheckoutBorrowLicenseInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_checkout_borrow_license_input::ser_checkout_borrow_license_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_checkout_borrow_license(
    value: &[u8],
    mut builder: crate::operation::checkout_borrow_license::builders::CheckoutBorrowLicenseOutputBuilder,
) -> ::std::result::Result<
    crate::operation::checkout_borrow_license::builders::CheckoutBorrowLicenseOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "LicenseArn" => {
                    builder = builder.set_license_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "LicenseConsumptionToken" => {
                    builder = builder.set_license_consumption_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "EntitlementsAllowed" => {
                    builder = builder.set_entitlements_allowed(crate::protocol_serde::shape_entitlement_data_list::de_entitlement_data_list(tokens)?);
                }
                "NodeId" => {
                    builder = builder.set_node_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "SignedToken" => {
                    builder = builder.set_signed_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "IssuedAt" => {
                    builder = builder.set_issued_at(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Expiration" => {
                    builder = builder.set_expiration(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "CheckoutMetadata" => {
                    builder = builder.set_checkout_metadata(crate::protocol_serde::shape_metadata_list::de_metadata_list(tokens)?);
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
