// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_asset_bundle_import_job_override_parameters<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AssetBundleImportJobOverrideParameters>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AssetBundleImportJobOverrideParametersBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ResourceIdOverrideConfiguration" => {
                            builder = builder.set_resource_id_override_configuration(
                                    crate::protocol_serde::shape_asset_bundle_import_job_resource_id_override_configuration::de_asset_bundle_import_job_resource_id_override_configuration(tokens)?
                                );
                        }
                        "VPCConnections" => {
                            builder = builder.set_vpc_connections(
                                    crate::protocol_serde::shape_asset_bundle_import_job_vpc_connection_override_parameters_list::de_asset_bundle_import_job_vpc_connection_override_parameters_list(tokens)?
                                );
                        }
                        "RefreshSchedules" => {
                            builder = builder.set_refresh_schedules(
                                    crate::protocol_serde::shape_asset_bundle_import_job_refresh_schedule_override_parameters_list::de_asset_bundle_import_job_refresh_schedule_override_parameters_list(tokens)?
                                );
                        }
                        "DataSources" => {
                            builder = builder.set_data_sources(
                                    crate::protocol_serde::shape_asset_bundle_import_job_data_source_override_parameters_list::de_asset_bundle_import_job_data_source_override_parameters_list(tokens)?
                                );
                        }
                        "DataSets" => {
                            builder = builder.set_data_sets(
                                    crate::protocol_serde::shape_asset_bundle_import_job_data_set_override_parameters_list::de_asset_bundle_import_job_data_set_override_parameters_list(tokens)?
                                );
                        }
                        "Themes" => {
                            builder = builder.set_themes(
                                    crate::protocol_serde::shape_asset_bundle_import_job_theme_override_parameters_list::de_asset_bundle_import_job_theme_override_parameters_list(tokens)?
                                );
                        }
                        "Analyses" => {
                            builder = builder.set_analyses(
                                    crate::protocol_serde::shape_asset_bundle_import_job_analysis_override_parameters_list::de_asset_bundle_import_job_analysis_override_parameters_list(tokens)?
                                );
                        }
                        "Dashboards" => {
                            builder = builder.set_dashboards(
                                    crate::protocol_serde::shape_asset_bundle_import_job_dashboard_override_parameters_list::de_asset_bundle_import_job_dashboard_override_parameters_list(tokens)?
                                );
                        }
                        "Folders" => {
                            builder = builder.set_folders(
                                    crate::protocol_serde::shape_asset_bundle_import_job_folder_override_parameters_list::de_asset_bundle_import_job_folder_override_parameters_list(tokens)?
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

pub fn ser_asset_bundle_import_job_override_parameters(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AssetBundleImportJobOverrideParameters,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.resource_id_override_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ResourceIdOverrideConfiguration").start_object();
        crate::protocol_serde::shape_asset_bundle_import_job_resource_id_override_configuration::ser_asset_bundle_import_job_resource_id_override_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.vpc_connections {
        let mut array_4 = object.key("VPCConnections").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_vpc_connection_override_parameters::ser_asset_bundle_import_job_vpc_connection_override_parameters(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.refresh_schedules {
        let mut array_8 = object.key("RefreshSchedules").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_refresh_schedule_override_parameters::ser_asset_bundle_import_job_refresh_schedule_override_parameters(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.data_sources {
        let mut array_12 = object.key("DataSources").start_array();
        for item_13 in var_11 {
            {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_data_source_override_parameters::ser_asset_bundle_import_job_data_source_override_parameters(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.data_sets {
        let mut array_16 = object.key("DataSets").start_array();
        for item_17 in var_15 {
            {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_data_set_override_parameters::ser_asset_bundle_import_job_data_set_override_parameters(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    if let Some(var_19) = &input.themes {
        let mut array_20 = object.key("Themes").start_array();
        for item_21 in var_19 {
            {
                #[allow(unused_mut)]
                let mut object_22 = array_20.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_theme_override_parameters::ser_asset_bundle_import_job_theme_override_parameters(&mut object_22, item_21)?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    if let Some(var_23) = &input.analyses {
        let mut array_24 = object.key("Analyses").start_array();
        for item_25 in var_23 {
            {
                #[allow(unused_mut)]
                let mut object_26 = array_24.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_analysis_override_parameters::ser_asset_bundle_import_job_analysis_override_parameters(&mut object_26, item_25)?;
                object_26.finish();
            }
        }
        array_24.finish();
    }
    if let Some(var_27) = &input.dashboards {
        let mut array_28 = object.key("Dashboards").start_array();
        for item_29 in var_27 {
            {
                #[allow(unused_mut)]
                let mut object_30 = array_28.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_dashboard_override_parameters::ser_asset_bundle_import_job_dashboard_override_parameters(&mut object_30, item_29)?;
                object_30.finish();
            }
        }
        array_28.finish();
    }
    if let Some(var_31) = &input.folders {
        let mut array_32 = object.key("Folders").start_array();
        for item_33 in var_31 {
            {
                #[allow(unused_mut)]
                let mut object_34 = array_32.value().start_object();
                crate::protocol_serde::shape_asset_bundle_import_job_folder_override_parameters::ser_asset_bundle_import_job_folder_override_parameters(&mut object_34, item_33)?;
                object_34.finish();
            }
        }
        array_32.finish();
    }
    Ok(())
}
