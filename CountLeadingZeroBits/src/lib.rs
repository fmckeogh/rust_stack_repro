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
use HighestSetBit::*;
use common::*;
pub fn CountLeadingZeroBits<T: Tracer>(state: &mut State, tracer: &T, x: Bits) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        x: Bits,
    }
    let fn_state = FunctionState {
        x,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_0_0: read-var x:bv
        let s_0_0: Bits = fn_state.x;
        // D s_0_1: size-of s_0_0
        let s_0_1: u16 = s_0_0.length();
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: read-var x:bv
        let s_0_3: Bits = fn_state.x;
        // D s_0_4: call HighestSetBit(s_0_3)
        let s_0_4: i128 = HighestSetBit(state, tracer, s_0_3);
        // C s_0_5: const #1s : i
        let s_0_5: i128 = 1;
        // D s_0_6: add s_0_4 s_0_5
        let s_0_6: i128 = (s_0_4 + s_0_5);
        // D s_0_7: sub s_0_2 s_0_6
        let s_0_7: i128 = ((s_0_2) - (s_0_6));
        // N s_0_8: return s_0_7
        return s_0_7;
    }
}
