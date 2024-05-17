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
pub fn execute_SQRDCMLAH_Z_ZZZi_H<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    index: i64,
    m: i64,
    n: i64,
    sel_a: i64,
    sel_b: i64,
    sub_i: bool,
    sub_r: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_210582: i64,
        VLshadow_3880: i64,
        p: i64,
        product_r: i64,
        elt3_r: Bits,
        VLshadow_3879: i64,
        elt3_i: Bits,
        operand1: Bits,
        operand2: Bits,
        pairspersegment: i64,
        operand3: Bits,
        product_i: i64,
        res_i: i128,
        res_r: i128,
        result: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        index: i64,
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
        index,
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
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#3879 <= s_0_2
        fn_state.VLshadow_3879 = s_0_2;
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
        // D s_1_0: read-var VLshadow#3879:i64
        let s_1_0: i64 = fn_state.VLshadow_3879;
        // D s_1_1: write-var VLshadow#3880 <= s_1_0
        fn_state.VLshadow_3880 = s_1_0;
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
        // D s_1_7: read-var VLshadow#3880:i64
        let s_1_7: i64 = fn_state.VLshadow_3880;
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
        // D s_1_21: write-var pairspersegment <= s_1_20
        fn_state.pairspersegment = s_1_20;
        // D s_1_22: read-var VLshadow#3880:i64
        let s_1_22: i64 = fn_state.VLshadow_3880;
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
        // D s_1_30: read-var VLshadow#3880:i64
        let s_1_30: i64 = fn_state.VLshadow_3880;
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
        // D s_1_38: read-var VLshadow#3880:i64
        let s_1_38: i64 = fn_state.VLshadow_3880;
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
        // D s_1_45: write-var operand3 <= s_1_44
        fn_state.operand3 = s_1_44;
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
        // D s_1_51: write-var gs#210582 <= s_1_50
        fn_state.gs_210582 = s_1_50;
        // D s_1_52: write-var p <= s_1_46
        fn_state.p = s_1_46;
        // N s_1_53: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#210582:i64
        let s_2_1: i64 = fn_state.gs_210582;
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
        // D s_3_0: read-var p:i64
        let s_3_0: i64 = fn_state.p;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var pairspersegment:i64
        let s_3_2: i64 = fn_state.pairspersegment;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: mod s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) % (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var p:i64
        let s_3_6: i64 = fn_state.p;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast zx s_3_5 -> i
        let s_3_8: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_9: sub s_3_7 s_3_8
        let s_3_9: i128 = ((s_3_7) - (s_3_8));
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: read-var index:i64
        let s_3_12: i64 = fn_state.index;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: add s_3_11 s_3_13
        let s_3_14: i128 = (s_3_11 + s_3_13);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // C s_3_16: const #2s : i
        let s_3_16: i128 = 2;
        // D s_3_17: read-var p:i64
        let s_3_17: i64 = fn_state.p;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: mul s_3_16 s_3_18
        let s_3_19: i128 = ((s_3_16) * (s_3_18));
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: read-var sel_a:i64
        let s_3_22: i64 = fn_state.sel_a;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: add s_3_21 s_3_23
        let s_3_24: i128 = (s_3_21 + s_3_23);
        // D s_3_25: cast reint s_3_24 -> i64
        let s_3_25: i64 = (s_3_24 as i64);
        // D s_3_26: read-var esize:i64
        let s_3_26: i64 = fn_state.esize;
        // D s_3_27: cast zx s_3_26 -> i
        let s_3_27: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_28: cast reint s_3_27 -> i64
        let s_3_28: i64 = (s_3_27 as i64);
        // D s_3_29: cast zx s_3_25 -> i
        let s_3_29: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_30: cast zx s_3_28 -> i
        let s_3_30: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_31: read-var operand1:bv
        let s_3_31: Bits = fn_state.operand1;
        // D s_3_32: call Elem_read(s_3_31, s_3_29, s_3_30)
        let s_3_32: Bits = Elem_read(state, tracer, s_3_31, s_3_29, s_3_30);
        // D s_3_33: cast reint s_3_32 -> u16
        let s_3_33: u16 = (s_3_32.value() as u16);
        // D s_3_34: cast zx s_3_33 -> bv
        let s_3_34: Bits = Bits::new(s_3_33 as u128, 16u16);
        // D s_3_35: cast sx s_3_34 -> i
        let s_3_35: i128 = {
            let sign_bit = s_3_34.length() - 1;
            let mut result = s_3_34.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_36: cast reint s_3_35 -> i64
        let s_3_36: i64 = (s_3_35 as i64);
        // C s_3_37: const #2s : i
        let s_3_37: i128 = 2;
        // D s_3_38: cast zx s_3_15 -> i
        let s_3_38: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_39: mul s_3_37 s_3_38
        let s_3_39: i128 = ((s_3_37) * (s_3_38));
        // D s_3_40: cast reint s_3_39 -> i64
        let s_3_40: i64 = (s_3_39 as i64);
        // D s_3_41: cast zx s_3_40 -> i
        let s_3_41: i128 = (i128::try_from(s_3_40).unwrap());
        // D s_3_42: read-var sel_a:i64
        let s_3_42: i64 = fn_state.sel_a;
        // D s_3_43: cast zx s_3_42 -> i
        let s_3_43: i128 = (i128::try_from(s_3_42).unwrap());
        // D s_3_44: add s_3_41 s_3_43
        let s_3_44: i128 = (s_3_41 + s_3_43);
        // D s_3_45: cast reint s_3_44 -> i64
        let s_3_45: i64 = (s_3_44 as i64);
        // D s_3_46: read-var esize:i64
        let s_3_46: i64 = fn_state.esize;
        // D s_3_47: cast zx s_3_46 -> i
        let s_3_47: i128 = (i128::try_from(s_3_46).unwrap());
        // D s_3_48: cast reint s_3_47 -> i64
        let s_3_48: i64 = (s_3_47 as i64);
        // D s_3_49: cast zx s_3_45 -> i
        let s_3_49: i128 = (i128::try_from(s_3_45).unwrap());
        // D s_3_50: cast zx s_3_48 -> i
        let s_3_50: i128 = (i128::try_from(s_3_48).unwrap());
        // D s_3_51: read-var operand2:bv
        let s_3_51: Bits = fn_state.operand2;
        // D s_3_52: call Elem_read(s_3_51, s_3_49, s_3_50)
        let s_3_52: Bits = Elem_read(state, tracer, s_3_51, s_3_49, s_3_50);
        // D s_3_53: cast reint s_3_52 -> u16
        let s_3_53: u16 = (s_3_52.value() as u16);
        // D s_3_54: cast zx s_3_53 -> bv
        let s_3_54: Bits = Bits::new(s_3_53 as u128, 16u16);
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
        // D s_3_58: cast zx s_3_15 -> i
        let s_3_58: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_59: mul s_3_57 s_3_58
        let s_3_59: i128 = ((s_3_57) * (s_3_58));
        // D s_3_60: cast reint s_3_59 -> i64
        let s_3_60: i64 = (s_3_59 as i64);
        // D s_3_61: cast zx s_3_60 -> i
        let s_3_61: i128 = (i128::try_from(s_3_60).unwrap());
        // D s_3_62: read-var sel_b:i64
        let s_3_62: i64 = fn_state.sel_b;
        // D s_3_63: cast zx s_3_62 -> i
        let s_3_63: i128 = (i128::try_from(s_3_62).unwrap());
        // D s_3_64: add s_3_61 s_3_63
        let s_3_64: i128 = (s_3_61 + s_3_63);
        // D s_3_65: cast reint s_3_64 -> i64
        let s_3_65: i64 = (s_3_64 as i64);
        // D s_3_66: read-var esize:i64
        let s_3_66: i64 = fn_state.esize;
        // D s_3_67: cast zx s_3_66 -> i
        let s_3_67: i128 = (i128::try_from(s_3_66).unwrap());
        // D s_3_68: cast reint s_3_67 -> i64
        let s_3_68: i64 = (s_3_67 as i64);
        // D s_3_69: cast zx s_3_65 -> i
        let s_3_69: i128 = (i128::try_from(s_3_65).unwrap());
        // D s_3_70: cast zx s_3_68 -> i
        let s_3_70: i128 = (i128::try_from(s_3_68).unwrap());
        // D s_3_71: read-var operand2:bv
        let s_3_71: Bits = fn_state.operand2;
        // D s_3_72: call Elem_read(s_3_71, s_3_69, s_3_70)
        let s_3_72: Bits = Elem_read(state, tracer, s_3_71, s_3_69, s_3_70);
        // D s_3_73: cast reint s_3_72 -> u16
        let s_3_73: u16 = (s_3_72.value() as u16);
        // D s_3_74: cast zx s_3_73 -> bv
        let s_3_74: Bits = Bits::new(s_3_73 as u128, 16u16);
        // D s_3_75: cast sx s_3_74 -> i
        let s_3_75: i128 = {
            let sign_bit = s_3_74.length() - 1;
            let mut result = s_3_74.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_3_76: cast reint s_3_75 -> i64
        let s_3_76: i64 = (s_3_75 as i64);
        // C s_3_77: const #2s : i
        let s_3_77: i128 = 2;
        // D s_3_78: read-var p:i64
        let s_3_78: i64 = fn_state.p;
        // D s_3_79: cast zx s_3_78 -> i
        let s_3_79: i128 = (i128::try_from(s_3_78).unwrap());
        // D s_3_80: mul s_3_77 s_3_79
        let s_3_80: i128 = ((s_3_77) * (s_3_79));
        // D s_3_81: cast reint s_3_80 -> i64
        let s_3_81: i64 = (s_3_80 as i64);
        // C s_3_82: const #0s : i
        let s_3_82: i128 = 0;
        // D s_3_83: cast zx s_3_81 -> i
        let s_3_83: i128 = (i128::try_from(s_3_81).unwrap());
        // D s_3_84: add s_3_83 s_3_82
        let s_3_84: i128 = (s_3_83 + s_3_82);
        // D s_3_85: cast reint s_3_84 -> i64
        let s_3_85: i64 = (s_3_84 as i64);
        // D s_3_86: read-var esize:i64
        let s_3_86: i64 = fn_state.esize;
        // D s_3_87: cast zx s_3_86 -> i
        let s_3_87: i128 = (i128::try_from(s_3_86).unwrap());
        // D s_3_88: cast reint s_3_87 -> i64
        let s_3_88: i64 = (s_3_87 as i64);
        // D s_3_89: cast zx s_3_85 -> i
        let s_3_89: i128 = (i128::try_from(s_3_85).unwrap());
        // D s_3_90: cast zx s_3_88 -> i
        let s_3_90: i128 = (i128::try_from(s_3_88).unwrap());
        // D s_3_91: read-var operand3:bv
        let s_3_91: Bits = fn_state.operand3;
        // D s_3_92: call Elem_read(s_3_91, s_3_89, s_3_90)
        let s_3_92: Bits = Elem_read(state, tracer, s_3_91, s_3_89, s_3_90);
        // D s_3_93: write-var elt3_r <= s_3_92
        fn_state.elt3_r = s_3_92;
        // C s_3_94: const #2s : i
        let s_3_94: i128 = 2;
        // D s_3_95: read-var p:i64
        let s_3_95: i64 = fn_state.p;
        // D s_3_96: cast zx s_3_95 -> i
        let s_3_96: i128 = (i128::try_from(s_3_95).unwrap());
        // D s_3_97: mul s_3_94 s_3_96
        let s_3_97: i128 = ((s_3_94) * (s_3_96));
        // D s_3_98: cast reint s_3_97 -> i64
        let s_3_98: i64 = (s_3_97 as i64);
        // C s_3_99: const #1s : i
        let s_3_99: i128 = 1;
        // D s_3_100: cast zx s_3_98 -> i
        let s_3_100: i128 = (i128::try_from(s_3_98).unwrap());
        // D s_3_101: add s_3_100 s_3_99
        let s_3_101: i128 = (s_3_100 + s_3_99);
        // D s_3_102: cast reint s_3_101 -> i64
        let s_3_102: i64 = (s_3_101 as i64);
        // D s_3_103: read-var esize:i64
        let s_3_103: i64 = fn_state.esize;
        // D s_3_104: cast zx s_3_103 -> i
        let s_3_104: i128 = (i128::try_from(s_3_103).unwrap());
        // D s_3_105: cast reint s_3_104 -> i64
        let s_3_105: i64 = (s_3_104 as i64);
        // D s_3_106: cast zx s_3_102 -> i
        let s_3_106: i128 = (i128::try_from(s_3_102).unwrap());
        // D s_3_107: cast zx s_3_105 -> i
        let s_3_107: i128 = (i128::try_from(s_3_105).unwrap());
        // D s_3_108: read-var operand3:bv
        let s_3_108: Bits = fn_state.operand3;
        // D s_3_109: call Elem_read(s_3_108, s_3_106, s_3_107)
        let s_3_109: Bits = Elem_read(state, tracer, s_3_108, s_3_106, s_3_107);
        // D s_3_110: write-var elt3_i <= s_3_109
        fn_state.elt3_i = s_3_109;
        // D s_3_111: cast zx s_3_36 -> i
        let s_3_111: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_112: cast zx s_3_56 -> i
        let s_3_112: i128 = (i128::try_from(s_3_56).unwrap());
        // D s_3_113: mul s_3_111 s_3_112
        let s_3_113: i128 = ((s_3_111) * (s_3_112));
        // D s_3_114: cast reint s_3_113 -> i64
        let s_3_114: i64 = (s_3_113 as i64);
        // D s_3_115: write-var product_r <= s_3_114
        fn_state.product_r = s_3_114;
        // D s_3_116: cast zx s_3_36 -> i
        let s_3_116: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_117: cast zx s_3_76 -> i
        let s_3_117: i128 = (i128::try_from(s_3_76).unwrap());
        // D s_3_118: mul s_3_116 s_3_117
        let s_3_118: i128 = ((s_3_116) * (s_3_117));
        // D s_3_119: cast reint s_3_118 -> i64
        let s_3_119: i64 = (s_3_118 as i64);
        // D s_3_120: write-var product_i <= s_3_119
        fn_state.product_i = s_3_119;
        // D s_3_121: read-var sub_r:u8
        let s_3_121: bool = fn_state.sub_r;
        // N s_3_122: branch s_3_121 b9 b4
        if s_3_121 {
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
        // D s_4_4: read-var esize:i64
        let s_4_4: i64 = fn_state.esize;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: lsl s_4_3 s_4_5
        let s_4_6: i128 = s_4_3 << s_4_5;
        // C s_4_7: const #2s : i
        let s_4_7: i128 = 2;
        // D s_4_8: read-var product_r:i64
        let s_4_8: i64 = fn_state.product_r;
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_10: mul s_4_7 s_4_9
        let s_4_10: i128 = ((s_4_7) * (s_4_9));
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: add s_4_6 s_4_12
        let s_4_13: i128 = (s_4_6 + s_4_12);
        // D s_4_14: write-var res_r <= s_4_13
        fn_state.res_r = s_4_13;
        // N s_4_15: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // D s_5_1: read-var esize:i64
        let s_5_1: i64 = fn_state.esize;
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
        // D s_5_10: read-var esize:i64
        let s_5_10: i64 = fn_state.esize;
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
        // D s_5_23: read-var esize:i64
        let s_5_23: i64 = fn_state.esize;
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: cast reint s_5_24 -> i64
        let s_5_25: i64 = (s_5_24 as i64);
        // D s_5_26: read-var esize:i64
        let s_5_26: i64 = fn_state.esize;
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
        // D s_5_32: cast reint s_5_31 -> u16
        let s_5_32: u16 = (s_5_31.value() as u16);
        // D s_5_33: cast zx s_5_22 -> i
        let s_5_33: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_34: cast zx s_5_25 -> i
        let s_5_34: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_35: cast zx s_5_32 -> bv
        let s_5_35: Bits = Bits::new(s_5_32 as u128, 16u16);
        // D s_5_36: read-var result:bv
        let s_5_36: Bits = fn_state.result;
        // D s_5_37: call Elem_set(s_5_36, s_5_33, s_5_34, s_5_35)
        let s_5_37: Bits = Elem_set(state, tracer, s_5_36, s_5_33, s_5_34, s_5_35);
        // D s_5_38: write-var result <= s_5_37
        fn_state.result = s_5_37;
        // D s_5_39: read-var sub_i:u8
        let s_5_39: bool = fn_state.sub_i;
        // N s_5_40: branch s_5_39 b8 b6
        if s_5_39 {
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
        // D s_6_4: read-var esize:i64
        let s_6_4: i64 = fn_state.esize;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: lsl s_6_3 s_6_5
        let s_6_6: i128 = s_6_3 << s_6_5;
        // C s_6_7: const #2s : i
        let s_6_7: i128 = 2;
        // D s_6_8: read-var product_i:i64
        let s_6_8: i64 = fn_state.product_i;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: mul s_6_7 s_6_9
        let s_6_10: i128 = ((s_6_7) * (s_6_9));
        // D s_6_11: cast reint s_6_10 -> i64
        let s_6_11: i64 = (s_6_10 as i64);
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_13: add s_6_6 s_6_12
        let s_6_13: i128 = (s_6_6 + s_6_12);
        // D s_6_14: write-var res_i <= s_6_13
        fn_state.res_i = s_6_13;
        // N s_6_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // D s_7_1: read-var esize:i64
        let s_7_1: i64 = fn_state.esize;
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
        // D s_7_10: read-var esize:i64
        let s_7_10: i64 = fn_state.esize;
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
        // D s_7_23: read-var esize:i64
        let s_7_23: i64 = fn_state.esize;
        // D s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // D s_7_25: cast reint s_7_24 -> i64
        let s_7_25: i64 = (s_7_24 as i64);
        // D s_7_26: read-var esize:i64
        let s_7_26: i64 = fn_state.esize;
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
        // D s_7_32: cast reint s_7_31 -> u16
        let s_7_32: u16 = (s_7_31.value() as u16);
        // D s_7_33: cast zx s_7_22 -> i
        let s_7_33: i128 = (i128::try_from(s_7_22).unwrap());
        // D s_7_34: cast zx s_7_25 -> i
        let s_7_34: i128 = (i128::try_from(s_7_25).unwrap());
        // D s_7_35: cast zx s_7_32 -> bv
        let s_7_35: Bits = Bits::new(s_7_32 as u128, 16u16);
        // D s_7_36: read-var result:bv
        let s_7_36: Bits = fn_state.result;
        // D s_7_37: call Elem_set(s_7_36, s_7_33, s_7_34, s_7_35)
        let s_7_37: Bits = Elem_set(state, tracer, s_7_36, s_7_33, s_7_34, s_7_35);
        // D s_7_38: write-var result <= s_7_37
        fn_state.result = s_7_37;
        // D s_7_39: read-var p:i64
        let s_7_39: i64 = fn_state.p;
        // C s_7_40: const #1s : i64
        let s_7_40: i64 = 1;
        // D s_7_41: add s_7_39 s_7_40
        let s_7_41: i64 = (s_7_39 + s_7_40);
        // D s_7_42: write-var p <= s_7_41
        fn_state.p = s_7_41;
        // N s_7_43: jump b2
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
        // D s_8_4: read-var esize:i64
        let s_8_4: i64 = fn_state.esize;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: lsl s_8_3 s_8_5
        let s_8_6: i128 = s_8_3 << s_8_5;
        // C s_8_7: const #2s : i
        let s_8_7: i128 = 2;
        // D s_8_8: read-var product_i:i64
        let s_8_8: i64 = fn_state.product_i;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: mul s_8_7 s_8_9
        let s_8_10: i128 = ((s_8_7) * (s_8_9));
        // D s_8_11: cast reint s_8_10 -> i64
        let s_8_11: i64 = (s_8_10 as i64);
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: sub s_8_6 s_8_12
        let s_8_13: i128 = ((s_8_6) - (s_8_12));
        // D s_8_14: write-var res_i <= s_8_13
        fn_state.res_i = s_8_13;
        // N s_8_15: jump b7
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
        // D s_9_4: read-var esize:i64
        let s_9_4: i64 = fn_state.esize;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: lsl s_9_3 s_9_5
        let s_9_6: i128 = s_9_3 << s_9_5;
        // C s_9_7: const #2s : i
        let s_9_7: i128 = 2;
        // D s_9_8: read-var product_r:i64
        let s_9_8: i64 = fn_state.product_r;
        // D s_9_9: cast zx s_9_8 -> i
        let s_9_9: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_10: mul s_9_7 s_9_9
        let s_9_10: i128 = ((s_9_7) * (s_9_9));
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // D s_9_12: cast zx s_9_11 -> i
        let s_9_12: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_13: sub s_9_6 s_9_12
        let s_9_13: i128 = ((s_9_6) - (s_9_12));
        // D s_9_14: write-var res_r <= s_9_13
        fn_state.res_r = s_9_13;
        // N s_9_15: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#3880:i64
        let s_10_0: i64 = fn_state.VLshadow_3880;
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
