// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_configured_table_association_analysis_rule_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleOutput,
    crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output).map_err(crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "InternalServerException" => crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output).map_err(crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output).map_err(crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output).build().map_err(crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::unhandled)?
                }
            ;
            tmp
        }),
        "ThrottlingException" => crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output).map_err(crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "ValidationException" => crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output).map_err(crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        _ => crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_configured_table_association_analysis_rule_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleOutput,
    crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_configured_table_association_analysis_rule::builders::GetConfiguredTableAssociationAnalysisRuleOutputBuilder::default();
        output = crate::protocol_serde::shape_get_configured_table_association_analysis_rule::de_get_configured_table_association_analysis_rule(
            _response_body,
            output,
        )
        .map_err(crate::operation::get_configured_table_association_analysis_rule::GetConfiguredTableAssociationAnalysisRuleError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_configured_table_association_analysis_rule_output_output_correct_errors(output).build()
    })
}

pub(crate) fn de_get_configured_table_association_analysis_rule(
    value: &[u8],
    mut builder: crate::operation::get_configured_table_association_analysis_rule::builders::GetConfiguredTableAssociationAnalysisRuleOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_configured_table_association_analysis_rule::builders::GetConfiguredTableAssociationAnalysisRuleOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "analysisRule" => {
                    builder = builder.set_analysis_rule(
                        crate::protocol_serde::shape_configured_table_association_analysis_rule::de_configured_table_association_analysis_rule(
                            tokens,
                        )?,
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
