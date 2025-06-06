// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_nielsen_non_linear_watermark_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::NielsenNonLinearWatermarkSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.active_watermark_process {
        object.key("activeWatermarkProcess").string(var_1.as_str());
    }
    if let Some(var_2) = &input.adi_filename {
        object.key("adiFilename").string(var_2.as_str());
    }
    if let Some(var_3) = &input.asset_id {
        object.key("assetId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.asset_name {
        object.key("assetName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.cbet_source_id {
        object.key("cbetSourceId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.episode_id {
        object.key("episodeId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.metadata_destination {
        object.key("metadataDestination").string(var_7.as_str());
    }
    if let Some(var_8) = &input.source_id {
        object.key("sourceId").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.source_watermark_status {
        object.key("sourceWatermarkStatus").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tic_server_url {
        object.key("ticServerUrl").string(var_10.as_str());
    }
    if let Some(var_11) = &input.unique_tic_per_audio_track {
        object.key("uniqueTicPerAudioTrack").string(var_11.as_str());
    }
    Ok(())
}

pub(crate) fn de_nielsen_non_linear_watermark_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::NielsenNonLinearWatermarkSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::NielsenNonLinearWatermarkSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "activeWatermarkProcess" => {
                            builder = builder.set_active_watermark_process(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::NielsenActiveWatermarkProcessType::from(u.as_ref()))
                                    })
                                    .transpose()?,
                            );
                        }
                        "adiFilename" => {
                            builder = builder.set_adi_filename(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "assetId" => {
                            builder = builder.set_asset_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "assetName" => {
                            builder = builder.set_asset_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "cbetSourceId" => {
                            builder = builder.set_cbet_source_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "episodeId" => {
                            builder = builder.set_episode_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "metadataDestination" => {
                            builder = builder.set_metadata_destination(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "sourceId" => {
                            builder = builder.set_source_id(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "sourceWatermarkStatus" => {
                            builder = builder.set_source_watermark_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::NielsenSourceWatermarkStatusType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ticServerUrl" => {
                            builder = builder.set_tic_server_url(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "uniqueTicPerAudioTrack" => {
                            builder = builder.set_unique_tic_per_audio_track(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::NielsenUniqueTicPerAudioTrackType::from(u.as_ref()))
                                    })
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
