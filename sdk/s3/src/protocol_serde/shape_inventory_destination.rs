// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_inventory_destination(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::InventoryDestination, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InventoryDestination::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("S3BucketDestination") /* S3BucketDestination com.amazonaws.s3#InventoryDestination$S3BucketDestination */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_inventory_s3_bucket_destination::de_inventory_s3_bucket_destination(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_s3_bucket_destination(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::inventory_destination_correct_errors(builder).build())
}

pub fn ser_inventory_destination(
    input: &crate::types::InventoryDestination,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_2) = &input.s3_bucket_destination {
        let inner_writer = scope.start_el("S3BucketDestination");
        crate::protocol_serde::shape_inventory_s3_bucket_destination::ser_inventory_s3_bucket_destination(var_2, inner_writer)?
    }
    scope.finish();
    Ok(())
}
