// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_route_server_endpoint_input_input_input(
    input: &crate::operation::create_route_server_endpoint::CreateRouteServerEndpointInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateRouteServerEndpoint", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("RouteServerId");
    if let Some(var_2) = &input.route_server_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SubnetId");
    if let Some(var_4) = &input.subnet_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ClientToken");
    if let Some(var_6) = &input.client_token {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DryRun");
    if let Some(var_8) = &input.dry_run {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("TagSpecification");
    if let Some(var_10) = &input.tag_specifications {
        if !var_10.is_empty() {
            let mut list_12 = scope_9.start_list(true, Some("item"));
            for item_11 in var_10 {
                #[allow(unused_mut)]
                let mut entry_13 = list_12.entry();
                crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_13, item_11)?;
            }
            list_12.finish();
        }
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
