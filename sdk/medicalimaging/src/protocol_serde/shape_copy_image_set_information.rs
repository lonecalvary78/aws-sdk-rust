// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_copy_image_set_information(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CopyImageSetInformation,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.source_image_set {
        #[allow(unused_mut)]
        let mut object_2 = object.key("sourceImageSet").start_object();
        crate::protocol_serde::shape_copy_source_image_set_information::ser_copy_source_image_set_information(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.destination_image_set {
        #[allow(unused_mut)]
        let mut object_4 = object.key("destinationImageSet").start_object();
        crate::protocol_serde::shape_copy_destination_image_set::ser_copy_destination_image_set(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
