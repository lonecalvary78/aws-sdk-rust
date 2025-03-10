// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_target_group_tuple(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::TargetGroupTuple,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TargetGroupArn");
    if let Some(var_2) = &input.target_group_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Weight");
    if let Some(var_4) = &input.weight {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_target_group_tuple(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::TargetGroupTuple, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TargetGroupTuple::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TargetGroupArn") /* TargetGroupArn com.amazonaws.elasticloadbalancingv2#TargetGroupTuple$TargetGroupArn */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_target_group_arn(var_5);
            }
            ,
            s if s.matches("Weight") /* Weight com.amazonaws.elasticloadbalancingv2#TargetGroupTuple$Weight */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.elasticloadbalancingv2#TargetGroupWeight`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_weight(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
