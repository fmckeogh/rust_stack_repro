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
use extzv::*;
use slice_mask::*;
use common::*;
pub fn place_slice<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i128,
    xs: Bits,
    i: i128,
    l: i128,
    shift: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        m: i128,
        xs: Bits,
        i: i128,
        l: i128,
        shift: i128,
    }
    let fn_state = FunctionState {
        m,
        xs,
        i,
        l,
        shift,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var xs:bv
        let s_0_0: Bits = fn_state.xs;
        // D s_0_1: size-of s_0_0
        let s_0_1: u16 = s_0_0.length();
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: read-var i:i
        let s_0_3: i128 = fn_state.i;
        // D s_0_4: read-var l:i
        let s_0_4: i128 = fn_state.l;
        // D s_0_5: call slice_mask(s_0_2, s_0_3, s_0_4)
        let s_0_5: Bits = slice_mask(state, tracer, s_0_2, s_0_3, s_0_4);
        // D s_0_6: read-var xs:bv
        let s_0_6: Bits = fn_state.xs;
        // D s_0_7: and s_0_6 s_0_5
        let s_0_7: Bits = ((s_0_6) & (s_0_5));
        // D s_0_8: read-var i:i
        let s_0_8: i128 = fn_state.i;
        // D s_0_9: lsr s_0_7 s_0_8
        let s_0_9: Bits = s_0_7 >> s_0_8;
        // D s_0_10: read-var m:i
        let s_0_10: i128 = fn_state.m;
        // D s_0_11: call extzv(s_0_10, s_0_9)
        let s_0_11: Bits = extzv(state, tracer, s_0_10, s_0_9);
        // D s_0_12: read-var shift:i
        let s_0_12: i128 = fn_state.shift;
        // D s_0_13: lsl s_0_11 s_0_12
        let s_0_13: Bits = s_0_11 << s_0_12;
        // N s_0_14: return s_0_13
        return s_0_13;
    }
}
