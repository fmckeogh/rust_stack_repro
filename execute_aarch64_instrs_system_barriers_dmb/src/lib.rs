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
use DataMemoryBarrier::*;
use common::*;
pub fn execute_aarch64_instrs_system_barriers_dmb<T: Tracer>(
    state: &mut State,
    tracer: &T,
    domain: u32,
    types: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        domain: u32,
        types: u32,
    }
    let fn_state = FunctionState {
        domain,
        types,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var domain:u32
        let s_0_0: u32 = fn_state.domain;
        // D s_0_1: read-var types:u32
        let s_0_1: u32 = fn_state.types;
        // D s_0_2: call DataMemoryBarrier(s_0_0, s_0_1)
        let s_0_2: () = DataMemoryBarrier(state, tracer, s_0_0, s_0_1);
        // N s_0_3: return
        return;
    }
}
