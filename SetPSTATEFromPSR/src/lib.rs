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
use SetPSTATEFromPSR__1::*;
use IllegalExceptionReturn::*;
use common::*;
pub fn SetPSTATEFromPSR<T: Tracer>(state: &mut State, tracer: &T, spsr: Bits) -> () {
    #[derive(Default)]
    struct FunctionState {
        spsr: Bits,
    }
    let fn_state = FunctionState {
        spsr,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var spsr:bv
        let s_0_0: Bits = fn_state.spsr;
        // D s_0_1: call IllegalExceptionReturn(s_0_0)
        let s_0_1: bool = IllegalExceptionReturn(state, tracer, s_0_0);
        // D s_0_2: read-var spsr:bv
        let s_0_2: Bits = fn_state.spsr;
        // D s_0_3: call SetPSTATEFromPSR__1(s_0_2, s_0_1)
        let s_0_3: () = SetPSTATEFromPSR__1(state, tracer, s_0_2, s_0_1);
        // N s_0_4: return
        return;
    }
}
