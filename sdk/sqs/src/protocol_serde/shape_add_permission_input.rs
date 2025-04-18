// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_permission_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::add_permission::AddPermissionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.queue_url {
        object.key("QueueUrl").string(var_1.as_str());
    }
    if let Some(var_2) = &input.label {
        object.key("Label").string(var_2.as_str());
    }
    if let Some(var_3) = &input.aws_account_ids {
        let mut array_4 = object.key("AWSAccountIds").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.actions {
        let mut array_7 = object.key("Actions").start_array();
        for item_8 in var_6 {
            {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    Ok(())
}
