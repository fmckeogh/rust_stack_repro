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
use ImpDefBits::*;
use common::*;
pub fn u__IMPDEF_bits<T: Tracer>(
    state: &mut State,
    tracer: &T,
    N: i128,
    x: &'static str,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        N: i128,
        x: &'static str,
    }
    let fn_state = FunctionState {
        N,
        x,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var N:i
        let s_0_0: i128 = fn_state.N;
        // D s_0_1: read-var x:str
        let s_0_1: &'static str = fn_state.x;
        // D s_0_2: tail-call ImpDefBits(s_0_1, s_0_0)
        return ImpDefBits(state, tracer, s_0_1, s_0_0);
    }
}
