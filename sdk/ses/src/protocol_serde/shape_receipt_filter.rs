// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_receipt_filter(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::ReceiptFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Name");
    {
        scope_1.string(&input.name);
    }
    #[allow(unused_mut)]
    let mut scope_2 = writer.prefix("IpFilter");
    if let Some(var_3) = &input.ip_filter {
        crate::protocol_serde::shape_receipt_ip_filter::ser_receipt_ip_filter(scope_2, var_3)?;
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_receipt_filter(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ReceiptFilter, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ReceiptFilter::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Name") /* Name com.amazonaws.ses#ReceiptFilter$Name */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_4);
            }
            ,
            s if s.matches("IpFilter") /* IpFilter com.amazonaws.ses#ReceiptFilter$IpFilter */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_receipt_ip_filter::de_receipt_ip_filter(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ip_filter(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::receipt_filter_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
