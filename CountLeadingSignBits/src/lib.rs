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
use CountLeadingZeroBits::*;
use common::*;
pub fn CountLeadingSignBits<T: Tracer>(state: &mut State, tracer: &T, x: Bits) -> i128 {
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
        // C s_0_3: const #1s : i
        let s_0_3: i128 = 1;
        // D s_0_4: sub s_0_2 s_0_3
        let s_0_4: i128 = ((s_0_2) - (s_0_3));
        // C s_0_5: const #1s : i
        let s_0_5: i128 = 1;
        // D s_0_6: read-var x:bv
        let s_0_6: Bits = fn_state.x;
        // C s_0_7: const #1s : i64
        let s_0_7: i64 = 1;
        // C s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: sub s_0_4 s_0_5
        let s_0_9: i128 = ((s_0_4) - (s_0_5));
        // D s_0_10: add s_0_9 s_0_8
        let s_0_10: i128 = (s_0_9 + s_0_8);
        // D s_0_11: bit-extract s_0_6 s_0_5 s_0_10
        let s_0_11: Bits = (Bits::new(
            ((s_0_6) >> (s_0_5)).value(),
            u16::try_from(s_0_10).unwrap(),
        ));
        // D s_0_12: read-var x:bv
        let s_0_12: Bits = fn_state.x;
        // D s_0_13: size-of s_0_12
        let s_0_13: u16 = s_0_12.length();
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // C s_0_15: const #2s : i
        let s_0_15: i128 = 2;
        // D s_0_16: sub s_0_14 s_0_15
        let s_0_16: i128 = ((s_0_14) - (s_0_15));
        // C s_0_17: const #0s : i
        let s_0_17: i128 = 0;
        // D s_0_18: read-var x:bv
        let s_0_18: Bits = fn_state.x;
        // C s_0_19: const #1s : i64
        let s_0_19: i64 = 1;
        // C s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: sub s_0_16 s_0_17
        let s_0_21: i128 = ((s_0_16) - (s_0_17));
        // D s_0_22: add s_0_21 s_0_20
        let s_0_22: i128 = (s_0_21 + s_0_20);
        // D s_0_23: bit-extract s_0_18 s_0_17 s_0_22
        let s_0_23: Bits = (Bits::new(
            ((s_0_18) >> (s_0_17)).value(),
            u16::try_from(s_0_22).unwrap(),
        ));
        // D s_0_24: xor s_0_11 s_0_23
        let s_0_24: Bits = ((s_0_11) ^ (s_0_23));
        // D s_0_25: tail-call CountLeadingZeroBits(s_0_24)
        return CountLeadingZeroBits(state, tracer, s_0_24);
    }
}
