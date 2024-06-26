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
use Elem_set::*;
use CheckSVEEnabled::*;
use u_shl_int_general::*;
use P_read::*;
use ActivePredicateElement::*;
use u_shr_int_general::*;
use integer_subrange::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_SRSHR_Z_P_ZI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    g: i64,
    shift: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        element1: i64,
        gs_204500: i64,
        esizeshadow_3460: i64,
        VLshadow_3461: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_3462: i64,
        mask: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        g: i64,
        shift: i128,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        g,
        shift,
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
        // D s_0_3: write-var esizeshadow#3460 <= s_0_2
        fn_state.esizeshadow_3460 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3461 <= s_0_6
        fn_state.VLshadow_3461 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3461:i64
        let s_1_0: i64 = fn_state.VLshadow_3461;
        // D s_1_1: write-var VLshadow#3462 <= s_1_0
        fn_state.VLshadow_3462 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#3462:i64
        let s_1_3: i64 = fn_state.VLshadow_3462;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#3462:i64
        let s_1_7: i64 = fn_state.VLshadow_3462;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#3460:i64
        let s_1_9: i64 = fn_state.esizeshadow_3460;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: read-var VLshadow#3462:i64
        let s_1_13: i64 = fn_state.VLshadow_3462;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var dn:i64
        let s_1_16: i64 = fn_state.dn;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call Z_read(s_1_17, s_1_18)
        let s_1_19: Bits = Z_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var operand1 <= s_1_19
        fn_state.operand1 = s_1_19;
        // D s_1_21: cast zx s_1_6 -> i
        let s_1_21: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var g:i64
        let s_1_23: i64 = fn_state.g;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call P_read(s_1_24, s_1_25)
        let s_1_26: Bits = P_read(state, tracer, s_1_24, s_1_25);
        // D s_1_27: write-var mask <= s_1_26
        fn_state.mask = s_1_26;
        // C s_1_28: const #0s : i64
        let s_1_28: i64 = 0;
        // C s_1_29: const #1s : i
        let s_1_29: i128 = 1;
        // D s_1_30: cast zx s_1_12 -> i
        let s_1_30: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_31: sub s_1_30 s_1_29
        let s_1_31: i128 = ((s_1_30) - (s_1_29));
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: write-var gs#204500 <= s_1_32
        fn_state.gs_204500 = s_1_32;
        // D s_1_34: write-var e <= s_1_28
        fn_state.e = s_1_28;
        // N s_1_35: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#204500:i64
        let s_2_1: i64 = fn_state.gs_204500;
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
        // D s_3_0: read-var esizeshadow#3460:i64
        let s_3_0: i64 = fn_state.esizeshadow_3460;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand1:bv
        let s_3_6: Bits = fn_state.operand1;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: cast sx s_3_7 -> i
        let s_3_8: i128 = {
            let sign_bit = s_3_7.length() - 1;
            let mut result = s_3_7.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: write-var element1 <= s_3_9
        fn_state.element1 = s_3_9;
        // D s_3_11: read-var e:i64
        let s_3_11: i64 = fn_state.e;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: read-var esizeshadow#3460:i64
        let s_3_13: i64 = fn_state.esizeshadow_3460;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: read-var mask:bv
        let s_3_15: Bits = fn_state.mask;
        // D s_3_16: call ActivePredicateElement(s_3_15, s_3_12, s_3_14)
        let s_3_16: bool = ActivePredicateElement(state, tracer, s_3_15, s_3_12, s_3_14);
        // N s_3_17: branch s_3_16 b6 b4
        if s_3_16 {
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
        // D s_4_0: read-var esizeshadow#3460:i64
        let s_4_0: i64 = fn_state.esizeshadow_3460;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var esizeshadow#3460:i64
        let s_4_3: i64 = fn_state.esizeshadow_3460;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: read-var e:i64
        let s_4_6: i64 = fn_state.e;
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: cast zx s_4_5 -> i
        let s_4_8: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_9: read-var operand1:bv
        let s_4_9: Bits = fn_state.operand1;
        // D s_4_10: call Elem_read(s_4_9, s_4_7, s_4_8)
        let s_4_10: Bits = Elem_read(state, tracer, s_4_9, s_4_7, s_4_8);
        // D s_4_11: read-var e:i64
        let s_4_11: i64 = fn_state.e;
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: cast zx s_4_2 -> i
        let s_4_13: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_14: read-var result:bv
        let s_4_14: Bits = fn_state.result;
        // D s_4_15: call Elem_set(s_4_14, s_4_12, s_4_13, s_4_10)
        let s_4_15: Bits = Elem_set(state, tracer, s_4_14, s_4_12, s_4_13, s_4_10);
        // D s_4_16: write-var result <= s_4_15
        fn_state.result = s_4_15;
        // N s_4_17: jump b5
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
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: read-var shift:i
        let s_6_1: i128 = fn_state.shift;
        // D s_6_2: sub s_6_1 s_6_0
        let s_6_2: i128 = ((s_6_1) - (s_6_0));
        // C s_6_3: const #1s : i
        let s_6_3: i128 = 1;
        // D s_6_4: call _shl_int_general(s_6_3, s_6_2)
        let s_6_4: i128 = u_shl_int_general(state, tracer, s_6_3, s_6_2);
        // D s_6_5: read-var element1:i64
        let s_6_5: i64 = fn_state.element1;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: add s_6_6 s_6_4
        let s_6_7: i128 = (s_6_6 + s_6_4);
        // D s_6_8: read-var shift:i
        let s_6_8: i128 = fn_state.shift;
        // D s_6_9: call _shr_int_general(s_6_7, s_6_8)
        let s_6_9: i128 = u_shr_int_general(state, tracer, s_6_7, s_6_8);
        // D s_6_10: read-var esizeshadow#3460:i64
        let s_6_10: i64 = fn_state.esizeshadow_3460;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: cast reint s_6_11 -> i64
        let s_6_12: i64 = (s_6_11 as i64);
        // C s_6_13: const #1s : i
        let s_6_13: i128 = 1;
        // D s_6_14: read-var esizeshadow#3460:i64
        let s_6_14: i64 = fn_state.esizeshadow_3460;
        // D s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (i128::try_from(s_6_14).unwrap());
        // D s_6_16: sub s_6_15 s_6_13
        let s_6_16: i128 = ((s_6_15) - (s_6_13));
        // D s_6_17: cast reint s_6_16 -> i64
        let s_6_17: i64 = (s_6_16 as i64);
        // C s_6_18: const #0s : i
        let s_6_18: i128 = 0;
        // D s_6_19: cast zx s_6_17 -> i
        let s_6_19: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_20: call integer_subrange(s_6_9, s_6_19, s_6_18)
        let s_6_20: Bits = integer_subrange(state, tracer, s_6_9, s_6_19, s_6_18);
        // D s_6_21: read-var e:i64
        let s_6_21: i64 = fn_state.e;
        // D s_6_22: cast zx s_6_21 -> i
        let s_6_22: i128 = (i128::try_from(s_6_21).unwrap());
        // D s_6_23: cast zx s_6_12 -> i
        let s_6_23: i128 = (i128::try_from(s_6_12).unwrap());
        // D s_6_24: read-var result:bv
        let s_6_24: Bits = fn_state.result;
        // D s_6_25: call Elem_set(s_6_24, s_6_22, s_6_23, s_6_20)
        let s_6_25: Bits = Elem_set(state, tracer, s_6_24, s_6_22, s_6_23, s_6_20);
        // D s_6_26: write-var result <= s_6_25
        fn_state.result = s_6_25;
        // N s_6_27: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#3462:i64
        let s_7_0: i64 = fn_state.VLshadow_3462;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var dn:i64
        let s_7_3: i64 = fn_state.dn;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var result:bv
        let s_7_6: Bits = fn_state.result;
        // D s_7_7: call Z_set(s_7_4, s_7_5, s_7_6)
        let s_7_7: () = Z_set(state, tracer, s_7_4, s_7_5, s_7_6);
        // N s_7_8: return
        return;
    }
}
