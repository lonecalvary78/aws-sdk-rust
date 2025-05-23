// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_reports_for_report_group_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_reports_for_report_group::ListReportsForReportGroupInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.report_group_arn {
        object.key("reportGroupArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.next_token {
        object.key("nextToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.sort_order {
        object.key("sortOrder").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.filter {
        #[allow(unused_mut)]
        let mut object_6 = object.key("filter").start_object();
        crate::protocol_serde::shape_report_filter::ser_report_filter(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
