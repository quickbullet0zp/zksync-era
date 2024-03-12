use serde::{Deserialize, Serialize};
use zksync_basic_types::web3::types::{Bytes, U256};

use crate::{
    api::{DebugCall, DebugCallType, ResultDebugCall},
    Address,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugCallFlat {
    pub action: Action,
    pub result: CallResult,
    pub subtraces: usize,
    pub traceaddress: Vec<usize>,
    pub error: Option<String>,
    pub revert_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub r#type: DebugCallType,
    pub from: Address,
    pub to: Address,
    pub gas: U256,
    pub value: U256,
    pub input: Bytes,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallResult {
    pub output: Bytes,
    pub gas_used: U256,
}

pub fn flatten_debug_calls(calls: Vec<ResultDebugCall>) -> Vec<DebugCallFlat> {
    let mut flattened_calls = Vec::new();
    for (index, result_debug_call) in calls.into_iter().enumerate() {
        let mut trace_address = vec![index]; // Initialize the trace addressees with the index of the top-level call
        flatten_call_recursive(
            &result_debug_call.result,
            &mut flattened_calls,
            &mut trace_address,
        );
    }
    flattened_calls
}

fn flatten_call_recursive(
    call: &DebugCall,
    flattened_calls: &mut Vec<DebugCallFlat>,
    trace_address: &mut Vec<usize>,
) {
    let flat_call = DebugCallFlat {
        action: Action {
            r#type: call.r#type.clone(),
            from: call.from,
            to: call.to,
            gas: call.gas,
            value: call.value,
            input: call.input.clone(),
        },
        result: CallResult {
            output: call.output.clone(),
            gas_used: call.gas_used,
        },
        subtraces: call.calls.len(),
        traceaddress: trace_address.clone(), // Clone the current trace address
        error: call.error.clone(),
        revert_reason: call.revert_reason.clone(),
    };
    flattened_calls.push(flat_call);

    // Process nested calls
    for (index, nested_call) in call.calls.iter().enumerate() {
        trace_address.push(index); // Update trace addressees for the nested call
        flatten_call_recursive(nested_call, flattened_calls, trace_address);
        trace_address.pop(); // Reset trace addressees after processing the nested call (prevent to keep filling the vector)
    }
}
