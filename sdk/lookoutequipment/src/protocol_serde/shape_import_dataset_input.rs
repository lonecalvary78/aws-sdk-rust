// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_dataset_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::import_dataset::ImportDatasetInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.source_dataset_arn {
        object.key("SourceDatasetArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.dataset_name {
        object.key("DatasetName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.client_token {
        object.key("ClientToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.server_side_kms_key_id {
        object.key("ServerSideKmsKeyId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}
