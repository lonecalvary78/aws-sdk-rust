// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_warm_pool_configuration(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::WarmPoolConfiguration, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::WarmPoolConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("MaxGroupPreparedCapacity") /* MaxGroupPreparedCapacity com.amazonaws.autoscaling#WarmPoolConfiguration$MaxGroupPreparedCapacity */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#MaxGroupPreparedCapacity`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_group_prepared_capacity(var_1);
            }
            ,
            s if s.matches("MinSize") /* MinSize com.amazonaws.autoscaling#WarmPoolConfiguration$MinSize */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.autoscaling#WarmPoolMinSize`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_min_size(var_2);
            }
            ,
            s if s.matches("PoolState") /* PoolState com.amazonaws.autoscaling#WarmPoolConfiguration$PoolState */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::WarmPoolState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::WarmPoolState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_pool_state(var_3);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.autoscaling#WarmPoolConfiguration$Status */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::WarmPoolStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::WarmPoolStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_4);
            }
            ,
            s if s.matches("InstanceReusePolicy") /* InstanceReusePolicy com.amazonaws.autoscaling#WarmPoolConfiguration$InstanceReusePolicy */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_instance_reuse_policy::de_instance_reuse_policy(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_reuse_policy(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
