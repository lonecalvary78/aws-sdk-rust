// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_workload_estimate_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_workload_estimate::CreateWorkloadEstimateOutput,
    crate::operation::create_workload_estimate::CreateWorkloadEstimateError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConflictException" => crate::operation::create_workload_estimate::CreateWorkloadEstimateError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::conflict_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?
            };
            tmp
        }),
        "DataUnavailableException" => crate::operation::create_workload_estimate::CreateWorkloadEstimateError::DataUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DataUnavailableExceptionBuilder::default();
                output = crate::protocol_serde::shape_data_unavailable_exception::de_data_unavailable_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::data_unavailable_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?
            };
            tmp
        }),
        "ServiceQuotaExceededException" => crate::operation::create_workload_estimate::CreateWorkloadEstimateError::ServiceQuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceQuotaExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_quota_exceeded_exception::de_service_quota_exceeded_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::service_quota_exceeded_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?
            };
            tmp
        }),
        "AccessDeniedException" => crate::operation::create_workload_estimate::CreateWorkloadEstimateError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => crate::operation::create_workload_estimate::CreateWorkloadEstimateError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::create_workload_estimate::CreateWorkloadEstimateError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::create_workload_estimate::CreateWorkloadEstimateError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::create_workload_estimate::CreateWorkloadEstimateError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_workload_estimate_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_workload_estimate::CreateWorkloadEstimateOutput,
    crate::operation::create_workload_estimate::CreateWorkloadEstimateError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_workload_estimate::builders::CreateWorkloadEstimateOutputBuilder::default();
        output = crate::protocol_serde::shape_create_workload_estimate::de_create_workload_estimate(_response_body, output)
            .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::create_workload_estimate_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::create_workload_estimate::CreateWorkloadEstimateError::unhandled)?
    })
}

pub fn ser_create_workload_estimate_input(
    input: &crate::operation::create_workload_estimate::CreateWorkloadEstimateInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_workload_estimate_input::ser_create_workload_estimate_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_create_workload_estimate(
    value: &[u8],
    mut builder: crate::operation::create_workload_estimate::builders::CreateWorkloadEstimateOutputBuilder,
) -> ::std::result::Result<
    crate::operation::create_workload_estimate::builders::CreateWorkloadEstimateOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "id" => {
                    builder = builder.set_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "name" => {
                    builder = builder.set_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "createdAt" => {
                    builder = builder.set_created_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "expiresAt" => {
                    builder = builder.set_expires_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "rateType" => {
                    builder = builder.set_rate_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::WorkloadEstimateRateType::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "rateTimestamp" => {
                    builder = builder.set_rate_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::WorkloadEstimateStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "totalCost" => {
                    builder = builder
                        .set_total_cost(::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()));
                }
                "costCurrency" => {
                    builder = builder.set_cost_currency(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::CurrencyCode::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "failureMessage" => {
                    builder = builder.set_failure_message(
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
