// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_scaling_rule(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ScalingRule,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.action {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Action").start_object();
        crate::protocol_serde::shape_scaling_action::ser_scaling_action(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.trigger {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Trigger").start_object();
        crate::protocol_serde::shape_scaling_trigger::ser_scaling_trigger(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_scaling_rule<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ScalingRule>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ScalingRuleBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Description" => {
                            builder = builder.set_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Action" => {
                            builder = builder.set_action(crate::protocol_serde::shape_scaling_action::de_scaling_action(tokens)?);
                        }
                        "Trigger" => {
                            builder = builder.set_trigger(crate::protocol_serde::shape_scaling_trigger::de_scaling_trigger(tokens)?);
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
            Ok(Some(crate::serde_util::scaling_rule_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
