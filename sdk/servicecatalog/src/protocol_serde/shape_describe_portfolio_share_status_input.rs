// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_portfolio_share_status_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_portfolio_share_status::DescribePortfolioShareStatusInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.portfolio_share_token {
        object.key("PortfolioShareToken").string(var_1.as_str());
    }
    Ok(())
}
