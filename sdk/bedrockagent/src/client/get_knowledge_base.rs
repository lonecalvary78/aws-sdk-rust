// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetKnowledgeBase`](crate::operation::get_knowledge_base::builders::GetKnowledgeBaseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`knowledge_base_id(impl Into<String>)`](crate::operation::get_knowledge_base::builders::GetKnowledgeBaseFluentBuilder::knowledge_base_id) / [`set_knowledge_base_id(Option<String>)`](crate::operation::get_knowledge_base::builders::GetKnowledgeBaseFluentBuilder::set_knowledge_base_id):<br>required: **true**<br><p>The unique identifier of the knowledge base you want to get information on.</p><br>
    /// - On success, responds with [`GetKnowledgeBaseOutput`](crate::operation::get_knowledge_base::GetKnowledgeBaseOutput) with field(s):
    ///   - [`knowledge_base(Option<KnowledgeBase>)`](crate::operation::get_knowledge_base::GetKnowledgeBaseOutput::knowledge_base): <p>Contains details about the knowledge base.</p>
    /// - On failure, responds with [`SdkError<GetKnowledgeBaseError>`](crate::operation::get_knowledge_base::GetKnowledgeBaseError)
    pub fn get_knowledge_base(&self) -> crate::operation::get_knowledge_base::builders::GetKnowledgeBaseFluentBuilder {
        crate::operation::get_knowledge_base::builders::GetKnowledgeBaseFluentBuilder::new(self.handle.clone())
    }
}
