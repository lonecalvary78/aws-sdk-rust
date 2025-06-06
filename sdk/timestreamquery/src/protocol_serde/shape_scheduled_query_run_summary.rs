// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_scheduled_query_run_summary<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ScheduledQueryRunSummary>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ScheduledQueryRunSummaryBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "InvocationTime" => {
                            builder = builder.set_invocation_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "TriggerTime" => {
                            builder = builder.set_trigger_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "RunStatus" => {
                            builder = builder.set_run_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ScheduledQueryRunStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ExecutionStats" => {
                            builder = builder.set_execution_stats(crate::protocol_serde::shape_execution_stats::de_execution_stats(tokens)?);
                        }
                        "QueryInsightsResponse" => {
                            builder = builder.set_query_insights_response(
                                crate::protocol_serde::shape_scheduled_query_insights_response::de_scheduled_query_insights_response(tokens)?,
                            );
                        }
                        "ErrorReportLocation" => {
                            builder = builder
                                .set_error_report_location(crate::protocol_serde::shape_error_report_location::de_error_report_location(tokens)?);
                        }
                        "FailureReason" => {
                            builder = builder.set_failure_reason(
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
