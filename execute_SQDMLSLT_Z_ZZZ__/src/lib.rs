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
pub fn execute_SQDMLSLT_Z_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    m: i64,
    n: i64,
    sel1: i64,
    sel2: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_206807: i64,
        VLshadow_3669: i64,
        VLshadow_3670: i64,
        result: Bits,
        esizeshadow_3668: i64,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        m: i64,
        n: i64,
        sel1: i64,
        sel2: i64,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        m,
        n,
        sel1,
        sel2,
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
        // D s_0_3: write-var esizeshadow#3668 <= s_0_2
        fn_state.esizeshadow_3668 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3669 <= s_0_6
        fn_state.VLshadow_3669 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3669:i64
        let s_1_0: i64 = fn_state.VLshadow_3669;
        // D s_1_1: write-var VLshadow#3670 <= s_1_0
        fn_state.VLshadow_3670 = s_1_0;
        // D s_1_2: read-var VLshadow#3670:i64
        let s_1_2: i64 = fn_state.VLshadow_3670;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#3668:i64
        let s_1_4: i64 = fn_state.esizeshadow_3668;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#3670:i64
        let s_1_8: i64 = fn_state.VLshadow_3670;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var n:i64
        let s_1_11: i64 = fn_state.n;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // D s_1_16: read-var VLshadow#3670:i64
        let s_1_16: i64 = fn_state.VLshadow_3670;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var m:i64
        let s_1_19: i64 = fn_state.m;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call Z_read(s_1_20, s_1_21)
        let s_1_22: Bits = Z_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var operand2 <= s_1_22
        fn_state.operand2 = s_1_22;
        // D s_1_24: read-var VLshadow#3670:i64
        let s_1_24: i64 = fn_state.VLshadow_3670;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: read-var da:i64
        let s_1_27: i64 = fn_state.da;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast zx s_1_26 -> i
        let s_1_29: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_30: call Z_read(s_1_28, s_1_29)
        let s_1_30: Bits = Z_read(state, tracer, s_1_28, s_1_29);
        // D s_1_31: write-var result <= s_1_30
        fn_state.result = s_1_30;
        // C s_1_32: const #0s : i64
        let s_1_32: i64 = 0;
        // C s_1_33: const #1s : i
        let s_1_33: i128 = 1;
        // D s_1_34: cast zx s_1_7 -> i
        let s_1_34: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_35: sub s_1_34 s_1_33
        let s_1_35: i128 = ((s_1_34) - (s_1_33));
        // D s_1_36: cast reint s_1_35 -> i64
        let s_1_36: i64 = (s_1_35 as i64);
        // D s_1_37: write-var gs#206807 <= s_1_36
        fn_state.gs_206807 = s_1_36;
        // D s_1_38: write-var e <= s_1_32
        fn_state.e = s_1_32;
        // N s_1_39: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#206807:i64
        let s_2_1: i64 = fn_state.gs_206807;
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
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: read-var sel1:i64
        let s_3_6: i64 = fn_state.sel1;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: add s_3_5 s_3_7
        let s_3_8: i128 = (s_3_5 + s_3_7);
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // C s_3_10: const #2s : i
        let s_3_10: i128 = 2;
        // D s_3_11: read-var esizeshadow#3668:i64
        let s_3_11: i64 = fn_state.esizeshadow_3668;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: div s_3_12 s_3_10
        let s_3_13: i128 = ((s_3_12) / (s_3_10));
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: cast reint s_3_15 -> i64
        let s_3_16: i64 = (s_3_15 as i64);
        // D s_3_17: cast zx s_3_9 -> i
        let s_3_17: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_18: cast zx s_3_16 -> i
        let s_3_18: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_19: read-var operand1:bv
        let s_3_19: Bits = fn_state.operand1;
        // D s_3_20: call Elem_read(s_3_19, s_3_17, s_3_18)
        let s_3_20: Bits = Elem_read(state, tracer, s_3_19, s_3_17, s_3_18);
        // D s_3_21: cast sx s_3_20 -> i
        let s_3_21: i128 = {
            let sign_bit = s_3_20.length() - 1;
            let mut result = s_3_20.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_22: cast reint s_3_21 -> i64
        let s_3_22: i64 = (s_3_21 as i64);
        // C s_3_23: const #2s : i
        let s_3_23: i128 = 2;
        // D s_3_24: read-var e:i64
        let s_3_24: i64 = fn_state.e;
        // D s_3_25: cast zx s_3_24 -> i
        let s_3_25: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_26: mul s_3_23 s_3_25
        let s_3_26: i128 = ((s_3_23) * (s_3_25));
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: cast zx s_3_27 -> i
        let s_3_28: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_29: read-var sel2:i64
        let s_3_29: i64 = fn_state.sel2;
        // D s_3_30: cast zx s_3_29 -> i
        let s_3_30: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_31: add s_3_28 s_3_30
        let s_3_31: i128 = (s_3_28 + s_3_30);
        // D s_3_32: cast reint s_3_31 -> i64
        let s_3_32: i64 = (s_3_31 as i64);
        // C s_3_33: const #2s : i
        let s_3_33: i128 = 2;
        // D s_3_34: read-var esizeshadow#3668:i64
        let s_3_34: i64 = fn_state.esizeshadow_3668;
        // D s_3_35: cast zx s_3_34 -> i
        let s_3_35: i128 = (i128::try_from(s_3_34).unwrap());
        // D s_3_36: div s_3_35 s_3_33
        let s_3_36: i128 = ((s_3_35) / (s_3_33));
        // D s_3_37: cast reint s_3_36 -> i64
        let s_3_37: i64 = (s_3_36 as i64);
        // D s_3_38: cast zx s_3_37 -> i
        let s_3_38: i128 = (i128::try_from(s_3_37).unwrap());
        // D s_3_39: cast reint s_3_38 -> i64
        let s_3_39: i64 = (s_3_38 as i64);
        // D s_3_40: cast zx s_3_32 -> i
        let s_3_40: i128 = (i128::try_from(s_3_32).unwrap());
        // D s_3_41: cast zx s_3_39 -> i
        let s_3_41: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_42: read-var operand2:bv
        let s_3_42: Bits = fn_state.operand2;
        // D s_3_43: call Elem_read(s_3_42, s_3_40, s_3_41)
        let s_3_43: Bits = Elem_read(state, tracer, s_3_42, s_3_40, s_3_41);
        // D s_3_44: cast sx s_3_43 -> i
        let s_3_44: i128 = {
            let sign_bit = s_3_43.length() - 1;
            let mut result = s_3_43.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_45: cast reint s_3_44 -> i64
        let s_3_45: i64 = (s_3_44 as i64);
        // D s_3_46: read-var esizeshadow#3668:i64
        let s_3_46: i64 = fn_state.esizeshadow_3668;
        // D s_3_47: cast zx s_3_46 -> i
        let s_3_47: i128 = (i128::try_from(s_3_46).unwrap());
        // D s_3_48: cast reint s_3_47 -> i64
        let s_3_48: i64 = (s_3_47 as i64);
        // D s_3_49: read-var e:i64
        let s_3_49: i64 = fn_state.e;
        // D s_3_50: cast zx s_3_49 -> i
        let s_3_50: i128 = (i128::try_from(s_3_49).unwrap());
        // D s_3_51: cast zx s_3_48 -> i
        let s_3_51: i128 = (i128::try_from(s_3_48).unwrap());
        // D s_3_52: read-var result:bv
        let s_3_52: Bits = fn_state.result;
        // D s_3_53: call Elem_read(s_3_52, s_3_50, s_3_51)
        let s_3_53: Bits = Elem_read(state, tracer, s_3_52, s_3_50, s_3_51);
        // D s_3_54: cast sx s_3_53 -> i
        let s_3_54: i128 = {
            let sign_bit = s_3_53.length() - 1;
            let mut result = s_3_53.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_55: cast reint s_3_54 -> i64
        let s_3_55: i64 = (s_3_54 as i64);
        // C s_3_56: const #2s : i
        let s_3_56: i128 = 2;
        // D s_3_57: cast zx s_3_22 -> i
        let s_3_57: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_58: mul s_3_56 s_3_57
        let s_3_58: i128 = ((s_3_56) * (s_3_57));
        // D s_3_59: cast reint s_3_58 -> i64
        let s_3_59: i64 = (s_3_58 as i64);
        // D s_3_60: cast zx s_3_59 -> i
        let s_3_60: i128 = (i128::try_from(s_3_59).unwrap());
        // D s_3_61: cast zx s_3_45 -> i
        let s_3_61: i128 = (i128::try_from(s_3_45).unwrap());
        // D s_3_62: mul s_3_60 s_3_61
        let s_3_62: i128 = ((s_3_60) * (s_3_61));
        // D s_3_63: read-var esizeshadow#3668:i64
        let s_3_63: i64 = fn_state.esizeshadow_3668;
        // D s_3_64: cast zx s_3_63 -> i
        let s_3_64: i128 = (i128::try_from(s_3_63).unwrap());
        // D s_3_65: cast reint s_3_64 -> i64
        let s_3_65: i64 = (s_3_64 as i64);
        // D s_3_66: cast zx s_3_65 -> i
        let s_3_66: i128 = (i128::try_from(s_3_65).unwrap());
        // D s_3_67: call SignedSat(s_3_62, s_3_66)
        let s_3_67: Bits = SignedSat(state, tracer, s_3_62, s_3_66);
        // D s_3_68: cast sx s_3_67 -> i
        let s_3_68: i128 = {
            let sign_bit = s_3_67.length() - 1;
            let mut result = s_3_67.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_69: cast reint s_3_68 -> i64
        let s_3_69: i64 = (s_3_68 as i64);
        // D s_3_70: read-var esizeshadow#3668:i64
        let s_3_70: i64 = fn_state.esizeshadow_3668;
        // D s_3_71: cast zx s_3_70 -> i
        let s_3_71: i128 = (i128::try_from(s_3_70).unwrap());
        // D s_3_72: cast reint s_3_71 -> i64
        let s_3_72: i64 = (s_3_71 as i64);
        // D s_3_73: cast zx s_3_55 -> i
        let s_3_73: i128 = (i128::try_from(s_3_55).unwrap());
        // D s_3_74: cast zx s_3_69 -> i
        let s_3_74: i128 = (i128::try_from(s_3_69).unwrap());
        // D s_3_75: sub s_3_73 s_3_74
        let s_3_75: i128 = ((s_3_73) - (s_3_74));
        // D s_3_76: read-var esizeshadow#3668:i64
        let s_3_76: i64 = fn_state.esizeshadow_3668;
        // D s_3_77: cast zx s_3_76 -> i
        let s_3_77: i128 = (i128::try_from(s_3_76).unwrap());
        // D s_3_78: cast reint s_3_77 -> i64
        let s_3_78: i64 = (s_3_77 as i64);
        // D s_3_79: cast zx s_3_78 -> i
        let s_3_79: i128 = (i128::try_from(s_3_78).unwrap());
        // D s_3_80: call SignedSat(s_3_75, s_3_79)
        let s_3_80: Bits = SignedSat(state, tracer, s_3_75, s_3_79);
        // D s_3_81: read-var e:i64
        let s_3_81: i64 = fn_state.e;
        // D s_3_82: cast zx s_3_81 -> i
        let s_3_82: i128 = (i128::try_from(s_3_81).unwrap());
        // D s_3_83: cast zx s_3_72 -> i
        let s_3_83: i128 = (i128::try_from(s_3_72).unwrap());
        // D s_3_84: read-var result:bv
        let s_3_84: Bits = fn_state.result;
        // D s_3_85: call Elem_set(s_3_84, s_3_82, s_3_83, s_3_80)
        let s_3_85: Bits = Elem_set(state, tracer, s_3_84, s_3_82, s_3_83, s_3_80);
        // D s_3_86: write-var result <= s_3_85
        fn_state.result = s_3_85;
        // D s_3_87: read-var e:i64
        let s_3_87: i64 = fn_state.e;
        // C s_3_88: const #1s : i64
        let s_3_88: i64 = 1;
        // D s_3_89: add s_3_87 s_3_88
        let s_3_89: i64 = (s_3_87 + s_3_88);
        // D s_3_90: write-var e <= s_3_89
        fn_state.e = s_3_89;
        // N s_3_91: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#3670:i64
        let s_4_0: i64 = fn_state.VLshadow_3670;
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
