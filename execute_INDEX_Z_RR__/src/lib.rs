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
use X_read::*;
use Elem_set::*;
use CheckSVEEnabled::*;
use integer_subrange::*;
use Z_set::*;
use common::*;
pub fn execute_INDEX_Z_RR__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: i64,
        e: i64,
        element1: i64,
        VLshadow_2859: i64,
        gs_194130: i64,
        esizeshadow_2858: i64,
        VLshadow_2860: i64,
        result: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        n,
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
        // D s_0_3: write-var esizeshadow#2858 <= s_0_2
        fn_state.esizeshadow_2858 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2859 <= s_0_6
        fn_state.VLshadow_2859 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2859:i64
        let s_1_0: i64 = fn_state.VLshadow_2859;
        // D s_1_1: write-var VLshadow#2860 <= s_1_0
        fn_state.VLshadow_2860 = s_1_0;
        // D s_1_2: read-var VLshadow#2860:i64
        let s_1_2: i64 = fn_state.VLshadow_2860;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#2858:i64
        let s_1_4: i64 = fn_state.esizeshadow_2858;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var esizeshadow#2858:i64
        let s_1_8: i64 = fn_state.esizeshadow_2858;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var n:i64
        let s_1_11: i64 = fn_state.n;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call X_read(s_1_12, s_1_10)
        let s_1_13: Bits = X_read(state, tracer, s_1_12, s_1_10);
        // D s_1_14: cast sx s_1_13 -> i
        let s_1_14: i128 = {
            let sign_bit = s_1_13.length() - 1;
            let mut result = s_1_13.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: write-var element1 <= s_1_15
        fn_state.element1 = s_1_15;
        // D s_1_17: read-var esizeshadow#2858:i64
        let s_1_17: i64 = fn_state.esizeshadow_2858;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: read-var m:i64
        let s_1_20: i64 = fn_state.m;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: call X_read(s_1_21, s_1_19)
        let s_1_22: Bits = X_read(state, tracer, s_1_21, s_1_19);
        // D s_1_23: cast sx s_1_22 -> i
        let s_1_23: i128 = {
            let sign_bit = s_1_22.length() - 1;
            let mut result = s_1_22.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: write-var element2 <= s_1_24
        fn_state.element2 = s_1_24;
        // C s_1_26: const #0s : i64
        let s_1_26: i64 = 0;
        // C s_1_27: const #1s : i
        let s_1_27: i128 = 1;
        // D s_1_28: cast zx s_1_7 -> i
        let s_1_28: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_29: sub s_1_28 s_1_27
        let s_1_29: i128 = ((s_1_28) - (s_1_27));
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: write-var gs#194130 <= s_1_30
        fn_state.gs_194130 = s_1_30;
        // D s_1_32: write-var e <= s_1_26
        fn_state.e = s_1_26;
        // N s_1_33: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#194130:i64
        let s_2_1: i64 = fn_state.gs_194130;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var element2:i64
        let s_3_2: i64 = fn_state.element2;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: mul s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) * (s_3_3));
        // D s_3_5: read-var element1:i64
        let s_3_5: i64 = fn_state.element1;
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: add s_3_6 s_3_4
        let s_3_7: i128 = (s_3_6 + s_3_4);
        // D s_3_8: read-var esizeshadow#2858:i64
        let s_3_8: i64 = fn_state.esizeshadow_2858;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // C s_3_11: const #1s : i
        let s_3_11: i128 = 1;
        // D s_3_12: read-var esizeshadow#2858:i64
        let s_3_12: i64 = fn_state.esizeshadow_2858;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: sub s_3_13 s_3_11
        let s_3_14: i128 = ((s_3_13) - (s_3_11));
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // C s_3_16: const #0s : i
        let s_3_16: i128 = 0;
        // D s_3_17: cast zx s_3_15 -> i
        let s_3_17: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_18: call integer_subrange(s_3_7, s_3_17, s_3_16)
        let s_3_18: Bits = integer_subrange(state, tracer, s_3_7, s_3_17, s_3_16);
        // D s_3_19: read-var e:i64
        let s_3_19: i64 = fn_state.e;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: cast zx s_3_10 -> i
        let s_3_21: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_22: read-var result:bv
        let s_3_22: Bits = fn_state.result;
        // D s_3_23: call Elem_set(s_3_22, s_3_20, s_3_21, s_3_18)
        let s_3_23: Bits = Elem_set(state, tracer, s_3_22, s_3_20, s_3_21, s_3_18);
        // D s_3_24: write-var result <= s_3_23
        fn_state.result = s_3_23;
        // D s_3_25: read-var e:i64
        let s_3_25: i64 = fn_state.e;
        // C s_3_26: const #1s : i64
        let s_3_26: i64 = 1;
        // D s_3_27: add s_3_25 s_3_26
        let s_3_27: i64 = (s_3_25 + s_3_26);
        // D s_3_28: write-var e <= s_3_27
        fn_state.e = s_3_27;
        // N s_3_29: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2860:i64
        let s_4_0: i64 = fn_state.VLshadow_2860;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
