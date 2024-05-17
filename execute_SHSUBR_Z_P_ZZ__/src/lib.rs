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
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use integer_subrange::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_SHSUBR_Z_P_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    g: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        esizeshadow_4006: i64,
        gs_214745: i64,
        mask: Bits,
        element2: i64,
        VLshadow_4008: i64,
        element1: i64,
        elements: i64,
        VLshadow_4007: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        g: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        g,
        m,
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
        // D s_0_3: write-var esizeshadow#4006 <= s_0_2
        fn_state.esizeshadow_4006 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4007 <= s_0_6
        fn_state.VLshadow_4007 = s_0_6;
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
        // D s_1_0: read-var VLshadow#4007:i64
        let s_1_0: i64 = fn_state.VLshadow_4007;
        // D s_1_1: write-var VLshadow#4008 <= s_1_0
        fn_state.VLshadow_4008 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#4008:i64
        let s_1_3: i64 = fn_state.VLshadow_4008;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4008:i64
        let s_1_7: i64 = fn_state.VLshadow_4008;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#4006:i64
        let s_1_9: i64 = fn_state.esizeshadow_4006;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var elements <= s_1_12
        fn_state.elements = s_1_12;
        // D s_1_14: cast zx s_1_6 -> i
        let s_1_14: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var g:i64
        let s_1_16: i64 = fn_state.g;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var mask <= s_1_19
        fn_state.mask = s_1_19;
        // D s_1_21: read-var VLshadow#4008:i64
        let s_1_21: i64 = fn_state.VLshadow_4008;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var dn:i64
        let s_1_24: i64 = fn_state.dn;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast zx s_1_23 -> i
        let s_1_26: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_27: call Z_read(s_1_25, s_1_26)
        let s_1_27: Bits = Z_read(state, tracer, s_1_25, s_1_26);
        // D s_1_28: write-var operand1 <= s_1_27
        fn_state.operand1 = s_1_27;
        // D s_1_29: read-var esizeshadow#4006:i64
        let s_1_29: i64 = fn_state.esizeshadow_4006;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: read-var mask:bv
        let s_1_31: Bits = fn_state.mask;
        // D s_1_32: call AnyActiveElement(s_1_31, s_1_30)
        let s_1_32: bool = AnyActiveElement(state, tracer, s_1_31, s_1_30);
        // N s_1_33: branch s_1_32 b10 b2
        if s_1_32 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#4008:i64
        let s_2_0: i64 = fn_state.VLshadow_4008;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand2 <= s_2_2
        fn_state.operand2 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#214745 <= s_3_5
        fn_state.gs_214745 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#214745:i64
        let s_4_1: i64 = fn_state.gs_214745;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#4006:i64
        let s_5_0: i64 = fn_state.esizeshadow_4006;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var e:i64
        let s_5_3: i64 = fn_state.e;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var operand1:bv
        let s_5_6: Bits = fn_state.operand1;
        // D s_5_7: call Elem_read(s_5_6, s_5_4, s_5_5)
        let s_5_7: Bits = Elem_read(state, tracer, s_5_6, s_5_4, s_5_5);
        // D s_5_8: cast sx s_5_7 -> i
        let s_5_8: i128 = {
            let sign_bit = s_5_7.length() - 1;
            let mut result = s_5_7.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: write-var element1 <= s_5_9
        fn_state.element1 = s_5_9;
        // D s_5_11: read-var esizeshadow#4006:i64
        let s_5_11: i64 = fn_state.esizeshadow_4006;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: read-var e:i64
        let s_5_14: i64 = fn_state.e;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: cast zx s_5_13 -> i
        let s_5_16: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_17: read-var operand2:bv
        let s_5_17: Bits = fn_state.operand2;
        // D s_5_18: call Elem_read(s_5_17, s_5_15, s_5_16)
        let s_5_18: Bits = Elem_read(state, tracer, s_5_17, s_5_15, s_5_16);
        // D s_5_19: cast sx s_5_18 -> i
        let s_5_19: i128 = {
            let sign_bit = s_5_18.length() - 1;
            let mut result = s_5_18.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_20: cast reint s_5_19 -> i64
        let s_5_20: i64 = (s_5_19 as i64);
        // D s_5_21: write-var element2 <= s_5_20
        fn_state.element2 = s_5_20;
        // D s_5_22: read-var e:i64
        let s_5_22: i64 = fn_state.e;
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_24: read-var esizeshadow#4006:i64
        let s_5_24: i64 = fn_state.esizeshadow_4006;
        // D s_5_25: cast zx s_5_24 -> i
        let s_5_25: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_26: read-var mask:bv
        let s_5_26: Bits = fn_state.mask;
        // D s_5_27: call ActivePredicateElement(s_5_26, s_5_23, s_5_25)
        let s_5_27: bool = ActivePredicateElement(state, tracer, s_5_26, s_5_23, s_5_25);
        // N s_5_28: branch s_5_27 b8 b6
        if s_5_27 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#4006:i64
        let s_6_0: i64 = fn_state.esizeshadow_4006;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var esizeshadow#4006:i64
        let s_6_3: i64 = fn_state.esizeshadow_4006;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: read-var e:i64
        let s_6_6: i64 = fn_state.e;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: cast zx s_6_5 -> i
        let s_6_8: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_9: read-var operand1:bv
        let s_6_9: Bits = fn_state.operand1;
        // D s_6_10: call Elem_read(s_6_9, s_6_7, s_6_8)
        let s_6_10: Bits = Elem_read(state, tracer, s_6_9, s_6_7, s_6_8);
        // D s_6_11: read-var e:i64
        let s_6_11: i64 = fn_state.e;
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_13: cast zx s_6_2 -> i
        let s_6_13: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_14: read-var result:bv
        let s_6_14: Bits = fn_state.result;
        // D s_6_15: call Elem_set(s_6_14, s_6_12, s_6_13, s_6_10)
        let s_6_15: Bits = Elem_set(state, tracer, s_6_14, s_6_12, s_6_13, s_6_10);
        // D s_6_16: write-var result <= s_6_15
        fn_state.result = s_6_15;
        // N s_6_17: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var e <= s_7_2
        fn_state.e = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var element2:i64
        let s_8_0: i64 = fn_state.element2;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var element1:i64
        let s_8_2: i64 = fn_state.element1;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: sub s_8_1 s_8_3
        let s_8_4: i128 = ((s_8_1) - (s_8_3));
        // C s_8_5: const #1s : i
        let s_8_5: i128 = 1;
        // D s_8_6: lsr s_8_4 s_8_5
        let s_8_6: i128 = s_8_4 >> s_8_5;
        // D s_8_7: read-var esizeshadow#4006:i64
        let s_8_7: i64 = fn_state.esizeshadow_4006;
        // D s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_9: cast reint s_8_8 -> i64
        let s_8_9: i64 = (s_8_8 as i64);
        // C s_8_10: const #1s : i
        let s_8_10: i128 = 1;
        // D s_8_11: read-var esizeshadow#4006:i64
        let s_8_11: i64 = fn_state.esizeshadow_4006;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: sub s_8_12 s_8_10
        let s_8_13: i128 = ((s_8_12) - (s_8_10));
        // D s_8_14: cast reint s_8_13 -> i64
        let s_8_14: i64 = (s_8_13 as i64);
        // C s_8_15: const #0s : i
        let s_8_15: i128 = 0;
        // D s_8_16: cast zx s_8_14 -> i
        let s_8_16: i128 = (i128::try_from(s_8_14).unwrap());
        // D s_8_17: call integer_subrange(s_8_6, s_8_16, s_8_15)
        let s_8_17: Bits = integer_subrange(state, tracer, s_8_6, s_8_16, s_8_15);
        // D s_8_18: read-var e:i64
        let s_8_18: i64 = fn_state.e;
        // D s_8_19: cast zx s_8_18 -> i
        let s_8_19: i128 = (i128::try_from(s_8_18).unwrap());
        // D s_8_20: cast zx s_8_9 -> i
        let s_8_20: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_21: read-var result:bv
        let s_8_21: Bits = fn_state.result;
        // D s_8_22: call Elem_set(s_8_21, s_8_19, s_8_20, s_8_17)
        let s_8_22: Bits = Elem_set(state, tracer, s_8_21, s_8_19, s_8_20, s_8_17);
        // D s_8_23: write-var result <= s_8_22
        fn_state.result = s_8_22;
        // N s_8_24: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VLshadow#4008:i64
        let s_9_0: i64 = fn_state.VLshadow_4008;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var dn:i64
        let s_9_3: i64 = fn_state.dn;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast zx s_9_2 -> i
        let s_9_5: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_6: read-var result:bv
        let s_9_6: Bits = fn_state.result;
        // D s_9_7: call Z_set(s_9_4, s_9_5, s_9_6)
        let s_9_7: () = Z_set(state, tracer, s_9_4, s_9_5, s_9_6);
        // N s_9_8: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#4008:i64
        let s_10_0: i64 = fn_state.VLshadow_4008;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var m:i64
        let s_10_3: i64 = fn_state.m;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: call Z_read(s_10_4, s_10_5)
        let s_10_6: Bits = Z_read(state, tracer, s_10_4, s_10_5);
        // D s_10_7: write-var operand2 <= s_10_6
        fn_state.operand2 = s_10_6;
        // N s_10_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
