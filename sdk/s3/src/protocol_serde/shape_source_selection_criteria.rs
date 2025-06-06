// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_source_selection_criteria(
    input: &crate::types::SourceSelectionCriteria,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.sse_kms_encrypted_objects {
        let inner_writer = scope.start_el("SseKmsEncryptedObjects");
        crate::protocol_serde::shape_sse_kms_encrypted_objects::ser_sse_kms_encrypted_objects(var_1, inner_writer)?
    }
    if let Some(var_2) = &input.replica_modifications {
        let inner_writer = scope.start_el("ReplicaModifications");
        crate::protocol_serde::shape_replica_modifications::ser_replica_modifications(var_2, inner_writer)?
    }
    scope.finish();
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_source_selection_criteria(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::SourceSelectionCriteria, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::SourceSelectionCriteria::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("SseKmsEncryptedObjects") /* SseKmsEncryptedObjects com.amazonaws.s3#SourceSelectionCriteria$SseKmsEncryptedObjects */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_sse_kms_encrypted_objects::de_sse_kms_encrypted_objects(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_sse_kms_encrypted_objects(var_3);
            }
            ,
            s if s.matches("ReplicaModifications") /* ReplicaModifications com.amazonaws.s3#SourceSelectionCriteria$ReplicaModifications */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_replica_modifications::de_replica_modifications(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_replica_modifications(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
