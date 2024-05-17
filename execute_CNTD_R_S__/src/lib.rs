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
use X_set::*;
use DecodePredCount::*;
use CheckSVEEnabled::*;
use integer_subrange::*;
use common::*;
pub fn execute_CNTD_R_S__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    imm: i64,
    pat: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        esize: i64,
        imm: i64,
        pat: u8,
    }
    let fn_state = FunctionState {
        d,
        esize,
        imm,
        pat,
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
        // D s_1_0: read-var esize:i64
        let s_1_0: i64 = fn_state.esize;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: read-var pat:u8
        let s_1_2: u8 = fn_state.pat;
        // D s_1_3: call DecodePredCount(s_1_2, s_1_1)
        let s_1_3: i128 = DecodePredCount(state, tracer, s_1_2, s_1_1);
        // C s_1_4: const #64s : i64
        let s_1_4: i64 = 64;
        // D s_1_5: read-var imm:i64
        let s_1_5: i64 = fn_state.imm;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: mul s_1_3 s_1_6
        let s_1_7: i128 = ((s_1_3) * (s_1_6));
        // C s_1_8: const #63s : i
        let s_1_8: i128 = 63;
        // C s_1_9: const #0s : i
        let s_1_9: i128 = 0;
        // D s_1_10: call integer_subrange(s_1_7, s_1_8, s_1_9)
        let s_1_10: Bits = integer_subrange(state, tracer, s_1_7, s_1_8, s_1_9);
        // D s_1_11: cast reint s_1_10 -> u64
        let s_1_11: u64 = (s_1_10.value() as u64);
        // D s_1_12: read-var d:i64
        let s_1_12: i64 = fn_state.d;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast zx s_1_11 -> bv
        let s_1_14: Bits = Bits::new(s_1_11 as u128, 64u16);
        // D s_1_15: call X_set(s_1_13, s_1_4, s_1_14)
        let s_1_15: () = X_set(state, tracer, s_1_13, s_1_4, s_1_14);
        // N s_1_16: return
        return;
    }
}
