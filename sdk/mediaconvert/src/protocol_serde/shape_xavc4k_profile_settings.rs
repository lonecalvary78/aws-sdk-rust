// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_xavc4k_profile_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Xavc4kProfileSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.bitrate_class {
        object.key("bitrateClass").string(var_1.as_str());
    }
    if let Some(var_2) = &input.codec_profile {
        object.key("codecProfile").string(var_2.as_str());
    }
    if let Some(var_3) = &input.flicker_adaptive_quantization {
        object.key("flickerAdaptiveQuantization").string(var_3.as_str());
    }
    if let Some(var_4) = &input.gop_b_reference {
        object.key("gopBReference").string(var_4.as_str());
    }
    if let Some(var_5) = &input.gop_closed_cadence {
        object.key("gopClosedCadence").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.hrd_buffer_size {
        object.key("hrdBufferSize").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.quality_tuning_level {
        object.key("qualityTuningLevel").string(var_7.as_str());
    }
    if let Some(var_8) = &input.slices {
        object.key("slices").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    Ok(())
}

pub(crate) fn de_xavc4k_profile_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::Xavc4kProfileSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::Xavc4kProfileSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "bitrateClass" => {
                            builder = builder.set_bitrate_class(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Xavc4kProfileBitrateClass::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "codecProfile" => {
                            builder = builder.set_codec_profile(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Xavc4kProfileCodecProfile::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "flickerAdaptiveQuantization" => {
                            builder = builder.set_flicker_adaptive_quantization(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::XavcFlickerAdaptiveQuantization::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "gopBReference" => {
                            builder = builder.set_gop_b_reference(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::XavcGopBReference::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "gopClosedCadence" => {
                            builder = builder.set_gop_closed_cadence(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "hrdBufferSize" => {
                            builder = builder.set_hrd_buffer_size(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "qualityTuningLevel" => {
                            builder = builder.set_quality_tuning_level(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Xavc4kProfileQualityTuningLevel::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "slices" => {
                            builder = builder.set_slices(
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
