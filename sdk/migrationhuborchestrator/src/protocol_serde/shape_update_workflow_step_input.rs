// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_workflow_step_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_workflow_step::UpdateWorkflowStepInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next {
        let mut array_4 = object.key("next").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.outputs {
        let mut array_7 = object.key("outputs").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_workflow_step_output::ser_workflow_step_output(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.previous {
        let mut array_11 = object.key("previous").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.status {
        object.key("status").string(var_13.as_str());
    }
    if let Some(var_14) = &input.step_action_type {
        object.key("stepActionType").string(var_14.as_str());
    }
    if let Some(var_15) = &input.step_group_id {
        object.key("stepGroupId").string(var_15.as_str());
    }
    if let Some(var_16) = &input.step_target {
        let mut array_17 = object.key("stepTarget").start_array();
        for item_18 in var_16 {
            {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    if let Some(var_19) = &input.workflow_id {
        object.key("workflowId").string(var_19.as_str());
    }
    if let Some(var_20) = &input.workflow_step_automation_configuration {
        #[allow(unused_mut)]
        let mut object_21 = object.key("workflowStepAutomationConfiguration").start_object();
        crate::protocol_serde::shape_workflow_step_automation_configuration::ser_workflow_step_automation_configuration(&mut object_21, var_20)?;
        object_21.finish();
    }
    Ok(())
}
