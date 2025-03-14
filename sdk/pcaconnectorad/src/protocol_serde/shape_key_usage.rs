// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_key_usage(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::KeyUsage,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.critical {
        object.key("Critical").boolean(*var_1);
    }
    if let Some(var_2) = &input.usage_flags {
        #[allow(unused_mut)]
        let mut object_3 = object.key("UsageFlags").start_object();
        crate::protocol_serde::shape_key_usage_flags::ser_key_usage_flags(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

pub(crate) fn de_key_usage<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::KeyUsage>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::KeyUsageBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Critical" => {
                            builder = builder.set_critical(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "UsageFlags" => {
                            builder = builder.set_usage_flags(crate::protocol_serde::shape_key_usage_flags::de_key_usage_flags(tokens)?);
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
            Ok(Some(crate::serde_util::key_usage_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
