// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_command_parameter_value<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::CommandParameterValue>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CommandParameterValueBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "S" => {
                            builder = builder.set_s(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "B" => {
                            builder = builder.set_b(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "I" => {
                            builder = builder.set_i(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "L" => {
                            builder = builder.set_l(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "D" => {
                            builder =
                                builder.set_d(::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()));
                        }
                        "BIN" => {
                            builder = builder.set_bin(::aws_smithy_json::deserialize::token::expect_blob_or_null(tokens.next())?);
                        }
                        "UL" => {
                            builder = builder.set_ul(
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}

pub fn ser_command_parameter_value(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CommandParameterValue,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.s {
        object.key("S").string(var_1.as_str());
    }
    if let Some(var_2) = &input.b {
        object.key("B").boolean(*var_2);
    }
    if let Some(var_3) = &input.i {
        object.key("I").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.l {
        object.key("L").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.d {
        object.key("D").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.bin {
        object.key("BIN").string_unchecked(&::aws_smithy_types::base64::encode(var_6));
    }
    if let Some(var_7) = &input.ul {
        object.key("UL").string(var_7.as_str());
    }
    Ok(())
}
