// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_model_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_model::UpdateModelInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.model_name {
        object.key("ModelName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.labels_input_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("LabelsInputConfiguration").start_object();
        crate::protocol_serde::shape_labels_input_configuration::ser_labels_input_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.role_arn {
        object.key("RoleArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.model_diagnostics_output_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("ModelDiagnosticsOutputConfiguration").start_object();
        crate::protocol_serde::shape_model_diagnostics_output_configuration::ser_model_diagnostics_output_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
