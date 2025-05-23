// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_lambda_action(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::LambdaAction,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TopicArn");
    if let Some(var_2) = &input.topic_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("FunctionArn");
    {
        scope_3.string(&input.function_arn);
    }
    #[allow(unused_mut)]
    let mut scope_4 = writer.prefix("InvocationType");
    if let Some(var_5) = &input.invocation_type {
        scope_4.string(var_5.as_str());
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_lambda_action(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::LambdaAction, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LambdaAction::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TopicArn") /* TopicArn com.amazonaws.ses#LambdaAction$TopicArn */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_topic_arn(var_6);
            }
            ,
            s if s.matches("FunctionArn") /* FunctionArn com.amazonaws.ses#LambdaAction$FunctionArn */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_function_arn(var_7);
            }
            ,
            s if s.matches("InvocationType") /* InvocationType com.amazonaws.ses#LambdaAction$InvocationType */ =>  {
                let var_8 =
                    Some(
                        Result::<crate::types::InvocationType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InvocationType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_invocation_type(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::lambda_action_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
