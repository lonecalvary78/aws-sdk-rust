// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resource_type(
    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ResourceType,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::ResourceType::CloudFormation(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_2.key("cloudFormation").start_object();
            crate::protocol_serde::shape_cloud_formation::ser_cloud_formation(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::ResourceType::M2ManagedApplication(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_2.key("m2ManagedApplication").start_object();
            crate::protocol_serde::shape_m2_managed_application::ser_m2_managed_application(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::ResourceType::M2NonManagedApplication(inner) => {
            #[allow(unused_mut)]
            let mut object_3 = object_2.key("m2NonManagedApplication").start_object();
            crate::protocol_serde::shape_m2_non_managed_application::ser_m2_non_managed_application(&mut object_3, inner)?;
            object_3.finish();
        }
        crate::types::ResourceType::Unknown => return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant("ResourceType")),
    }
    Ok(())
}

pub(crate) fn de_resource_type<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ResourceType>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
                        "cloudFormation" => Some(crate::types::ResourceType::CloudFormation(
                            crate::protocol_serde::shape_cloud_formation::de_cloud_formation(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'cloudFormation' cannot be null")
                            })?,
                        )),
                        "m2ManagedApplication" => Some(crate::types::ResourceType::M2ManagedApplication(
                            crate::protocol_serde::shape_m2_managed_application::de_m2_managed_application(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'm2ManagedApplication' cannot be null")
                            })?,
                        )),
                        "m2NonManagedApplication" => Some(crate::types::ResourceType::M2NonManagedApplication(
                            crate::protocol_serde::shape_m2_non_managed_application::de_m2_non_managed_application(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'm2NonManagedApplication' cannot be null")
                            })?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::ResourceType::Unknown)
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
