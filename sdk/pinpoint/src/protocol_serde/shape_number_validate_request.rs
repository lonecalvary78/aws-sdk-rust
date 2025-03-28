// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_number_validate_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::NumberValidateRequest,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.iso_country_code {
        object.key("IsoCountryCode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.phone_number {
        object.key("PhoneNumber").string(var_2.as_str());
    }
    Ok(())
}
