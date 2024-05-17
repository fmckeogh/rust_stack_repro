#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use execute_aarch64_instrs_system_exceptions_runtime_hvc::*;
use common::*;
pub fn decode_hvc_aarch64_instrs_system_exceptions_runtime_hvc<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm16: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm16: u16,
    }
    let fn_state = FunctionState {
        imm16,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var imm16:u16
        let s_0_0: u16 = fn_state.imm16;
        // D s_0_1: call execute_aarch64_instrs_system_exceptions_runtime_hvc(s_0_0)
        let s_0_1: () = execute_aarch64_instrs_system_exceptions_runtime_hvc(
            state,
            tracer,
            s_0_0,
        );
        // N s_0_2: return
        return;
    }
}
