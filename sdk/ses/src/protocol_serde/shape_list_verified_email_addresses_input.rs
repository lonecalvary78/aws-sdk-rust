// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_verified_email_addresses_input_input_input(
    input: &crate::operation::list_verified_email_addresses::ListVerifiedEmailAddressesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let _ = input;
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ListVerifiedEmailAddresses", "2010-12-01");
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
