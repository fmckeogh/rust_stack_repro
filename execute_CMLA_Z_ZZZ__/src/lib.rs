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
use Z_set::*;
use common::*;
pub fn execute_CMLA_Z_ZZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    m: i64,
    n: i64,
    sel_a: i64,
    sel_b: i64,
    sub_i: bool,
    sub_r: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        p: i64,
        operand3: Bits,
        product_i: i128,
        VLshadow_3870: i64,
        product_r: i128,
        elt3_r: Bits,
        VLshadow_3871: i64,
        result: Bits,
        elt3_i: Bits,
        operand1: Bits,
        esizeshadow_3869: i64,
        gs_210203: i64,
        operand2: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        m: i64,
        n: i64,
        sel_a: i64,
        sel_b: i64,
        sub_i: bool,
        sub_r: bool,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        m,
        n,
        sel_a,
        sel_b,
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
        // D s_0_3: write-var esizeshadow#3869 <= s_0_2
        fn_state.esizeshadow_3869 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3870 <= s_0_6
        fn_state.VLshadow_3870 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3870:i64
        let s_1_0: i64 = fn_state.VLshadow_3870;
        // D s_1_1: write-var VLshadow#3871 <= s_1_0
        fn_state.VLshadow_3871 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esizeshadow#3869:i64
        let s_1_3: i64 = fn_state.esizeshadow_3869;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_2 s_1_4
        let s_1_5: i128 = ((s_1_2) * (s_1_4));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#3871:i64
        let s_1_7: i64 = fn_state.VLshadow_3871;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#3871:i64
        let s_1_12: i64 = fn_state.VLshadow_3871;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var n:i64
        let s_1_15: i64 = fn_state.n;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call Z_read(s_1_16, s_1_17)
        let s_1_18: Bits = Z_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var operand1 <= s_1_18
        fn_state.operand1 = s_1_18;
        // D s_1_20: read-var VLshadow#3871:i64
        let s_1_20: i64 = fn_state.VLshadow_3871;
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
        // D s_1_28: read-var VLshadow#3871:i64
        let s_1_28: i64 = fn_state.VLshadow_3871;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: read-var da:i64
        let s_1_31: i64 = fn_state.da;
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: cast zx s_1_30 -> i
        let s_1_33: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_34: call Z_read(s_1_32, s_1_33)
        let s_1_34: Bits = Z_read(state, tracer, s_1_32, s_1_33);
        // D s_1_35: write-var operand3 <= s_1_34
        fn_state.operand3 = s_1_34;
        // C s_1_36: const #0s : i64
        let s_1_36: i64 = 0;
        // C s_1_37: const #1s : i
        let s_1_37: i128 = 1;
        // D s_1_38: cast zx s_1_11 -> i
        let s_1_38: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_39: sub s_1_38 s_1_37
        let s_1_39: i128 = ((s_1_38) - (s_1_37));
        // D s_1_40: cast reint s_1_39 -> i64
        let s_1_40: i64 = (s_1_39 as i64);
        // D s_1_41: write-var gs#210203 <= s_1_40
        fn_state.gs_210203 = s_1_40;
        // D s_1_42: write-var p <= s_1_36
        fn_state.p = s_1_36;
        // N s_1_43: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#210203:i64
        let s_2_1: i64 = fn_state.gs_210203;
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
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: read-var sel_a:i64
        let s_3_6: i64 = fn_state.sel_a;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: add s_3_5 s_3_7
        let s_3_8: i128 = (s_3_5 + s_3_7);
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: read-var esizeshadow#3869:i64
        let s_3_10: i64 = fn_state.esizeshadow_3869;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: cast zx s_3_9 -> i
        let s_3_13: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_14: cast zx s_3_12 -> i
        let s_3_14: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_15: read-var operand1:bv
        let s_3_15: Bits = fn_state.operand1;
        // D s_3_16: call Elem_read(s_3_15, s_3_13, s_3_14)
        let s_3_16: Bits = Elem_read(state, tracer, s_3_15, s_3_13, s_3_14);
        // D s_3_17: cast sx s_3_16 -> i
        let s_3_17: i128 = {
            let sign_bit = s_3_16.length() - 1;
            let mut result = s_3_16.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // C s_3_19: const #2s : i
        let s_3_19: i128 = 2;
        // D s_3_20: read-var p:i64
        let s_3_20: i64 = fn_state.p;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: mul s_3_19 s_3_21
        let s_3_22: i128 = ((s_3_19) * (s_3_21));
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: read-var sel_a:i64
        let s_3_25: i64 = fn_state.sel_a;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: add s_3_24 s_3_26
        let s_3_27: i128 = (s_3_24 + s_3_26);
        // D s_3_28: cast reint s_3_27 -> i64
        let s_3_28: i64 = (s_3_27 as i64);
        // D s_3_29: read-var esizeshadow#3869:i64
        let s_3_29: i64 = fn_state.esizeshadow_3869;
        // D s_3_30: cast zx s_3_29 -> i
        let s_3_30: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_31: cast reint s_3_30 -> i64
        let s_3_31: i64 = (s_3_30 as i64);
        // D s_3_32: cast zx s_3_28 -> i
        let s_3_32: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_33: cast zx s_3_31 -> i
        let s_3_33: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_34: read-var operand2:bv
        let s_3_34: Bits = fn_state.operand2;
        // D s_3_35: call Elem_read(s_3_34, s_3_32, s_3_33)
        let s_3_35: Bits = Elem_read(state, tracer, s_3_34, s_3_32, s_3_33);
        // D s_3_36: cast sx s_3_35 -> i
        let s_3_36: i128 = {
            let sign_bit = s_3_35.length() - 1;
            let mut result = s_3_35.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_37: cast reint s_3_36 -> i64
        let s_3_37: i64 = (s_3_36 as i64);
        // C s_3_38: const #2s : i
        let s_3_38: i128 = 2;
        // D s_3_39: read-var p:i64
        let s_3_39: i64 = fn_state.p;
        // D s_3_40: cast zx s_3_39 -> i
        let s_3_40: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_41: mul s_3_38 s_3_40
        let s_3_41: i128 = ((s_3_38) * (s_3_40));
        // D s_3_42: cast reint s_3_41 -> i64
        let s_3_42: i64 = (s_3_41 as i64);
        // D s_3_43: cast zx s_3_42 -> i
        let s_3_43: i128 = (i128::try_from(s_3_42).unwrap());
        // D s_3_44: read-var sel_b:i64
        let s_3_44: i64 = fn_state.sel_b;
        // D s_3_45: cast zx s_3_44 -> i
        let s_3_45: i128 = (i128::try_from(s_3_44).unwrap());
        // D s_3_46: add s_3_43 s_3_45
        let s_3_46: i128 = (s_3_43 + s_3_45);
        // D s_3_47: cast reint s_3_46 -> i64
        let s_3_47: i64 = (s_3_46 as i64);
        // D s_3_48: read-var esizeshadow#3869:i64
        let s_3_48: i64 = fn_state.esizeshadow_3869;
        // D s_3_49: cast zx s_3_48 -> i
        let s_3_49: i128 = (i128::try_from(s_3_48).unwrap());
        // D s_3_50: cast reint s_3_49 -> i64
        let s_3_50: i64 = (s_3_49 as i64);
        // D s_3_51: cast zx s_3_47 -> i
        let s_3_51: i128 = (i128::try_from(s_3_47).unwrap());
        // D s_3_52: cast zx s_3_50 -> i
        let s_3_52: i128 = (i128::try_from(s_3_50).unwrap());
        // D s_3_53: read-var operand2:bv
        let s_3_53: Bits = fn_state.operand2;
        // D s_3_54: call Elem_read(s_3_53, s_3_51, s_3_52)
        let s_3_54: Bits = Elem_read(state, tracer, s_3_53, s_3_51, s_3_52);
        // D s_3_55: cast sx s_3_54 -> i
        let s_3_55: i128 = {
            let sign_bit = s_3_54.length() - 1;
            let mut result = s_3_54.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_56: cast reint s_3_55 -> i64
        let s_3_56: i64 = (s_3_55 as i64);
        // C s_3_57: const #2s : i
        let s_3_57: i128 = 2;
        // D s_3_58: read-var p:i64
        let s_3_58: i64 = fn_state.p;
        // D s_3_59: cast zx s_3_58 -> i
        let s_3_59: i128 = (i128::try_from(s_3_58).unwrap());
        // D s_3_60: mul s_3_57 s_3_59
        let s_3_60: i128 = ((s_3_57) * (s_3_59));
        // D s_3_61: cast reint s_3_60 -> i64
        let s_3_61: i64 = (s_3_60 as i64);
        // C s_3_62: const #0s : i
        let s_3_62: i128 = 0;
        // D s_3_63: cast zx s_3_61 -> i
        let s_3_63: i128 = (i128::try_from(s_3_61).unwrap());
        // D s_3_64: add s_3_63 s_3_62
        let s_3_64: i128 = (s_3_63 + s_3_62);
        // D s_3_65: cast reint s_3_64 -> i64
        let s_3_65: i64 = (s_3_64 as i64);
        // D s_3_66: read-var esizeshadow#3869:i64
        let s_3_66: i64 = fn_state.esizeshadow_3869;
        // D s_3_67: cast zx s_3_66 -> i
        let s_3_67: i128 = (i128::try_from(s_3_66).unwrap());
        // D s_3_68: cast reint s_3_67 -> i64
        let s_3_68: i64 = (s_3_67 as i64);
        // D s_3_69: cast zx s_3_65 -> i
        let s_3_69: i128 = (i128::try_from(s_3_65).unwrap());
        // D s_3_70: cast zx s_3_68 -> i
        let s_3_70: i128 = (i128::try_from(s_3_68).unwrap());
        // D s_3_71: read-var operand3:bv
        let s_3_71: Bits = fn_state.operand3;
        // D s_3_72: call Elem_read(s_3_71, s_3_69, s_3_70)
        let s_3_72: Bits = Elem_read(state, tracer, s_3_71, s_3_69, s_3_70);
        // D s_3_73: write-var elt3_r <= s_3_72
        fn_state.elt3_r = s_3_72;
        // C s_3_74: const #2s : i
        let s_3_74: i128 = 2;
        // D s_3_75: read-var p:i64
        let s_3_75: i64 = fn_state.p;
        // D s_3_76: cast zx s_3_75 -> i
        let s_3_76: i128 = (i128::try_from(s_3_75).unwrap());
        // D s_3_77: mul s_3_74 s_3_76
        let s_3_77: i128 = ((s_3_74) * (s_3_76));
        // D s_3_78: cast reint s_3_77 -> i64
        let s_3_78: i64 = (s_3_77 as i64);
        // C s_3_79: const #1s : i
        let s_3_79: i128 = 1;
        // D s_3_80: cast zx s_3_78 -> i
        let s_3_80: i128 = (i128::try_from(s_3_78).unwrap());
        // D s_3_81: add s_3_80 s_3_79
        let s_3_81: i128 = (s_3_80 + s_3_79);
        // D s_3_82: cast reint s_3_81 -> i64
        let s_3_82: i64 = (s_3_81 as i64);
        // D s_3_83: read-var esizeshadow#3869:i64
        let s_3_83: i64 = fn_state.esizeshadow_3869;
        // D s_3_84: cast zx s_3_83 -> i
        let s_3_84: i128 = (i128::try_from(s_3_83).unwrap());
        // D s_3_85: cast reint s_3_84 -> i64
        let s_3_85: i64 = (s_3_84 as i64);
        // D s_3_86: cast zx s_3_82 -> i
        let s_3_86: i128 = (i128::try_from(s_3_82).unwrap());
        // D s_3_87: cast zx s_3_85 -> i
        let s_3_87: i128 = (i128::try_from(s_3_85).unwrap());
        // D s_3_88: read-var operand3:bv
        let s_3_88: Bits = fn_state.operand3;
        // D s_3_89: call Elem_read(s_3_88, s_3_86, s_3_87)
        let s_3_89: Bits = Elem_read(state, tracer, s_3_88, s_3_86, s_3_87);
        // D s_3_90: write-var elt3_i <= s_3_89
        fn_state.elt3_i = s_3_89;
        // D s_3_91: cast zx s_3_18 -> i
        let s_3_91: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_92: cast zx s_3_37 -> i
        let s_3_92: i128 = (i128::try_from(s_3_37).unwrap());
        // D s_3_93: mul s_3_91 s_3_92
        let s_3_93: i128 = ((s_3_91) * (s_3_92));
        // D s_3_94: write-var product_r <= s_3_93
        fn_state.product_r = s_3_93;
        // D s_3_95: cast zx s_3_18 -> i
        let s_3_95: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_96: cast zx s_3_56 -> i
        let s_3_96: i128 = (i128::try_from(s_3_56).unwrap());
        // D s_3_97: mul s_3_95 s_3_96
        let s_3_97: i128 = ((s_3_95) * (s_3_96));
        // D s_3_98: write-var product_i <= s_3_97
        fn_state.product_i = s_3_97;
        // D s_3_99: read-var sub_r:u8
        let s_3_99: bool = fn_state.sub_r;
        // N s_3_100: branch s_3_99 b9 b4
        if s_3_99 {
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
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var p:i64
        let s_4_1: i64 = fn_state.p;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_0 s_4_2
        let s_4_3: i128 = ((s_4_0) * (s_4_2));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // C s_4_5: const #0s : i
        let s_4_5: i128 = 0;
        // D s_4_6: cast zx s_4_4 -> i
        let s_4_6: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_7: add s_4_6 s_4_5
        let s_4_7: i128 = (s_4_6 + s_4_5);
        // D s_4_8: cast reint s_4_7 -> i64
        let s_4_8: i64 = (s_4_7 as i64);
        // D s_4_9: read-var esizeshadow#3869:i64
        let s_4_9: i64 = fn_state.esizeshadow_3869;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: read-var elt3_r:bv
        let s_4_12: Bits = fn_state.elt3_r;
        // D s_4_13: read-var product_r:i
        let s_4_13: i128 = fn_state.product_r;
        // D s_4_14: cast cvt s_4_13 -> bv
        let s_4_14: Bits = Bits::new(s_4_13 as u128, 128);
        // D s_4_15: add s_4_12 s_4_14
        let s_4_15: Bits = (s_4_12 + s_4_14);
        // D s_4_16: cast zx s_4_8 -> i
        let s_4_16: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_17: cast zx s_4_11 -> i
        let s_4_17: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_18: read-var result:bv
        let s_4_18: Bits = fn_state.result;
        // D s_4_19: call Elem_set(s_4_18, s_4_16, s_4_17, s_4_15)
        let s_4_19: Bits = Elem_set(state, tracer, s_4_18, s_4_16, s_4_17, s_4_15);
        // D s_4_20: write-var result <= s_4_19
        fn_state.result = s_4_19;
        // N s_4_21: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var sub_i:u8
        let s_5_0: bool = fn_state.sub_i;
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
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var p:i64
        let s_6_1: i64 = fn_state.p;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: mul s_6_0 s_6_2
        let s_6_3: i128 = ((s_6_0) * (s_6_2));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // C s_6_5: const #1s : i
        let s_6_5: i128 = 1;
        // D s_6_6: cast zx s_6_4 -> i
        let s_6_6: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_7: add s_6_6 s_6_5
        let s_6_7: i128 = (s_6_6 + s_6_5);
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // D s_6_9: read-var esizeshadow#3869:i64
        let s_6_9: i64 = fn_state.esizeshadow_3869;
        // D s_6_10: cast zx s_6_9 -> i
        let s_6_10: i128 = (i128::try_from(s_6_9).unwrap());
        // D s_6_11: cast reint s_6_10 -> i64
        let s_6_11: i64 = (s_6_10 as i64);
        // D s_6_12: read-var elt3_i:bv
        let s_6_12: Bits = fn_state.elt3_i;
        // D s_6_13: read-var product_i:i
        let s_6_13: i128 = fn_state.product_i;
        // D s_6_14: cast cvt s_6_13 -> bv
        let s_6_14: Bits = Bits::new(s_6_13 as u128, 128);
        // D s_6_15: add s_6_12 s_6_14
        let s_6_15: Bits = (s_6_12 + s_6_14);
        // D s_6_16: cast zx s_6_8 -> i
        let s_6_16: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_17: cast zx s_6_11 -> i
        let s_6_17: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_18: read-var result:bv
        let s_6_18: Bits = fn_state.result;
        // D s_6_19: call Elem_set(s_6_18, s_6_16, s_6_17, s_6_15)
        let s_6_19: Bits = Elem_set(state, tracer, s_6_18, s_6_16, s_6_17, s_6_15);
        // D s_6_20: write-var result <= s_6_19
        fn_state.result = s_6_19;
        // N s_6_21: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var p:i64
        let s_7_0: i64 = fn_state.p;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var p <= s_7_2
        fn_state.p = s_7_2;
        // N s_7_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2s : i
        let s_8_0: i128 = 2;
        // D s_8_1: read-var p:i64
        let s_8_1: i64 = fn_state.p;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: mul s_8_0 s_8_2
        let s_8_3: i128 = ((s_8_0) * (s_8_2));
        // D s_8_4: cast reint s_8_3 -> i64
        let s_8_4: i64 = (s_8_3 as i64);
        // C s_8_5: const #1s : i
        let s_8_5: i128 = 1;
        // D s_8_6: cast zx s_8_4 -> i
        let s_8_6: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_7: add s_8_6 s_8_5
        let s_8_7: i128 = (s_8_6 + s_8_5);
        // D s_8_8: cast reint s_8_7 -> i64
        let s_8_8: i64 = (s_8_7 as i64);
        // D s_8_9: read-var esizeshadow#3869:i64
        let s_8_9: i64 = fn_state.esizeshadow_3869;
        // D s_8_10: cast zx s_8_9 -> i
        let s_8_10: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_11: cast reint s_8_10 -> i64
        let s_8_11: i64 = (s_8_10 as i64);
        // D s_8_12: read-var elt3_i:bv
        let s_8_12: Bits = fn_state.elt3_i;
        // D s_8_13: read-var product_i:i
        let s_8_13: i128 = fn_state.product_i;
        // D s_8_14: cast cvt s_8_13 -> bv
        let s_8_14: Bits = Bits::new(s_8_13 as u128, 128);
        // D s_8_15: sub s_8_12 s_8_14
        let s_8_15: Bits = ((s_8_12) - (s_8_14));
        // D s_8_16: cast zx s_8_8 -> i
        let s_8_16: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_17: cast zx s_8_11 -> i
        let s_8_17: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_18: read-var result:bv
        let s_8_18: Bits = fn_state.result;
        // D s_8_19: call Elem_set(s_8_18, s_8_16, s_8_17, s_8_15)
        let s_8_19: Bits = Elem_set(state, tracer, s_8_18, s_8_16, s_8_17, s_8_15);
        // D s_8_20: write-var result <= s_8_19
        fn_state.result = s_8_19;
        // N s_8_21: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #2s : i
        let s_9_0: i128 = 2;
        // D s_9_1: read-var p:i64
        let s_9_1: i64 = fn_state.p;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: mul s_9_0 s_9_2
        let s_9_3: i128 = ((s_9_0) * (s_9_2));
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // C s_9_5: const #0s : i
        let s_9_5: i128 = 0;
        // D s_9_6: cast zx s_9_4 -> i
        let s_9_6: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_7: add s_9_6 s_9_5
        let s_9_7: i128 = (s_9_6 + s_9_5);
        // D s_9_8: cast reint s_9_7 -> i64
        let s_9_8: i64 = (s_9_7 as i64);
        // D s_9_9: read-var esizeshadow#3869:i64
        let s_9_9: i64 = fn_state.esizeshadow_3869;
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // D s_9_12: read-var elt3_r:bv
        let s_9_12: Bits = fn_state.elt3_r;
        // D s_9_13: read-var product_r:i
        let s_9_13: i128 = fn_state.product_r;
        // D s_9_14: cast cvt s_9_13 -> bv
        let s_9_14: Bits = Bits::new(s_9_13 as u128, 128);
        // D s_9_15: sub s_9_12 s_9_14
        let s_9_15: Bits = ((s_9_12) - (s_9_14));
        // D s_9_16: cast zx s_9_8 -> i
        let s_9_16: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_17: cast zx s_9_11 -> i
        let s_9_17: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_18: read-var result:bv
        let s_9_18: Bits = fn_state.result;
        // D s_9_19: call Elem_set(s_9_18, s_9_16, s_9_17, s_9_15)
        let s_9_19: Bits = Elem_set(state, tracer, s_9_18, s_9_16, s_9_17, s_9_15);
        // D s_9_20: write-var result <= s_9_19
        fn_state.result = s_9_19;
        // N s_9_21: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#3871:i64
        let s_10_0: i64 = fn_state.VLshadow_3871;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var da:i64
        let s_10_3: i64 = fn_state.da;
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
