// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_category_inner_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CategoryInnerFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.column {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Column").start_object();
        crate::protocol_serde::shape_column_identifier::ser_column_identifier(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Configuration").start_object();
        crate::protocol_serde::shape_category_filter_configuration::ser_category_filter_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.default_filter_control_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("DefaultFilterControlConfiguration").start_object();
        crate::protocol_serde::shape_default_filter_control_configuration::ser_default_filter_control_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_category_inner_filter<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::CategoryInnerFilter>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CategoryInnerFilterBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Column" => {
                            builder = builder.set_column(crate::protocol_serde::shape_column_identifier::de_column_identifier(tokens)?);
                        }
                        "Configuration" => {
                            builder = builder.set_configuration(
                                crate::protocol_serde::shape_category_filter_configuration::de_category_filter_configuration(tokens)?,
                            );
                        }
                        "DefaultFilterControlConfiguration" => {
                            builder = builder.set_default_filter_control_configuration(
                                crate::protocol_serde::shape_default_filter_control_configuration::de_default_filter_control_configuration(tokens)?,
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
            Ok(Some(crate::serde_util::category_inner_filter_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
