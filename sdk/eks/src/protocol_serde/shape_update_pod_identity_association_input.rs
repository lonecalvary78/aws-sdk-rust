// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_pod_identity_association_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_pod_identity_association::UpdatePodIdentityAssociationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("clientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.disable_session_tags {
        object.key("disableSessionTags").boolean(*var_2);
    }
    if let Some(var_3) = &input.role_arn {
        object.key("roleArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.target_role_arn {
        object.key("targetRoleArn").string(var_4.as_str());
    }
    Ok(())
}
