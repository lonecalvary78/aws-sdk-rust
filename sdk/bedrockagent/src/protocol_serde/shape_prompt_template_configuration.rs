// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_prompt_template_configuration(
    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PromptTemplateConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::PromptTemplateConfiguration::Text(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_2.key("text").start_object();
            crate::protocol_serde::shape_text_prompt_template_configuration::ser_text_prompt_template_configuration(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::PromptTemplateConfiguration::Chat(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_2.key("chat").start_object();
            crate::protocol_serde::shape_chat_prompt_template_configuration::ser_chat_prompt_template_configuration(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::PromptTemplateConfiguration::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "PromptTemplateConfiguration",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_prompt_template_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::PromptTemplateConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
                        "text" => Some(crate::types::PromptTemplateConfiguration::Text(
                            crate::protocol_serde::shape_text_prompt_template_configuration::de_text_prompt_template_configuration(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'text' cannot be null"))?,
                        )),
                        "chat" => Some(crate::types::PromptTemplateConfiguration::Chat(
                            crate::protocol_serde::shape_chat_prompt_template_configuration::de_chat_prompt_template_configuration(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'chat' cannot be null"))?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::PromptTemplateConfiguration::Unknown)
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
