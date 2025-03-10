// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_analysis_rule_list(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AnalysisRuleList,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("joinColumns").start_array();
        for item_2 in &input.join_columns {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    if let Some(var_3) = &input.allowed_join_operators {
        let mut array_4 = object.key("allowedJoinOperators").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    {
        let mut array_6 = object.key("listColumns").start_array();
        for item_7 in &input.list_columns {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.additional_analyses {
        object.key("additionalAnalyses").string(var_8.as_str());
    }
    Ok(())
}

pub(crate) fn de_analysis_rule_list<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AnalysisRuleList>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AnalysisRuleListBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "joinColumns" => {
                            builder = builder.set_join_columns(crate::protocol_serde::shape_analysis_rule_column_list::de_analysis_rule_column_list(
                                tokens,
                            )?);
                        }
                        "allowedJoinOperators" => {
                            builder =
                                builder.set_allowed_join_operators(crate::protocol_serde::shape_join_operators_list::de_join_operators_list(tokens)?);
                        }
                        "listColumns" => {
                            builder = builder.set_list_columns(crate::protocol_serde::shape_analysis_rule_column_list::de_analysis_rule_column_list(
                                tokens,
                            )?);
                        }
                        "additionalAnalyses" => {
                            builder = builder.set_additional_analyses(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AdditionalAnalyses::from(u.as_ref())))
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
            Ok(Some(crate::serde_util::analysis_rule_list_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
