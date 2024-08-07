// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchPutDataQualityStatisticAnnotation`](crate::operation::batch_put_data_quality_statistic_annotation::builders::BatchPutDataQualityStatisticAnnotationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`inclusion_annotations(DatapointInclusionAnnotation)`](crate::operation::batch_put_data_quality_statistic_annotation::builders::BatchPutDataQualityStatisticAnnotationFluentBuilder::inclusion_annotations) / [`set_inclusion_annotations(Option<Vec::<DatapointInclusionAnnotation>>)`](crate::operation::batch_put_data_quality_statistic_annotation::builders::BatchPutDataQualityStatisticAnnotationFluentBuilder::set_inclusion_annotations):<br>required: **true**<br><p>A list of <code>DatapointInclusionAnnotation</code>'s.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::batch_put_data_quality_statistic_annotation::builders::BatchPutDataQualityStatisticAnnotationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::batch_put_data_quality_statistic_annotation::builders::BatchPutDataQualityStatisticAnnotationFluentBuilder::set_client_token):<br>required: **false**<br><p>Client Token.</p><br>
    /// - On success, responds with [`BatchPutDataQualityStatisticAnnotationOutput`](crate::operation::batch_put_data_quality_statistic_annotation::BatchPutDataQualityStatisticAnnotationOutput) with field(s):
    ///   - [`failed_inclusion_annotations(Option<Vec::<AnnotationError>>)`](crate::operation::batch_put_data_quality_statistic_annotation::BatchPutDataQualityStatisticAnnotationOutput::failed_inclusion_annotations): <p>A list of <code>AnnotationError</code>'s.</p>
    /// - On failure, responds with [`SdkError<BatchPutDataQualityStatisticAnnotationError>`](crate::operation::batch_put_data_quality_statistic_annotation::BatchPutDataQualityStatisticAnnotationError)
    pub fn batch_put_data_quality_statistic_annotation(
        &self,
    ) -> crate::operation::batch_put_data_quality_statistic_annotation::builders::BatchPutDataQualityStatisticAnnotationFluentBuilder {
        crate::operation::batch_put_data_quality_statistic_annotation::builders::BatchPutDataQualityStatisticAnnotationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
