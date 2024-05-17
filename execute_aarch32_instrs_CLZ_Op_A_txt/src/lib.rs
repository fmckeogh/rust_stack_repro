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
use R_read::*;
use CountLeadingZeroBits::*;
use R_set::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch32_instrs_CLZ_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: call CountLeadingZeroBits(s_0_3)
        let s_0_4: i128 = CountLeadingZeroBits(state, tracer, s_0_3);
        // C s_0_5: const #31s : i
        let s_0_5: i128 = 31;
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // D s_0_7: call integer_subrange(s_0_4, s_0_5, s_0_6)
        let s_0_7: Bits = integer_subrange(state, tracer, s_0_4, s_0_5, s_0_6);
        // D s_0_8: cast reint s_0_7 -> u32
        let s_0_8: u32 = (s_0_7.value() as u32);
        // D s_0_9: read-var d:i64
        let s_0_9: i64 = fn_state.d;
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_11: call R_set(s_0_10, s_0_8)
        let s_0_11: () = R_set(state, tracer, s_0_10, s_0_8);
        // N s_0_12: return
        return;
    }
}
