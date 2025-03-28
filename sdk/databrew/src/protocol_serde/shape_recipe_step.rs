// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_recipe_step(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RecipeStep,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.action {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Action").start_object();
        crate::protocol_serde::shape_recipe_action::ser_recipe_action(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.condition_expressions {
        let mut array_4 = object.key("ConditionExpressions").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_condition_expression::ser_condition_expression(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}

pub(crate) fn de_recipe_step<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::RecipeStep>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RecipeStepBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Action" => {
                            builder = builder.set_action(crate::protocol_serde::shape_recipe_action::de_recipe_action(tokens)?);
                        }
                        "ConditionExpressions" => {
                            builder = builder.set_condition_expressions(
                                crate::protocol_serde::shape_condition_expression_list::de_condition_expression_list(tokens)?,
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
            Ok(Some(crate::serde_util::recipe_step_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
