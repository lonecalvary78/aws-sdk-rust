// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_catalog_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_catalog::CreateCatalogInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.catalog_input {
        #[allow(unused_mut)]
        let mut object_3 = object.key("CatalogInput").start_object();
        crate::protocol_serde::shape_catalog_input::ser_catalog_input(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.tags {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Tags").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    Ok(())
}
