// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopConfigurationRecorder`](crate::operation::stop_configuration_recorder::builders::StopConfigurationRecorderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_recorder_name(impl Into<String>)`](crate::operation::stop_configuration_recorder::builders::StopConfigurationRecorderFluentBuilder::configuration_recorder_name) / [`set_configuration_recorder_name(Option<String>)`](crate::operation::stop_configuration_recorder::builders::StopConfigurationRecorderFluentBuilder::set_configuration_recorder_name):<br>required: **true**<br><p>The name of the customer managed configuration recorder that you want to stop.</p><br>
    /// - On success, responds with [`StopConfigurationRecorderOutput`](crate::operation::stop_configuration_recorder::StopConfigurationRecorderOutput)
    /// - On failure, responds with [`SdkError<StopConfigurationRecorderError>`](crate::operation::stop_configuration_recorder::StopConfigurationRecorderError)
    pub fn stop_configuration_recorder(&self) -> crate::operation::stop_configuration_recorder::builders::StopConfigurationRecorderFluentBuilder {
        crate::operation::stop_configuration_recorder::builders::StopConfigurationRecorderFluentBuilder::new(self.handle.clone())
    }
}
