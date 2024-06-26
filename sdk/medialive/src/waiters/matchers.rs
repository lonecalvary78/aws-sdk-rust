// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"output":{"path":"State","expected":"IDLE","comparator":"stringEquals"}}
pub(crate) fn match_describe_channel_00b43f5a154c428ea(
    _result: ::std::result::Result<
        &crate::operation::describe_channel::DescribeChannelOutput,
        &crate::operation::describe_channel::DescribeChannelError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_channel::DescribeChannelOutput,
    ) -> ::std::option::Option<&'a crate::types::ChannelState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "IDLE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"CREATING","comparator":"stringEquals"}}
pub(crate) fn match_describe_channel_8866bdf154f29ea53(
    _result: ::std::result::Result<
        &crate::operation::describe_channel::DescribeChannelOutput,
        &crate::operation::describe_channel::DescribeChannelError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_channel::DescribeChannelOutput,
    ) -> ::std::option::Option<&'a crate::types::ChannelState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"InternalServerErrorException"}
pub(crate) fn match_describe_channel_23a4ee68df28eed70(
    _result: ::std::result::Result<
        &crate::operation::describe_channel::DescribeChannelOutput,
        &crate::operation::describe_channel::DescribeChannelError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "InternalServerErrorException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"State","expected":"CREATE_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_describe_channel_8659e07b86810791e(
    _result: ::std::result::Result<
        &crate::operation::describe_channel::DescribeChannelOutput,
        &crate::operation::describe_channel::DescribeChannelError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_channel::DescribeChannelOutput,
    ) -> ::std::option::Option<&'a crate::types::ChannelState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATE_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"DELETED","comparator":"stringEquals"}}
pub(crate) fn match_describe_channel_dae4612b5507a5589(
    _result: ::std::result::Result<
        &crate::operation::describe_channel::DescribeChannelOutput,
        &crate::operation::describe_channel::DescribeChannelError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_channel::DescribeChannelOutput,
    ) -> ::std::option::Option<&'a crate::types::ChannelState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"DELETING","comparator":"stringEquals"}}
pub(crate) fn match_describe_channel_788667cd7ad1cc1c2(
    _result: ::std::result::Result<
        &crate::operation::describe_channel::DescribeChannelOutput,
        &crate::operation::describe_channel::DescribeChannelError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_channel::DescribeChannelOutput,
    ) -> ::std::option::Option<&'a crate::types::ChannelState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"RUNNING","comparator":"stringEquals"}}
pub(crate) fn match_describe_channel_4ad6a60d9048c7d92(
    _result: ::std::result::Result<
        &crate::operation::describe_channel::DescribeChannelOutput,
        &crate::operation::describe_channel::DescribeChannelError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_channel::DescribeChannelOutput,
    ) -> ::std::option::Option<&'a crate::types::ChannelState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "RUNNING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"STARTING","comparator":"stringEquals"}}
pub(crate) fn match_describe_channel_05ea55539c40bdd50(
    _result: ::std::result::Result<
        &crate::operation::describe_channel::DescribeChannelOutput,
        &crate::operation::describe_channel::DescribeChannelError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_channel::DescribeChannelOutput,
    ) -> ::std::option::Option<&'a crate::types::ChannelState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "STARTING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"STOPPING","comparator":"stringEquals"}}
pub(crate) fn match_describe_channel_b656d56ada3218574(
    _result: ::std::result::Result<
        &crate::operation::describe_channel::DescribeChannelOutput,
        &crate::operation::describe_channel::DescribeChannelError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_channel::DescribeChannelOutput,
    ) -> ::std::option::Option<&'a crate::types::ChannelState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "STOPPING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"ATTACHED","comparator":"stringEquals"}}
pub(crate) fn match_describe_input_ac3862e9a8fbc2096(
    _result: ::std::result::Result<&crate::operation::describe_input::DescribeInputOutput, &crate::operation::describe_input::DescribeInputError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_input::DescribeInputOutput) -> ::std::option::Option<&'a crate::types::InputState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "ATTACHED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"DETACHED","comparator":"stringEquals"}}
pub(crate) fn match_describe_input_04acddec8d4cca918(
    _result: ::std::result::Result<&crate::operation::describe_input::DescribeInputOutput, &crate::operation::describe_input::DescribeInputError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_input::DescribeInputOutput) -> ::std::option::Option<&'a crate::types::InputState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DETACHED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"InternalServerErrorException"}
pub(crate) fn match_describe_input_23a4ee68df28eed70(
    _result: ::std::result::Result<&crate::operation::describe_input::DescribeInputOutput, &crate::operation::describe_input::DescribeInputError>,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "InternalServerErrorException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"State","expected":"DELETED","comparator":"stringEquals"}}
pub(crate) fn match_describe_input_dae4612b5507a5589(
    _result: ::std::result::Result<&crate::operation::describe_input::DescribeInputOutput, &crate::operation::describe_input::DescribeInputError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_input::DescribeInputOutput) -> ::std::option::Option<&'a crate::types::InputState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"DELETING","comparator":"stringEquals"}}
pub(crate) fn match_describe_input_788667cd7ad1cc1c2(
    _result: ::std::result::Result<&crate::operation::describe_input::DescribeInputOutput, &crate::operation::describe_input::DescribeInputError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_input::DescribeInputOutput) -> ::std::option::Option<&'a crate::types::InputState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"CREATING","comparator":"stringEquals"}}
pub(crate) fn match_describe_input_8866bdf154f29ea53(
    _result: ::std::result::Result<&crate::operation::describe_input::DescribeInputOutput, &crate::operation::describe_input::DescribeInputError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::describe_input::DescribeInputOutput) -> ::std::option::Option<&'a crate::types::InputState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"IDLE","comparator":"stringEquals"}}
pub(crate) fn match_describe_multiplex_00b43f5a154c428ea(
    _result: ::std::result::Result<
        &crate::operation::describe_multiplex::DescribeMultiplexOutput,
        &crate::operation::describe_multiplex::DescribeMultiplexError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_multiplex::DescribeMultiplexOutput,
    ) -> ::std::option::Option<&'a crate::types::MultiplexState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "IDLE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"CREATING","comparator":"stringEquals"}}
pub(crate) fn match_describe_multiplex_8866bdf154f29ea53(
    _result: ::std::result::Result<
        &crate::operation::describe_multiplex::DescribeMultiplexOutput,
        &crate::operation::describe_multiplex::DescribeMultiplexError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_multiplex::DescribeMultiplexOutput,
    ) -> ::std::option::Option<&'a crate::types::MultiplexState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"InternalServerErrorException"}
pub(crate) fn match_describe_multiplex_23a4ee68df28eed70(
    _result: ::std::result::Result<
        &crate::operation::describe_multiplex::DescribeMultiplexOutput,
        &crate::operation::describe_multiplex::DescribeMultiplexError,
    >,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "InternalServerErrorException";
        }
    }
    false
}

/// Matcher union: {"output":{"path":"State","expected":"CREATE_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_describe_multiplex_8659e07b86810791e(
    _result: ::std::result::Result<
        &crate::operation::describe_multiplex::DescribeMultiplexOutput,
        &crate::operation::describe_multiplex::DescribeMultiplexError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_multiplex::DescribeMultiplexOutput,
    ) -> ::std::option::Option<&'a crate::types::MultiplexState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATE_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"DELETED","comparator":"stringEquals"}}
pub(crate) fn match_describe_multiplex_dae4612b5507a5589(
    _result: ::std::result::Result<
        &crate::operation::describe_multiplex::DescribeMultiplexOutput,
        &crate::operation::describe_multiplex::DescribeMultiplexError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_multiplex::DescribeMultiplexOutput,
    ) -> ::std::option::Option<&'a crate::types::MultiplexState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"DELETING","comparator":"stringEquals"}}
pub(crate) fn match_describe_multiplex_788667cd7ad1cc1c2(
    _result: ::std::result::Result<
        &crate::operation::describe_multiplex::DescribeMultiplexOutput,
        &crate::operation::describe_multiplex::DescribeMultiplexError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_multiplex::DescribeMultiplexOutput,
    ) -> ::std::option::Option<&'a crate::types::MultiplexState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"RUNNING","comparator":"stringEquals"}}
pub(crate) fn match_describe_multiplex_4ad6a60d9048c7d92(
    _result: ::std::result::Result<
        &crate::operation::describe_multiplex::DescribeMultiplexOutput,
        &crate::operation::describe_multiplex::DescribeMultiplexError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_multiplex::DescribeMultiplexOutput,
    ) -> ::std::option::Option<&'a crate::types::MultiplexState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "RUNNING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"STARTING","comparator":"stringEquals"}}
pub(crate) fn match_describe_multiplex_05ea55539c40bdd50(
    _result: ::std::result::Result<
        &crate::operation::describe_multiplex::DescribeMultiplexOutput,
        &crate::operation::describe_multiplex::DescribeMultiplexError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_multiplex::DescribeMultiplexOutput,
    ) -> ::std::option::Option<&'a crate::types::MultiplexState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "STARTING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"State","expected":"STOPPING","comparator":"stringEquals"}}
pub(crate) fn match_describe_multiplex_b656d56ada3218574(
    _result: ::std::result::Result<
        &crate::operation::describe_multiplex::DescribeMultiplexOutput,
        &crate::operation::describe_multiplex::DescribeMultiplexError,
    >,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::describe_multiplex::DescribeMultiplexOutput,
    ) -> ::std::option::Option<&'a crate::types::MultiplexState> {
        let _fld_1 = _output.state.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "STOPPING";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Status","expected":"CREATE_COMPLETE","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_ee24601e746b68869(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapStatus> {
        let _fld_1 = _output.status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATE_COMPLETE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Status","expected":"CREATE_IN_PROGRESS","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_fd38fc0610d7a1d7e(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapStatus> {
        let _fld_1 = _output.status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATE_IN_PROGRESS";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Status","expected":"CREATE_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_22c66a56e806df25e(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapStatus> {
        let _fld_1 = _output.status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "CREATE_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"MonitorDeployment.Status","expected":"DELETE_COMPLETE","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_037e38367de27324e(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapMonitorDeploymentStatus> {
        let _fld_1 = _output.monitor_deployment.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETE_COMPLETE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"MonitorDeployment.Status","expected":"DELETE_IN_PROGRESS","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_6acaa12bd2a1a8f10(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapMonitorDeploymentStatus> {
        let _fld_1 = _output.monitor_deployment.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETE_IN_PROGRESS";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"MonitorDeployment.Status","expected":"DELETE_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_f9db6b67f2714e3dc(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapMonitorDeploymentStatus> {
        let _fld_1 = _output.monitor_deployment.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DELETE_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"MonitorDeployment.Status","expected":"DRY_RUN_DEPLOYMENT_COMPLETE","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_37ac2c1c3bff11e11(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapMonitorDeploymentStatus> {
        let _fld_1 = _output.monitor_deployment.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DRY_RUN_DEPLOYMENT_COMPLETE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"MonitorDeployment.Status","expected":"DEPLOYMENT_COMPLETE","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_2bba0822f48f2e4b7(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapMonitorDeploymentStatus> {
        let _fld_1 = _output.monitor_deployment.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DEPLOYMENT_COMPLETE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"MonitorDeployment.Status","expected":"DRY_RUN_DEPLOYMENT_IN_PROGRESS","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_84ce42f94df967eaa(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapMonitorDeploymentStatus> {
        let _fld_1 = _output.monitor_deployment.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DRY_RUN_DEPLOYMENT_IN_PROGRESS";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"MonitorDeployment.Status","expected":"DEPLOYMENT_IN_PROGRESS","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_a1a4b253b0344e3c2(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapMonitorDeploymentStatus> {
        let _fld_1 = _output.monitor_deployment.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DEPLOYMENT_IN_PROGRESS";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"MonitorDeployment.Status","expected":"DRY_RUN_DEPLOYMENT_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_b43e22c14edb507f5(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapMonitorDeploymentStatus> {
        let _fld_1 = _output.monitor_deployment.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DRY_RUN_DEPLOYMENT_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"MonitorDeployment.Status","expected":"DEPLOYMENT_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_5a9dc9d12b328f5b7(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapMonitorDeploymentStatus> {
        let _fld_1 = _output.monitor_deployment.as_ref()?;
        let _fld_2 = _fld_1.status.as_ref()?;
        ::std::option::Option::Some(_fld_2)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "DEPLOYMENT_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Status","expected":"UPDATE_COMPLETE","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_7ae6032346259aca1(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapStatus> {
        let _fld_1 = _output.status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "UPDATE_COMPLETE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Status","expected":"UPDATE_IN_PROGRESS","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_d58ed03d2e3493f72(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapStatus> {
        let _fld_1 = _output.status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "UPDATE_IN_PROGRESS";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Status","expected":"UPDATE_FAILED","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_ad70565119d7f8338(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapStatus> {
        let _fld_1 = _output.status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "UPDATE_FAILED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"output":{"path":"Status","expected":"UPDATE_REVERTED","comparator":"stringEquals"}}
pub(crate) fn match_get_signal_map_7f7f76acdbc8290f8(
    _result: ::std::result::Result<&crate::operation::get_signal_map::GetSignalMapOutput, &crate::operation::get_signal_map::GetSignalMapError>,
) -> bool {
    fn path_traversal<'a>(
        _output: &'a crate::operation::get_signal_map::GetSignalMapOutput,
    ) -> ::std::option::Option<&'a crate::types::SignalMapStatus> {
        let _fld_1 = _output.status.as_ref()?;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "UPDATE_REVERTED";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}
