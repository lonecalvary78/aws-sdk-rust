// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ebu_tt_d_destination_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EbuTtDDestinationSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.copyright_holder {
        object.key("copyrightHolder").string(var_1.as_str());
    }
    if let Some(var_2) = &input.fill_line_gap {
        object.key("fillLineGap").string(var_2.as_str());
    }
    if let Some(var_3) = &input.font_family {
        object.key("fontFamily").string(var_3.as_str());
    }
    if let Some(var_4) = &input.style_control {
        object.key("styleControl").string(var_4.as_str());
    }
    if let Some(var_5) = &input.default_font_size {
        object.key("defaultFontSize").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.default_line_height {
        object.key("defaultLineHeight").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    Ok(())
}

pub(crate) fn de_ebu_tt_d_destination_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::EbuTtDDestinationSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::EbuTtDDestinationSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "copyrightHolder" => {
                            builder = builder.set_copyright_holder(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "fillLineGap" => {
                            builder = builder.set_fill_line_gap(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EbuTtDFillLineGapControl::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "fontFamily" => {
                            builder = builder.set_font_family(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "styleControl" => {
                            builder = builder.set_style_control(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EbuTtDDestinationStyleControl::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "defaultFontSize" => {
                            builder = builder.set_default_font_size(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "defaultLineHeight" => {
                            builder = builder.set_default_line_height(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
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
