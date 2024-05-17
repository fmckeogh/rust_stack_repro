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
use u__id::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use FPCR_read::*;
use FPNeg::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use FPMulAdd::*;
use Z_set::*;
use common::*;
pub fn execute_FCMLA_Z_P_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
    neg_i: bool,
    neg_r: bool,
    sel_a: i64,
    sel_b: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        p: i64,
        gs_180992: bool,
        gs_181003: bool,
        elt2_a: Bits,
        mask: Bits,
        VLshadow_2300: i64,
        gs_181008: bool,
        operand1: Bits,
        gs_181001: bool,
        u_2577: Bits,
        operand2: Bits,
        operand3: Bits,
        gs_180994: bool,
        addend_i: Bits,
        elt2_b: Bits,
        esizeshadow_2298: i64,
        gs_180966: i64,
        gs_180987: bool,
        pairs: i64,
        gs_181010: bool,
        elt1_a: Bits,
        result: Bits,
        gs_180985: bool,
        VLshadow_2299: i64,
        addend_r: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
        neg_i: bool,
        neg_r: bool,
        sel_a: i64,
        sel_b: i64,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        g,
        m,
        n,
        neg_i,
        neg_r,
        sel_a,
        sel_b,
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
        // D s_0_3: write-var esizeshadow#2298 <= s_0_2
        fn_state.esizeshadow_2298 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2299 <= s_0_6
        fn_state.VLshadow_2299 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2299:i64
        let s_1_0: i64 = fn_state.VLshadow_2299;
        // D s_1_1: write-var VLshadow#2300 <= s_1_0
        fn_state.VLshadow_2300 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2300:i64
        let s_1_3: i64 = fn_state.VLshadow_2300;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #2s : i
        let s_1_7: i128 = 2;
        // D s_1_8: read-var esizeshadow#2298:i64
        let s_1_8: i64 = fn_state.esizeshadow_2298;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: mul s_1_7 s_1_9
        let s_1_10: i128 = ((s_1_7) * (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#2300:i64
        let s_1_12: i64 = fn_state.VLshadow_2300;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast zx s_1_11 -> i
        let s_1_14: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_15: div s_1_13 s_1_14
        let s_1_15: i128 = ((s_1_13) / (s_1_14));
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: write-var pairs <= s_1_16
        fn_state.pairs = s_1_16;
        // D s_1_18: cast zx s_1_6 -> i
        let s_1_18: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: read-var g:i64
        let s_1_20: i64 = fn_state.g;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast zx s_1_19 -> i
        let s_1_22: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_23: call P_read(s_1_21, s_1_22)
        let s_1_23: Bits = P_read(state, tracer, s_1_21, s_1_22);
        // D s_1_24: write-var mask <= s_1_23
        fn_state.mask = s_1_23;
        // D s_1_25: read-var esizeshadow#2298:i64
        let s_1_25: i64 = fn_state.esizeshadow_2298;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: read-var mask:bv
        let s_1_27: Bits = fn_state.mask;
        // D s_1_28: call AnyActiveElement(s_1_27, s_1_26)
        let s_1_28: bool = AnyActiveElement(state, tracer, s_1_27, s_1_26);
        // N s_1_29: branch s_1_28 b50 b2
        if s_1_28 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2300:i64
        let s_2_0: i64 = fn_state.VLshadow_2300;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call Zeros(s_2_1)
        let s_2_2: Bits = Zeros(state, tracer, s_2_1);
        // D s_2_3: write-var operand1 <= s_2_2
        fn_state.operand1 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#2298:i64
        let s_3_0: i64 = fn_state.esizeshadow_2298;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var mask:bv
        let s_3_2: Bits = fn_state.mask;
        // D s_3_3: call AnyActiveElement(s_3_2, s_3_1)
        let s_3_3: bool = AnyActiveElement(state, tracer, s_3_2, s_3_1);
        // N s_3_4: branch s_3_3 b49 b4
        if s_3_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2300:i64
        let s_4_0: i64 = fn_state.VLshadow_2300;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call Zeros(s_4_1)
        let s_4_2: Bits = Zeros(state, tracer, s_4_1);
        // D s_4_3: write-var operand2 <= s_4_2
        fn_state.operand2 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VLshadow#2300:i64
        let s_5_0: i64 = fn_state.VLshadow_2300;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var da:i64
        let s_5_3: i64 = fn_state.da;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: call Z_read(s_5_4, s_5_5)
        let s_5_6: Bits = Z_read(state, tracer, s_5_4, s_5_5);
        // D s_5_7: write-var operand3 <= s_5_6
        fn_state.operand3 = s_5_6;
        // C s_5_8: const #0s : i64
        let s_5_8: i64 = 0;
        // C s_5_9: const #1s : i
        let s_5_9: i128 = 1;
        // D s_5_10: read-var pairs:i64
        let s_5_10: i64 = fn_state.pairs;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: sub s_5_11 s_5_9
        let s_5_12: i128 = ((s_5_11) - (s_5_9));
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var gs#180966 <= s_5_13
        fn_state.gs_180966 = s_5_13;
        // D s_5_15: write-var p <= s_5_8
        fn_state.p = s_5_8;
        // N s_5_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var p:i64
        let s_6_0: i64 = fn_state.p;
        // D s_6_1: read-var gs#180966:i64
        let s_6_1: i64 = fn_state.gs_180966;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b48 b7
        if s_6_2 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i
        let s_7_0: i128 = 2;
        // D s_7_1: read-var p:i64
        let s_7_1: i64 = fn_state.p;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // C s_7_5: const #0s : i
        let s_7_5: i128 = 0;
        // D s_7_6: cast zx s_7_4 -> i
        let s_7_6: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_7: add s_7_6 s_7_5
        let s_7_7: i128 = (s_7_6 + s_7_5);
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // D s_7_9: read-var esizeshadow#2298:i64
        let s_7_9: i64 = fn_state.esizeshadow_2298;
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_11: cast reint s_7_10 -> i64
        let s_7_11: i64 = (s_7_10 as i64);
        // D s_7_12: cast zx s_7_8 -> i
        let s_7_12: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_13: cast zx s_7_11 -> i
        let s_7_13: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_14: read-var operand3:bv
        let s_7_14: Bits = fn_state.operand3;
        // D s_7_15: call Elem_read(s_7_14, s_7_12, s_7_13)
        let s_7_15: Bits = Elem_read(state, tracer, s_7_14, s_7_12, s_7_13);
        // D s_7_16: write-var addend_r <= s_7_15
        fn_state.addend_r = s_7_15;
        // C s_7_17: const #2s : i
        let s_7_17: i128 = 2;
        // D s_7_18: read-var p:i64
        let s_7_18: i64 = fn_state.p;
        // D s_7_19: cast zx s_7_18 -> i
        let s_7_19: i128 = (i128::try_from(s_7_18).unwrap());
        // D s_7_20: mul s_7_17 s_7_19
        let s_7_20: i128 = ((s_7_17) * (s_7_19));
        // D s_7_21: cast reint s_7_20 -> i64
        let s_7_21: i64 = (s_7_20 as i64);
        // C s_7_22: const #1s : i
        let s_7_22: i128 = 1;
        // D s_7_23: cast zx s_7_21 -> i
        let s_7_23: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_24: add s_7_23 s_7_22
        let s_7_24: i128 = (s_7_23 + s_7_22);
        // D s_7_25: cast reint s_7_24 -> i64
        let s_7_25: i64 = (s_7_24 as i64);
        // D s_7_26: read-var esizeshadow#2298:i64
        let s_7_26: i64 = fn_state.esizeshadow_2298;
        // D s_7_27: cast zx s_7_26 -> i
        let s_7_27: i128 = (i128::try_from(s_7_26).unwrap());
        // D s_7_28: cast reint s_7_27 -> i64
        let s_7_28: i64 = (s_7_27 as i64);
        // D s_7_29: cast zx s_7_25 -> i
        let s_7_29: i128 = (i128::try_from(s_7_25).unwrap());
        // D s_7_30: cast zx s_7_28 -> i
        let s_7_30: i128 = (i128::try_from(s_7_28).unwrap());
        // D s_7_31: read-var operand3:bv
        let s_7_31: Bits = fn_state.operand3;
        // D s_7_32: call Elem_read(s_7_31, s_7_29, s_7_30)
        let s_7_32: Bits = Elem_read(state, tracer, s_7_31, s_7_29, s_7_30);
        // D s_7_33: write-var addend_i <= s_7_32
        fn_state.addend_i = s_7_32;
        // C s_7_34: const #2s : i
        let s_7_34: i128 = 2;
        // D s_7_35: read-var p:i64
        let s_7_35: i64 = fn_state.p;
        // D s_7_36: cast zx s_7_35 -> i
        let s_7_36: i128 = (i128::try_from(s_7_35).unwrap());
        // D s_7_37: mul s_7_34 s_7_36
        let s_7_37: i128 = ((s_7_34) * (s_7_36));
        // D s_7_38: cast reint s_7_37 -> i64
        let s_7_38: i64 = (s_7_37 as i64);
        // C s_7_39: const #0s : i
        let s_7_39: i128 = 0;
        // D s_7_40: cast zx s_7_38 -> i
        let s_7_40: i128 = (i128::try_from(s_7_38).unwrap());
        // D s_7_41: add s_7_40 s_7_39
        let s_7_41: i128 = (s_7_40 + s_7_39);
        // D s_7_42: cast reint s_7_41 -> i64
        let s_7_42: i64 = (s_7_41 as i64);
        // D s_7_43: cast zx s_7_42 -> i
        let s_7_43: i128 = (i128::try_from(s_7_42).unwrap());
        // D s_7_44: read-var esizeshadow#2298:i64
        let s_7_44: i64 = fn_state.esizeshadow_2298;
        // D s_7_45: cast zx s_7_44 -> i
        let s_7_45: i128 = (i128::try_from(s_7_44).unwrap());
        // D s_7_46: read-var mask:bv
        let s_7_46: Bits = fn_state.mask;
        // D s_7_47: call ActivePredicateElement(s_7_46, s_7_43, s_7_45)
        let s_7_47: bool = ActivePredicateElement(state, tracer, s_7_46, s_7_43, s_7_45);
        // N s_7_48: branch s_7_47 b30 b8
        if s_7_47 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #2s : i
        let s_9_0: i128 = 2;
        // D s_9_1: read-var p:i64
        let s_9_1: i64 = fn_state.p;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: mul s_9_0 s_9_2
        let s_9_3: i128 = ((s_9_0) * (s_9_2));
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // C s_9_5: const #1s : i
        let s_9_5: i128 = 1;
        // D s_9_6: cast zx s_9_4 -> i
        let s_9_6: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_7: add s_9_6 s_9_5
        let s_9_7: i128 = (s_9_6 + s_9_5);
        // D s_9_8: cast reint s_9_7 -> i64
        let s_9_8: i64 = (s_9_7 as i64);
        // D s_9_9: cast zx s_9_8 -> i
        let s_9_9: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_10: read-var esizeshadow#2298:i64
        let s_9_10: i64 = fn_state.esizeshadow_2298;
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (i128::try_from(s_9_10).unwrap());
        // D s_9_12: read-var mask:bv
        let s_9_12: Bits = fn_state.mask;
        // D s_9_13: call ActivePredicateElement(s_9_12, s_9_9, s_9_11)
        let s_9_13: bool = ActivePredicateElement(state, tracer, s_9_12, s_9_9, s_9_11);
        // N s_9_14: branch s_9_13 b12 b10
        if s_9_13 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2s : i
        let s_11_0: i128 = 2;
        // D s_11_1: read-var p:i64
        let s_11_1: i64 = fn_state.p;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: mul s_11_0 s_11_2
        let s_11_3: i128 = ((s_11_0) * (s_11_2));
        // D s_11_4: cast reint s_11_3 -> i64
        let s_11_4: i64 = (s_11_3 as i64);
        // C s_11_5: const #0s : i
        let s_11_5: i128 = 0;
        // D s_11_6: cast zx s_11_4 -> i
        let s_11_6: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_7: add s_11_6 s_11_5
        let s_11_7: i128 = (s_11_6 + s_11_5);
        // D s_11_8: cast reint s_11_7 -> i64
        let s_11_8: i64 = (s_11_7 as i64);
        // D s_11_9: read-var esizeshadow#2298:i64
        let s_11_9: i64 = fn_state.esizeshadow_2298;
        // D s_11_10: cast zx s_11_9 -> i
        let s_11_10: i128 = (i128::try_from(s_11_9).unwrap());
        // D s_11_11: cast reint s_11_10 -> i64
        let s_11_11: i64 = (s_11_10 as i64);
        // D s_11_12: cast zx s_11_8 -> i
        let s_11_12: i128 = (i128::try_from(s_11_8).unwrap());
        // D s_11_13: cast zx s_11_11 -> i
        let s_11_13: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_14: read-var result:bv
        let s_11_14: Bits = fn_state.result;
        // D s_11_15: read-var addend_r:bv
        let s_11_15: Bits = fn_state.addend_r;
        // D s_11_16: call Elem_set(s_11_14, s_11_12, s_11_13, s_11_15)
        let s_11_16: Bits = Elem_set(state, tracer, s_11_14, s_11_12, s_11_13, s_11_15);
        // D s_11_17: write-var result <= s_11_16
        fn_state.result = s_11_16;
        // C s_11_18: const #2s : i
        let s_11_18: i128 = 2;
        // D s_11_19: read-var p:i64
        let s_11_19: i64 = fn_state.p;
        // D s_11_20: cast zx s_11_19 -> i
        let s_11_20: i128 = (i128::try_from(s_11_19).unwrap());
        // D s_11_21: mul s_11_18 s_11_20
        let s_11_21: i128 = ((s_11_18) * (s_11_20));
        // D s_11_22: cast reint s_11_21 -> i64
        let s_11_22: i64 = (s_11_21 as i64);
        // C s_11_23: const #1s : i
        let s_11_23: i128 = 1;
        // D s_11_24: cast zx s_11_22 -> i
        let s_11_24: i128 = (i128::try_from(s_11_22).unwrap());
        // D s_11_25: add s_11_24 s_11_23
        let s_11_25: i128 = (s_11_24 + s_11_23);
        // D s_11_26: cast reint s_11_25 -> i64
        let s_11_26: i64 = (s_11_25 as i64);
        // D s_11_27: read-var esizeshadow#2298:i64
        let s_11_27: i64 = fn_state.esizeshadow_2298;
        // D s_11_28: cast zx s_11_27 -> i
        let s_11_28: i128 = (i128::try_from(s_11_27).unwrap());
        // D s_11_29: cast reint s_11_28 -> i64
        let s_11_29: i64 = (s_11_28 as i64);
        // D s_11_30: cast zx s_11_26 -> i
        let s_11_30: i128 = (i128::try_from(s_11_26).unwrap());
        // D s_11_31: cast zx s_11_29 -> i
        let s_11_31: i128 = (i128::try_from(s_11_29).unwrap());
        // D s_11_32: read-var result:bv
        let s_11_32: Bits = fn_state.result;
        // D s_11_33: read-var addend_i:bv
        let s_11_33: Bits = fn_state.addend_i;
        // D s_11_34: call Elem_set(s_11_32, s_11_30, s_11_31, s_11_33)
        let s_11_34: Bits = Elem_set(state, tracer, s_11_32, s_11_30, s_11_31, s_11_33);
        // D s_11_35: write-var result <= s_11_34
        fn_state.result = s_11_34;
        // D s_11_36: read-var p:i64
        let s_11_36: i64 = fn_state.p;
        // C s_11_37: const #1s : i64
        let s_11_37: i64 = 1;
        // D s_11_38: add s_11_36 s_11_37
        let s_11_38: i64 = (s_11_36 + s_11_37);
        // D s_11_39: write-var p <= s_11_38
        fn_state.p = s_11_38;
        // N s_11_40: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2s : i
        let s_12_0: i128 = 2;
        // D s_12_1: read-var p:i64
        let s_12_1: i64 = fn_state.p;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: mul s_12_0 s_12_2
        let s_12_3: i128 = ((s_12_0) * (s_12_2));
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: read-var sel_a:i64
        let s_12_6: i64 = fn_state.sel_a;
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: add s_12_5 s_12_7
        let s_12_8: i128 = (s_12_5 + s_12_7);
        // D s_12_9: cast reint s_12_8 -> i64
        let s_12_9: i64 = (s_12_8 as i64);
        // D s_12_10: read-var esizeshadow#2298:i64
        let s_12_10: i64 = fn_state.esizeshadow_2298;
        // D s_12_11: cast zx s_12_10 -> i
        let s_12_11: i128 = (i128::try_from(s_12_10).unwrap());
        // D s_12_12: cast reint s_12_11 -> i64
        let s_12_12: i64 = (s_12_11 as i64);
        // D s_12_13: cast zx s_12_9 -> i
        let s_12_13: i128 = (i128::try_from(s_12_9).unwrap());
        // D s_12_14: cast zx s_12_12 -> i
        let s_12_14: i128 = (i128::try_from(s_12_12).unwrap());
        // D s_12_15: read-var operand1:bv
        let s_12_15: Bits = fn_state.operand1;
        // D s_12_16: call Elem_read(s_12_15, s_12_13, s_12_14)
        let s_12_16: Bits = Elem_read(state, tracer, s_12_15, s_12_13, s_12_14);
        // D s_12_17: write-var u#2577 <= s_12_16
        fn_state.u_2577 = s_12_16;
        // C s_12_18: const #2s : i
        let s_12_18: i128 = 2;
        // D s_12_19: read-var p:i64
        let s_12_19: i64 = fn_state.p;
        // D s_12_20: cast zx s_12_19 -> i
        let s_12_20: i128 = (i128::try_from(s_12_19).unwrap());
        // D s_12_21: mul s_12_18 s_12_20
        let s_12_21: i128 = ((s_12_18) * (s_12_20));
        // D s_12_22: cast reint s_12_21 -> i64
        let s_12_22: i64 = (s_12_21 as i64);
        // D s_12_23: cast zx s_12_22 -> i
        let s_12_23: i128 = (i128::try_from(s_12_22).unwrap());
        // D s_12_24: read-var sel_b:i64
        let s_12_24: i64 = fn_state.sel_b;
        // D s_12_25: cast zx s_12_24 -> i
        let s_12_25: i128 = (i128::try_from(s_12_24).unwrap());
        // D s_12_26: add s_12_23 s_12_25
        let s_12_26: i128 = (s_12_23 + s_12_25);
        // D s_12_27: cast reint s_12_26 -> i64
        let s_12_27: i64 = (s_12_26 as i64);
        // D s_12_28: read-var esizeshadow#2298:i64
        let s_12_28: i64 = fn_state.esizeshadow_2298;
        // D s_12_29: cast zx s_12_28 -> i
        let s_12_29: i128 = (i128::try_from(s_12_28).unwrap());
        // D s_12_30: cast reint s_12_29 -> i64
        let s_12_30: i64 = (s_12_29 as i64);
        // D s_12_31: cast zx s_12_27 -> i
        let s_12_31: i128 = (i128::try_from(s_12_27).unwrap());
        // D s_12_32: cast zx s_12_30 -> i
        let s_12_32: i128 = (i128::try_from(s_12_30).unwrap());
        // D s_12_33: read-var operand2:bv
        let s_12_33: Bits = fn_state.operand2;
        // D s_12_34: call Elem_read(s_12_33, s_12_31, s_12_32)
        let s_12_34: Bits = Elem_read(state, tracer, s_12_33, s_12_31, s_12_32);
        // D s_12_35: write-var elt2_b <= s_12_34
        fn_state.elt2_b = s_12_34;
        // D s_12_36: read-var neg_i:u8
        let s_12_36: bool = fn_state.neg_i;
        // N s_12_37: branch s_12_36 b22 b13
        if s_12_36 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var esizeshadow#2298:i64
        let s_14_0: i64 = fn_state.esizeshadow_2298;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #16s : i
        let s_14_4: i128 = 16;
        // D s_14_5: cast zx s_14_3 -> i
        let s_14_5: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_6: cmp-eq s_14_5 s_14_4
        let s_14_6: bool = ((s_14_5) == (s_14_4));
        // N s_14_7: branch s_14_6 b21 b15
        if s_14_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esizeshadow#2298:i64
        let s_15_0: i64 = fn_state.esizeshadow_2298;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #32s : i
        let s_15_4: i128 = 32;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: cmp-eq s_15_5 s_15_4
        let s_15_6: bool = ((s_15_5) == (s_15_4));
        // D s_15_7: write-var gs#180985 <= s_15_6
        fn_state.gs_180985 = s_15_6;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#180985:u8
        let s_16_0: bool = fn_state.gs_180985;
        // N s_16_1: branch s_16_0 b20 b17
        if s_16_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var esizeshadow#2298:i64
        let s_17_0: i64 = fn_state.esizeshadow_2298;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call __id(s_17_1)
        let s_17_2: i128 = u__id(state, tracer, s_17_1);
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: const #64s : i
        let s_17_4: i128 = 64;
        // D s_17_5: cast zx s_17_3 -> i
        let s_17_5: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_6: cmp-eq s_17_5 s_17_4
        let s_17_6: bool = ((s_17_5) == (s_17_4));
        // D s_17_7: write-var gs#180987 <= s_17_6
        fn_state.gs_180987 = s_17_6;
        // N s_17_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#180987:u8
        let s_18_0: bool = fn_state.gs_180987;
        // N s_18_1: assert s_18_0
        let s_18_1: () = assert!(s_18_0);
        // C s_18_2: const #() : ()
        let s_18_2: () = ();
        // S s_18_3: call FPCR_read(s_18_2)
        let s_18_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_18_2);
        // D s_18_4: read-var addend_i:bv
        let s_18_4: Bits = fn_state.addend_i;
        // D s_18_5: read-var u#2577:bv
        let s_18_5: Bits = fn_state.u_2577;
        // D s_18_6: read-var elt2_b:bv
        let s_18_6: Bits = fn_state.elt2_b;
        // D s_18_7: call FPMulAdd(s_18_4, s_18_5, s_18_6, s_18_3)
        let s_18_7: Bits = FPMulAdd(state, tracer, s_18_4, s_18_5, s_18_6, s_18_3);
        // D s_18_8: write-var addend_i <= s_18_7
        fn_state.addend_i = s_18_7;
        // N s_18_9: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#180987 <= s_20_0
        fn_state.gs_180987 = s_20_0;
        // N s_20_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#180985 <= s_21_0
        fn_state.gs_180985 = s_21_0;
        // N s_21_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var esizeshadow#2298:i64
        let s_22_0: i64 = fn_state.esizeshadow_2298;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: call __id(s_22_1)
        let s_22_2: i128 = u__id(state, tracer, s_22_1);
        // D s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: const #16s : i
        let s_22_4: i128 = 16;
        // D s_22_5: cast zx s_22_3 -> i
        let s_22_5: i128 = (i128::try_from(s_22_3).unwrap());
        // D s_22_6: cmp-eq s_22_5 s_22_4
        let s_22_6: bool = ((s_22_5) == (s_22_4));
        // N s_22_7: branch s_22_6 b29 b23
        if s_22_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var esizeshadow#2298:i64
        let s_23_0: i64 = fn_state.esizeshadow_2298;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call __id(s_23_1)
        let s_23_2: i128 = u__id(state, tracer, s_23_1);
        // D s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: const #32s : i
        let s_23_4: i128 = 32;
        // D s_23_5: cast zx s_23_3 -> i
        let s_23_5: i128 = (i128::try_from(s_23_3).unwrap());
        // D s_23_6: cmp-eq s_23_5 s_23_4
        let s_23_6: bool = ((s_23_5) == (s_23_4));
        // D s_23_7: write-var gs#180992 <= s_23_6
        fn_state.gs_180992 = s_23_6;
        // N s_23_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#180992:u8
        let s_24_0: bool = fn_state.gs_180992;
        // N s_24_1: branch s_24_0 b28 b25
        if s_24_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var esizeshadow#2298:i64
        let s_25_0: i64 = fn_state.esizeshadow_2298;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call __id(s_25_1)
        let s_25_2: i128 = u__id(state, tracer, s_25_1);
        // D s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: const #64s : i
        let s_25_4: i128 = 64;
        // D s_25_5: cast zx s_25_3 -> i
        let s_25_5: i128 = (i128::try_from(s_25_3).unwrap());
        // D s_25_6: cmp-eq s_25_5 s_25_4
        let s_25_6: bool = ((s_25_5) == (s_25_4));
        // D s_25_7: write-var gs#180994 <= s_25_6
        fn_state.gs_180994 = s_25_6;
        // N s_25_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#180994:u8
        let s_26_0: bool = fn_state.gs_180994;
        // N s_26_1: assert s_26_0
        let s_26_1: () = assert!(s_26_0);
        // D s_26_2: read-var elt2_b:bv
        let s_26_2: Bits = fn_state.elt2_b;
        // D s_26_3: call FPNeg(s_26_2)
        let s_26_3: Bits = FPNeg(state, tracer, s_26_2);
        // D s_26_4: write-var elt2_b <= s_26_3
        fn_state.elt2_b = s_26_3;
        // N s_26_5: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#180994 <= s_28_0
        fn_state.gs_180994 = s_28_0;
        // N s_28_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#180992 <= s_29_0
        fn_state.gs_180992 = s_29_0;
        // N s_29_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #2s : i
        let s_30_0: i128 = 2;
        // D s_30_1: read-var p:i64
        let s_30_1: i64 = fn_state.p;
        // D s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (i128::try_from(s_30_1).unwrap());
        // D s_30_3: mul s_30_0 s_30_2
        let s_30_3: i128 = ((s_30_0) * (s_30_2));
        // D s_30_4: cast reint s_30_3 -> i64
        let s_30_4: i64 = (s_30_3 as i64);
        // D s_30_5: cast zx s_30_4 -> i
        let s_30_5: i128 = (i128::try_from(s_30_4).unwrap());
        // D s_30_6: read-var sel_a:i64
        let s_30_6: i64 = fn_state.sel_a;
        // D s_30_7: cast zx s_30_6 -> i
        let s_30_7: i128 = (i128::try_from(s_30_6).unwrap());
        // D s_30_8: add s_30_5 s_30_7
        let s_30_8: i128 = (s_30_5 + s_30_7);
        // D s_30_9: cast reint s_30_8 -> i64
        let s_30_9: i64 = (s_30_8 as i64);
        // D s_30_10: read-var esizeshadow#2298:i64
        let s_30_10: i64 = fn_state.esizeshadow_2298;
        // D s_30_11: cast zx s_30_10 -> i
        let s_30_11: i128 = (i128::try_from(s_30_10).unwrap());
        // D s_30_12: cast reint s_30_11 -> i64
        let s_30_12: i64 = (s_30_11 as i64);
        // D s_30_13: cast zx s_30_9 -> i
        let s_30_13: i128 = (i128::try_from(s_30_9).unwrap());
        // D s_30_14: cast zx s_30_12 -> i
        let s_30_14: i128 = (i128::try_from(s_30_12).unwrap());
        // D s_30_15: read-var operand1:bv
        let s_30_15: Bits = fn_state.operand1;
        // D s_30_16: call Elem_read(s_30_15, s_30_13, s_30_14)
        let s_30_16: Bits = Elem_read(state, tracer, s_30_15, s_30_13, s_30_14);
        // D s_30_17: write-var elt1_a <= s_30_16
        fn_state.elt1_a = s_30_16;
        // C s_30_18: const #2s : i
        let s_30_18: i128 = 2;
        // D s_30_19: read-var p:i64
        let s_30_19: i64 = fn_state.p;
        // D s_30_20: cast zx s_30_19 -> i
        let s_30_20: i128 = (i128::try_from(s_30_19).unwrap());
        // D s_30_21: mul s_30_18 s_30_20
        let s_30_21: i128 = ((s_30_18) * (s_30_20));
        // D s_30_22: cast reint s_30_21 -> i64
        let s_30_22: i64 = (s_30_21 as i64);
        // D s_30_23: cast zx s_30_22 -> i
        let s_30_23: i128 = (i128::try_from(s_30_22).unwrap());
        // D s_30_24: read-var sel_a:i64
        let s_30_24: i64 = fn_state.sel_a;
        // D s_30_25: cast zx s_30_24 -> i
        let s_30_25: i128 = (i128::try_from(s_30_24).unwrap());
        // D s_30_26: add s_30_23 s_30_25
        let s_30_26: i128 = (s_30_23 + s_30_25);
        // D s_30_27: cast reint s_30_26 -> i64
        let s_30_27: i64 = (s_30_26 as i64);
        // D s_30_28: read-var esizeshadow#2298:i64
        let s_30_28: i64 = fn_state.esizeshadow_2298;
        // D s_30_29: cast zx s_30_28 -> i
        let s_30_29: i128 = (i128::try_from(s_30_28).unwrap());
        // D s_30_30: cast reint s_30_29 -> i64
        let s_30_30: i64 = (s_30_29 as i64);
        // D s_30_31: cast zx s_30_27 -> i
        let s_30_31: i128 = (i128::try_from(s_30_27).unwrap());
        // D s_30_32: cast zx s_30_30 -> i
        let s_30_32: i128 = (i128::try_from(s_30_30).unwrap());
        // D s_30_33: read-var operand2:bv
        let s_30_33: Bits = fn_state.operand2;
        // D s_30_34: call Elem_read(s_30_33, s_30_31, s_30_32)
        let s_30_34: Bits = Elem_read(state, tracer, s_30_33, s_30_31, s_30_32);
        // D s_30_35: write-var elt2_a <= s_30_34
        fn_state.elt2_a = s_30_34;
        // D s_30_36: read-var neg_r:u8
        let s_30_36: bool = fn_state.neg_r;
        // N s_30_37: branch s_30_36 b40 b31
        if s_30_36 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var esizeshadow#2298:i64
        let s_32_0: i64 = fn_state.esizeshadow_2298;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: call __id(s_32_1)
        let s_32_2: i128 = u__id(state, tracer, s_32_1);
        // D s_32_3: cast reint s_32_2 -> i64
        let s_32_3: i64 = (s_32_2 as i64);
        // C s_32_4: const #16s : i
        let s_32_4: i128 = 16;
        // D s_32_5: cast zx s_32_3 -> i
        let s_32_5: i128 = (i128::try_from(s_32_3).unwrap());
        // D s_32_6: cmp-eq s_32_5 s_32_4
        let s_32_6: bool = ((s_32_5) == (s_32_4));
        // N s_32_7: branch s_32_6 b39 b33
        if s_32_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var esizeshadow#2298:i64
        let s_33_0: i64 = fn_state.esizeshadow_2298;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: const #32s : i
        let s_33_4: i128 = 32;
        // D s_33_5: cast zx s_33_3 -> i
        let s_33_5: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_6: cmp-eq s_33_5 s_33_4
        let s_33_6: bool = ((s_33_5) == (s_33_4));
        // D s_33_7: write-var gs#181001 <= s_33_6
        fn_state.gs_181001 = s_33_6;
        // N s_33_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#181001:u8
        let s_34_0: bool = fn_state.gs_181001;
        // N s_34_1: branch s_34_0 b38 b35
        if s_34_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var esizeshadow#2298:i64
        let s_35_0: i64 = fn_state.esizeshadow_2298;
        // D s_35_1: cast zx s_35_0 -> i
        let s_35_1: i128 = (i128::try_from(s_35_0).unwrap());
        // D s_35_2: call __id(s_35_1)
        let s_35_2: i128 = u__id(state, tracer, s_35_1);
        // D s_35_3: cast reint s_35_2 -> i64
        let s_35_3: i64 = (s_35_2 as i64);
        // C s_35_4: const #64s : i
        let s_35_4: i128 = 64;
        // D s_35_5: cast zx s_35_3 -> i
        let s_35_5: i128 = (i128::try_from(s_35_3).unwrap());
        // D s_35_6: cmp-eq s_35_5 s_35_4
        let s_35_6: bool = ((s_35_5) == (s_35_4));
        // D s_35_7: write-var gs#181003 <= s_35_6
        fn_state.gs_181003 = s_35_6;
        // N s_35_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#181003:u8
        let s_36_0: bool = fn_state.gs_181003;
        // N s_36_1: assert s_36_0
        let s_36_1: () = assert!(s_36_0);
        // C s_36_2: const #() : ()
        let s_36_2: () = ();
        // S s_36_3: call FPCR_read(s_36_2)
        let s_36_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_36_2);
        // D s_36_4: read-var addend_r:bv
        let s_36_4: Bits = fn_state.addend_r;
        // D s_36_5: read-var elt1_a:bv
        let s_36_5: Bits = fn_state.elt1_a;
        // D s_36_6: read-var elt2_a:bv
        let s_36_6: Bits = fn_state.elt2_a;
        // D s_36_7: call FPMulAdd(s_36_4, s_36_5, s_36_6, s_36_3)
        let s_36_7: Bits = FPMulAdd(state, tracer, s_36_4, s_36_5, s_36_6, s_36_3);
        // D s_36_8: write-var addend_r <= s_36_7
        fn_state.addend_r = s_36_7;
        // N s_36_9: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#181003 <= s_38_0
        fn_state.gs_181003 = s_38_0;
        // N s_38_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#181001 <= s_39_0
        fn_state.gs_181001 = s_39_0;
        // N s_39_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var esizeshadow#2298:i64
        let s_40_0: i64 = fn_state.esizeshadow_2298;
        // D s_40_1: cast zx s_40_0 -> i
        let s_40_1: i128 = (i128::try_from(s_40_0).unwrap());
        // D s_40_2: call __id(s_40_1)
        let s_40_2: i128 = u__id(state, tracer, s_40_1);
        // D s_40_3: cast reint s_40_2 -> i64
        let s_40_3: i64 = (s_40_2 as i64);
        // C s_40_4: const #16s : i
        let s_40_4: i128 = 16;
        // D s_40_5: cast zx s_40_3 -> i
        let s_40_5: i128 = (i128::try_from(s_40_3).unwrap());
        // D s_40_6: cmp-eq s_40_5 s_40_4
        let s_40_6: bool = ((s_40_5) == (s_40_4));
        // N s_40_7: branch s_40_6 b47 b41
        if s_40_6 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var esizeshadow#2298:i64
        let s_41_0: i64 = fn_state.esizeshadow_2298;
        // D s_41_1: cast zx s_41_0 -> i
        let s_41_1: i128 = (i128::try_from(s_41_0).unwrap());
        // D s_41_2: call __id(s_41_1)
        let s_41_2: i128 = u__id(state, tracer, s_41_1);
        // D s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: const #32s : i
        let s_41_4: i128 = 32;
        // D s_41_5: cast zx s_41_3 -> i
        let s_41_5: i128 = (i128::try_from(s_41_3).unwrap());
        // D s_41_6: cmp-eq s_41_5 s_41_4
        let s_41_6: bool = ((s_41_5) == (s_41_4));
        // D s_41_7: write-var gs#181008 <= s_41_6
        fn_state.gs_181008 = s_41_6;
        // N s_41_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#181008:u8
        let s_42_0: bool = fn_state.gs_181008;
        // N s_42_1: branch s_42_0 b46 b43
        if s_42_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var esizeshadow#2298:i64
        let s_43_0: i64 = fn_state.esizeshadow_2298;
        // D s_43_1: cast zx s_43_0 -> i
        let s_43_1: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_2: call __id(s_43_1)
        let s_43_2: i128 = u__id(state, tracer, s_43_1);
        // D s_43_3: cast reint s_43_2 -> i64
        let s_43_3: i64 = (s_43_2 as i64);
        // C s_43_4: const #64s : i
        let s_43_4: i128 = 64;
        // D s_43_5: cast zx s_43_3 -> i
        let s_43_5: i128 = (i128::try_from(s_43_3).unwrap());
        // D s_43_6: cmp-eq s_43_5 s_43_4
        let s_43_6: bool = ((s_43_5) == (s_43_4));
        // D s_43_7: write-var gs#181010 <= s_43_6
        fn_state.gs_181010 = s_43_6;
        // N s_43_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#181010:u8
        let s_44_0: bool = fn_state.gs_181010;
        // N s_44_1: assert s_44_0
        let s_44_1: () = assert!(s_44_0);
        // D s_44_2: read-var elt2_a:bv
        let s_44_2: Bits = fn_state.elt2_a;
        // D s_44_3: call FPNeg(s_44_2)
        let s_44_3: Bits = FPNeg(state, tracer, s_44_2);
        // D s_44_4: write-var elt2_a <= s_44_3
        fn_state.elt2_a = s_44_3;
        // N s_44_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#181010 <= s_46_0
        fn_state.gs_181010 = s_46_0;
        // N s_46_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#181008 <= s_47_0
        fn_state.gs_181008 = s_47_0;
        // N s_47_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var VLshadow#2300:i64
        let s_48_0: i64 = fn_state.VLshadow_2300;
        // D s_48_1: cast zx s_48_0 -> i
        let s_48_1: i128 = (i128::try_from(s_48_0).unwrap());
        // D s_48_2: cast reint s_48_1 -> i64
        let s_48_2: i64 = (s_48_1 as i64);
        // D s_48_3: read-var da:i64
        let s_48_3: i64 = fn_state.da;
        // D s_48_4: cast zx s_48_3 -> i
        let s_48_4: i128 = (i128::try_from(s_48_3).unwrap());
        // D s_48_5: cast zx s_48_2 -> i
        let s_48_5: i128 = (i128::try_from(s_48_2).unwrap());
        // D s_48_6: read-var result:bv
        let s_48_6: Bits = fn_state.result;
        // D s_48_7: call Z_set(s_48_4, s_48_5, s_48_6)
        let s_48_7: () = Z_set(state, tracer, s_48_4, s_48_5, s_48_6);
        // N s_48_8: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var VLshadow#2300:i64
        let s_49_0: i64 = fn_state.VLshadow_2300;
        // D s_49_1: cast zx s_49_0 -> i
        let s_49_1: i128 = (i128::try_from(s_49_0).unwrap());
        // D s_49_2: cast reint s_49_1 -> i64
        let s_49_2: i64 = (s_49_1 as i64);
        // D s_49_3: read-var m:i64
        let s_49_3: i64 = fn_state.m;
        // D s_49_4: cast zx s_49_3 -> i
        let s_49_4: i128 = (i128::try_from(s_49_3).unwrap());
        // D s_49_5: cast zx s_49_2 -> i
        let s_49_5: i128 = (i128::try_from(s_49_2).unwrap());
        // D s_49_6: call Z_read(s_49_4, s_49_5)
        let s_49_6: Bits = Z_read(state, tracer, s_49_4, s_49_5);
        // D s_49_7: write-var operand2 <= s_49_6
        fn_state.operand2 = s_49_6;
        // N s_49_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var VLshadow#2300:i64
        let s_50_0: i64 = fn_state.VLshadow_2300;
        // D s_50_1: cast zx s_50_0 -> i
        let s_50_1: i128 = (i128::try_from(s_50_0).unwrap());
        // D s_50_2: cast reint s_50_1 -> i64
        let s_50_2: i64 = (s_50_1 as i64);
        // D s_50_3: read-var n:i64
        let s_50_3: i64 = fn_state.n;
        // D s_50_4: cast zx s_50_3 -> i
        let s_50_4: i128 = (i128::try_from(s_50_3).unwrap());
        // D s_50_5: cast zx s_50_2 -> i
        let s_50_5: i128 = (i128::try_from(s_50_2).unwrap());
        // D s_50_6: call Z_read(s_50_4, s_50_5)
        let s_50_6: Bits = Z_read(state, tracer, s_50_4, s_50_5);
        // D s_50_7: write-var operand1 <= s_50_6
        fn_state.operand1 = s_50_6;
        // N s_50_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
