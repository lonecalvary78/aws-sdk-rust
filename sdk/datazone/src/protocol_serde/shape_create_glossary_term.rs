// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_glossary_term_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_glossary_term::CreateGlossaryTermOutput,
    crate::operation::create_glossary_term::CreateGlossaryTermError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::create_glossary_term::CreateGlossaryTermError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?
            };
            tmp
        }),
        "ConflictException" => crate::operation::create_glossary_term::CreateGlossaryTermError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::conflict_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => crate::operation::create_glossary_term::CreateGlossaryTermError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::create_glossary_term::CreateGlossaryTermError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?
            };
            tmp
        }),
        "ServiceQuotaExceededException" => crate::operation::create_glossary_term::CreateGlossaryTermError::ServiceQuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceQuotaExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_quota_exceeded_exception::de_service_quota_exceeded_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::service_quota_exceeded_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::create_glossary_term::CreateGlossaryTermError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::create_glossary_term::CreateGlossaryTermError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?
            };
            tmp
        }),
        "UnauthorizedException" => crate::operation::create_glossary_term::CreateGlossaryTermError::UnauthorizedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::unauthorized_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::create_glossary_term::CreateGlossaryTermError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_glossary_term_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_glossary_term::CreateGlossaryTermOutput,
    crate::operation::create_glossary_term::CreateGlossaryTermError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_glossary_term::builders::CreateGlossaryTermOutputBuilder::default();
        output = crate::protocol_serde::shape_create_glossary_term::de_create_glossary_term(_response_body, output)
            .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::create_glossary_term_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::create_glossary_term::CreateGlossaryTermError::unhandled)?
    })
}

pub fn ser_create_glossary_term_input(
    input: &crate::operation::create_glossary_term::CreateGlossaryTermInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_glossary_term_input::ser_create_glossary_term_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_create_glossary_term(
    value: &[u8],
    mut builder: crate::operation::create_glossary_term::builders::CreateGlossaryTermOutputBuilder,
) -> ::std::result::Result<
    crate::operation::create_glossary_term::builders::CreateGlossaryTermOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "domainId" => {
                    builder = builder.set_domain_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "glossaryId" => {
                    builder = builder.set_glossary_id(
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
                "longDescription" => {
                    builder = builder.set_long_description(
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
                "shortDescription" => {
                    builder = builder.set_short_description(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::GlossaryTermStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "termRelations" => {
                    builder = builder.set_term_relations(crate::protocol_serde::shape_term_relations::de_term_relations(tokens)?);
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
