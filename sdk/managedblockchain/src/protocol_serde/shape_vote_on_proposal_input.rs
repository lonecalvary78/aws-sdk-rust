// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_vote_on_proposal_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::vote_on_proposal::VoteOnProposalInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.vote {
        object.key("Vote").string(var_1.as_str());
    }
    if let Some(var_2) = &input.voter_member_id {
        object.key("VoterMemberId").string(var_2.as_str());
    }
    Ok(())
}
