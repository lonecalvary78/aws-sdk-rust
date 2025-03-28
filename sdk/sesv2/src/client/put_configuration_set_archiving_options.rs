// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutConfigurationSetArchivingOptions`](crate::operation::put_configuration_set_archiving_options::builders::PutConfigurationSetArchivingOptionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_set_name(impl Into<String>)`](crate::operation::put_configuration_set_archiving_options::builders::PutConfigurationSetArchivingOptionsFluentBuilder::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::operation::put_configuration_set_archiving_options::builders::PutConfigurationSetArchivingOptionsFluentBuilder::set_configuration_set_name):<br>required: **true**<br><p>The name of the configuration set to associate with a MailManager archive.</p><br>
    ///   - [`archive_arn(impl Into<String>)`](crate::operation::put_configuration_set_archiving_options::builders::PutConfigurationSetArchivingOptionsFluentBuilder::archive_arn) / [`set_archive_arn(Option<String>)`](crate::operation::put_configuration_set_archiving_options::builders::PutConfigurationSetArchivingOptionsFluentBuilder::set_archive_arn):<br>required: **false**<br><p>The Amazon Resource Name (ARN) of the MailManager archive that the Amazon SES API v2 sends email to.</p><br>
    /// - On success, responds with [`PutConfigurationSetArchivingOptionsOutput`](crate::operation::put_configuration_set_archiving_options::PutConfigurationSetArchivingOptionsOutput)
    /// - On failure, responds with [`SdkError<PutConfigurationSetArchivingOptionsError>`](crate::operation::put_configuration_set_archiving_options::PutConfigurationSetArchivingOptionsError)
    pub fn put_configuration_set_archiving_options(
        &self,
    ) -> crate::operation::put_configuration_set_archiving_options::builders::PutConfigurationSetArchivingOptionsFluentBuilder {
        crate::operation::put_configuration_set_archiving_options::builders::PutConfigurationSetArchivingOptionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
