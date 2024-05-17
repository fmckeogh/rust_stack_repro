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
pub fn execute_SQCADD_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    m: i64,
    sub_i: bool,
    sub_r: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand2: Bits,
        p: i64,
        acc_r: i128,
        VLshadow_3866: i64,
        elt2_r: i64,
        VLshadow_3865: i64,
        gs_210133: i64,
        acc_i: i128,
        elt2_i: i64,
        esizeshadow_3864: i64,
        result: Bits,
        operand1: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        m: i64,
        sub_i: bool,
        sub_r: bool,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        m,
        sub_i,
        sub_r,
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
        // D s_0_3: write-var esizeshadow#3864 <= s_0_2
        fn_state.esizeshadow_3864 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3865 <= s_0_6
        fn_state.VLshadow_3865 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3865:i64
        let s_1_0: i64 = fn_state.VLshadow_3865;
        // D s_1_1: write-var VLshadow#3866 <= s_1_0
        fn_state.VLshadow_3866 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esizeshadow#3864:i64
        let s_1_3: i64 = fn_state.esizeshadow_3864;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_2 s_1_4
        let s_1_5: i128 = ((s_1_2) * (s_1_4));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#3866:i64
        let s_1_7: i64 = fn_state.VLshadow_3866;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#3866:i64
        let s_1_12: i64 = fn_state.VLshadow_3866;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var dn:i64
        let s_1_15: i64 = fn_state.dn;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call Z_read(s_1_16, s_1_17)
        let s_1_18: Bits = Z_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var operand1 <= s_1_18
        fn_state.operand1 = s_1_18;
        // D s_1_20: read-var VLshadow#3866:i64
        let s_1_20: i64 = fn_state.VLshadow_3866;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var m:i64
        let s_1_23: i64 = fn_state.m;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call Z_read(s_1_24, s_1_25)
        let s_1_26: Bits = Z_read(state, tracer, s_1_24, s_1_25);
        // D s_1_27: write-var operand2 <= s_1_26
        fn_state.operand2 = s_1_26;
        // C s_1_28: const #0s : i64
        let s_1_28: i64 = 0;
        // C s_1_29: const #1s : i
        let s_1_29: i128 = 1;
        // D s_1_30: cast zx s_1_11 -> i
        let s_1_30: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_31: sub s_1_30 s_1_29
        let s_1_31: i128 = ((s_1_30) - (s_1_29));
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: write-var gs#210133 <= s_1_32
        fn_state.gs_210133 = s_1_32;
        // D s_1_34: write-var p <= s_1_28
        fn_state.p = s_1_28;
        // N s_1_35: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#210133:i64
        let s_2_1: i64 = fn_state.gs_210133;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
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
        // D s_3_1: read-var p:i64
        let s_3_1: i64 = fn_state.p;
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
        // D s_3_9: read-var esizeshadow#3864:i64
        let s_3_9: i64 = fn_state.esizeshadow_3864;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: cast zx s_3_8 -> i
        let s_3_12: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_13: cast zx s_3_11 -> i
        let s_3_13: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_14: read-var operand1:bv
        let s_3_14: Bits = fn_state.operand1;
        // D s_3_15: call Elem_read(s_3_14, s_3_12, s_3_13)
        let s_3_15: Bits = Elem_read(state, tracer, s_3_14, s_3_12, s_3_13);
        // D s_3_16: cast sx s_3_15 -> i
        let s_3_16: i128 = {
            let sign_bit = s_3_15.length() - 1;
            let mut result = s_3_15.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_17: write-var acc_r <= s_3_16
        fn_state.acc_r = s_3_16;
        // C s_3_18: const #2s : i
        let s_3_18: i128 = 2;
        // D s_3_19: read-var p:i64
        let s_3_19: i64 = fn_state.p;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: mul s_3_18 s_3_20
        let s_3_21: i128 = ((s_3_18) * (s_3_20));
        // D s_3_22: cast reint s_3_21 -> i64
        let s_3_22: i64 = (s_3_21 as i64);
        // C s_3_23: const #1s : i
        let s_3_23: i128 = 1;
        // D s_3_24: cast zx s_3_22 -> i
        let s_3_24: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_25: add s_3_24 s_3_23
        let s_3_25: i128 = (s_3_24 + s_3_23);
        // D s_3_26: cast reint s_3_25 -> i64
        let s_3_26: i64 = (s_3_25 as i64);
        // D s_3_27: read-var esizeshadow#3864:i64
        let s_3_27: i64 = fn_state.esizeshadow_3864;
        // D s_3_28: cast zx s_3_27 -> i
        let s_3_28: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_29: cast reint s_3_28 -> i64
        let s_3_29: i64 = (s_3_28 as i64);
        // D s_3_30: cast zx s_3_26 -> i
        let s_3_30: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_31: cast zx s_3_29 -> i
        let s_3_31: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_32: read-var operand1:bv
        let s_3_32: Bits = fn_state.operand1;
        // D s_3_33: call Elem_read(s_3_32, s_3_30, s_3_31)
        let s_3_33: Bits = Elem_read(state, tracer, s_3_32, s_3_30, s_3_31);
        // D s_3_34: cast sx s_3_33 -> i
        let s_3_34: i128 = {
            let sign_bit = s_3_33.length() - 1;
            let mut result = s_3_33.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_35: write-var acc_i <= s_3_34
        fn_state.acc_i = s_3_34;
        // C s_3_36: const #2s : i
        let s_3_36: i128 = 2;
        // D s_3_37: read-var p:i64
        let s_3_37: i64 = fn_state.p;
        // D s_3_38: cast zx s_3_37 -> i
        let s_3_38: i128 = (i128::try_from(s_3_37).unwrap());
        // D s_3_39: mul s_3_36 s_3_38
        let s_3_39: i128 = ((s_3_36) * (s_3_38));
        // D s_3_40: cast reint s_3_39 -> i64
        let s_3_40: i64 = (s_3_39 as i64);
        // C s_3_41: const #0s : i
        let s_3_41: i128 = 0;
        // D s_3_42: cast zx s_3_40 -> i
        let s_3_42: i128 = (i128::try_from(s_3_40).unwrap());
        // D s_3_43: add s_3_42 s_3_41
        let s_3_43: i128 = (s_3_42 + s_3_41);
        // D s_3_44: cast reint s_3_43 -> i64
        let s_3_44: i64 = (s_3_43 as i64);
        // D s_3_45: read-var esizeshadow#3864:i64
        let s_3_45: i64 = fn_state.esizeshadow_3864;
        // D s_3_46: cast zx s_3_45 -> i
        let s_3_46: i128 = (i128::try_from(s_3_45).unwrap());
        // D s_3_47: cast reint s_3_46 -> i64
        let s_3_47: i64 = (s_3_46 as i64);
        // D s_3_48: cast zx s_3_44 -> i
        let s_3_48: i128 = (i128::try_from(s_3_44).unwrap());
        // D s_3_49: cast zx s_3_47 -> i
        let s_3_49: i128 = (i128::try_from(s_3_47).unwrap());
        // D s_3_50: read-var operand2:bv
        let s_3_50: Bits = fn_state.operand2;
        // D s_3_51: call Elem_read(s_3_50, s_3_48, s_3_49)
        let s_3_51: Bits = Elem_read(state, tracer, s_3_50, s_3_48, s_3_49);
        // D s_3_52: cast sx s_3_51 -> i
        let s_3_52: i128 = {
            let sign_bit = s_3_51.length() - 1;
            let mut result = s_3_51.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_53: cast reint s_3_52 -> i64
        let s_3_53: i64 = (s_3_52 as i64);
        // D s_3_54: write-var elt2_r <= s_3_53
        fn_state.elt2_r = s_3_53;
        // C s_3_55: const #2s : i
        let s_3_55: i128 = 2;
        // D s_3_56: read-var p:i64
        let s_3_56: i64 = fn_state.p;
        // D s_3_57: cast zx s_3_56 -> i
        let s_3_57: i128 = (i128::try_from(s_3_56).unwrap());
        // D s_3_58: mul s_3_55 s_3_57
        let s_3_58: i128 = ((s_3_55) * (s_3_57));
        // D s_3_59: cast reint s_3_58 -> i64
        let s_3_59: i64 = (s_3_58 as i64);
        // C s_3_60: const #1s : i
        let s_3_60: i128 = 1;
        // D s_3_61: cast zx s_3_59 -> i
        let s_3_61: i128 = (i128::try_from(s_3_59).unwrap());
        // D s_3_62: add s_3_61 s_3_60
        let s_3_62: i128 = (s_3_61 + s_3_60);
        // D s_3_63: cast reint s_3_62 -> i64
        let s_3_63: i64 = (s_3_62 as i64);
        // D s_3_64: read-var esizeshadow#3864:i64
        let s_3_64: i64 = fn_state.esizeshadow_3864;
        // D s_3_65: cast zx s_3_64 -> i
        let s_3_65: i128 = (i128::try_from(s_3_64).unwrap());
        // D s_3_66: cast reint s_3_65 -> i64
        let s_3_66: i64 = (s_3_65 as i64);
        // D s_3_67: cast zx s_3_63 -> i
        let s_3_67: i128 = (i128::try_from(s_3_63).unwrap());
        // D s_3_68: cast zx s_3_66 -> i
        let s_3_68: i128 = (i128::try_from(s_3_66).unwrap());
        // D s_3_69: read-var operand2:bv
        let s_3_69: Bits = fn_state.operand2;
        // D s_3_70: call Elem_read(s_3_69, s_3_67, s_3_68)
        let s_3_70: Bits = Elem_read(state, tracer, s_3_69, s_3_67, s_3_68);
        // D s_3_71: cast sx s_3_70 -> i
        let s_3_71: i128 = {
            let sign_bit = s_3_70.length() - 1;
            let mut result = s_3_70.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_72: cast reint s_3_71 -> i64
        let s_3_72: i64 = (s_3_71 as i64);
        // D s_3_73: write-var elt2_i <= s_3_72
        fn_state.elt2_i = s_3_72;
        // D s_3_74: read-var sub_i:u8
        let s_3_74: bool = fn_state.sub_i;
        // N s_3_75: branch s_3_74 b9 b4
        if s_3_74 {
            return block_9(state, tracer, fn_state);
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
        // D s_5_0: read-var sub_r:u8
        let s_5_0: bool = fn_state.sub_r;
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
            return block_8(state, tracer, fn_state);
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
        // D s_7_0: read-var acc_r:i
        let s_7_0: i128 = fn_state.acc_r;
        // D s_7_1: read-var acc_i:i
        let s_7_1: i128 = fn_state.acc_i;
        // C s_7_2: const #2s : i
        let s_7_2: i128 = 2;
        // D s_7_3: read-var p:i64
        let s_7_3: i64 = fn_state.p;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: mul s_7_2 s_7_4
        let s_7_5: i128 = ((s_7_2) * (s_7_4));
        // D s_7_6: cast reint s_7_5 -> i64
        let s_7_6: i64 = (s_7_5 as i64);
        // C s_7_7: const #0s : i
        let s_7_7: i128 = 0;
        // D s_7_8: cast zx s_7_6 -> i
        let s_7_8: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_9: add s_7_8 s_7_7
        let s_7_9: i128 = (s_7_8 + s_7_7);
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: read-var esizeshadow#3864:i64
        let s_7_11: i64 = fn_state.esizeshadow_3864;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // D s_7_14: read-var esizeshadow#3864:i64
        let s_7_14: i64 = fn_state.esizeshadow_3864;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: cast reint s_7_15 -> i64
        let s_7_16: i64 = (s_7_15 as i64);
        // D s_7_17: cast zx s_7_16 -> i
        let s_7_17: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_18: call SignedSat(s_7_0, s_7_17)
        let s_7_18: Bits = SignedSat(state, tracer, s_7_0, s_7_17);
        // D s_7_19: cast zx s_7_10 -> i
        let s_7_19: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_20: cast zx s_7_13 -> i
        let s_7_20: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_21: read-var result:bv
        let s_7_21: Bits = fn_state.result;
        // D s_7_22: call Elem_set(s_7_21, s_7_19, s_7_20, s_7_18)
        let s_7_22: Bits = Elem_set(state, tracer, s_7_21, s_7_19, s_7_20, s_7_18);
        // D s_7_23: write-var result <= s_7_22
        fn_state.result = s_7_22;
        // C s_7_24: const #2s : i
        let s_7_24: i128 = 2;
        // D s_7_25: read-var p:i64
        let s_7_25: i64 = fn_state.p;
        // D s_7_26: cast zx s_7_25 -> i
        let s_7_26: i128 = (i128::try_from(s_7_25).unwrap());
        // D s_7_27: mul s_7_24 s_7_26
        let s_7_27: i128 = ((s_7_24) * (s_7_26));
        // D s_7_28: cast reint s_7_27 -> i64
        let s_7_28: i64 = (s_7_27 as i64);
        // C s_7_29: const #1s : i
        let s_7_29: i128 = 1;
        // D s_7_30: cast zx s_7_28 -> i
        let s_7_30: i128 = (i128::try_from(s_7_28).unwrap());
        // D s_7_31: add s_7_30 s_7_29
        let s_7_31: i128 = (s_7_30 + s_7_29);
        // D s_7_32: cast reint s_7_31 -> i64
        let s_7_32: i64 = (s_7_31 as i64);
        // D s_7_33: read-var esizeshadow#3864:i64
        let s_7_33: i64 = fn_state.esizeshadow_3864;
        // D s_7_34: cast zx s_7_33 -> i
        let s_7_34: i128 = (i128::try_from(s_7_33).unwrap());
        // D s_7_35: cast reint s_7_34 -> i64
        let s_7_35: i64 = (s_7_34 as i64);
        // D s_7_36: read-var esizeshadow#3864:i64
        let s_7_36: i64 = fn_state.esizeshadow_3864;
        // D s_7_37: cast zx s_7_36 -> i
        let s_7_37: i128 = (i128::try_from(s_7_36).unwrap());
        // D s_7_38: cast reint s_7_37 -> i64
        let s_7_38: i64 = (s_7_37 as i64);
        // D s_7_39: cast zx s_7_38 -> i
        let s_7_39: i128 = (i128::try_from(s_7_38).unwrap());
        // D s_7_40: call SignedSat(s_7_1, s_7_39)
        let s_7_40: Bits = SignedSat(state, tracer, s_7_1, s_7_39);
        // D s_7_41: cast zx s_7_32 -> i
        let s_7_41: i128 = (i128::try_from(s_7_32).unwrap());
        // D s_7_42: cast zx s_7_35 -> i
        let s_7_42: i128 = (i128::try_from(s_7_35).unwrap());
        // D s_7_43: read-var result:bv
        let s_7_43: Bits = fn_state.result;
        // D s_7_44: call Elem_set(s_7_43, s_7_41, s_7_42, s_7_40)
        let s_7_44: Bits = Elem_set(state, tracer, s_7_43, s_7_41, s_7_42, s_7_40);
        // D s_7_45: write-var result <= s_7_44
        fn_state.result = s_7_44;
        // D s_7_46: read-var p:i64
        let s_7_46: i64 = fn_state.p;
        // C s_7_47: const #1s : i64
        let s_7_47: i64 = 1;
        // D s_7_48: add s_7_46 s_7_47
        let s_7_48: i64 = (s_7_46 + s_7_47);
        // D s_7_49: write-var p <= s_7_48
        fn_state.p = s_7_48;
        // N s_7_50: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var elt2_i:i64
        let s_8_0: i64 = fn_state.elt2_i;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var acc_r:i
        let s_8_2: i128 = fn_state.acc_r;
        // D s_8_3: add s_8_2 s_8_1
        let s_8_3: i128 = (s_8_2 + s_8_1);
        // D s_8_4: write-var acc_r <= s_8_3
        fn_state.acc_r = s_8_3;
        // D s_8_5: read-var elt2_r:i64
        let s_8_5: i64 = fn_state.elt2_r;
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: read-var acc_i:i
        let s_8_7: i128 = fn_state.acc_i;
        // D s_8_8: sub s_8_7 s_8_6
        let s_8_8: i128 = ((s_8_7) - (s_8_6));
        // D s_8_9: write-var acc_i <= s_8_8
        fn_state.acc_i = s_8_8;
        // N s_8_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var elt2_i:i64
        let s_9_0: i64 = fn_state.elt2_i;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var acc_r:i
        let s_9_2: i128 = fn_state.acc_r;
        // D s_9_3: sub s_9_2 s_9_1
        let s_9_3: i128 = ((s_9_2) - (s_9_1));
        // D s_9_4: write-var acc_r <= s_9_3
        fn_state.acc_r = s_9_3;
        // D s_9_5: read-var elt2_r:i64
        let s_9_5: i64 = fn_state.elt2_r;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: read-var acc_i:i
        let s_9_7: i128 = fn_state.acc_i;
        // D s_9_8: add s_9_7 s_9_6
        let s_9_8: i128 = (s_9_7 + s_9_6);
        // D s_9_9: write-var acc_i <= s_9_8
        fn_state.acc_i = s_9_8;
        // N s_9_10: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#3866:i64
        let s_10_0: i64 = fn_state.VLshadow_3866;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var dn:i64
        let s_10_3: i64 = fn_state.dn;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: read-var result:bv
        let s_10_6: Bits = fn_state.result;
        // D s_10_7: call Z_set(s_10_4, s_10_5, s_10_6)
        let s_10_7: () = Z_set(state, tracer, s_10_4, s_10_5, s_10_6);
        // N s_10_8: return
        return;
    }
}
