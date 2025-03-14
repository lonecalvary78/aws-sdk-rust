// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_vpc_endpoint_service_configuration_input_input_input(
    input: &crate::operation::modify_vpc_endpoint_service_configuration::ModifyVpcEndpointServiceConfigurationInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyVpcEndpointServiceConfiguration", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ServiceId");
    if let Some(var_4) = &input.service_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PrivateDnsName");
    if let Some(var_6) = &input.private_dns_name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("RemovePrivateDnsName");
    if let Some(var_8) = &input.remove_private_dns_name {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("AcceptanceRequired");
    if let Some(var_10) = &input.acceptance_required {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("AddNetworkLoadBalancerArn");
    if let Some(var_12) = &input.add_network_load_balancer_arns {
        if !var_12.is_empty() {
            let mut list_14 = scope_11.start_list(true, Some("item"));
            for item_13 in var_12 {
                #[allow(unused_mut)]
                let mut entry_15 = list_14.entry();
                entry_15.string(item_13);
            }
            list_14.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("RemoveNetworkLoadBalancerArn");
    if let Some(var_17) = &input.remove_network_load_balancer_arns {
        if !var_17.is_empty() {
            let mut list_19 = scope_16.start_list(true, Some("item"));
            for item_18 in var_17 {
                #[allow(unused_mut)]
                let mut entry_20 = list_19.entry();
                entry_20.string(item_18);
            }
            list_19.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("AddGatewayLoadBalancerArn");
    if let Some(var_22) = &input.add_gateway_load_balancer_arns {
        if !var_22.is_empty() {
            let mut list_24 = scope_21.start_list(true, Some("item"));
            for item_23 in var_22 {
                #[allow(unused_mut)]
                let mut entry_25 = list_24.entry();
                entry_25.string(item_23);
            }
            list_24.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("RemoveGatewayLoadBalancerArn");
    if let Some(var_27) = &input.remove_gateway_load_balancer_arns {
        if !var_27.is_empty() {
            let mut list_29 = scope_26.start_list(true, Some("item"));
            for item_28 in var_27 {
                #[allow(unused_mut)]
                let mut entry_30 = list_29.entry();
                entry_30.string(item_28);
            }
            list_29.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("AddSupportedIpAddressType");
    if let Some(var_32) = &input.add_supported_ip_address_types {
        if !var_32.is_empty() {
            let mut list_34 = scope_31.start_list(true, Some("item"));
            for item_33 in var_32 {
                #[allow(unused_mut)]
                let mut entry_35 = list_34.entry();
                entry_35.string(item_33);
            }
            list_34.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_36 = writer.prefix("RemoveSupportedIpAddressType");
    if let Some(var_37) = &input.remove_supported_ip_address_types {
        if !var_37.is_empty() {
            let mut list_39 = scope_36.start_list(true, Some("item"));
            for item_38 in var_37 {
                #[allow(unused_mut)]
                let mut entry_40 = list_39.entry();
                entry_40.string(item_38);
            }
            list_39.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("AddSupportedRegion");
    if let Some(var_42) = &input.add_supported_regions {
        if !var_42.is_empty() {
            let mut list_44 = scope_41.start_list(true, Some("item"));
            for item_43 in var_42 {
                #[allow(unused_mut)]
                let mut entry_45 = list_44.entry();
                entry_45.string(item_43);
            }
            list_44.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_46 = writer.prefix("RemoveSupportedRegion");
    if let Some(var_47) = &input.remove_supported_regions {
        if !var_47.is_empty() {
            let mut list_49 = scope_46.start_list(true, Some("item"));
            for item_48 in var_47 {
                #[allow(unused_mut)]
                let mut entry_50 = list_49.entry();
                entry_50.string(item_48);
            }
            list_49.finish();
        }
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
