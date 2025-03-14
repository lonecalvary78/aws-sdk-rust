// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_buckets_aggregation_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_buckets_aggregation::GetBucketsAggregationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.aggregation_field {
        object.key("aggregationField").string(var_1.as_str());
    }
    if let Some(var_2) = &input.buckets_aggregation_type {
        #[allow(unused_mut)]
        let mut object_3 = object.key("bucketsAggregationType").start_object();
        crate::protocol_serde::shape_buckets_aggregation_type::ser_buckets_aggregation_type(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.index_name {
        object.key("indexName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.query_string {
        object.key("queryString").string(var_5.as_str());
    }
    if let Some(var_6) = &input.query_version {
        object.key("queryVersion").string(var_6.as_str());
    }
    Ok(())
}
