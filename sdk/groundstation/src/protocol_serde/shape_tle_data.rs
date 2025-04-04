// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tle_data(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TleData,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("tleLine1").string(input.tle_line1.as_str());
    }
    {
        object.key("tleLine2").string(input.tle_line2.as_str());
    }
    if let Some(var_1) = &input.valid_time_range {
        #[allow(unused_mut)]
        let mut object_2 = object.key("validTimeRange").start_object();
        crate::protocol_serde::shape_time_range::ser_time_range(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
