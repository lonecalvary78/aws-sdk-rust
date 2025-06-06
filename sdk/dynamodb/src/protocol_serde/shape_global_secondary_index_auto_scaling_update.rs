// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_global_secondary_index_auto_scaling_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GlobalSecondaryIndexAutoScalingUpdate,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.index_name {
        object.key("IndexName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.provisioned_write_capacity_auto_scaling_update {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ProvisionedWriteCapacityAutoScalingUpdate").start_object();
        crate::protocol_serde::shape_auto_scaling_settings_update::ser_auto_scaling_settings_update(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}
