// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_message_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MessageInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.value {
        #[allow(unused_mut)]
        let mut object_2 = object.key("value").start_object();
        crate::protocol_serde::shape_message_data::ser_message_data(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
