// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_service_discovery(
    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ServiceDiscovery,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::ServiceDiscovery::Dns(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_2.key("dns").start_object();
            crate::protocol_serde::shape_dns_service_discovery::ser_dns_service_discovery(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::ServiceDiscovery::AwsCloudMap(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_2.key("awsCloudMap").start_object();
            crate::protocol_serde::shape_aws_cloud_map_service_discovery::ser_aws_cloud_map_service_discovery(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::ServiceDiscovery::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "ServiceDiscovery",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_service_discovery<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ServiceDiscovery>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
                        "dns" => Some(crate::types::ServiceDiscovery::Dns(
                            crate::protocol_serde::shape_dns_service_discovery::de_dns_service_discovery(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'dns' cannot be null"))?,
                        )),
                        "awsCloudMap" => Some(crate::types::ServiceDiscovery::AwsCloudMap(
                            crate::protocol_serde::shape_aws_cloud_map_service_discovery::de_aws_cloud_map_service_discovery(tokens)?.ok_or_else(
                                || ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'awsCloudMap' cannot be null"),
                            )?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::ServiceDiscovery::Unknown)
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
