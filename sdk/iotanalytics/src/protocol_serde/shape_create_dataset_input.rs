// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_dataset_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_dataset::CreateDatasetInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.actions {
        let mut array_2 = object.key("actions").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_dataset_action::ser_dataset_action(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.content_delivery_rules {
        let mut array_6 = object.key("contentDeliveryRules").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_dataset_content_delivery_rule::ser_dataset_content_delivery_rule(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.dataset_name {
        object.key("datasetName").string(var_9.as_str());
    }
    if let Some(var_10) = &input.late_data_rules {
        let mut array_11 = object.key("lateDataRules").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_late_data_rule::ser_late_data_rule(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.retention_period {
        #[allow(unused_mut)]
        let mut object_15 = object.key("retentionPeriod").start_object();
        crate::protocol_serde::shape_retention_period::ser_retention_period(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.tags {
        let mut array_17 = object.key("tags").start_array();
        for item_18 in var_16 {
            {
                #[allow(unused_mut)]
                let mut object_19 = array_17.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_19, item_18)?;
                object_19.finish();
            }
        }
        array_17.finish();
    }
    if let Some(var_20) = &input.triggers {
        let mut array_21 = object.key("triggers").start_array();
        for item_22 in var_20 {
            {
                #[allow(unused_mut)]
                let mut object_23 = array_21.value().start_object();
                crate::protocol_serde::shape_dataset_trigger::ser_dataset_trigger(&mut object_23, item_22)?;
                object_23.finish();
            }
        }
        array_21.finish();
    }
    if let Some(var_24) = &input.versioning_configuration {
        #[allow(unused_mut)]
        let mut object_25 = object.key("versioningConfiguration").start_object();
        crate::protocol_serde::shape_versioning_configuration::ser_versioning_configuration(&mut object_25, var_24)?;
        object_25.finish();
    }
    Ok(())
}
