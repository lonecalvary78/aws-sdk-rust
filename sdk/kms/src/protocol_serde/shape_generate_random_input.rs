// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_generate_random_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::generate_random::GenerateRandomInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.number_of_bytes {
        object.key("NumberOfBytes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.custom_key_store_id {
        object.key("CustomKeyStoreId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.recipient {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Recipient").start_object();
        crate::protocol_serde::shape_recipient_info::ser_recipient_info(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
