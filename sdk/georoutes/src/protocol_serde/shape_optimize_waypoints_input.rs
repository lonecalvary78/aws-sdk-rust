// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_optimize_waypoints_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::optimize_waypoints::OptimizeWaypointsInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.avoid {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Avoid").start_object();
        crate::protocol_serde::shape_waypoint_optimization_avoidance_options::ser_waypoint_optimization_avoidance_options(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.departure_time {
        object.key("DepartureTime").string(var_3.as_str());
    }
    if let Some(var_4) = &input.destination {
        let mut array_5 = object.key("Destination").start_array();
        for item_6 in var_4 {
            {
                array_5.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::Float((*item_6).into()),
                );
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.destination_options {
        #[allow(unused_mut)]
        let mut object_8 = object.key("DestinationOptions").start_object();
        crate::protocol_serde::shape_waypoint_optimization_destination_options::ser_waypoint_optimization_destination_options(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.driver {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Driver").start_object();
        crate::protocol_serde::shape_waypoint_optimization_driver_options::ser_waypoint_optimization_driver_options(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.exclude {
        #[allow(unused_mut)]
        let mut object_12 = object.key("Exclude").start_object();
        crate::protocol_serde::shape_waypoint_optimization_exclusion_options::ser_waypoint_optimization_exclusion_options(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.optimize_sequencing_for {
        object.key("OptimizeSequencingFor").string(var_13.as_str());
    }
    if let Some(var_14) = &input.origin {
        let mut array_15 = object.key("Origin").start_array();
        for item_16 in var_14 {
            {
                array_15.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::Float((*item_16).into()),
                );
            }
        }
        array_15.finish();
    }
    if let Some(var_17) = &input.origin_options {
        #[allow(unused_mut)]
        let mut object_18 = object.key("OriginOptions").start_object();
        crate::protocol_serde::shape_waypoint_optimization_origin_options::ser_waypoint_optimization_origin_options(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.traffic {
        #[allow(unused_mut)]
        let mut object_20 = object.key("Traffic").start_object();
        crate::protocol_serde::shape_waypoint_optimization_traffic_options::ser_waypoint_optimization_traffic_options(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.travel_mode {
        object.key("TravelMode").string(var_21.as_str());
    }
    if let Some(var_22) = &input.travel_mode_options {
        #[allow(unused_mut)]
        let mut object_23 = object.key("TravelModeOptions").start_object();
        crate::protocol_serde::shape_waypoint_optimization_travel_mode_options::ser_waypoint_optimization_travel_mode_options(
            &mut object_23,
            var_22,
        )?;
        object_23.finish();
    }
    if let Some(var_24) = &input.waypoints {
        let mut array_25 = object.key("Waypoints").start_array();
        for item_26 in var_24 {
            {
                #[allow(unused_mut)]
                let mut object_27 = array_25.value().start_object();
                crate::protocol_serde::shape_waypoint_optimization_waypoint::ser_waypoint_optimization_waypoint(&mut object_27, item_26)?;
                object_27.finish();
            }
        }
        array_25.finish();
    }
    Ok(())
}
