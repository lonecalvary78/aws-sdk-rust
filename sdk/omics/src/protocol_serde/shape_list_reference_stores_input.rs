// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_reference_stores_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_reference_stores::ListReferenceStoresInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.filter {
        #[allow(unused_mut)]
        let mut object_2 = object.key("filter").start_object();
        crate::protocol_serde::shape_reference_store_filter::ser_reference_store_filter(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
