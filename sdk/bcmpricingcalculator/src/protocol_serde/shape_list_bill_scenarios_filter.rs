// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_bill_scenarios_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ListBillScenariosFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    {
        let mut array_1 = object.key("values").start_array();
        for item_2 in &input.values {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    if let Some(var_3) = &input.match_option {
        object.key("matchOption").string(var_3.as_str());
    }
    Ok(())
}
