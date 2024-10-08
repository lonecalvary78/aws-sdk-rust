// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_clusters_output_output_next_token(
    input: &crate::operation::list_clusters::ListClustersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_compute_node_groups_output_output_next_token(
    input: &crate::operation::list_compute_node_groups::ListComputeNodeGroupsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_queues_output_output_next_token(
    input: &crate::operation::list_queues::ListQueuesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_clusters_output_output_clusters(
    input: crate::operation::list_clusters::ListClustersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ClusterSummary>> {
    let input = input.clusters;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_compute_node_groups_output_output_compute_node_groups(
    input: crate::operation::list_compute_node_groups::ListComputeNodeGroupsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ComputeNodeGroupSummary>> {
    let input = input.compute_node_groups;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_queues_output_output_queues(
    input: crate::operation::list_queues::ListQueuesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::QueueSummary>> {
    let input = input.queues;
    ::std::option::Option::Some(input)
}
