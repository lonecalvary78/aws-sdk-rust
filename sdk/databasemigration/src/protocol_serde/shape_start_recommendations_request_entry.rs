// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_recommendations_request_entry(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::StartRecommendationsRequestEntry,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("DatabaseId").string(input.database_id.as_str());
    }
    if let Some(var_1) = &input.settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Settings").start_object();
        crate::protocol_serde::shape_recommendation_settings::ser_recommendation_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
