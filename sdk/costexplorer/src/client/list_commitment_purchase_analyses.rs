// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCommitmentPurchaseAnalyses`](crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`analysis_status(AnalysisStatus)`](crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder::analysis_status) / [`set_analysis_status(Option<AnalysisStatus>)`](crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder::set_analysis_status):<br>required: **false**<br><p>The status of the analysis.</p><br>
    ///   - [`next_page_token(impl Into<String>)`](crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder::next_page_token) / [`set_next_page_token(Option<String>)`](crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder::set_next_page_token):<br>required: **false**<br><p>The token to retrieve the next set of results.</p><br>
    ///   - [`page_size(i32)`](crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder::page_size) / [`set_page_size(Option<i32>)`](crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder::set_page_size):<br>required: **false**<br><p>The number of analyses that you want returned in a single response object.</p><br>
    ///   - [`analysis_ids(impl Into<String>)`](crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder::analysis_ids) / [`set_analysis_ids(Option<Vec::<String>>)`](crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder::set_analysis_ids):<br>required: **false**<br><p>The analysis IDs associated with the commitment purchase analyses.</p><br>
    /// - On success, responds with [`ListCommitmentPurchaseAnalysesOutput`](crate::operation::list_commitment_purchase_analyses::ListCommitmentPurchaseAnalysesOutput) with field(s):
    ///   - [`analysis_summary_list(Option<Vec::<AnalysisSummary>>)`](crate::operation::list_commitment_purchase_analyses::ListCommitmentPurchaseAnalysesOutput::analysis_summary_list): <p>The list of analyses.</p>
    ///   - [`next_page_token(Option<String>)`](crate::operation::list_commitment_purchase_analyses::ListCommitmentPurchaseAnalysesOutput::next_page_token): <p>The token to retrieve the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListCommitmentPurchaseAnalysesError>`](crate::operation::list_commitment_purchase_analyses::ListCommitmentPurchaseAnalysesError)
    pub fn list_commitment_purchase_analyses(
        &self,
    ) -> crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder {
        crate::operation::list_commitment_purchase_analyses::builders::ListCommitmentPurchaseAnalysesFluentBuilder::new(self.handle.clone())
    }
}
