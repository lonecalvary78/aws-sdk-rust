// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_watchlist_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_watchlist::UpdateWatchlistInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.domain_id {
        object.key("DomainId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.watchlist_id {
        object.key("WatchlistId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.description {
        object.key("Description").string(var_4.as_str());
    }
    Ok(())
}
