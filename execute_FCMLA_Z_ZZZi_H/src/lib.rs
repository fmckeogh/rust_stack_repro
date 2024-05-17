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
use CheckSVEEnabled::*;
use FPCR_read::*;
use FPNeg::*;
use Elem_read::*;
use FPMulAdd::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_FCMLA_Z_ZZZi_H<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    index: i64,
    m: i64,
    n: i64,
    neg_i: bool,
    neg_r: bool,
    sel_a: i64,
    sel_b: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        pairspersegment: i64,
        p: i64,
        operand3: Bits,
        addend_i: Bits,
        elt2_b: Bits,
        elt2_a: Bits,
        gs_181080: i64,
        VLshadow_2301: i64,
        VLshadow_2302: i64,
        elt1_a: Bits,
        result: Bits,
        operand1: Bits,
        addend_r: Bits,
        operand2: Bits,
        VL: i64,
        da: i64,
        esize: i64,
        index: i64,
        m: i64,
        n: i64,
        neg_i: bool,
        neg_r: bool,
        sel_a: i64,
        sel_b: i64,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        index,
        m,
        n,
        neg_i,
        neg_r,
        sel_a,
        sel_b,
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
        // D s_0_3: write-var VLshadow#2301 <= s_0_2
        fn_state.VLshadow_2301 = s_0_2;
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
        // D s_1_0: read-var VLshadow#2301:i64
        let s_1_0: i64 = fn_state.VLshadow_2301;
        // D s_1_1: write-var VLshadow#2302 <= s_1_0
        fn_state.VLshadow_2302 = s_1_0;
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
        // D s_1_7: read-var VLshadow#2302:i64
        let s_1_7: i64 = fn_state.VLshadow_2302;
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
        // D s_1_22: read-var VLshadow#2302:i64
        let s_1_22: i64 = fn_state.VLshadow_2302;
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
        // D s_1_30: read-var VLshadow#2302:i64
        let s_1_30: i64 = fn_state.VLshadow_2302;
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
        // D s_1_38: read-var VLshadow#2302:i64
        let s_1_38: i64 = fn_state.VLshadow_2302;
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
        // D s_1_51: write-var gs#181080 <= s_1_50
        fn_state.gs_181080 = s_1_50;
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
        // D s_2_1: read-var gs#181080:i64
        let s_2_1: i64 = fn_state.gs_181080;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b14 b3
        if s_2_2 {
            return block_14(state, tracer, fn_state);
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
        // D s_3_10: read-var index:i64
        let s_3_10: i64 = fn_state.index;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: add s_3_9 s_3_11
        let s_3_12: i128 = (s_3_9 + s_3_11);
        // C s_3_13: const #2s : i
        let s_3_13: i128 = 2;
        // D s_3_14: read-var p:i64
        let s_3_14: i64 = fn_state.p;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: mul s_3_13 s_3_15
        let s_3_16: i128 = ((s_3_13) * (s_3_15));
        // D s_3_17: cast reint s_3_16 -> i64
        let s_3_17: i64 = (s_3_16 as i64);
        // C s_3_18: const #0s : i
        let s_3_18: i128 = 0;
        // D s_3_19: cast zx s_3_17 -> i
        let s_3_19: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_20: add s_3_19 s_3_18
        let s_3_20: i128 = (s_3_19 + s_3_18);
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: read-var esize:i64
        let s_3_22: i64 = fn_state.esize;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: cast zx s_3_21 -> i
        let s_3_25: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_26: cast zx s_3_24 -> i
        let s_3_26: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_27: read-var operand3:bv
        let s_3_27: Bits = fn_state.operand3;
        // D s_3_28: call Elem_read(s_3_27, s_3_25, s_3_26)
        let s_3_28: Bits = Elem_read(state, tracer, s_3_27, s_3_25, s_3_26);
        // D s_3_29: write-var addend_r <= s_3_28
        fn_state.addend_r = s_3_28;
        // C s_3_30: const #2s : i
        let s_3_30: i128 = 2;
        // D s_3_31: read-var p:i64
        let s_3_31: i64 = fn_state.p;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: mul s_3_30 s_3_32
        let s_3_33: i128 = ((s_3_30) * (s_3_32));
        // D s_3_34: cast reint s_3_33 -> i64
        let s_3_34: i64 = (s_3_33 as i64);
        // C s_3_35: const #1s : i
        let s_3_35: i128 = 1;
        // D s_3_36: cast zx s_3_34 -> i
        let s_3_36: i128 = (i128::try_from(s_3_34).unwrap());
        // D s_3_37: add s_3_36 s_3_35
        let s_3_37: i128 = (s_3_36 + s_3_35);
        // D s_3_38: cast reint s_3_37 -> i64
        let s_3_38: i64 = (s_3_37 as i64);
        // D s_3_39: read-var esize:i64
        let s_3_39: i64 = fn_state.esize;
        // D s_3_40: cast zx s_3_39 -> i
        let s_3_40: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_41: cast reint s_3_40 -> i64
        let s_3_41: i64 = (s_3_40 as i64);
        // D s_3_42: cast zx s_3_38 -> i
        let s_3_42: i128 = (i128::try_from(s_3_38).unwrap());
        // D s_3_43: cast zx s_3_41 -> i
        let s_3_43: i128 = (i128::try_from(s_3_41).unwrap());
        // D s_3_44: read-var operand3:bv
        let s_3_44: Bits = fn_state.operand3;
        // D s_3_45: call Elem_read(s_3_44, s_3_42, s_3_43)
        let s_3_45: Bits = Elem_read(state, tracer, s_3_44, s_3_42, s_3_43);
        // D s_3_46: write-var addend_i <= s_3_45
        fn_state.addend_i = s_3_45;
        // C s_3_47: const #2s : i
        let s_3_47: i128 = 2;
        // D s_3_48: read-var p:i64
        let s_3_48: i64 = fn_state.p;
        // D s_3_49: cast zx s_3_48 -> i
        let s_3_49: i128 = (i128::try_from(s_3_48).unwrap());
        // D s_3_50: mul s_3_47 s_3_49
        let s_3_50: i128 = ((s_3_47) * (s_3_49));
        // D s_3_51: cast reint s_3_50 -> i64
        let s_3_51: i64 = (s_3_50 as i64);
        // D s_3_52: cast zx s_3_51 -> i
        let s_3_52: i128 = (i128::try_from(s_3_51).unwrap());
        // D s_3_53: read-var sel_a:i64
        let s_3_53: i64 = fn_state.sel_a;
        // D s_3_54: cast zx s_3_53 -> i
        let s_3_54: i128 = (i128::try_from(s_3_53).unwrap());
        // D s_3_55: add s_3_52 s_3_54
        let s_3_55: i128 = (s_3_52 + s_3_54);
        // D s_3_56: cast reint s_3_55 -> i64
        let s_3_56: i64 = (s_3_55 as i64);
        // D s_3_57: read-var esize:i64
        let s_3_57: i64 = fn_state.esize;
        // D s_3_58: cast zx s_3_57 -> i
        let s_3_58: i128 = (i128::try_from(s_3_57).unwrap());
        // D s_3_59: cast reint s_3_58 -> i64
        let s_3_59: i64 = (s_3_58 as i64);
        // D s_3_60: cast zx s_3_56 -> i
        let s_3_60: i128 = (i128::try_from(s_3_56).unwrap());
        // D s_3_61: cast zx s_3_59 -> i
        let s_3_61: i128 = (i128::try_from(s_3_59).unwrap());
        // D s_3_62: read-var operand1:bv
        let s_3_62: Bits = fn_state.operand1;
        // D s_3_63: call Elem_read(s_3_62, s_3_60, s_3_61)
        let s_3_63: Bits = Elem_read(state, tracer, s_3_62, s_3_60, s_3_61);
        // D s_3_64: write-var elt1_a <= s_3_63
        fn_state.elt1_a = s_3_63;
        // C s_3_65: const #2s : i
        let s_3_65: i128 = 2;
        // D s_3_66: mul s_3_65 s_3_12
        let s_3_66: i128 = ((s_3_65) * (s_3_12));
        // D s_3_67: read-var sel_a:i64
        let s_3_67: i64 = fn_state.sel_a;
        // D s_3_68: cast zx s_3_67 -> i
        let s_3_68: i128 = (i128::try_from(s_3_67).unwrap());
        // D s_3_69: add s_3_66 s_3_68
        let s_3_69: i128 = (s_3_66 + s_3_68);
        // D s_3_70: read-var esize:i64
        let s_3_70: i64 = fn_state.esize;
        // D s_3_71: cast zx s_3_70 -> i
        let s_3_71: i128 = (i128::try_from(s_3_70).unwrap());
        // D s_3_72: cast reint s_3_71 -> i64
        let s_3_72: i64 = (s_3_71 as i64);
        // D s_3_73: cast zx s_3_72 -> i
        let s_3_73: i128 = (i128::try_from(s_3_72).unwrap());
        // D s_3_74: read-var operand2:bv
        let s_3_74: Bits = fn_state.operand2;
        // D s_3_75: call Elem_read(s_3_74, s_3_69, s_3_73)
        let s_3_75: Bits = Elem_read(state, tracer, s_3_74, s_3_69, s_3_73);
        // D s_3_76: write-var elt2_a <= s_3_75
        fn_state.elt2_a = s_3_75;
        // C s_3_77: const #2s : i
        let s_3_77: i128 = 2;
        // D s_3_78: mul s_3_77 s_3_12
        let s_3_78: i128 = ((s_3_77) * (s_3_12));
        // D s_3_79: read-var sel_b:i64
        let s_3_79: i64 = fn_state.sel_b;
        // D s_3_80: cast zx s_3_79 -> i
        let s_3_80: i128 = (i128::try_from(s_3_79).unwrap());
        // D s_3_81: add s_3_78 s_3_80
        let s_3_81: i128 = (s_3_78 + s_3_80);
        // D s_3_82: read-var esize:i64
        let s_3_82: i64 = fn_state.esize;
        // D s_3_83: cast zx s_3_82 -> i
        let s_3_83: i128 = (i128::try_from(s_3_82).unwrap());
        // D s_3_84: cast reint s_3_83 -> i64
        let s_3_84: i64 = (s_3_83 as i64);
        // D s_3_85: cast zx s_3_84 -> i
        let s_3_85: i128 = (i128::try_from(s_3_84).unwrap());
        // D s_3_86: read-var operand2:bv
        let s_3_86: Bits = fn_state.operand2;
        // D s_3_87: call Elem_read(s_3_86, s_3_81, s_3_85)
        let s_3_87: Bits = Elem_read(state, tracer, s_3_86, s_3_81, s_3_85);
        // D s_3_88: write-var elt2_b <= s_3_87
        fn_state.elt2_b = s_3_87;
        // D s_3_89: read-var neg_r:u8
        let s_3_89: bool = fn_state.neg_r;
        // N s_3_90: branch s_3_89 b12 b4
        if s_3_89 {
            return block_12(state, tracer, fn_state);
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
        // D s_5_0: read-var neg_i:u8
        let s_5_0: bool = fn_state.neg_i;
        // N s_5_1: branch s_5_0 b10 b6
        if s_5_0 {
            return block_10(state, tracer, fn_state);
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
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call FPCR_read(s_7_0)
        let s_7_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_7_0);
        // D s_7_2: read-var addend_r:bv
        let s_7_2: Bits = fn_state.addend_r;
        // D s_7_3: read-var elt1_a:bv
        let s_7_3: Bits = fn_state.elt1_a;
        // D s_7_4: read-var elt2_a:bv
        let s_7_4: Bits = fn_state.elt2_a;
        // D s_7_5: call FPMulAdd(s_7_2, s_7_3, s_7_4, s_7_1)
        let s_7_5: Bits = FPMulAdd(state, tracer, s_7_2, s_7_3, s_7_4, s_7_1);
        // D s_7_6: write-var addend_r <= s_7_5
        fn_state.addend_r = s_7_5;
        // N s_7_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call FPCR_read(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_8_0);
        // D s_8_2: read-var addend_i:bv
        let s_8_2: Bits = fn_state.addend_i;
        // D s_8_3: read-var elt1_a:bv
        let s_8_3: Bits = fn_state.elt1_a;
        // D s_8_4: read-var elt2_b:bv
        let s_8_4: Bits = fn_state.elt2_b;
        // D s_8_5: call FPMulAdd(s_8_2, s_8_3, s_8_4, s_8_1)
        let s_8_5: Bits = FPMulAdd(state, tracer, s_8_2, s_8_3, s_8_4, s_8_1);
        // D s_8_6: write-var addend_i <= s_8_5
        fn_state.addend_i = s_8_5;
        // N s_8_7: jump b9
        return block_9(state, tracer, fn_state);
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
        // D s_9_12: cast zx s_9_8 -> i
        let s_9_12: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_13: cast zx s_9_11 -> i
        let s_9_13: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_14: read-var result:bv
        let s_9_14: Bits = fn_state.result;
        // D s_9_15: read-var addend_r:bv
        let s_9_15: Bits = fn_state.addend_r;
        // D s_9_16: call Elem_set(s_9_14, s_9_12, s_9_13, s_9_15)
        let s_9_16: Bits = Elem_set(state, tracer, s_9_14, s_9_12, s_9_13, s_9_15);
        // D s_9_17: write-var result <= s_9_16
        fn_state.result = s_9_16;
        // C s_9_18: const #2s : i
        let s_9_18: i128 = 2;
        // D s_9_19: read-var p:i64
        let s_9_19: i64 = fn_state.p;
        // D s_9_20: cast zx s_9_19 -> i
        let s_9_20: i128 = (i128::try_from(s_9_19).unwrap());
        // D s_9_21: mul s_9_18 s_9_20
        let s_9_21: i128 = ((s_9_18) * (s_9_20));
        // D s_9_22: cast reint s_9_21 -> i64
        let s_9_22: i64 = (s_9_21 as i64);
        // C s_9_23: const #1s : i
        let s_9_23: i128 = 1;
        // D s_9_24: cast zx s_9_22 -> i
        let s_9_24: i128 = (i128::try_from(s_9_22).unwrap());
        // D s_9_25: add s_9_24 s_9_23
        let s_9_25: i128 = (s_9_24 + s_9_23);
        // D s_9_26: cast reint s_9_25 -> i64
        let s_9_26: i64 = (s_9_25 as i64);
        // D s_9_27: read-var esize:i64
        let s_9_27: i64 = fn_state.esize;
        // D s_9_28: cast zx s_9_27 -> i
        let s_9_28: i128 = (i128::try_from(s_9_27).unwrap());
        // D s_9_29: cast reint s_9_28 -> i64
        let s_9_29: i64 = (s_9_28 as i64);
        // D s_9_30: cast zx s_9_26 -> i
        let s_9_30: i128 = (i128::try_from(s_9_26).unwrap());
        // D s_9_31: cast zx s_9_29 -> i
        let s_9_31: i128 = (i128::try_from(s_9_29).unwrap());
        // D s_9_32: read-var result:bv
        let s_9_32: Bits = fn_state.result;
        // D s_9_33: read-var addend_i:bv
        let s_9_33: Bits = fn_state.addend_i;
        // D s_9_34: call Elem_set(s_9_32, s_9_30, s_9_31, s_9_33)
        let s_9_34: Bits = Elem_set(state, tracer, s_9_32, s_9_30, s_9_31, s_9_33);
        // D s_9_35: write-var result <= s_9_34
        fn_state.result = s_9_34;
        // D s_9_36: read-var p:i64
        let s_9_36: i64 = fn_state.p;
        // C s_9_37: const #1s : i64
        let s_9_37: i64 = 1;
        // D s_9_38: add s_9_36 s_9_37
        let s_9_38: i64 = (s_9_36 + s_9_37);
        // D s_9_39: write-var p <= s_9_38
        fn_state.p = s_9_38;
        // N s_9_40: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var elt2_b:bv
        let s_10_0: Bits = fn_state.elt2_b;
        // D s_10_1: call FPNeg(s_10_0)
        let s_10_1: Bits = FPNeg(state, tracer, s_10_0);
        // D s_10_2: write-var elt2_b <= s_10_1
        fn_state.elt2_b = s_10_1;
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var elt2_a:bv
        let s_12_0: Bits = fn_state.elt2_a;
        // D s_12_1: call FPNeg(s_12_0)
        let s_12_1: Bits = FPNeg(state, tracer, s_12_0);
        // D s_12_2: write-var elt2_a <= s_12_1
        fn_state.elt2_a = s_12_1;
        // N s_12_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var VLshadow#2302:i64
        let s_14_0: i64 = fn_state.VLshadow_2302;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: cast reint s_14_1 -> i64
        let s_14_2: i64 = (s_14_1 as i64);
        // D s_14_3: read-var da:i64
        let s_14_3: i64 = fn_state.da;
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: cast zx s_14_2 -> i
        let s_14_5: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_6: read-var result:bv
        let s_14_6: Bits = fn_state.result;
        // D s_14_7: call Z_set(s_14_4, s_14_5, s_14_6)
        let s_14_7: () = Z_set(state, tracer, s_14_4, s_14_5, s_14_6);
        // N s_14_8: return
        return;
    }
}
