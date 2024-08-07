// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDataQualityStatistics`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`statistic_id(impl Into<String>)`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::statistic_id) / [`set_statistic_id(Option<String>)`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::set_statistic_id):<br>required: **false**<br><p>The Statistic ID.</p><br>
    ///   - [`profile_id(impl Into<String>)`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::profile_id) / [`set_profile_id(Option<String>)`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::set_profile_id):<br>required: **false**<br><p>The Profile ID.</p><br>
    ///   - [`timestamp_filter(TimestampFilter)`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::timestamp_filter) / [`set_timestamp_filter(Option<TimestampFilter>)`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::set_timestamp_filter):<br>required: **false**<br><p>A timestamp filter.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in this request.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::set_next_token):<br>required: **false**<br><p>A pagination token to request the next page of results.</p><br>
    /// - On success, responds with [`ListDataQualityStatisticsOutput`](crate::operation::list_data_quality_statistics::ListDataQualityStatisticsOutput) with field(s):
    ///   - [`statistics(Option<Vec::<StatisticSummary>>)`](crate::operation::list_data_quality_statistics::ListDataQualityStatisticsOutput::statistics): <p>A <code>StatisticSummaryList</code>.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_data_quality_statistics::ListDataQualityStatisticsOutput::next_token): <p>A pagination token to request the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListDataQualityStatisticsError>`](crate::operation::list_data_quality_statistics::ListDataQualityStatisticsError)
    pub fn list_data_quality_statistics(&self) -> crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder {
        crate::operation::list_data_quality_statistics::builders::ListDataQualityStatisticsFluentBuilder::new(self.handle.clone())
    }
}
