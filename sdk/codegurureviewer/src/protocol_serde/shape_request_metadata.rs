// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_request_metadata<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::RequestMetadata>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RequestMetadataBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "RequestId" => {
                            builder = builder.set_request_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Requester" => {
                            builder = builder.set_requester(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "EventInfo" => {
                            builder = builder.set_event_info(crate::protocol_serde::shape_event_info::de_event_info(tokens)?);
                        }
                        "VendorName" => {
                            builder = builder.set_vendor_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::VendorName::from(u.as_ref())))
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

pub fn ser_request_metadata(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RequestMetadata,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.request_id {
        object.key("RequestId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.requester {
        object.key("Requester").string(var_2.as_str());
    }
    if let Some(var_3) = &input.event_info {
        #[allow(unused_mut)]
        let mut object_4 = object.key("EventInfo").start_object();
        crate::protocol_serde::shape_event_info::ser_event_info(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.vendor_name {
        object.key("VendorName").string(var_5.as_str());
    }
    Ok(())
}
