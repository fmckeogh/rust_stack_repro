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
use Replicate__1::*;
use u__id::*;
use CheckSVEEnabled::*;
use integer_subrange::*;
use Z_set::*;
use common::*;
pub fn execute_DUP_Z_I__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    imm: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_2785: i64,
        esizeshadow_2784: i64,
        VL: i64,
        d: i64,
        esize: i64,
        imm: i128,
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
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#2784 <= s_0_2
        fn_state.esizeshadow_2784 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2785 <= s_0_6
        fn_state.VLshadow_2785 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2785:i64
        let s_1_0: i64 = fn_state.VLshadow_2785;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call __id(s_1_1)
        let s_1_2: i128 = u__id(state, tracer, s_1_1);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #1s : i
        let s_1_4: i128 = 1;
        // D s_1_5: read-var esizeshadow#2784:i64
        let s_1_5: i64 = fn_state.esizeshadow_2784;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: sub s_1_6 s_1_4
        let s_1_7: i128 = ((s_1_6) - (s_1_4));
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // C s_1_9: const #0s : i
        let s_1_9: i128 = 0;
        // D s_1_10: cast zx s_1_8 -> i
        let s_1_10: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_11: read-var imm:i
        let s_1_11: i128 = fn_state.imm;
        // D s_1_12: call integer_subrange(s_1_11, s_1_10, s_1_9)
        let s_1_12: Bits = integer_subrange(state, tracer, s_1_11, s_1_10, s_1_9);
        // D s_1_13: cast zx s_1_3 -> i
        let s_1_13: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_14: call Replicate__1(s_1_13, s_1_12)
        let s_1_14: Bits = Replicate__1(state, tracer, s_1_13, s_1_12);
        // D s_1_15: cast zx s_1_0 -> i
        let s_1_15: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var d:i64
        let s_1_17: i64 = fn_state.d;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast zx s_1_16 -> i
        let s_1_19: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_20: call Z_set(s_1_18, s_1_19, s_1_14)
        let s_1_20: () = Z_set(state, tracer, s_1_18, s_1_19, s_1_14);
        // N s_1_21: return
        return;
    }
}
