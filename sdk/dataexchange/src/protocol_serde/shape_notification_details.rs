// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_notification_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::NotificationDetails,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.data_update {
        #[allow(unused_mut)]
        let mut object_2 = object.key("DataUpdate").start_object();
        crate::protocol_serde::shape_data_update_request_details::ser_data_update_request_details(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.deprecation {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Deprecation").start_object();
        crate::protocol_serde::shape_deprecation_request_details::ser_deprecation_request_details(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.schema_change {
        #[allow(unused_mut)]
        let mut object_6 = object.key("SchemaChange").start_object();
        crate::protocol_serde::shape_schema_change_request_details::ser_schema_change_request_details(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
