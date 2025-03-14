// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_api_schema(
    object_6: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ApiSchema,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::ApiSchema::S3(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_6.key("s3").start_object();
            crate::protocol_serde::shape_s3_identifier::ser_s3_identifier(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::ApiSchema::Payload(inner) => {
            object_6.key("payload").string(inner.as_str());
        }
        crate::types::ApiSchema::Unknown => return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant("ApiSchema")),
    }
    Ok(())
}
