// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_approval_rule_template_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_approval_rule_template::GetApprovalRuleTemplateInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.approval_rule_template_name {
        object.key("approvalRuleTemplateName").string(var_1.as_str());
    }
    Ok(())
}
