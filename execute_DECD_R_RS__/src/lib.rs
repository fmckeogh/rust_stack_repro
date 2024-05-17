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
use X_read::*;
use CheckSVEEnabled::*;
use common::*;
pub fn execute_DECD_R_RS__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    imm: i64,
    pat: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VL: i64,
        dn: i64,
        esize: i64,
        imm: i64,
        pat: u8,
    }
    let fn_state = FunctionState {
        VL,
        dn,
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
        // D s_1_5: read-var dn:i64
        let s_1_5: i64 = fn_state.dn;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: call X_read(s_1_6, s_1_4)
        let s_1_7: Bits = X_read(state, tracer, s_1_6, s_1_4);
        // D s_1_8: cast reint s_1_7 -> u64
        let s_1_8: u64 = (s_1_7.value() as u64);
        // C s_1_9: const #64s : i64
        let s_1_9: i64 = 64;
        // D s_1_10: read-var imm:i64
        let s_1_10: i64 = fn_state.imm;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: mul s_1_3 s_1_11
        let s_1_12: i128 = ((s_1_3) * (s_1_11));
        // D s_1_13: cast zx s_1_8 -> bv
        let s_1_13: Bits = Bits::new(s_1_8 as u128, 64u16);
        // D s_1_14: cast cvt s_1_12 -> bv
        let s_1_14: Bits = Bits::new(s_1_12 as u128, 128);
        // D s_1_15: sub s_1_13 s_1_14
        let s_1_15: Bits = ((s_1_13) - (s_1_14));
        // D s_1_16: cast reint s_1_15 -> u64
        let s_1_16: u64 = (s_1_15.value() as u64);
        // D s_1_17: read-var dn:i64
        let s_1_17: i64 = fn_state.dn;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast zx s_1_16 -> bv
        let s_1_19: Bits = Bits::new(s_1_16 as u128, 64u16);
        // D s_1_20: call X_set(s_1_18, s_1_9, s_1_19)
        let s_1_20: () = X_set(state, tracer, s_1_18, s_1_9, s_1_19);
        // N s_1_21: return
        return;
    }
}
