// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_filter_criterion(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FilterCriterion,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.criterion_key {
        object.key("criterionKey").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filter_condition {
        #[allow(unused_mut)]
        let mut object_3 = object.key("filterCondition").start_object();
        crate::protocol_serde::shape_filter_condition::ser_filter_condition(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}
