// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_accelerate_configuration(
    input: &crate::types::AccelerateConfiguration,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.status {
        let mut inner_writer = scope.start_el("Status").finish();
        inner_writer.data(var_1.as_str());
    }
    scope.finish();
    Ok(())
}
