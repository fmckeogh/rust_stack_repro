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
use CheckStreamingSVEAndZAEnabled::*;
use ZAtile_read::*;
use ActivePredicateElement::*;
use P_read::*;
use asl_Int::*;
use Elem_read::*;
use ZAtile_set::*;
use Z_read::*;
use common::*;
pub fn execute_UMOPA_ZA_PP_ZZ_64<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dim_dim_esize: i64,
    a: i64,
    b: i64,
    da: i64,
    esize: i64,
    m: i64,
    n: i64,
    op1_unsigned: bool,
    op2_unsigned: bool,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        sum: Bits,
        k: i64,
        operand3: Bits,
        row: i64,
        VLshadow_5476: i64,
        gs_256144: i64,
        prod: i128,
        gs_256138: i64,
        col: i64,
        dim: i64,
        dim_dim_esizeshadow_5474: i64,
        VLshadow_5475: i64,
        gs_256157: bool,
        result: Bits,
        mask1: Bits,
        operand1: Bits,
        operand2: Bits,
        mask2: Bits,
        VL: i64,
        dim_dim_esize: i64,
        a: i64,
        b: i64,
        da: i64,
        esize: i64,
        m: i64,
        n: i64,
        op1_unsigned: bool,
        op2_unsigned: bool,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        VL,
        dim_dim_esize,
        a,
        b,
        da,
        esize,
        m,
        n,
        op1_unsigned,
        op2_unsigned,
        sub_op,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var dim_dim_esize:i64
        let s_0_0: i64 = fn_state.dim_dim_esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var dim_dim_esizeshadow#5474 <= s_0_2
        fn_state.dim_dim_esizeshadow_5474 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#5475 <= s_0_6
        fn_state.VLshadow_5475 = s_0_6;
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
        // D s_1_0: read-var VLshadow#5475:i64
        let s_1_0: i64 = fn_state.VLshadow_5475;
        // D s_1_1: write-var VLshadow#5476 <= s_1_0
        fn_state.VLshadow_5476 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5476:i64
        let s_1_3: i64 = fn_state.VLshadow_5476;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#5476:i64
        let s_1_7: i64 = fn_state.VLshadow_5476;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var dim <= s_1_12
        fn_state.dim = s_1_12;
        // D s_1_14: cast zx s_1_6 -> i
        let s_1_14: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var a:i64
        let s_1_16: i64 = fn_state.a;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var mask1 <= s_1_19
        fn_state.mask1 = s_1_19;
        // D s_1_21: cast zx s_1_6 -> i
        let s_1_21: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var b:i64
        let s_1_23: i64 = fn_state.b;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call P_read(s_1_24, s_1_25)
        let s_1_26: Bits = P_read(state, tracer, s_1_24, s_1_25);
        // D s_1_27: write-var mask2 <= s_1_26
        fn_state.mask2 = s_1_26;
        // D s_1_28: read-var VLshadow#5476:i64
        let s_1_28: i64 = fn_state.VLshadow_5476;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: read-var n:i64
        let s_1_31: i64 = fn_state.n;
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: cast zx s_1_30 -> i
        let s_1_33: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_34: call Z_read(s_1_32, s_1_33)
        let s_1_34: Bits = Z_read(state, tracer, s_1_32, s_1_33);
        // D s_1_35: write-var operand1 <= s_1_34
        fn_state.operand1 = s_1_34;
        // D s_1_36: read-var VLshadow#5476:i64
        let s_1_36: i64 = fn_state.VLshadow_5476;
        // D s_1_37: cast zx s_1_36 -> i
        let s_1_37: i128 = (i128::try_from(s_1_36).unwrap());
        // D s_1_38: cast reint s_1_37 -> i64
        let s_1_38: i64 = (s_1_37 as i64);
        // D s_1_39: read-var m:i64
        let s_1_39: i64 = fn_state.m;
        // D s_1_40: cast zx s_1_39 -> i
        let s_1_40: i128 = (i128::try_from(s_1_39).unwrap());
        // D s_1_41: cast zx s_1_38 -> i
        let s_1_41: i128 = (i128::try_from(s_1_38).unwrap());
        // D s_1_42: call Z_read(s_1_40, s_1_41)
        let s_1_42: Bits = Z_read(state, tracer, s_1_40, s_1_41);
        // D s_1_43: write-var operand2 <= s_1_42
        fn_state.operand2 = s_1_42;
        // D s_1_44: read-var VLshadow#5476:i64
        let s_1_44: i64 = fn_state.VLshadow_5476;
        // D s_1_45: cast zx s_1_44 -> i
        let s_1_45: i128 = (i128::try_from(s_1_44).unwrap());
        // D s_1_46: cast reint s_1_45 -> i64
        let s_1_46: i64 = (s_1_45 as i64);
        // D s_1_47: read-var dim_dim_esizeshadow#5474:i64
        let s_1_47: i64 = fn_state.dim_dim_esizeshadow_5474;
        // D s_1_48: cast zx s_1_47 -> i
        let s_1_48: i128 = (i128::try_from(s_1_47).unwrap());
        // D s_1_49: cast reint s_1_48 -> i64
        let s_1_49: i64 = (s_1_48 as i64);
        // D s_1_50: cast zx s_1_46 -> i
        let s_1_50: i128 = (i128::try_from(s_1_46).unwrap());
        // D s_1_51: read-var da:i64
        let s_1_51: i64 = fn_state.da;
        // D s_1_52: cast zx s_1_51 -> i
        let s_1_52: i128 = (i128::try_from(s_1_51).unwrap());
        // D s_1_53: read-var esize:i64
        let s_1_53: i64 = fn_state.esize;
        // D s_1_54: cast zx s_1_53 -> i
        let s_1_54: i128 = (i128::try_from(s_1_53).unwrap());
        // D s_1_55: cast zx s_1_49 -> i
        let s_1_55: i128 = (i128::try_from(s_1_49).unwrap());
        // D s_1_56: call ZAtile_read(s_1_50, s_1_52, s_1_54, s_1_55)
        let s_1_56: Bits = ZAtile_read(state, tracer, s_1_50, s_1_52, s_1_54, s_1_55);
        // D s_1_57: write-var operand3 <= s_1_56
        fn_state.operand3 = s_1_56;
        // C s_1_58: const #0s : i64
        let s_1_58: i64 = 0;
        // C s_1_59: const #1s : i
        let s_1_59: i128 = 1;
        // D s_1_60: read-var dim:i64
        let s_1_60: i64 = fn_state.dim;
        // D s_1_61: cast zx s_1_60 -> i
        let s_1_61: i128 = (i128::try_from(s_1_60).unwrap());
        // D s_1_62: sub s_1_61 s_1_59
        let s_1_62: i128 = ((s_1_61) - (s_1_59));
        // D s_1_63: cast reint s_1_62 -> i64
        let s_1_63: i64 = (s_1_62 as i64);
        // D s_1_64: write-var gs#256138 <= s_1_63
        fn_state.gs_256138 = s_1_63;
        // D s_1_65: write-var row <= s_1_58
        fn_state.row = s_1_58;
        // N s_1_66: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var row:i64
        let s_2_0: i64 = fn_state.row;
        // D s_2_1: read-var gs#256138:i64
        let s_2_1: i64 = fn_state.gs_256138;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b19 b3
        if s_2_2 {
            return block_19(state, tracer, fn_state);
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
        // D s_3_6: write-var gs#256144 <= s_3_5
        fn_state.gs_256144 = s_3_5;
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
        // D s_4_1: read-var gs#256144:i64
        let s_4_1: i64 = fn_state.gs_256144;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b18 b5
        if s_4_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var row:i64
        let s_5_0: i64 = fn_state.row;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var dim:i64
        let s_5_2: i64 = fn_state.dim;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: mul s_5_1 s_5_3
        let s_5_4: i128 = ((s_5_1) * (s_5_3));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: read-var col:i64
        let s_5_7: i64 = fn_state.col;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: add s_5_6 s_5_8
        let s_5_9: i128 = (s_5_6 + s_5_8);
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: read-var esize:i64
        let s_5_11: i64 = fn_state.esize;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: cast zx s_5_10 -> i
        let s_5_14: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_15: cast zx s_5_13 -> i
        let s_5_15: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_16: read-var operand3:bv
        let s_5_16: Bits = fn_state.operand3;
        // D s_5_17: call Elem_read(s_5_16, s_5_14, s_5_15)
        let s_5_17: Bits = Elem_read(state, tracer, s_5_16, s_5_14, s_5_15);
        // D s_5_18: write-var sum <= s_5_17
        fn_state.sum = s_5_17;
        // C s_5_19: const #0s : i64
        let s_5_19: i64 = 0;
        // D s_5_20: write-var k <= s_5_19
        fn_state.k = s_5_19;
        // N s_5_21: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var k:i64
        let s_6_0: i64 = fn_state.k;
        // C s_6_1: const #3s : i64
        let s_6_1: i64 = 3;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b17 b7
        if s_6_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #4s : i
        let s_7_0: i128 = 4;
        // D s_7_1: read-var row:i64
        let s_7_1: i64 = fn_state.row;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var k:i64
        let s_7_6: i64 = fn_state.k;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: add s_7_5 s_7_7
        let s_7_8: i128 = (s_7_5 + s_7_7);
        // D s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // C s_7_10: const #4s : i
        let s_7_10: i128 = 4;
        // D s_7_11: read-var esize:i64
        let s_7_11: i64 = fn_state.esize;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: div s_7_12 s_7_10
        let s_7_13: i128 = ((s_7_12) / (s_7_10));
        // D s_7_14: cast reint s_7_13 -> i64
        let s_7_14: i64 = (s_7_13 as i64);
        // D s_7_15: cast zx s_7_9 -> i
        let s_7_15: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_16: cast zx s_7_14 -> i
        let s_7_16: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_17: read-var mask1:bv
        let s_7_17: Bits = fn_state.mask1;
        // D s_7_18: call ActivePredicateElement(s_7_17, s_7_15, s_7_16)
        let s_7_18: bool = ActivePredicateElement(state, tracer, s_7_17, s_7_15, s_7_16);
        // N s_7_19: branch s_7_18 b16 b8
        if s_7_18 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#256157 <= s_8_0
        fn_state.gs_256157 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#256157:u8
        let s_9_0: bool = fn_state.gs_256157;
        // N s_9_1: branch s_9_0 b12 b10
        if s_9_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var k:i64
        let s_11_0: i64 = fn_state.k;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var k <= s_11_2
        fn_state.k = s_11_2;
        // N s_11_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #4s : i
        let s_12_0: i128 = 4;
        // D s_12_1: read-var row:i64
        let s_12_1: i64 = fn_state.row;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: mul s_12_0 s_12_2
        let s_12_3: i128 = ((s_12_0) * (s_12_2));
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: read-var k:i64
        let s_12_6: i64 = fn_state.k;
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: add s_12_5 s_12_7
        let s_12_8: i128 = (s_12_5 + s_12_7);
        // D s_12_9: cast reint s_12_8 -> i64
        let s_12_9: i64 = (s_12_8 as i64);
        // C s_12_10: const #4s : i
        let s_12_10: i128 = 4;
        // D s_12_11: read-var esize:i64
        let s_12_11: i64 = fn_state.esize;
        // D s_12_12: cast zx s_12_11 -> i
        let s_12_12: i128 = (i128::try_from(s_12_11).unwrap());
        // D s_12_13: div s_12_12 s_12_10
        let s_12_13: i128 = ((s_12_12) / (s_12_10));
        // D s_12_14: cast reint s_12_13 -> i64
        let s_12_14: i64 = (s_12_13 as i64);
        // D s_12_15: cast zx s_12_14 -> i
        let s_12_15: i128 = (i128::try_from(s_12_14).unwrap());
        // D s_12_16: cast reint s_12_15 -> i64
        let s_12_16: i64 = (s_12_15 as i64);
        // D s_12_17: cast zx s_12_9 -> i
        let s_12_17: i128 = (i128::try_from(s_12_9).unwrap());
        // D s_12_18: cast zx s_12_16 -> i
        let s_12_18: i128 = (i128::try_from(s_12_16).unwrap());
        // D s_12_19: read-var operand1:bv
        let s_12_19: Bits = fn_state.operand1;
        // D s_12_20: call Elem_read(s_12_19, s_12_17, s_12_18)
        let s_12_20: Bits = Elem_read(state, tracer, s_12_19, s_12_17, s_12_18);
        // D s_12_21: cast reint s_12_20 -> u16
        let s_12_21: u16 = (s_12_20.value() as u16);
        // D s_12_22: cast zx s_12_21 -> bv
        let s_12_22: Bits = Bits::new(s_12_21 as u128, 16u16);
        // D s_12_23: read-var op1_unsigned:u8
        let s_12_23: bool = fn_state.op1_unsigned;
        // D s_12_24: call asl_Int(s_12_22, s_12_23)
        let s_12_24: i128 = asl_Int(state, tracer, s_12_22, s_12_23);
        // C s_12_25: const #4s : i
        let s_12_25: i128 = 4;
        // D s_12_26: read-var col:i64
        let s_12_26: i64 = fn_state.col;
        // D s_12_27: cast zx s_12_26 -> i
        let s_12_27: i128 = (i128::try_from(s_12_26).unwrap());
        // D s_12_28: mul s_12_25 s_12_27
        let s_12_28: i128 = ((s_12_25) * (s_12_27));
        // D s_12_29: cast reint s_12_28 -> i64
        let s_12_29: i64 = (s_12_28 as i64);
        // D s_12_30: cast zx s_12_29 -> i
        let s_12_30: i128 = (i128::try_from(s_12_29).unwrap());
        // D s_12_31: read-var k:i64
        let s_12_31: i64 = fn_state.k;
        // D s_12_32: cast zx s_12_31 -> i
        let s_12_32: i128 = (i128::try_from(s_12_31).unwrap());
        // D s_12_33: add s_12_30 s_12_32
        let s_12_33: i128 = (s_12_30 + s_12_32);
        // D s_12_34: cast reint s_12_33 -> i64
        let s_12_34: i64 = (s_12_33 as i64);
        // C s_12_35: const #4s : i
        let s_12_35: i128 = 4;
        // D s_12_36: read-var esize:i64
        let s_12_36: i64 = fn_state.esize;
        // D s_12_37: cast zx s_12_36 -> i
        let s_12_37: i128 = (i128::try_from(s_12_36).unwrap());
        // D s_12_38: div s_12_37 s_12_35
        let s_12_38: i128 = ((s_12_37) / (s_12_35));
        // D s_12_39: cast reint s_12_38 -> i64
        let s_12_39: i64 = (s_12_38 as i64);
        // D s_12_40: cast zx s_12_39 -> i
        let s_12_40: i128 = (i128::try_from(s_12_39).unwrap());
        // D s_12_41: cast reint s_12_40 -> i64
        let s_12_41: i64 = (s_12_40 as i64);
        // D s_12_42: cast zx s_12_34 -> i
        let s_12_42: i128 = (i128::try_from(s_12_34).unwrap());
        // D s_12_43: cast zx s_12_41 -> i
        let s_12_43: i128 = (i128::try_from(s_12_41).unwrap());
        // D s_12_44: read-var operand2:bv
        let s_12_44: Bits = fn_state.operand2;
        // D s_12_45: call Elem_read(s_12_44, s_12_42, s_12_43)
        let s_12_45: Bits = Elem_read(state, tracer, s_12_44, s_12_42, s_12_43);
        // D s_12_46: cast reint s_12_45 -> u16
        let s_12_46: u16 = (s_12_45.value() as u16);
        // D s_12_47: cast zx s_12_46 -> bv
        let s_12_47: Bits = Bits::new(s_12_46 as u128, 16u16);
        // D s_12_48: read-var op2_unsigned:u8
        let s_12_48: bool = fn_state.op2_unsigned;
        // D s_12_49: call asl_Int(s_12_47, s_12_48)
        let s_12_49: i128 = asl_Int(state, tracer, s_12_47, s_12_48);
        // D s_12_50: mul s_12_24 s_12_49
        let s_12_50: i128 = ((s_12_24) * (s_12_49));
        // D s_12_51: write-var prod <= s_12_50
        fn_state.prod = s_12_50;
        // D s_12_52: read-var sub_op:u8
        let s_12_52: bool = fn_state.sub_op;
        // N s_12_53: branch s_12_52 b15 b13
        if s_12_52 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var sum:bv
        let s_14_0: Bits = fn_state.sum;
        // D s_14_1: read-var prod:i
        let s_14_1: i128 = fn_state.prod;
        // D s_14_2: cast cvt s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 128);
        // D s_14_3: add s_14_0 s_14_2
        let s_14_3: Bits = (s_14_0 + s_14_2);
        // D s_14_4: write-var sum <= s_14_3
        fn_state.sum = s_14_3;
        // N s_14_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var prod:i
        let s_15_0: i128 = fn_state.prod;
        // D s_15_1: neg s_15_0
        let s_15_1: i128 = -s_15_0;
        // D s_15_2: write-var prod <= s_15_1
        fn_state.prod = s_15_1;
        // N s_15_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #4s : i
        let s_16_0: i128 = 4;
        // D s_16_1: read-var col:i64
        let s_16_1: i64 = fn_state.col;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: mul s_16_0 s_16_2
        let s_16_3: i128 = ((s_16_0) * (s_16_2));
        // D s_16_4: cast reint s_16_3 -> i64
        let s_16_4: i64 = (s_16_3 as i64);
        // D s_16_5: cast zx s_16_4 -> i
        let s_16_5: i128 = (i128::try_from(s_16_4).unwrap());
        // D s_16_6: read-var k:i64
        let s_16_6: i64 = fn_state.k;
        // D s_16_7: cast zx s_16_6 -> i
        let s_16_7: i128 = (i128::try_from(s_16_6).unwrap());
        // D s_16_8: add s_16_5 s_16_7
        let s_16_8: i128 = (s_16_5 + s_16_7);
        // D s_16_9: cast reint s_16_8 -> i64
        let s_16_9: i64 = (s_16_8 as i64);
        // C s_16_10: const #4s : i
        let s_16_10: i128 = 4;
        // D s_16_11: read-var esize:i64
        let s_16_11: i64 = fn_state.esize;
        // D s_16_12: cast zx s_16_11 -> i
        let s_16_12: i128 = (i128::try_from(s_16_11).unwrap());
        // D s_16_13: div s_16_12 s_16_10
        let s_16_13: i128 = ((s_16_12) / (s_16_10));
        // D s_16_14: cast reint s_16_13 -> i64
        let s_16_14: i64 = (s_16_13 as i64);
        // D s_16_15: cast zx s_16_9 -> i
        let s_16_15: i128 = (i128::try_from(s_16_9).unwrap());
        // D s_16_16: cast zx s_16_14 -> i
        let s_16_16: i128 = (i128::try_from(s_16_14).unwrap());
        // D s_16_17: read-var mask2:bv
        let s_16_17: Bits = fn_state.mask2;
        // D s_16_18: call ActivePredicateElement(s_16_17, s_16_15, s_16_16)
        let s_16_18: bool = ActivePredicateElement(
            state,
            tracer,
            s_16_17,
            s_16_15,
            s_16_16,
        );
        // D s_16_19: write-var gs#256157 <= s_16_18
        fn_state.gs_256157 = s_16_18;
        // N s_16_20: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var row:i64
        let s_17_0: i64 = fn_state.row;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var dim:i64
        let s_17_2: i64 = fn_state.dim;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: mul s_17_1 s_17_3
        let s_17_4: i128 = ((s_17_1) * (s_17_3));
        // D s_17_5: cast reint s_17_4 -> i64
        let s_17_5: i64 = (s_17_4 as i64);
        // D s_17_6: cast zx s_17_5 -> i
        let s_17_6: i128 = (i128::try_from(s_17_5).unwrap());
        // D s_17_7: read-var col:i64
        let s_17_7: i64 = fn_state.col;
        // D s_17_8: cast zx s_17_7 -> i
        let s_17_8: i128 = (i128::try_from(s_17_7).unwrap());
        // D s_17_9: add s_17_6 s_17_8
        let s_17_9: i128 = (s_17_6 + s_17_8);
        // D s_17_10: cast reint s_17_9 -> i64
        let s_17_10: i64 = (s_17_9 as i64);
        // D s_17_11: read-var esize:i64
        let s_17_11: i64 = fn_state.esize;
        // D s_17_12: cast zx s_17_11 -> i
        let s_17_12: i128 = (i128::try_from(s_17_11).unwrap());
        // D s_17_13: cast reint s_17_12 -> i64
        let s_17_13: i64 = (s_17_12 as i64);
        // D s_17_14: cast zx s_17_10 -> i
        let s_17_14: i128 = (i128::try_from(s_17_10).unwrap());
        // D s_17_15: cast zx s_17_13 -> i
        let s_17_15: i128 = (i128::try_from(s_17_13).unwrap());
        // D s_17_16: read-var result:bv
        let s_17_16: Bits = fn_state.result;
        // D s_17_17: read-var sum:bv
        let s_17_17: Bits = fn_state.sum;
        // D s_17_18: call Elem_set(s_17_16, s_17_14, s_17_15, s_17_17)
        let s_17_18: Bits = Elem_set(state, tracer, s_17_16, s_17_14, s_17_15, s_17_17);
        // D s_17_19: write-var result <= s_17_18
        fn_state.result = s_17_18;
        // D s_17_20: read-var col:i64
        let s_17_20: i64 = fn_state.col;
        // C s_17_21: const #1s : i64
        let s_17_21: i64 = 1;
        // D s_17_22: add s_17_20 s_17_21
        let s_17_22: i64 = (s_17_20 + s_17_21);
        // D s_17_23: write-var col <= s_17_22
        fn_state.col = s_17_22;
        // N s_17_24: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var row:i64
        let s_18_0: i64 = fn_state.row;
        // C s_18_1: const #1s : i64
        let s_18_1: i64 = 1;
        // D s_18_2: add s_18_0 s_18_1
        let s_18_2: i64 = (s_18_0 + s_18_1);
        // D s_18_3: write-var row <= s_18_2
        fn_state.row = s_18_2;
        // N s_18_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var VLshadow#5476:i64
        let s_19_0: i64 = fn_state.VLshadow_5476;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: cast reint s_19_1 -> i64
        let s_19_2: i64 = (s_19_1 as i64);
        // D s_19_3: read-var dim_dim_esizeshadow#5474:i64
        let s_19_3: i64 = fn_state.dim_dim_esizeshadow_5474;
        // D s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_5: cast reint s_19_4 -> i64
        let s_19_5: i64 = (s_19_4 as i64);
        // D s_19_6: cast zx s_19_2 -> i
        let s_19_6: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_7: read-var da:i64
        let s_19_7: i64 = fn_state.da;
        // D s_19_8: cast zx s_19_7 -> i
        let s_19_8: i128 = (i128::try_from(s_19_7).unwrap());
        // D s_19_9: read-var esize:i64
        let s_19_9: i64 = fn_state.esize;
        // D s_19_10: cast zx s_19_9 -> i
        let s_19_10: i128 = (i128::try_from(s_19_9).unwrap());
        // D s_19_11: cast zx s_19_5 -> i
        let s_19_11: i128 = (i128::try_from(s_19_5).unwrap());
        // D s_19_12: read-var result:bv
        let s_19_12: Bits = fn_state.result;
        // D s_19_13: call ZAtile_set(s_19_6, s_19_8, s_19_10, s_19_11, s_19_12)
        let s_19_13: () = ZAtile_set(
            state,
            tracer,
            s_19_6,
            s_19_8,
            s_19_10,
            s_19_11,
            s_19_12,
        );
        // N s_19_14: return
        return;
    }
}
