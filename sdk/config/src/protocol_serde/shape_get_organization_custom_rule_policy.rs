// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_organization_custom_rule_policy_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyOutput,
    crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "NoSuchOrganizationConfigRuleException" => {
            crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError::NoSuchOrganizationConfigRuleException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchOrganizationConfigRuleExceptionBuilder::default();
                    output = crate::protocol_serde::shape_no_such_organization_config_rule_exception::de_no_such_organization_config_rule_exception_json_err(_response_body, output).map_err(crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "OrganizationAccessDeniedException" => {
            crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError::OrganizationAccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::OrganizationAccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_organization_custom_rule_policy_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyOutput,
    crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_organization_custom_rule_policy::builders::GetOrganizationCustomRulePolicyOutputBuilder::default();
        output = crate::protocol_serde::shape_get_organization_custom_rule_policy::de_get_organization_custom_rule_policy(_response_body, output)
            .map_err(crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_organization_custom_rule_policy_input(
    input: &crate::operation::get_organization_custom_rule_policy::GetOrganizationCustomRulePolicyInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_organization_custom_rule_policy_input::ser_get_organization_custom_rule_policy_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_organization_custom_rule_policy(
    value: &[u8],
    mut builder: crate::operation::get_organization_custom_rule_policy::builders::GetOrganizationCustomRulePolicyOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_organization_custom_rule_policy::builders::GetOrganizationCustomRulePolicyOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "PolicyText" => {
                    builder = builder.set_policy_text(
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
