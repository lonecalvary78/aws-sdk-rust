// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_typed_link_facet(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TypedLinkFacet,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Name").string(input.name.as_str());
    }
    {
        let mut array_1 = object.key("Attributes").start_array();
        for item_2 in &input.attributes {
            {
                #[allow(unused_mut)]
                let mut object_3 = array_1.value().start_object();
                crate::protocol_serde::shape_typed_link_attribute_definition::ser_typed_link_attribute_definition(&mut object_3, item_2)?;
                object_3.finish();
            }
        }
        array_1.finish();
    }
    {
        let mut array_4 = object.key("IdentityAttributeOrder").start_array();
        for item_5 in &input.identity_attribute_order {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}
