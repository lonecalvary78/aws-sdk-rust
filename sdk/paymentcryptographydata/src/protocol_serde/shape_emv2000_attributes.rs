// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_emv2000_attributes(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Emv2000Attributes,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("MajorKeyDerivationMode").string(input.major_key_derivation_mode.as_str());
    }
    {
        object.key("PrimaryAccountNumber").string(input.primary_account_number.as_str());
    }
    {
        object.key("PanSequenceNumber").string(input.pan_sequence_number.as_str());
    }
    {
        object
            .key("ApplicationTransactionCounter")
            .string(input.application_transaction_counter.as_str());
    }
    Ok(())
}
