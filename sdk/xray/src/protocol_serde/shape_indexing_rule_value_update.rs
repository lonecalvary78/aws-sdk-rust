// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_indexing_rule_value_update(
    object_3: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::IndexingRuleValueUpdate,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::IndexingRuleValueUpdate::Probabilistic(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_3.key("Probabilistic").start_object();
            crate::protocol_serde::shape_probabilistic_rule_value_update::ser_probabilistic_rule_value_update(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::IndexingRuleValueUpdate::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "IndexingRuleValueUpdate",
            ))
        }
    }
    Ok(())
}
