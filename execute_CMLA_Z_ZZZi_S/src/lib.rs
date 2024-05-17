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
pub fn execute_CMLA_Z_ZZZi_S<T: Tracer>(
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
        p: i64,
        VLshadow_3874: i64,
        product_r: i64,
        elt3_r: Bits,
        VLshadow_3875: i64,
        elt3_i: Bits,
        gs_210390: i64,
        operand1: Bits,
        operand2: Bits,
        pairspersegment: i64,
        operand3: Bits,
        product_i: i64,
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
        // D s_0_3: write-var VLshadow#3874 <= s_0_2
        fn_state.VLshadow_3874 = s_0_2;
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
        // D s_1_0: read-var VLshadow#3874:i64
        let s_1_0: i64 = fn_state.VLshadow_3874;
        // D s_1_1: write-var VLshadow#3875 <= s_1_0
        fn_state.VLshadow_3875 = s_1_0;
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
        // D s_1_7: read-var VLshadow#3875:i64
        let s_1_7: i64 = fn_state.VLshadow_3875;
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
        // D s_1_22: read-var VLshadow#3875:i64
        let s_1_22: i64 = fn_state.VLshadow_3875;
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
        // D s_1_30: read-var VLshadow#3875:i64
        let s_1_30: i64 = fn_state.VLshadow_3875;
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
        // D s_1_38: read-var VLshadow#3875:i64
        let s_1_38: i64 = fn_state.VLshadow_3875;
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
        // D s_1_51: write-var gs#210390 <= s_1_50
        fn_state.gs_210390 = s_1_50;
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
        // D s_2_1: read-var gs#210390:i64
        let s_2_1: i64 = fn_state.gs_210390;
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
        // D s_3_33: cast reint s_3_32 -> u32
        let s_3_33: u32 = (s_3_32.value() as u32);
        // D s_3_34: cast zx s_3_33 -> bv
        let s_3_34: Bits = Bits::new(s_3_33 as u128, 32u16);
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
        // D s_3_53: cast reint s_3_52 -> u32
        let s_3_53: u32 = (s_3_52.value() as u32);
        // D s_3_54: cast zx s_3_53 -> bv
        let s_3_54: Bits = Bits::new(s_3_53 as u128, 32u16);
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
        // D s_3_73: cast reint s_3_72 -> u32
        let s_3_73: u32 = (s_3_72.value() as u32);
        // D s_3_74: cast zx s_3_73 -> bv
        let s_3_74: Bits = Bits::new(s_3_73 as u128, 32u16);
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
        // D s_4_9: read-var esize:i64
        let s_4_9: i64 = fn_state.esize;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // D s_4_12: read-var product_r:i64
        let s_4_12: i64 = fn_state.product_r;
        // D s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_14: read-var elt3_r:bv
        let s_4_14: Bits = fn_state.elt3_r;
        // D s_4_15: cast cvt s_4_13 -> bv
        let s_4_15: Bits = Bits::new(s_4_13 as u128, 128);
        // D s_4_16: add s_4_14 s_4_15
        let s_4_16: Bits = (s_4_14 + s_4_15);
        // D s_4_17: cast reint s_4_16 -> u32
        let s_4_17: u32 = (s_4_16.value() as u32);
        // D s_4_18: cast zx s_4_8 -> i
        let s_4_18: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_19: cast zx s_4_11 -> i
        let s_4_19: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_20: cast zx s_4_17 -> bv
        let s_4_20: Bits = Bits::new(s_4_17 as u128, 32u16);
        // D s_4_21: read-var result:bv
        let s_4_21: Bits = fn_state.result;
        // D s_4_22: call Elem_set(s_4_21, s_4_18, s_4_19, s_4_20)
        let s_4_22: Bits = Elem_set(state, tracer, s_4_21, s_4_18, s_4_19, s_4_20);
        // D s_4_23: write-var result <= s_4_22
        fn_state.result = s_4_22;
        // N s_4_24: jump b5
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
        // D s_6_9: read-var esize:i64
        let s_6_9: i64 = fn_state.esize;
        // D s_6_10: cast zx s_6_9 -> i
        let s_6_10: i128 = (i128::try_from(s_6_9).unwrap());
        // D s_6_11: cast reint s_6_10 -> i64
        let s_6_11: i64 = (s_6_10 as i64);
        // D s_6_12: read-var product_i:i64
        let s_6_12: i64 = fn_state.product_i;
        // D s_6_13: cast zx s_6_12 -> i
        let s_6_13: i128 = (i128::try_from(s_6_12).unwrap());
        // D s_6_14: read-var elt3_i:bv
        let s_6_14: Bits = fn_state.elt3_i;
        // D s_6_15: cast cvt s_6_13 -> bv
        let s_6_15: Bits = Bits::new(s_6_13 as u128, 128);
        // D s_6_16: add s_6_14 s_6_15
        let s_6_16: Bits = (s_6_14 + s_6_15);
        // D s_6_17: cast reint s_6_16 -> u32
        let s_6_17: u32 = (s_6_16.value() as u32);
        // D s_6_18: cast zx s_6_8 -> i
        let s_6_18: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_19: cast zx s_6_11 -> i
        let s_6_19: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_20: cast zx s_6_17 -> bv
        let s_6_20: Bits = Bits::new(s_6_17 as u128, 32u16);
        // D s_6_21: read-var result:bv
        let s_6_21: Bits = fn_state.result;
        // D s_6_22: call Elem_set(s_6_21, s_6_18, s_6_19, s_6_20)
        let s_6_22: Bits = Elem_set(state, tracer, s_6_21, s_6_18, s_6_19, s_6_20);
        // D s_6_23: write-var result <= s_6_22
        fn_state.result = s_6_22;
        // N s_6_24: jump b7
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
        // D s_8_9: read-var esize:i64
        let s_8_9: i64 = fn_state.esize;
        // D s_8_10: cast zx s_8_9 -> i
        let s_8_10: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_11: cast reint s_8_10 -> i64
        let s_8_11: i64 = (s_8_10 as i64);
        // D s_8_12: read-var product_i:i64
        let s_8_12: i64 = fn_state.product_i;
        // D s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_14: read-var elt3_i:bv
        let s_8_14: Bits = fn_state.elt3_i;
        // D s_8_15: cast cvt s_8_13 -> bv
        let s_8_15: Bits = Bits::new(s_8_13 as u128, 128);
        // D s_8_16: sub s_8_14 s_8_15
        let s_8_16: Bits = ((s_8_14) - (s_8_15));
        // D s_8_17: cast reint s_8_16 -> u32
        let s_8_17: u32 = (s_8_16.value() as u32);
        // D s_8_18: cast zx s_8_8 -> i
        let s_8_18: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_19: cast zx s_8_11 -> i
        let s_8_19: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_20: cast zx s_8_17 -> bv
        let s_8_20: Bits = Bits::new(s_8_17 as u128, 32u16);
        // D s_8_21: read-var result:bv
        let s_8_21: Bits = fn_state.result;
        // D s_8_22: call Elem_set(s_8_21, s_8_18, s_8_19, s_8_20)
        let s_8_22: Bits = Elem_set(state, tracer, s_8_21, s_8_18, s_8_19, s_8_20);
        // D s_8_23: write-var result <= s_8_22
        fn_state.result = s_8_22;
        // N s_8_24: jump b7
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
        // D s_9_9: read-var esize:i64
        let s_9_9: i64 = fn_state.esize;
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // D s_9_12: read-var product_r:i64
        let s_9_12: i64 = fn_state.product_r;
        // D s_9_13: cast zx s_9_12 -> i
        let s_9_13: i128 = (i128::try_from(s_9_12).unwrap());
        // D s_9_14: read-var elt3_r:bv
        let s_9_14: Bits = fn_state.elt3_r;
        // D s_9_15: cast cvt s_9_13 -> bv
        let s_9_15: Bits = Bits::new(s_9_13 as u128, 128);
        // D s_9_16: sub s_9_14 s_9_15
        let s_9_16: Bits = ((s_9_14) - (s_9_15));
        // D s_9_17: cast reint s_9_16 -> u32
        let s_9_17: u32 = (s_9_16.value() as u32);
        // D s_9_18: cast zx s_9_8 -> i
        let s_9_18: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_19: cast zx s_9_11 -> i
        let s_9_19: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_20: cast zx s_9_17 -> bv
        let s_9_20: Bits = Bits::new(s_9_17 as u128, 32u16);
        // D s_9_21: read-var result:bv
        let s_9_21: Bits = fn_state.result;
        // D s_9_22: call Elem_set(s_9_21, s_9_18, s_9_19, s_9_20)
        let s_9_22: Bits = Elem_set(state, tracer, s_9_21, s_9_18, s_9_19, s_9_20);
        // D s_9_23: write-var result <= s_9_22
        fn_state.result = s_9_22;
        // N s_9_24: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#3875:i64
        let s_10_0: i64 = fn_state.VLshadow_3875;
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