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
use Z_set::*;
use replicate_bits_borealis_internal::*;
use common::*;
pub fn execute_DUPM_Z_I__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    imm: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_2796: i64,
        VL: i64,
        d: i64,
        esize: i64,
        imm: Bits,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        imm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#2796 <= s_0_2
        fn_state.VLshadow_2796 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSVEEnabled(s_0_4)
        let s_0_5: () = CheckSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2796:i64
        let s_1_0: i64 = fn_state.VLshadow_2796;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: read-var esize:i64
        let s_1_2: i64 = fn_state.esize;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: div s_1_1 s_1_3
        let s_1_4: i128 = ((s_1_1) / (s_1_3));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: read-var imm:bv
        let s_1_7: Bits = fn_state.imm;
        // D s_1_8: cast reint s_1_6 -> u64
        let s_1_8: u64 = (s_1_6 as u64);
        // D s_1_9: call replicate_bits_borealis_internal(s_1_7, s_1_8)
        let s_1_9: Bits = replicate_bits_borealis_internal(state, tracer, s_1_7, s_1_8);
        // D s_1_10: cast zx s_1_0 -> i
        let s_1_10: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var d:i64
        let s_1_12: i64 = fn_state.d;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast zx s_1_11 -> i
        let s_1_14: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_15: call Z_set(s_1_13, s_1_14, s_1_9)
        let s_1_15: () = Z_set(state, tracer, s_1_13, s_1_14, s_1_9);
        // N s_1_16: return
        return;
    }
}
