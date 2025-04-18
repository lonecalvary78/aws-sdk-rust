// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_eks_pod_properties_detail<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::EksPodPropertiesDetail>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::EksPodPropertiesDetailBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "serviceAccountName" => {
                            builder = builder.set_service_account_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "hostNetwork" => {
                            builder = builder.set_host_network(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "dnsPolicy" => {
                            builder = builder.set_dns_policy(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "imagePullSecrets" => {
                            builder = builder.set_image_pull_secrets(crate::protocol_serde::shape_image_pull_secrets::de_image_pull_secrets(tokens)?);
                        }
                        "containers" => {
                            builder = builder.set_containers(crate::protocol_serde::shape_eks_container_details::de_eks_container_details(tokens)?);
                        }
                        "initContainers" => {
                            builder =
                                builder.set_init_containers(crate::protocol_serde::shape_eks_container_details::de_eks_container_details(tokens)?);
                        }
                        "volumes" => {
                            builder = builder.set_volumes(crate::protocol_serde::shape_eks_volumes::de_eks_volumes(tokens)?);
                        }
                        "podName" => {
                            builder = builder.set_pod_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "nodeName" => {
                            builder = builder.set_node_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "metadata" => {
                            builder = builder.set_metadata(crate::protocol_serde::shape_eks_metadata::de_eks_metadata(tokens)?);
                        }
                        "shareProcessNamespace" => {
                            builder = builder.set_share_process_namespace(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
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
