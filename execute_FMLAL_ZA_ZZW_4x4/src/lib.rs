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
use CheckStreamingSVEAndZAEnabled::*;
use FPCR_read::*;
use FPNeg::*;
use fmod_int::*;
use X_read::*;
use Elem_read::*;
use FPMulAddH_ZA::*;
use Z_read::*;
use ZAvector_set::*;
use ZAvector_read::*;
use common::*;
pub fn execute_FMLAL_ZA_ZZW_4x4<T: Tracer>(
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
        element3: u32,
        gs_839522: Bits,
        operand3: Bits,
        VLshadow_5941: i64,
        vecshadow_5943: i128,
        vstride: i64,
        vec: i128,
        gs_268366: i64,
        element2: u16,
        element1: u16,
        gs_268379: i64,
        elements: i64,
        result: Bits,
        i: i64,
        gs_839517: Bits,
        operand1: Bits,
        VLshadow_5942: i64,
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
        // D s_0_3: write-var VLshadow#5941 <= s_0_2
        fn_state.VLshadow_5941 = s_0_2;
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
        // D s_1_0: read-var VLshadow#5941:i64
        let s_1_0: i64 = fn_state.VLshadow_5941;
        // D s_1_1: write-var VLshadow#5942 <= s_1_0
        fn_state.VLshadow_5942 = s_1_0;
        // C s_1_2: const #32s : i
        let s_1_2: i128 = 32;
        // D s_1_3: read-var VLshadow#5942:i64
        let s_1_3: i64 = fn_state.VLshadow_5942;
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
        // D s_1_9: read-var VLshadow#5942:i64
        let s_1_9: i64 = fn_state.VLshadow_5942;
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
        // C s_1_37: const #2s : i
        let s_1_37: i128 = 2;
        // D s_1_38: read-var vec:i
        let s_1_38: i128 = fn_state.vec;
        // D s_1_39: call fmod_int(s_1_38, s_1_37)
        let s_1_39: i128 = fmod_int(state, tracer, s_1_38, s_1_37);
        // D s_1_40: read-var vec:i
        let s_1_40: i128 = fn_state.vec;
        // D s_1_41: sub s_1_40 s_1_39
        let s_1_41: i128 = ((s_1_40) - (s_1_39));
        // D s_1_42: write-var vec <= s_1_41
        fn_state.vec = s_1_41;
        // C s_1_43: const #0s : i64
        let s_1_43: i64 = 0;
        // C s_1_44: const #1s : i
        let s_1_44: i128 = 1;
        // D s_1_45: read-var nreg:i64
        let s_1_45: i64 = fn_state.nreg;
        // D s_1_46: cast zx s_1_45 -> i
        let s_1_46: i128 = (i128::try_from(s_1_45).unwrap());
        // D s_1_47: sub s_1_46 s_1_44
        let s_1_47: i128 = ((s_1_46) - (s_1_44));
        // D s_1_48: cast reint s_1_47 -> i64
        let s_1_48: i64 = (s_1_47 as i64);
        // D s_1_49: write-var gs#268366 <= s_1_48
        fn_state.gs_268366 = s_1_48;
        // D s_1_50: write-var r <= s_1_43
        fn_state.r = s_1_43;
        // N s_1_51: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#268366:i64
        let s_2_1: i64 = fn_state.gs_268366;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b15 b3
        if s_2_2 {
            return block_15(state, tracer, fn_state);
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
        // D s_3_6: read-var VLshadow#5942:i64
        let s_3_6: i64 = fn_state.VLshadow_5942;
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
        // D s_3_19: read-var VLshadow#5942:i64
        let s_3_19: i64 = fn_state.VLshadow_5942;
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
        // D s_3_27: write-var vecshadow#5943 <= s_3_26
        fn_state.vecshadow_5943 = s_3_26;
        // C s_3_28: const #0s : i64
        let s_3_28: i64 = 0;
        // D s_3_29: write-var i <= s_3_28
        fn_state.i = s_3_28;
        // N s_3_30: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #1s : i64
        let s_4_1: i64 = 1;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b14 b5
        if s_4_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var i:i64
        let s_5_0: i64 = fn_state.i;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var vecshadow#5943:i
        let s_5_2: i128 = fn_state.vecshadow_5943;
        // D s_5_3: add s_5_2 s_5_1
        let s_5_3: i128 = (s_5_2 + s_5_1);
        // D s_5_4: read-var VLshadow#5942:i64
        let s_5_4: i64 = fn_state.VLshadow_5942;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: call ZAvector_read(s_5_3, s_5_7)
        let s_5_8: Bits = ZAvector_read(state, tracer, s_5_3, s_5_7);
        // D s_5_9: write-var operand3 <= s_5_8
        fn_state.operand3 = s_5_8;
        // C s_5_10: const #0s : i64
        let s_5_10: i64 = 0;
        // C s_5_11: const #1s : i
        let s_5_11: i128 = 1;
        // D s_5_12: read-var elements:i64
        let s_5_12: i64 = fn_state.elements;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: sub s_5_13 s_5_11
        let s_5_14: i128 = ((s_5_13) - (s_5_11));
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // D s_5_16: write-var gs#268379 <= s_5_15
        fn_state.gs_268379 = s_5_15;
        // D s_5_17: write-var e <= s_5_10
        fn_state.e = s_5_10;
        // N s_5_18: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#268379:i64
        let s_6_1: i64 = fn_state.gs_268379;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b13 b7
        if s_6_2 {
            return block_13(state, tracer, fn_state);
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
        // D s_7_1: read-var e:i64
        let s_7_1: i64 = fn_state.e;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var i:i64
        let s_7_6: i64 = fn_state.i;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: add s_7_5 s_7_7
        let s_7_8: i128 = (s_7_5 + s_7_7);
        // D s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // C s_7_10: const #16s : i64
        let s_7_10: i64 = 16;
        // D s_7_11: cast zx s_7_9 -> i
        let s_7_11: i128 = (i128::try_from(s_7_9).unwrap());
        // C s_7_12: cast zx s_7_10 -> i
        let s_7_12: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_13: read-var operand1:bv
        let s_7_13: Bits = fn_state.operand1;
        // D s_7_14: call Elem_read(s_7_13, s_7_11, s_7_12)
        let s_7_14: Bits = Elem_read(state, tracer, s_7_13, s_7_11, s_7_12);
        // D s_7_15: cast reint s_7_14 -> u16
        let s_7_15: u16 = (s_7_14.value() as u16);
        // D s_7_16: write-var element1 <= s_7_15
        fn_state.element1 = s_7_15;
        // C s_7_17: const #2s : i
        let s_7_17: i128 = 2;
        // D s_7_18: read-var e:i64
        let s_7_18: i64 = fn_state.e;
        // D s_7_19: cast zx s_7_18 -> i
        let s_7_19: i128 = (i128::try_from(s_7_18).unwrap());
        // D s_7_20: mul s_7_17 s_7_19
        let s_7_20: i128 = ((s_7_17) * (s_7_19));
        // D s_7_21: cast reint s_7_20 -> i64
        let s_7_21: i64 = (s_7_20 as i64);
        // D s_7_22: cast zx s_7_21 -> i
        let s_7_22: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_23: read-var i:i64
        let s_7_23: i64 = fn_state.i;
        // D s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // D s_7_25: add s_7_22 s_7_24
        let s_7_25: i128 = (s_7_22 + s_7_24);
        // D s_7_26: cast reint s_7_25 -> i64
        let s_7_26: i64 = (s_7_25 as i64);
        // C s_7_27: const #16s : i64
        let s_7_27: i64 = 16;
        // D s_7_28: cast zx s_7_26 -> i
        let s_7_28: i128 = (i128::try_from(s_7_26).unwrap());
        // C s_7_29: cast zx s_7_27 -> i
        let s_7_29: i128 = (i128::try_from(s_7_27).unwrap());
        // D s_7_30: read-var operand2:bv
        let s_7_30: Bits = fn_state.operand2;
        // D s_7_31: call Elem_read(s_7_30, s_7_28, s_7_29)
        let s_7_31: Bits = Elem_read(state, tracer, s_7_30, s_7_28, s_7_29);
        // D s_7_32: cast reint s_7_31 -> u16
        let s_7_32: u16 = (s_7_31.value() as u16);
        // D s_7_33: write-var element2 <= s_7_32
        fn_state.element2 = s_7_32;
        // C s_7_34: const #32s : i64
        let s_7_34: i64 = 32;
        // D s_7_35: read-var e:i64
        let s_7_35: i64 = fn_state.e;
        // D s_7_36: cast zx s_7_35 -> i
        let s_7_36: i128 = (i128::try_from(s_7_35).unwrap());
        // C s_7_37: cast zx s_7_34 -> i
        let s_7_37: i128 = (i128::try_from(s_7_34).unwrap());
        // D s_7_38: read-var operand3:bv
        let s_7_38: Bits = fn_state.operand3;
        // D s_7_39: call Elem_read(s_7_38, s_7_36, s_7_37)
        let s_7_39: Bits = Elem_read(state, tracer, s_7_38, s_7_36, s_7_37);
        // D s_7_40: cast reint s_7_39 -> u32
        let s_7_40: u32 = (s_7_39.value() as u32);
        // D s_7_41: write-var element3 <= s_7_40
        fn_state.element3 = s_7_40;
        // D s_7_42: read-var sub_op:u8
        let s_7_42: bool = fn_state.sub_op;
        // N s_7_43: branch s_7_42 b11 b8
        if s_7_42 {
            return block_11(state, tracer, fn_state);
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
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call FPCR_read(s_9_0)
        let s_9_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_9_0);
        // D s_9_2: read-var element3:u32
        let s_9_2: u32 = fn_state.element3;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 32u16);
        // D s_9_4: read-var element1:u16
        let s_9_4: u16 = fn_state.element1;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 16u16);
        // D s_9_6: read-var element2:u16
        let s_9_6: u16 = fn_state.element2;
        // D s_9_7: cast zx s_9_6 -> bv
        let s_9_7: Bits = Bits::new(s_9_6 as u128, 16u16);
        // D s_9_8: call FPMulAddH_ZA(s_9_3, s_9_5, s_9_7, s_9_1)
        let s_9_8: Bits = FPMulAddH_ZA(state, tracer, s_9_3, s_9_5, s_9_7, s_9_1);
        // D s_9_9: write-var gs#839522 <= s_9_8
        fn_state.gs_839522 = s_9_8;
        // N s_9_10: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#839522:bv
        let s_10_0: Bits = fn_state.gs_839522;
        // D s_10_1: cast reint s_10_0 -> u32
        let s_10_1: u32 = (s_10_0.value() as u32);
        // D s_10_2: read-var e:i64
        let s_10_2: i64 = fn_state.e;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // C s_10_4: const #32s : i64
        let s_10_4: i64 = 32;
        // C s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: cast zx s_10_1 -> bv
        let s_10_6: Bits = Bits::new(s_10_1 as u128, 32u16);
        // D s_10_7: read-var result:bv
        let s_10_7: Bits = fn_state.result;
        // D s_10_8: call Elem_set(s_10_7, s_10_3, s_10_5, s_10_6)
        let s_10_8: Bits = Elem_set(state, tracer, s_10_7, s_10_3, s_10_5, s_10_6);
        // D s_10_9: write-var result <= s_10_8
        fn_state.result = s_10_8;
        // D s_10_10: read-var e:i64
        let s_10_10: i64 = fn_state.e;
        // C s_10_11: const #1s : i64
        let s_10_11: i64 = 1;
        // D s_10_12: add s_10_10 s_10_11
        let s_10_12: i64 = (s_10_10 + s_10_11);
        // D s_10_13: write-var e <= s_10_12
        fn_state.e = s_10_12;
        // N s_10_14: jump b6
        return block_6(state, tracer, fn_state);
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
        // D s_11_2: call FPNeg(s_11_1)
        let s_11_2: Bits = FPNeg(state, tracer, s_11_1);
        // D s_11_3: write-var gs#839517 <= s_11_2
        fn_state.gs_839517 = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#839517:bv
        let s_12_0: Bits = fn_state.gs_839517;
        // D s_12_1: cast reint s_12_0 -> u16
        let s_12_1: u16 = (s_12_0.value() as u16);
        // D s_12_2: write-var element1 <= s_12_1
        fn_state.element1 = s_12_1;
        // N s_12_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var i:i64
        let s_13_0: i64 = fn_state.i;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var vecshadow#5943:i
        let s_13_2: i128 = fn_state.vecshadow_5943;
        // D s_13_3: add s_13_2 s_13_1
        let s_13_3: i128 = (s_13_2 + s_13_1);
        // D s_13_4: read-var VLshadow#5942:i64
        let s_13_4: i64 = fn_state.VLshadow_5942;
        // D s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // D s_13_6: cast reint s_13_5 -> i64
        let s_13_6: i64 = (s_13_5 as i64);
        // D s_13_7: cast zx s_13_6 -> i
        let s_13_7: i128 = (i128::try_from(s_13_6).unwrap());
        // D s_13_8: read-var result:bv
        let s_13_8: Bits = fn_state.result;
        // D s_13_9: call ZAvector_set(s_13_3, s_13_7, s_13_8)
        let s_13_9: () = ZAvector_set(state, tracer, s_13_3, s_13_7, s_13_8);
        // D s_13_10: read-var i:i64
        let s_13_10: i64 = fn_state.i;
        // C s_13_11: const #1s : i64
        let s_13_11: i64 = 1;
        // D s_13_12: add s_13_10 s_13_11
        let s_13_12: i64 = (s_13_10 + s_13_11);
        // D s_13_13: write-var i <= s_13_12
        fn_state.i = s_13_12;
        // N s_13_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var vstride:i64
        let s_14_0: i64 = fn_state.vstride;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var vec:i
        let s_14_2: i128 = fn_state.vec;
        // D s_14_3: add s_14_2 s_14_1
        let s_14_3: i128 = (s_14_2 + s_14_1);
        // D s_14_4: write-var vec <= s_14_3
        fn_state.vec = s_14_3;
        // D s_14_5: read-var r:i64
        let s_14_5: i64 = fn_state.r;
        // C s_14_6: const #1s : i64
        let s_14_6: i64 = 1;
        // D s_14_7: add s_14_5 s_14_6
        let s_14_7: i64 = (s_14_5 + s_14_6);
        // D s_14_8: write-var r <= s_14_7
        fn_state.r = s_14_7;
        // N s_14_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
}
