// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_branch_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_branch::GetBranchOutput, crate::operation::get_branch::GetBranchError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_branch::GetBranchError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BranchDoesNotExistException" => crate::operation::get_branch::GetBranchError::BranchDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BranchDoesNotExistExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_branch_does_not_exist_exception::de_branch_does_not_exist_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "BranchNameRequiredException" => crate::operation::get_branch::GetBranchError::BranchNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BranchNameRequiredExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_branch_name_required_exception::de_branch_name_required_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionIntegrityChecksFailedException" => crate::operation::get_branch::GetBranchError::EncryptionIntegrityChecksFailedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionIntegrityChecksFailedExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_integrity_checks_failed_exception::de_encryption_integrity_checks_failed_exception_json_err(_response_body, output).map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyAccessDeniedException" => crate::operation::get_branch::GetBranchError::EncryptionKeyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyAccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_access_denied_exception::de_encryption_key_access_denied_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyDisabledException" => crate::operation::get_branch::GetBranchError::EncryptionKeyDisabledException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyDisabledExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_disabled_exception::de_encryption_key_disabled_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyNotFoundException" => crate::operation::get_branch::GetBranchError::EncryptionKeyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_not_found_exception::de_encryption_key_not_found_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyUnavailableException" => crate::operation::get_branch::GetBranchError::EncryptionKeyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyUnavailableExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_unavailable_exception::de_encryption_key_unavailable_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidBranchNameException" => crate::operation::get_branch::GetBranchError::InvalidBranchNameException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidBranchNameExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_branch_name_exception::de_invalid_branch_name_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRepositoryNameException" => crate::operation::get_branch::GetBranchError::InvalidRepositoryNameException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRepositoryNameExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_repository_name_exception::de_invalid_repository_name_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RepositoryDoesNotExistException" => crate::operation::get_branch::GetBranchError::RepositoryDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RepositoryDoesNotExistExceptionBuilder::default();
                output = crate::protocol_serde::shape_repository_does_not_exist_exception::de_repository_does_not_exist_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RepositoryNameRequiredException" => crate::operation::get_branch::GetBranchError::RepositoryNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RepositoryNameRequiredExceptionBuilder::default();
                output = crate::protocol_serde::shape_repository_name_required_exception::de_repository_name_required_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_branch::GetBranchError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_branch_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_branch::GetBranchOutput, crate::operation::get_branch::GetBranchError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_branch::builders::GetBranchOutputBuilder::default();
        output = crate::protocol_serde::shape_get_branch::de_get_branch(_response_body, output)
            .map_err(crate::operation::get_branch::GetBranchError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_branch_input(
    input: &crate::operation::get_branch::GetBranchInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_branch_input::ser_get_branch_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_branch(
    value: &[u8],
    mut builder: crate::operation::get_branch::builders::GetBranchOutputBuilder,
) -> ::std::result::Result<crate::operation::get_branch::builders::GetBranchOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "branch" => {
                    builder = builder.set_branch(crate::protocol_serde::shape_branch_info::de_branch_info(tokens)?);
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
