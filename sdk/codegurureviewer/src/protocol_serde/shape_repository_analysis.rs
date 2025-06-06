// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_repository_analysis(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RepositoryAnalysis,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.repository_head {
        #[allow(unused_mut)]
        let mut object_2 = object.key("RepositoryHead").start_object();
        crate::protocol_serde::shape_repository_head_source_code_type::ser_repository_head_source_code_type(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.source_code_type {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SourceCodeType").start_object();
        crate::protocol_serde::shape_source_code_type::ser_source_code_type(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
