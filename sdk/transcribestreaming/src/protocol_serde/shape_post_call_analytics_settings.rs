// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_post_call_analytics_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PostCallAnalyticsSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("OutputLocation").string(input.output_location.as_str());
    }
    {
        object.key("DataAccessRoleArn").string(input.data_access_role_arn.as_str());
    }
    if let Some(var_1) = &input.content_redaction_output {
        object.key("ContentRedactionOutput").string(var_1.as_str());
    }
    if let Some(var_2) = &input.output_encryption_kms_key_id {
        object.key("OutputEncryptionKMSKeyId").string(var_2.as_str());
    }
    Ok(())
}
