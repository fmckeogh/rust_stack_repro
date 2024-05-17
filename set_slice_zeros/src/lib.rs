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
use slice_mask::*;
use common::*;
pub fn set_slice_zeros<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    xs: Bits,
    i: i128,
    l: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        n: i128,
        xs: Bits,
        i: i128,
        l: i128,
    }
    let fn_state = FunctionState {
        n,
        xs,
        i,
        l,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var n:i
        let s_0_0: i128 = fn_state.n;
        // D s_0_1: read-var i:i
        let s_0_1: i128 = fn_state.i;
        // D s_0_2: read-var l:i
        let s_0_2: i128 = fn_state.l;
        // D s_0_3: call slice_mask(s_0_0, s_0_1, s_0_2)
        let s_0_3: Bits = slice_mask(state, tracer, s_0_0, s_0_1, s_0_2);
        // D s_0_4: not s_0_3
        let s_0_4: Bits = !s_0_3;
        // D s_0_5: read-var xs:bv
        let s_0_5: Bits = fn_state.xs;
        // D s_0_6: and s_0_5 s_0_4
        let s_0_6: Bits = ((s_0_5) & (s_0_4));
        // N s_0_7: return s_0_6
        return s_0_6;
    }
}
