// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_organization_overview_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_organization_overview::DescribeOrganizationOverviewInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.account_ids {
        let mut array_2 = object.key("AccountIds").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.from_time {
        object
            .key("FromTime")
            .date_time(var_4, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_5) = &input.organizational_unit_ids {
        let mut array_6 = object.key("OrganizationalUnitIds").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.to_time {
        object
            .key("ToTime")
            .date_time(var_8, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}
