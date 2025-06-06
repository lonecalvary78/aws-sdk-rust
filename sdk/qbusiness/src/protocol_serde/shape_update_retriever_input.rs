// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_retriever_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_retriever::UpdateRetrieverInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("configuration").start_object();
        crate::protocol_serde::shape_retriever_configuration::ser_retriever_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.display_name {
        object.key("displayName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.role_arn {
        object.key("roleArn").string(var_4.as_str());
    }
    Ok(())
}
