// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_column_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ColumnConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.column {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Column").start_object();
        crate::protocol_serde::shape_column_identifier::ser_column_identifier(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.format_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("FormatConfiguration").start_object();
        crate::protocol_serde::shape_format_configuration::ser_format_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.role {
        object.key("Role").string(var_5.as_str());
    }
    if let Some(var_6) = &input.colors_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ColorsConfiguration").start_object();
        crate::protocol_serde::shape_colors_configuration::ser_colors_configuration(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}

pub(crate) fn de_column_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ColumnConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ColumnConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Column" => {
                            builder = builder.set_column(crate::protocol_serde::shape_column_identifier::de_column_identifier(tokens)?);
                        }
                        "FormatConfiguration" => {
                            builder =
                                builder.set_format_configuration(crate::protocol_serde::shape_format_configuration::de_format_configuration(tokens)?);
                        }
                        "Role" => {
                            builder = builder.set_role(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ColumnRole::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ColorsConfiguration" => {
                            builder =
                                builder.set_colors_configuration(crate::protocol_serde::shape_colors_configuration::de_colors_configuration(tokens)?);
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
            Ok(Some(crate::serde_util::column_configuration_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
