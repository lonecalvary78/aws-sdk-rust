// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_model_package_group_policy_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_model_package_group_policy::DeleteModelPackageGroupPolicyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.model_package_group_name {
        object.key("ModelPackageGroupName").string(var_1.as_str());
    }
    Ok(())
}
