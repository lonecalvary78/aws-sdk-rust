// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_resource_snapshot_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_resource_snapshot::CreateResourceSnapshotInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.catalog {
        object.key("Catalog").string(var_1.as_str());
    }
    if let Some(var_2) = &input.engagement_identifier {
        object.key("EngagementIdentifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.resource_type {
        object.key("ResourceType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.resource_identifier {
        object.key("ResourceIdentifier").string(var_4.as_str());
    }
    if let Some(var_5) = &input.resource_snapshot_template_identifier {
        object.key("ResourceSnapshotTemplateIdentifier").string(var_5.as_str());
    }
    if let Some(var_6) = &input.client_token {
        object.key("ClientToken").string(var_6.as_str());
    }
    Ok(())
}
