// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_queries_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::QueriesConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("Queries").start_array();
        for item_2 in &input.queries {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_query::ser_query(&mut object_3, item_2)?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    Ok(())
}
