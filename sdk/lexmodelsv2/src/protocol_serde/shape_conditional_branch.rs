// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_conditional_branch(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ConditionalBranch,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    if let Some(var_1) = &input.condition {
        #[allow(unused_mut)]
        let mut object_2 = object.key("condition").start_object();
        crate::protocol_serde::shape_condition::ser_condition(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.next_step {
        #[allow(unused_mut)]
        let mut object_4 = object.key("nextStep").start_object();
        crate::protocol_serde::shape_dialog_state::ser_dialog_state(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.response {
        #[allow(unused_mut)]
        let mut object_6 = object.key("response").start_object();
        crate::protocol_serde::shape_response_specification::ser_response_specification(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_conditional_branch<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ConditionalBranch>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ConditionalBranchBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "condition" => {
                            builder = builder.set_condition(crate::protocol_serde::shape_condition::de_condition(tokens)?);
                        }
                        "nextStep" => {
                            builder = builder.set_next_step(crate::protocol_serde::shape_dialog_state::de_dialog_state(tokens)?);
                        }
                        "response" => {
                            builder = builder.set_response(crate::protocol_serde::shape_response_specification::de_response_specification(tokens)?);
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
            Ok(Some(crate::serde_util::conditional_branch_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
