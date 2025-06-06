// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_vpc_peering_connections_input_input_input(
    input: &crate::operation::describe_vpc_peering_connections::DescribeVpcPeeringConnectionsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeVpcPeeringConnections", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("NextToken");
    if let Some(var_2) = &input.next_token {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("MaxResults");
    if let Some(var_4) = &input.max_results {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DryRun");
    if let Some(var_6) = &input.dry_run {
        scope_5.boolean(*var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("VpcPeeringConnectionId");
    if let Some(var_8) = &input.vpc_peering_connection_ids {
        if !var_8.is_empty() {
            let mut list_10 = scope_7.start_list(true, Some("item"));
            for item_9 in var_8 {
                #[allow(unused_mut)]
                let mut entry_11 = list_10.entry();
                entry_11.string(item_9);
            }
            list_10.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("Filter");
    if let Some(var_13) = &input.filters {
        if !var_13.is_empty() {
            let mut list_15 = scope_12.start_list(true, Some("Filter"));
            for item_14 in var_13 {
                #[allow(unused_mut)]
                let mut entry_16 = list_15.entry();
                crate::protocol_serde::shape_filter::ser_filter(entry_16, item_14)?;
            }
            list_15.finish();
        }
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
