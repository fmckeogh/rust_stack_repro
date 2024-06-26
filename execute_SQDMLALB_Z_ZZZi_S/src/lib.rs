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
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use SignedSat::*;
use Z_set::*;
use common::*;
pub fn execute_SQDMLALB_Z_ZZZi_S<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    index: i64,
    m: i64,
    n: i64,
    sel: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        VLshadow_3960: i64,
        gs_213338: i64,
        result: Bits,
        VLshadow_3961: i64,
        operand1: Bits,
        eltspersegment: i64,
        operand2: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        index: i64,
        m: i64,
        n: i64,
        sel: i64,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        index,
        m,
        n,
        sel,
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
        // D s_0_3: write-var VLshadow#3960 <= s_0_2
        fn_state.VLshadow_3960 = s_0_2;
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
        // D s_1_0: read-var VLshadow#3960:i64
        let s_1_0: i64 = fn_state.VLshadow_3960;
        // D s_1_1: write-var VLshadow#3961 <= s_1_0
        fn_state.VLshadow_3961 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esize:i64
        let s_1_3: i64 = fn_state.esize;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_2 s_1_4
        let s_1_5: i128 = ((s_1_2) * (s_1_4));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#3961:i64
        let s_1_7: i64 = fn_state.VLshadow_3961;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // C s_1_12: const #2s : i
        let s_1_12: i128 = 2;
        // D s_1_13: read-var esize:i64
        let s_1_13: i64 = fn_state.esize;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: mul s_1_12 s_1_14
        let s_1_15: i128 = ((s_1_12) * (s_1_14));
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // C s_1_17: const #128s : i
        let s_1_17: i128 = 128;
        // D s_1_18: cast zx s_1_16 -> i
        let s_1_18: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_19: div s_1_17 s_1_18
        let s_1_19: i128 = ((s_1_17) / (s_1_18));
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var eltspersegment <= s_1_20
        fn_state.eltspersegment = s_1_20;
        // D s_1_22: read-var VLshadow#3961:i64
        let s_1_22: i64 = fn_state.VLshadow_3961;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: read-var n:i64
        let s_1_25: i64 = fn_state.n;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: cast zx s_1_24 -> i
        let s_1_27: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_28: call Z_read(s_1_26, s_1_27)
        let s_1_28: Bits = Z_read(state, tracer, s_1_26, s_1_27);
        // D s_1_29: write-var operand1 <= s_1_28
        fn_state.operand1 = s_1_28;
        // D s_1_30: read-var VLshadow#3961:i64
        let s_1_30: i64 = fn_state.VLshadow_3961;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: read-var m:i64
        let s_1_33: i64 = fn_state.m;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: cast zx s_1_32 -> i
        let s_1_35: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_36: call Z_read(s_1_34, s_1_35)
        let s_1_36: Bits = Z_read(state, tracer, s_1_34, s_1_35);
        // D s_1_37: write-var operand2 <= s_1_36
        fn_state.operand2 = s_1_36;
        // D s_1_38: read-var VLshadow#3961:i64
        let s_1_38: i64 = fn_state.VLshadow_3961;
        // D s_1_39: cast zx s_1_38 -> i
        let s_1_39: i128 = (i128::try_from(s_1_38).unwrap());
        // D s_1_40: cast reint s_1_39 -> i64
        let s_1_40: i64 = (s_1_39 as i64);
        // D s_1_41: read-var da:i64
        let s_1_41: i64 = fn_state.da;
        // D s_1_42: cast zx s_1_41 -> i
        let s_1_42: i128 = (i128::try_from(s_1_41).unwrap());
        // D s_1_43: cast zx s_1_40 -> i
        let s_1_43: i128 = (i128::try_from(s_1_40).unwrap());
        // D s_1_44: call Z_read(s_1_42, s_1_43)
        let s_1_44: Bits = Z_read(state, tracer, s_1_42, s_1_43);
        // D s_1_45: write-var result <= s_1_44
        fn_state.result = s_1_44;
        // C s_1_46: const #0s : i64
        let s_1_46: i64 = 0;
        // C s_1_47: const #1s : i
        let s_1_47: i128 = 1;
        // D s_1_48: cast zx s_1_11 -> i
        let s_1_48: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_49: sub s_1_48 s_1_47
        let s_1_49: i128 = ((s_1_48) - (s_1_47));
        // D s_1_50: cast reint s_1_49 -> i64
        let s_1_50: i64 = (s_1_49 as i64);
        // D s_1_51: write-var gs#213338 <= s_1_50
        fn_state.gs_213338 = s_1_50;
        // D s_1_52: write-var e <= s_1_46
        fn_state.e = s_1_46;
        // N s_1_53: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#213338:i64
        let s_2_1: i64 = fn_state.gs_213338;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var eltspersegment:i64
        let s_3_2: i64 = fn_state.eltspersegment;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: mod s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) % (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var e:i64
        let s_3_6: i64 = fn_state.e;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast zx s_3_5 -> i
        let s_3_8: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_9: sub s_3_7 s_3_8
        let s_3_9: i128 = ((s_3_7) - (s_3_8));
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // C s_3_11: const #2s : i
        let s_3_11: i128 = 2;
        // D s_3_12: read-var e:i64
        let s_3_12: i64 = fn_state.e;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: mul s_3_11 s_3_13
        let s_3_14: i128 = ((s_3_11) * (s_3_13));
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: read-var sel:i64
        let s_3_17: i64 = fn_state.sel;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: add s_3_16 s_3_18
        let s_3_19: i128 = (s_3_16 + s_3_18);
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: read-var esize:i64
        let s_3_21: i64 = fn_state.esize;
        // D s_3_22: cast zx s_3_21 -> i
        let s_3_22: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // D s_3_24: cast zx s_3_20 -> i
        let s_3_24: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_26: read-var operand1:bv
        let s_3_26: Bits = fn_state.operand1;
        // D s_3_27: call Elem_read(s_3_26, s_3_24, s_3_25)
        let s_3_27: Bits = Elem_read(state, tracer, s_3_26, s_3_24, s_3_25);
        // D s_3_28: cast reint s_3_27 -> u16
        let s_3_28: u16 = (s_3_27.value() as u16);
        // D s_3_29: cast zx s_3_28 -> bv
        let s_3_29: Bits = Bits::new(s_3_28 as u128, 16u16);
        // D s_3_30: cast sx s_3_29 -> i
        let s_3_30: i128 = {
            let sign_bit = s_3_29.length() - 1;
            let mut result = s_3_29.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_31: cast reint s_3_30 -> i64
        let s_3_31: i64 = (s_3_30 as i64);
        // C s_3_32: const #2s : i
        let s_3_32: i128 = 2;
        // D s_3_33: cast zx s_3_10 -> i
        let s_3_33: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_34: mul s_3_32 s_3_33
        let s_3_34: i128 = ((s_3_32) * (s_3_33));
        // D s_3_35: cast reint s_3_34 -> i64
        let s_3_35: i64 = (s_3_34 as i64);
        // D s_3_36: cast zx s_3_35 -> i
        let s_3_36: i128 = (i128::try_from(s_3_35).unwrap());
        // D s_3_37: read-var index:i64
        let s_3_37: i64 = fn_state.index;
        // D s_3_38: cast zx s_3_37 -> i
        let s_3_38: i128 = (i128::try_from(s_3_37).unwrap());
        // D s_3_39: add s_3_36 s_3_38
        let s_3_39: i128 = (s_3_36 + s_3_38);
        // D s_3_40: cast reint s_3_39 -> i64
        let s_3_40: i64 = (s_3_39 as i64);
        // D s_3_41: read-var esize:i64
        let s_3_41: i64 = fn_state.esize;
        // D s_3_42: cast zx s_3_41 -> i
        let s_3_42: i128 = (i128::try_from(s_3_41).unwrap());
        // D s_3_43: cast reint s_3_42 -> i64
        let s_3_43: i64 = (s_3_42 as i64);
        // D s_3_44: cast zx s_3_40 -> i
        let s_3_44: i128 = (i128::try_from(s_3_40).unwrap());
        // D s_3_45: cast zx s_3_43 -> i
        let s_3_45: i128 = (i128::try_from(s_3_43).unwrap());
        // D s_3_46: read-var operand2:bv
        let s_3_46: Bits = fn_state.operand2;
        // D s_3_47: call Elem_read(s_3_46, s_3_44, s_3_45)
        let s_3_47: Bits = Elem_read(state, tracer, s_3_46, s_3_44, s_3_45);
        // D s_3_48: cast reint s_3_47 -> u16
        let s_3_48: u16 = (s_3_47.value() as u16);
        // D s_3_49: cast zx s_3_48 -> bv
        let s_3_49: Bits = Bits::new(s_3_48 as u128, 16u16);
        // D s_3_50: cast sx s_3_49 -> i
        let s_3_50: i128 = {
            let sign_bit = s_3_49.length() - 1;
            let mut result = s_3_49.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_51: cast reint s_3_50 -> i64
        let s_3_51: i64 = (s_3_50 as i64);
        // C s_3_52: const #2s : i
        let s_3_52: i128 = 2;
        // D s_3_53: read-var esize:i64
        let s_3_53: i64 = fn_state.esize;
        // D s_3_54: cast zx s_3_53 -> i
        let s_3_54: i128 = (i128::try_from(s_3_53).unwrap());
        // D s_3_55: mul s_3_52 s_3_54
        let s_3_55: i128 = ((s_3_52) * (s_3_54));
        // D s_3_56: cast reint s_3_55 -> i64
        let s_3_56: i64 = (s_3_55 as i64);
        // D s_3_57: cast zx s_3_56 -> i
        let s_3_57: i128 = (i128::try_from(s_3_56).unwrap());
        // D s_3_58: cast reint s_3_57 -> i64
        let s_3_58: i64 = (s_3_57 as i64);
        // D s_3_59: read-var e:i64
        let s_3_59: i64 = fn_state.e;
        // D s_3_60: cast zx s_3_59 -> i
        let s_3_60: i128 = (i128::try_from(s_3_59).unwrap());
        // D s_3_61: cast zx s_3_58 -> i
        let s_3_61: i128 = (i128::try_from(s_3_58).unwrap());
        // D s_3_62: read-var result:bv
        let s_3_62: Bits = fn_state.result;
        // D s_3_63: call Elem_read(s_3_62, s_3_60, s_3_61)
        let s_3_63: Bits = Elem_read(state, tracer, s_3_62, s_3_60, s_3_61);
        // D s_3_64: cast reint s_3_63 -> u32
        let s_3_64: u32 = (s_3_63.value() as u32);
        // D s_3_65: cast zx s_3_64 -> bv
        let s_3_65: Bits = Bits::new(s_3_64 as u128, 32u16);
        // D s_3_66: cast sx s_3_65 -> i
        let s_3_66: i128 = {
            let sign_bit = s_3_65.length() - 1;
            let mut result = s_3_65.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_67: cast reint s_3_66 -> i64
        let s_3_67: i64 = (s_3_66 as i64);
        // C s_3_68: const #2s : i
        let s_3_68: i128 = 2;
        // D s_3_69: cast zx s_3_31 -> i
        let s_3_69: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_70: mul s_3_68 s_3_69
        let s_3_70: i128 = ((s_3_68) * (s_3_69));
        // D s_3_71: cast reint s_3_70 -> i64
        let s_3_71: i64 = (s_3_70 as i64);
        // D s_3_72: cast zx s_3_71 -> i
        let s_3_72: i128 = (i128::try_from(s_3_71).unwrap());
        // D s_3_73: cast zx s_3_51 -> i
        let s_3_73: i128 = (i128::try_from(s_3_51).unwrap());
        // D s_3_74: mul s_3_72 s_3_73
        let s_3_74: i128 = ((s_3_72) * (s_3_73));
        // D s_3_75: cast reint s_3_74 -> i64
        let s_3_75: i64 = (s_3_74 as i64);
        // C s_3_76: const #2s : i
        let s_3_76: i128 = 2;
        // D s_3_77: read-var esize:i64
        let s_3_77: i64 = fn_state.esize;
        // D s_3_78: cast zx s_3_77 -> i
        let s_3_78: i128 = (i128::try_from(s_3_77).unwrap());
        // D s_3_79: mul s_3_76 s_3_78
        let s_3_79: i128 = ((s_3_76) * (s_3_78));
        // D s_3_80: cast reint s_3_79 -> i64
        let s_3_80: i64 = (s_3_79 as i64);
        // D s_3_81: cast zx s_3_80 -> i
        let s_3_81: i128 = (i128::try_from(s_3_80).unwrap());
        // D s_3_82: cast reint s_3_81 -> i64
        let s_3_82: i64 = (s_3_81 as i64);
        // D s_3_83: cast zx s_3_75 -> i
        let s_3_83: i128 = (i128::try_from(s_3_75).unwrap());
        // D s_3_84: cast zx s_3_82 -> i
        let s_3_84: i128 = (i128::try_from(s_3_82).unwrap());
        // D s_3_85: call SignedSat(s_3_83, s_3_84)
        let s_3_85: Bits = SignedSat(state, tracer, s_3_83, s_3_84);
        // D s_3_86: cast reint s_3_85 -> u32
        let s_3_86: u32 = (s_3_85.value() as u32);
        // D s_3_87: cast zx s_3_86 -> bv
        let s_3_87: Bits = Bits::new(s_3_86 as u128, 32u16);
        // D s_3_88: cast sx s_3_87 -> i
        let s_3_88: i128 = {
            let sign_bit = s_3_87.length() - 1;
            let mut result = s_3_87.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_89: cast reint s_3_88 -> i64
        let s_3_89: i64 = (s_3_88 as i64);
        // D s_3_90: cast zx s_3_67 -> i
        let s_3_90: i128 = (i128::try_from(s_3_67).unwrap());
        // D s_3_91: cast zx s_3_89 -> i
        let s_3_91: i128 = (i128::try_from(s_3_89).unwrap());
        // D s_3_92: add s_3_90 s_3_91
        let s_3_92: i128 = (s_3_90 + s_3_91);
        // D s_3_93: cast reint s_3_92 -> i64
        let s_3_93: i64 = (s_3_92 as i64);
        // C s_3_94: const #2s : i
        let s_3_94: i128 = 2;
        // D s_3_95: read-var esize:i64
        let s_3_95: i64 = fn_state.esize;
        // D s_3_96: cast zx s_3_95 -> i
        let s_3_96: i128 = (i128::try_from(s_3_95).unwrap());
        // D s_3_97: mul s_3_94 s_3_96
        let s_3_97: i128 = ((s_3_94) * (s_3_96));
        // D s_3_98: cast reint s_3_97 -> i64
        let s_3_98: i64 = (s_3_97 as i64);
        // D s_3_99: cast zx s_3_98 -> i
        let s_3_99: i128 = (i128::try_from(s_3_98).unwrap());
        // D s_3_100: cast reint s_3_99 -> i64
        let s_3_100: i64 = (s_3_99 as i64);
        // C s_3_101: const #2s : i
        let s_3_101: i128 = 2;
        // D s_3_102: read-var esize:i64
        let s_3_102: i64 = fn_state.esize;
        // D s_3_103: cast zx s_3_102 -> i
        let s_3_103: i128 = (i128::try_from(s_3_102).unwrap());
        // D s_3_104: mul s_3_101 s_3_103
        let s_3_104: i128 = ((s_3_101) * (s_3_103));
        // D s_3_105: cast reint s_3_104 -> i64
        let s_3_105: i64 = (s_3_104 as i64);
        // D s_3_106: cast zx s_3_105 -> i
        let s_3_106: i128 = (i128::try_from(s_3_105).unwrap());
        // D s_3_107: cast reint s_3_106 -> i64
        let s_3_107: i64 = (s_3_106 as i64);
        // D s_3_108: cast zx s_3_93 -> i
        let s_3_108: i128 = (i128::try_from(s_3_93).unwrap());
        // D s_3_109: cast zx s_3_107 -> i
        let s_3_109: i128 = (i128::try_from(s_3_107).unwrap());
        // D s_3_110: call SignedSat(s_3_108, s_3_109)
        let s_3_110: Bits = SignedSat(state, tracer, s_3_108, s_3_109);
        // D s_3_111: cast reint s_3_110 -> u32
        let s_3_111: u32 = (s_3_110.value() as u32);
        // D s_3_112: read-var e:i64
        let s_3_112: i64 = fn_state.e;
        // D s_3_113: cast zx s_3_112 -> i
        let s_3_113: i128 = (i128::try_from(s_3_112).unwrap());
        // D s_3_114: cast zx s_3_100 -> i
        let s_3_114: i128 = (i128::try_from(s_3_100).unwrap());
        // D s_3_115: cast zx s_3_111 -> bv
        let s_3_115: Bits = Bits::new(s_3_111 as u128, 32u16);
        // D s_3_116: read-var result:bv
        let s_3_116: Bits = fn_state.result;
        // D s_3_117: call Elem_set(s_3_116, s_3_113, s_3_114, s_3_115)
        let s_3_117: Bits = Elem_set(state, tracer, s_3_116, s_3_113, s_3_114, s_3_115);
        // D s_3_118: write-var result <= s_3_117
        fn_state.result = s_3_117;
        // D s_3_119: read-var e:i64
        let s_3_119: i64 = fn_state.e;
        // C s_3_120: const #1s : i64
        let s_3_120: i64 = 1;
        // D s_3_121: add s_3_119 s_3_120
        let s_3_121: i64 = (s_3_119 + s_3_120);
        // D s_3_122: write-var e <= s_3_121
        fn_state.e = s_3_121;
        // N s_3_123: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#3961:i64
        let s_4_0: i64 = fn_state.VLshadow_3961;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var da:i64
        let s_4_3: i64 = fn_state.da;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
