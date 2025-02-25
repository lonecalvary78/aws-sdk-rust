// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_pickup_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PickupDetails,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.phone_number {
        object.key("PhoneNumber").string(var_2.as_str());
    }
    if let Some(var_3) = &input.email {
        object.key("Email").string(var_3.as_str());
    }
    if let Some(var_4) = &input.identification_number {
        object.key("IdentificationNumber").string(var_4.as_str());
    }
    if let Some(var_5) = &input.identification_expiration_date {
        object
            .key("IdentificationExpirationDate")
            .date_time(var_5, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_6) = &input.identification_issuing_org {
        object.key("IdentificationIssuingOrg").string(var_6.as_str());
    }
    if let Some(var_7) = &input.device_pickup_id {
        object.key("DevicePickupId").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_pickup_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::PickupDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::PickupDetailsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PhoneNumber" => {
                            builder = builder.set_phone_number(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Email" => {
                            builder = builder.set_email(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "IdentificationNumber" => {
                            builder = builder.set_identification_number(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "IdentificationExpirationDate" => {
                            builder = builder.set_identification_expiration_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "IdentificationIssuingOrg" => {
                            builder = builder.set_identification_issuing_org(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "DevicePickupId" => {
                            builder = builder.set_device_pickup_id(
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
