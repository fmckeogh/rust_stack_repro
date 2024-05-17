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
use BFNeg::*;
use Elem_set::*;
use CheckSVEEnabled::*;
use FPCR_read::*;
use BFMulAdd::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_BFMLS_Z_ZZZi_H<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    index: i64,
    m: i64,
    n: i64,
    op1_neg: bool,
    op3_neg: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: u16,
        gs_734180: Bits,
        e: i64,
        element1: u16,
        element3: u16,
        VLshadow_2448: i64,
        gs_184834: i64,
        VLshadow_2447: i64,
        gs_734173: Bits,
        result: Bits,
        operand1: Bits,
        gs_734175: Bits,
        operand2: Bits,
        VL: i64,
        da: i64,
        index: i64,
        m: i64,
        n: i64,
        op1_neg: bool,
        op3_neg: bool,
    }
    let fn_state = FunctionState {
        VL,
        da,
        index,
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
        // D s_0_3: write-var VLshadow#2447 <= s_0_2
        fn_state.VLshadow_2447 = s_0_2;
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
        // D s_1_0: read-var VLshadow#2447:i64
        let s_1_0: i64 = fn_state.VLshadow_2447;
        // D s_1_1: write-var VLshadow#2448 <= s_1_0
        fn_state.VLshadow_2448 = s_1_0;
        // C s_1_2: const #16s : i
        let s_1_2: i128 = 16;
        // D s_1_3: read-var VLshadow#2448:i64
        let s_1_3: i64 = fn_state.VLshadow_2448;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2448:i64
        let s_1_7: i64 = fn_state.VLshadow_2448;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var n:i64
        let s_1_10: i64 = fn_state.n;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast zx s_1_9 -> i
        let s_1_12: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_13: call Z_read(s_1_11, s_1_12)
        let s_1_13: Bits = Z_read(state, tracer, s_1_11, s_1_12);
        // D s_1_14: write-var operand1 <= s_1_13
        fn_state.operand1 = s_1_13;
        // D s_1_15: read-var VLshadow#2448:i64
        let s_1_15: i64 = fn_state.VLshadow_2448;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: read-var m:i64
        let s_1_18: i64 = fn_state.m;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast zx s_1_17 -> i
        let s_1_20: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_21: call Z_read(s_1_19, s_1_20)
        let s_1_21: Bits = Z_read(state, tracer, s_1_19, s_1_20);
        // D s_1_22: write-var operand2 <= s_1_21
        fn_state.operand2 = s_1_21;
        // D s_1_23: read-var VLshadow#2448:i64
        let s_1_23: i64 = fn_state.VLshadow_2448;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: read-var da:i64
        let s_1_26: i64 = fn_state.da;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: cast zx s_1_25 -> i
        let s_1_28: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_29: call Z_read(s_1_27, s_1_28)
        let s_1_29: Bits = Z_read(state, tracer, s_1_27, s_1_28);
        // D s_1_30: write-var result <= s_1_29
        fn_state.result = s_1_29;
        // C s_1_31: const #0s : i64
        let s_1_31: i64 = 0;
        // C s_1_32: const #1s : i
        let s_1_32: i128 = 1;
        // D s_1_33: cast zx s_1_6 -> i
        let s_1_33: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_34: sub s_1_33 s_1_32
        let s_1_34: i128 = ((s_1_33) - (s_1_32));
        // D s_1_35: cast reint s_1_34 -> i64
        let s_1_35: i64 = (s_1_34 as i64);
        // D s_1_36: write-var gs#184834 <= s_1_35
        fn_state.gs_184834 = s_1_35;
        // D s_1_37: write-var e <= s_1_31
        fn_state.e = s_1_31;
        // N s_1_38: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#184834:i64
        let s_2_1: i64 = fn_state.gs_184834;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b13 b3
        if s_2_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mod s_3_2 s_3_0
        let s_3_3: i128 = ((s_3_2) % (s_3_0));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: read-var e:i64
        let s_3_5: i64 = fn_state.e;
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: cast zx s_3_4 -> i
        let s_3_7: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_8: sub s_3_6 s_3_7
        let s_3_8: i128 = ((s_3_6) - (s_3_7));
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: read-var index:i64
        let s_3_11: i64 = fn_state.index;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: add s_3_10 s_3_12
        let s_3_13: i128 = (s_3_10 + s_3_12);
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // C s_3_15: const #16s : i64
        let s_3_15: i64 = 16;
        // D s_3_16: read-var e:i64
        let s_3_16: i64 = fn_state.e;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // C s_3_18: cast zx s_3_15 -> i
        let s_3_18: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_19: read-var operand1:bv
        let s_3_19: Bits = fn_state.operand1;
        // D s_3_20: call Elem_read(s_3_19, s_3_17, s_3_18)
        let s_3_20: Bits = Elem_read(state, tracer, s_3_19, s_3_17, s_3_18);
        // D s_3_21: cast reint s_3_20 -> u16
        let s_3_21: u16 = (s_3_20.value() as u16);
        // D s_3_22: write-var element1 <= s_3_21
        fn_state.element1 = s_3_21;
        // C s_3_23: const #16s : i64
        let s_3_23: i64 = 16;
        // D s_3_24: cast zx s_3_14 -> i
        let s_3_24: i128 = (i128::try_from(s_3_14).unwrap());
        // C s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_26: read-var operand2:bv
        let s_3_26: Bits = fn_state.operand2;
        // D s_3_27: call Elem_read(s_3_26, s_3_24, s_3_25)
        let s_3_27: Bits = Elem_read(state, tracer, s_3_26, s_3_24, s_3_25);
        // D s_3_28: cast reint s_3_27 -> u16
        let s_3_28: u16 = (s_3_27.value() as u16);
        // D s_3_29: write-var element2 <= s_3_28
        fn_state.element2 = s_3_28;
        // C s_3_30: const #16s : i64
        let s_3_30: i64 = 16;
        // D s_3_31: read-var e:i64
        let s_3_31: i64 = fn_state.e;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // C s_3_33: cast zx s_3_30 -> i
        let s_3_33: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_34: read-var result:bv
        let s_3_34: Bits = fn_state.result;
        // D s_3_35: call Elem_read(s_3_34, s_3_32, s_3_33)
        let s_3_35: Bits = Elem_read(state, tracer, s_3_34, s_3_32, s_3_33);
        // D s_3_36: cast reint s_3_35 -> u16
        let s_3_36: u16 = (s_3_35.value() as u16);
        // D s_3_37: write-var element3 <= s_3_36
        fn_state.element3 = s_3_36;
        // D s_3_38: read-var op1_neg:u8
        let s_3_38: bool = fn_state.op1_neg;
        // N s_3_39: branch s_3_38 b11 b4
        if s_3_38 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_0: read-var op3_neg:u8
        let s_5_0: bool = fn_state.op3_neg;
        // N s_5_1: branch s_5_0 b9 b6
        if s_5_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call FPCR_read(s_7_0)
        let s_7_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_7_0);
        // D s_7_2: read-var element3:u16
        let s_7_2: u16 = fn_state.element3;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 16u16);
        // D s_7_4: read-var element1:u16
        let s_7_4: u16 = fn_state.element1;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 16u16);
        // D s_7_6: read-var element2:u16
        let s_7_6: u16 = fn_state.element2;
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 16u16);
        // D s_7_8: call BFMulAdd(s_7_3, s_7_5, s_7_7, s_7_1)
        let s_7_8: Bits = BFMulAdd(state, tracer, s_7_3, s_7_5, s_7_7, s_7_1);
        // D s_7_9: write-var gs#734180 <= s_7_8
        fn_state.gs_734180 = s_7_8;
        // N s_7_10: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#734180:bv
        let s_8_0: Bits = fn_state.gs_734180;
        // D s_8_1: cast reint s_8_0 -> u16
        let s_8_1: u16 = (s_8_0.value() as u16);
        // D s_8_2: read-var e:i64
        let s_8_2: i64 = fn_state.e;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // C s_8_4: const #16s : i64
        let s_8_4: i64 = 16;
        // C s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: cast zx s_8_1 -> bv
        let s_8_6: Bits = Bits::new(s_8_1 as u128, 16u16);
        // D s_8_7: read-var result:bv
        let s_8_7: Bits = fn_state.result;
        // D s_8_8: call Elem_set(s_8_7, s_8_3, s_8_5, s_8_6)
        let s_8_8: Bits = Elem_set(state, tracer, s_8_7, s_8_3, s_8_5, s_8_6);
        // D s_8_9: write-var result <= s_8_8
        fn_state.result = s_8_8;
        // D s_8_10: read-var e:i64
        let s_8_10: i64 = fn_state.e;
        // C s_8_11: const #1s : i64
        let s_8_11: i64 = 1;
        // D s_8_12: add s_8_10 s_8_11
        let s_8_12: i64 = (s_8_10 + s_8_11);
        // D s_8_13: write-var e <= s_8_12
        fn_state.e = s_8_12;
        // N s_8_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var element3:u16
        let s_9_0: u16 = fn_state.element3;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 16u16);
        // D s_9_2: call BFNeg(s_9_1)
        let s_9_2: Bits = BFNeg(state, tracer, s_9_1);
        // D s_9_3: write-var gs#734175 <= s_9_2
        fn_state.gs_734175 = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#734175:bv
        let s_10_0: Bits = fn_state.gs_734175;
        // D s_10_1: cast reint s_10_0 -> u16
        let s_10_1: u16 = (s_10_0.value() as u16);
        // D s_10_2: write-var element3 <= s_10_1
        fn_state.element3 = s_10_1;
        // N s_10_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var element1:u16
        let s_11_0: u16 = fn_state.element1;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 16u16);
        // D s_11_2: call BFNeg(s_11_1)
        let s_11_2: Bits = BFNeg(state, tracer, s_11_1);
        // D s_11_3: write-var gs#734173 <= s_11_2
        fn_state.gs_734173 = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#734173:bv
        let s_12_0: Bits = fn_state.gs_734173;
        // D s_12_1: cast reint s_12_0 -> u16
        let s_12_1: u16 = (s_12_0.value() as u16);
        // D s_12_2: write-var element1 <= s_12_1
        fn_state.element1 = s_12_1;
        // N s_12_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VLshadow#2448:i64
        let s_13_0: i64 = fn_state.VLshadow_2448;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: read-var da:i64
        let s_13_3: i64 = fn_state.da;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: cast zx s_13_2 -> i
        let s_13_5: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_6: read-var result:bv
        let s_13_6: Bits = fn_state.result;
        // D s_13_7: call Z_set(s_13_4, s_13_5, s_13_6)
        let s_13_7: () = Z_set(state, tracer, s_13_4, s_13_5, s_13_6);
        // N s_13_8: return
        return;
    }
}
