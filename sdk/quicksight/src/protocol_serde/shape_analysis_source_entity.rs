// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_analysis_source_entity(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AnalysisSourceEntity,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.source_template {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SourceTemplate").start_object();
        crate::protocol_serde::shape_analysis_source_template::ser_analysis_source_template(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
