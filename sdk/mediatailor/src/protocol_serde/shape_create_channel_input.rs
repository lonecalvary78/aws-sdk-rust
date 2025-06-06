// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_channel_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_channel::CreateChannelInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.audiences {
        let mut array_2 = object.key("Audiences").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.filler_slate {
        #[allow(unused_mut)]
        let mut object_5 = object.key("FillerSlate").start_object();
        crate::protocol_serde::shape_slate_source::ser_slate_source(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.outputs {
        let mut array_7 = object.key("Outputs").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_request_output_item::ser_request_output_item(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.playback_mode {
        object.key("PlaybackMode").string(var_10.as_str());
    }
    if let Some(var_11) = &input.tags {
        #[allow(unused_mut)]
        let mut object_12 = object.key("tags").start_object();
        for (key_13, value_14) in var_11 {
            {
                object_12.key(key_13.as_str()).string(value_14.as_str());
            }
        }
        object_12.finish();
    }
    if let Some(var_15) = &input.tier {
        object.key("Tier").string(var_15.as_str());
    }
    if let Some(var_16) = &input.time_shift_configuration {
        #[allow(unused_mut)]
        let mut object_17 = object.key("TimeShiftConfiguration").start_object();
        crate::protocol_serde::shape_time_shift_configuration::ser_time_shift_configuration(&mut object_17, var_16)?;
        object_17.finish();
    }
    Ok(())
}
