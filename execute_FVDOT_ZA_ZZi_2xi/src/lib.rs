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
pub fn execute_FVDOT_ZA_ZZi_2xi<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    index: i64,
    m: i64,
    n: i64,
    offset: i64,
    v: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        operand1b: Bits,
        e: i64,
        gs_837359: Bits,
        operand3: Bits,
        VLshadow_5879: i64,
        vstride: i64,
        vec: i128,
        operand1a: Bits,
        VLshadow_5880: i64,
        elements: i64,
        gs_266780: i64,
        result: Bits,
        operand2: Bits,
        VL: i64,
        index: i64,
        m: i64,
        n: i64,
        offset: i64,
        v: i64,
    }
    let fn_state = FunctionState {
        VL,
        index,
        m,
        n,
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
        // D s_0_3: write-var VLshadow#5879 <= s_0_2
        fn_state.VLshadow_5879 = s_0_2;
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
        // D s_1_0: read-var VLshadow#5879:i64
        let s_1_0: i64 = fn_state.VLshadow_5879;
        // D s_1_1: write-var VLshadow#5880 <= s_1_0
        fn_state.VLshadow_5880 = s_1_0;
        // C s_1_2: const #32s : i
        let s_1_2: i128 = 32;
        // D s_1_3: read-var VLshadow#5880:i64
        let s_1_3: i64 = fn_state.VLshadow_5880;
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
        // D s_1_9: read-var VLshadow#5880:i64
        let s_1_9: i64 = fn_state.VLshadow_5880;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_10 s_1_8
        let s_1_11: i128 = ((s_1_10) / (s_1_8));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // C s_1_13: const #2s : i
        let s_1_13: i128 = 2;
        // D s_1_14: cast zx s_1_12 -> i
        let s_1_14: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_15: div s_1_14 s_1_13
        let s_1_15: i128 = ((s_1_14) / (s_1_13));
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: write-var vstride <= s_1_16
        fn_state.vstride = s_1_16;
        // C s_1_18: const #32s : i64
        let s_1_18: i64 = 32;
        // D s_1_19: read-var v:i64
        let s_1_19: i64 = fn_state.v;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: call X_read(s_1_20, s_1_18)
        let s_1_21: Bits = X_read(state, tracer, s_1_20, s_1_18);
        // D s_1_22: cast reint s_1_21 -> u32
        let s_1_22: u32 = (s_1_21.value() as u32);
        // D s_1_23: cast zx s_1_22 -> bv
        let s_1_23: Bits = Bits::new(s_1_22 as u128, 32u16);
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (s_1_23.value() as i128);
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: read-var offset:i64
        let s_1_27: i64 = fn_state.offset;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: add s_1_26 s_1_28
        let s_1_29: i128 = (s_1_26 + s_1_28);
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: read-var vstride:i64
        let s_1_32: i64 = fn_state.vstride;
        // D s_1_33: cast zx s_1_32 -> i
        let s_1_33: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_34: mod s_1_31 s_1_33
        let s_1_34: i128 = ((s_1_31) % (s_1_33));
        // D s_1_35: write-var vec <= s_1_34
        fn_state.vec = s_1_34;
        // C s_1_36: const #0s : i64
        let s_1_36: i64 = 0;
        // D s_1_37: write-var r <= s_1_36
        fn_state.r = s_1_36;
        // N s_1_38: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // C s_2_1: const #1s : i64
        let s_2_1: i64 = 1;
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
        // D s_3_0: read-var VLshadow#5880:i64
        let s_3_0: i64 = fn_state.VLshadow_5880;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var n:i64
        let s_3_3: i64 = fn_state.n;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: call Z_read(s_3_4, s_3_5)
        let s_3_6: Bits = Z_read(state, tracer, s_3_4, s_3_5);
        // D s_3_7: write-var operand1a <= s_3_6
        fn_state.operand1a = s_3_6;
        // C s_3_8: const #1s : i
        let s_3_8: i128 = 1;
        // D s_3_9: read-var n:i64
        let s_3_9: i64 = fn_state.n;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: add s_3_10 s_3_8
        let s_3_11: i128 = (s_3_10 + s_3_8);
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: read-var VLshadow#5880:i64
        let s_3_13: i64 = fn_state.VLshadow_5880;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: cast zx s_3_12 -> i
        let s_3_16: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_17: cast zx s_3_15 -> i
        let s_3_17: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_18: call Z_read(s_3_16, s_3_17)
        let s_3_18: Bits = Z_read(state, tracer, s_3_16, s_3_17);
        // D s_3_19: write-var operand1b <= s_3_18
        fn_state.operand1b = s_3_18;
        // D s_3_20: read-var VLshadow#5880:i64
        let s_3_20: i64 = fn_state.VLshadow_5880;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: cast reint s_3_21 -> i64
        let s_3_22: i64 = (s_3_21 as i64);
        // D s_3_23: read-var m:i64
        let s_3_23: i64 = fn_state.m;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: cast zx s_3_22 -> i
        let s_3_25: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_26: call Z_read(s_3_24, s_3_25)
        let s_3_26: Bits = Z_read(state, tracer, s_3_24, s_3_25);
        // D s_3_27: write-var operand2 <= s_3_26
        fn_state.operand2 = s_3_26;
        // D s_3_28: read-var vec:i
        let s_3_28: i128 = fn_state.vec;
        // D s_3_29: read-var VLshadow#5880:i64
        let s_3_29: i64 = fn_state.VLshadow_5880;
        // D s_3_30: cast zx s_3_29 -> i
        let s_3_30: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_31: cast reint s_3_30 -> i64
        let s_3_31: i64 = (s_3_30 as i64);
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: call ZAvector_read(s_3_28, s_3_32)
        let s_3_33: Bits = ZAvector_read(state, tracer, s_3_28, s_3_32);
        // D s_3_34: write-var operand3 <= s_3_33
        fn_state.operand3 = s_3_33;
        // C s_3_35: const #0s : i64
        let s_3_35: i64 = 0;
        // C s_3_36: const #1s : i
        let s_3_36: i128 = 1;
        // D s_3_37: read-var elements:i64
        let s_3_37: i64 = fn_state.elements;
        // D s_3_38: cast zx s_3_37 -> i
        let s_3_38: i128 = (i128::try_from(s_3_37).unwrap());
        // D s_3_39: sub s_3_38 s_3_36
        let s_3_39: i128 = ((s_3_38) - (s_3_36));
        // D s_3_40: cast reint s_3_39 -> i64
        let s_3_40: i64 = (s_3_39 as i64);
        // D s_3_41: write-var gs#266780 <= s_3_40
        fn_state.gs_266780 = s_3_40;
        // D s_3_42: write-var e <= s_3_35
        fn_state.e = s_3_35;
        // N s_3_43: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#266780:i64
        let s_4_1: i64 = fn_state.gs_266780;
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
        // C s_5_0: const #4s : i
        let s_5_0: i128 = 4;
        // D s_5_1: read-var e:i64
        let s_5_1: i64 = fn_state.e;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mod s_5_2 s_5_0
        let s_5_3: i128 = ((s_5_2) % (s_5_0));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: read-var e:i64
        let s_5_5: i64 = fn_state.e;
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: cast zx s_5_4 -> i
        let s_5_7: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_8: sub s_5_6 s_5_7
        let s_5_8: i128 = ((s_5_6) - (s_5_7));
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: read-var index:i64
        let s_5_11: i64 = fn_state.index;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: add s_5_10 s_5_12
        let s_5_13: i128 = (s_5_10 + s_5_12);
        // D s_5_14: cast reint s_5_13 -> i64
        let s_5_14: i64 = (s_5_13 as i64);
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
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: read-var r:i64
        let s_5_21: i64 = fn_state.r;
        // D s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_23: add s_5_20 s_5_22
        let s_5_23: i128 = (s_5_20 + s_5_22);
        // D s_5_24: cast reint s_5_23 -> i64
        let s_5_24: i64 = (s_5_23 as i64);
        // C s_5_25: const #16s : i64
        let s_5_25: i64 = 16;
        // D s_5_26: cast zx s_5_24 -> i
        let s_5_26: i128 = (i128::try_from(s_5_24).unwrap());
        // C s_5_27: cast zx s_5_25 -> i
        let s_5_27: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_28: read-var operand1a:bv
        let s_5_28: Bits = fn_state.operand1a;
        // D s_5_29: call Elem_read(s_5_28, s_5_26, s_5_27)
        let s_5_29: Bits = Elem_read(state, tracer, s_5_28, s_5_26, s_5_27);
        // D s_5_30: cast reint s_5_29 -> u16
        let s_5_30: u16 = (s_5_29.value() as u16);
        // C s_5_31: const #2s : i
        let s_5_31: i128 = 2;
        // D s_5_32: read-var e:i64
        let s_5_32: i64 = fn_state.e;
        // D s_5_33: cast zx s_5_32 -> i
        let s_5_33: i128 = (i128::try_from(s_5_32).unwrap());
        // D s_5_34: mul s_5_31 s_5_33
        let s_5_34: i128 = ((s_5_31) * (s_5_33));
        // D s_5_35: cast reint s_5_34 -> i64
        let s_5_35: i64 = (s_5_34 as i64);
        // D s_5_36: cast zx s_5_35 -> i
        let s_5_36: i128 = (i128::try_from(s_5_35).unwrap());
        // D s_5_37: read-var r:i64
        let s_5_37: i64 = fn_state.r;
        // D s_5_38: cast zx s_5_37 -> i
        let s_5_38: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_39: add s_5_36 s_5_38
        let s_5_39: i128 = (s_5_36 + s_5_38);
        // D s_5_40: cast reint s_5_39 -> i64
        let s_5_40: i64 = (s_5_39 as i64);
        // C s_5_41: const #16s : i64
        let s_5_41: i64 = 16;
        // D s_5_42: cast zx s_5_40 -> i
        let s_5_42: i128 = (i128::try_from(s_5_40).unwrap());
        // C s_5_43: cast zx s_5_41 -> i
        let s_5_43: i128 = (i128::try_from(s_5_41).unwrap());
        // D s_5_44: read-var operand1b:bv
        let s_5_44: Bits = fn_state.operand1b;
        // D s_5_45: call Elem_read(s_5_44, s_5_42, s_5_43)
        let s_5_45: Bits = Elem_read(state, tracer, s_5_44, s_5_42, s_5_43);
        // D s_5_46: cast reint s_5_45 -> u16
        let s_5_46: u16 = (s_5_45.value() as u16);
        // C s_5_47: const #2s : i
        let s_5_47: i128 = 2;
        // D s_5_48: cast zx s_5_14 -> i
        let s_5_48: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_49: mul s_5_47 s_5_48
        let s_5_49: i128 = ((s_5_47) * (s_5_48));
        // D s_5_50: cast reint s_5_49 -> i64
        let s_5_50: i64 = (s_5_49 as i64);
        // C s_5_51: const #0s : i
        let s_5_51: i128 = 0;
        // D s_5_52: cast zx s_5_50 -> i
        let s_5_52: i128 = (i128::try_from(s_5_50).unwrap());
        // D s_5_53: add s_5_52 s_5_51
        let s_5_53: i128 = (s_5_52 + s_5_51);
        // D s_5_54: cast reint s_5_53 -> i64
        let s_5_54: i64 = (s_5_53 as i64);
        // C s_5_55: const #16s : i64
        let s_5_55: i64 = 16;
        // D s_5_56: cast zx s_5_54 -> i
        let s_5_56: i128 = (i128::try_from(s_5_54).unwrap());
        // C s_5_57: cast zx s_5_55 -> i
        let s_5_57: i128 = (i128::try_from(s_5_55).unwrap());
        // D s_5_58: read-var operand2:bv
        let s_5_58: Bits = fn_state.operand2;
        // D s_5_59: call Elem_read(s_5_58, s_5_56, s_5_57)
        let s_5_59: Bits = Elem_read(state, tracer, s_5_58, s_5_56, s_5_57);
        // D s_5_60: cast reint s_5_59 -> u16
        let s_5_60: u16 = (s_5_59.value() as u16);
        // C s_5_61: const #2s : i
        let s_5_61: i128 = 2;
        // D s_5_62: cast zx s_5_14 -> i
        let s_5_62: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_63: mul s_5_61 s_5_62
        let s_5_63: i128 = ((s_5_61) * (s_5_62));
        // D s_5_64: cast reint s_5_63 -> i64
        let s_5_64: i64 = (s_5_63 as i64);
        // C s_5_65: const #1s : i
        let s_5_65: i128 = 1;
        // D s_5_66: cast zx s_5_64 -> i
        let s_5_66: i128 = (i128::try_from(s_5_64).unwrap());
        // D s_5_67: add s_5_66 s_5_65
        let s_5_67: i128 = (s_5_66 + s_5_65);
        // D s_5_68: cast reint s_5_67 -> i64
        let s_5_68: i64 = (s_5_67 as i64);
        // C s_5_69: const #16s : i64
        let s_5_69: i64 = 16;
        // D s_5_70: cast zx s_5_68 -> i
        let s_5_70: i128 = (i128::try_from(s_5_68).unwrap());
        // C s_5_71: cast zx s_5_69 -> i
        let s_5_71: i128 = (i128::try_from(s_5_69).unwrap());
        // D s_5_72: read-var operand2:bv
        let s_5_72: Bits = fn_state.operand2;
        // D s_5_73: call Elem_read(s_5_72, s_5_70, s_5_71)
        let s_5_73: Bits = Elem_read(state, tracer, s_5_72, s_5_70, s_5_71);
        // D s_5_74: cast reint s_5_73 -> u16
        let s_5_74: u16 = (s_5_73.value() as u16);
        // C s_5_75: const #32s : i64
        let s_5_75: i64 = 32;
        // D s_5_76: read-var e:i64
        let s_5_76: i64 = fn_state.e;
        // D s_5_77: cast zx s_5_76 -> i
        let s_5_77: i128 = (i128::try_from(s_5_76).unwrap());
        // C s_5_78: cast zx s_5_75 -> i
        let s_5_78: i128 = (i128::try_from(s_5_75).unwrap());
        // D s_5_79: read-var operand3:bv
        let s_5_79: Bits = fn_state.operand3;
        // D s_5_80: call Elem_read(s_5_79, s_5_77, s_5_78)
        let s_5_80: Bits = Elem_read(state, tracer, s_5_79, s_5_77, s_5_78);
        // D s_5_81: cast reint s_5_80 -> u32
        let s_5_81: u32 = (s_5_80.value() as u32);
        // C s_5_82: const #() : ()
        let s_5_82: () = ();
        // S s_5_83: call FPCR_read(s_5_82)
        let s_5_83: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_5_82);
        // D s_5_84: cast zx s_5_81 -> bv
        let s_5_84: Bits = Bits::new(s_5_81 as u128, 32u16);
        // D s_5_85: cast zx s_5_30 -> bv
        let s_5_85: Bits = Bits::new(s_5_30 as u128, 16u16);
        // D s_5_86: cast zx s_5_46 -> bv
        let s_5_86: Bits = Bits::new(s_5_46 as u128, 16u16);
        // D s_5_87: cast zx s_5_60 -> bv
        let s_5_87: Bits = Bits::new(s_5_60 as u128, 16u16);
        // D s_5_88: cast zx s_5_74 -> bv
        let s_5_88: Bits = Bits::new(s_5_74 as u128, 16u16);
        // D s_5_89: call FPDotAdd_ZA(s_5_84, s_5_85, s_5_86, s_5_87, s_5_88, s_5_83)
        let s_5_89: Bits = FPDotAdd_ZA(
            state,
            tracer,
            s_5_84,
            s_5_85,
            s_5_86,
            s_5_87,
            s_5_88,
            s_5_83,
        );
        // D s_5_90: write-var gs#837359 <= s_5_89
        fn_state.gs_837359 = s_5_89;
        // N s_5_91: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#837359:bv
        let s_6_0: Bits = fn_state.gs_837359;
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
        // D s_7_1: read-var VLshadow#5880:i64
        let s_7_1: i64 = fn_state.VLshadow_5880;
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
