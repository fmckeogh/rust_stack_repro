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
pub fn place_subrange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i128,
    xs: Bits,
    i: i128,
    j: i128,
    shift: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        m: i128,
        xs: Bits,
        i: i128,
        j: i128,
        shift: i128,
    }
    let fn_state = FunctionState {
        m,
        xs,
        i,
        j,
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
        // D s_0_4: read-var j:i
        let s_0_4: i128 = fn_state.j;
        // D s_0_5: sub s_0_3 s_0_4
        let s_0_5: i128 = ((s_0_3) - (s_0_4));
        // C s_0_6: const #1s : i
        let s_0_6: i128 = 1;
        // D s_0_7: add s_0_5 s_0_6
        let s_0_7: i128 = (s_0_5 + s_0_6);
        // D s_0_8: read-var j:i
        let s_0_8: i128 = fn_state.j;
        // D s_0_9: call slice_mask(s_0_2, s_0_8, s_0_7)
        let s_0_9: Bits = slice_mask(state, tracer, s_0_2, s_0_8, s_0_7);
        // D s_0_10: read-var xs:bv
        let s_0_10: Bits = fn_state.xs;
        // D s_0_11: and s_0_10 s_0_9
        let s_0_11: Bits = ((s_0_10) & (s_0_9));
        // D s_0_12: read-var j:i
        let s_0_12: i128 = fn_state.j;
        // D s_0_13: lsr s_0_11 s_0_12
        let s_0_13: Bits = s_0_11 >> s_0_12;
        // D s_0_14: read-var m:i
        let s_0_14: i128 = fn_state.m;
        // D s_0_15: call extzv(s_0_14, s_0_13)
        let s_0_15: Bits = extzv(state, tracer, s_0_14, s_0_13);
        // D s_0_16: read-var shift:i
        let s_0_16: i128 = fn_state.shift;
        // D s_0_17: lsl s_0_15 s_0_16
        let s_0_17: Bits = s_0_15 << s_0_16;
        // N s_0_18: return s_0_17
        return s_0_17;
    }
}
