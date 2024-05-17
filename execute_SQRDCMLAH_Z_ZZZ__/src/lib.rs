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
pub fn execute_SQRDCMLAH_Z_ZZZ__<T: Tracer>(
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
        VLshadow_3878: i64,
        product_r: i128,
        elt3_r: Bits,
        esizeshadow_3876: i64,
        VLshadow_3877: i64,
        elt3_i: Bits,
        operand1: Bits,
        gs_210484: i64,
        operand2: Bits,
        operand3: Bits,
        product_i: i128,
        res_i: i128,
        res_r: i128,
        result: Bits,
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
        // D s_0_3: write-var esizeshadow#3876 <= s_0_2
        fn_state.esizeshadow_3876 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3877 <= s_0_6
        fn_state.VLshadow_3877 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3877:i64
        let s_1_0: i64 = fn_state.VLshadow_3877;
        // D s_1_1: write-var VLshadow#3878 <= s_1_0
        fn_state.VLshadow_3878 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esizeshadow#3876:i64
        let s_1_3: i64 = fn_state.esizeshadow_3876;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_2 s_1_4
        let s_1_5: i128 = ((s_1_2) * (s_1_4));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#3878:i64
        let s_1_7: i64 = fn_state.VLshadow_3878;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var VLshadow#3878:i64
        let s_1_12: i64 = fn_state.VLshadow_3878;
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
        // D s_1_20: read-var VLshadow#3878:i64
        let s_1_20: i64 = fn_state.VLshadow_3878;
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
        // D s_1_28: read-var VLshadow#3878:i64
        let s_1_28: i64 = fn_state.VLshadow_3878;
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
        // D s_1_41: write-var gs#210484 <= s_1_40
        fn_state.gs_210484 = s_1_40;
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
        // D s_2_1: read-var gs#210484:i64
        let s_2_1: i64 = fn_state.gs_210484;
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
        // D s_3_10: read-var esizeshadow#3876:i64
        let s_3_10: i64 = fn_state.esizeshadow_3876;
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
        // D s_3_29: read-var esizeshadow#3876:i64
        let s_3_29: i64 = fn_state.esizeshadow_3876;
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
        // D s_3_48: read-var esizeshadow#3876:i64
        let s_3_48: i64 = fn_state.esizeshadow_3876;
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
        // D s_3_66: read-var esizeshadow#3876:i64
        let s_3_66: i64 = fn_state.esizeshadow_3876;
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
        // D s_3_83: read-var esizeshadow#3876:i64
        let s_3_83: i64 = fn_state.esizeshadow_3876;
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
        // D s_4_0: read-var elt3_r:bv
        let s_4_0: Bits = fn_state.elt3_r;
        // D s_4_1: cast sx s_4_0 -> i
        let s_4_1: i128 = {
            let sign_bit = s_4_0.length() - 1;
            let mut result = s_4_0.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var esizeshadow#3876:i64
        let s_4_4: i64 = fn_state.esizeshadow_3876;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: lsl s_4_3 s_4_5
        let s_4_6: i128 = s_4_3 << s_4_5;
        // C s_4_7: const #2s : i
        let s_4_7: i128 = 2;
        // D s_4_8: read-var product_r:i
        let s_4_8: i128 = fn_state.product_r;
        // D s_4_9: mul s_4_7 s_4_8
        let s_4_9: i128 = ((s_4_7) * (s_4_8));
        // D s_4_10: add s_4_6 s_4_9
        let s_4_10: i128 = (s_4_6 + s_4_9);
        // D s_4_11: write-var res_r <= s_4_10
        fn_state.res_r = s_4_10;
        // N s_4_12: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // D s_5_1: read-var esizeshadow#3876:i64
        let s_5_1: i64 = fn_state.esizeshadow_3876;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: sub s_5_2 s_5_0
        let s_5_3: i128 = ((s_5_2) - (s_5_0));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // C s_5_5: const #1s : i
        let s_5_5: i128 = 1;
        // D s_5_6: cast zx s_5_4 -> i
        let s_5_6: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_7: lsl s_5_5 s_5_6
        let s_5_7: i128 = s_5_5 << s_5_6;
        // D s_5_8: read-var res_r:i
        let s_5_8: i128 = fn_state.res_r;
        // D s_5_9: add s_5_8 s_5_7
        let s_5_9: i128 = (s_5_8 + s_5_7);
        // D s_5_10: read-var esizeshadow#3876:i64
        let s_5_10: i64 = fn_state.esizeshadow_3876;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: lsr s_5_9 s_5_11
        let s_5_12: i128 = s_5_9 >> s_5_11;
        // D s_5_13: write-var res_r <= s_5_12
        fn_state.res_r = s_5_12;
        // C s_5_14: const #2s : i
        let s_5_14: i128 = 2;
        // D s_5_15: read-var p:i64
        let s_5_15: i64 = fn_state.p;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: mul s_5_14 s_5_16
        let s_5_17: i128 = ((s_5_14) * (s_5_16));
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // C s_5_19: const #0s : i
        let s_5_19: i128 = 0;
        // D s_5_20: cast zx s_5_18 -> i
        let s_5_20: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_21: add s_5_20 s_5_19
        let s_5_21: i128 = (s_5_20 + s_5_19);
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: read-var esizeshadow#3876:i64
        let s_5_23: i64 = fn_state.esizeshadow_3876;
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: cast reint s_5_24 -> i64
        let s_5_25: i64 = (s_5_24 as i64);
        // D s_5_26: read-var esizeshadow#3876:i64
        let s_5_26: i64 = fn_state.esizeshadow_3876;
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // D s_5_29: cast zx s_5_28 -> i
        let s_5_29: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_30: read-var res_r:i
        let s_5_30: i128 = fn_state.res_r;
        // D s_5_31: call SignedSat(s_5_30, s_5_29)
        let s_5_31: Bits = SignedSat(state, tracer, s_5_30, s_5_29);
        // D s_5_32: cast zx s_5_22 -> i
        let s_5_32: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_33: cast zx s_5_25 -> i
        let s_5_33: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_34: read-var result:bv
        let s_5_34: Bits = fn_state.result;
        // D s_5_35: call Elem_set(s_5_34, s_5_32, s_5_33, s_5_31)
        let s_5_35: Bits = Elem_set(state, tracer, s_5_34, s_5_32, s_5_33, s_5_31);
        // D s_5_36: write-var result <= s_5_35
        fn_state.result = s_5_35;
        // D s_5_37: read-var sub_i:u8
        let s_5_37: bool = fn_state.sub_i;
        // N s_5_38: branch s_5_37 b8 b6
        if s_5_37 {
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
        // D s_6_0: read-var elt3_i:bv
        let s_6_0: Bits = fn_state.elt3_i;
        // D s_6_1: cast sx s_6_0 -> i
        let s_6_1: i128 = {
            let sign_bit = s_6_0.length() - 1;
            let mut result = s_6_0.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-var esizeshadow#3876:i64
        let s_6_4: i64 = fn_state.esizeshadow_3876;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: lsl s_6_3 s_6_5
        let s_6_6: i128 = s_6_3 << s_6_5;
        // C s_6_7: const #2s : i
        let s_6_7: i128 = 2;
        // D s_6_8: read-var product_i:i
        let s_6_8: i128 = fn_state.product_i;
        // D s_6_9: mul s_6_7 s_6_8
        let s_6_9: i128 = ((s_6_7) * (s_6_8));
        // D s_6_10: add s_6_6 s_6_9
        let s_6_10: i128 = (s_6_6 + s_6_9);
        // D s_6_11: write-var res_i <= s_6_10
        fn_state.res_i = s_6_10;
        // N s_6_12: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // D s_7_1: read-var esizeshadow#3876:i64
        let s_7_1: i64 = fn_state.esizeshadow_3876;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: sub s_7_2 s_7_0
        let s_7_3: i128 = ((s_7_2) - (s_7_0));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // C s_7_5: const #1s : i
        let s_7_5: i128 = 1;
        // D s_7_6: cast zx s_7_4 -> i
        let s_7_6: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_7: lsl s_7_5 s_7_6
        let s_7_7: i128 = s_7_5 << s_7_6;
        // D s_7_8: read-var res_i:i
        let s_7_8: i128 = fn_state.res_i;
        // D s_7_9: add s_7_8 s_7_7
        let s_7_9: i128 = (s_7_8 + s_7_7);
        // D s_7_10: read-var esizeshadow#3876:i64
        let s_7_10: i64 = fn_state.esizeshadow_3876;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: lsr s_7_9 s_7_11
        let s_7_12: i128 = s_7_9 >> s_7_11;
        // D s_7_13: write-var res_i <= s_7_12
        fn_state.res_i = s_7_12;
        // C s_7_14: const #2s : i
        let s_7_14: i128 = 2;
        // D s_7_15: read-var p:i64
        let s_7_15: i64 = fn_state.p;
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (i128::try_from(s_7_15).unwrap());
        // D s_7_17: mul s_7_14 s_7_16
        let s_7_17: i128 = ((s_7_14) * (s_7_16));
        // D s_7_18: cast reint s_7_17 -> i64
        let s_7_18: i64 = (s_7_17 as i64);
        // C s_7_19: const #1s : i
        let s_7_19: i128 = 1;
        // D s_7_20: cast zx s_7_18 -> i
        let s_7_20: i128 = (i128::try_from(s_7_18).unwrap());
        // D s_7_21: add s_7_20 s_7_19
        let s_7_21: i128 = (s_7_20 + s_7_19);
        // D s_7_22: cast reint s_7_21 -> i64
        let s_7_22: i64 = (s_7_21 as i64);
        // D s_7_23: read-var esizeshadow#3876:i64
        let s_7_23: i64 = fn_state.esizeshadow_3876;
        // D s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // D s_7_25: cast reint s_7_24 -> i64
        let s_7_25: i64 = (s_7_24 as i64);
        // D s_7_26: read-var esizeshadow#3876:i64
        let s_7_26: i64 = fn_state.esizeshadow_3876;
        // D s_7_27: cast zx s_7_26 -> i
        let s_7_27: i128 = (i128::try_from(s_7_26).unwrap());
        // D s_7_28: cast reint s_7_27 -> i64
        let s_7_28: i64 = (s_7_27 as i64);
        // D s_7_29: cast zx s_7_28 -> i
        let s_7_29: i128 = (i128::try_from(s_7_28).unwrap());
        // D s_7_30: read-var res_i:i
        let s_7_30: i128 = fn_state.res_i;
        // D s_7_31: call SignedSat(s_7_30, s_7_29)
        let s_7_31: Bits = SignedSat(state, tracer, s_7_30, s_7_29);
        // D s_7_32: cast zx s_7_22 -> i
        let s_7_32: i128 = (i128::try_from(s_7_22).unwrap());
        // D s_7_33: cast zx s_7_25 -> i
        let s_7_33: i128 = (i128::try_from(s_7_25).unwrap());
        // D s_7_34: read-var result:bv
        let s_7_34: Bits = fn_state.result;
        // D s_7_35: call Elem_set(s_7_34, s_7_32, s_7_33, s_7_31)
        let s_7_35: Bits = Elem_set(state, tracer, s_7_34, s_7_32, s_7_33, s_7_31);
        // D s_7_36: write-var result <= s_7_35
        fn_state.result = s_7_35;
        // D s_7_37: read-var p:i64
        let s_7_37: i64 = fn_state.p;
        // C s_7_38: const #1s : i64
        let s_7_38: i64 = 1;
        // D s_7_39: add s_7_37 s_7_38
        let s_7_39: i64 = (s_7_37 + s_7_38);
        // D s_7_40: write-var p <= s_7_39
        fn_state.p = s_7_39;
        // N s_7_41: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var elt3_i:bv
        let s_8_0: Bits = fn_state.elt3_i;
        // D s_8_1: cast sx s_8_0 -> i
        let s_8_1: i128 = {
            let sign_bit = s_8_0.length() - 1;
            let mut result = s_8_0.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-var esizeshadow#3876:i64
        let s_8_4: i64 = fn_state.esizeshadow_3876;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: lsl s_8_3 s_8_5
        let s_8_6: i128 = s_8_3 << s_8_5;
        // C s_8_7: const #2s : i
        let s_8_7: i128 = 2;
        // D s_8_8: read-var product_i:i
        let s_8_8: i128 = fn_state.product_i;
        // D s_8_9: mul s_8_7 s_8_8
        let s_8_9: i128 = ((s_8_7) * (s_8_8));
        // D s_8_10: sub s_8_6 s_8_9
        let s_8_10: i128 = ((s_8_6) - (s_8_9));
        // D s_8_11: write-var res_i <= s_8_10
        fn_state.res_i = s_8_10;
        // N s_8_12: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var elt3_r:bv
        let s_9_0: Bits = fn_state.elt3_r;
        // D s_9_1: cast sx s_9_0 -> i
        let s_9_1: i128 = {
            let sign_bit = s_9_0.length() - 1;
            let mut result = s_9_0.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var esizeshadow#3876:i64
        let s_9_4: i64 = fn_state.esizeshadow_3876;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: lsl s_9_3 s_9_5
        let s_9_6: i128 = s_9_3 << s_9_5;
        // C s_9_7: const #2s : i
        let s_9_7: i128 = 2;
        // D s_9_8: read-var product_r:i
        let s_9_8: i128 = fn_state.product_r;
        // D s_9_9: mul s_9_7 s_9_8
        let s_9_9: i128 = ((s_9_7) * (s_9_8));
        // D s_9_10: sub s_9_6 s_9_9
        let s_9_10: i128 = ((s_9_6) - (s_9_9));
        // D s_9_11: write-var res_r <= s_9_10
        fn_state.res_r = s_9_10;
        // N s_9_12: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#3878:i64
        let s_10_0: i64 = fn_state.VLshadow_3878;
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
