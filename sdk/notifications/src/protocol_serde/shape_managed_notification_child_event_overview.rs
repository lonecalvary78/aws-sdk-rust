// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_managed_notification_child_event_overview<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ManagedNotificationChildEventOverview>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ManagedNotificationChildEventOverviewBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "arn" => {
                                builder = builder.set_arn(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "managedNotificationConfigurationArn" => {
                                builder = builder.set_managed_notification_configuration_arn(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "relatedAccount" => {
                                builder = builder.set_related_account(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "creationTime" => {
                                builder = builder.set_creation_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                    tokens.next(),
                                    ::aws_smithy_types::date_time::Format::DateTimeWithOffset,
                                )?);
                            }
                            "childEvent" => {
                                builder = builder.set_child_event(
                                    crate::protocol_serde::shape_managed_notification_child_event_summary::de_managed_notification_child_event_summary(tokens)?
                                );
                            }
                            "aggregateManagedNotificationEventArn" => {
                                builder = builder.set_aggregate_managed_notification_event_arn(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "organizationalUnitId" => {
                                builder = builder.set_organizational_unit_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
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
            Ok(Some(
                crate::serde_util::managed_notification_child_event_overview_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
