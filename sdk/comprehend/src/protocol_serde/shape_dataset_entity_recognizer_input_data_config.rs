// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_dataset_entity_recognizer_input_data_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DatasetEntityRecognizerInputDataConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.annotations {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Annotations").start_object();
        crate::protocol_serde::shape_dataset_entity_recognizer_annotations::ser_dataset_entity_recognizer_annotations(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.documents {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Documents").start_object();
        crate::protocol_serde::shape_dataset_entity_recognizer_documents::ser_dataset_entity_recognizer_documents(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.entity_list {
        #[allow(unused_mut)]
        let mut object_6 = object.key("EntityList").start_object();
        crate::protocol_serde::shape_dataset_entity_recognizer_entity_list::ser_dataset_entity_recognizer_entity_list(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
