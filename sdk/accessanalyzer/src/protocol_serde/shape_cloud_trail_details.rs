// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cloud_trail_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CloudTrailDetails,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("trails").start_array();
        for item_2 in &input.trails {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_trail::ser_trail(&mut object_3, item_2)?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    {
        object.key("accessRole").string(input.access_role.as_str());
    }
    {
        object
            .key("startTime")
            .date_time(&input.start_time, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    if let Some(var_4) = &input.end_time {
        object.key("endTime").date_time(var_4, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    Ok(())
}
