// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_function_list_payload(
    body: &[u8],
) -> std::result::Result<::std::option::Option<crate::types::FunctionList>, crate::operation::list_functions::ListFunctionsError> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_list_functions_output::de_function_list(body)
                .map_err(crate::operation::list_functions::ListFunctionsError::unhandled)
        })
        .transpose()
}

pub fn de_function_list(inp: &[u8]) -> std::result::Result<crate::types::FunctionList, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("FunctionList")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected FunctionList got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_function_list::de_function_list(&mut decoder)
}
