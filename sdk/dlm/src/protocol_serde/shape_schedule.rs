// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_schedule(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Schedule,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.copy_tags {
        object.key("CopyTags").boolean(*var_2);
    }
    if let Some(var_3) = &input.tags_to_add {
        let mut array_4 = object.key("TagsToAdd").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.variable_tags {
        let mut array_8 = object.key("VariableTags").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.create_rule {
        #[allow(unused_mut)]
        let mut object_12 = object.key("CreateRule").start_object();
        crate::protocol_serde::shape_create_rule::ser_create_rule(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.retain_rule {
        #[allow(unused_mut)]
        let mut object_14 = object.key("RetainRule").start_object();
        crate::protocol_serde::shape_retain_rule::ser_retain_rule(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.fast_restore_rule {
        #[allow(unused_mut)]
        let mut object_16 = object.key("FastRestoreRule").start_object();
        crate::protocol_serde::shape_fast_restore_rule::ser_fast_restore_rule(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.cross_region_copy_rules {
        let mut array_18 = object.key("CrossRegionCopyRules").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_cross_region_copy_rule::ser_cross_region_copy_rule(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.share_rules {
        let mut array_22 = object.key("ShareRules").start_array();
        for item_23 in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_24 = array_22.value().start_object();
                crate::protocol_serde::shape_share_rule::ser_share_rule(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if let Some(var_25) = &input.deprecate_rule {
        #[allow(unused_mut)]
        let mut object_26 = object.key("DeprecateRule").start_object();
        crate::protocol_serde::shape_deprecate_rule::ser_deprecate_rule(&mut object_26, var_25)?;
        object_26.finish();
    }
    if let Some(var_27) = &input.archive_rule {
        #[allow(unused_mut)]
        let mut object_28 = object.key("ArchiveRule").start_object();
        crate::protocol_serde::shape_archive_rule::ser_archive_rule(&mut object_28, var_27)?;
        object_28.finish();
    }
    Ok(())
}

pub(crate) fn de_schedule<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::Schedule>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ScheduleBuilder::default();
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
                        "CopyTags" => {
                            builder = builder.set_copy_tags(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "TagsToAdd" => {
                            builder = builder.set_tags_to_add(crate::protocol_serde::shape_tags_to_add_list::de_tags_to_add_list(tokens)?);
                        }
                        "VariableTags" => {
                            builder = builder.set_variable_tags(crate::protocol_serde::shape_variable_tags_list::de_variable_tags_list(tokens)?);
                        }
                        "CreateRule" => {
                            builder = builder.set_create_rule(crate::protocol_serde::shape_create_rule::de_create_rule(tokens)?);
                        }
                        "RetainRule" => {
                            builder = builder.set_retain_rule(crate::protocol_serde::shape_retain_rule::de_retain_rule(tokens)?);
                        }
                        "FastRestoreRule" => {
                            builder = builder.set_fast_restore_rule(crate::protocol_serde::shape_fast_restore_rule::de_fast_restore_rule(tokens)?);
                        }
                        "CrossRegionCopyRules" => {
                            builder = builder.set_cross_region_copy_rules(
                                crate::protocol_serde::shape_cross_region_copy_rules::de_cross_region_copy_rules(tokens)?,
                            );
                        }
                        "ShareRules" => {
                            builder = builder.set_share_rules(crate::protocol_serde::shape_share_rules::de_share_rules(tokens)?);
                        }
                        "DeprecateRule" => {
                            builder = builder.set_deprecate_rule(crate::protocol_serde::shape_deprecate_rule::de_deprecate_rule(tokens)?);
                        }
                        "ArchiveRule" => {
                            builder = builder.set_archive_rule(crate::protocol_serde::shape_archive_rule::de_archive_rule(tokens)?);
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
