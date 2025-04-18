// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_service_settings_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_service_settings::UpdateServiceSettingsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.s3_bucket_arn {
        object.key("S3BucketArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.sns_topic_arn {
        object.key("SnsTopicArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.organization_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("OrganizationConfiguration").start_object();
        crate::protocol_serde::shape_organization_configuration::ser_organization_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.enable_cross_accounts_discovery {
        object.key("EnableCrossAccountsDiscovery").boolean(*var_5);
    }
    Ok(())
}
