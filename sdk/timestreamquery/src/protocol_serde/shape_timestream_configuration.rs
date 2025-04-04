// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_timestream_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TimestreamConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("DatabaseName").string(input.database_name.as_str());
    }
    {
        object.key("TableName").string(input.table_name.as_str());
    }
    {
        object.key("TimeColumn").string(input.time_column.as_str());
    }
    {
        let mut array_1 = object.key("DimensionMappings").start_array();
        for item_2 in &input.dimension_mappings {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_dimension_mapping::ser_dimension_mapping(&mut object_3, item_2)?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    if let Some(var_4) = &input.multi_measure_mappings {
        #[allow(unused_mut)]
        let mut object_5 = object.key("MultiMeasureMappings").start_object();
        crate::protocol_serde::shape_multi_measure_mappings::ser_multi_measure_mappings(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.mixed_measure_mappings {
        let mut array_7 = object.key("MixedMeasureMappings").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_mixed_measure_mapping::ser_mixed_measure_mapping(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.measure_name_column {
        object.key("MeasureNameColumn").string(var_10.as_str());
    }
    Ok(())
}

pub(crate) fn de_timestream_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TimestreamConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TimestreamConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "DatabaseName" => {
                            builder = builder.set_database_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TableName" => {
                            builder = builder.set_table_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TimeColumn" => {
                            builder = builder.set_time_column(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "DimensionMappings" => {
                            builder = builder
                                .set_dimension_mappings(crate::protocol_serde::shape_dimension_mapping_list::de_dimension_mapping_list(tokens)?);
                        }
                        "MultiMeasureMappings" => {
                            builder = builder
                                .set_multi_measure_mappings(crate::protocol_serde::shape_multi_measure_mappings::de_multi_measure_mappings(tokens)?);
                        }
                        "MixedMeasureMappings" => {
                            builder = builder.set_mixed_measure_mappings(
                                crate::protocol_serde::shape_mixed_measure_mapping_list::de_mixed_measure_mapping_list(tokens)?,
                            );
                        }
                        "MeasureNameColumn" => {
                            builder = builder.set_measure_name_column(
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
                crate::serde_util::timestream_configuration_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
