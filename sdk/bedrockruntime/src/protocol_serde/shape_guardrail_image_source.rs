// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_guardrail_image_source(
    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GuardrailImageSource,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::GuardrailImageSource::Bytes(inner) => {
            object_2.key("bytes").string_unchecked(&::aws_smithy_types::base64::encode(inner));
        }
        crate::types::GuardrailImageSource::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "GuardrailImageSource",
            ))
        }
    }
    Ok(())
}
