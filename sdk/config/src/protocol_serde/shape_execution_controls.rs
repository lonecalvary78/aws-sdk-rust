// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_execution_controls(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ExecutionControls,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.ssm_controls {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SsmControls").start_object();
        crate::protocol_serde::shape_ssm_controls::ser_ssm_controls(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_execution_controls<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ExecutionControls>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ExecutionControlsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "SsmControls" => {
                            builder = builder.set_ssm_controls(crate::protocol_serde::shape_ssm_controls::de_ssm_controls(tokens)?);
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
