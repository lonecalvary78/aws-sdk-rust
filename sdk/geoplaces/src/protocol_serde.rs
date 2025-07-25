// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_autocomplete;

pub(crate) mod shape_geocode;

pub(crate) mod shape_get_place;

pub(crate) mod shape_reverse_geocode;

pub(crate) mod shape_search_nearby;

pub(crate) mod shape_search_text;

pub(crate) mod shape_suggest;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_autocomplete_input;

pub(crate) mod shape_autocomplete_output;

pub(crate) mod shape_geocode_input;

pub(crate) mod shape_geocode_output;

pub(crate) mod shape_get_place_output;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_reverse_geocode_input;

pub(crate) mod shape_reverse_geocode_output;

pub(crate) mod shape_search_nearby_input;

pub(crate) mod shape_search_nearby_output;

pub(crate) mod shape_search_text_input;

pub(crate) mod shape_search_text_output;

pub(crate) mod shape_suggest_input;

pub(crate) mod shape_suggest_output;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_access_point_list;

pub(crate) mod shape_access_restriction_list;

pub(crate) mod shape_address;

pub(crate) mod shape_autocomplete_filter;

pub(crate) mod shape_autocomplete_result_item_list;

pub(crate) mod shape_bounding_box;

pub(crate) mod shape_business_chain_list;

pub(crate) mod shape_category_list;

pub(crate) mod shape_contacts;

pub(crate) mod shape_food_type_list;

pub(crate) mod shape_geocode_filter;

pub(crate) mod shape_geocode_query_components;

pub(crate) mod shape_geocode_result_item_list;

pub(crate) mod shape_opening_hours_list;

pub(crate) mod shape_phoneme_details;

pub(crate) mod shape_position;

pub(crate) mod shape_postal_code_details_list;

pub(crate) mod shape_query_refinement_list;

pub(crate) mod shape_related_place;

pub(crate) mod shape_related_place_list;

pub(crate) mod shape_reverse_geocode_filter;

pub(crate) mod shape_reverse_geocode_result_item_list;

pub(crate) mod shape_search_nearby_filter;

pub(crate) mod shape_search_nearby_result_item_list;

pub(crate) mod shape_search_text_filter;

pub(crate) mod shape_search_text_result_item_list;

pub(crate) mod shape_suggest_filter;

pub(crate) mod shape_suggest_result_item_list;

pub(crate) mod shape_time_zone;

pub(crate) mod shape_validation_exception_field_list;

pub(crate) mod shape_access_point;

pub(crate) mod shape_access_restriction;

pub(crate) mod shape_address_component_phonemes;

pub(crate) mod shape_autocomplete_result_item;

pub(crate) mod shape_business_chain;

pub(crate) mod shape_category;

pub(crate) mod shape_contact_details_list;

pub(crate) mod shape_country;

pub(crate) mod shape_filter_circle;

pub(crate) mod shape_food_type;

pub(crate) mod shape_geocode_result_item;

pub(crate) mod shape_intersection_street_list;

pub(crate) mod shape_opening_hours;

pub(crate) mod shape_phoneme_transcription_list;

pub(crate) mod shape_postal_code_details;

pub(crate) mod shape_query_refinement;

pub(crate) mod shape_region;

pub(crate) mod shape_reverse_geocode_result_item;

pub(crate) mod shape_search_nearby_result_item;

pub(crate) mod shape_search_text_result_item;

pub(crate) mod shape_secondary_address_component_list;

pub(crate) mod shape_street_components_list;

pub(crate) mod shape_sub_region;

pub(crate) mod shape_suggest_result_item;

pub(crate) mod shape_validation_exception_field;

pub(crate) mod shape_autocomplete_highlights;

pub(crate) mod shape_contact_details;

pub(crate) mod shape_geocode_parsed_query;

pub(crate) mod shape_intersection_list;

pub(crate) mod shape_match_score_details;

pub(crate) mod shape_opening_hours_components_list;

pub(crate) mod shape_opening_hours_display_list;

pub(crate) mod shape_phoneme_transcription;

pub(crate) mod shape_secondary_address_component;

pub(crate) mod shape_street_components;

pub(crate) mod shape_suggest_highlights;

pub(crate) mod shape_suggest_place_result;

pub(crate) mod shape_suggest_query_result;

pub(crate) mod shape_usps_zip;

pub(crate) mod shape_usps_zip_plus4;

pub(crate) mod shape_autocomplete_address_highlights;

pub(crate) mod shape_component_match_scores;

pub(crate) mod shape_geocode_parsed_query_address_components;

pub(crate) mod shape_highlight_list;

pub(crate) mod shape_intersection;

pub(crate) mod shape_opening_hours_components;

pub(crate) mod shape_parsed_query_component_list;

pub(crate) mod shape_suggest_address_highlights;

pub(crate) mod shape_address_component_match_scores;

pub(crate) mod shape_country_highlights;

pub(crate) mod shape_highlight;

pub(crate) mod shape_intersection_highlights_list;

pub(crate) mod shape_parsed_query_component;

pub(crate) mod shape_parsed_query_secondary_address_component_list;

pub(crate) mod shape_region_highlights;

pub(crate) mod shape_sub_region_highlights;

pub(crate) mod shape_match_score_list;

pub(crate) mod shape_parsed_query_secondary_address_component;

pub(crate) mod shape_secondary_address_component_match_score_list;

pub(crate) mod shape_secondary_address_component_match_score;
