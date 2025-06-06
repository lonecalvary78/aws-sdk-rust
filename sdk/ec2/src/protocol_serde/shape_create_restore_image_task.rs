// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_restore_image_task_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_restore_image_task::CreateRestoreImageTaskOutput,
    crate::operation::create_restore_image_task::CreateRestoreImageTaskError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_restore_image_task::CreateRestoreImageTaskError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::create_restore_image_task::CreateRestoreImageTaskError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_restore_image_task_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_restore_image_task::CreateRestoreImageTaskOutput,
    crate::operation::create_restore_image_task::CreateRestoreImageTaskError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_restore_image_task::builders::CreateRestoreImageTaskOutputBuilder::default();
        output = crate::protocol_serde::shape_create_restore_image_task::de_create_restore_image_task(_response_body, output)
            .map_err(crate::operation::create_restore_image_task::CreateRestoreImageTaskError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_restore_image_task(
    inp: &[u8],
    mut builder: crate::operation::create_restore_image_task::builders::CreateRestoreImageTaskOutputBuilder,
) -> std::result::Result<
    crate::operation::create_restore_image_task::builders::CreateRestoreImageTaskOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateRestoreImageTaskResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateRestoreImageTaskResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("imageId") /* ImageId com.amazonaws.ec2.synthetic#CreateRestoreImageTaskOutput$ImageId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_image_id(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
