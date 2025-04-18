// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_merge_branches_by_fast_forward_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::merge_branches_by_fast_forward::MergeBranchesByFastForwardInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.repository_name {
        object.key("repositoryName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.source_commit_specifier {
        object.key("sourceCommitSpecifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.destination_commit_specifier {
        object.key("destinationCommitSpecifier").string(var_3.as_str());
    }
    if let Some(var_4) = &input.target_branch {
        object.key("targetBranch").string(var_4.as_str());
    }
    Ok(())
}
