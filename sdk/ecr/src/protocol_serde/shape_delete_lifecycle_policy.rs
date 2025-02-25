// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_lifecycle_policy_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyOutput,
    crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidParameterException" => crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "LifecyclePolicyNotFoundException" => {
            crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::LifecyclePolicyNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LifecyclePolicyNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_lifecycle_policy_not_found_exception::de_lifecycle_policy_not_found_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "RepositoryNotFoundException" => crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::RepositoryNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RepositoryNotFoundExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_repository_not_found_exception::de_repository_not_found_exception_json_err(_response_body, output)
                        .map_err(crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServerException" => crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::ServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_server_exception::de_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ValidationException" => crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_lifecycle_policy_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyOutput,
    crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_lifecycle_policy::de_delete_lifecycle_policy(_response_body, output)
            .map_err(crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_delete_lifecycle_policy_input(
    input: &crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_lifecycle_policy_input::ser_delete_lifecycle_policy_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_delete_lifecycle_policy(
    value: &[u8],
    mut builder: crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyOutputBuilder,
) -> ::std::result::Result<
    crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "registryId" => {
                    builder = builder.set_registry_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "repositoryName" => {
                    builder = builder.set_repository_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "lifecyclePolicyText" => {
                    builder = builder.set_lifecycle_policy_text(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "lastEvaluatedAt" => {
                    builder = builder.set_last_evaluated_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
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
