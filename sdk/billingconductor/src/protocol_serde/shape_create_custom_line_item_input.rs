// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_custom_line_item_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_custom_line_item::CreateCustomLineItemInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("AccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.billing_group_arn {
        object.key("BillingGroupArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.billing_period_range {
        #[allow(unused_mut)]
        let mut object_4 = object.key("BillingPeriodRange").start_object();
        crate::protocol_serde::shape_custom_line_item_billing_period_range::ser_custom_line_item_billing_period_range(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.charge_details {
        #[allow(unused_mut)]
        let mut object_6 = object.key("ChargeDetails").start_object();
        crate::protocol_serde::shape_custom_line_item_charge_details::ser_custom_line_item_charge_details(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.description {
        object.key("Description").string(var_7.as_str());
    }
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    Ok(())
}
