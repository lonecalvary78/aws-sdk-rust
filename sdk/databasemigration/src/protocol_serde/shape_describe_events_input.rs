// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_events_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_events::DescribeEventsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.source_identifier {
        object.key("SourceIdentifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.source_type {
        object.key("SourceType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_3, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_4, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_5) = &input.duration {
        object.key("Duration").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.event_categories {
        let mut array_7 = object.key("EventCategories").start_array();
        for item_8 in var_6 {
            {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.filters {
        let mut array_10 = object.key("Filters").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_filter::ser_filter(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.max_records {
        object.key("MaxRecords").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_13).into()),
        );
    }
    if let Some(var_14) = &input.marker {
        object.key("Marker").string(var_14.as_str());
    }
    Ok(())
}
