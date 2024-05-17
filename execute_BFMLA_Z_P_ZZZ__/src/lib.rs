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
use BFMulAdd::*;
use CheckSVEEnabled::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use FPCR_read::*;
use BFNeg::*;
use Elem_read::*;
use Zeros::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_BFMLA_Z_P_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    g: i64,
    m: i64,
    n: i64,
    op1_neg: bool,
    op3_neg: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_733944: Bits,
        element3: u16,
        operand3: Bits,
        VLshadow_2442: i64,
        gs_733951: Bits,
        mask: Bits,
        gs_184668: i64,
        element2: u16,
        element1: u16,
        elements: i64,
        VLshadow_2441: i64,
        gs_733946: Bits,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        da: i64,
        g: i64,
        m: i64,
        n: i64,
        op1_neg: bool,
        op3_neg: bool,
    }
    let fn_state = FunctionState {
        VL,
        da,
        g,
        m,
        n,
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
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#2441 <= s_0_2
        fn_state.VLshadow_2441 = s_0_2;
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
        // D s_1_0: read-var VLshadow#2441:i64
        let s_1_0: i64 = fn_state.VLshadow_2441;
        // D s_1_1: write-var VLshadow#2442 <= s_1_0
        fn_state.VLshadow_2442 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#2442:i64
        let s_1_3: i64 = fn_state.VLshadow_2442;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #16s : i
        let s_1_7: i128 = 16;
        // D s_1_8: read-var VLshadow#2442:i64
        let s_1_8: i64 = fn_state.VLshadow_2442;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: div s_1_9 s_1_7
        let s_1_10: i128 = ((s_1_9) / (s_1_7));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var elements <= s_1_11
        fn_state.elements = s_1_11;
        // D s_1_13: cast zx s_1_6 -> i
        let s_1_13: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var g:i64
        let s_1_15: i64 = fn_state.g;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call P_read(s_1_16, s_1_17)
        let s_1_18: Bits = P_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var mask <= s_1_18
        fn_state.mask = s_1_18;
        // C s_1_20: const #16s : i
        let s_1_20: i128 = 16;
        // D s_1_21: read-var mask:bv
        let s_1_21: Bits = fn_state.mask;
        // D s_1_22: call AnyActiveElement(s_1_21, s_1_20)
        let s_1_22: bool = AnyActiveElement(state, tracer, s_1_21, s_1_20);
        // N s_1_23: branch s_1_22 b22 b2
        if s_1_22 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2442:i64
        let s_2_0: i64 = fn_state.VLshadow_2442;
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
        // C s_3_0: const #16s : i
        let s_3_0: i128 = 16;
        // D s_3_1: read-var mask:bv
        let s_3_1: Bits = fn_state.mask;
        // D s_3_2: call AnyActiveElement(s_3_1, s_3_0)
        let s_3_2: bool = AnyActiveElement(state, tracer, s_3_1, s_3_0);
        // N s_3_3: branch s_3_2 b21 b4
        if s_3_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2442:i64
        let s_4_0: i64 = fn_state.VLshadow_2442;
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
        // D s_5_0: read-var VLshadow#2442:i64
        let s_5_0: i64 = fn_state.VLshadow_2442;
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
        // D s_5_10: read-var elements:i64
        let s_5_10: i64 = fn_state.elements;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: sub s_5_11 s_5_9
        let s_5_12: i128 = ((s_5_11) - (s_5_9));
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var gs#184668 <= s_5_13
        fn_state.gs_184668 = s_5_13;
        // D s_5_15: write-var e <= s_5_8
        fn_state.e = s_5_8;
        // N s_5_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#184668:i64
        let s_6_1: i64 = fn_state.gs_184668;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b20 b7
        if s_6_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16s : i
        let s_7_0: i128 = 16;
        // D s_7_1: read-var e:i64
        let s_7_1: i64 = fn_state.e;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: read-var mask:bv
        let s_7_3: Bits = fn_state.mask;
        // D s_7_4: call ActivePredicateElement(s_7_3, s_7_2, s_7_0)
        let s_7_4: bool = ActivePredicateElement(state, tracer, s_7_3, s_7_2, s_7_0);
        // N s_7_5: branch s_7_4 b10 b8
        if s_7_4 {
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
        // C s_8_0: const #16s : i64
        let s_8_0: i64 = 16;
        // C s_8_1: const #16s : i64
        let s_8_1: i64 = 16;
        // D s_8_2: read-var e:i64
        let s_8_2: i64 = fn_state.e;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // C s_8_4: cast zx s_8_1 -> i
        let s_8_4: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_5: read-var operand3:bv
        let s_8_5: Bits = fn_state.operand3;
        // D s_8_6: call Elem_read(s_8_5, s_8_3, s_8_4)
        let s_8_6: Bits = Elem_read(state, tracer, s_8_5, s_8_3, s_8_4);
        // D s_8_7: cast reint s_8_6 -> u16
        let s_8_7: u16 = (s_8_6.value() as u16);
        // D s_8_8: read-var e:i64
        let s_8_8: i64 = fn_state.e;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // C s_8_10: cast zx s_8_0 -> i
        let s_8_10: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_11: cast zx s_8_7 -> bv
        let s_8_11: Bits = Bits::new(s_8_7 as u128, 16u16);
        // D s_8_12: read-var result:bv
        let s_8_12: Bits = fn_state.result;
        // D s_8_13: call Elem_set(s_8_12, s_8_9, s_8_10, s_8_11)
        let s_8_13: Bits = Elem_set(state, tracer, s_8_12, s_8_9, s_8_10, s_8_11);
        // D s_8_14: write-var result <= s_8_13
        fn_state.result = s_8_13;
        // N s_8_15: jump b9
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
        // C s_10_0: const #16s : i64
        let s_10_0: i64 = 16;
        // D s_10_1: read-var e:i64
        let s_10_1: i64 = fn_state.e;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // C s_10_3: cast zx s_10_0 -> i
        let s_10_3: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_4: read-var operand1:bv
        let s_10_4: Bits = fn_state.operand1;
        // D s_10_5: call Elem_read(s_10_4, s_10_2, s_10_3)
        let s_10_5: Bits = Elem_read(state, tracer, s_10_4, s_10_2, s_10_3);
        // D s_10_6: cast reint s_10_5 -> u16
        let s_10_6: u16 = (s_10_5.value() as u16);
        // D s_10_7: write-var element1 <= s_10_6
        fn_state.element1 = s_10_6;
        // C s_10_8: const #16s : i64
        let s_10_8: i64 = 16;
        // D s_10_9: read-var e:i64
        let s_10_9: i64 = fn_state.e;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // C s_10_11: cast zx s_10_8 -> i
        let s_10_11: i128 = (i128::try_from(s_10_8).unwrap());
        // D s_10_12: read-var operand2:bv
        let s_10_12: Bits = fn_state.operand2;
        // D s_10_13: call Elem_read(s_10_12, s_10_10, s_10_11)
        let s_10_13: Bits = Elem_read(state, tracer, s_10_12, s_10_10, s_10_11);
        // D s_10_14: cast reint s_10_13 -> u16
        let s_10_14: u16 = (s_10_13.value() as u16);
        // D s_10_15: write-var element2 <= s_10_14
        fn_state.element2 = s_10_14;
        // C s_10_16: const #16s : i64
        let s_10_16: i64 = 16;
        // D s_10_17: read-var e:i64
        let s_10_17: i64 = fn_state.e;
        // D s_10_18: cast zx s_10_17 -> i
        let s_10_18: i128 = (i128::try_from(s_10_17).unwrap());
        // C s_10_19: cast zx s_10_16 -> i
        let s_10_19: i128 = (i128::try_from(s_10_16).unwrap());
        // D s_10_20: read-var operand3:bv
        let s_10_20: Bits = fn_state.operand3;
        // D s_10_21: call Elem_read(s_10_20, s_10_18, s_10_19)
        let s_10_21: Bits = Elem_read(state, tracer, s_10_20, s_10_18, s_10_19);
        // D s_10_22: cast reint s_10_21 -> u16
        let s_10_22: u16 = (s_10_21.value() as u16);
        // D s_10_23: write-var element3 <= s_10_22
        fn_state.element3 = s_10_22;
        // D s_10_24: read-var op1_neg:u8
        let s_10_24: bool = fn_state.op1_neg;
        // N s_10_25: branch s_10_24 b18 b11
        if s_10_24 {
            return block_18(state, tracer, fn_state);
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
        // N s_12_1: branch s_12_0 b16 b13
        if s_12_0 {
            return block_16(state, tracer, fn_state);
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
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call FPCR_read(s_14_0)
        let s_14_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_14_0);
        // D s_14_2: read-var element3:u16
        let s_14_2: u16 = fn_state.element3;
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 16u16);
        // D s_14_4: read-var element1:u16
        let s_14_4: u16 = fn_state.element1;
        // D s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 16u16);
        // D s_14_6: read-var element2:u16
        let s_14_6: u16 = fn_state.element2;
        // D s_14_7: cast zx s_14_6 -> bv
        let s_14_7: Bits = Bits::new(s_14_6 as u128, 16u16);
        // D s_14_8: call BFMulAdd(s_14_3, s_14_5, s_14_7, s_14_1)
        let s_14_8: Bits = BFMulAdd(state, tracer, s_14_3, s_14_5, s_14_7, s_14_1);
        // D s_14_9: write-var gs#733951 <= s_14_8
        fn_state.gs_733951 = s_14_8;
        // N s_14_10: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#733951:bv
        let s_15_0: Bits = fn_state.gs_733951;
        // D s_15_1: cast reint s_15_0 -> u16
        let s_15_1: u16 = (s_15_0.value() as u16);
        // D s_15_2: read-var e:i64
        let s_15_2: i64 = fn_state.e;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // C s_15_4: const #16s : i64
        let s_15_4: i64 = 16;
        // C s_15_5: cast zx s_15_4 -> i
        let s_15_5: i128 = (i128::try_from(s_15_4).unwrap());
        // D s_15_6: cast zx s_15_1 -> bv
        let s_15_6: Bits = Bits::new(s_15_1 as u128, 16u16);
        // D s_15_7: read-var result:bv
        let s_15_7: Bits = fn_state.result;
        // D s_15_8: call Elem_set(s_15_7, s_15_3, s_15_5, s_15_6)
        let s_15_8: Bits = Elem_set(state, tracer, s_15_7, s_15_3, s_15_5, s_15_6);
        // D s_15_9: write-var result <= s_15_8
        fn_state.result = s_15_8;
        // N s_15_10: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var element3:u16
        let s_16_0: u16 = fn_state.element3;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 16u16);
        // D s_16_2: call BFNeg(s_16_1)
        let s_16_2: Bits = BFNeg(state, tracer, s_16_1);
        // D s_16_3: write-var gs#733946 <= s_16_2
        fn_state.gs_733946 = s_16_2;
        // N s_16_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#733946:bv
        let s_17_0: Bits = fn_state.gs_733946;
        // D s_17_1: cast reint s_17_0 -> u16
        let s_17_1: u16 = (s_17_0.value() as u16);
        // D s_17_2: write-var element3 <= s_17_1
        fn_state.element3 = s_17_1;
        // N s_17_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var element1:u16
        let s_18_0: u16 = fn_state.element1;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 16u16);
        // D s_18_2: call BFNeg(s_18_1)
        let s_18_2: Bits = BFNeg(state, tracer, s_18_1);
        // D s_18_3: write-var gs#733944 <= s_18_2
        fn_state.gs_733944 = s_18_2;
        // N s_18_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#733944:bv
        let s_19_0: Bits = fn_state.gs_733944;
        // D s_19_1: cast reint s_19_0 -> u16
        let s_19_1: u16 = (s_19_0.value() as u16);
        // D s_19_2: write-var element1 <= s_19_1
        fn_state.element1 = s_19_1;
        // N s_19_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var VLshadow#2442:i64
        let s_20_0: i64 = fn_state.VLshadow_2442;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: cast reint s_20_1 -> i64
        let s_20_2: i64 = (s_20_1 as i64);
        // D s_20_3: read-var da:i64
        let s_20_3: i64 = fn_state.da;
        // D s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_5: cast zx s_20_2 -> i
        let s_20_5: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_6: read-var result:bv
        let s_20_6: Bits = fn_state.result;
        // D s_20_7: call Z_set(s_20_4, s_20_5, s_20_6)
        let s_20_7: () = Z_set(state, tracer, s_20_4, s_20_5, s_20_6);
        // N s_20_8: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var VLshadow#2442:i64
        let s_21_0: i64 = fn_state.VLshadow_2442;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: cast reint s_21_1 -> i64
        let s_21_2: i64 = (s_21_1 as i64);
        // D s_21_3: read-var m:i64
        let s_21_3: i64 = fn_state.m;
        // D s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_5: cast zx s_21_2 -> i
        let s_21_5: i128 = (i128::try_from(s_21_2).unwrap());
        // D s_21_6: call Z_read(s_21_4, s_21_5)
        let s_21_6: Bits = Z_read(state, tracer, s_21_4, s_21_5);
        // D s_21_7: write-var operand2 <= s_21_6
        fn_state.operand2 = s_21_6;
        // N s_21_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var VLshadow#2442:i64
        let s_22_0: i64 = fn_state.VLshadow_2442;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: cast reint s_22_1 -> i64
        let s_22_2: i64 = (s_22_1 as i64);
        // D s_22_3: read-var n:i64
        let s_22_3: i64 = fn_state.n;
        // D s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // D s_22_5: cast zx s_22_2 -> i
        let s_22_5: i128 = (i128::try_from(s_22_2).unwrap());
        // D s_22_6: call Z_read(s_22_4, s_22_5)
        let s_22_6: Bits = Z_read(state, tracer, s_22_4, s_22_5);
        // D s_22_7: write-var operand1 <= s_22_6
        fn_state.operand1 = s_22_6;
        // N s_22_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
