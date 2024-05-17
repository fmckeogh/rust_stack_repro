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
use integer_subrange::*;
use V_set::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use P_read::*;
use ActivePredicateElement::*;
use common::*;
pub fn execute_SADDV_R_P_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        gs_193642: i64,
        VLshadow_2826: i64,
        e: i64,
        sum: i128,
        esizeshadow_2825: i64,
        mask: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
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
        // D s_0_3: write-var esizeshadow#2825 <= s_0_2
        fn_state.esizeshadow_2825 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2826 <= s_0_6
        fn_state.VLshadow_2826 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2826:i64
        let s_1_0: i64 = fn_state.VLshadow_2826;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_0 -> i
        let s_1_5: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_6: read-var esizeshadow#2825:i64
        let s_1_6: i64 = fn_state.esizeshadow_2825;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: div s_1_5 s_1_7
        let s_1_8: i128 = ((s_1_5) / (s_1_7));
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: cast zx s_1_4 -> i
        let s_1_10: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var g:i64
        let s_1_12: i64 = fn_state.g;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast zx s_1_11 -> i
        let s_1_14: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_15: call P_read(s_1_13, s_1_14)
        let s_1_15: Bits = P_read(state, tracer, s_1_13, s_1_14);
        // D s_1_16: write-var mask <= s_1_15
        fn_state.mask = s_1_15;
        // D s_1_17: cast zx s_1_0 -> i
        let s_1_17: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var n:i64
        let s_1_19: i64 = fn_state.n;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call Z_read(s_1_20, s_1_21)
        let s_1_22: Bits = Z_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var operand <= s_1_22
        fn_state.operand = s_1_22;
        // C s_1_24: const #0s : i
        let s_1_24: i128 = 0;
        // D s_1_25: write-var sum <= s_1_24
        fn_state.sum = s_1_24;
        // C s_1_26: const #0s : i64
        let s_1_26: i64 = 0;
        // C s_1_27: const #1s : i
        let s_1_27: i128 = 1;
        // D s_1_28: cast zx s_1_9 -> i
        let s_1_28: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_29: sub s_1_28 s_1_27
        let s_1_29: i128 = ((s_1_28) - (s_1_27));
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: write-var gs#193642 <= s_1_30
        fn_state.gs_193642 = s_1_30;
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
        // D s_2_1: read-var gs#193642:i64
        let s_2_1: i64 = fn_state.gs_193642;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
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
        // D s_3_2: read-var esizeshadow#2825:i64
        let s_3_2: i64 = fn_state.esizeshadow_2825;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var mask:bv
        let s_3_4: Bits = fn_state.mask;
        // D s_3_5: call ActivePredicateElement(s_3_4, s_3_1, s_3_3)
        let s_3_5: bool = ActivePredicateElement(state, tracer, s_3_4, s_3_1, s_3_3);
        // N s_3_6: branch s_3_5 b6 b4
        if s_3_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var e <= s_5_2
        fn_state.e = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#2825:i64
        let s_6_0: i64 = fn_state.esizeshadow_2825;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var e:i64
        let s_6_3: i64 = fn_state.e;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast zx s_6_2 -> i
        let s_6_5: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_6: read-var operand:bv
        let s_6_6: Bits = fn_state.operand;
        // D s_6_7: call Elem_read(s_6_6, s_6_4, s_6_5)
        let s_6_7: Bits = Elem_read(state, tracer, s_6_6, s_6_4, s_6_5);
        // D s_6_8: cast sx s_6_7 -> i
        let s_6_8: i128 = {
            let sign_bit = s_6_7.length() - 1;
            let mut result = s_6_7.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_6_9: cast reint s_6_8 -> i64
        let s_6_9: i64 = (s_6_8 as i64);
        // D s_6_10: cast zx s_6_9 -> i
        let s_6_10: i128 = (i128::try_from(s_6_9).unwrap());
        // D s_6_11: read-var sum:i
        let s_6_11: i128 = fn_state.sum;
        // D s_6_12: add s_6_11 s_6_10
        let s_6_12: i128 = (s_6_11 + s_6_10);
        // D s_6_13: write-var sum <= s_6_12
        fn_state.sum = s_6_12;
        // N s_6_14: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var sum:i
        let s_7_0: i128 = fn_state.sum;
        // C s_7_1: const #64s : i64
        let s_7_1: i64 = 64;
        // C s_7_2: const #63s : i
        let s_7_2: i128 = 63;
        // C s_7_3: const #0s : i
        let s_7_3: i128 = 0;
        // D s_7_4: call integer_subrange(s_7_0, s_7_2, s_7_3)
        let s_7_4: Bits = integer_subrange(state, tracer, s_7_0, s_7_2, s_7_3);
        // D s_7_5: cast reint s_7_4 -> u64
        let s_7_5: u64 = (s_7_4.value() as u64);
        // D s_7_6: read-var d:i64
        let s_7_6: i64 = fn_state.d;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: cast zx s_7_5 -> bv
        let s_7_8: Bits = Bits::new(s_7_5 as u128, 64u16);
        // D s_7_9: call V_set(s_7_7, s_7_1, s_7_8)
        let s_7_9: () = V_set(state, tracer, s_7_7, s_7_1, s_7_8);
        // N s_7_10: return
        return;
    }
}
