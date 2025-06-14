// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_describe_objects_output_output_marker(
    input: &crate::operation::describe_objects::DescribeObjectsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_pipelines_output_output_marker(
    input: &crate::operation::list_pipelines::ListPipelinesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_query_objects_output_output_marker(
    input: &crate::operation::query_objects::QueryObjectsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_describe_objects_output_output_pipeline_objects(
    input: crate::operation::describe_objects::DescribeObjectsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::PipelineObject>> {
    let input = input.pipeline_objects;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_pipelines_output_output_pipeline_id_list(
    input: crate::operation::list_pipelines::ListPipelinesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::PipelineIdName>> {
    let input = input.pipeline_id_list;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_query_objects_output_output_ids(
    input: crate::operation::query_objects::QueryObjectsOutput,
) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
    let input = input.ids?;
    ::std::option::Option::Some(input)
}
