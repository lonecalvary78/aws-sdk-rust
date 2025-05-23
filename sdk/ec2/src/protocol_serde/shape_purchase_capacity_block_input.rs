// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_purchase_capacity_block_input_input_input(
    input: &crate::operation::purchase_capacity_block::PurchaseCapacityBlockInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "PurchaseCapacityBlock", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TagSpecification");
    if let Some(var_4) = &input.tag_specifications {
        if !var_4.is_empty() {
            let mut list_6 = scope_3.start_list(true, Some("item"));
            for item_5 in var_4 {
                #[allow(unused_mut)]
                let mut entry_7 = list_6.entry();
                crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_7, item_5)?;
            }
            list_6.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("CapacityBlockOfferingId");
    if let Some(var_9) = &input.capacity_block_offering_id {
        scope_8.string(var_9);
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("InstancePlatform");
    if let Some(var_11) = &input.instance_platform {
        scope_10.string(var_11.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
