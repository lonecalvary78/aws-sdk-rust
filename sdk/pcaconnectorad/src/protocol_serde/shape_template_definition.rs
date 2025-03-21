// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_template_definition(
    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TemplateDefinition,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::TemplateDefinition::TemplateV2(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_4.key("TemplateV2").start_object();
            crate::protocol_serde::shape_template_v2::ser_template_v2(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::TemplateDefinition::TemplateV3(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_4.key("TemplateV3").start_object();
            crate::protocol_serde::shape_template_v3::ser_template_v3(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::TemplateDefinition::TemplateV4(inner) => {
            #[allow(unused_mut)]
            let mut object_3 = object_4.key("TemplateV4").start_object();
            crate::protocol_serde::shape_template_v4::ser_template_v4(&mut object_3, inner)?;
            object_3.finish();
        }
        crate::types::TemplateDefinition::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "TemplateDefinition",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_template_definition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TemplateDefinition>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
                        "TemplateV2" => Some(crate::types::TemplateDefinition::TemplateV2(
                            crate::protocol_serde::shape_template_v2::de_template_v2(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'TemplateV2' cannot be null")
                            })?,
                        )),
                        "TemplateV3" => Some(crate::types::TemplateDefinition::TemplateV3(
                            crate::protocol_serde::shape_template_v3::de_template_v3(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'TemplateV3' cannot be null")
                            })?,
                        )),
                        "TemplateV4" => Some(crate::types::TemplateDefinition::TemplateV4(
                            crate::protocol_serde::shape_template_v4::de_template_v4(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'TemplateV4' cannot be null")
                            })?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::TemplateDefinition::Unknown)
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
