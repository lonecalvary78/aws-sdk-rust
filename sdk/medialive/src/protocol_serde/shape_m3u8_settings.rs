// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_m3u8_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::M3u8Settings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.audio_frames_per_pes {
        object.key("audioFramesPerPes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.audio_pids {
        object.key("audioPids").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ecm_pid {
        object.key("ecmPid").string(var_3.as_str());
    }
    if let Some(var_4) = &input.nielsen_id3_behavior {
        object.key("nielsenId3Behavior").string(var_4.as_str());
    }
    if let Some(var_5) = &input.pat_interval {
        object.key("patInterval").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.pcr_control {
        object.key("pcrControl").string(var_6.as_str());
    }
    if let Some(var_7) = &input.pcr_period {
        object.key("pcrPeriod").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.pcr_pid {
        object.key("pcrPid").string(var_8.as_str());
    }
    if let Some(var_9) = &input.pmt_interval {
        object.key("pmtInterval").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_9).into()),
        );
    }
    if let Some(var_10) = &input.pmt_pid {
        object.key("pmtPid").string(var_10.as_str());
    }
    if let Some(var_11) = &input.program_num {
        object.key("programNum").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.scte35_behavior {
        object.key("scte35Behavior").string(var_12.as_str());
    }
    if let Some(var_13) = &input.scte35_pid {
        object.key("scte35Pid").string(var_13.as_str());
    }
    if let Some(var_14) = &input.timed_metadata_behavior {
        object.key("timedMetadataBehavior").string(var_14.as_str());
    }
    if let Some(var_15) = &input.timed_metadata_pid {
        object.key("timedMetadataPid").string(var_15.as_str());
    }
    if let Some(var_16) = &input.transport_stream_id {
        object.key("transportStreamId").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    if let Some(var_17) = &input.video_pid {
        object.key("videoPid").string(var_17.as_str());
    }
    if let Some(var_18) = &input.klv_behavior {
        object.key("klvBehavior").string(var_18.as_str());
    }
    if let Some(var_19) = &input.klv_data_pids {
        object.key("klvDataPids").string(var_19.as_str());
    }
    Ok(())
}

pub(crate) fn de_m3u8_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::M3u8Settings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::M3u8SettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "audioFramesPerPes" => {
                            builder = builder.set_audio_frames_per_pes(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "audioPids" => {
                            builder = builder.set_audio_pids(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ecmPid" => {
                            builder = builder.set_ecm_pid(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "nielsenId3Behavior" => {
                            builder = builder.set_nielsen_id3_behavior(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::M3u8NielsenId3Behavior::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "patInterval" => {
                            builder = builder.set_pat_interval(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "pcrControl" => {
                            builder = builder.set_pcr_control(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::M3u8PcrControl::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "pcrPeriod" => {
                            builder = builder.set_pcr_period(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "pcrPid" => {
                            builder = builder.set_pcr_pid(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "pmtInterval" => {
                            builder = builder.set_pmt_interval(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "pmtPid" => {
                            builder = builder.set_pmt_pid(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "programNum" => {
                            builder = builder.set_program_num(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "scte35Behavior" => {
                            builder = builder.set_scte35_behavior(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::M3u8Scte35Behavior::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "scte35Pid" => {
                            builder = builder.set_scte35_pid(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "timedMetadataBehavior" => {
                            builder = builder.set_timed_metadata_behavior(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::M3u8TimedMetadataBehavior::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "timedMetadataPid" => {
                            builder = builder.set_timed_metadata_pid(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "transportStreamId" => {
                            builder = builder.set_transport_stream_id(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "videoPid" => {
                            builder = builder.set_video_pid(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "klvBehavior" => {
                            builder = builder.set_klv_behavior(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::M3u8KlvBehavior::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "klvDataPids" => {
                            builder = builder.set_klv_data_pids(
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
