// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_load_balancers_config(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::LoadBalancersConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClassicLoadBalancersConfig");
    if let Some(var_2) = &input.classic_load_balancers_config {
        crate::protocol_serde::shape_classic_load_balancers_config::ser_classic_load_balancers_config(scope_1, var_2)?;
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TargetGroupsConfig");
    if let Some(var_4) = &input.target_groups_config {
        crate::protocol_serde::shape_target_groups_config::ser_target_groups_config(scope_3, var_4)?;
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_load_balancers_config(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::LoadBalancersConfig, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LoadBalancersConfig::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("classicLoadBalancersConfig") /* ClassicLoadBalancersConfig com.amazonaws.ec2#LoadBalancersConfig$ClassicLoadBalancersConfig */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_classic_load_balancers_config::de_classic_load_balancers_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_classic_load_balancers_config(var_5);
            }
            ,
            s if s.matches("targetGroupsConfig") /* TargetGroupsConfig com.amazonaws.ec2#LoadBalancersConfig$TargetGroupsConfig */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_target_groups_config::de_target_groups_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_target_groups_config(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
