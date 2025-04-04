// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_cluster_parameter_group_input_input_input(
    input: &crate::operation::modify_cluster_parameter_group::ModifyClusterParameterGroupInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyClusterParameterGroup", "2012-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ParameterGroupName");
    if let Some(var_2) = &input.parameter_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Parameters");
    if let Some(var_4) = &input.parameters {
        let mut list_6 = scope_3.start_list(false, Some("Parameter"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            crate::protocol_serde::shape_parameter::ser_parameter(entry_7, item_5)?;
        }
        list_6.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
