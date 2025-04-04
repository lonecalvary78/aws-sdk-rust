// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_assessment_run<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AssessmentRun>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AssessmentRunBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "arn" => {
                            builder = builder.set_arn(
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
                        "assessmentTemplateArn" => {
                            builder = builder.set_assessment_template_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "state" => {
                            builder = builder.set_state(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AssessmentRunState::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "durationInSeconds" => {
                            builder = builder.set_duration_in_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "rulesPackageArns" => {
                            builder = builder.set_rules_package_arns(
                                crate::protocol_serde::shape_assessment_rules_package_arn_list::de_assessment_rules_package_arn_list(tokens)?,
                            );
                        }
                        "userAttributesForFindings" => {
                            builder = builder
                                .set_user_attributes_for_findings(crate::protocol_serde::shape_user_attribute_list::de_user_attribute_list(tokens)?);
                        }
                        "createdAt" => {
                            builder = builder.set_created_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "startedAt" => {
                            builder = builder.set_started_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "completedAt" => {
                            builder = builder.set_completed_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "stateChangedAt" => {
                            builder = builder.set_state_changed_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "dataCollected" => {
                            builder = builder.set_data_collected(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "stateChanges" => {
                            builder = builder.set_state_changes(
                                crate::protocol_serde::shape_assessment_run_state_change_list::de_assessment_run_state_change_list(tokens)?,
                            );
                        }
                        "notifications" => {
                            builder = builder.set_notifications(
                                crate::protocol_serde::shape_assessment_run_notification_list::de_assessment_run_notification_list(tokens)?,
                            );
                        }
                        "findingCounts" => {
                            builder = builder.set_finding_counts(
                                crate::protocol_serde::shape_assessment_run_finding_counts::de_assessment_run_finding_counts(tokens)?,
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
            Ok(Some(crate::serde_util::assessment_run_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
