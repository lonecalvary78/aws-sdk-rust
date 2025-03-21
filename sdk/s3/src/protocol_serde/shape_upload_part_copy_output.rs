// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_bucket_key_enabled_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<bool>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption-bucket-key-enabled");
    let var_1 = ::aws_smithy_http::header::read_many_primitive::<bool>(headers)?;
    if var_1.len() > 1 {
        Err(::aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_1.len()
        )))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub(crate) fn de_copy_part_result_payload(
    body: &[u8],
) -> std::result::Result<::std::option::Option<crate::types::CopyPartResult>, crate::operation::upload_part_copy::UploadPartCopyError> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_upload_part_copy_output::de_copy_part_result(body)
                .map_err(crate::operation::upload_part_copy::UploadPartCopyError::unhandled)
        })
        .transpose()
}

pub(crate) fn de_copy_source_version_id_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-copy-source-version-id");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_request_charged_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<crate::types::RequestCharged>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-request-charged");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_sse_customer_algorithm_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption-customer-algorithm");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_sse_customer_key_md5_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption-customer-key-MD5");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_ssekms_key_id_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption-aws-kms-key-id");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_server_side_encryption_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<crate::types::ServerSideEncryption>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-server-side-encryption");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub fn de_copy_part_result(inp: &[u8]) -> std::result::Result<crate::types::CopyPartResult, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("CopyPartResult")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CopyPartResult got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_copy_part_result::de_copy_part_result(&mut decoder)
}
