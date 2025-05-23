// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_inventory_item(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::InventoryItem,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("TypeName").string(input.type_name.as_str());
    }
    {
        object.key("SchemaVersion").string(input.schema_version.as_str());
    }
    {
        object.key("CaptureTime").string(input.capture_time.as_str());
    }
    if let Some(var_1) = &input.content_hash {
        object.key("ContentHash").string(var_1.as_str());
    }
    if let Some(var_2) = &input.content {
        let mut array_3 = object.key("Content").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                for (key_6, value_7) in item_4 {
                    {
                        object_5.key(key_6.as_str()).string(value_7.as_str());
                    }
                }
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_8) = &input.context {
        #[allow(unused_mut)]
        let mut object_9 = object.key("Context").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}
