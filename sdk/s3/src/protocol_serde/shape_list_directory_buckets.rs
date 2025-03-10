// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_directory_buckets_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_directory_buckets::ListDirectoryBucketsOutput,
    crate::operation::list_directory_buckets::ListDirectoryBucketsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_directory_buckets::ListDirectoryBucketsError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::list_directory_buckets::ListDirectoryBucketsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_directory_buckets_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_directory_buckets::ListDirectoryBucketsOutput,
    crate::operation::list_directory_buckets::ListDirectoryBucketsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_directory_buckets::builders::ListDirectoryBucketsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_directory_buckets::de_list_directory_buckets(_response_body, output)
            .map_err(crate::operation::list_directory_buckets::ListDirectoryBucketsError::unhandled)?;
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers).map(str::to_string));
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_directory_buckets(
    inp: &[u8],
    mut builder: crate::operation::list_directory_buckets::builders::ListDirectoryBucketsOutputBuilder,
) -> std::result::Result<
    crate::operation::list_directory_buckets::builders::ListDirectoryBucketsOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ContinuationToken") /* ContinuationToken com.amazonaws.s3.synthetic#ListDirectoryBucketsOutput$ContinuationToken */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_continuation_token(var_1);
            }
            ,
            s if s.matches("Buckets") /* Buckets com.amazonaws.s3.synthetic#ListDirectoryBucketsOutput$Buckets */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_buckets::de_buckets(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_buckets(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
