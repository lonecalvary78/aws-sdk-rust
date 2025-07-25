// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_inventory_table_configuration(
    input: &crate::types::InventoryTableConfiguration,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    {
        let mut inner_writer = scope.start_el("ConfigurationState").finish();
        inner_writer.data(input.configuration_state.as_str());
    }
    if let Some(var_1) = &input.encryption_configuration {
        let inner_writer = scope.start_el("EncryptionConfiguration");
        crate::protocol_serde::shape_metadata_table_encryption_configuration::ser_metadata_table_encryption_configuration(var_1, inner_writer)?
    }
    scope.finish();
    Ok(())
}
