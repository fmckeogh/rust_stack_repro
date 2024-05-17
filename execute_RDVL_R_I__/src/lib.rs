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
use CheckSVEEnabled::*;
use X_set::*;
use integer_subrange::*;
use common::*;
pub fn execute_RDVL_R_I__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    imm: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VL: i64,
        d: i64,
        imm: i128,
    }
    let fn_state = FunctionState {
        VL,
        d,
        imm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckSVEEnabled(s_0_0)
        let s_0_1: () = CheckSVEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VL:i64
        let s_1_0: i64 = fn_state.VL;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: read-var imm:i
        let s_1_6: i128 = fn_state.imm;
        // D s_1_7: mul s_1_6 s_1_5
        let s_1_7: i128 = ((s_1_6) * (s_1_5));
        // C s_1_8: const #64s : i64
        let s_1_8: i64 = 64;
        // C s_1_9: const #63s : i
        let s_1_9: i128 = 63;
        // C s_1_10: const #0s : i
        let s_1_10: i128 = 0;
        // D s_1_11: call integer_subrange(s_1_7, s_1_9, s_1_10)
        let s_1_11: Bits = integer_subrange(state, tracer, s_1_7, s_1_9, s_1_10);
        // D s_1_12: cast reint s_1_11 -> u64
        let s_1_12: u64 = (s_1_11.value() as u64);
        // D s_1_13: read-var d:i64
        let s_1_13: i64 = fn_state.d;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast zx s_1_12 -> bv
        let s_1_15: Bits = Bits::new(s_1_12 as u128, 64u16);
        // D s_1_16: call X_set(s_1_14, s_1_8, s_1_15)
        let s_1_16: () = X_set(state, tracer, s_1_14, s_1_8, s_1_15);
        // N s_1_17: return
        return;
    }
}
