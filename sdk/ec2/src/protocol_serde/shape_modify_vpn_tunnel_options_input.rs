// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_vpn_tunnel_options_input_input_input(
    input: &crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyVpnTunnelOptions", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("VpnConnectionId");
    if let Some(var_2) = &input.vpn_connection_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VpnTunnelOutsideIpAddress");
    if let Some(var_4) = &input.vpn_tunnel_outside_ip_address {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TunnelOptions");
    if let Some(var_6) = &input.tunnel_options {
        crate::protocol_serde::shape_modify_vpn_tunnel_options_specification::ser_modify_vpn_tunnel_options_specification(scope_5, var_6)?;
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DryRun");
    if let Some(var_8) = &input.dry_run {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("SkipTunnelReplacement");
    if let Some(var_10) = &input.skip_tunnel_replacement {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("PreSharedKeyStorage");
    if let Some(var_12) = &input.pre_shared_key_storage {
        scope_11.string(var_12);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
