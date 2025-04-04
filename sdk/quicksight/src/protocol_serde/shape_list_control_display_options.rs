// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_control_display_options(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ListControlDisplayOptions,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.search_options {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SearchOptions").start_object();
        crate::protocol_serde::shape_list_control_search_options::ser_list_control_search_options(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.select_all_options {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SelectAllOptions").start_object();
        crate::protocol_serde::shape_list_control_select_all_options::ser_list_control_select_all_options(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.title_options {
        #[allow(unused_mut)]
        let mut object_6 = object.key("TitleOptions").start_object();
        crate::protocol_serde::shape_label_options::ser_label_options(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.info_icon_label_options {
        #[allow(unused_mut)]
        let mut object_8 = object.key("InfoIconLabelOptions").start_object();
        crate::protocol_serde::shape_sheet_control_info_icon_label_options::ser_sheet_control_info_icon_label_options(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}

pub(crate) fn de_list_control_display_options<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ListControlDisplayOptions>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ListControlDisplayOptionsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "SearchOptions" => {
                            builder = builder.set_search_options(
                                crate::protocol_serde::shape_list_control_search_options::de_list_control_search_options(tokens)?,
                            );
                        }
                        "SelectAllOptions" => {
                            builder = builder.set_select_all_options(
                                crate::protocol_serde::shape_list_control_select_all_options::de_list_control_select_all_options(tokens)?,
                            );
                        }
                        "TitleOptions" => {
                            builder = builder.set_title_options(crate::protocol_serde::shape_label_options::de_label_options(tokens)?);
                        }
                        "InfoIconLabelOptions" => {
                            builder = builder.set_info_icon_label_options(
                                crate::protocol_serde::shape_sheet_control_info_icon_label_options::de_sheet_control_info_icon_label_options(tokens)?,
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
