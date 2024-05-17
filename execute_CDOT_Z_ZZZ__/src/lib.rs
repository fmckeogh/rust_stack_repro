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
pub fn execute_CDOT_Z_ZZZ__<T: Tracer>(
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
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_216611: i64,
        operand3: Bits,
        elt2_b: i64,
        VLshadow_4134: i64,
        res: Bits,
        elt1_i: i64,
        elt2_a: i64,
        elt1_r: i64,
        VLshadow_4135: i64,
        result: Bits,
        i: i64,
        operand1: Bits,
        esizeshadow_4133: i64,
        operand2: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        m: i64,
        n: i64,
        sel_a: i64,
        sel_b: i64,
        sub_i: bool,
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
        // D s_0_3: write-var esizeshadow#4133 <= s_0_2
        fn_state.esizeshadow_4133 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4134 <= s_0_6
        fn_state.VLshadow_4134 = s_0_6;
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
        // D s_1_0: read-var VLshadow#4134:i64
        let s_1_0: i64 = fn_state.VLshadow_4134;
        // D s_1_1: write-var VLshadow#4135 <= s_1_0
        fn_state.VLshadow_4135 = s_1_0;
        // D s_1_2: read-var VLshadow#4135:i64
        let s_1_2: i64 = fn_state.VLshadow_4135;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#4133:i64
        let s_1_4: i64 = fn_state.esizeshadow_4133;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#4135:i64
        let s_1_8: i64 = fn_state.VLshadow_4135;
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
        // D s_1_16: read-var VLshadow#4135:i64
        let s_1_16: i64 = fn_state.VLshadow_4135;
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
        // D s_1_24: read-var VLshadow#4135:i64
        let s_1_24: i64 = fn_state.VLshadow_4135;
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
        // D s_1_31: write-var operand3 <= s_1_30
        fn_state.operand3 = s_1_30;
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
        // D s_1_37: write-var gs#216611 <= s_1_36
        fn_state.gs_216611 = s_1_36;
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
        // D s_2_1: read-var gs#216611:i64
        let s_2_1: i64 = fn_state.gs_216611;
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
        // D s_3_0: read-var esizeshadow#4133:i64
        let s_3_0: i64 = fn_state.esizeshadow_4133;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand3:bv
        let s_3_6: Bits = fn_state.operand3;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: write-var res <= s_3_7
        fn_state.res = s_3_7;
        // C s_3_9: const #0s : i64
        let s_3_9: i64 = 0;
        // D s_3_10: write-var i <= s_3_9
        fn_state.i = s_3_9;
        // N s_3_11: jump b4
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
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
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
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // C s_5_5: const #2s : i
        let s_5_5: i128 = 2;
        // D s_5_6: read-var i:i64
        let s_5_6: i64 = fn_state.i;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: mul s_5_5 s_5_7
        let s_5_8: i128 = ((s_5_5) * (s_5_7));
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: cast zx s_5_4 -> i
        let s_5_10: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_11: cast zx s_5_9 -> i
        let s_5_11: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_12: add s_5_10 s_5_11
        let s_5_12: i128 = (s_5_10 + s_5_11);
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // C s_5_14: const #0s : i
        let s_5_14: i128 = 0;
        // D s_5_15: cast zx s_5_13 -> i
        let s_5_15: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_16: add s_5_15 s_5_14
        let s_5_16: i128 = (s_5_15 + s_5_14);
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // C s_5_18: const #4s : i
        let s_5_18: i128 = 4;
        // D s_5_19: read-var esizeshadow#4133:i64
        let s_5_19: i64 = fn_state.esizeshadow_4133;
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: div s_5_20 s_5_18
        let s_5_21: i128 = ((s_5_20) / (s_5_18));
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_24: cast reint s_5_23 -> i64
        let s_5_24: i64 = (s_5_23 as i64);
        // D s_5_25: cast zx s_5_17 -> i
        let s_5_25: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_26: cast zx s_5_24 -> i
        let s_5_26: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_27: read-var operand1:bv
        let s_5_27: Bits = fn_state.operand1;
        // D s_5_28: call Elem_read(s_5_27, s_5_25, s_5_26)
        let s_5_28: Bits = Elem_read(state, tracer, s_5_27, s_5_25, s_5_26);
        // D s_5_29: cast sx s_5_28 -> i
        let s_5_29: i128 = {
            let sign_bit = s_5_28.length() - 1;
            let mut result = s_5_28.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_30: cast reint s_5_29 -> i64
        let s_5_30: i64 = (s_5_29 as i64);
        // D s_5_31: write-var elt1_r <= s_5_30
        fn_state.elt1_r = s_5_30;
        // C s_5_32: const #4s : i
        let s_5_32: i128 = 4;
        // D s_5_33: read-var e:i64
        let s_5_33: i64 = fn_state.e;
        // D s_5_34: cast zx s_5_33 -> i
        let s_5_34: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_35: mul s_5_32 s_5_34
        let s_5_35: i128 = ((s_5_32) * (s_5_34));
        // D s_5_36: cast reint s_5_35 -> i64
        let s_5_36: i64 = (s_5_35 as i64);
        // C s_5_37: const #2s : i
        let s_5_37: i128 = 2;
        // D s_5_38: read-var i:i64
        let s_5_38: i64 = fn_state.i;
        // D s_5_39: cast zx s_5_38 -> i
        let s_5_39: i128 = (i128::try_from(s_5_38).unwrap());
        // D s_5_40: mul s_5_37 s_5_39
        let s_5_40: i128 = ((s_5_37) * (s_5_39));
        // D s_5_41: cast reint s_5_40 -> i64
        let s_5_41: i64 = (s_5_40 as i64);
        // D s_5_42: cast zx s_5_36 -> i
        let s_5_42: i128 = (i128::try_from(s_5_36).unwrap());
        // D s_5_43: cast zx s_5_41 -> i
        let s_5_43: i128 = (i128::try_from(s_5_41).unwrap());
        // D s_5_44: add s_5_42 s_5_43
        let s_5_44: i128 = (s_5_42 + s_5_43);
        // D s_5_45: cast reint s_5_44 -> i64
        let s_5_45: i64 = (s_5_44 as i64);
        // C s_5_46: const #1s : i
        let s_5_46: i128 = 1;
        // D s_5_47: cast zx s_5_45 -> i
        let s_5_47: i128 = (i128::try_from(s_5_45).unwrap());
        // D s_5_48: add s_5_47 s_5_46
        let s_5_48: i128 = (s_5_47 + s_5_46);
        // D s_5_49: cast reint s_5_48 -> i64
        let s_5_49: i64 = (s_5_48 as i64);
        // C s_5_50: const #4s : i
        let s_5_50: i128 = 4;
        // D s_5_51: read-var esizeshadow#4133:i64
        let s_5_51: i64 = fn_state.esizeshadow_4133;
        // D s_5_52: cast zx s_5_51 -> i
        let s_5_52: i128 = (i128::try_from(s_5_51).unwrap());
        // D s_5_53: div s_5_52 s_5_50
        let s_5_53: i128 = ((s_5_52) / (s_5_50));
        // D s_5_54: cast reint s_5_53 -> i64
        let s_5_54: i64 = (s_5_53 as i64);
        // D s_5_55: cast zx s_5_54 -> i
        let s_5_55: i128 = (i128::try_from(s_5_54).unwrap());
        // D s_5_56: cast reint s_5_55 -> i64
        let s_5_56: i64 = (s_5_55 as i64);
        // D s_5_57: cast zx s_5_49 -> i
        let s_5_57: i128 = (i128::try_from(s_5_49).unwrap());
        // D s_5_58: cast zx s_5_56 -> i
        let s_5_58: i128 = (i128::try_from(s_5_56).unwrap());
        // D s_5_59: read-var operand1:bv
        let s_5_59: Bits = fn_state.operand1;
        // D s_5_60: call Elem_read(s_5_59, s_5_57, s_5_58)
        let s_5_60: Bits = Elem_read(state, tracer, s_5_59, s_5_57, s_5_58);
        // D s_5_61: cast sx s_5_60 -> i
        let s_5_61: i128 = {
            let sign_bit = s_5_60.length() - 1;
            let mut result = s_5_60.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_62: cast reint s_5_61 -> i64
        let s_5_62: i64 = (s_5_61 as i64);
        // D s_5_63: write-var elt1_i <= s_5_62
        fn_state.elt1_i = s_5_62;
        // C s_5_64: const #4s : i
        let s_5_64: i128 = 4;
        // D s_5_65: read-var e:i64
        let s_5_65: i64 = fn_state.e;
        // D s_5_66: cast zx s_5_65 -> i
        let s_5_66: i128 = (i128::try_from(s_5_65).unwrap());
        // D s_5_67: mul s_5_64 s_5_66
        let s_5_67: i128 = ((s_5_64) * (s_5_66));
        // D s_5_68: cast reint s_5_67 -> i64
        let s_5_68: i64 = (s_5_67 as i64);
        // C s_5_69: const #2s : i
        let s_5_69: i128 = 2;
        // D s_5_70: read-var i:i64
        let s_5_70: i64 = fn_state.i;
        // D s_5_71: cast zx s_5_70 -> i
        let s_5_71: i128 = (i128::try_from(s_5_70).unwrap());
        // D s_5_72: mul s_5_69 s_5_71
        let s_5_72: i128 = ((s_5_69) * (s_5_71));
        // D s_5_73: cast reint s_5_72 -> i64
        let s_5_73: i64 = (s_5_72 as i64);
        // D s_5_74: cast zx s_5_68 -> i
        let s_5_74: i128 = (i128::try_from(s_5_68).unwrap());
        // D s_5_75: cast zx s_5_73 -> i
        let s_5_75: i128 = (i128::try_from(s_5_73).unwrap());
        // D s_5_76: add s_5_74 s_5_75
        let s_5_76: i128 = (s_5_74 + s_5_75);
        // D s_5_77: cast reint s_5_76 -> i64
        let s_5_77: i64 = (s_5_76 as i64);
        // D s_5_78: cast zx s_5_77 -> i
        let s_5_78: i128 = (i128::try_from(s_5_77).unwrap());
        // D s_5_79: read-var sel_a:i64
        let s_5_79: i64 = fn_state.sel_a;
        // D s_5_80: cast zx s_5_79 -> i
        let s_5_80: i128 = (i128::try_from(s_5_79).unwrap());
        // D s_5_81: add s_5_78 s_5_80
        let s_5_81: i128 = (s_5_78 + s_5_80);
        // D s_5_82: cast reint s_5_81 -> i64
        let s_5_82: i64 = (s_5_81 as i64);
        // C s_5_83: const #4s : i
        let s_5_83: i128 = 4;
        // D s_5_84: read-var esizeshadow#4133:i64
        let s_5_84: i64 = fn_state.esizeshadow_4133;
        // D s_5_85: cast zx s_5_84 -> i
        let s_5_85: i128 = (i128::try_from(s_5_84).unwrap());
        // D s_5_86: div s_5_85 s_5_83
        let s_5_86: i128 = ((s_5_85) / (s_5_83));
        // D s_5_87: cast reint s_5_86 -> i64
        let s_5_87: i64 = (s_5_86 as i64);
        // D s_5_88: cast zx s_5_87 -> i
        let s_5_88: i128 = (i128::try_from(s_5_87).unwrap());
        // D s_5_89: cast reint s_5_88 -> i64
        let s_5_89: i64 = (s_5_88 as i64);
        // D s_5_90: cast zx s_5_82 -> i
        let s_5_90: i128 = (i128::try_from(s_5_82).unwrap());
        // D s_5_91: cast zx s_5_89 -> i
        let s_5_91: i128 = (i128::try_from(s_5_89).unwrap());
        // D s_5_92: read-var operand2:bv
        let s_5_92: Bits = fn_state.operand2;
        // D s_5_93: call Elem_read(s_5_92, s_5_90, s_5_91)
        let s_5_93: Bits = Elem_read(state, tracer, s_5_92, s_5_90, s_5_91);
        // D s_5_94: cast sx s_5_93 -> i
        let s_5_94: i128 = {
            let sign_bit = s_5_93.length() - 1;
            let mut result = s_5_93.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_95: cast reint s_5_94 -> i64
        let s_5_95: i64 = (s_5_94 as i64);
        // D s_5_96: write-var elt2_a <= s_5_95
        fn_state.elt2_a = s_5_95;
        // C s_5_97: const #4s : i
        let s_5_97: i128 = 4;
        // D s_5_98: read-var e:i64
        let s_5_98: i64 = fn_state.e;
        // D s_5_99: cast zx s_5_98 -> i
        let s_5_99: i128 = (i128::try_from(s_5_98).unwrap());
        // D s_5_100: mul s_5_97 s_5_99
        let s_5_100: i128 = ((s_5_97) * (s_5_99));
        // D s_5_101: cast reint s_5_100 -> i64
        let s_5_101: i64 = (s_5_100 as i64);
        // C s_5_102: const #2s : i
        let s_5_102: i128 = 2;
        // D s_5_103: read-var i:i64
        let s_5_103: i64 = fn_state.i;
        // D s_5_104: cast zx s_5_103 -> i
        let s_5_104: i128 = (i128::try_from(s_5_103).unwrap());
        // D s_5_105: mul s_5_102 s_5_104
        let s_5_105: i128 = ((s_5_102) * (s_5_104));
        // D s_5_106: cast reint s_5_105 -> i64
        let s_5_106: i64 = (s_5_105 as i64);
        // D s_5_107: cast zx s_5_101 -> i
        let s_5_107: i128 = (i128::try_from(s_5_101).unwrap());
        // D s_5_108: cast zx s_5_106 -> i
        let s_5_108: i128 = (i128::try_from(s_5_106).unwrap());
        // D s_5_109: add s_5_107 s_5_108
        let s_5_109: i128 = (s_5_107 + s_5_108);
        // D s_5_110: cast reint s_5_109 -> i64
        let s_5_110: i64 = (s_5_109 as i64);
        // D s_5_111: cast zx s_5_110 -> i
        let s_5_111: i128 = (i128::try_from(s_5_110).unwrap());
        // D s_5_112: read-var sel_b:i64
        let s_5_112: i64 = fn_state.sel_b;
        // D s_5_113: cast zx s_5_112 -> i
        let s_5_113: i128 = (i128::try_from(s_5_112).unwrap());
        // D s_5_114: add s_5_111 s_5_113
        let s_5_114: i128 = (s_5_111 + s_5_113);
        // D s_5_115: cast reint s_5_114 -> i64
        let s_5_115: i64 = (s_5_114 as i64);
        // C s_5_116: const #4s : i
        let s_5_116: i128 = 4;
        // D s_5_117: read-var esizeshadow#4133:i64
        let s_5_117: i64 = fn_state.esizeshadow_4133;
        // D s_5_118: cast zx s_5_117 -> i
        let s_5_118: i128 = (i128::try_from(s_5_117).unwrap());
        // D s_5_119: div s_5_118 s_5_116
        let s_5_119: i128 = ((s_5_118) / (s_5_116));
        // D s_5_120: cast reint s_5_119 -> i64
        let s_5_120: i64 = (s_5_119 as i64);
        // D s_5_121: cast zx s_5_120 -> i
        let s_5_121: i128 = (i128::try_from(s_5_120).unwrap());
        // D s_5_122: cast reint s_5_121 -> i64
        let s_5_122: i64 = (s_5_121 as i64);
        // D s_5_123: cast zx s_5_115 -> i
        let s_5_123: i128 = (i128::try_from(s_5_115).unwrap());
        // D s_5_124: cast zx s_5_122 -> i
        let s_5_124: i128 = (i128::try_from(s_5_122).unwrap());
        // D s_5_125: read-var operand2:bv
        let s_5_125: Bits = fn_state.operand2;
        // D s_5_126: call Elem_read(s_5_125, s_5_123, s_5_124)
        let s_5_126: Bits = Elem_read(state, tracer, s_5_125, s_5_123, s_5_124);
        // D s_5_127: cast sx s_5_126 -> i
        let s_5_127: i128 = {
            let sign_bit = s_5_126.length() - 1;
            let mut result = s_5_126.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_128: cast reint s_5_127 -> i64
        let s_5_128: i64 = (s_5_127 as i64);
        // D s_5_129: write-var elt2_b <= s_5_128
        fn_state.elt2_b = s_5_128;
        // D s_5_130: read-var sub_i:u8
        let s_5_130: bool = fn_state.sub_i;
        // N s_5_131: branch s_5_130 b8 b6
        if s_5_130 {
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
        // D s_6_0: read-var elt1_r:i64
        let s_6_0: i64 = fn_state.elt1_r;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var elt2_a:i64
        let s_6_2: i64 = fn_state.elt2_a;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: mul s_6_1 s_6_3
        let s_6_4: i128 = ((s_6_1) * (s_6_3));
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: read-var res:bv
        let s_6_7: Bits = fn_state.res;
        // D s_6_8: cast cvt s_6_6 -> bv
        let s_6_8: Bits = Bits::new(s_6_6 as u128, 128);
        // D s_6_9: add s_6_7 s_6_8
        let s_6_9: Bits = (s_6_7 + s_6_8);
        // D s_6_10: read-var elt1_i:i64
        let s_6_10: i64 = fn_state.elt1_i;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: read-var elt2_b:i64
        let s_6_12: i64 = fn_state.elt2_b;
        // D s_6_13: cast zx s_6_12 -> i
        let s_6_13: i128 = (i128::try_from(s_6_12).unwrap());
        // D s_6_14: mul s_6_11 s_6_13
        let s_6_14: i128 = ((s_6_11) * (s_6_13));
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (i128::try_from(s_6_15).unwrap());
        // D s_6_17: cast cvt s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 128);
        // D s_6_18: add s_6_9 s_6_17
        let s_6_18: Bits = (s_6_9 + s_6_17);
        // D s_6_19: write-var res <= s_6_18
        fn_state.res = s_6_18;
        // N s_6_20: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var i:i64
        let s_7_0: i64 = fn_state.i;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var i <= s_7_2
        fn_state.i = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var elt1_r:i64
        let s_8_0: i64 = fn_state.elt1_r;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var elt2_a:i64
        let s_8_2: i64 = fn_state.elt2_a;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: mul s_8_1 s_8_3
        let s_8_4: i128 = ((s_8_1) * (s_8_3));
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: read-var res:bv
        let s_8_7: Bits = fn_state.res;
        // D s_8_8: cast cvt s_8_6 -> bv
        let s_8_8: Bits = Bits::new(s_8_6 as u128, 128);
        // D s_8_9: add s_8_7 s_8_8
        let s_8_9: Bits = (s_8_7 + s_8_8);
        // D s_8_10: read-var elt1_i:i64
        let s_8_10: i64 = fn_state.elt1_i;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: read-var elt2_b:i64
        let s_8_12: i64 = fn_state.elt2_b;
        // D s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_14: mul s_8_11 s_8_13
        let s_8_14: i128 = ((s_8_11) * (s_8_13));
        // D s_8_15: cast reint s_8_14 -> i64
        let s_8_15: i64 = (s_8_14 as i64);
        // D s_8_16: cast zx s_8_15 -> i
        let s_8_16: i128 = (i128::try_from(s_8_15).unwrap());
        // D s_8_17: cast cvt s_8_16 -> bv
        let s_8_17: Bits = Bits::new(s_8_16 as u128, 128);
        // D s_8_18: sub s_8_9 s_8_17
        let s_8_18: Bits = ((s_8_9) - (s_8_17));
        // D s_8_19: write-var res <= s_8_18
        fn_state.res = s_8_18;
        // N s_8_20: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esizeshadow#4133:i64
        let s_9_0: i64 = fn_state.esizeshadow_4133;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var e:i64
        let s_9_3: i64 = fn_state.e;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast zx s_9_2 -> i
        let s_9_5: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_6: read-var result:bv
        let s_9_6: Bits = fn_state.result;
        // D s_9_7: read-var res:bv
        let s_9_7: Bits = fn_state.res;
        // D s_9_8: call Elem_set(s_9_6, s_9_4, s_9_5, s_9_7)
        let s_9_8: Bits = Elem_set(state, tracer, s_9_6, s_9_4, s_9_5, s_9_7);
        // D s_9_9: write-var result <= s_9_8
        fn_state.result = s_9_8;
        // D s_9_10: read-var e:i64
        let s_9_10: i64 = fn_state.e;
        // C s_9_11: const #1s : i64
        let s_9_11: i64 = 1;
        // D s_9_12: add s_9_10 s_9_11
        let s_9_12: i64 = (s_9_10 + s_9_11);
        // D s_9_13: write-var e <= s_9_12
        fn_state.e = s_9_12;
        // N s_9_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#4135:i64
        let s_10_0: i64 = fn_state.VLshadow_4135;
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
