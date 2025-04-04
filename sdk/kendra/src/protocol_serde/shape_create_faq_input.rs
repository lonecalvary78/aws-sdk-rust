// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_faq_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_faq::CreateFaqInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.index_id {
        object.key("IndexId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.s3_path {
        #[allow(unused_mut)]
        let mut object_5 = object.key("S3Path").start_object();
        crate::protocol_serde::shape_s3_path::ser_s3_path(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.role_arn {
        object.key("RoleArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        let mut array_8 = object.key("Tags").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.file_format {
        object.key("FileFormat").string(var_11.as_str());
    }
    if let Some(var_12) = &input.client_token {
        object.key("ClientToken").string(var_12.as_str());
    }
    if let Some(var_13) = &input.language_code {
        object.key("LanguageCode").string(var_13.as_str());
    }
    Ok(())
}
