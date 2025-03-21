// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"output":{"path":"Environments[].Status","expected":"Ready","comparator":"allStringEquals"}}
pub(crate) fn match_describe_environments_0c537620ab36f6939(
    _result: ::std::result::Result<
        &crate::operation::describe_environments::DescribeEnvironmentsOutput,
        &crate::operation::describe_environments::DescribeEnvironmentsError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_environments::DescribeEnvironmentsOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a crate::types::EnvironmentStatus>> {
        let _fld_1 = _output.environments.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::EnvironmentDescription) -> ::std::option::Option<&crate::types::EnvironmentStatus> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "Ready";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Environments[].Status","expected":"Launching","comparator":"allStringEquals"}}
pub(crate) fn match_describe_environments_5274bc882067d5d8d(
    _result: ::std::result::Result<
        &crate::operation::describe_environments::DescribeEnvironmentsOutput,
        &crate::operation::describe_environments::DescribeEnvironmentsError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_environments::DescribeEnvironmentsOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a crate::types::EnvironmentStatus>> {
        let _fld_1 = _output.environments.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::EnvironmentDescription) -> ::std::option::Option<&crate::types::EnvironmentStatus> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "Launching";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Environments[].Status","expected":"Terminated","comparator":"allStringEquals"}}
pub(crate) fn match_describe_environments_ed794ae98072768c5(
    _result: ::std::result::Result<
        &crate::operation::describe_environments::DescribeEnvironmentsOutput,
        &crate::operation::describe_environments::DescribeEnvironmentsError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_environments::DescribeEnvironmentsOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a crate::types::EnvironmentStatus>> {
        let _fld_1 = _output.environments.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::EnvironmentDescription) -> ::std::option::Option<&crate::types::EnvironmentStatus> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "Terminated";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Environments[].Status","expected":"Terminating","comparator":"allStringEquals"}}
pub(crate) fn match_describe_environments_0f48372938831eb5d(
    _result: ::std::result::Result<
        &crate::operation::describe_environments::DescribeEnvironmentsOutput,
        &crate::operation::describe_environments::DescribeEnvironmentsError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_environments::DescribeEnvironmentsOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a crate::types::EnvironmentStatus>> {
        let _fld_1 = _output.environments.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::EnvironmentDescription) -> ::std::option::Option<&crate::types::EnvironmentStatus> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "Terminating";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Environments[].Status","expected":"Updating","comparator":"allStringEquals"}}
pub(crate) fn match_describe_environments_360c965395e4199b1(
    _result: ::std::result::Result<
        &crate::operation::describe_environments::DescribeEnvironmentsOutput,
        &crate::operation::describe_environments::DescribeEnvironmentsError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_environments::DescribeEnvironmentsOutput,
    ) -> ::std::option::Option<::std::vec::Vec<&'a crate::types::EnvironmentStatus>> {
        let _fld_1 = _output.environments.as_ref()?;
        let _prj_3 = _fld_1
            .iter()
            .flat_map(|v| {
                #[allow(clippy::let_and_return)]
                fn map(_v: &crate::types::EnvironmentDescription) -> ::std::option::Option<&crate::types::EnvironmentStatus> {
                    let _fld_2 = _v.status.as_ref();
                    _fld_2
                }
                map(v)
            })
            .collect::<::std::vec::Vec<_>>();
        ::std::option::Option::Some(_prj_3)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            !value.is_empty()
                && value.iter().all(|value| {
                    let _tmp_2 = value.as_str();
                    let right = "Updating";
                    let _cmp_1 = _tmp_2 == right;
                    _cmp_1
                })
        })
        .unwrap_or_default()
}
