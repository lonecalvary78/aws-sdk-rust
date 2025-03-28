// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_metric_data_result(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::MetricDataResult, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::MetricDataResult::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.cloudwatch#MetricDataResult$Id */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_1);
            }
            ,
            s if s.matches("Label") /* Label com.amazonaws.cloudwatch#MetricDataResult$Label */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_label(var_2);
            }
            ,
            s if s.matches("Timestamps") /* Timestamps com.amazonaws.cloudwatch#MetricDataResult$Timestamps */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_timestamps::de_timestamps(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_timestamps(var_3);
            }
            ,
            s if s.matches("Values") /* Values com.amazonaws.cloudwatch#MetricDataResult$Values */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_datapoint_values::de_datapoint_values(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_values(var_4);
            }
            ,
            s if s.matches("StatusCode") /* StatusCode com.amazonaws.cloudwatch#MetricDataResult$StatusCode */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::StatusCode, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::StatusCode::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status_code(var_5);
            }
            ,
            s if s.matches("Messages") /* Messages com.amazonaws.cloudwatch#MetricDataResult$Messages */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_metric_data_result_messages::de_metric_data_result_messages(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_messages(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
