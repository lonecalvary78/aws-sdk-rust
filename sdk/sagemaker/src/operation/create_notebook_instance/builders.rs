// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_notebook_instance::_create_notebook_instance_output::CreateNotebookInstanceOutputBuilder;

pub use crate::operation::create_notebook_instance::_create_notebook_instance_input::CreateNotebookInstanceInputBuilder;

impl crate::operation::create_notebook_instance::builders::CreateNotebookInstanceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_notebook_instance::CreateNotebookInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_notebook_instance::CreateNotebookInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_notebook_instance();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateNotebookInstance`.
///
/// <p>Creates an SageMaker AI notebook instance. A notebook instance is a machine learning (ML) compute instance running on a Jupyter notebook.</p>
/// <p>In a <code>CreateNotebookInstance</code> request, specify the type of ML compute instance that you want to run. SageMaker AI launches the instance, installs common libraries that you can use to explore datasets for model training, and attaches an ML storage volume to the notebook instance.</p>
/// <p>SageMaker AI also provides a set of example notebooks. Each notebook demonstrates how to use SageMaker AI with a specific algorithm or with a machine learning framework.</p>
/// <p>After receiving the request, SageMaker AI does the following:</p>
/// <ol>
/// <li>
/// <p>Creates a network interface in the SageMaker AI VPC.</p></li>
/// <li>
/// <p>(Option) If you specified <code>SubnetId</code>, SageMaker AI creates a network interface in your own VPC, which is inferred from the subnet ID that you provide in the input. When creating this network interface, SageMaker AI attaches the security group that you specified in the request to the network interface that it creates in your VPC.</p></li>
/// <li>
/// <p>Launches an EC2 instance of the type specified in the request in the SageMaker AI VPC. If you specified <code>SubnetId</code> of your VPC, SageMaker AI specifies both network interfaces when launching this instance. This enables inbound traffic from your own VPC to the notebook instance, assuming that the security groups allow it.</p></li>
/// </ol>
/// <p>After creating the notebook instance, SageMaker AI returns its Amazon Resource Name (ARN). You can't change the name of a notebook instance after you create it.</p>
/// <p>After SageMaker AI creates the notebook instance, you can connect to the Jupyter server and work in Jupyter notebooks. For example, you can write code to explore a dataset that you can use for model training, train a model, host models by creating SageMaker AI endpoints, and validate hosted models.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateNotebookInstanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_notebook_instance::builders::CreateNotebookInstanceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_notebook_instance::CreateNotebookInstanceOutput,
        crate::operation::create_notebook_instance::CreateNotebookInstanceError,
    > for CreateNotebookInstanceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_notebook_instance::CreateNotebookInstanceOutput,
            crate::operation::create_notebook_instance::CreateNotebookInstanceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateNotebookInstanceFluentBuilder {
    /// Creates a new `CreateNotebookInstanceFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateNotebookInstance as a reference.
    pub fn as_input(&self) -> &crate::operation::create_notebook_instance::builders::CreateNotebookInstanceInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_notebook_instance::CreateNotebookInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_notebook_instance::CreateNotebookInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_notebook_instance::CreateNotebookInstance::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_notebook_instance::CreateNotebookInstance::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_notebook_instance::CreateNotebookInstanceOutput,
        crate::operation::create_notebook_instance::CreateNotebookInstanceError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the new notebook instance.</p>
    pub fn notebook_instance_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.notebook_instance_name(input.into());
        self
    }
    /// <p>The name of the new notebook instance.</p>
    pub fn set_notebook_instance_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_notebook_instance_name(input);
        self
    }
    /// <p>The name of the new notebook instance.</p>
    pub fn get_notebook_instance_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_notebook_instance_name()
    }
    /// <p>The type of ML compute instance to launch for the notebook instance.</p>
    pub fn instance_type(mut self, input: crate::types::InstanceType) -> Self {
        self.inner = self.inner.instance_type(input);
        self
    }
    /// <p>The type of ML compute instance to launch for the notebook instance.</p>
    pub fn set_instance_type(mut self, input: ::std::option::Option<crate::types::InstanceType>) -> Self {
        self.inner = self.inner.set_instance_type(input);
        self
    }
    /// <p>The type of ML compute instance to launch for the notebook instance.</p>
    pub fn get_instance_type(&self) -> &::std::option::Option<crate::types::InstanceType> {
        self.inner.get_instance_type()
    }
    /// <p>The ID of the subnet in a VPC to which you would like to have a connectivity from your ML compute instance.</p>
    pub fn subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subnet_id(input.into());
        self
    }
    /// <p>The ID of the subnet in a VPC to which you would like to have a connectivity from your ML compute instance.</p>
    pub fn set_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subnet_id(input);
        self
    }
    /// <p>The ID of the subnet in a VPC to which you would like to have a connectivity from your ML compute instance.</p>
    pub fn get_subnet_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_subnet_id()
    }
    ///
    /// Appends an item to `SecurityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>The VPC security group IDs, in the form sg-xxxxxxxx. The security groups must be for the same VPC as specified in the subnet.</p>
    pub fn security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.security_group_ids(input.into());
        self
    }
    /// <p>The VPC security group IDs, in the form sg-xxxxxxxx. The security groups must be for the same VPC as specified in the subnet.</p>
    pub fn set_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_security_group_ids(input);
        self
    }
    /// <p>The VPC security group IDs, in the form sg-xxxxxxxx. The security groups must be for the same VPC as specified in the subnet.</p>
    pub fn get_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_security_group_ids()
    }
    /// <p>When you send any requests to Amazon Web Services resources from the notebook instance, SageMaker AI assumes this role to perform tasks on your behalf. You must grant this role necessary permissions so SageMaker AI can perform these tasks. The policy must allow the SageMaker AI service principal (sagemaker.amazonaws.com) permissions to assume this role. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">SageMaker AI Roles</a>.</p><note>
    /// <p>To be able to pass this role to SageMaker AI, the caller of this API must have the <code>iam:PassRole</code> permission.</p>
    /// </note>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>When you send any requests to Amazon Web Services resources from the notebook instance, SageMaker AI assumes this role to perform tasks on your behalf. You must grant this role necessary permissions so SageMaker AI can perform these tasks. The policy must allow the SageMaker AI service principal (sagemaker.amazonaws.com) permissions to assume this role. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">SageMaker AI Roles</a>.</p><note>
    /// <p>To be able to pass this role to SageMaker AI, the caller of this API must have the <code>iam:PassRole</code> permission.</p>
    /// </note>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>When you send any requests to Amazon Web Services resources from the notebook instance, SageMaker AI assumes this role to perform tasks on your behalf. You must grant this role necessary permissions so SageMaker AI can perform these tasks. The policy must allow the SageMaker AI service principal (sagemaker.amazonaws.com) permissions to assume this role. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html">SageMaker AI Roles</a>.</p><note>
    /// <p>To be able to pass this role to SageMaker AI, the caller of this API must have the <code>iam:PassRole</code> permission.</p>
    /// </note>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// <p>The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service key that SageMaker AI uses to encrypt data on the storage volume attached to your notebook instance. The KMS key you provide must be enabled. For information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/enabling-keys.html">Enabling and Disabling Keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service key that SageMaker AI uses to encrypt data on the storage volume attached to your notebook instance. The KMS key you provide must be enabled. For information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/enabling-keys.html">Enabling and Disabling Keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service key that SageMaker AI uses to encrypt data on the storage volume attached to your notebook instance. The KMS key you provide must be enabled. For information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/enabling-keys.html">Enabling and Disabling Keys</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_id()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>The name of a lifecycle configuration to associate with the notebook instance. For information about lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    pub fn lifecycle_config_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lifecycle_config_name(input.into());
        self
    }
    /// <p>The name of a lifecycle configuration to associate with the notebook instance. For information about lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    pub fn set_lifecycle_config_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lifecycle_config_name(input);
        self
    }
    /// <p>The name of a lifecycle configuration to associate with the notebook instance. For information about lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a>.</p>
    pub fn get_lifecycle_config_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_lifecycle_config_name()
    }
    /// <p>Sets whether SageMaker AI provides internet access to the notebook instance. If you set this to <code>Disabled</code> this notebook instance is able to access resources only in your VPC, and is not be able to connect to SageMaker AI training and endpoint services unless you configure a NAT Gateway in your VPC.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/appendix-additional-considerations.html#appendix-notebook-and-internet-access">Notebook Instances Are Internet-Enabled by Default</a>. You can set the value of this parameter to <code>Disabled</code> only if you set a value for the <code>SubnetId</code> parameter.</p>
    pub fn direct_internet_access(mut self, input: crate::types::DirectInternetAccess) -> Self {
        self.inner = self.inner.direct_internet_access(input);
        self
    }
    /// <p>Sets whether SageMaker AI provides internet access to the notebook instance. If you set this to <code>Disabled</code> this notebook instance is able to access resources only in your VPC, and is not be able to connect to SageMaker AI training and endpoint services unless you configure a NAT Gateway in your VPC.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/appendix-additional-considerations.html#appendix-notebook-and-internet-access">Notebook Instances Are Internet-Enabled by Default</a>. You can set the value of this parameter to <code>Disabled</code> only if you set a value for the <code>SubnetId</code> parameter.</p>
    pub fn set_direct_internet_access(mut self, input: ::std::option::Option<crate::types::DirectInternetAccess>) -> Self {
        self.inner = self.inner.set_direct_internet_access(input);
        self
    }
    /// <p>Sets whether SageMaker AI provides internet access to the notebook instance. If you set this to <code>Disabled</code> this notebook instance is able to access resources only in your VPC, and is not be able to connect to SageMaker AI training and endpoint services unless you configure a NAT Gateway in your VPC.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/appendix-additional-considerations.html#appendix-notebook-and-internet-access">Notebook Instances Are Internet-Enabled by Default</a>. You can set the value of this parameter to <code>Disabled</code> only if you set a value for the <code>SubnetId</code> parameter.</p>
    pub fn get_direct_internet_access(&self) -> &::std::option::Option<crate::types::DirectInternetAccess> {
        self.inner.get_direct_internet_access()
    }
    /// <p>The size, in GB, of the ML storage volume to attach to the notebook instance. The default value is 5 GB.</p>
    pub fn volume_size_in_gb(mut self, input: i32) -> Self {
        self.inner = self.inner.volume_size_in_gb(input);
        self
    }
    /// <p>The size, in GB, of the ML storage volume to attach to the notebook instance. The default value is 5 GB.</p>
    pub fn set_volume_size_in_gb(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_volume_size_in_gb(input);
        self
    }
    /// <p>The size, in GB, of the ML storage volume to attach to the notebook instance. The default value is 5 GB.</p>
    pub fn get_volume_size_in_gb(&self) -> &::std::option::Option<i32> {
        self.inner.get_volume_size_in_gb()
    }
    ///
    /// Appends an item to `AcceleratorTypes`.
    ///
    /// To override the contents of this collection use [`set_accelerator_types`](Self::set_accelerator_types).
    ///
    /// <p>This parameter is no longer supported. Elastic Inference (EI) is no longer available.</p>
    /// <p>This parameter was used to specify a list of EI instance types to associate with this notebook instance.</p>
    pub fn accelerator_types(mut self, input: crate::types::NotebookInstanceAcceleratorType) -> Self {
        self.inner = self.inner.accelerator_types(input);
        self
    }
    /// <p>This parameter is no longer supported. Elastic Inference (EI) is no longer available.</p>
    /// <p>This parameter was used to specify a list of EI instance types to associate with this notebook instance.</p>
    pub fn set_accelerator_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::NotebookInstanceAcceleratorType>>) -> Self {
        self.inner = self.inner.set_accelerator_types(input);
        self
    }
    /// <p>This parameter is no longer supported. Elastic Inference (EI) is no longer available.</p>
    /// <p>This parameter was used to specify a list of EI instance types to associate with this notebook instance.</p>
    pub fn get_accelerator_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::NotebookInstanceAcceleratorType>> {
        self.inner.get_accelerator_types()
    }
    /// <p>A Git repository to associate with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker AI Notebook Instances</a>.</p>
    pub fn default_code_repository(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.default_code_repository(input.into());
        self
    }
    /// <p>A Git repository to associate with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker AI Notebook Instances</a>.</p>
    pub fn set_default_code_repository(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_default_code_repository(input);
        self
    }
    /// <p>A Git repository to associate with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker AI Notebook Instances</a>.</p>
    pub fn get_default_code_repository(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_default_code_repository()
    }
    ///
    /// Appends an item to `AdditionalCodeRepositories`.
    ///
    /// To override the contents of this collection use [`set_additional_code_repositories`](Self::set_additional_code_repositories).
    ///
    /// <p>An array of up to three Git repositories to associate with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker AI Notebook Instances</a>.</p>
    pub fn additional_code_repositories(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.additional_code_repositories(input.into());
        self
    }
    /// <p>An array of up to three Git repositories to associate with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker AI Notebook Instances</a>.</p>
    pub fn set_additional_code_repositories(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_additional_code_repositories(input);
        self
    }
    /// <p>An array of up to three Git repositories to associate with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker AI Notebook Instances</a>.</p>
    pub fn get_additional_code_repositories(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_additional_code_repositories()
    }
    /// <p>Whether root access is enabled or disabled for users of the notebook instance. The default value is <code>Enabled</code>.</p><note>
    /// <p>Lifecycle configurations need root access to be able to set up a notebook instance. Because of this, lifecycle configurations associated with a notebook instance always run with root access even if you disable root access for users.</p>
    /// </note>
    pub fn root_access(mut self, input: crate::types::RootAccess) -> Self {
        self.inner = self.inner.root_access(input);
        self
    }
    /// <p>Whether root access is enabled or disabled for users of the notebook instance. The default value is <code>Enabled</code>.</p><note>
    /// <p>Lifecycle configurations need root access to be able to set up a notebook instance. Because of this, lifecycle configurations associated with a notebook instance always run with root access even if you disable root access for users.</p>
    /// </note>
    pub fn set_root_access(mut self, input: ::std::option::Option<crate::types::RootAccess>) -> Self {
        self.inner = self.inner.set_root_access(input);
        self
    }
    /// <p>Whether root access is enabled or disabled for users of the notebook instance. The default value is <code>Enabled</code>.</p><note>
    /// <p>Lifecycle configurations need root access to be able to set up a notebook instance. Because of this, lifecycle configurations associated with a notebook instance always run with root access even if you disable root access for users.</p>
    /// </note>
    pub fn get_root_access(&self) -> &::std::option::Option<crate::types::RootAccess> {
        self.inner.get_root_access()
    }
    /// <p>The platform identifier of the notebook instance runtime environment.</p>
    pub fn platform_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.platform_identifier(input.into());
        self
    }
    /// <p>The platform identifier of the notebook instance runtime environment.</p>
    pub fn set_platform_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_platform_identifier(input);
        self
    }
    /// <p>The platform identifier of the notebook instance runtime environment.</p>
    pub fn get_platform_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_platform_identifier()
    }
    /// <p>Information on the IMDS configuration of the notebook instance</p>
    pub fn instance_metadata_service_configuration(mut self, input: crate::types::InstanceMetadataServiceConfiguration) -> Self {
        self.inner = self.inner.instance_metadata_service_configuration(input);
        self
    }
    /// <p>Information on the IMDS configuration of the notebook instance</p>
    pub fn set_instance_metadata_service_configuration(
        mut self,
        input: ::std::option::Option<crate::types::InstanceMetadataServiceConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_instance_metadata_service_configuration(input);
        self
    }
    /// <p>Information on the IMDS configuration of the notebook instance</p>
    pub fn get_instance_metadata_service_configuration(&self) -> &::std::option::Option<crate::types::InstanceMetadataServiceConfiguration> {
        self.inner.get_instance_metadata_service_configuration()
    }
}
