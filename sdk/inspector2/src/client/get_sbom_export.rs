// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSbomExport`](crate::operation::get_sbom_export::builders::GetSbomExportFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`report_id(impl Into<String>)`](crate::operation::get_sbom_export::builders::GetSbomExportFluentBuilder::report_id) / [`set_report_id(Option<String>)`](crate::operation::get_sbom_export::builders::GetSbomExportFluentBuilder::set_report_id):<br>required: **true**<br><p>The report ID of the SBOM export to get details for.</p><br>
    /// - On success, responds with [`GetSbomExportOutput`](crate::operation::get_sbom_export::GetSbomExportOutput) with field(s):
    ///   - [`report_id(Option<String>)`](crate::operation::get_sbom_export::GetSbomExportOutput::report_id): <p>The report ID of the software bill of materials (SBOM) report.</p>
    ///   - [`format(Option<SbomReportFormat>)`](crate::operation::get_sbom_export::GetSbomExportOutput::format): <p>The format of the software bill of materials (SBOM) report.</p>
    ///   - [`status(Option<ExternalReportStatus>)`](crate::operation::get_sbom_export::GetSbomExportOutput::status): <p>The status of the software bill of materials (SBOM) report.</p>
    ///   - [`error_code(Option<ReportingErrorCode>)`](crate::operation::get_sbom_export::GetSbomExportOutput::error_code): <p>An error code.</p>
    ///   - [`error_message(Option<String>)`](crate::operation::get_sbom_export::GetSbomExportOutput::error_message): <p>An error message.</p>
    ///   - [`s3_destination(Option<Destination>)`](crate::operation::get_sbom_export::GetSbomExportOutput::s3_destination): <p>Contains details of the Amazon S3 bucket and KMS key used to export findings</p>
    ///   - [`filter_criteria(Option<ResourceFilterCriteria>)`](crate::operation::get_sbom_export::GetSbomExportOutput::filter_criteria): <p>Contains details about the resource filter criteria used for the software bill of materials (SBOM) report.</p>
    /// - On failure, responds with [`SdkError<GetSbomExportError>`](crate::operation::get_sbom_export::GetSbomExportError)
    pub fn get_sbom_export(&self) -> crate::operation::get_sbom_export::builders::GetSbomExportFluentBuilder {
        crate::operation::get_sbom_export::builders::GetSbomExportFluentBuilder::new(self.handle.clone())
    }
}
