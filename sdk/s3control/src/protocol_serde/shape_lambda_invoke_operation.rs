// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lambda_invoke_operation(
    input: &crate::types::LambdaInvokeOperation,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.function_arn {
        let mut inner_writer = scope.start_el("FunctionArn").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.invocation_schema_version {
        let mut inner_writer = scope.start_el("InvocationSchemaVersion").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.user_arguments {
        let mut inner_writer = scope.start_el("UserArguments").finish();
        for (key_4, value_5) in var_3 {
            let mut entry = inner_writer.start_el("entry").finish();
            {
                let mut inner_writer = entry.start_el("key").finish();
                inner_writer.data(key_4.as_str());
            }
            {
                let mut inner_writer = entry.start_el("value").finish();
                inner_writer.data(value_5.as_str());
            }
        }
    }
    scope.finish();
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_lambda_invoke_operation(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::LambdaInvokeOperation, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LambdaInvokeOperation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("FunctionArn") /* FunctionArn com.amazonaws.s3control#LambdaInvokeOperation$FunctionArn */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_function_arn(var_6);
            }
            ,
            s if s.matches("InvocationSchemaVersion") /* InvocationSchemaVersion com.amazonaws.s3control#LambdaInvokeOperation$InvocationSchemaVersion */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_invocation_schema_version(var_7);
            }
            ,
            s if s.matches("UserArguments") /* UserArguments com.amazonaws.s3control#LambdaInvokeOperation$UserArguments */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_user_arguments::de_user_arguments(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_user_arguments(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
