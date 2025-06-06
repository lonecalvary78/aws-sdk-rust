// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_msk_cluster_cluster_info_client_authentication_sasl_iam_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsMskClusterClusterInfoClientAuthenticationSaslIamDetails,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.enabled {
        object.key("Enabled").boolean(*var_1);
    }
    Ok(())
}

pub(crate) fn de_aws_msk_cluster_cluster_info_client_authentication_sasl_iam_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<
    Option<crate::types::AwsMskClusterClusterInfoClientAuthenticationSaslIamDetails>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsMskClusterClusterInfoClientAuthenticationSaslIamDetailsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Enabled" => {
                            builder = builder.set_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
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
