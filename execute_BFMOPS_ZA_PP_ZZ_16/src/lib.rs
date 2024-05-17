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
use BFNeg::*;
use CheckStreamingSVEAndZAEnabled::*;
use ZAtile_read::*;
use ActivePredicateElement::*;
use P_read::*;
use FPCR_read::*;
use BFMulAdd_ZA::*;
use Elem_read::*;
use ZAtile_set::*;
use Z_read::*;
use common::*;
pub fn execute_BFMOPS_ZA_PP_ZZ_16<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dim_dim_16: i64,
    a: i64,
    b: i64,
    da: i64,
    m: i64,
    n: i64,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_873229: Bits,
        element3: u16,
        operand3: Bits,
        gs_291832: i64,
        row: i64,
        gs_873218: Bits,
        ga_341092: i64,
        dim_dim_16shadow_6935: i64,
        gs_291846: bool,
        element2: u16,
        element1: u16,
        col: i64,
        gs_291838: i64,
        dim: i64,
        VLshadow_6936: i64,
        result: Bits,
        mask1: Bits,
        VLshadow_6937: i64,
        operand1: Bits,
        mask2: Bits,
        operand2: Bits,
        VL: i64,
        dim_dim_16: i64,
        a: i64,
        b: i64,
        da: i64,
        m: i64,
        n: i64,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        VL,
        dim_dim_16,
        a,
        b,
        da,
        m,
        n,
        sub_op,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var dim_dim_16:i64
        let s_0_0: i64 = fn_state.dim_dim_16;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var dim_dim_16shadow#6935 <= s_0_2
        fn_state.dim_dim_16shadow_6935 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6936 <= s_0_6
        fn_state.VLshadow_6936 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckStreamingSVEAndZAEnabled(s_0_8)
        let s_0_9: () = CheckStreamingSVEAndZAEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#6936:i64
        let s_1_0: i64 = fn_state.VLshadow_6936;
        // D s_1_1: write-var VLshadow#6937 <= s_1_0
        fn_state.VLshadow_6937 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#6937:i64
        let s_1_3: i64 = fn_state.VLshadow_6937;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #16s : i
        let s_1_7: i128 = 16;
        // D s_1_8: read-var VLshadow#6937:i64
        let s_1_8: i64 = fn_state.VLshadow_6937;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: div s_1_9 s_1_7
        let s_1_10: i128 = ((s_1_9) / (s_1_7));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var dim <= s_1_11
        fn_state.dim = s_1_11;
        // D s_1_13: cast zx s_1_6 -> i
        let s_1_13: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var a:i64
        let s_1_15: i64 = fn_state.a;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call P_read(s_1_16, s_1_17)
        let s_1_18: Bits = P_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var mask1 <= s_1_18
        fn_state.mask1 = s_1_18;
        // D s_1_20: cast zx s_1_6 -> i
        let s_1_20: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_21: cast reint s_1_20 -> i64
        let s_1_21: i64 = (s_1_20 as i64);
        // D s_1_22: read-var b:i64
        let s_1_22: i64 = fn_state.b;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast zx s_1_21 -> i
        let s_1_24: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_25: call P_read(s_1_23, s_1_24)
        let s_1_25: Bits = P_read(state, tracer, s_1_23, s_1_24);
        // D s_1_26: write-var mask2 <= s_1_25
        fn_state.mask2 = s_1_25;
        // D s_1_27: read-var VLshadow#6937:i64
        let s_1_27: i64 = fn_state.VLshadow_6937;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast reint s_1_28 -> i64
        let s_1_29: i64 = (s_1_28 as i64);
        // D s_1_30: read-var n:i64
        let s_1_30: i64 = fn_state.n;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: cast zx s_1_29 -> i
        let s_1_32: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_33: call Z_read(s_1_31, s_1_32)
        let s_1_33: Bits = Z_read(state, tracer, s_1_31, s_1_32);
        // D s_1_34: write-var operand1 <= s_1_33
        fn_state.operand1 = s_1_33;
        // D s_1_35: read-var VLshadow#6937:i64
        let s_1_35: i64 = fn_state.VLshadow_6937;
        // D s_1_36: cast zx s_1_35 -> i
        let s_1_36: i128 = (i128::try_from(s_1_35).unwrap());
        // D s_1_37: cast reint s_1_36 -> i64
        let s_1_37: i64 = (s_1_36 as i64);
        // D s_1_38: read-var m:i64
        let s_1_38: i64 = fn_state.m;
        // D s_1_39: cast zx s_1_38 -> i
        let s_1_39: i128 = (i128::try_from(s_1_38).unwrap());
        // D s_1_40: cast zx s_1_37 -> i
        let s_1_40: i128 = (i128::try_from(s_1_37).unwrap());
        // D s_1_41: call Z_read(s_1_39, s_1_40)
        let s_1_41: Bits = Z_read(state, tracer, s_1_39, s_1_40);
        // D s_1_42: write-var operand2 <= s_1_41
        fn_state.operand2 = s_1_41;
        // D s_1_43: read-var VLshadow#6937:i64
        let s_1_43: i64 = fn_state.VLshadow_6937;
        // D s_1_44: cast zx s_1_43 -> i
        let s_1_44: i128 = (i128::try_from(s_1_43).unwrap());
        // D s_1_45: cast reint s_1_44 -> i64
        let s_1_45: i64 = (s_1_44 as i64);
        // D s_1_46: read-var dim_dim_16shadow#6935:i64
        let s_1_46: i64 = fn_state.dim_dim_16shadow_6935;
        // D s_1_47: cast zx s_1_46 -> i
        let s_1_47: i128 = (i128::try_from(s_1_46).unwrap());
        // D s_1_48: cast reint s_1_47 -> i64
        let s_1_48: i64 = (s_1_47 as i64);
        // C s_1_49: const #16s : i
        let s_1_49: i128 = 16;
        // D s_1_50: cast zx s_1_45 -> i
        let s_1_50: i128 = (i128::try_from(s_1_45).unwrap());
        // D s_1_51: read-var da:i64
        let s_1_51: i64 = fn_state.da;
        // D s_1_52: cast zx s_1_51 -> i
        let s_1_52: i128 = (i128::try_from(s_1_51).unwrap());
        // D s_1_53: cast zx s_1_48 -> i
        let s_1_53: i128 = (i128::try_from(s_1_48).unwrap());
        // D s_1_54: call ZAtile_read(s_1_50, s_1_52, s_1_49, s_1_53)
        let s_1_54: Bits = ZAtile_read(state, tracer, s_1_50, s_1_52, s_1_49, s_1_53);
        // D s_1_55: write-var operand3 <= s_1_54
        fn_state.operand3 = s_1_54;
        // C s_1_56: const #0s : i64
        let s_1_56: i64 = 0;
        // C s_1_57: const #1s : i
        let s_1_57: i128 = 1;
        // D s_1_58: read-var dim:i64
        let s_1_58: i64 = fn_state.dim;
        // D s_1_59: cast zx s_1_58 -> i
        let s_1_59: i128 = (i128::try_from(s_1_58).unwrap());
        // D s_1_60: sub s_1_59 s_1_57
        let s_1_60: i128 = ((s_1_59) - (s_1_57));
        // D s_1_61: cast reint s_1_60 -> i64
        let s_1_61: i64 = (s_1_60 as i64);
        // D s_1_62: write-var gs#291832 <= s_1_61
        fn_state.gs_291832 = s_1_61;
        // D s_1_63: write-var row <= s_1_56
        fn_state.row = s_1_56;
        // N s_1_64: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var row:i64
        let s_2_0: i64 = fn_state.row;
        // D s_2_1: read-var gs#291832:i64
        let s_2_1: i64 = fn_state.gs_291832;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b18 b3
        if s_2_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var dim:i64
        let s_3_2: i64 = fn_state.dim;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#291838 <= s_3_5
        fn_state.gs_291838 = s_3_5;
        // D s_3_7: write-var col <= s_3_0
        fn_state.col = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var col:i64
        let s_4_0: i64 = fn_state.col;
        // D s_4_1: read-var gs#291838:i64
        let s_4_1: i64 = fn_state.gs_291838;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b17 b5
        if s_4_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16s : i64
        let s_5_0: i64 = 16;
        // D s_5_1: read-var row:i64
        let s_5_1: i64 = fn_state.row;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // C s_5_3: cast zx s_5_0 -> i
        let s_5_3: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_4: read-var operand1:bv
        let s_5_4: Bits = fn_state.operand1;
        // D s_5_5: call Elem_read(s_5_4, s_5_2, s_5_3)
        let s_5_5: Bits = Elem_read(state, tracer, s_5_4, s_5_2, s_5_3);
        // D s_5_6: cast reint s_5_5 -> u16
        let s_5_6: u16 = (s_5_5.value() as u16);
        // D s_5_7: write-var element1 <= s_5_6
        fn_state.element1 = s_5_6;
        // C s_5_8: const #16s : i64
        let s_5_8: i64 = 16;
        // D s_5_9: read-var col:i64
        let s_5_9: i64 = fn_state.col;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // C s_5_11: cast zx s_5_8 -> i
        let s_5_11: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_12: read-var operand2:bv
        let s_5_12: Bits = fn_state.operand2;
        // D s_5_13: call Elem_read(s_5_12, s_5_10, s_5_11)
        let s_5_13: Bits = Elem_read(state, tracer, s_5_12, s_5_10, s_5_11);
        // D s_5_14: cast reint s_5_13 -> u16
        let s_5_14: u16 = (s_5_13.value() as u16);
        // D s_5_15: write-var element2 <= s_5_14
        fn_state.element2 = s_5_14;
        // D s_5_16: read-var row:i64
        let s_5_16: i64 = fn_state.row;
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_18: read-var dim:i64
        let s_5_18: i64 = fn_state.dim;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: mul s_5_17 s_5_19
        let s_5_20: i128 = ((s_5_17) * (s_5_19));
        // D s_5_21: cast reint s_5_20 -> i64
        let s_5_21: i64 = (s_5_20 as i64);
        // D s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_23: read-var col:i64
        let s_5_23: i64 = fn_state.col;
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: add s_5_22 s_5_24
        let s_5_25: i128 = (s_5_22 + s_5_24);
        // D s_5_26: cast reint s_5_25 -> i64
        let s_5_26: i64 = (s_5_25 as i64);
        // C s_5_27: const #16s : i64
        let s_5_27: i64 = 16;
        // D s_5_28: cast zx s_5_26 -> i
        let s_5_28: i128 = (i128::try_from(s_5_26).unwrap());
        // C s_5_29: cast zx s_5_27 -> i
        let s_5_29: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_30: read-var operand3:bv
        let s_5_30: Bits = fn_state.operand3;
        // D s_5_31: call Elem_read(s_5_30, s_5_28, s_5_29)
        let s_5_31: Bits = Elem_read(state, tracer, s_5_30, s_5_28, s_5_29);
        // D s_5_32: cast reint s_5_31 -> u16
        let s_5_32: u16 = (s_5_31.value() as u16);
        // D s_5_33: write-var element3 <= s_5_32
        fn_state.element3 = s_5_32;
        // C s_5_34: const #16s : i
        let s_5_34: i128 = 16;
        // D s_5_35: read-var row:i64
        let s_5_35: i64 = fn_state.row;
        // D s_5_36: cast zx s_5_35 -> i
        let s_5_36: i128 = (i128::try_from(s_5_35).unwrap());
        // D s_5_37: read-var mask1:bv
        let s_5_37: Bits = fn_state.mask1;
        // D s_5_38: call ActivePredicateElement(s_5_37, s_5_36, s_5_34)
        let s_5_38: bool = ActivePredicateElement(state, tracer, s_5_37, s_5_36, s_5_34);
        // N s_5_39: branch s_5_38 b16 b6
        if s_5_38 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#291846 <= s_6_0
        fn_state.gs_291846 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#291846:u8
        let s_7_0: bool = fn_state.gs_291846;
        // N s_7_1: branch s_7_0 b10 b8
        if s_7_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var row:i64
        let s_8_0: i64 = fn_state.row;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var dim:i64
        let s_8_2: i64 = fn_state.dim;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: mul s_8_1 s_8_3
        let s_8_4: i128 = ((s_8_1) * (s_8_3));
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: read-var col:i64
        let s_8_7: i64 = fn_state.col;
        // D s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_9: add s_8_6 s_8_8
        let s_8_9: i128 = (s_8_6 + s_8_8);
        // D s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // C s_8_11: const #16s : i64
        let s_8_11: i64 = 16;
        // D s_8_12: cast zx s_8_10 -> i
        let s_8_12: i128 = (i128::try_from(s_8_10).unwrap());
        // C s_8_13: cast zx s_8_11 -> i
        let s_8_13: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_14: read-var element3:u16
        let s_8_14: u16 = fn_state.element3;
        // D s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 16u16);
        // D s_8_16: read-var result:bv
        let s_8_16: Bits = fn_state.result;
        // D s_8_17: call Elem_set(s_8_16, s_8_12, s_8_13, s_8_15)
        let s_8_17: Bits = Elem_set(state, tracer, s_8_16, s_8_12, s_8_13, s_8_15);
        // D s_8_18: write-var result <= s_8_17
        fn_state.result = s_8_17;
        // N s_8_19: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var col:i64
        let s_9_0: i64 = fn_state.col;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var col <= s_9_2
        fn_state.col = s_9_2;
        // N s_9_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var sub_op:u8
        let s_10_0: bool = fn_state.sub_op;
        // N s_10_1: branch s_10_0 b14 b11
        if s_10_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var row:i64
        let s_12_0: i64 = fn_state.row;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var dim:i64
        let s_12_2: i64 = fn_state.dim;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: mul s_12_1 s_12_3
        let s_12_4: i128 = ((s_12_1) * (s_12_3));
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_7: read-var col:i64
        let s_12_7: i64 = fn_state.col;
        // D s_12_8: cast zx s_12_7 -> i
        let s_12_8: i128 = (i128::try_from(s_12_7).unwrap());
        // D s_12_9: add s_12_6 s_12_8
        let s_12_9: i128 = (s_12_6 + s_12_8);
        // D s_12_10: cast reint s_12_9 -> i64
        let s_12_10: i64 = (s_12_9 as i64);
        // D s_12_11: write-var ga#341092 <= s_12_10
        fn_state.ga_341092 = s_12_10;
        // C s_12_12: const #() : ()
        let s_12_12: () = ();
        // S s_12_13: call FPCR_read(s_12_12)
        let s_12_13: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_12_12);
        // D s_12_14: read-var element3:u16
        let s_12_14: u16 = fn_state.element3;
        // D s_12_15: cast zx s_12_14 -> bv
        let s_12_15: Bits = Bits::new(s_12_14 as u128, 16u16);
        // D s_12_16: read-var element1:u16
        let s_12_16: u16 = fn_state.element1;
        // D s_12_17: cast zx s_12_16 -> bv
        let s_12_17: Bits = Bits::new(s_12_16 as u128, 16u16);
        // D s_12_18: read-var element2:u16
        let s_12_18: u16 = fn_state.element2;
        // D s_12_19: cast zx s_12_18 -> bv
        let s_12_19: Bits = Bits::new(s_12_18 as u128, 16u16);
        // D s_12_20: call BFMulAdd_ZA(s_12_15, s_12_17, s_12_19, s_12_13)
        let s_12_20: Bits = BFMulAdd_ZA(
            state,
            tracer,
            s_12_15,
            s_12_17,
            s_12_19,
            s_12_13,
        );
        // D s_12_21: write-var gs#873229 <= s_12_20
        fn_state.gs_873229 = s_12_20;
        // N s_12_22: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#873229:bv
        let s_13_0: Bits = fn_state.gs_873229;
        // D s_13_1: cast reint s_13_0 -> u16
        let s_13_1: u16 = (s_13_0.value() as u16);
        // D s_13_2: read-var ga#341092:i64
        let s_13_2: i64 = fn_state.ga_341092;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // C s_13_4: const #16s : i64
        let s_13_4: i64 = 16;
        // C s_13_5: cast zx s_13_4 -> i
        let s_13_5: i128 = (i128::try_from(s_13_4).unwrap());
        // D s_13_6: cast zx s_13_1 -> bv
        let s_13_6: Bits = Bits::new(s_13_1 as u128, 16u16);
        // D s_13_7: read-var result:bv
        let s_13_7: Bits = fn_state.result;
        // D s_13_8: call Elem_set(s_13_7, s_13_3, s_13_5, s_13_6)
        let s_13_8: Bits = Elem_set(state, tracer, s_13_7, s_13_3, s_13_5, s_13_6);
        // D s_13_9: write-var result <= s_13_8
        fn_state.result = s_13_8;
        // N s_13_10: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var element1:u16
        let s_14_0: u16 = fn_state.element1;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 16u16);
        // D s_14_2: call BFNeg(s_14_1)
        let s_14_2: Bits = BFNeg(state, tracer, s_14_1);
        // D s_14_3: write-var gs#873218 <= s_14_2
        fn_state.gs_873218 = s_14_2;
        // N s_14_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#873218:bv
        let s_15_0: Bits = fn_state.gs_873218;
        // D s_15_1: cast reint s_15_0 -> u16
        let s_15_1: u16 = (s_15_0.value() as u16);
        // D s_15_2: write-var element1 <= s_15_1
        fn_state.element1 = s_15_1;
        // N s_15_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16s : i
        let s_16_0: i128 = 16;
        // D s_16_1: read-var col:i64
        let s_16_1: i64 = fn_state.col;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: read-var mask2:bv
        let s_16_3: Bits = fn_state.mask2;
        // D s_16_4: call ActivePredicateElement(s_16_3, s_16_2, s_16_0)
        let s_16_4: bool = ActivePredicateElement(state, tracer, s_16_3, s_16_2, s_16_0);
        // D s_16_5: write-var gs#291846 <= s_16_4
        fn_state.gs_291846 = s_16_4;
        // N s_16_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var row:i64
        let s_17_0: i64 = fn_state.row;
        // C s_17_1: const #1s : i64
        let s_17_1: i64 = 1;
        // D s_17_2: add s_17_0 s_17_1
        let s_17_2: i64 = (s_17_0 + s_17_1);
        // D s_17_3: write-var row <= s_17_2
        fn_state.row = s_17_2;
        // N s_17_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var VLshadow#6937:i64
        let s_18_0: i64 = fn_state.VLshadow_6937;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: cast reint s_18_1 -> i64
        let s_18_2: i64 = (s_18_1 as i64);
        // D s_18_3: read-var dim_dim_16shadow#6935:i64
        let s_18_3: i64 = fn_state.dim_dim_16shadow_6935;
        // D s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_5: cast reint s_18_4 -> i64
        let s_18_5: i64 = (s_18_4 as i64);
        // C s_18_6: const #16s : i
        let s_18_6: i128 = 16;
        // D s_18_7: cast zx s_18_2 -> i
        let s_18_7: i128 = (i128::try_from(s_18_2).unwrap());
        // D s_18_8: read-var da:i64
        let s_18_8: i64 = fn_state.da;
        // D s_18_9: cast zx s_18_8 -> i
        let s_18_9: i128 = (i128::try_from(s_18_8).unwrap());
        // D s_18_10: cast zx s_18_5 -> i
        let s_18_10: i128 = (i128::try_from(s_18_5).unwrap());
        // D s_18_11: read-var result:bv
        let s_18_11: Bits = fn_state.result;
        // D s_18_12: call ZAtile_set(s_18_7, s_18_9, s_18_6, s_18_10, s_18_11)
        let s_18_12: () = ZAtile_set(
            state,
            tracer,
            s_18_7,
            s_18_9,
            s_18_6,
            s_18_10,
            s_18_11,
        );
        // N s_18_13: return
        return;
    }
}
