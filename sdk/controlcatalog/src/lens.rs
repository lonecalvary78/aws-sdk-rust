// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_common_controls_output_output_next_token(
    input: &crate::operation::list_common_controls::ListCommonControlsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_control_mappings_output_output_next_token(
    input: &crate::operation::list_control_mappings::ListControlMappingsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_controls_output_output_next_token(
    input: &crate::operation::list_controls::ListControlsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_domains_output_output_next_token(
    input: &crate::operation::list_domains::ListDomainsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_objectives_output_output_next_token(
    input: &crate::operation::list_objectives::ListObjectivesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_common_controls_output_output_common_controls(
    input: crate::operation::list_common_controls::ListCommonControlsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::CommonControlSummary>> {
    let input = input.common_controls;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_control_mappings_output_output_control_mappings(
    input: crate::operation::list_control_mappings::ListControlMappingsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ControlMapping>> {
    let input = input.control_mappings;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_controls_output_output_controls(
    input: crate::operation::list_controls::ListControlsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ControlSummary>> {
    let input = input.controls;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_domains_output_output_domains(
    input: crate::operation::list_domains::ListDomainsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::DomainSummary>> {
    let input = input.domains;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_objectives_output_output_objectives(
    input: crate::operation::list_objectives::ListObjectivesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ObjectiveSummary>> {
    let input = input.objectives;
    ::std::option::Option::Some(input)
}
