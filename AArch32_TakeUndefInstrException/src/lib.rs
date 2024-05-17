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
use ExceptionSyndrome::*;
use AArch32_TakeUndefInstrException__1::*;
use common::*;
pub fn AArch32_TakeUndefInstrException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_10100: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_10100: (),
    }
    let fn_state = FunctionState {
        gs_10100,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // S s_0_1: call ExceptionSyndrome(s_0_0)
        let s_0_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_0_0);
        // S s_0_2: call AArch32_TakeUndefInstrException__1(s_0_1)
        let s_0_2: () = AArch32_TakeUndefInstrException__1(state, tracer, s_0_1);
        // N s_0_3: return
        return;
    }
}
