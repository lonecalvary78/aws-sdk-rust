// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_caption_source_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CaptionSourceSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.ancillary_source_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ancillarySourceSettings").start_object();
        crate::protocol_serde::shape_ancillary_source_settings::ser_ancillary_source_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.dvb_sub_source_settings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("dvbSubSourceSettings").start_object();
        crate::protocol_serde::shape_dvb_sub_source_settings::ser_dvb_sub_source_settings(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.embedded_source_settings {
        #[allow(unused_mut)]
        let mut object_6 = object.key("embeddedSourceSettings").start_object();
        crate::protocol_serde::shape_embedded_source_settings::ser_embedded_source_settings(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.file_source_settings {
        #[allow(unused_mut)]
        let mut object_8 = object.key("fileSourceSettings").start_object();
        crate::protocol_serde::shape_file_source_settings::ser_file_source_settings(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.source_type {
        object.key("sourceType").string(var_9.as_str());
    }
    if let Some(var_10) = &input.teletext_source_settings {
        #[allow(unused_mut)]
        let mut object_11 = object.key("teletextSourceSettings").start_object();
        crate::protocol_serde::shape_teletext_source_settings::ser_teletext_source_settings(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.track_source_settings {
        #[allow(unused_mut)]
        let mut object_13 = object.key("trackSourceSettings").start_object();
        crate::protocol_serde::shape_track_source_settings::ser_track_source_settings(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.webvtt_hls_source_settings {
        #[allow(unused_mut)]
        let mut object_15 = object.key("webvttHlsSourceSettings").start_object();
        crate::protocol_serde::shape_webvtt_hls_source_settings::ser_webvtt_hls_source_settings(&mut object_15, var_14)?;
        object_15.finish();
    }
    Ok(())
}

pub(crate) fn de_caption_source_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::CaptionSourceSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CaptionSourceSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ancillarySourceSettings" => {
                            builder = builder.set_ancillary_source_settings(
                                crate::protocol_serde::shape_ancillary_source_settings::de_ancillary_source_settings(tokens)?,
                            );
                        }
                        "dvbSubSourceSettings" => {
                            builder = builder.set_dvb_sub_source_settings(
                                crate::protocol_serde::shape_dvb_sub_source_settings::de_dvb_sub_source_settings(tokens)?,
                            );
                        }
                        "embeddedSourceSettings" => {
                            builder = builder.set_embedded_source_settings(
                                crate::protocol_serde::shape_embedded_source_settings::de_embedded_source_settings(tokens)?,
                            );
                        }
                        "fileSourceSettings" => {
                            builder =
                                builder.set_file_source_settings(crate::protocol_serde::shape_file_source_settings::de_file_source_settings(tokens)?);
                        }
                        "sourceType" => {
                            builder = builder.set_source_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::CaptionSourceType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "teletextSourceSettings" => {
                            builder = builder.set_teletext_source_settings(
                                crate::protocol_serde::shape_teletext_source_settings::de_teletext_source_settings(tokens)?,
                            );
                        }
                        "trackSourceSettings" => {
                            builder = builder
                                .set_track_source_settings(crate::protocol_serde::shape_track_source_settings::de_track_source_settings(tokens)?);
                        }
                        "webvttHlsSourceSettings" => {
                            builder = builder.set_webvtt_hls_source_settings(
                                crate::protocol_serde::shape_webvtt_hls_source_settings::de_webvtt_hls_source_settings(tokens)?,
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
