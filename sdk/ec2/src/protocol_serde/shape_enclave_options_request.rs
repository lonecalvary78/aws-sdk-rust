// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_enclave_options_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::EnclaveOptionsRequest,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Enabled");
    if let Some(var_2) = &input.enabled {
        scope_1.boolean(*var_2);
    }
    Ok(())
}
