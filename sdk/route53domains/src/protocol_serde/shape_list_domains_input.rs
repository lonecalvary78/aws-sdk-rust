// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_domains_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_domains::ListDomainsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.filter_conditions {
        let mut array_2 = object.key("FilterConditions").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_filter_condition::ser_filter_condition(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.sort_condition {
        #[allow(unused_mut)]
        let mut object_6 = object.key("SortCondition").start_object();
        crate::protocol_serde::shape_sort_condition::ser_sort_condition(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.marker {
        object.key("Marker").string(var_7.as_str());
    }
    if let Some(var_8) = &input.max_items {
        object.key("MaxItems").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    Ok(())
}
