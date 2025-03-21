// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_test_grid_project_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_test_grid_project::CreateTestGridProjectInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.vpc_config {
        #[allow(unused_mut)]
        let mut object_4 = object.key("vpcConfig").start_object();
        crate::protocol_serde::shape_test_grid_vpc_config::ser_test_grid_vpc_config(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
