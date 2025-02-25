// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_deployment_group_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_deployment_group::UpdateDeploymentGroupInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.application_name {
        object.key("applicationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.current_deployment_group_name {
        object.key("currentDeploymentGroupName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.new_deployment_group_name {
        object.key("newDeploymentGroupName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.deployment_config_name {
        object.key("deploymentConfigName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.ec2_tag_filters {
        let mut array_6 = object.key("ec2TagFilters").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_ec2_tag_filter::ser_ec2_tag_filter(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.on_premises_instance_tag_filters {
        let mut array_10 = object.key("onPremisesInstanceTagFilters").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_tag_filter::ser_tag_filter(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.auto_scaling_groups {
        let mut array_14 = object.key("autoScalingGroups").start_array();
        for item_15 in var_13 {
            {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    if let Some(var_16) = &input.service_role_arn {
        object.key("serviceRoleArn").string(var_16.as_str());
    }
    if let Some(var_17) = &input.trigger_configurations {
        let mut array_18 = object.key("triggerConfigurations").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_trigger_config::ser_trigger_config(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.alarm_configuration {
        #[allow(unused_mut)]
        let mut object_22 = object.key("alarmConfiguration").start_object();
        crate::protocol_serde::shape_alarm_configuration::ser_alarm_configuration(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.auto_rollback_configuration {
        #[allow(unused_mut)]
        let mut object_24 = object.key("autoRollbackConfiguration").start_object();
        crate::protocol_serde::shape_auto_rollback_configuration::ser_auto_rollback_configuration(&mut object_24, var_23)?;
        object_24.finish();
    }
    if let Some(var_25) = &input.outdated_instances_strategy {
        object.key("outdatedInstancesStrategy").string(var_25.as_str());
    }
    if let Some(var_26) = &input.deployment_style {
        #[allow(unused_mut)]
        let mut object_27 = object.key("deploymentStyle").start_object();
        crate::protocol_serde::shape_deployment_style::ser_deployment_style(&mut object_27, var_26)?;
        object_27.finish();
    }
    if let Some(var_28) = &input.blue_green_deployment_configuration {
        #[allow(unused_mut)]
        let mut object_29 = object.key("blueGreenDeploymentConfiguration").start_object();
        crate::protocol_serde::shape_blue_green_deployment_configuration::ser_blue_green_deployment_configuration(&mut object_29, var_28)?;
        object_29.finish();
    }
    if let Some(var_30) = &input.load_balancer_info {
        #[allow(unused_mut)]
        let mut object_31 = object.key("loadBalancerInfo").start_object();
        crate::protocol_serde::shape_load_balancer_info::ser_load_balancer_info(&mut object_31, var_30)?;
        object_31.finish();
    }
    if let Some(var_32) = &input.ec2_tag_set {
        #[allow(unused_mut)]
        let mut object_33 = object.key("ec2TagSet").start_object();
        crate::protocol_serde::shape_ec2_tag_set::ser_ec2_tag_set(&mut object_33, var_32)?;
        object_33.finish();
    }
    if let Some(var_34) = &input.ecs_services {
        let mut array_35 = object.key("ecsServices").start_array();
        for item_36 in var_34 {
            {
                #[allow(unused_mut)]
                let mut object_37 = array_35.value().start_object();
                crate::protocol_serde::shape_ecs_service::ser_ecs_service(&mut object_37, item_36)?;
                object_37.finish();
            }
        }
        array_35.finish();
    }
    if let Some(var_38) = &input.on_premises_tag_set {
        #[allow(unused_mut)]
        let mut object_39 = object.key("onPremisesTagSet").start_object();
        crate::protocol_serde::shape_on_premises_tag_set::ser_on_premises_tag_set(&mut object_39, var_38)?;
        object_39.finish();
    }
    if let Some(var_40) = &input.termination_hook_enabled {
        object.key("terminationHookEnabled").boolean(*var_40);
    }
    Ok(())
}
