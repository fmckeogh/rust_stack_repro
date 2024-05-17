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
pub fn execute_FNMAD_Z_P_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    a: i64,
    dn: i64,
    esize: i64,
    g: i64,
    m: i64,
    op1_neg: bool,
    op3_neg: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        element3: Bits,
        gs_180277: bool,
        operand3: Bits,
        gs_180270: bool,
        mask: Bits,
        ga_274511: i64,
        element2: Bits,
        gs_180268: bool,
        element1: Bits,
        gs_180262: i64,
        VLshadow_2264: i64,
        ga_274512: Bits,
        gs_180284: bool,
        elements: i64,
        gs_180275: bool,
        result: Bits,
        operand1: Bits,
        esizeshadow_2262: i64,
        gs_180282: bool,
        VLshadow_2263: i64,
        operand2: Bits,
        VL: i64,
        a: i64,
        dn: i64,
        esize: i64,
        g: i64,
        m: i64,
        op1_neg: bool,
        op3_neg: bool,
    }
    let fn_state = FunctionState {
        VL,
        a,
        dn,
        esize,
        g,
        m,
        op1_neg,
        op3_neg,
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
        // D s_0_3: write-var esizeshadow#2262 <= s_0_2
        fn_state.esizeshadow_2262 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2263 <= s_0_6
        fn_state.VLshadow_2263 = s_0_6;
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
        // D s_1_0: read-var VLshadow#2263:i64
        let s_1_0: i64 = fn_state.VLshadow_2263;
        // D s_1_1: write-var VLshadow#2264 <= s_1_0
        fn_state.VLshadow_2264 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2264:i64
        let s_1_3: i64 = fn_state.VLshadow_2264;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2264:i64
        let s_1_7: i64 = fn_state.VLshadow_2264;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#2262:i64
        let s_1_9: i64 = fn_state.esizeshadow_2262;
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
        // D s_1_21: read-var VLshadow#2264:i64
        let s_1_21: i64 = fn_state.VLshadow_2264;
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
        // D s_1_29: read-var esizeshadow#2262:i64
        let s_1_29: i64 = fn_state.esizeshadow_2262;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: read-var mask:bv
        let s_1_31: Bits = fn_state.mask;
        // D s_1_32: call AnyActiveElement(s_1_31, s_1_30)
        let s_1_32: bool = AnyActiveElement(state, tracer, s_1_31, s_1_30);
        // N s_1_33: branch s_1_32 b40 b2
        if s_1_32 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2264:i64
        let s_2_0: i64 = fn_state.VLshadow_2264;
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
        // D s_3_0: read-var esizeshadow#2262:i64
        let s_3_0: i64 = fn_state.esizeshadow_2262;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var mask:bv
        let s_3_2: Bits = fn_state.mask;
        // D s_3_3: call AnyActiveElement(s_3_2, s_3_1)
        let s_3_3: bool = AnyActiveElement(state, tracer, s_3_2, s_3_1);
        // N s_3_4: branch s_3_3 b39 b4
        if s_3_3 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2264:i64
        let s_4_0: i64 = fn_state.VLshadow_2264;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call Zeros(s_4_1)
        let s_4_2: Bits = Zeros(state, tracer, s_4_1);
        // D s_4_3: write-var operand3 <= s_4_2
        fn_state.operand3 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var elements:i64
        let s_5_2: i64 = fn_state.elements;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: sub s_5_3 s_5_1
        let s_5_4: i128 = ((s_5_3) - (s_5_1));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: write-var gs#180262 <= s_5_5
        fn_state.gs_180262 = s_5_5;
        // D s_5_7: write-var e <= s_5_0
        fn_state.e = s_5_0;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#180262:i64
        let s_6_1: i64 = fn_state.gs_180262;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b38 b7
        if s_6_2 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var esizeshadow#2262:i64
        let s_7_2: i64 = fn_state.esizeshadow_2262;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var mask:bv
        let s_7_4: Bits = fn_state.mask;
        // D s_7_5: call ActivePredicateElement(s_7_4, s_7_1, s_7_3)
        let s_7_5: bool = ActivePredicateElement(state, tracer, s_7_4, s_7_1, s_7_3);
        // N s_7_6: branch s_7_5 b10 b8
        if s_7_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#2262:i64
        let s_8_0: i64 = fn_state.esizeshadow_2262;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var esizeshadow#2262:i64
        let s_8_3: i64 = fn_state.esizeshadow_2262;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: read-var e:i64
        let s_8_6: i64 = fn_state.e;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: cast zx s_8_5 -> i
        let s_8_8: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_9: read-var operand1:bv
        let s_8_9: Bits = fn_state.operand1;
        // D s_8_10: call Elem_read(s_8_9, s_8_7, s_8_8)
        let s_8_10: Bits = Elem_read(state, tracer, s_8_9, s_8_7, s_8_8);
        // D s_8_11: read-var e:i64
        let s_8_11: i64 = fn_state.e;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: cast zx s_8_2 -> i
        let s_8_13: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_14: read-var result:bv
        let s_8_14: Bits = fn_state.result;
        // D s_8_15: call Elem_set(s_8_14, s_8_12, s_8_13, s_8_10)
        let s_8_15: Bits = Elem_set(state, tracer, s_8_14, s_8_12, s_8_13, s_8_10);
        // D s_8_16: write-var result <= s_8_15
        fn_state.result = s_8_15;
        // N s_8_17: jump b9
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
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#2262:i64
        let s_10_0: i64 = fn_state.esizeshadow_2262;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var e:i64
        let s_10_3: i64 = fn_state.e;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: read-var operand1:bv
        let s_10_6: Bits = fn_state.operand1;
        // D s_10_7: call Elem_read(s_10_6, s_10_4, s_10_5)
        let s_10_7: Bits = Elem_read(state, tracer, s_10_6, s_10_4, s_10_5);
        // D s_10_8: write-var element1 <= s_10_7
        fn_state.element1 = s_10_7;
        // D s_10_9: read-var esizeshadow#2262:i64
        let s_10_9: i64 = fn_state.esizeshadow_2262;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_11: cast reint s_10_10 -> i64
        let s_10_11: i64 = (s_10_10 as i64);
        // D s_10_12: read-var e:i64
        let s_10_12: i64 = fn_state.e;
        // D s_10_13: cast zx s_10_12 -> i
        let s_10_13: i128 = (i128::try_from(s_10_12).unwrap());
        // D s_10_14: cast zx s_10_11 -> i
        let s_10_14: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_15: read-var operand2:bv
        let s_10_15: Bits = fn_state.operand2;
        // D s_10_16: call Elem_read(s_10_15, s_10_13, s_10_14)
        let s_10_16: Bits = Elem_read(state, tracer, s_10_15, s_10_13, s_10_14);
        // D s_10_17: write-var element2 <= s_10_16
        fn_state.element2 = s_10_16;
        // D s_10_18: read-var esizeshadow#2262:i64
        let s_10_18: i64 = fn_state.esizeshadow_2262;
        // D s_10_19: cast zx s_10_18 -> i
        let s_10_19: i128 = (i128::try_from(s_10_18).unwrap());
        // D s_10_20: cast reint s_10_19 -> i64
        let s_10_20: i64 = (s_10_19 as i64);
        // D s_10_21: read-var e:i64
        let s_10_21: i64 = fn_state.e;
        // D s_10_22: cast zx s_10_21 -> i
        let s_10_22: i128 = (i128::try_from(s_10_21).unwrap());
        // D s_10_23: cast zx s_10_20 -> i
        let s_10_23: i128 = (i128::try_from(s_10_20).unwrap());
        // D s_10_24: read-var operand3:bv
        let s_10_24: Bits = fn_state.operand3;
        // D s_10_25: call Elem_read(s_10_24, s_10_22, s_10_23)
        let s_10_25: Bits = Elem_read(state, tracer, s_10_24, s_10_22, s_10_23);
        // D s_10_26: write-var element3 <= s_10_25
        fn_state.element3 = s_10_25;
        // D s_10_27: read-var op1_neg:u8
        let s_10_27: bool = fn_state.op1_neg;
        // N s_10_28: branch s_10_27 b30 b11
        if s_10_27 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var op3_neg:u8
        let s_12_0: bool = fn_state.op3_neg;
        // N s_12_1: branch s_12_0 b22 b13
        if s_12_0 {
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
        // D s_14_0: read-var esizeshadow#2262:i64
        let s_14_0: i64 = fn_state.esizeshadow_2262;
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
        // D s_15_0: read-var esizeshadow#2262:i64
        let s_15_0: i64 = fn_state.esizeshadow_2262;
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
        // D s_15_7: write-var gs#180268 <= s_15_6
        fn_state.gs_180268 = s_15_6;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#180268:u8
        let s_16_0: bool = fn_state.gs_180268;
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
        // D s_17_0: read-var esizeshadow#2262:i64
        let s_17_0: i64 = fn_state.esizeshadow_2262;
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
        // D s_17_7: write-var gs#180270 <= s_17_6
        fn_state.gs_180270 = s_17_6;
        // N s_17_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#180270:u8
        let s_18_0: bool = fn_state.gs_180270;
        // N s_18_1: assert s_18_0
        let s_18_1: () = assert!(s_18_0);
        // D s_18_2: read-var esizeshadow#2262:i64
        let s_18_2: i64 = fn_state.esizeshadow_2262;
        // D s_18_3: cast zx s_18_2 -> i
        let s_18_3: i128 = (i128::try_from(s_18_2).unwrap());
        // D s_18_4: cast reint s_18_3 -> i64
        let s_18_4: i64 = (s_18_3 as i64);
        // D s_18_5: write-var ga#274511 <= s_18_4
        fn_state.ga_274511 = s_18_4;
        // C s_18_6: const #() : ()
        let s_18_6: () = ();
        // S s_18_7: call FPCR_read(s_18_6)
        let s_18_7: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_18_6);
        // D s_18_8: read-var element3:bv
        let s_18_8: Bits = fn_state.element3;
        // D s_18_9: read-var element1:bv
        let s_18_9: Bits = fn_state.element1;
        // D s_18_10: read-var element2:bv
        let s_18_10: Bits = fn_state.element2;
        // D s_18_11: call FPMulAdd(s_18_8, s_18_9, s_18_10, s_18_7)
        let s_18_11: Bits = FPMulAdd(state, tracer, s_18_8, s_18_9, s_18_10, s_18_7);
        // D s_18_12: write-var ga#274512 <= s_18_11
        fn_state.ga_274512 = s_18_11;
        // N s_18_13: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var e:i64
        let s_19_0: i64 = fn_state.e;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: read-var ga#274511:i64
        let s_19_2: i64 = fn_state.ga_274511;
        // D s_19_3: cast zx s_19_2 -> i
        let s_19_3: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_4: read-var result:bv
        let s_19_4: Bits = fn_state.result;
        // D s_19_5: read-var ga#274512:bv
        let s_19_5: Bits = fn_state.ga_274512;
        // D s_19_6: call Elem_set(s_19_4, s_19_1, s_19_3, s_19_5)
        let s_19_6: Bits = Elem_set(state, tracer, s_19_4, s_19_1, s_19_3, s_19_5);
        // D s_19_7: write-var result <= s_19_6
        fn_state.result = s_19_6;
        // N s_19_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#180270 <= s_20_0
        fn_state.gs_180270 = s_20_0;
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
        // D s_21_1: write-var gs#180268 <= s_21_0
        fn_state.gs_180268 = s_21_0;
        // N s_21_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var esizeshadow#2262:i64
        let s_22_0: i64 = fn_state.esizeshadow_2262;
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
        // D s_23_0: read-var esizeshadow#2262:i64
        let s_23_0: i64 = fn_state.esizeshadow_2262;
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
        // D s_23_7: write-var gs#180275 <= s_23_6
        fn_state.gs_180275 = s_23_6;
        // N s_23_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#180275:u8
        let s_24_0: bool = fn_state.gs_180275;
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
        // D s_25_0: read-var esizeshadow#2262:i64
        let s_25_0: i64 = fn_state.esizeshadow_2262;
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
        // D s_25_7: write-var gs#180277 <= s_25_6
        fn_state.gs_180277 = s_25_6;
        // N s_25_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#180277:u8
        let s_26_0: bool = fn_state.gs_180277;
        // N s_26_1: assert s_26_0
        let s_26_1: () = assert!(s_26_0);
        // D s_26_2: read-var element3:bv
        let s_26_2: Bits = fn_state.element3;
        // D s_26_3: call FPNeg(s_26_2)
        let s_26_3: Bits = FPNeg(state, tracer, s_26_2);
        // D s_26_4: write-var element3 <= s_26_3
        fn_state.element3 = s_26_3;
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
        // D s_28_1: write-var gs#180277 <= s_28_0
        fn_state.gs_180277 = s_28_0;
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
        // D s_29_1: write-var gs#180275 <= s_29_0
        fn_state.gs_180275 = s_29_0;
        // N s_29_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var esizeshadow#2262:i64
        let s_30_0: i64 = fn_state.esizeshadow_2262;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call __id(s_30_1)
        let s_30_2: i128 = u__id(state, tracer, s_30_1);
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #16s : i
        let s_30_4: i128 = 16;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: cmp-eq s_30_5 s_30_4
        let s_30_6: bool = ((s_30_5) == (s_30_4));
        // N s_30_7: branch s_30_6 b37 b31
        if s_30_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var esizeshadow#2262:i64
        let s_31_0: i64 = fn_state.esizeshadow_2262;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // D s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #32s : i
        let s_31_4: i128 = 32;
        // D s_31_5: cast zx s_31_3 -> i
        let s_31_5: i128 = (i128::try_from(s_31_3).unwrap());
        // D s_31_6: cmp-eq s_31_5 s_31_4
        let s_31_6: bool = ((s_31_5) == (s_31_4));
        // D s_31_7: write-var gs#180282 <= s_31_6
        fn_state.gs_180282 = s_31_6;
        // N s_31_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#180282:u8
        let s_32_0: bool = fn_state.gs_180282;
        // N s_32_1: branch s_32_0 b36 b33
        if s_32_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var esizeshadow#2262:i64
        let s_33_0: i64 = fn_state.esizeshadow_2262;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // D s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: const #64s : i
        let s_33_4: i128 = 64;
        // D s_33_5: cast zx s_33_3 -> i
        let s_33_5: i128 = (i128::try_from(s_33_3).unwrap());
        // D s_33_6: cmp-eq s_33_5 s_33_4
        let s_33_6: bool = ((s_33_5) == (s_33_4));
        // D s_33_7: write-var gs#180284 <= s_33_6
        fn_state.gs_180284 = s_33_6;
        // N s_33_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#180284:u8
        let s_34_0: bool = fn_state.gs_180284;
        // N s_34_1: assert s_34_0
        let s_34_1: () = assert!(s_34_0);
        // D s_34_2: read-var element1:bv
        let s_34_2: Bits = fn_state.element1;
        // D s_34_3: call FPNeg(s_34_2)
        let s_34_3: Bits = FPNeg(state, tracer, s_34_2);
        // D s_34_4: write-var element1 <= s_34_3
        fn_state.element1 = s_34_3;
        // N s_34_5: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#180284 <= s_36_0
        fn_state.gs_180284 = s_36_0;
        // N s_36_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#180282 <= s_37_0
        fn_state.gs_180282 = s_37_0;
        // N s_37_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var VLshadow#2264:i64
        let s_38_0: i64 = fn_state.VLshadow_2264;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: cast reint s_38_1 -> i64
        let s_38_2: i64 = (s_38_1 as i64);
        // D s_38_3: read-var dn:i64
        let s_38_3: i64 = fn_state.dn;
        // D s_38_4: cast zx s_38_3 -> i
        let s_38_4: i128 = (i128::try_from(s_38_3).unwrap());
        // D s_38_5: cast zx s_38_2 -> i
        let s_38_5: i128 = (i128::try_from(s_38_2).unwrap());
        // D s_38_6: read-var result:bv
        let s_38_6: Bits = fn_state.result;
        // D s_38_7: call Z_set(s_38_4, s_38_5, s_38_6)
        let s_38_7: () = Z_set(state, tracer, s_38_4, s_38_5, s_38_6);
        // N s_38_8: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var VLshadow#2264:i64
        let s_39_0: i64 = fn_state.VLshadow_2264;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: cast reint s_39_1 -> i64
        let s_39_2: i64 = (s_39_1 as i64);
        // D s_39_3: read-var a:i64
        let s_39_3: i64 = fn_state.a;
        // D s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_5: cast zx s_39_2 -> i
        let s_39_5: i128 = (i128::try_from(s_39_2).unwrap());
        // D s_39_6: call Z_read(s_39_4, s_39_5)
        let s_39_6: Bits = Z_read(state, tracer, s_39_4, s_39_5);
        // D s_39_7: write-var operand3 <= s_39_6
        fn_state.operand3 = s_39_6;
        // N s_39_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var VLshadow#2264:i64
        let s_40_0: i64 = fn_state.VLshadow_2264;
        // D s_40_1: cast zx s_40_0 -> i
        let s_40_1: i128 = (i128::try_from(s_40_0).unwrap());
        // D s_40_2: cast reint s_40_1 -> i64
        let s_40_2: i64 = (s_40_1 as i64);
        // D s_40_3: read-var m:i64
        let s_40_3: i64 = fn_state.m;
        // D s_40_4: cast zx s_40_3 -> i
        let s_40_4: i128 = (i128::try_from(s_40_3).unwrap());
        // D s_40_5: cast zx s_40_2 -> i
        let s_40_5: i128 = (i128::try_from(s_40_2).unwrap());
        // D s_40_6: call Z_read(s_40_4, s_40_5)
        let s_40_6: Bits = Z_read(state, tracer, s_40_4, s_40_5);
        // D s_40_7: write-var operand2 <= s_40_6
        fn_state.operand2 = s_40_6;
        // N s_40_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
