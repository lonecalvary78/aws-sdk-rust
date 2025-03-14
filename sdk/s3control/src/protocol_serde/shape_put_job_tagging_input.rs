// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_job_tagging_input_input_input(
    input: &crate::operation::put_job_tagging::PutJobTaggingInput,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.tags {
        let mut inner_writer = scope.start_el("Tags").finish();
        for list_item_2 in var_1 {
            {
                let inner_writer = inner_writer.start_el("member");
                crate::protocol_serde::shape_s3_tag::ser_s3_tag(list_item_2, inner_writer)?
            }
        }
    }
    scope.finish();
    Ok(())
}
