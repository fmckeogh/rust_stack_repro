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
use FPDotAdd_ZA::*;
use Elem_set::*;
use CheckStreamingSVEAndZAEnabled::*;
use ZAtile_read::*;
use ActivePredicateElement::*;
use P_read::*;
use FPCR_read::*;
use FPNeg::*;
use FPZero::*;
use Elem_read::*;
use ZAtile_set::*;
use Z_read::*;
use common::*;
pub fn execute_FMOPS_ZA32_PP_ZZ_16<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dim_dim_32: i64,
    a: i64,
    b: i64,
    da: i64,
    m: i64,
    n: i64,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VLshadow_5439: i64,
        gs_820498: Bits,
        gs_255099: bool,
        row: i64,
        pcol_1: bool,
        dim: i64,
        dim_dim_32shadow_5438: i64,
        mask1: Bits,
        operand1: Bits,
        operand2: Bits,
        prow_1: bool,
        sum: u32,
        operand3: Bits,
        erow_0: u16,
        ecol_0: u16,
        erow_1: u16,
        gs_255082: i64,
        VLshadow_5440: i64,
        prow_0: bool,
        col: i64,
        ecol_1: u16,
        gs_255098: bool,
        gs_820490: Bits,
        result: Bits,
        gs_820492: Bits,
        pcol_0: bool,
        gs_255076: i64,
        gs_255100: bool,
        mask2: Bits,
        VL: i64,
        dim_dim_32: i64,
        a: i64,
        b: i64,
        da: i64,
        m: i64,
        n: i64,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        VL,
        dim_dim_32,
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
        // D s_0_0: read-var dim_dim_32:i64
        let s_0_0: i64 = fn_state.dim_dim_32;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var dim_dim_32shadow#5438 <= s_0_2
        fn_state.dim_dim_32shadow_5438 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#5439 <= s_0_6
        fn_state.VLshadow_5439 = s_0_6;
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
        // D s_1_0: read-var VLshadow#5439:i64
        let s_1_0: i64 = fn_state.VLshadow_5439;
        // D s_1_1: write-var VLshadow#5440 <= s_1_0
        fn_state.VLshadow_5440 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5440:i64
        let s_1_3: i64 = fn_state.VLshadow_5440;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #32s : i
        let s_1_7: i128 = 32;
        // D s_1_8: read-var VLshadow#5440:i64
        let s_1_8: i64 = fn_state.VLshadow_5440;
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
        // D s_1_27: read-var VLshadow#5440:i64
        let s_1_27: i64 = fn_state.VLshadow_5440;
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
        // D s_1_35: read-var VLshadow#5440:i64
        let s_1_35: i64 = fn_state.VLshadow_5440;
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
        // D s_1_43: read-var VLshadow#5440:i64
        let s_1_43: i64 = fn_state.VLshadow_5440;
        // D s_1_44: cast zx s_1_43 -> i
        let s_1_44: i128 = (i128::try_from(s_1_43).unwrap());
        // D s_1_45: cast reint s_1_44 -> i64
        let s_1_45: i64 = (s_1_44 as i64);
        // D s_1_46: read-var dim_dim_32shadow#5438:i64
        let s_1_46: i64 = fn_state.dim_dim_32shadow_5438;
        // D s_1_47: cast zx s_1_46 -> i
        let s_1_47: i128 = (i128::try_from(s_1_46).unwrap());
        // D s_1_48: cast reint s_1_47 -> i64
        let s_1_48: i64 = (s_1_47 as i64);
        // C s_1_49: const #32s : i
        let s_1_49: i128 = 32;
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
        // D s_1_62: write-var gs#255076 <= s_1_61
        fn_state.gs_255076 = s_1_61;
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
        // D s_2_1: read-var gs#255076:i64
        let s_2_1: i64 = fn_state.gs_255076;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b43 b3
        if s_2_2 {
            return block_43(state, tracer, fn_state);
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
        // D s_3_6: write-var gs#255082 <= s_3_5
        fn_state.gs_255082 = s_3_5;
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
        // D s_4_1: read-var gs#255082:i64
        let s_4_1: i64 = fn_state.gs_255082;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b42 b5
        if s_4_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var row:i64
        let s_5_1: i64 = fn_state.row;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // C s_5_5: const #0s : i
        let s_5_5: i128 = 0;
        // D s_5_6: cast zx s_5_4 -> i
        let s_5_6: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // C s_5_9: const #16s : i
        let s_5_9: i128 = 16;
        // D s_5_10: cast zx s_5_8 -> i
        let s_5_10: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_11: read-var mask1:bv
        let s_5_11: Bits = fn_state.mask1;
        // D s_5_12: call ActivePredicateElement(s_5_11, s_5_10, s_5_9)
        let s_5_12: bool = ActivePredicateElement(state, tracer, s_5_11, s_5_10, s_5_9);
        // D s_5_13: write-var prow_0 <= s_5_12
        fn_state.prow_0 = s_5_12;
        // C s_5_14: const #2s : i
        let s_5_14: i128 = 2;
        // D s_5_15: read-var row:i64
        let s_5_15: i64 = fn_state.row;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: mul s_5_14 s_5_16
        let s_5_17: i128 = ((s_5_14) * (s_5_16));
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // C s_5_19: const #1s : i
        let s_5_19: i128 = 1;
        // D s_5_20: cast zx s_5_18 -> i
        let s_5_20: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_21: add s_5_20 s_5_19
        let s_5_21: i128 = (s_5_20 + s_5_19);
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // C s_5_23: const #16s : i
        let s_5_23: i128 = 16;
        // D s_5_24: cast zx s_5_22 -> i
        let s_5_24: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_25: read-var mask1:bv
        let s_5_25: Bits = fn_state.mask1;
        // D s_5_26: call ActivePredicateElement(s_5_25, s_5_24, s_5_23)
        let s_5_26: bool = ActivePredicateElement(state, tracer, s_5_25, s_5_24, s_5_23);
        // D s_5_27: write-var prow_1 <= s_5_26
        fn_state.prow_1 = s_5_26;
        // C s_5_28: const #2s : i
        let s_5_28: i128 = 2;
        // D s_5_29: read-var col:i64
        let s_5_29: i64 = fn_state.col;
        // D s_5_30: cast zx s_5_29 -> i
        let s_5_30: i128 = (i128::try_from(s_5_29).unwrap());
        // D s_5_31: mul s_5_28 s_5_30
        let s_5_31: i128 = ((s_5_28) * (s_5_30));
        // D s_5_32: cast reint s_5_31 -> i64
        let s_5_32: i64 = (s_5_31 as i64);
        // C s_5_33: const #0s : i
        let s_5_33: i128 = 0;
        // D s_5_34: cast zx s_5_32 -> i
        let s_5_34: i128 = (i128::try_from(s_5_32).unwrap());
        // D s_5_35: add s_5_34 s_5_33
        let s_5_35: i128 = (s_5_34 + s_5_33);
        // D s_5_36: cast reint s_5_35 -> i64
        let s_5_36: i64 = (s_5_35 as i64);
        // C s_5_37: const #16s : i
        let s_5_37: i128 = 16;
        // D s_5_38: cast zx s_5_36 -> i
        let s_5_38: i128 = (i128::try_from(s_5_36).unwrap());
        // D s_5_39: read-var mask2:bv
        let s_5_39: Bits = fn_state.mask2;
        // D s_5_40: call ActivePredicateElement(s_5_39, s_5_38, s_5_37)
        let s_5_40: bool = ActivePredicateElement(state, tracer, s_5_39, s_5_38, s_5_37);
        // D s_5_41: write-var pcol_0 <= s_5_40
        fn_state.pcol_0 = s_5_40;
        // C s_5_42: const #2s : i
        let s_5_42: i128 = 2;
        // D s_5_43: read-var col:i64
        let s_5_43: i64 = fn_state.col;
        // D s_5_44: cast zx s_5_43 -> i
        let s_5_44: i128 = (i128::try_from(s_5_43).unwrap());
        // D s_5_45: mul s_5_42 s_5_44
        let s_5_45: i128 = ((s_5_42) * (s_5_44));
        // D s_5_46: cast reint s_5_45 -> i64
        let s_5_46: i64 = (s_5_45 as i64);
        // C s_5_47: const #1s : i
        let s_5_47: i128 = 1;
        // D s_5_48: cast zx s_5_46 -> i
        let s_5_48: i128 = (i128::try_from(s_5_46).unwrap());
        // D s_5_49: add s_5_48 s_5_47
        let s_5_49: i128 = (s_5_48 + s_5_47);
        // D s_5_50: cast reint s_5_49 -> i64
        let s_5_50: i64 = (s_5_49 as i64);
        // C s_5_51: const #16s : i
        let s_5_51: i128 = 16;
        // D s_5_52: cast zx s_5_50 -> i
        let s_5_52: i128 = (i128::try_from(s_5_50).unwrap());
        // D s_5_53: read-var mask2:bv
        let s_5_53: Bits = fn_state.mask2;
        // D s_5_54: call ActivePredicateElement(s_5_53, s_5_52, s_5_51)
        let s_5_54: bool = ActivePredicateElement(state, tracer, s_5_53, s_5_52, s_5_51);
        // D s_5_55: write-var pcol_1 <= s_5_54
        fn_state.pcol_1 = s_5_54;
        // D s_5_56: read-var row:i64
        let s_5_56: i64 = fn_state.row;
        // D s_5_57: cast zx s_5_56 -> i
        let s_5_57: i128 = (i128::try_from(s_5_56).unwrap());
        // D s_5_58: read-var dim:i64
        let s_5_58: i64 = fn_state.dim;
        // D s_5_59: cast zx s_5_58 -> i
        let s_5_59: i128 = (i128::try_from(s_5_58).unwrap());
        // D s_5_60: mul s_5_57 s_5_59
        let s_5_60: i128 = ((s_5_57) * (s_5_59));
        // D s_5_61: cast reint s_5_60 -> i64
        let s_5_61: i64 = (s_5_60 as i64);
        // D s_5_62: cast zx s_5_61 -> i
        let s_5_62: i128 = (i128::try_from(s_5_61).unwrap());
        // D s_5_63: read-var col:i64
        let s_5_63: i64 = fn_state.col;
        // D s_5_64: cast zx s_5_63 -> i
        let s_5_64: i128 = (i128::try_from(s_5_63).unwrap());
        // D s_5_65: add s_5_62 s_5_64
        let s_5_65: i128 = (s_5_62 + s_5_64);
        // D s_5_66: cast reint s_5_65 -> i64
        let s_5_66: i64 = (s_5_65 as i64);
        // C s_5_67: const #32s : i64
        let s_5_67: i64 = 32;
        // D s_5_68: cast zx s_5_66 -> i
        let s_5_68: i128 = (i128::try_from(s_5_66).unwrap());
        // C s_5_69: cast zx s_5_67 -> i
        let s_5_69: i128 = (i128::try_from(s_5_67).unwrap());
        // D s_5_70: read-var operand3:bv
        let s_5_70: Bits = fn_state.operand3;
        // D s_5_71: call Elem_read(s_5_70, s_5_68, s_5_69)
        let s_5_71: Bits = Elem_read(state, tracer, s_5_70, s_5_68, s_5_69);
        // D s_5_72: cast reint s_5_71 -> u32
        let s_5_72: u32 = (s_5_71.value() as u32);
        // D s_5_73: write-var sum <= s_5_72
        fn_state.sum = s_5_72;
        // D s_5_74: read-var prow_0:u8
        let s_5_74: bool = fn_state.prow_0;
        // N s_5_75: branch s_5_74 b41 b6
        if s_5_74 {
            return block_41(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#255098 <= s_6_0
        fn_state.gs_255098 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#255098:u8
        let s_7_0: bool = fn_state.gs_255098;
        // N s_7_1: branch s_7_0 b40 b8
        if s_7_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var prow_1:u8
        let s_8_0: bool = fn_state.prow_1;
        // N s_8_1: branch s_8_0 b39 b9
        if s_8_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#255099 <= s_9_0
        fn_state.gs_255099 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#255099:u8
        let s_10_0: bool = fn_state.gs_255099;
        // D s_10_1: write-var gs#255100 <= s_10_0
        fn_state.gs_255100 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#255100:u8
        let s_11_0: bool = fn_state.gs_255100;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var row:i64
        let s_13_0: i64 = fn_state.row;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var dim:i64
        let s_13_2: i64 = fn_state.dim;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: mul s_13_1 s_13_3
        let s_13_4: i128 = ((s_13_1) * (s_13_3));
        // D s_13_5: cast reint s_13_4 -> i64
        let s_13_5: i64 = (s_13_4 as i64);
        // D s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // D s_13_7: read-var col:i64
        let s_13_7: i64 = fn_state.col;
        // D s_13_8: cast zx s_13_7 -> i
        let s_13_8: i128 = (i128::try_from(s_13_7).unwrap());
        // D s_13_9: add s_13_6 s_13_8
        let s_13_9: i128 = (s_13_6 + s_13_8);
        // D s_13_10: cast reint s_13_9 -> i64
        let s_13_10: i64 = (s_13_9 as i64);
        // C s_13_11: const #32s : i64
        let s_13_11: i64 = 32;
        // D s_13_12: cast zx s_13_10 -> i
        let s_13_12: i128 = (i128::try_from(s_13_10).unwrap());
        // C s_13_13: cast zx s_13_11 -> i
        let s_13_13: i128 = (i128::try_from(s_13_11).unwrap());
        // D s_13_14: read-var sum:u32
        let s_13_14: u32 = fn_state.sum;
        // D s_13_15: cast zx s_13_14 -> bv
        let s_13_15: Bits = Bits::new(s_13_14 as u128, 32u16);
        // D s_13_16: read-var result:bv
        let s_13_16: Bits = fn_state.result;
        // D s_13_17: call Elem_set(s_13_16, s_13_12, s_13_13, s_13_15)
        let s_13_17: Bits = Elem_set(state, tracer, s_13_16, s_13_12, s_13_13, s_13_15);
        // D s_13_18: write-var result <= s_13_17
        fn_state.result = s_13_17;
        // D s_13_19: read-var col:i64
        let s_13_19: i64 = fn_state.col;
        // C s_13_20: const #1s : i64
        let s_13_20: i64 = 1;
        // D s_13_21: add s_13_19 s_13_20
        let s_13_21: i64 = (s_13_19 + s_13_20);
        // D s_13_22: write-var col <= s_13_21
        fn_state.col = s_13_21;
        // N s_13_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var prow_0:u8
        let s_14_0: bool = fn_state.prow_0;
        // N s_14_1: branch s_14_0 b38 b15
        if s_14_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16s : i64
        let s_15_0: i64 = 16;
        // C s_15_1: const #0u : u8
        let s_15_1: bool = false;
        // S s_15_2: call FPZero(s_15_1, s_15_0)
        let s_15_2: Bits = FPZero(state, tracer, s_15_1, s_15_0);
        // S s_15_3: cast reint s_15_2 -> u16
        let s_15_3: u16 = (s_15_2.value() as u16);
        // D s_15_4: write-var erow_0 <= s_15_3
        fn_state.erow_0 = s_15_3;
        // N s_15_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var prow_1:u8
        let s_16_0: bool = fn_state.prow_1;
        // N s_16_1: branch s_16_0 b37 b17
        if s_16_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #16s : i64
        let s_17_0: i64 = 16;
        // C s_17_1: const #0u : u8
        let s_17_1: bool = false;
        // S s_17_2: call FPZero(s_17_1, s_17_0)
        let s_17_2: Bits = FPZero(state, tracer, s_17_1, s_17_0);
        // S s_17_3: cast reint s_17_2 -> u16
        let s_17_3: u16 = (s_17_2.value() as u16);
        // D s_17_4: write-var erow_1 <= s_17_3
        fn_state.erow_1 = s_17_3;
        // N s_17_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var pcol_0:u8
        let s_18_0: bool = fn_state.pcol_0;
        // N s_18_1: branch s_18_0 b36 b19
        if s_18_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #16s : i64
        let s_19_0: i64 = 16;
        // C s_19_1: const #0u : u8
        let s_19_1: bool = false;
        // S s_19_2: call FPZero(s_19_1, s_19_0)
        let s_19_2: Bits = FPZero(state, tracer, s_19_1, s_19_0);
        // S s_19_3: cast reint s_19_2 -> u16
        let s_19_3: u16 = (s_19_2.value() as u16);
        // D s_19_4: write-var ecol_0 <= s_19_3
        fn_state.ecol_0 = s_19_3;
        // N s_19_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var pcol_1:u8
        let s_20_0: bool = fn_state.pcol_1;
        // N s_20_1: branch s_20_0 b35 b21
        if s_20_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #16s : i64
        let s_21_0: i64 = 16;
        // C s_21_1: const #0u : u8
        let s_21_1: bool = false;
        // S s_21_2: call FPZero(s_21_1, s_21_0)
        let s_21_2: Bits = FPZero(state, tracer, s_21_1, s_21_0);
        // S s_21_3: cast reint s_21_2 -> u16
        let s_21_3: u16 = (s_21_2.value() as u16);
        // D s_21_4: write-var ecol_1 <= s_21_3
        fn_state.ecol_1 = s_21_3;
        // N s_21_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var sub_op:u8
        let s_22_0: bool = fn_state.sub_op;
        // N s_22_1: branch s_22_0 b26 b23
        if s_22_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call FPCR_read(s_24_0)
        let s_24_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_24_0);
        // D s_24_2: read-var sum:u32
        let s_24_2: u32 = fn_state.sum;
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 32u16);
        // D s_24_4: read-var erow_0:u16
        let s_24_4: u16 = fn_state.erow_0;
        // D s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 16u16);
        // D s_24_6: read-var erow_1:u16
        let s_24_6: u16 = fn_state.erow_1;
        // D s_24_7: cast zx s_24_6 -> bv
        let s_24_7: Bits = Bits::new(s_24_6 as u128, 16u16);
        // D s_24_8: read-var ecol_0:u16
        let s_24_8: u16 = fn_state.ecol_0;
        // D s_24_9: cast zx s_24_8 -> bv
        let s_24_9: Bits = Bits::new(s_24_8 as u128, 16u16);
        // D s_24_10: read-var ecol_1:u16
        let s_24_10: u16 = fn_state.ecol_1;
        // D s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 16u16);
        // D s_24_12: call FPDotAdd_ZA(s_24_3, s_24_5, s_24_7, s_24_9, s_24_11, s_24_1)
        let s_24_12: Bits = FPDotAdd_ZA(
            state,
            tracer,
            s_24_3,
            s_24_5,
            s_24_7,
            s_24_9,
            s_24_11,
            s_24_1,
        );
        // D s_24_13: write-var gs#820498 <= s_24_12
        fn_state.gs_820498 = s_24_12;
        // N s_24_14: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#820498:bv
        let s_25_0: Bits = fn_state.gs_820498;
        // D s_25_1: cast reint s_25_0 -> u32
        let s_25_1: u32 = (s_25_0.value() as u32);
        // D s_25_2: write-var sum <= s_25_1
        fn_state.sum = s_25_1;
        // N s_25_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var prow_0:u8
        let s_26_0: bool = fn_state.prow_0;
        // N s_26_1: branch s_26_0 b33 b27
        if s_26_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var prow_1:u8
        let s_28_0: bool = fn_state.prow_1;
        // N s_28_1: branch s_28_0 b31 b29
        if s_28_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var erow_1:u16
        let s_31_0: u16 = fn_state.erow_1;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 16u16);
        // D s_31_2: call FPNeg(s_31_1)
        let s_31_2: Bits = FPNeg(state, tracer, s_31_1);
        // D s_31_3: write-var gs#820492 <= s_31_2
        fn_state.gs_820492 = s_31_2;
        // N s_31_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#820492:bv
        let s_32_0: Bits = fn_state.gs_820492;
        // D s_32_1: cast reint s_32_0 -> u16
        let s_32_1: u16 = (s_32_0.value() as u16);
        // D s_32_2: write-var erow_1 <= s_32_1
        fn_state.erow_1 = s_32_1;
        // N s_32_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var erow_0:u16
        let s_33_0: u16 = fn_state.erow_0;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 16u16);
        // D s_33_2: call FPNeg(s_33_1)
        let s_33_2: Bits = FPNeg(state, tracer, s_33_1);
        // D s_33_3: write-var gs#820490 <= s_33_2
        fn_state.gs_820490 = s_33_2;
        // N s_33_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#820490:bv
        let s_34_0: Bits = fn_state.gs_820490;
        // D s_34_1: cast reint s_34_0 -> u16
        let s_34_1: u16 = (s_34_0.value() as u16);
        // D s_34_2: write-var erow_0 <= s_34_1
        fn_state.erow_0 = s_34_1;
        // N s_34_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #2s : i
        let s_35_0: i128 = 2;
        // D s_35_1: read-var col:i64
        let s_35_1: i64 = fn_state.col;
        // D s_35_2: cast zx s_35_1 -> i
        let s_35_2: i128 = (i128::try_from(s_35_1).unwrap());
        // D s_35_3: mul s_35_0 s_35_2
        let s_35_3: i128 = ((s_35_0) * (s_35_2));
        // D s_35_4: cast reint s_35_3 -> i64
        let s_35_4: i64 = (s_35_3 as i64);
        // C s_35_5: const #1s : i
        let s_35_5: i128 = 1;
        // D s_35_6: cast zx s_35_4 -> i
        let s_35_6: i128 = (i128::try_from(s_35_4).unwrap());
        // D s_35_7: add s_35_6 s_35_5
        let s_35_7: i128 = (s_35_6 + s_35_5);
        // D s_35_8: cast reint s_35_7 -> i64
        let s_35_8: i64 = (s_35_7 as i64);
        // C s_35_9: const #16s : i64
        let s_35_9: i64 = 16;
        // D s_35_10: cast zx s_35_8 -> i
        let s_35_10: i128 = (i128::try_from(s_35_8).unwrap());
        // C s_35_11: cast zx s_35_9 -> i
        let s_35_11: i128 = (i128::try_from(s_35_9).unwrap());
        // D s_35_12: read-var operand2:bv
        let s_35_12: Bits = fn_state.operand2;
        // D s_35_13: call Elem_read(s_35_12, s_35_10, s_35_11)
        let s_35_13: Bits = Elem_read(state, tracer, s_35_12, s_35_10, s_35_11);
        // D s_35_14: cast reint s_35_13 -> u16
        let s_35_14: u16 = (s_35_13.value() as u16);
        // D s_35_15: write-var ecol_1 <= s_35_14
        fn_state.ecol_1 = s_35_14;
        // N s_35_16: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #2s : i
        let s_36_0: i128 = 2;
        // D s_36_1: read-var col:i64
        let s_36_1: i64 = fn_state.col;
        // D s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (i128::try_from(s_36_1).unwrap());
        // D s_36_3: mul s_36_0 s_36_2
        let s_36_3: i128 = ((s_36_0) * (s_36_2));
        // D s_36_4: cast reint s_36_3 -> i64
        let s_36_4: i64 = (s_36_3 as i64);
        // C s_36_5: const #0s : i
        let s_36_5: i128 = 0;
        // D s_36_6: cast zx s_36_4 -> i
        let s_36_6: i128 = (i128::try_from(s_36_4).unwrap());
        // D s_36_7: add s_36_6 s_36_5
        let s_36_7: i128 = (s_36_6 + s_36_5);
        // D s_36_8: cast reint s_36_7 -> i64
        let s_36_8: i64 = (s_36_7 as i64);
        // C s_36_9: const #16s : i64
        let s_36_9: i64 = 16;
        // D s_36_10: cast zx s_36_8 -> i
        let s_36_10: i128 = (i128::try_from(s_36_8).unwrap());
        // C s_36_11: cast zx s_36_9 -> i
        let s_36_11: i128 = (i128::try_from(s_36_9).unwrap());
        // D s_36_12: read-var operand2:bv
        let s_36_12: Bits = fn_state.operand2;
        // D s_36_13: call Elem_read(s_36_12, s_36_10, s_36_11)
        let s_36_13: Bits = Elem_read(state, tracer, s_36_12, s_36_10, s_36_11);
        // D s_36_14: cast reint s_36_13 -> u16
        let s_36_14: u16 = (s_36_13.value() as u16);
        // D s_36_15: write-var ecol_0 <= s_36_14
        fn_state.ecol_0 = s_36_14;
        // N s_36_16: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #2s : i
        let s_37_0: i128 = 2;
        // D s_37_1: read-var row:i64
        let s_37_1: i64 = fn_state.row;
        // D s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (i128::try_from(s_37_1).unwrap());
        // D s_37_3: mul s_37_0 s_37_2
        let s_37_3: i128 = ((s_37_0) * (s_37_2));
        // D s_37_4: cast reint s_37_3 -> i64
        let s_37_4: i64 = (s_37_3 as i64);
        // C s_37_5: const #1s : i
        let s_37_5: i128 = 1;
        // D s_37_6: cast zx s_37_4 -> i
        let s_37_6: i128 = (i128::try_from(s_37_4).unwrap());
        // D s_37_7: add s_37_6 s_37_5
        let s_37_7: i128 = (s_37_6 + s_37_5);
        // D s_37_8: cast reint s_37_7 -> i64
        let s_37_8: i64 = (s_37_7 as i64);
        // C s_37_9: const #16s : i64
        let s_37_9: i64 = 16;
        // D s_37_10: cast zx s_37_8 -> i
        let s_37_10: i128 = (i128::try_from(s_37_8).unwrap());
        // C s_37_11: cast zx s_37_9 -> i
        let s_37_11: i128 = (i128::try_from(s_37_9).unwrap());
        // D s_37_12: read-var operand1:bv
        let s_37_12: Bits = fn_state.operand1;
        // D s_37_13: call Elem_read(s_37_12, s_37_10, s_37_11)
        let s_37_13: Bits = Elem_read(state, tracer, s_37_12, s_37_10, s_37_11);
        // D s_37_14: cast reint s_37_13 -> u16
        let s_37_14: u16 = (s_37_13.value() as u16);
        // D s_37_15: write-var erow_1 <= s_37_14
        fn_state.erow_1 = s_37_14;
        // N s_37_16: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #2s : i
        let s_38_0: i128 = 2;
        // D s_38_1: read-var row:i64
        let s_38_1: i64 = fn_state.row;
        // D s_38_2: cast zx s_38_1 -> i
        let s_38_2: i128 = (i128::try_from(s_38_1).unwrap());
        // D s_38_3: mul s_38_0 s_38_2
        let s_38_3: i128 = ((s_38_0) * (s_38_2));
        // D s_38_4: cast reint s_38_3 -> i64
        let s_38_4: i64 = (s_38_3 as i64);
        // C s_38_5: const #0s : i
        let s_38_5: i128 = 0;
        // D s_38_6: cast zx s_38_4 -> i
        let s_38_6: i128 = (i128::try_from(s_38_4).unwrap());
        // D s_38_7: add s_38_6 s_38_5
        let s_38_7: i128 = (s_38_6 + s_38_5);
        // D s_38_8: cast reint s_38_7 -> i64
        let s_38_8: i64 = (s_38_7 as i64);
        // C s_38_9: const #16s : i64
        let s_38_9: i64 = 16;
        // D s_38_10: cast zx s_38_8 -> i
        let s_38_10: i128 = (i128::try_from(s_38_8).unwrap());
        // C s_38_11: cast zx s_38_9 -> i
        let s_38_11: i128 = (i128::try_from(s_38_9).unwrap());
        // D s_38_12: read-var operand1:bv
        let s_38_12: Bits = fn_state.operand1;
        // D s_38_13: call Elem_read(s_38_12, s_38_10, s_38_11)
        let s_38_13: Bits = Elem_read(state, tracer, s_38_12, s_38_10, s_38_11);
        // D s_38_14: cast reint s_38_13 -> u16
        let s_38_14: u16 = (s_38_13.value() as u16);
        // D s_38_15: write-var erow_0 <= s_38_14
        fn_state.erow_0 = s_38_14;
        // N s_38_16: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var pcol_1:u8
        let s_39_0: bool = fn_state.pcol_1;
        // D s_39_1: write-var gs#255099 <= s_39_0
        fn_state.gs_255099 = s_39_0;
        // N s_39_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#255100 <= s_40_0
        fn_state.gs_255100 = s_40_0;
        // N s_40_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var pcol_0:u8
        let s_41_0: bool = fn_state.pcol_0;
        // D s_41_1: write-var gs#255098 <= s_41_0
        fn_state.gs_255098 = s_41_0;
        // N s_41_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var row:i64
        let s_42_0: i64 = fn_state.row;
        // C s_42_1: const #1s : i64
        let s_42_1: i64 = 1;
        // D s_42_2: add s_42_0 s_42_1
        let s_42_2: i64 = (s_42_0 + s_42_1);
        // D s_42_3: write-var row <= s_42_2
        fn_state.row = s_42_2;
        // N s_42_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var VLshadow#5440:i64
        let s_43_0: i64 = fn_state.VLshadow_5440;
        // D s_43_1: cast zx s_43_0 -> i
        let s_43_1: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_2: cast reint s_43_1 -> i64
        let s_43_2: i64 = (s_43_1 as i64);
        // D s_43_3: read-var dim_dim_32shadow#5438:i64
        let s_43_3: i64 = fn_state.dim_dim_32shadow_5438;
        // D s_43_4: cast zx s_43_3 -> i
        let s_43_4: i128 = (i128::try_from(s_43_3).unwrap());
        // D s_43_5: cast reint s_43_4 -> i64
        let s_43_5: i64 = (s_43_4 as i64);
        // C s_43_6: const #32s : i
        let s_43_6: i128 = 32;
        // D s_43_7: cast zx s_43_2 -> i
        let s_43_7: i128 = (i128::try_from(s_43_2).unwrap());
        // D s_43_8: read-var da:i64
        let s_43_8: i64 = fn_state.da;
        // D s_43_9: cast zx s_43_8 -> i
        let s_43_9: i128 = (i128::try_from(s_43_8).unwrap());
        // D s_43_10: cast zx s_43_5 -> i
        let s_43_10: i128 = (i128::try_from(s_43_5).unwrap());
        // D s_43_11: read-var result:bv
        let s_43_11: Bits = fn_state.result;
        // D s_43_12: call ZAtile_set(s_43_7, s_43_9, s_43_6, s_43_10, s_43_11)
        let s_43_12: () = ZAtile_set(
            state,
            tracer,
            s_43_7,
            s_43_9,
            s_43_6,
            s_43_10,
            s_43_11,
        );
        // N s_43_13: return
        return;
    }
}
