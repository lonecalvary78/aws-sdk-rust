// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_events_input_input_input(
    input: &crate::operation::describe_events::DescribeEventsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeEvents", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ApplicationName");
    if let Some(var_2) = &input.application_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VersionLabel");
    if let Some(var_4) = &input.version_label {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TemplateName");
    if let Some(var_6) = &input.template_name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("EnvironmentId");
    if let Some(var_8) = &input.environment_id {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("EnvironmentName");
    if let Some(var_10) = &input.environment_name {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("PlatformArn");
    if let Some(var_12) = &input.platform_arn {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("RequestId");
    if let Some(var_14) = &input.request_id {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("Severity");
    if let Some(var_16) = &input.severity {
        scope_15.string(var_16.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("StartTime");
    if let Some(var_18) = &input.start_time {
        scope_17.date_time(var_18, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("EndTime");
    if let Some(var_20) = &input.end_time {
        scope_19.date_time(var_20, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("MaxRecords");
    if let Some(var_22) = &input.max_records {
        scope_21.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_22).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("NextToken");
    if let Some(var_24) = &input.next_token {
        scope_23.string(var_24);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
