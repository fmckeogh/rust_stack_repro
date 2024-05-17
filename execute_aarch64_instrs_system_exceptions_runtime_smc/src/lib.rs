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
use AArch64_CheckForSMCUndefOrTrap::*;
use AArch64_CallSecureMonitor::*;
use common::*;
pub fn execute_aarch64_instrs_system_exceptions_runtime_smc<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm: u16,
    }
    let fn_state = FunctionState {
        imm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var imm:u16
        let s_0_0: u16 = fn_state.imm;
        // D s_0_1: call AArch64_CheckForSMCUndefOrTrap(s_0_0)
        let s_0_1: () = AArch64_CheckForSMCUndefOrTrap(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var imm:u16
        let s_1_0: u16 = fn_state.imm;
        // D s_1_1: call AArch64_CallSecureMonitor(s_1_0)
        let s_1_1: () = AArch64_CallSecureMonitor(state, tracer, s_1_0);
        // N s_1_2: return
        return;
    }
}
