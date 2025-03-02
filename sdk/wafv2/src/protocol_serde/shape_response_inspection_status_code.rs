// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_response_inspection_status_code(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ResponseInspectionStatusCode,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("SuccessCodes").start_array();
        for item_2 in &input.success_codes {
            {
                array_1.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::NegInt((*item_2).into()),
                );
            }
        }
        array_1.finish();
    }
    {
        let mut array_3 = object.key("FailureCodes").start_array();
        for item_4 in &input.failure_codes {
            {
                array_3.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::NegInt((*item_4).into()),
                );
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub(crate) fn de_response_inspection_status_code<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ResponseInspectionStatusCode>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ResponseInspectionStatusCodeBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "SuccessCodes" => {
                            builder = builder.set_success_codes(
                                    crate::protocol_serde::shape_response_inspection_status_code_success_codes::de_response_inspection_status_code_success_codes(tokens)?
                                );
                        }
                        "FailureCodes" => {
                            builder = builder.set_failure_codes(
                                    crate::protocol_serde::shape_response_inspection_status_code_failure_codes::de_response_inspection_status_code_failure_codes(tokens)?
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
            Ok(Some(
                crate::serde_util::response_inspection_status_code_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
