// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_telemetry_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TelemetryConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TelemetryConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "AccountIdentifier" => {
                            builder = builder.set_account_identifier(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TelemetryConfigurationState" => {
                            builder = builder.set_telemetry_configuration_state(
                                crate::protocol_serde::shape_telemetry_configuration_state::de_telemetry_configuration_state(tokens)?,
                            );
                        }
                        "ResourceType" => {
                            builder = builder.set_resource_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ResourceType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ResourceIdentifier" => {
                            builder = builder.set_resource_identifier(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ResourceTags" => {
                            builder = builder.set_resource_tags(crate::protocol_serde::shape_tag_map_output::de_tag_map_output(tokens)?);
                        }
                        "LastUpdateTimeStamp" => {
                            builder = builder.set_last_update_time_stamp(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
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
