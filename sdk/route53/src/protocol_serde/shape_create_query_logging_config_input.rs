// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_query_logging_config_input_input_input(
    input: &crate::operation::create_query_logging_config::CreateQueryLoggingConfigInput,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.cloud_watch_logs_log_group_arn {
        let mut inner_writer = scope.start_el("CloudWatchLogsLogGroupArn").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.hosted_zone_id {
        let mut inner_writer = scope.start_el("HostedZoneId").finish();
        inner_writer.data(var_2.as_str());
    }
    scope.finish();
    Ok(())
}
