// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_group_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_group::PutGroupInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.data_source_id {
        object.key("dataSourceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.group_members {
        #[allow(unused_mut)]
        let mut object_3 = object.key("groupMembers").start_object();
        crate::protocol_serde::shape_group_members::ser_group_members(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.group_name {
        object.key("groupName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.role_arn {
        object.key("roleArn").string(var_5.as_str());
    }
    if let Some(var_6) = &input.r#type {
        object.key("type").string(var_6.as_str());
    }
    Ok(())
}
