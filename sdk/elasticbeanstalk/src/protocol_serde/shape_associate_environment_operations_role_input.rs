// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_environment_operations_role_input_input_input(
    input: &crate::operation::associate_environment_operations_role::AssociateEnvironmentOperationsRoleInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssociateEnvironmentOperationsRole", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EnvironmentName");
    if let Some(var_2) = &input.environment_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("OperationsRole");
    if let Some(var_4) = &input.operations_role {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
