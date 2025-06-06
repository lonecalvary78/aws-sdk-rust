// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_untag_stream_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::untag_stream::UntagStreamInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.stream_arn {
        object.key("StreamARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.stream_name {
        object.key("StreamName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tag_key_list {
        let mut array_4 = object.key("TagKeyList").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}
