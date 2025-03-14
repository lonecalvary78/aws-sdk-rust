// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_output_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::OutputConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.s3_output_location {
        object.key("S3OutputLocation").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_device {
        object.key("TargetDevice").string(var_2.as_str());
    }
    if let Some(var_3) = &input.target_platform {
        #[allow(unused_mut)]
        let mut object_4 = object.key("TargetPlatform").start_object();
        crate::protocol_serde::shape_target_platform::ser_target_platform(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.compiler_options {
        object.key("CompilerOptions").string(var_5.as_str());
    }
    if let Some(var_6) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_6.as_str());
    }
    Ok(())
}

pub(crate) fn de_output_config<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::OutputConfig>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::OutputConfigBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "S3OutputLocation" => {
                            builder = builder.set_s3_output_location(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TargetDevice" => {
                            builder = builder.set_target_device(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::TargetDevice::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "TargetPlatform" => {
                            builder = builder.set_target_platform(crate::protocol_serde::shape_target_platform::de_target_platform(tokens)?);
                        }
                        "CompilerOptions" => {
                            builder = builder.set_compiler_options(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "KmsKeyId" => {
                            builder = builder.set_kms_key_id(
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
            Ok(Some(crate::serde_util::output_config_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
