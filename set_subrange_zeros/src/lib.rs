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
use set_slice_zeros::*;
use common::*;
pub fn set_subrange_zeros<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    xs: Bits,
    hi: i128,
    lo: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        n: i128,
        xs: Bits,
        hi: i128,
        lo: i128,
    }
    let fn_state = FunctionState {
        n,
        xs,
        hi,
        lo,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var hi:i
        let s_0_0: i128 = fn_state.hi;
        // D s_0_1: read-var lo:i
        let s_0_1: i128 = fn_state.lo;
        // D s_0_2: sub s_0_0 s_0_1
        let s_0_2: i128 = ((s_0_0) - (s_0_1));
        // C s_0_3: const #1s : i
        let s_0_3: i128 = 1;
        // D s_0_4: add s_0_2 s_0_3
        let s_0_4: i128 = (s_0_2 + s_0_3);
        // D s_0_5: read-var n:i
        let s_0_5: i128 = fn_state.n;
        // D s_0_6: read-var xs:bv
        let s_0_6: Bits = fn_state.xs;
        // D s_0_7: read-var lo:i
        let s_0_7: i128 = fn_state.lo;
        // D s_0_8: tail-call set_slice_zeros(s_0_5, s_0_6, s_0_7, s_0_4)
        return set_slice_zeros(state, tracer, s_0_5, s_0_6, s_0_7, s_0_4);
    }
}
