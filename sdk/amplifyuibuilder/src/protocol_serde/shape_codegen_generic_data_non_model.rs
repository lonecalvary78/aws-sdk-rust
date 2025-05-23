// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_codegen_generic_data_non_model(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CodegenGenericDataNonModel,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        #[allow(unused_mut)]
        let mut object_1 = object.key("fields").start_object();
        for (key_2, value_3) in &input.fields {
            {
                #[allow(unused_mut)]
                let mut object_4 = object_1.key(key_2.as_str()).start_object();
                crate::protocol_serde::shape_codegen_generic_data_field::ser_codegen_generic_data_field(&mut object_4, value_3)?;
                object_4.finish();
            }
        }
        object_1.finish();
    }
    Ok(())
}

pub(crate) fn de_codegen_generic_data_non_model<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::CodegenGenericDataNonModel>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CodegenGenericDataNonModelBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "fields" => {
                            builder = builder.set_fields(
                                crate::protocol_serde::shape_codegen_generic_data_non_model_fields::de_codegen_generic_data_non_model_fields(tokens)?,
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
                crate::serde_util::codegen_generic_data_non_model_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
