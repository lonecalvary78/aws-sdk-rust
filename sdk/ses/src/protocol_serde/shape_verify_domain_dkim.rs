// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_verify_domain_dkim_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::verify_domain_dkim::VerifyDomainDkimOutput, crate::operation::verify_domain_dkim::VerifyDomainDkimError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::verify_domain_dkim::VerifyDomainDkimError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::verify_domain_dkim::VerifyDomainDkimError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_verify_domain_dkim_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::verify_domain_dkim::VerifyDomainDkimOutput, crate::operation::verify_domain_dkim::VerifyDomainDkimError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::verify_domain_dkim::builders::VerifyDomainDkimOutputBuilder::default();
        output = crate::protocol_serde::shape_verify_domain_dkim::de_verify_domain_dkim(_response_body, output)
            .map_err(crate::operation::verify_domain_dkim::VerifyDomainDkimError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::verify_domain_dkim_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::verify_domain_dkim::VerifyDomainDkimError::unhandled)?
    })
}

#[allow(unused_mut)]
pub fn de_verify_domain_dkim(
    inp: &[u8],
    mut builder: crate::operation::verify_domain_dkim::builders::VerifyDomainDkimOutputBuilder,
) -> std::result::Result<crate::operation::verify_domain_dkim::builders::VerifyDomainDkimOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("VerifyDomainDkimResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected VerifyDomainDkimResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("VerifyDomainDkimResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected VerifyDomainDkimResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("DkimTokens") /* DkimTokens com.amazonaws.ses.synthetic#VerifyDomainDkimOutput$DkimTokens */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_verification_token_list::de_verification_token_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_dkim_tokens(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected VerifyDomainDkimResult tag"));
    };
    Ok(builder)
}
