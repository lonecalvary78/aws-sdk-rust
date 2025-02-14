// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_monitor_deployment_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_monitor_deployment::StartMonitorDeploymentInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.dry_run {
        object.key("dryRun").boolean(*var_1);
    }
    if let Some(var_2) = &input.request_id {
        object.key("requestId").string(var_2.as_str());
    }
    Ok(())
}
