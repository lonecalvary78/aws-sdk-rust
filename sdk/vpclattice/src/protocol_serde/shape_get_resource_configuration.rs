// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_resource_configuration_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_resource_configuration::GetResourceConfigurationOutput,
    crate::operation::get_resource_configuration::GetResourceConfigurationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_resource_configuration::GetResourceConfigurationError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => crate::operation::get_resource_configuration::GetResourceConfigurationError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_internal_server_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled(
                            "Failed to parse retryAfterSeconds from header `Retry-After",
                        )
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_resource_configuration::GetResourceConfigurationError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::get_resource_configuration::GetResourceConfigurationError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled(
                            "Failed to parse retryAfterSeconds from header `Retry-After",
                        )
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::get_resource_configuration::GetResourceConfigurationError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::get_resource_configuration::GetResourceConfigurationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_resource_configuration_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_resource_configuration::GetResourceConfigurationOutput,
    crate::operation::get_resource_configuration::GetResourceConfigurationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_resource_configuration::builders::GetResourceConfigurationOutputBuilder::default();
        output = crate::protocol_serde::shape_get_resource_configuration::de_get_resource_configuration(_response_body, output)
            .map_err(crate::operation::get_resource_configuration::GetResourceConfigurationError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_resource_configuration(
    value: &[u8],
    mut builder: crate::operation::get_resource_configuration::builders::GetResourceConfigurationOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_resource_configuration::builders::GetResourceConfigurationOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "allowAssociationToShareableServiceNetwork" => {
                    builder = builder.set_allow_association_to_shareable_service_network(::aws_smithy_json::deserialize::token::expect_bool_or_null(
                        tokens.next(),
                    )?);
                }
                "amazonManaged" => {
                    builder = builder.set_amazon_managed(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                }
                "arn" => {
                    builder = builder.set_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "createdAt" => {
                    builder = builder.set_created_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::DateTimeWithOffset,
                    )?);
                }
                "customDomainName" => {
                    builder = builder.set_custom_domain_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "failureReason" => {
                    builder = builder.set_failure_reason(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "id" => {
                    builder = builder.set_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "lastUpdatedAt" => {
                    builder = builder.set_last_updated_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::DateTimeWithOffset,
                    )?);
                }
                "name" => {
                    builder = builder.set_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "portRanges" => {
                    builder = builder.set_port_ranges(crate::protocol_serde::shape_port_range_list::de_port_range_list(tokens)?);
                }
                "protocol" => {
                    builder = builder.set_protocol(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::ProtocolType::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "resourceConfigurationDefinition" => {
                    builder = builder.set_resource_configuration_definition(
                        crate::protocol_serde::shape_resource_configuration_definition::de_resource_configuration_definition(tokens)?,
                    );
                }
                "resourceConfigurationGroupId" => {
                    builder = builder.set_resource_configuration_group_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "resourceGatewayId" => {
                    builder = builder.set_resource_gateway_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::ResourceConfigurationStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "type" => {
                    builder = builder.set_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::ResourceConfigurationType::from(u.as_ref())))
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
