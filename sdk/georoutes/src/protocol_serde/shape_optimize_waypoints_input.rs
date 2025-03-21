// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_optimize_waypoints_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::optimize_waypoints::OptimizeWaypointsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.avoid {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Avoid").start_object();
        crate::protocol_serde::shape_waypoint_optimization_avoidance_options::ser_waypoint_optimization_avoidance_options(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.clustering {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Clustering").start_object();
        crate::protocol_serde::shape_waypoint_optimization_clustering_options::ser_waypoint_optimization_clustering_options(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.departure_time {
        object.key("DepartureTime").string(var_5.as_str());
    }
    if let Some(var_6) = &input.destination {
        let mut array_7 = object.key("Destination").start_array();
        for item_8 in var_6 {
            {
                array_7.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::Float((*item_8).into()),
                );
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.destination_options {
        #[allow(unused_mut)]
        let mut object_10 = object.key("DestinationOptions").start_object();
        crate::protocol_serde::shape_waypoint_optimization_destination_options::ser_waypoint_optimization_destination_options(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.driver {
        #[allow(unused_mut)]
        let mut object_12 = object.key("Driver").start_object();
        crate::protocol_serde::shape_waypoint_optimization_driver_options::ser_waypoint_optimization_driver_options(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.exclude {
        #[allow(unused_mut)]
        let mut object_14 = object.key("Exclude").start_object();
        crate::protocol_serde::shape_waypoint_optimization_exclusion_options::ser_waypoint_optimization_exclusion_options(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.optimize_sequencing_for {
        object.key("OptimizeSequencingFor").string(var_15.as_str());
    }
    if let Some(var_16) = &input.origin {
        let mut array_17 = object.key("Origin").start_array();
        for item_18 in var_16 {
            {
                array_17.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::Float((*item_18).into()),
                );
            }
        }
        array_17.finish();
    }
    if let Some(var_19) = &input.origin_options {
        #[allow(unused_mut)]
        let mut object_20 = object.key("OriginOptions").start_object();
        crate::protocol_serde::shape_waypoint_optimization_origin_options::ser_waypoint_optimization_origin_options(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.traffic {
        #[allow(unused_mut)]
        let mut object_22 = object.key("Traffic").start_object();
        crate::protocol_serde::shape_waypoint_optimization_traffic_options::ser_waypoint_optimization_traffic_options(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.travel_mode {
        object.key("TravelMode").string(var_23.as_str());
    }
    if let Some(var_24) = &input.travel_mode_options {
        #[allow(unused_mut)]
        let mut object_25 = object.key("TravelModeOptions").start_object();
        crate::protocol_serde::shape_waypoint_optimization_travel_mode_options::ser_waypoint_optimization_travel_mode_options(
            &mut object_25,
            var_24,
        )?;
        object_25.finish();
    }
    if let Some(var_26) = &input.waypoints {
        let mut array_27 = object.key("Waypoints").start_array();
        for item_28 in var_26 {
            {
                #[allow(unused_mut)]
                let mut object_29 = array_27.value().start_object();
                crate::protocol_serde::shape_waypoint_optimization_waypoint::ser_waypoint_optimization_waypoint(&mut object_29, item_28)?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    Ok(())
}
