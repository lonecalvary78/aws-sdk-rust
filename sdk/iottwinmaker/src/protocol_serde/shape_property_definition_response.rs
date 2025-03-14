// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_property_definition_response<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::PropertyDefinitionResponse>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::PropertyDefinitionResponseBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "dataType" => {
                            builder = builder.set_data_type(crate::protocol_serde::shape_data_type::de_data_type(tokens)?);
                        }
                        "isTimeSeries" => {
                            builder = builder.set_is_time_series(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "isRequiredInEntity" => {
                            builder = builder.set_is_required_in_entity(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "isExternalId" => {
                            builder = builder.set_is_external_id(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "isStoredExternally" => {
                            builder = builder.set_is_stored_externally(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "isImported" => {
                            builder = builder.set_is_imported(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "isFinal" => {
                            builder = builder.set_is_final(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "isInherited" => {
                            builder = builder.set_is_inherited(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "defaultValue" => {
                            builder = builder.set_default_value(crate::protocol_serde::shape_data_value::de_data_value(tokens)?);
                        }
                        "configuration" => {
                            builder = builder.set_configuration(crate::protocol_serde::shape_configuration::de_configuration(tokens)?);
                        }
                        "displayName" => {
                            builder = builder.set_display_name(
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
            Ok(Some(
                crate::serde_util::property_definition_response_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
