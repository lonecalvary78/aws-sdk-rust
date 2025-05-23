// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_avail_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AvailSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.esam {
        #[allow(unused_mut)]
        let mut object_2 = object.key("esam").start_object();
        crate::protocol_serde::shape_esam::ser_esam(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.scte35_splice_insert {
        #[allow(unused_mut)]
        let mut object_4 = object.key("scte35SpliceInsert").start_object();
        crate::protocol_serde::shape_scte35_splice_insert::ser_scte35_splice_insert(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.scte35_time_signal_apos {
        #[allow(unused_mut)]
        let mut object_6 = object.key("scte35TimeSignalApos").start_object();
        crate::protocol_serde::shape_scte35_time_signal_apos::ser_scte35_time_signal_apos(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_avail_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AvailSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AvailSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "esam" => {
                            builder = builder.set_esam(crate::protocol_serde::shape_esam::de_esam(tokens)?);
                        }
                        "scte35SpliceInsert" => {
                            builder =
                                builder.set_scte35_splice_insert(crate::protocol_serde::shape_scte35_splice_insert::de_scte35_splice_insert(tokens)?);
                        }
                        "scte35TimeSignalApos" => {
                            builder = builder.set_scte35_time_signal_apos(
                                crate::protocol_serde::shape_scte35_time_signal_apos::de_scte35_time_signal_apos(tokens)?,
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
