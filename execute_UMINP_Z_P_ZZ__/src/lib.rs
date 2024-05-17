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
use Zeros::*;
use Elem_read::*;
use IsEven::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_UMINP_Z_P_ZZ__<T: Tracer>(
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
        esizeshadow_3384: i64,
        VLshadow_3386: i64,
        VLshadow_3385: i64,
        mask: Bits,
        element2: i128,
        element1: i128,
        gs_203196: i64,
        elements: i64,
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
        // D s_0_3: write-var esizeshadow#3384 <= s_0_2
        fn_state.esizeshadow_3384 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3385 <= s_0_6
        fn_state.VLshadow_3385 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3385:i64
        let s_1_0: i64 = fn_state.VLshadow_3385;
        // D s_1_1: write-var VLshadow#3386 <= s_1_0
        fn_state.VLshadow_3386 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#3386:i64
        let s_1_3: i64 = fn_state.VLshadow_3386;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#3386:i64
        let s_1_7: i64 = fn_state.VLshadow_3386;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#3384:i64
        let s_1_9: i64 = fn_state.esizeshadow_3384;
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
        // D s_1_21: read-var VLshadow#3386:i64
        let s_1_21: i64 = fn_state.VLshadow_3386;
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
        // D s_1_29: read-var esizeshadow#3384:i64
        let s_1_29: i64 = fn_state.esizeshadow_3384;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: read-var mask:bv
        let s_1_31: Bits = fn_state.mask;
        // D s_1_32: call AnyActiveElement(s_1_31, s_1_30)
        let s_1_32: bool = AnyActiveElement(state, tracer, s_1_31, s_1_30);
        // N s_1_33: branch s_1_32 b13 b2
        if s_1_32 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#3386:i64
        let s_2_0: i64 = fn_state.VLshadow_3386;
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
        // D s_3_6: write-var gs#203196 <= s_3_5
        fn_state.gs_203196 = s_3_5;
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
        // D s_4_1: read-var gs#203196:i64
        let s_4_1: i64 = fn_state.gs_203196;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b12 b5
        if s_4_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var esizeshadow#3384:i64
        let s_5_2: i64 = fn_state.esizeshadow_3384;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var mask:bv
        let s_5_4: Bits = fn_state.mask;
        // D s_5_5: call ActivePredicateElement(s_5_4, s_5_1, s_5_3)
        let s_5_5: bool = ActivePredicateElement(state, tracer, s_5_4, s_5_1, s_5_3);
        // D s_5_6: not s_5_5
        let s_5_6: bool = !s_5_5;
        // N s_5_7: branch s_5_6 b11 b6
        if s_5_6 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call IsEven(s_6_1)
        let s_6_2: bool = IsEven(state, tracer, s_6_1);
        // N s_6_3: branch s_6_2 b10 b7
        if s_6_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // D s_7_1: read-var e:i64
        let s_7_1: i64 = fn_state.e;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: sub s_7_2 s_7_0
        let s_7_3: i128 = ((s_7_2) - (s_7_0));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: read-var esizeshadow#3384:i64
        let s_7_5: i64 = fn_state.esizeshadow_3384;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: cast zx s_7_4 -> i
        let s_7_8: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_9: cast zx s_7_7 -> i
        let s_7_9: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_10: read-var operand2:bv
        let s_7_10: Bits = fn_state.operand2;
        // D s_7_11: call Elem_read(s_7_10, s_7_8, s_7_9)
        let s_7_11: Bits = Elem_read(state, tracer, s_7_10, s_7_8, s_7_9);
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (s_7_11.value() as i128);
        // D s_7_13: write-var element1 <= s_7_12
        fn_state.element1 = s_7_12;
        // C s_7_14: const #0s : i
        let s_7_14: i128 = 0;
        // D s_7_15: read-var e:i64
        let s_7_15: i64 = fn_state.e;
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (i128::try_from(s_7_15).unwrap());
        // D s_7_17: add s_7_16 s_7_14
        let s_7_17: i128 = (s_7_16 + s_7_14);
        // D s_7_18: cast reint s_7_17 -> i64
        let s_7_18: i64 = (s_7_17 as i64);
        // D s_7_19: read-var esizeshadow#3384:i64
        let s_7_19: i64 = fn_state.esizeshadow_3384;
        // D s_7_20: cast zx s_7_19 -> i
        let s_7_20: i128 = (i128::try_from(s_7_19).unwrap());
        // D s_7_21: cast reint s_7_20 -> i64
        let s_7_21: i64 = (s_7_20 as i64);
        // D s_7_22: cast zx s_7_18 -> i
        let s_7_22: i128 = (i128::try_from(s_7_18).unwrap());
        // D s_7_23: cast zx s_7_21 -> i
        let s_7_23: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_24: read-var operand2:bv
        let s_7_24: Bits = fn_state.operand2;
        // D s_7_25: call Elem_read(s_7_24, s_7_22, s_7_23)
        let s_7_25: Bits = Elem_read(state, tracer, s_7_24, s_7_22, s_7_23);
        // D s_7_26: cast zx s_7_25 -> i
        let s_7_26: i128 = (s_7_25.value() as i128);
        // D s_7_27: write-var element2 <= s_7_26
        fn_state.element2 = s_7_26;
        // N s_7_28: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var element1:i
        let s_8_0: i128 = fn_state.element1;
        // D s_8_1: read-var element2:i
        let s_8_1: i128 = fn_state.element2;
        // D s_8_2: cmp-lt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) < (s_8_1));
        // D s_8_3: select s_8_2 s_8_0 s_8_1
        let s_8_3: i128 = if s_8_2 { s_8_0 } else { s_8_1 };
        // D s_8_4: read-var esizeshadow#3384:i64
        let s_8_4: i64 = fn_state.esizeshadow_3384;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: cast reint s_8_5 -> i64
        let s_8_6: i64 = (s_8_5 as i64);
        // C s_8_7: const #1s : i
        let s_8_7: i128 = 1;
        // D s_8_8: read-var esizeshadow#3384:i64
        let s_8_8: i64 = fn_state.esizeshadow_3384;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: sub s_8_9 s_8_7
        let s_8_10: i128 = ((s_8_9) - (s_8_7));
        // D s_8_11: cast reint s_8_10 -> i64
        let s_8_11: i64 = (s_8_10 as i64);
        // C s_8_12: const #0s : i
        let s_8_12: i128 = 0;
        // D s_8_13: cast zx s_8_11 -> i
        let s_8_13: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_14: call integer_subrange(s_8_3, s_8_13, s_8_12)
        let s_8_14: Bits = integer_subrange(state, tracer, s_8_3, s_8_13, s_8_12);
        // D s_8_15: read-var e:i64
        let s_8_15: i64 = fn_state.e;
        // D s_8_16: cast zx s_8_15 -> i
        let s_8_16: i128 = (i128::try_from(s_8_15).unwrap());
        // D s_8_17: cast zx s_8_6 -> i
        let s_8_17: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_18: read-var result:bv
        let s_8_18: Bits = fn_state.result;
        // D s_8_19: call Elem_set(s_8_18, s_8_16, s_8_17, s_8_14)
        let s_8_19: Bits = Elem_set(state, tracer, s_8_18, s_8_16, s_8_17, s_8_14);
        // D s_8_20: write-var result <= s_8_19
        fn_state.result = s_8_19;
        // N s_8_21: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var e <= s_9_2
        fn_state.e = s_9_2;
        // N s_9_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var e:i64
        let s_10_1: i64 = fn_state.e;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: add s_10_2 s_10_0
        let s_10_3: i128 = (s_10_2 + s_10_0);
        // D s_10_4: cast reint s_10_3 -> i64
        let s_10_4: i64 = (s_10_3 as i64);
        // D s_10_5: read-var esizeshadow#3384:i64
        let s_10_5: i64 = fn_state.esizeshadow_3384;
        // D s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_7: cast reint s_10_6 -> i64
        let s_10_7: i64 = (s_10_6 as i64);
        // D s_10_8: cast zx s_10_4 -> i
        let s_10_8: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_9: cast zx s_10_7 -> i
        let s_10_9: i128 = (i128::try_from(s_10_7).unwrap());
        // D s_10_10: read-var operand1:bv
        let s_10_10: Bits = fn_state.operand1;
        // D s_10_11: call Elem_read(s_10_10, s_10_8, s_10_9)
        let s_10_11: Bits = Elem_read(state, tracer, s_10_10, s_10_8, s_10_9);
        // D s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (s_10_11.value() as i128);
        // D s_10_13: write-var element1 <= s_10_12
        fn_state.element1 = s_10_12;
        // C s_10_14: const #1s : i
        let s_10_14: i128 = 1;
        // D s_10_15: read-var e:i64
        let s_10_15: i64 = fn_state.e;
        // D s_10_16: cast zx s_10_15 -> i
        let s_10_16: i128 = (i128::try_from(s_10_15).unwrap());
        // D s_10_17: add s_10_16 s_10_14
        let s_10_17: i128 = (s_10_16 + s_10_14);
        // D s_10_18: cast reint s_10_17 -> i64
        let s_10_18: i64 = (s_10_17 as i64);
        // D s_10_19: read-var esizeshadow#3384:i64
        let s_10_19: i64 = fn_state.esizeshadow_3384;
        // D s_10_20: cast zx s_10_19 -> i
        let s_10_20: i128 = (i128::try_from(s_10_19).unwrap());
        // D s_10_21: cast reint s_10_20 -> i64
        let s_10_21: i64 = (s_10_20 as i64);
        // D s_10_22: cast zx s_10_18 -> i
        let s_10_22: i128 = (i128::try_from(s_10_18).unwrap());
        // D s_10_23: cast zx s_10_21 -> i
        let s_10_23: i128 = (i128::try_from(s_10_21).unwrap());
        // D s_10_24: read-var operand1:bv
        let s_10_24: Bits = fn_state.operand1;
        // D s_10_25: call Elem_read(s_10_24, s_10_22, s_10_23)
        let s_10_25: Bits = Elem_read(state, tracer, s_10_24, s_10_22, s_10_23);
        // D s_10_26: cast zx s_10_25 -> i
        let s_10_26: i128 = (s_10_25.value() as i128);
        // D s_10_27: write-var element2 <= s_10_26
        fn_state.element2 = s_10_26;
        // N s_10_28: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#3384:i64
        let s_11_0: i64 = fn_state.esizeshadow_3384;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: read-var esizeshadow#3384:i64
        let s_11_3: i64 = fn_state.esizeshadow_3384;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: cast reint s_11_4 -> i64
        let s_11_5: i64 = (s_11_4 as i64);
        // D s_11_6: read-var e:i64
        let s_11_6: i64 = fn_state.e;
        // D s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (i128::try_from(s_11_6).unwrap());
        // D s_11_8: cast zx s_11_5 -> i
        let s_11_8: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_9: read-var operand1:bv
        let s_11_9: Bits = fn_state.operand1;
        // D s_11_10: call Elem_read(s_11_9, s_11_7, s_11_8)
        let s_11_10: Bits = Elem_read(state, tracer, s_11_9, s_11_7, s_11_8);
        // D s_11_11: read-var e:i64
        let s_11_11: i64 = fn_state.e;
        // D s_11_12: cast zx s_11_11 -> i
        let s_11_12: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_13: cast zx s_11_2 -> i
        let s_11_13: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_14: read-var result:bv
        let s_11_14: Bits = fn_state.result;
        // D s_11_15: call Elem_set(s_11_14, s_11_12, s_11_13, s_11_10)
        let s_11_15: Bits = Elem_set(state, tracer, s_11_14, s_11_12, s_11_13, s_11_10);
        // D s_11_16: write-var result <= s_11_15
        fn_state.result = s_11_15;
        // N s_11_17: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var VLshadow#3386:i64
        let s_12_0: i64 = fn_state.VLshadow_3386;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: read-var dn:i64
        let s_12_3: i64 = fn_state.dn;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: cast zx s_12_2 -> i
        let s_12_5: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_6: read-var result:bv
        let s_12_6: Bits = fn_state.result;
        // D s_12_7: call Z_set(s_12_4, s_12_5, s_12_6)
        let s_12_7: () = Z_set(state, tracer, s_12_4, s_12_5, s_12_6);
        // N s_12_8: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VLshadow#3386:i64
        let s_13_0: i64 = fn_state.VLshadow_3386;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: read-var m:i64
        let s_13_3: i64 = fn_state.m;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: cast zx s_13_2 -> i
        let s_13_5: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_6: call Z_read(s_13_4, s_13_5)
        let s_13_6: Bits = Z_read(state, tracer, s_13_4, s_13_5);
        // D s_13_7: write-var operand2 <= s_13_6
        fn_state.operand2 = s_13_6;
        // N s_13_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
