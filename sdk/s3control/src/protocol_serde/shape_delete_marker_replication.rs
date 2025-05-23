// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_marker_replication(
    input: &crate::types::DeleteMarkerReplication,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    {
        let mut inner_writer = scope.start_el("Status").finish();
        inner_writer.data(input.status.as_str());
    }
    scope.finish();
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_delete_marker_replication(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::DeleteMarkerReplication, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DeleteMarkerReplication::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Status") /* Status com.amazonaws.s3control#DeleteMarkerReplication$Status */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::DeleteMarkerReplicationStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DeleteMarkerReplicationStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::delete_marker_replication_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
