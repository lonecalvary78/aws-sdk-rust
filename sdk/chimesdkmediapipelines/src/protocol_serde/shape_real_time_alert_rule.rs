// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_real_time_alert_rule(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RealTimeAlertRule,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Type").string(input.r#type.as_str());
    }
    if let Some(var_1) = &input.keyword_match_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("KeywordMatchConfiguration").start_object();
        crate::protocol_serde::shape_keyword_match_configuration::ser_keyword_match_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.sentiment_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SentimentConfiguration").start_object();
        crate::protocol_serde::shape_sentiment_configuration::ser_sentiment_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.issue_detection_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("IssueDetectionConfiguration").start_object();
        crate::protocol_serde::shape_issue_detection_configuration::ser_issue_detection_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_real_time_alert_rule<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::RealTimeAlertRule>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RealTimeAlertRuleBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Type" => {
                            builder = builder.set_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::RealTimeAlertRuleType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "KeywordMatchConfiguration" => {
                            builder = builder.set_keyword_match_configuration(
                                crate::protocol_serde::shape_keyword_match_configuration::de_keyword_match_configuration(tokens)?,
                            );
                        }
                        "SentimentConfiguration" => {
                            builder = builder.set_sentiment_configuration(
                                crate::protocol_serde::shape_sentiment_configuration::de_sentiment_configuration(tokens)?,
                            );
                        }
                        "IssueDetectionConfiguration" => {
                            builder = builder.set_issue_detection_configuration(
                                crate::protocol_serde::shape_issue_detection_configuration::de_issue_detection_configuration(tokens)?,
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
            Ok(Some(crate::serde_util::real_time_alert_rule_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
