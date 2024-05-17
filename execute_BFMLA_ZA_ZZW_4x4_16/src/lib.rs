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
use BFNeg::*;
use CheckStreamingSVEAndZAEnabled::*;
use FPCR_read::*;
use BFMulAdd_ZA::*;
use X_read::*;
use Elem_read::*;
use ZAvector_set::*;
use Z_read::*;
use ZAvector_read::*;
use common::*;
pub fn execute_BFMLA_ZA_ZZW_4x4_16<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    m: i64,
    n: i64,
    nreg: i64,
    offset: i64,
    sub_op: bool,
    v: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        gs_293111: i64,
        element3: u16,
        operand3: Bits,
        VLshadow_7010: i64,
        gs_293103: i64,
        vstride: i64,
        vec: i128,
        element2: u16,
        gs_874850: Bits,
        element1: u16,
        VLshadow_7011: i64,
        gs_874845: Bits,
        elements: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        m: i64,
        n: i64,
        nreg: i64,
        offset: i64,
        sub_op: bool,
        v: i64,
    }
    let fn_state = FunctionState {
        VL,
        m,
        n,
        nreg,
        offset,
        sub_op,
        v,
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
        // D s_0_3: write-var VLshadow#7010 <= s_0_2
        fn_state.VLshadow_7010 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckStreamingSVEAndZAEnabled(s_0_4)
        let s_0_5: () = CheckStreamingSVEAndZAEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#7010:i64
        let s_1_0: i64 = fn_state.VLshadow_7010;
        // D s_1_1: write-var VLshadow#7011 <= s_1_0
        fn_state.VLshadow_7011 = s_1_0;
        // C s_1_2: const #16s : i
        let s_1_2: i128 = 16;
        // D s_1_3: read-var VLshadow#7011:i64
        let s_1_3: i64 = fn_state.VLshadow_7011;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var elements <= s_1_6
        fn_state.elements = s_1_6;
        // C s_1_8: const #8s : i
        let s_1_8: i128 = 8;
        // D s_1_9: read-var VLshadow#7011:i64
        let s_1_9: i64 = fn_state.VLshadow_7011;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_10 s_1_8
        let s_1_11: i128 = ((s_1_10) / (s_1_8));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: read-var nreg:i64
        let s_1_14: i64 = fn_state.nreg;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: div s_1_13 s_1_15
        let s_1_16: i128 = ((s_1_13) / (s_1_15));
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: write-var vstride <= s_1_17
        fn_state.vstride = s_1_17;
        // C s_1_19: const #32s : i64
        let s_1_19: i64 = 32;
        // D s_1_20: read-var v:i64
        let s_1_20: i64 = fn_state.v;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: call X_read(s_1_21, s_1_19)
        let s_1_22: Bits = X_read(state, tracer, s_1_21, s_1_19);
        // D s_1_23: cast reint s_1_22 -> u32
        let s_1_23: u32 = (s_1_22.value() as u32);
        // D s_1_24: cast zx s_1_23 -> bv
        let s_1_24: Bits = Bits::new(s_1_23 as u128, 32u16);
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (s_1_24.value() as i128);
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: read-var offset:i64
        let s_1_28: i64 = fn_state.offset;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: add s_1_27 s_1_29
        let s_1_30: i128 = (s_1_27 + s_1_29);
        // D s_1_31: cast reint s_1_30 -> i64
        let s_1_31: i64 = (s_1_30 as i64);
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: read-var vstride:i64
        let s_1_33: i64 = fn_state.vstride;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: mod s_1_32 s_1_34
        let s_1_35: i128 = ((s_1_32) % (s_1_34));
        // D s_1_36: write-var vec <= s_1_35
        fn_state.vec = s_1_35;
        // C s_1_37: const #0s : i64
        let s_1_37: i64 = 0;
        // C s_1_38: const #1s : i
        let s_1_38: i128 = 1;
        // D s_1_39: read-var nreg:i64
        let s_1_39: i64 = fn_state.nreg;
        // D s_1_40: cast zx s_1_39 -> i
        let s_1_40: i128 = (i128::try_from(s_1_39).unwrap());
        // D s_1_41: sub s_1_40 s_1_38
        let s_1_41: i128 = ((s_1_40) - (s_1_38));
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: write-var gs#293103 <= s_1_42
        fn_state.gs_293103 = s_1_42;
        // D s_1_44: write-var r <= s_1_37
        fn_state.r = s_1_37;
        // N s_1_45: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#293103:i64
        let s_2_1: i64 = fn_state.gs_293103;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b12 b3
        if s_2_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var r:i64
        let s_3_2: i64 = fn_state.r;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var VLshadow#7011:i64
        let s_3_6: i64 = fn_state.VLshadow_7011;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: cast zx s_3_5 -> i
        let s_3_9: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_10: cast zx s_3_8 -> i
        let s_3_10: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_11: call Z_read(s_3_9, s_3_10)
        let s_3_11: Bits = Z_read(state, tracer, s_3_9, s_3_10);
        // D s_3_12: write-var operand1 <= s_3_11
        fn_state.operand1 = s_3_11;
        // D s_3_13: read-var m:i64
        let s_3_13: i64 = fn_state.m;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: read-var r:i64
        let s_3_15: i64 = fn_state.r;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: add s_3_14 s_3_16
        let s_3_17: i128 = (s_3_14 + s_3_16);
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: read-var VLshadow#7011:i64
        let s_3_19: i64 = fn_state.VLshadow_7011;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: cast zx s_3_18 -> i
        let s_3_22: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_23: cast zx s_3_21 -> i
        let s_3_23: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_24: call Z_read(s_3_22, s_3_23)
        let s_3_24: Bits = Z_read(state, tracer, s_3_22, s_3_23);
        // D s_3_25: write-var operand2 <= s_3_24
        fn_state.operand2 = s_3_24;
        // D s_3_26: read-var vec:i
        let s_3_26: i128 = fn_state.vec;
        // D s_3_27: read-var VLshadow#7011:i64
        let s_3_27: i64 = fn_state.VLshadow_7011;
        // D s_3_28: cast zx s_3_27 -> i
        let s_3_28: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_29: cast reint s_3_28 -> i64
        let s_3_29: i64 = (s_3_28 as i64);
        // D s_3_30: cast zx s_3_29 -> i
        let s_3_30: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_31: call ZAvector_read(s_3_26, s_3_30)
        let s_3_31: Bits = ZAvector_read(state, tracer, s_3_26, s_3_30);
        // D s_3_32: write-var operand3 <= s_3_31
        fn_state.operand3 = s_3_31;
        // C s_3_33: const #0s : i64
        let s_3_33: i64 = 0;
        // C s_3_34: const #1s : i
        let s_3_34: i128 = 1;
        // D s_3_35: read-var elements:i64
        let s_3_35: i64 = fn_state.elements;
        // D s_3_36: cast zx s_3_35 -> i
        let s_3_36: i128 = (i128::try_from(s_3_35).unwrap());
        // D s_3_37: sub s_3_36 s_3_34
        let s_3_37: i128 = ((s_3_36) - (s_3_34));
        // D s_3_38: cast reint s_3_37 -> i64
        let s_3_38: i64 = (s_3_37 as i64);
        // D s_3_39: write-var gs#293111 <= s_3_38
        fn_state.gs_293111 = s_3_38;
        // D s_3_40: write-var e <= s_3_33
        fn_state.e = s_3_33;
        // N s_3_41: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#293111:i64
        let s_4_1: i64 = fn_state.gs_293111;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b11 b5
        if s_4_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16s : i64
        let s_5_0: i64 = 16;
        // D s_5_1: read-var e:i64
        let s_5_1: i64 = fn_state.e;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // C s_5_3: cast zx s_5_0 -> i
        let s_5_3: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_4: read-var operand1:bv
        let s_5_4: Bits = fn_state.operand1;
        // D s_5_5: call Elem_read(s_5_4, s_5_2, s_5_3)
        let s_5_5: Bits = Elem_read(state, tracer, s_5_4, s_5_2, s_5_3);
        // D s_5_6: cast reint s_5_5 -> u16
        let s_5_6: u16 = (s_5_5.value() as u16);
        // D s_5_7: write-var element1 <= s_5_6
        fn_state.element1 = s_5_6;
        // C s_5_8: const #16s : i64
        let s_5_8: i64 = 16;
        // D s_5_9: read-var e:i64
        let s_5_9: i64 = fn_state.e;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // C s_5_11: cast zx s_5_8 -> i
        let s_5_11: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_12: read-var operand2:bv
        let s_5_12: Bits = fn_state.operand2;
        // D s_5_13: call Elem_read(s_5_12, s_5_10, s_5_11)
        let s_5_13: Bits = Elem_read(state, tracer, s_5_12, s_5_10, s_5_11);
        // D s_5_14: cast reint s_5_13 -> u16
        let s_5_14: u16 = (s_5_13.value() as u16);
        // D s_5_15: write-var element2 <= s_5_14
        fn_state.element2 = s_5_14;
        // C s_5_16: const #16s : i64
        let s_5_16: i64 = 16;
        // D s_5_17: read-var e:i64
        let s_5_17: i64 = fn_state.e;
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // C s_5_19: cast zx s_5_16 -> i
        let s_5_19: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_20: read-var operand3:bv
        let s_5_20: Bits = fn_state.operand3;
        // D s_5_21: call Elem_read(s_5_20, s_5_18, s_5_19)
        let s_5_21: Bits = Elem_read(state, tracer, s_5_20, s_5_18, s_5_19);
        // D s_5_22: cast reint s_5_21 -> u16
        let s_5_22: u16 = (s_5_21.value() as u16);
        // D s_5_23: write-var element3 <= s_5_22
        fn_state.element3 = s_5_22;
        // D s_5_24: read-var sub_op:u8
        let s_5_24: bool = fn_state.sub_op;
        // N s_5_25: branch s_5_24 b9 b6
        if s_5_24 {
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
        // D s_7_8: call BFMulAdd_ZA(s_7_3, s_7_5, s_7_7, s_7_1)
        let s_7_8: Bits = BFMulAdd_ZA(state, tracer, s_7_3, s_7_5, s_7_7, s_7_1);
        // D s_7_9: write-var gs#874850 <= s_7_8
        fn_state.gs_874850 = s_7_8;
        // N s_7_10: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#874850:bv
        let s_8_0: Bits = fn_state.gs_874850;
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
        // N s_8_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var element1:u16
        let s_9_0: u16 = fn_state.element1;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 16u16);
        // D s_9_2: call BFNeg(s_9_1)
        let s_9_2: Bits = BFNeg(state, tracer, s_9_1);
        // D s_9_3: write-var gs#874845 <= s_9_2
        fn_state.gs_874845 = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#874845:bv
        let s_10_0: Bits = fn_state.gs_874845;
        // D s_10_1: cast reint s_10_0 -> u16
        let s_10_1: u16 = (s_10_0.value() as u16);
        // D s_10_2: write-var element1 <= s_10_1
        fn_state.element1 = s_10_1;
        // N s_10_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var vec:i
        let s_11_0: i128 = fn_state.vec;
        // D s_11_1: read-var VLshadow#7011:i64
        let s_11_1: i64 = fn_state.VLshadow_7011;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: read-var result:bv
        let s_11_5: Bits = fn_state.result;
        // D s_11_6: call ZAvector_set(s_11_0, s_11_4, s_11_5)
        let s_11_6: () = ZAvector_set(state, tracer, s_11_0, s_11_4, s_11_5);
        // D s_11_7: read-var vstride:i64
        let s_11_7: i64 = fn_state.vstride;
        // D s_11_8: cast zx s_11_7 -> i
        let s_11_8: i128 = (i128::try_from(s_11_7).unwrap());
        // D s_11_9: read-var vec:i
        let s_11_9: i128 = fn_state.vec;
        // D s_11_10: add s_11_9 s_11_8
        let s_11_10: i128 = (s_11_9 + s_11_8);
        // D s_11_11: write-var vec <= s_11_10
        fn_state.vec = s_11_10;
        // D s_11_12: read-var r:i64
        let s_11_12: i64 = fn_state.r;
        // C s_11_13: const #1s : i64
        let s_11_13: i64 = 1;
        // D s_11_14: add s_11_12 s_11_13
        let s_11_14: i64 = (s_11_12 + s_11_13);
        // D s_11_15: write-var r <= s_11_14
        fn_state.r = s_11_14;
        // N s_11_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
}
