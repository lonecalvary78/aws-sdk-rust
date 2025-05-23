// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateResourceProfileDetections`](crate::operation::update_resource_profile_detections::builders::UpdateResourceProfileDetectionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::update_resource_profile_detections::builders::UpdateResourceProfileDetectionsFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::update_resource_profile_detections::builders::UpdateResourceProfileDetectionsFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the S3 bucket that the request applies to.</p><br>
    ///   - [`suppress_data_identifiers(SuppressDataIdentifier)`](crate::operation::update_resource_profile_detections::builders::UpdateResourceProfileDetectionsFluentBuilder::suppress_data_identifiers) / [`set_suppress_data_identifiers(Option<Vec::<SuppressDataIdentifier>>)`](crate::operation::update_resource_profile_detections::builders::UpdateResourceProfileDetectionsFluentBuilder::set_suppress_data_identifiers):<br>required: **false**<br><p>An array of objects, one for each custom data identifier or managed data identifier that detected a type of sensitive data to exclude from the bucket's score. To include all sensitive data types in the score, don't specify any values for this array.</p><br>
    /// - On success, responds with [`UpdateResourceProfileDetectionsOutput`](crate::operation::update_resource_profile_detections::UpdateResourceProfileDetectionsOutput)
    /// - On failure, responds with [`SdkError<UpdateResourceProfileDetectionsError>`](crate::operation::update_resource_profile_detections::UpdateResourceProfileDetectionsError)
    pub fn update_resource_profile_detections(
        &self,
    ) -> crate::operation::update_resource_profile_detections::builders::UpdateResourceProfileDetectionsFluentBuilder {
        crate::operation::update_resource_profile_detections::builders::UpdateResourceProfileDetectionsFluentBuilder::new(self.handle.clone())
    }
}
