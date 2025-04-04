// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_failed_queued_purchase_deletion(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::FailedQueuedPurchaseDeletion, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::FailedQueuedPurchaseDeletion::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("error") /* Error com.amazonaws.ec2#FailedQueuedPurchaseDeletion$Error */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_delete_queued_reserved_instances_error::de_delete_queued_reserved_instances_error(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_error(var_1);
            }
            ,
            s if s.matches("reservedInstancesId") /* ReservedInstancesId com.amazonaws.ec2#FailedQueuedPurchaseDeletion$ReservedInstancesId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_reserved_instances_id(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
