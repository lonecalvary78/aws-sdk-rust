// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_registered_user_quick_sight_console_embedding_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RegisteredUserQuickSightConsoleEmbeddingConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.initial_path {
        object.key("InitialPath").string(var_1.as_str());
    }
    if let Some(var_2) = &input.feature_configurations {
        #[allow(unused_mut)]
        let mut object_3 = object.key("FeatureConfigurations").start_object();
        crate::protocol_serde::shape_registered_user_console_feature_configurations::ser_registered_user_console_feature_configurations(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    Ok(())
}
