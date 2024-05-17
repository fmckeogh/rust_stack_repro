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
use TakePendingInterrupts::*;
use common::*;
pub fn TakeUnmaskedSErrorInterrupts<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_327616: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        interrupt_req: ProductTypefe062afb059b3bbc,
        gs_327616: (),
    }
    let fn_state = FunctionState {
        gs_327616,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: write-var interrupt_req.3 <= s_0_0
        fn_state.interrupt_req._3 = s_0_0;
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // D s_0_3: write-var interrupt_req.6 <= s_0_2
        fn_state.interrupt_req._6 = s_0_2;
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // D s_0_5: write-var interrupt_req.1 <= s_0_4
        fn_state.interrupt_req._1 = s_0_4;
        // C s_0_6: const #0u : u8
        let s_0_6: bool = false;
        // D s_0_7: write-var interrupt_req.4 <= s_0_6
        fn_state.interrupt_req._4 = s_0_6;
        // C s_0_8: const #0u : u8
        let s_0_8: bool = false;
        // D s_0_9: write-var interrupt_req.2 <= s_0_8
        fn_state.interrupt_req._2 = s_0_8;
        // C s_0_10: const #0u : u8
        let s_0_10: bool = false;
        // D s_0_11: write-var interrupt_req.5 <= s_0_10
        fn_state.interrupt_req._5 = s_0_10;
        // C s_0_12: const #0u : u8
        let s_0_12: bool = false;
        // D s_0_13: write-var interrupt_req.0 <= s_0_12
        fn_state.interrupt_req._0 = s_0_12;
        // C s_0_14: const #1u : u8
        let s_0_14: bool = true;
        // D s_0_15: write-var interrupt_req.3 <= s_0_14
        fn_state.interrupt_req._3 = s_0_14;
        // C s_0_16: const #1u : u8
        let s_0_16: bool = true;
        // D s_0_17: write-var interrupt_req.6 <= s_0_16
        fn_state.interrupt_req._6 = s_0_16;
        // D s_0_18: read-var interrupt_req:struct
        let s_0_18: ProductTypefe062afb059b3bbc = fn_state.interrupt_req;
        // D s_0_19: call TakePendingInterrupts(s_0_18)
        let s_0_19: bool = TakePendingInterrupts(state, tracer, s_0_18);
        // N s_0_20: return
        return;
    }
}
