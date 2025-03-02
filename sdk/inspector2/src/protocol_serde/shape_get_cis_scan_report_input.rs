// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_cis_scan_report_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_cis_scan_report::GetCisScanReportInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.report_format {
        object.key("reportFormat").string(var_1.as_str());
    }
    if let Some(var_2) = &input.scan_arn {
        object.key("scanArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.target_accounts {
        let mut array_4 = object.key("targetAccounts").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}
