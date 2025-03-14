// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_anycast_ip_list_input_input_input(
    input: &crate::operation::create_anycast_ip_list::CreateAnycastIpListInput,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.ip_count {
        let mut inner_writer = scope.start_el("IpCount").finish();
        inner_writer.data(::aws_smithy_types::primitive::Encoder::from(*var_1).encode());
    }
    if let Some(var_2) = &input.name {
        let mut inner_writer = scope.start_el("Name").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.tags {
        let inner_writer = scope.start_el("Tags");
        crate::protocol_serde::shape_tags::ser_tags(var_3, inner_writer)?
    }
    scope.finish();
    Ok(())
}
