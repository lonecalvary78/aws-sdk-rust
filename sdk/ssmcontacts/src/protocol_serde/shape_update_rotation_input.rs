// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_rotation_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_rotation::UpdateRotationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.rotation_id {
        object.key("RotationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.contact_ids {
        let mut array_3 = object.key("ContactIds").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_5, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_6) = &input.time_zone_id {
        object.key("TimeZoneId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.recurrence {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Recurrence").start_object();
        crate::protocol_serde::shape_recurrence_settings::ser_recurrence_settings(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}
