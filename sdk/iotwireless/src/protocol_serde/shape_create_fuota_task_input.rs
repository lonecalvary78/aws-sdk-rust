// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_fuota_task_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_fuota_task::CreateFuotaTaskInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.descriptor {
        object.key("Descriptor").string(var_3.as_str());
    }
    if let Some(var_4) = &input.firmware_update_image {
        object.key("FirmwareUpdateImage").string(var_4.as_str());
    }
    if let Some(var_5) = &input.firmware_update_role {
        object.key("FirmwareUpdateRole").string(var_5.as_str());
    }
    if let Some(var_6) = &input.fragment_interval_ms {
        object.key("FragmentIntervalMS").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.fragment_size_bytes {
        object.key("FragmentSizeBytes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.lo_ra_wan {
        #[allow(unused_mut)]
        let mut object_9 = object.key("LoRaWAN").start_object();
        crate::protocol_serde::shape_lo_ra_wan_fuota_task::ser_lo_ra_wan_fuota_task(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.name {
        object.key("Name").string(var_10.as_str());
    }
    if let Some(var_11) = &input.redundancy_percent {
        object.key("RedundancyPercent").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.tags {
        let mut array_13 = object.key("Tags").start_array();
        for item_14 in var_12 {
            {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    Ok(())
}
