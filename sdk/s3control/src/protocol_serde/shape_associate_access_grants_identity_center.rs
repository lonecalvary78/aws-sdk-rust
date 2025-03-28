// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_access_grants_identity_center_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_access_grants_identity_center::AssociateAccessGrantsIdentityCenterOutput,
    crate::operation::associate_access_grants_identity_center::AssociateAccessGrantsIdentityCenterError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::associate_access_grants_identity_center::AssociateAccessGrantsIdentityCenterError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::associate_access_grants_identity_center::AssociateAccessGrantsIdentityCenterError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_access_grants_identity_center_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_access_grants_identity_center::AssociateAccessGrantsIdentityCenterOutput,
    crate::operation::associate_access_grants_identity_center::AssociateAccessGrantsIdentityCenterError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::associate_access_grants_identity_center::builders::AssociateAccessGrantsIdentityCenterOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_associate_access_grants_identity_center_headers(
    input: &crate::operation::associate_access_grants_identity_center::AssociateAccessGrantsIdentityCenterInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.account_id {
        let formatted_2 = inner_1.as_str();
        let header_value = formatted_2;
        let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
            ::aws_smithy_types::error::operation::BuildError::invalid_field(
                "account_id",
                format!("`{}` cannot be used as a header value: {}", &header_value, err),
            )
        })?;
        builder = builder.header("x-amz-account-id", header_value);
    }
    Ok(builder)
}

pub fn ser_associate_access_grants_identity_center_op_input(
    input: &crate::operation::associate_access_grants_identity_center::AssociateAccessGrantsIdentityCenterInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("AssociateAccessGrantsIdentityCenterRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::protocol_serde::shape_associate_access_grants_identity_center_input::ser_associate_access_grants_identity_center_input_input_input(
            input, root,
        )?
    }
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
