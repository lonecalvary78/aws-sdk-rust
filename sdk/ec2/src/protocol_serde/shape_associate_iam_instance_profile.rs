// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_iam_instance_profile_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileOutput,
    crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_iam_instance_profile_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileOutput,
    crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::associate_iam_instance_profile::builders::AssociateIamInstanceProfileOutputBuilder::default();
        output = crate::protocol_serde::shape_associate_iam_instance_profile::de_associate_iam_instance_profile(_response_body, output)
            .map_err(crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_associate_iam_instance_profile(
    inp: &[u8],
    mut builder: crate::operation::associate_iam_instance_profile::builders::AssociateIamInstanceProfileOutputBuilder,
) -> std::result::Result<
    crate::operation::associate_iam_instance_profile::builders::AssociateIamInstanceProfileOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("AssociateIamInstanceProfileResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected AssociateIamInstanceProfileResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("iamInstanceProfileAssociation") /* IamInstanceProfileAssociation com.amazonaws.ec2.synthetic#AssociateIamInstanceProfileOutput$IamInstanceProfileAssociation */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_iam_instance_profile_association::de_iam_instance_profile_association(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_iam_instance_profile_association(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
