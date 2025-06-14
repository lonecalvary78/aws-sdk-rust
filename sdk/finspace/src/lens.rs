// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_kx_changesets_output_output_next_token(
    input: &crate::operation::list_kx_changesets::ListKxChangesetsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_kx_cluster_nodes_output_output_next_token(
    input: &crate::operation::list_kx_cluster_nodes::ListKxClusterNodesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_kx_databases_output_output_next_token(
    input: &crate::operation::list_kx_databases::ListKxDatabasesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_kx_dataviews_output_output_next_token(
    input: &crate::operation::list_kx_dataviews::ListKxDataviewsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_kx_environments_output_output_next_token(
    input: &crate::operation::list_kx_environments::ListKxEnvironmentsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_kx_scaling_groups_output_output_next_token(
    input: &crate::operation::list_kx_scaling_groups::ListKxScalingGroupsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_kx_environments_output_output_environments(
    input: crate::operation::list_kx_environments::ListKxEnvironmentsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::KxEnvironment>> {
    let input = input.environments?;
    ::std::option::Option::Some(input)
}
