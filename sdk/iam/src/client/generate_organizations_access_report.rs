// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GenerateOrganizationsAccessReport`](crate::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`entity_path(impl Into<String>)`](crate::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportFluentBuilder::entity_path) / [`set_entity_path(Option<String>)`](crate::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportFluentBuilder::set_entity_path):<br>required: **true**<br><p>The path of the Organizations entity (root, OU, or account). You can build an entity path using the known structure of your organization. For example, assume that your account ID is <code>123456789012</code> and its parent OU ID is <code>ou-rge0-awsabcde</code>. The organization root ID is <code>r-f6g7h8i9j0example</code> and your organization ID is <code>o-a1b2c3d4e5</code>. Your entity path is <code>o-a1b2c3d4e5/r-f6g7h8i9j0example/ou-rge0-awsabcde/123456789012</code>.</p><br>
    ///   - [`organizations_policy_id(impl Into<String>)`](crate::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportFluentBuilder::organizations_policy_id) / [`set_organizations_policy_id(Option<String>)`](crate::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportFluentBuilder::set_organizations_policy_id):<br>required: **false**<br><p>The identifier of the Organizations service control policy (SCP). This parameter is optional.</p> <p>This ID is used to generate information about when an account principal that is limited by the SCP attempted to access an Amazon Web Services service.</p><br>
    /// - On success, responds with [`GenerateOrganizationsAccessReportOutput`](crate::operation::generate_organizations_access_report::GenerateOrganizationsAccessReportOutput) with field(s):
    ///   - [`job_id(Option<String>)`](crate::operation::generate_organizations_access_report::GenerateOrganizationsAccessReportOutput::job_id): <p>The job identifier that you can use in the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetOrganizationsAccessReport.html">GetOrganizationsAccessReport</a> operation.</p>
    /// - On failure, responds with [`SdkError<GenerateOrganizationsAccessReportError>`](crate::operation::generate_organizations_access_report::GenerateOrganizationsAccessReportError)
    pub fn generate_organizations_access_report(
        &self,
    ) -> crate::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportFluentBuilder {
        crate::operation::generate_organizations_access_report::builders::GenerateOrganizationsAccessReportFluentBuilder::new(self.handle.clone())
    }
}
