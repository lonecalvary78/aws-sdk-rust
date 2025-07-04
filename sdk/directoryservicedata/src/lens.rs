// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_group_members_output_output_next_token(
    input: &crate::operation::list_group_members::ListGroupMembersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_groups_output_output_next_token(
    input: &crate::operation::list_groups::ListGroupsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_groups_for_member_output_output_next_token(
    input: &crate::operation::list_groups_for_member::ListGroupsForMemberOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_users_output_output_next_token(
    input: &crate::operation::list_users::ListUsersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_search_groups_output_output_next_token(
    input: &crate::operation::search_groups::SearchGroupsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_search_users_output_output_next_token(
    input: &crate::operation::search_users::SearchUsersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_group_members_output_output_members(
    input: crate::operation::list_group_members::ListGroupMembersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Member>> {
    let input = input.members?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_groups_output_output_groups(
    input: crate::operation::list_groups::ListGroupsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::GroupSummary>> {
    let input = input.groups?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_groups_for_member_output_output_groups(
    input: crate::operation::list_groups_for_member::ListGroupsForMemberOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::GroupSummary>> {
    let input = input.groups?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_users_output_output_users(
    input: crate::operation::list_users::ListUsersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::UserSummary>> {
    let input = input.users?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_search_groups_output_output_groups(
    input: crate::operation::search_groups::SearchGroupsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Group>> {
    let input = input.groups?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_search_users_output_output_users(
    input: crate::operation::search_users::SearchUsersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::User>> {
    let input = input.users?;
    ::std::option::Option::Some(input)
}
