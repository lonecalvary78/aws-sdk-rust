// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_certificate_validity(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CertificateValidity,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.validity_period {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ValidityPeriod").start_object();
        crate::protocol_serde::shape_validity_period::ser_validity_period(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.renewal_period {
        #[allow(unused_mut)]
        let mut object_4 = object.key("RenewalPeriod").start_object();
        crate::protocol_serde::shape_validity_period::ser_validity_period(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_certificate_validity<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::CertificateValidity>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CertificateValidityBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ValidityPeriod" => {
                            builder = builder.set_validity_period(crate::protocol_serde::shape_validity_period::de_validity_period(tokens)?);
                        }
                        "RenewalPeriod" => {
                            builder = builder.set_renewal_period(crate::protocol_serde::shape_validity_period::de_validity_period(tokens)?);
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
            Ok(Some(crate::serde_util::certificate_validity_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
