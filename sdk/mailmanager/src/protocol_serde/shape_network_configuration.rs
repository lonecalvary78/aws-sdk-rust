// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_network_configuration(
    object_9: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::NetworkConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::NetworkConfiguration::PublicNetworkConfiguration(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_9.key("PublicNetworkConfiguration").start_object();
            crate::protocol_serde::shape_public_network_configuration::ser_public_network_configuration(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::NetworkConfiguration::PrivateNetworkConfiguration(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_9.key("PrivateNetworkConfiguration").start_object();
            crate::protocol_serde::shape_private_network_configuration::ser_private_network_configuration(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::NetworkConfiguration::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "NetworkConfiguration",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_network_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::NetworkConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
                        "PublicNetworkConfiguration" => Some(crate::types::NetworkConfiguration::PublicNetworkConfiguration(
                            crate::protocol_serde::shape_public_network_configuration::de_public_network_configuration(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                    "value for 'PublicNetworkConfiguration' cannot be null",
                                )
                            })?,
                        )),
                        "PrivateNetworkConfiguration" => Some(crate::types::NetworkConfiguration::PrivateNetworkConfiguration(
                            crate::protocol_serde::shape_private_network_configuration::de_private_network_configuration(tokens)?.ok_or_else(
                                || {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                        "value for 'PrivateNetworkConfiguration' cannot be null",
                                    )
                                },
                            )?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::NetworkConfiguration::Unknown)
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
