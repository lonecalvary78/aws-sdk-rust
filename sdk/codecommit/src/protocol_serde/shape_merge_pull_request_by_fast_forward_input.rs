// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_merge_pull_request_by_fast_forward_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::merge_pull_request_by_fast_forward::MergePullRequestByFastForwardInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.pull_request_id {
        object.key("pullRequestId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.repository_name {
        object.key("repositoryName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.source_commit_id {
        object.key("sourceCommitId").string(var_3.as_str());
    }
    Ok(())
}
