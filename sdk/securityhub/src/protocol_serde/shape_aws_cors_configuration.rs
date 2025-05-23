// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_cors_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsCorsConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.allow_origins {
        let mut array_2 = object.key("AllowOrigins").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.allow_credentials {
        object.key("AllowCredentials").boolean(*var_4);
    }
    if let Some(var_5) = &input.expose_headers {
        let mut array_6 = object.key("ExposeHeaders").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.max_age {
        object.key("MaxAge").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.allow_methods {
        let mut array_10 = object.key("AllowMethods").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.allow_headers {
        let mut array_13 = object.key("AllowHeaders").start_array();
        for item_14 in var_12 {
            {
                array_13.value().string(item_14.as_str());
            }
        }
        array_13.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_cors_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AwsCorsConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsCorsConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "AllowOrigins" => {
                            builder =
                                builder.set_allow_origins(crate::protocol_serde::shape_non_empty_string_list::de_non_empty_string_list(tokens)?);
                        }
                        "AllowCredentials" => {
                            builder = builder.set_allow_credentials(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "ExposeHeaders" => {
                            builder =
                                builder.set_expose_headers(crate::protocol_serde::shape_non_empty_string_list::de_non_empty_string_list(tokens)?);
                        }
                        "MaxAge" => {
                            builder = builder.set_max_age(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "AllowMethods" => {
                            builder =
                                builder.set_allow_methods(crate::protocol_serde::shape_non_empty_string_list::de_non_empty_string_list(tokens)?);
                        }
                        "AllowHeaders" => {
                            builder =
                                builder.set_allow_headers(crate::protocol_serde::shape_non_empty_string_list::de_non_empty_string_list(tokens)?);
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
