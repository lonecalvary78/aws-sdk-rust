// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_asset_bundle_import_job_override_tags<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AssetBundleImportJobOverrideTags>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AssetBundleImportJobOverrideTagsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "VPCConnections" => {
                            builder = builder.set_vpc_connections(
                                    crate::protocol_serde::shape_asset_bundle_import_job_vpc_connection_override_tags_list::de_asset_bundle_import_job_vpc_connection_override_tags_list(tokens)?
                                );
                        }
                        "DataSources" => {
                            builder = builder.set_data_sources(
                                    crate::protocol_serde::shape_asset_bundle_import_job_data_source_override_tags_list::de_asset_bundle_import_job_data_source_override_tags_list(tokens)?
                                );
                        }
                        "DataSets" => {
                            builder = builder.set_data_sets(
                                    crate::protocol_serde::shape_asset_bundle_import_job_data_set_override_tags_list::de_asset_bundle_import_job_data_set_override_tags_list(tokens)?
                                );
                        }
                        "Themes" => {
                            builder = builder.set_themes(
                                    crate::protocol_serde::shape_asset_bundle_import_job_theme_override_tags_list::de_asset_bundle_import_job_theme_override_tags_list(tokens)?
                                );
                        }
                        "Analyses" => {
                            builder = builder.set_analyses(
                                    crate::protocol_serde::shape_asset_bundle_import_job_analysis_override_tags_list::de_asset_bundle_import_job_analysis_override_tags_list(tokens)?
                                );
                        }
                        "Dashboards" => {
                            builder = builder.set_dashboards(
                                    crate::protocol_serde::shape_asset_bundle_import_job_dashboard_override_tags_list::de_asset_bundle_import_job_dashboard_override_tags_list(tokens)?
                                );
                        }
                        "Folders" => {
                            builder = builder.set_folders(
                                    crate::protocol_serde::shape_asset_bundle_import_job_folder_override_tags_list::de_asset_bundle_import_job_folder_override_tags_list(tokens)?
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

pub fn ser_asset_bundle_import_job_override_tags(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AssetBundleImportJobOverrideTags,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.vpc_connections {
        let mut array_2 = object.key("VPCConnections").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_vpc_connection_override_tags::ser_asset_bundle_import_job_vpc_connection_override_tags(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.data_sources {
        let mut array_6 = object.key("DataSources").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_data_source_override_tags::ser_asset_bundle_import_job_data_source_override_tags(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.data_sets {
        let mut array_10 = object.key("DataSets").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_data_set_override_tags::ser_asset_bundle_import_job_data_set_override_tags(
                    &mut object_12,
                    item_11,
                )?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.themes {
        let mut array_14 = object.key("Themes").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_theme_override_tags::ser_asset_bundle_import_job_theme_override_tags(
                    &mut object_16,
                    item_15,
                )?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.analyses {
        let mut array_18 = object.key("Analyses").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_analysis_override_tags::ser_asset_bundle_import_job_analysis_override_tags(
                    &mut object_20,
                    item_19,
                )?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.dashboards {
        let mut array_22 = object.key("Dashboards").start_array();
        for item_23 in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_24 = array_22.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_dashboard_override_tags::ser_asset_bundle_import_job_dashboard_override_tags(
                    &mut object_24,
                    item_23,
                )?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if let Some(var_25) = &input.folders {
        let mut array_26 = object.key("Folders").start_array();
        for item_27 in var_25 {
            {
                #[allow(unused_mut)]
                let mut object_28 = array_26.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_folder_override_tags::ser_asset_bundle_import_job_folder_override_tags(
                    &mut object_28,
                    item_27,
                )?;
                object_28.finish();
            }
        }
        array_26.finish();
    }
    Ok(())
}
