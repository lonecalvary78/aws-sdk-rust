// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transact_get_item(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TransactGetItem,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.get {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Get").start_object();
        crate::protocol_serde::shape_get::ser_get(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
