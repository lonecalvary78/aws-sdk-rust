// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_eks_anywhere_subscription_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.auto_renew {
        object.key("autoRenew").boolean(*var_1);
    }
    if let Some(var_2) = &input.client_request_token {
        object.key("clientRequestToken").string(var_2.as_str());
    }
    Ok(())
}
