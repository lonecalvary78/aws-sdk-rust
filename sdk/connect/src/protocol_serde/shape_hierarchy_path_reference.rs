// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_hierarchy_path_reference<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::HierarchyPathReference>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::HierarchyPathReferenceBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "LevelOne" => {
                            builder = builder.set_level_one(
                                crate::protocol_serde::shape_hierarchy_group_summary_reference::de_hierarchy_group_summary_reference(tokens)?,
                            );
                        }
                        "LevelTwo" => {
                            builder = builder.set_level_two(
                                crate::protocol_serde::shape_hierarchy_group_summary_reference::de_hierarchy_group_summary_reference(tokens)?,
                            );
                        }
                        "LevelThree" => {
                            builder = builder.set_level_three(
                                crate::protocol_serde::shape_hierarchy_group_summary_reference::de_hierarchy_group_summary_reference(tokens)?,
                            );
                        }
                        "LevelFour" => {
                            builder = builder.set_level_four(
                                crate::protocol_serde::shape_hierarchy_group_summary_reference::de_hierarchy_group_summary_reference(tokens)?,
                            );
                        }
                        "LevelFive" => {
                            builder = builder.set_level_five(
                                crate::protocol_serde::shape_hierarchy_group_summary_reference::de_hierarchy_group_summary_reference(tokens)?,
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
