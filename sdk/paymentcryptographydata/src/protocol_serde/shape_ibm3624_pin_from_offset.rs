// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ibm3624_pin_from_offset(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Ibm3624PinFromOffset,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("DecimalizationTable").string(input.decimalization_table.as_str());
    }
    {
        object
            .key("PinValidationDataPadCharacter")
            .string(input.pin_validation_data_pad_character.as_str());
    }
    {
        object.key("PinValidationData").string(input.pin_validation_data.as_str());
    }
    {
        object.key("PinOffset").string(input.pin_offset.as_str());
    }
    Ok(())
}
