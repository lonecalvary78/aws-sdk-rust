// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_wafv2_web_acl_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsWafv2WebAclDetails,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.arn {
        object.key("Arn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.managedby_firewall_manager {
        object.key("ManagedbyFirewallManager").boolean(*var_3);
    }
    if let Some(var_4) = &input.id {
        object.key("Id").string(var_4.as_str());
    }
    if let Some(var_5) = &input.capacity {
        object.key("Capacity").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.captcha_config {
        #[allow(unused_mut)]
        let mut object_7 = object.key("CaptchaConfig").start_object();
        crate::protocol_serde::shape_aws_wafv2_web_acl_captcha_config_details::ser_aws_wafv2_web_acl_captcha_config_details(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.default_action {
        #[allow(unused_mut)]
        let mut object_9 = object.key("DefaultAction").start_object();
        crate::protocol_serde::shape_aws_wafv2_web_acl_action_details::ser_aws_wafv2_web_acl_action_details(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.description {
        object.key("Description").string(var_10.as_str());
    }
    if let Some(var_11) = &input.rules {
        let mut array_12 = object.key("Rules").start_array();
        for item_13 in var_11 {
            {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_aws_wafv2_rules_details::ser_aws_wafv2_rules_details(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.visibility_config {
        #[allow(unused_mut)]
        let mut object_16 = object.key("VisibilityConfig").start_object();
        crate::protocol_serde::shape_aws_wafv2_visibility_config_details::ser_aws_wafv2_visibility_config_details(&mut object_16, var_15)?;
        object_16.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_wafv2_web_acl_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AwsWafv2WebAclDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsWafv2WebAclDetailsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Name" => {
                                builder = builder.set_name(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "Arn" => {
                                builder = builder.set_arn(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "ManagedbyFirewallManager" => {
                                builder = builder
                                    .set_managedby_firewall_manager(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                            }
                            "Id" => {
                                builder = builder.set_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "Capacity" => {
                                builder = builder.set_capacity(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                        .map(i64::try_from)
                                        .transpose()?,
                                );
                            }
                            "CaptchaConfig" => {
                                builder = builder.set_captcha_config(
                                    crate::protocol_serde::shape_aws_wafv2_web_acl_captcha_config_details::de_aws_wafv2_web_acl_captcha_config_details(tokens)?
                                );
                            }
                            "DefaultAction" => {
                                builder = builder.set_default_action(
                                    crate::protocol_serde::shape_aws_wafv2_web_acl_action_details::de_aws_wafv2_web_acl_action_details(tokens)?,
                                );
                            }
                            "Description" => {
                                builder = builder.set_description(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "Rules" => {
                                builder = builder.set_rules(crate::protocol_serde::shape_aws_wafv2_rules_list::de_aws_wafv2_rules_list(tokens)?);
                            }
                            "VisibilityConfig" => {
                                builder = builder.set_visibility_config(
                                    crate::protocol_serde::shape_aws_wafv2_visibility_config_details::de_aws_wafv2_visibility_config_details(tokens)?,
                                );
                            }
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
