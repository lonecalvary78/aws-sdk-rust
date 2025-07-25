// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_library_items_output_output_next_token(
    input: &crate::operation::list_library_items::ListLibraryItemsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_q_apps_output_output_next_token(
    input: &crate::operation::list_q_apps::ListQAppsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_library_items_output_output_library_items(
    input: crate::operation::list_library_items::ListLibraryItemsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::LibraryItemMember>> {
    let input = input.library_items?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_q_apps_output_output_apps(
    input: crate::operation::list_q_apps::ListQAppsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::UserAppItem>> {
    let input = input.apps;
    ::std::option::Option::Some(input)
}
