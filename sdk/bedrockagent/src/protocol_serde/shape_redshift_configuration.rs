// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_redshift_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RedshiftConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("storageConfigurations").start_array();
        for item_2 in &input.storage_configurations {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_redshift_query_engine_storage_configuration::ser_redshift_query_engine_storage_configuration(
                    &mut object_3,
                    item_2,
                )?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    if let Some(var_4) = &input.query_engine_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("queryEngineConfiguration").start_object();
        crate::protocol_serde::shape_redshift_query_engine_configuration::ser_redshift_query_engine_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.query_generation_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("queryGenerationConfiguration").start_object();
        crate::protocol_serde::shape_query_generation_configuration::ser_query_generation_configuration(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}

pub(crate) fn de_redshift_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::RedshiftConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RedshiftConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "storageConfigurations" => {
                            builder = builder.set_storage_configurations(
                                    crate::protocol_serde::shape_redshift_query_engine_storage_configurations::de_redshift_query_engine_storage_configurations(tokens)?
                                );
                        }
                        "queryEngineConfiguration" => {
                            builder = builder.set_query_engine_configuration(
                                crate::protocol_serde::shape_redshift_query_engine_configuration::de_redshift_query_engine_configuration(tokens)?,
                            );
                        }
                        "queryGenerationConfiguration" => {
                            builder = builder.set_query_generation_configuration(
                                crate::protocol_serde::shape_query_generation_configuration::de_query_generation_configuration(tokens)?,
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
            Ok(Some(crate::serde_util::redshift_configuration_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
