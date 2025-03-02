// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_consumption_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ConsumptionConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.renew_type {
        object.key("RenewType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.provisional_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ProvisionalConfiguration").start_object();
        crate::protocol_serde::shape_provisional_configuration::ser_provisional_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.borrow_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("BorrowConfiguration").start_object();
        crate::protocol_serde::shape_borrow_configuration::ser_borrow_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

pub(crate) fn de_consumption_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ConsumptionConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ConsumptionConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "RenewType" => {
                            builder = builder.set_renew_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::RenewType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ProvisionalConfiguration" => {
                            builder = builder.set_provisional_configuration(
                                crate::protocol_serde::shape_provisional_configuration::de_provisional_configuration(tokens)?,
                            );
                        }
                        "BorrowConfiguration" => {
                            builder =
                                builder.set_borrow_configuration(crate::protocol_serde::shape_borrow_configuration::de_borrow_configuration(tokens)?);
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
