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
use FPDotAdd_ZA::*;
use Elem_set::*;
use CheckStreamingSVEAndZAEnabled::*;
use FPCR_read::*;
use X_read::*;
use Elem_read::*;
use ZAvector_set::*;
use Z_read::*;
use ZAvector_read::*;
use common::*;
pub fn execute_FDOT_ZA_ZZi_2xi<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    index: i64,
    m: i64,
    n: i64,
    nreg: i64,
    offset: i64,
    v: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        operand3: Bits,
        vstride: i64,
        vec: i128,
        gs_266521: i64,
        VLshadow_5865: i64,
        gs_836980: Bits,
        elements: i64,
        gs_266529: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_5864: i64,
        operand2: Bits,
        VL: i64,
        index: i64,
        m: i64,
        n: i64,
        nreg: i64,
        offset: i64,
        v: i64,
    }
    let fn_state = FunctionState {
        VL,
        index,
        m,
        n,
        nreg,
        offset,
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
        // D s_0_3: write-var VLshadow#5864 <= s_0_2
        fn_state.VLshadow_5864 = s_0_2;
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
        // D s_1_0: read-var VLshadow#5864:i64
        let s_1_0: i64 = fn_state.VLshadow_5864;
        // D s_1_1: write-var VLshadow#5865 <= s_1_0
        fn_state.VLshadow_5865 = s_1_0;
        // C s_1_2: const #32s : i
        let s_1_2: i128 = 32;
        // D s_1_3: read-var VLshadow#5865:i64
        let s_1_3: i64 = fn_state.VLshadow_5865;
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
        // D s_1_9: read-var VLshadow#5865:i64
        let s_1_9: i64 = fn_state.VLshadow_5865;
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
        // D s_1_43: write-var gs#266521 <= s_1_42
        fn_state.gs_266521 = s_1_42;
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
        // D s_2_1: read-var gs#266521:i64
        let s_2_1: i64 = fn_state.gs_266521;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b8 b3
        if s_2_2 {
            return block_8(state, tracer, fn_state);
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
        // D s_3_6: read-var VLshadow#5865:i64
        let s_3_6: i64 = fn_state.VLshadow_5865;
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
        // D s_3_13: read-var VLshadow#5865:i64
        let s_3_13: i64 = fn_state.VLshadow_5865;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: read-var m:i64
        let s_3_16: i64 = fn_state.m;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: cast zx s_3_15 -> i
        let s_3_18: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_19: call Z_read(s_3_17, s_3_18)
        let s_3_19: Bits = Z_read(state, tracer, s_3_17, s_3_18);
        // D s_3_20: write-var operand2 <= s_3_19
        fn_state.operand2 = s_3_19;
        // D s_3_21: read-var vec:i
        let s_3_21: i128 = fn_state.vec;
        // D s_3_22: read-var VLshadow#5865:i64
        let s_3_22: i64 = fn_state.VLshadow_5865;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: cast zx s_3_24 -> i
        let s_3_25: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_26: call ZAvector_read(s_3_21, s_3_25)
        let s_3_26: Bits = ZAvector_read(state, tracer, s_3_21, s_3_25);
        // D s_3_27: write-var operand3 <= s_3_26
        fn_state.operand3 = s_3_26;
        // C s_3_28: const #0s : i64
        let s_3_28: i64 = 0;
        // C s_3_29: const #1s : i
        let s_3_29: i128 = 1;
        // D s_3_30: read-var elements:i64
        let s_3_30: i64 = fn_state.elements;
        // D s_3_31: cast zx s_3_30 -> i
        let s_3_31: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_32: sub s_3_31 s_3_29
        let s_3_32: i128 = ((s_3_31) - (s_3_29));
        // D s_3_33: cast reint s_3_32 -> i64
        let s_3_33: i64 = (s_3_32 as i64);
        // D s_3_34: write-var gs#266529 <= s_3_33
        fn_state.gs_266529 = s_3_33;
        // D s_3_35: write-var e <= s_3_28
        fn_state.e = s_3_28;
        // N s_3_36: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#266529:i64
        let s_4_1: i64 = fn_state.gs_266529;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b7 b5
        if s_4_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var e:i64
        let s_5_1: i64 = fn_state.e;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // C s_5_5: const #0s : i
        let s_5_5: i128 = 0;
        // D s_5_6: cast zx s_5_4 -> i
        let s_5_6: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // C s_5_9: const #16s : i64
        let s_5_9: i64 = 16;
        // D s_5_10: cast zx s_5_8 -> i
        let s_5_10: i128 = (i128::try_from(s_5_8).unwrap());
        // C s_5_11: cast zx s_5_9 -> i
        let s_5_11: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_12: read-var operand1:bv
        let s_5_12: Bits = fn_state.operand1;
        // D s_5_13: call Elem_read(s_5_12, s_5_10, s_5_11)
        let s_5_13: Bits = Elem_read(state, tracer, s_5_12, s_5_10, s_5_11);
        // D s_5_14: cast reint s_5_13 -> u16
        let s_5_14: u16 = (s_5_13.value() as u16);
        // C s_5_15: const #2s : i
        let s_5_15: i128 = 2;
        // D s_5_16: read-var e:i64
        let s_5_16: i64 = fn_state.e;
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_18: mul s_5_15 s_5_17
        let s_5_18: i128 = ((s_5_15) * (s_5_17));
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // C s_5_20: const #1s : i
        let s_5_20: i128 = 1;
        // D s_5_21: cast zx s_5_19 -> i
        let s_5_21: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_22: add s_5_21 s_5_20
        let s_5_22: i128 = (s_5_21 + s_5_20);
        // D s_5_23: cast reint s_5_22 -> i64
        let s_5_23: i64 = (s_5_22 as i64);
        // C s_5_24: const #16s : i64
        let s_5_24: i64 = 16;
        // D s_5_25: cast zx s_5_23 -> i
        let s_5_25: i128 = (i128::try_from(s_5_23).unwrap());
        // C s_5_26: cast zx s_5_24 -> i
        let s_5_26: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_27: read-var operand1:bv
        let s_5_27: Bits = fn_state.operand1;
        // D s_5_28: call Elem_read(s_5_27, s_5_25, s_5_26)
        let s_5_28: Bits = Elem_read(state, tracer, s_5_27, s_5_25, s_5_26);
        // D s_5_29: cast reint s_5_28 -> u16
        let s_5_29: u16 = (s_5_28.value() as u16);
        // C s_5_30: const #4s : i
        let s_5_30: i128 = 4;
        // D s_5_31: read-var e:i64
        let s_5_31: i64 = fn_state.e;
        // D s_5_32: cast zx s_5_31 -> i
        let s_5_32: i128 = (i128::try_from(s_5_31).unwrap());
        // D s_5_33: mod s_5_32 s_5_30
        let s_5_33: i128 = ((s_5_32) % (s_5_30));
        // D s_5_34: cast reint s_5_33 -> i64
        let s_5_34: i64 = (s_5_33 as i64);
        // D s_5_35: read-var e:i64
        let s_5_35: i64 = fn_state.e;
        // D s_5_36: cast zx s_5_35 -> i
        let s_5_36: i128 = (i128::try_from(s_5_35).unwrap());
        // D s_5_37: cast zx s_5_34 -> i
        let s_5_37: i128 = (i128::try_from(s_5_34).unwrap());
        // D s_5_38: sub s_5_36 s_5_37
        let s_5_38: i128 = ((s_5_36) - (s_5_37));
        // D s_5_39: cast reint s_5_38 -> i64
        let s_5_39: i64 = (s_5_38 as i64);
        // D s_5_40: cast zx s_5_39 -> i
        let s_5_40: i128 = (i128::try_from(s_5_39).unwrap());
        // D s_5_41: read-var index:i64
        let s_5_41: i64 = fn_state.index;
        // D s_5_42: cast zx s_5_41 -> i
        let s_5_42: i128 = (i128::try_from(s_5_41).unwrap());
        // D s_5_43: add s_5_40 s_5_42
        let s_5_43: i128 = (s_5_40 + s_5_42);
        // D s_5_44: cast reint s_5_43 -> i64
        let s_5_44: i64 = (s_5_43 as i64);
        // C s_5_45: const #2s : i
        let s_5_45: i128 = 2;
        // D s_5_46: cast zx s_5_44 -> i
        let s_5_46: i128 = (i128::try_from(s_5_44).unwrap());
        // D s_5_47: mul s_5_45 s_5_46
        let s_5_47: i128 = ((s_5_45) * (s_5_46));
        // D s_5_48: cast reint s_5_47 -> i64
        let s_5_48: i64 = (s_5_47 as i64);
        // C s_5_49: const #0s : i
        let s_5_49: i128 = 0;
        // D s_5_50: cast zx s_5_48 -> i
        let s_5_50: i128 = (i128::try_from(s_5_48).unwrap());
        // D s_5_51: add s_5_50 s_5_49
        let s_5_51: i128 = (s_5_50 + s_5_49);
        // D s_5_52: cast reint s_5_51 -> i64
        let s_5_52: i64 = (s_5_51 as i64);
        // C s_5_53: const #16s : i64
        let s_5_53: i64 = 16;
        // D s_5_54: cast zx s_5_52 -> i
        let s_5_54: i128 = (i128::try_from(s_5_52).unwrap());
        // C s_5_55: cast zx s_5_53 -> i
        let s_5_55: i128 = (i128::try_from(s_5_53).unwrap());
        // D s_5_56: read-var operand2:bv
        let s_5_56: Bits = fn_state.operand2;
        // D s_5_57: call Elem_read(s_5_56, s_5_54, s_5_55)
        let s_5_57: Bits = Elem_read(state, tracer, s_5_56, s_5_54, s_5_55);
        // D s_5_58: cast reint s_5_57 -> u16
        let s_5_58: u16 = (s_5_57.value() as u16);
        // C s_5_59: const #2s : i
        let s_5_59: i128 = 2;
        // D s_5_60: cast zx s_5_44 -> i
        let s_5_60: i128 = (i128::try_from(s_5_44).unwrap());
        // D s_5_61: mul s_5_59 s_5_60
        let s_5_61: i128 = ((s_5_59) * (s_5_60));
        // D s_5_62: cast reint s_5_61 -> i64
        let s_5_62: i64 = (s_5_61 as i64);
        // C s_5_63: const #1s : i
        let s_5_63: i128 = 1;
        // D s_5_64: cast zx s_5_62 -> i
        let s_5_64: i128 = (i128::try_from(s_5_62).unwrap());
        // D s_5_65: add s_5_64 s_5_63
        let s_5_65: i128 = (s_5_64 + s_5_63);
        // D s_5_66: cast reint s_5_65 -> i64
        let s_5_66: i64 = (s_5_65 as i64);
        // C s_5_67: const #16s : i64
        let s_5_67: i64 = 16;
        // D s_5_68: cast zx s_5_66 -> i
        let s_5_68: i128 = (i128::try_from(s_5_66).unwrap());
        // C s_5_69: cast zx s_5_67 -> i
        let s_5_69: i128 = (i128::try_from(s_5_67).unwrap());
        // D s_5_70: read-var operand2:bv
        let s_5_70: Bits = fn_state.operand2;
        // D s_5_71: call Elem_read(s_5_70, s_5_68, s_5_69)
        let s_5_71: Bits = Elem_read(state, tracer, s_5_70, s_5_68, s_5_69);
        // D s_5_72: cast reint s_5_71 -> u16
        let s_5_72: u16 = (s_5_71.value() as u16);
        // C s_5_73: const #32s : i64
        let s_5_73: i64 = 32;
        // D s_5_74: read-var e:i64
        let s_5_74: i64 = fn_state.e;
        // D s_5_75: cast zx s_5_74 -> i
        let s_5_75: i128 = (i128::try_from(s_5_74).unwrap());
        // C s_5_76: cast zx s_5_73 -> i
        let s_5_76: i128 = (i128::try_from(s_5_73).unwrap());
        // D s_5_77: read-var operand3:bv
        let s_5_77: Bits = fn_state.operand3;
        // D s_5_78: call Elem_read(s_5_77, s_5_75, s_5_76)
        let s_5_78: Bits = Elem_read(state, tracer, s_5_77, s_5_75, s_5_76);
        // D s_5_79: cast reint s_5_78 -> u32
        let s_5_79: u32 = (s_5_78.value() as u32);
        // C s_5_80: const #() : ()
        let s_5_80: () = ();
        // S s_5_81: call FPCR_read(s_5_80)
        let s_5_81: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_5_80);
        // D s_5_82: cast zx s_5_79 -> bv
        let s_5_82: Bits = Bits::new(s_5_79 as u128, 32u16);
        // D s_5_83: cast zx s_5_14 -> bv
        let s_5_83: Bits = Bits::new(s_5_14 as u128, 16u16);
        // D s_5_84: cast zx s_5_29 -> bv
        let s_5_84: Bits = Bits::new(s_5_29 as u128, 16u16);
        // D s_5_85: cast zx s_5_58 -> bv
        let s_5_85: Bits = Bits::new(s_5_58 as u128, 16u16);
        // D s_5_86: cast zx s_5_72 -> bv
        let s_5_86: Bits = Bits::new(s_5_72 as u128, 16u16);
        // D s_5_87: call FPDotAdd_ZA(s_5_82, s_5_83, s_5_84, s_5_85, s_5_86, s_5_81)
        let s_5_87: Bits = FPDotAdd_ZA(
            state,
            tracer,
            s_5_82,
            s_5_83,
            s_5_84,
            s_5_85,
            s_5_86,
            s_5_81,
        );
        // D s_5_88: write-var gs#836980 <= s_5_87
        fn_state.gs_836980 = s_5_87;
        // N s_5_89: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#836980:bv
        let s_6_0: Bits = fn_state.gs_836980;
        // D s_6_1: cast reint s_6_0 -> u32
        let s_6_1: u32 = (s_6_0.value() as u32);
        // C s_6_2: const #32s : i64
        let s_6_2: i64 = 32;
        // D s_6_3: read-var e:i64
        let s_6_3: i64 = fn_state.e;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // C s_6_5: cast zx s_6_2 -> i
        let s_6_5: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_6: cast zx s_6_1 -> bv
        let s_6_6: Bits = Bits::new(s_6_1 as u128, 32u16);
        // D s_6_7: read-var result:bv
        let s_6_7: Bits = fn_state.result;
        // D s_6_8: call Elem_set(s_6_7, s_6_4, s_6_5, s_6_6)
        let s_6_8: Bits = Elem_set(state, tracer, s_6_7, s_6_4, s_6_5, s_6_6);
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
        // N s_6_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var vec:i
        let s_7_0: i128 = fn_state.vec;
        // D s_7_1: read-var VLshadow#5865:i64
        let s_7_1: i64 = fn_state.VLshadow_5865;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var result:bv
        let s_7_5: Bits = fn_state.result;
        // D s_7_6: call ZAvector_set(s_7_0, s_7_4, s_7_5)
        let s_7_6: () = ZAvector_set(state, tracer, s_7_0, s_7_4, s_7_5);
        // D s_7_7: read-var vstride:i64
        let s_7_7: i64 = fn_state.vstride;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: read-var vec:i
        let s_7_9: i128 = fn_state.vec;
        // D s_7_10: add s_7_9 s_7_8
        let s_7_10: i128 = (s_7_9 + s_7_8);
        // D s_7_11: write-var vec <= s_7_10
        fn_state.vec = s_7_10;
        // D s_7_12: read-var r:i64
        let s_7_12: i64 = fn_state.r;
        // C s_7_13: const #1s : i64
        let s_7_13: i64 = 1;
        // D s_7_14: add s_7_12 s_7_13
        let s_7_14: i64 = (s_7_12 + s_7_13);
        // D s_7_15: write-var r <= s_7_14
        fn_state.r = s_7_14;
        // N s_7_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
}
