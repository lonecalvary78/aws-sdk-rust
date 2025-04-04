// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tool_result_content_block(
    object_3: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ToolResultContentBlock,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::ToolResultContentBlock::Json(inner) => {
            object_3.key("json").document(inner);
        }
        crate::types::ToolResultContentBlock::Text(inner) => {
            object_3.key("text").string(inner.as_str());
        }
        crate::types::ToolResultContentBlock::Image(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_3.key("image").start_object();
            crate::protocol_serde::shape_image_block::ser_image_block(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::ToolResultContentBlock::Document(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_3.key("document").start_object();
            crate::protocol_serde::shape_document_block::ser_document_block(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::ToolResultContentBlock::Video(inner) => {
            #[allow(unused_mut)]
            let mut object_3 = object_3.key("video").start_object();
            crate::protocol_serde::shape_video_block::ser_video_block(&mut object_3, inner)?;
            object_3.finish();
        }
        crate::types::ToolResultContentBlock::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "ToolResultContentBlock",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_tool_result_content_block<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ToolResultContentBlock>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
                        "json" => Some(crate::types::ToolResultContentBlock::Json(
                            Some(::aws_smithy_json::deserialize::token::expect_document(tokens)?)
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'json' cannot be null"))?,
                        )),
                        "text" => Some(crate::types::ToolResultContentBlock::Text(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'text' cannot be null"))?,
                        )),
                        "image" => Some(crate::types::ToolResultContentBlock::Image(
                            crate::protocol_serde::shape_image_block::de_image_block(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'image' cannot be null"))?,
                        )),
                        "document" => Some(crate::types::ToolResultContentBlock::Document(
                            crate::protocol_serde::shape_document_block::de_document_block(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'document' cannot be null")
                            })?,
                        )),
                        "video" => Some(crate::types::ToolResultContentBlock::Video(
                            crate::protocol_serde::shape_video_block::de_video_block(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'video' cannot be null"))?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::ToolResultContentBlock::Unknown)
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
