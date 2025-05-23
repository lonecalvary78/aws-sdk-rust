// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_log_levels_by_resource_types_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.default_log_level {
        object.key("DefaultLogLevel").string(var_1.as_str());
    }
    if let Some(var_2) = &input.fuota_task_log_options {
        let mut array_3 = object.key("FuotaTaskLogOptions").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_fuota_task_log_option::ser_fuota_task_log_option(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.wireless_device_log_options {
        let mut array_7 = object.key("WirelessDeviceLogOptions").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_wireless_device_log_option::ser_wireless_device_log_option(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.wireless_gateway_log_options {
        let mut array_11 = object.key("WirelessGatewayLogOptions").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_wireless_gateway_log_option::ser_wireless_gateway_log_option(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
