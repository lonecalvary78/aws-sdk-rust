// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_mainframe_action_type(
    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MainframeActionType,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::MainframeActionType::Batch(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_2.key("batch").start_object();
            crate::protocol_serde::shape_batch::ser_batch(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::MainframeActionType::Tn3270(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_2.key("tn3270").start_object();
            crate::protocol_serde::shape_tn3270::ser_tn3270(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::MainframeActionType::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "MainframeActionType",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_mainframe_action_type<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::MainframeActionType>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
                        tokens.peek()
                    {
                        let _ = tokens.next().expect("peek returned a token")?;
                        continue;
                    }
                    let key = key.to_unescaped()?;
                    if key == "__type" {
                        ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                        continue;
                    }
                    if variant.is_some() {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            "encountered mixed variants in union",
                        ));
                    }
                    variant = match key.as_ref() {
                        "batch" => Some(crate::types::MainframeActionType::Batch(
                            crate::protocol_serde::shape_batch::de_batch(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'batch' cannot be null"))?,
                        )),
                        "tn3270" => Some(crate::types::MainframeActionType::Tn3270(
                            crate::protocol_serde::shape_tn3270::de_tn3270(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'tn3270' cannot be null")
                            })?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::MainframeActionType::Unknown)
                        }
                    };
                }
                other => {
                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )))
                }
            }
        },
        _ => {
            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ))
        }
    }
    if variant.is_none() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "Union did not contain a valid variant.",
        ));
    }
    Ok(variant)
}
