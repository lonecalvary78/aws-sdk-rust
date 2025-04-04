// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_action_execution_output<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ActionExecutionOutput>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ActionExecutionOutputBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "outputArtifacts" => {
                            builder =
                                builder.set_output_artifacts(crate::protocol_serde::shape_artifact_detail_list::de_artifact_detail_list(tokens)?);
                        }
                        "executionResult" => {
                            builder = builder
                                .set_execution_result(crate::protocol_serde::shape_action_execution_result::de_action_execution_result(tokens)?);
                        }
                        "outputVariables" => {
                            builder =
                                builder.set_output_variables(crate::protocol_serde::shape_output_variables_map::de_output_variables_map(tokens)?);
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
