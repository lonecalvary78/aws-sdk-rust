// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_rule_based_properties<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::RuleBasedProperties>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RuleBasedPropertiesBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "rules" => {
                            builder = builder.set_rules(crate::protocol_serde::shape_rule_list::de_rule_list(tokens)?);
                        }
                        "attributeMatchingModel" => {
                            builder = builder.set_attribute_matching_model(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AttributeMatchingModel::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "matchPurpose" => {
                            builder = builder.set_match_purpose(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::MatchPurpose::from(u.as_ref())))
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
            Ok(Some(crate::serde_util::rule_based_properties_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}

pub fn ser_rule_based_properties(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RuleBasedProperties,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("rules").start_array();
        for item_2 in &input.rules {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_rule::ser_rule(&mut object_3, item_2)?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    {
        object.key("attributeMatchingModel").string(input.attribute_matching_model.as_str());
    }
    if let Some(var_4) = &input.match_purpose {
        object.key("matchPurpose").string(var_4.as_str());
    }
    Ok(())
}
