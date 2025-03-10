// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_message_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MessageConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.adm_message {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ADMMessage").start_object();
        crate::protocol_serde::shape_message::ser_message(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.apns_message {
        #[allow(unused_mut)]
        let mut object_4 = object.key("APNSMessage").start_object();
        crate::protocol_serde::shape_message::ser_message(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.baidu_message {
        #[allow(unused_mut)]
        let mut object_6 = object.key("BaiduMessage").start_object();
        crate::protocol_serde::shape_message::ser_message(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.custom_message {
        #[allow(unused_mut)]
        let mut object_8 = object.key("CustomMessage").start_object();
        crate::protocol_serde::shape_campaign_custom_message::ser_campaign_custom_message(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.default_message {
        #[allow(unused_mut)]
        let mut object_10 = object.key("DefaultMessage").start_object();
        crate::protocol_serde::shape_message::ser_message(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.email_message {
        #[allow(unused_mut)]
        let mut object_12 = object.key("EmailMessage").start_object();
        crate::protocol_serde::shape_campaign_email_message::ser_campaign_email_message(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.gcm_message {
        #[allow(unused_mut)]
        let mut object_14 = object.key("GCMMessage").start_object();
        crate::protocol_serde::shape_message::ser_message(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.sms_message {
        #[allow(unused_mut)]
        let mut object_16 = object.key("SMSMessage").start_object();
        crate::protocol_serde::shape_campaign_sms_message::ser_campaign_sms_message(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.in_app_message {
        #[allow(unused_mut)]
        let mut object_18 = object.key("InAppMessage").start_object();
        crate::protocol_serde::shape_campaign_in_app_message::ser_campaign_in_app_message(&mut object_18, var_17)?;
        object_18.finish();
    }
    Ok(())
}

pub(crate) fn de_message_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::MessageConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::MessageConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ADMMessage" => {
                            builder = builder.set_adm_message(crate::protocol_serde::shape_message::de_message(tokens)?);
                        }
                        "APNSMessage" => {
                            builder = builder.set_apns_message(crate::protocol_serde::shape_message::de_message(tokens)?);
                        }
                        "BaiduMessage" => {
                            builder = builder.set_baidu_message(crate::protocol_serde::shape_message::de_message(tokens)?);
                        }
                        "CustomMessage" => {
                            builder =
                                builder.set_custom_message(crate::protocol_serde::shape_campaign_custom_message::de_campaign_custom_message(tokens)?);
                        }
                        "DefaultMessage" => {
                            builder = builder.set_default_message(crate::protocol_serde::shape_message::de_message(tokens)?);
                        }
                        "EmailMessage" => {
                            builder =
                                builder.set_email_message(crate::protocol_serde::shape_campaign_email_message::de_campaign_email_message(tokens)?);
                        }
                        "GCMMessage" => {
                            builder = builder.set_gcm_message(crate::protocol_serde::shape_message::de_message(tokens)?);
                        }
                        "SMSMessage" => {
                            builder = builder.set_sms_message(crate::protocol_serde::shape_campaign_sms_message::de_campaign_sms_message(tokens)?);
                        }
                        "InAppMessage" => {
                            builder =
                                builder.set_in_app_message(crate::protocol_serde::shape_campaign_in_app_message::de_campaign_in_app_message(tokens)?);
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
