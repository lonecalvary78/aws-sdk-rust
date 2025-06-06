// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_vpc_attribute_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_vpc_attribute::DescribeVpcAttributeOutput,
    crate::operation::describe_vpc_attribute::DescribeVpcAttributeError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_vpc_attribute::DescribeVpcAttributeError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::describe_vpc_attribute::DescribeVpcAttributeError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_vpc_attribute_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_vpc_attribute::DescribeVpcAttributeOutput,
    crate::operation::describe_vpc_attribute::DescribeVpcAttributeError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_vpc_attribute::builders::DescribeVpcAttributeOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_vpc_attribute::de_describe_vpc_attribute(_response_body, output)
            .map_err(crate::operation::describe_vpc_attribute::DescribeVpcAttributeError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_vpc_attribute(
    inp: &[u8],
    mut builder: crate::operation::describe_vpc_attribute::builders::DescribeVpcAttributeOutputBuilder,
) -> std::result::Result<
    crate::operation::describe_vpc_attribute::builders::DescribeVpcAttributeOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeVpcAttributeResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeVpcAttributeResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("enableDnsHostnames") /* EnableDnsHostnames com.amazonaws.ec2.synthetic#DescribeVpcAttributeOutput$EnableDnsHostnames */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_attribute_boolean_value::de_attribute_boolean_value(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_enable_dns_hostnames(var_1);
            }
            ,
            s if s.matches("enableDnsSupport") /* EnableDnsSupport com.amazonaws.ec2.synthetic#DescribeVpcAttributeOutput$EnableDnsSupport */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_attribute_boolean_value::de_attribute_boolean_value(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_enable_dns_support(var_2);
            }
            ,
            s if s.matches("enableNetworkAddressUsageMetrics") /* EnableNetworkAddressUsageMetrics com.amazonaws.ec2.synthetic#DescribeVpcAttributeOutput$EnableNetworkAddressUsageMetrics */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_attribute_boolean_value::de_attribute_boolean_value(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_enable_network_address_usage_metrics(var_3);
            }
            ,
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2.synthetic#DescribeVpcAttributeOutput$VpcId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
