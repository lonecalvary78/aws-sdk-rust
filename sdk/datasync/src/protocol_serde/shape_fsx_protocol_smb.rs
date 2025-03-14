// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_fsx_protocol_smb(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FsxProtocolSmb,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.domain {
        object.key("Domain").string(var_1.as_str());
    }
    if let Some(var_2) = &input.mount_options {
        #[allow(unused_mut)]
        let mut object_3 = object.key("MountOptions").start_object();
        crate::protocol_serde::shape_smb_mount_options::ser_smb_mount_options(&mut object_3, var_2)?;
        object_3.finish();
    }
    {
        object.key("Password").string(input.password.as_str());
    }
    {
        object.key("User").string(input.user.as_str());
    }
    Ok(())
}

pub(crate) fn de_fsx_protocol_smb<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::FsxProtocolSmb>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::FsxProtocolSmbBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Domain" => {
                            builder = builder.set_domain(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "MountOptions" => {
                            builder = builder.set_mount_options(crate::protocol_serde::shape_smb_mount_options::de_smb_mount_options(tokens)?);
                        }
                        "Password" => {
                            builder = builder.set_password(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "User" => {
                            builder = builder.set_user(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
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
            Ok(Some(crate::serde_util::fsx_protocol_smb_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
