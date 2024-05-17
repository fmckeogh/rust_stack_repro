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
use BFMulAddH::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_BFMLSLB_Z_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    m: i64,
    n: i64,
    op1_neg: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: u16,
        gs_731333: Bits,
        e: i64,
        VLshadow_2362: i64,
        element1: u16,
        element3: u32,
        operand3: Bits,
        gs_731338: Bits,
        result: Bits,
        gs_182639: i64,
        operand1: Bits,
        VLshadow_2363: i64,
        operand2: Bits,
        VL: i64,
        da: i64,
        m: i64,
        n: i64,
        op1_neg: bool,
    }
    let fn_state = FunctionState {
        VL,
        da,
        m,
        n,
        op1_neg,
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
        // D s_0_3: write-var VLshadow#2362 <= s_0_2
        fn_state.VLshadow_2362 = s_0_2;
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
        // D s_1_0: read-var VLshadow#2362:i64
        let s_1_0: i64 = fn_state.VLshadow_2362;
        // D s_1_1: write-var VLshadow#2363 <= s_1_0
        fn_state.VLshadow_2363 = s_1_0;
        // C s_1_2: const #32s : i
        let s_1_2: i128 = 32;
        // D s_1_3: read-var VLshadow#2363:i64
        let s_1_3: i64 = fn_state.VLshadow_2363;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#2363:i64
        let s_1_7: i64 = fn_state.VLshadow_2363;
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
        // D s_1_15: read-var VLshadow#2363:i64
        let s_1_15: i64 = fn_state.VLshadow_2363;
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
        // D s_1_23: read-var VLshadow#2363:i64
        let s_1_23: i64 = fn_state.VLshadow_2363;
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
        // D s_1_30: write-var operand3 <= s_1_29
        fn_state.operand3 = s_1_29;
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
        // D s_1_36: write-var gs#182639 <= s_1_35
        fn_state.gs_182639 = s_1_35;
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
        // D s_2_1: read-var gs#182639:i64
        let s_2_1: i64 = fn_state.gs_182639;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b9 b3
        if s_2_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // C s_3_5: const #0s : i
        let s_3_5: i128 = 0;
        // D s_3_6: cast zx s_3_4 -> i
        let s_3_6: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // C s_3_9: const #16s : i64
        let s_3_9: i64 = 16;
        // D s_3_10: cast zx s_3_8 -> i
        let s_3_10: i128 = (i128::try_from(s_3_8).unwrap());
        // C s_3_11: cast zx s_3_9 -> i
        let s_3_11: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_12: read-var operand1:bv
        let s_3_12: Bits = fn_state.operand1;
        // D s_3_13: call Elem_read(s_3_12, s_3_10, s_3_11)
        let s_3_13: Bits = Elem_read(state, tracer, s_3_12, s_3_10, s_3_11);
        // D s_3_14: cast reint s_3_13 -> u16
        let s_3_14: u16 = (s_3_13.value() as u16);
        // D s_3_15: write-var element1 <= s_3_14
        fn_state.element1 = s_3_14;
        // C s_3_16: const #2s : i
        let s_3_16: i128 = 2;
        // D s_3_17: read-var e:i64
        let s_3_17: i64 = fn_state.e;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: mul s_3_16 s_3_18
        let s_3_19: i128 = ((s_3_16) * (s_3_18));
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // C s_3_21: const #0s : i
        let s_3_21: i128 = 0;
        // D s_3_22: cast zx s_3_20 -> i
        let s_3_22: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_23: add s_3_22 s_3_21
        let s_3_23: i128 = (s_3_22 + s_3_21);
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // C s_3_25: const #16s : i64
        let s_3_25: i64 = 16;
        // D s_3_26: cast zx s_3_24 -> i
        let s_3_26: i128 = (i128::try_from(s_3_24).unwrap());
        // C s_3_27: cast zx s_3_25 -> i
        let s_3_27: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_28: read-var operand2:bv
        let s_3_28: Bits = fn_state.operand2;
        // D s_3_29: call Elem_read(s_3_28, s_3_26, s_3_27)
        let s_3_29: Bits = Elem_read(state, tracer, s_3_28, s_3_26, s_3_27);
        // D s_3_30: cast reint s_3_29 -> u16
        let s_3_30: u16 = (s_3_29.value() as u16);
        // D s_3_31: write-var element2 <= s_3_30
        fn_state.element2 = s_3_30;
        // C s_3_32: const #32s : i64
        let s_3_32: i64 = 32;
        // D s_3_33: read-var e:i64
        let s_3_33: i64 = fn_state.e;
        // D s_3_34: cast zx s_3_33 -> i
        let s_3_34: i128 = (i128::try_from(s_3_33).unwrap());
        // C s_3_35: cast zx s_3_32 -> i
        let s_3_35: i128 = (i128::try_from(s_3_32).unwrap());
        // D s_3_36: read-var operand3:bv
        let s_3_36: Bits = fn_state.operand3;
        // D s_3_37: call Elem_read(s_3_36, s_3_34, s_3_35)
        let s_3_37: Bits = Elem_read(state, tracer, s_3_36, s_3_34, s_3_35);
        // D s_3_38: cast reint s_3_37 -> u32
        let s_3_38: u32 = (s_3_37.value() as u32);
        // D s_3_39: write-var element3 <= s_3_38
        fn_state.element3 = s_3_38;
        // D s_3_40: read-var op1_neg:u8
        let s_3_40: bool = fn_state.op1_neg;
        // N s_3_41: branch s_3_40 b7 b4
        if s_3_40 {
            return block_7(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call FPCR_read(s_5_0)
        let s_5_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_5_0);
        // D s_5_2: read-var element3:u32
        let s_5_2: u32 = fn_state.element3;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_4: read-var element1:u16
        let s_5_4: u16 = fn_state.element1;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 16u16);
        // D s_5_6: read-var element2:u16
        let s_5_6: u16 = fn_state.element2;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 16u16);
        // D s_5_8: call BFMulAddH(s_5_3, s_5_5, s_5_7, s_5_1)
        let s_5_8: Bits = BFMulAddH(state, tracer, s_5_3, s_5_5, s_5_7, s_5_1);
        // D s_5_9: write-var gs#731338 <= s_5_8
        fn_state.gs_731338 = s_5_8;
        // N s_5_10: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#731338:bv
        let s_6_0: Bits = fn_state.gs_731338;
        // D s_6_1: cast reint s_6_0 -> u32
        let s_6_1: u32 = (s_6_0.value() as u32);
        // D s_6_2: read-var e:i64
        let s_6_2: i64 = fn_state.e;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // C s_6_4: const #32s : i64
        let s_6_4: i64 = 32;
        // C s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: cast zx s_6_1 -> bv
        let s_6_6: Bits = Bits::new(s_6_1 as u128, 32u16);
        // D s_6_7: read-var result:bv
        let s_6_7: Bits = fn_state.result;
        // D s_6_8: call Elem_set(s_6_7, s_6_3, s_6_5, s_6_6)
        let s_6_8: Bits = Elem_set(state, tracer, s_6_7, s_6_3, s_6_5, s_6_6);
        // D s_6_9: write-var result <= s_6_8
        fn_state.result = s_6_8;
        // D s_6_10: read-var e:i64
        let s_6_10: i64 = fn_state.e;
        // C s_6_11: const #1s : i64
        let s_6_11: i64 = 1;
        // D s_6_12: add s_6_10 s_6_11
        let s_6_12: i64 = (s_6_10 + s_6_11);
        // D s_6_13: write-var e <= s_6_12
        fn_state.e = s_6_12;
        // N s_6_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var element1:u16
        let s_7_0: u16 = fn_state.element1;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 16u16);
        // D s_7_2: call BFNeg(s_7_1)
        let s_7_2: Bits = BFNeg(state, tracer, s_7_1);
        // D s_7_3: write-var gs#731333 <= s_7_2
        fn_state.gs_731333 = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#731333:bv
        let s_8_0: Bits = fn_state.gs_731333;
        // D s_8_1: cast reint s_8_0 -> u16
        let s_8_1: u16 = (s_8_0.value() as u16);
        // D s_8_2: write-var element1 <= s_8_1
        fn_state.element1 = s_8_1;
        // N s_8_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VLshadow#2363:i64
        let s_9_0: i64 = fn_state.VLshadow_2363;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var da:i64
        let s_9_3: i64 = fn_state.da;
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
}
