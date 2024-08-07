// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateConfiguredTableAssociationAnalysisRule`](crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`membership_identifier(impl Into<String>)`](crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder::set_membership_identifier):<br>required: **true**<br><p>A unique identifier for the membership that the configured table association belongs to. Currently accepts the membership ID.</p><br>
    ///   - [`configured_table_association_identifier(impl Into<String>)`](crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder::configured_table_association_identifier) / [`set_configured_table_association_identifier(Option<String>)`](crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder::set_configured_table_association_identifier):<br>required: **true**<br><p>The unique ID for the configured table association. Currently accepts the configured table association ID.</p><br>
    ///   - [`analysis_rule_type(ConfiguredTableAssociationAnalysisRuleType)`](crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder::analysis_rule_type) / [`set_analysis_rule_type(Option<ConfiguredTableAssociationAnalysisRuleType>)`](crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder::set_analysis_rule_type):<br>required: **true**<br><p>The type of analysis rule.</p><br>
    ///   - [`analysis_rule_policy(ConfiguredTableAssociationAnalysisRulePolicy)`](crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder::analysis_rule_policy) / [`set_analysis_rule_policy(Option<ConfiguredTableAssociationAnalysisRulePolicy>)`](crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder::set_analysis_rule_policy):<br>required: **true**<br><p>The analysis rule policy that was created for the configured table association.</p><br>
    /// - On success, responds with [`CreateConfiguredTableAssociationAnalysisRuleOutput`](crate::operation::create_configured_table_association_analysis_rule::CreateConfiguredTableAssociationAnalysisRuleOutput) with field(s):
    ///   - [`analysis_rule(Option<ConfiguredTableAssociationAnalysisRule>)`](crate::operation::create_configured_table_association_analysis_rule::CreateConfiguredTableAssociationAnalysisRuleOutput::analysis_rule): <p>The analysis rule for the conﬁgured table association. In the console, the <code>ConfiguredTableAssociationAnalysisRule</code> is referred to as the <i>collaboration analysis rule</i>.</p>
    /// - On failure, responds with [`SdkError<CreateConfiguredTableAssociationAnalysisRuleError>`](crate::operation::create_configured_table_association_analysis_rule::CreateConfiguredTableAssociationAnalysisRuleError)
    pub fn create_configured_table_association_analysis_rule(
        &self,
    ) -> crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder
    {
        crate::operation::create_configured_table_association_analysis_rule::builders::CreateConfiguredTableAssociationAnalysisRuleFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
