// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_membership_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_membership::GetMembershipOutput, crate::operation::get_membership::GetMembershipError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_membership::GetMembershipError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_membership::GetMembershipError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?
            };
            tmp
        }),
        "ConflictException" => crate::operation::get_membership::GetMembershipError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::conflict_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => crate::operation::get_membership::GetMembershipError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_internal_server_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::get_membership::GetMembershipError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?
            };
            tmp
        }),
        "InvalidTokenException" => crate::operation::get_membership::GetMembershipError::InvalidTokenException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidTokenExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_token_exception::de_invalid_token_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::invalid_token_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_membership::GetMembershipError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?
            };
            tmp
        }),
        "SecurityIncidentResponseNotActiveException" => {
            crate::operation::get_membership::GetMembershipError::SecurityIncidentResponseNotActiveException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::SecurityIncidentResponseNotActiveExceptionBuilder::default();
                    output = crate::protocol_serde::shape_security_incident_response_not_active_exception::de_security_incident_response_not_active_exception_json_err(_response_body, output).map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::security_incident_response_not_active_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?
                };
                tmp
            })
        }
        "ServiceQuotaExceededException" => crate::operation::get_membership::GetMembershipError::ServiceQuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceQuotaExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_quota_exceeded_exception::de_service_quota_exceeded_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::service_quota_exceeded_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::get_membership::GetMembershipError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::get_membership::GetMembershipError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After")
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::get_membership::GetMembershipError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::get_membership::GetMembershipError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_membership_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_membership::GetMembershipOutput, crate::operation::get_membership::GetMembershipError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_membership::builders::GetMembershipOutputBuilder::default();
        output = crate::protocol_serde::shape_get_membership::de_get_membership(_response_body, output)
            .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_membership_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::get_membership::GetMembershipError::unhandled)?
    })
}

pub(crate) fn de_get_membership(
    value: &[u8],
    mut builder: crate::operation::get_membership::builders::GetMembershipOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_membership::builders::GetMembershipOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "accountId" => {
                    builder = builder.set_account_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "customerType" => {
                    builder = builder.set_customer_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::CustomerType::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "incidentResponseTeam" => {
                    builder =
                        builder.set_incident_response_team(crate::protocol_serde::shape_incident_response_team::de_incident_response_team(tokens)?);
                }
                "membershipActivationTimestamp" => {
                    builder = builder.set_membership_activation_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "membershipArn" => {
                    builder = builder.set_membership_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "membershipDeactivationTimestamp" => {
                    builder = builder.set_membership_deactivation_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "membershipId" => {
                    builder = builder.set_membership_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "membershipName" => {
                    builder = builder.set_membership_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "membershipStatus" => {
                    builder = builder.set_membership_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::MembershipStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "numberOfAccountsCovered" => {
                    builder = builder.set_number_of_accounts_covered(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i64::try_from)
                            .transpose()?,
                    );
                }
                "optInFeatures" => {
                    builder = builder.set_opt_in_features(crate::protocol_serde::shape_opt_in_features::de_opt_in_features(tokens)?);
                }
                "region" => {
                    builder = builder.set_region(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::AwsRegion::from(u.as_ref())))
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
