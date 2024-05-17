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
use Elem_read::*;
use ZAtile_set::*;
use Z_read::*;
use BitCount::*;
use common::*;
pub fn execute_BMOPA_ZA_PP_ZZ_32<T: Tracer>(
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
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_288937: i64,
        dim_dim_esizeshadow_6815: i64,
        element3: Bits,
        operand3: Bits,
        gs_288946: bool,
        row: i64,
        res: i128,
        gs_288943: i64,
        element2: Bits,
        VLshadow_6817: i64,
        element1: Bits,
        col: i64,
        dim: i64,
        VLshadow_6816: i64,
        result: Bits,
        mask1: Bits,
        operand1: Bits,
        mask2: Bits,
        operand2: Bits,
        VL: i64,
        dim_dim_esize: i64,
        a: i64,
        b: i64,
        da: i64,
        esize: i64,
        m: i64,
        n: i64,
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
        // D s_0_3: write-var dim_dim_esizeshadow#6815 <= s_0_2
        fn_state.dim_dim_esizeshadow_6815 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6816 <= s_0_6
        fn_state.VLshadow_6816 = s_0_6;
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
        // D s_1_0: read-var VLshadow#6816:i64
        let s_1_0: i64 = fn_state.VLshadow_6816;
        // D s_1_1: write-var VLshadow#6817 <= s_1_0
        fn_state.VLshadow_6817 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#6817:i64
        let s_1_3: i64 = fn_state.VLshadow_6817;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#6817:i64
        let s_1_7: i64 = fn_state.VLshadow_6817;
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
        // D s_1_28: read-var VLshadow#6817:i64
        let s_1_28: i64 = fn_state.VLshadow_6817;
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
        // D s_1_36: read-var VLshadow#6817:i64
        let s_1_36: i64 = fn_state.VLshadow_6817;
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
        // D s_1_44: read-var VLshadow#6817:i64
        let s_1_44: i64 = fn_state.VLshadow_6817;
        // D s_1_45: cast zx s_1_44 -> i
        let s_1_45: i128 = (i128::try_from(s_1_44).unwrap());
        // D s_1_46: cast reint s_1_45 -> i64
        let s_1_46: i64 = (s_1_45 as i64);
        // D s_1_47: read-var dim_dim_esizeshadow#6815:i64
        let s_1_47: i64 = fn_state.dim_dim_esizeshadow_6815;
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
        // D s_1_64: write-var gs#288937 <= s_1_63
        fn_state.gs_288937 = s_1_63;
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
        // D s_2_1: read-var gs#288937:i64
        let s_2_1: i64 = fn_state.gs_288937;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b16 b3
        if s_2_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esize:i64
        let s_3_0: i64 = fn_state.esize;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var row:i64
        let s_3_3: i64 = fn_state.row;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand1:bv
        let s_3_6: Bits = fn_state.operand1;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: write-var element1 <= s_3_7
        fn_state.element1 = s_3_7;
        // C s_3_9: const #0s : i64
        let s_3_9: i64 = 0;
        // C s_3_10: const #1s : i
        let s_3_10: i128 = 1;
        // D s_3_11: read-var dim:i64
        let s_3_11: i64 = fn_state.dim;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: sub s_3_12 s_3_10
        let s_3_13: i128 = ((s_3_12) - (s_3_10));
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: write-var gs#288943 <= s_3_14
        fn_state.gs_288943 = s_3_14;
        // D s_3_16: write-var col <= s_3_9
        fn_state.col = s_3_9;
        // N s_3_17: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var col:i64
        let s_4_0: i64 = fn_state.col;
        // D s_4_1: read-var gs#288943:i64
        let s_4_1: i64 = fn_state.gs_288943;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b15 b5
        if s_4_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esize:i64
        let s_5_0: i64 = fn_state.esize;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var col:i64
        let s_5_3: i64 = fn_state.col;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var operand2:bv
        let s_5_6: Bits = fn_state.operand2;
        // D s_5_7: call Elem_read(s_5_6, s_5_4, s_5_5)
        let s_5_7: Bits = Elem_read(state, tracer, s_5_6, s_5_4, s_5_5);
        // D s_5_8: write-var element2 <= s_5_7
        fn_state.element2 = s_5_7;
        // D s_5_9: read-var row:i64
        let s_5_9: i64 = fn_state.row;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: read-var dim:i64
        let s_5_11: i64 = fn_state.dim;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: mul s_5_10 s_5_12
        let s_5_13: i128 = ((s_5_10) * (s_5_12));
        // D s_5_14: cast reint s_5_13 -> i64
        let s_5_14: i64 = (s_5_13 as i64);
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: read-var col:i64
        let s_5_16: i64 = fn_state.col;
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_18: add s_5_15 s_5_17
        let s_5_18: i128 = (s_5_15 + s_5_17);
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // D s_5_20: read-var esize:i64
        let s_5_20: i64 = fn_state.esize;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: cast zx s_5_19 -> i
        let s_5_23: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_24: cast zx s_5_22 -> i
        let s_5_24: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_25: read-var operand3:bv
        let s_5_25: Bits = fn_state.operand3;
        // D s_5_26: call Elem_read(s_5_25, s_5_23, s_5_24)
        let s_5_26: Bits = Elem_read(state, tracer, s_5_25, s_5_23, s_5_24);
        // D s_5_27: write-var element3 <= s_5_26
        fn_state.element3 = s_5_26;
        // D s_5_28: read-var row:i64
        let s_5_28: i64 = fn_state.row;
        // D s_5_29: cast zx s_5_28 -> i
        let s_5_29: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_30: read-var esize:i64
        let s_5_30: i64 = fn_state.esize;
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_32: read-var mask1:bv
        let s_5_32: Bits = fn_state.mask1;
        // D s_5_33: call ActivePredicateElement(s_5_32, s_5_29, s_5_31)
        let s_5_33: bool = ActivePredicateElement(state, tracer, s_5_32, s_5_29, s_5_31);
        // N s_5_34: branch s_5_33 b14 b6
        if s_5_33 {
            return block_14(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#288946 <= s_6_0
        fn_state.gs_288946 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#288946:u8
        let s_7_0: bool = fn_state.gs_288946;
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
        // D s_8_11: read-var esize:i64
        let s_8_11: i64 = fn_state.esize;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: cast reint s_8_12 -> i64
        let s_8_13: i64 = (s_8_12 as i64);
        // D s_8_14: cast zx s_8_10 -> i
        let s_8_14: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_15: cast zx s_8_13 -> i
        let s_8_15: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_16: read-var result:bv
        let s_8_16: Bits = fn_state.result;
        // D s_8_17: read-var element3:bv
        let s_8_17: Bits = fn_state.element3;
        // D s_8_18: call Elem_set(s_8_16, s_8_14, s_8_15, s_8_17)
        let s_8_18: Bits = Elem_set(state, tracer, s_8_16, s_8_14, s_8_15, s_8_17);
        // D s_8_19: write-var result <= s_8_18
        fn_state.result = s_8_18;
        // N s_8_20: jump b9
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
        // D s_10_0: read-var element1:bv
        let s_10_0: Bits = fn_state.element1;
        // D s_10_1: read-var element2:bv
        let s_10_1: Bits = fn_state.element2;
        // D s_10_2: xor s_10_0 s_10_1
        let s_10_2: Bits = ((s_10_0) ^ (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: Bits = !s_10_2;
        // D s_10_4: call BitCount(s_10_3)
        let s_10_4: i128 = BitCount(state, tracer, s_10_3);
        // D s_10_5: write-var res <= s_10_4
        fn_state.res = s_10_4;
        // D s_10_6: read-var sub_op:u8
        let s_10_6: bool = fn_state.sub_op;
        // N s_10_7: branch s_10_6 b13 b11
        if s_10_6 {
            return block_13(state, tracer, fn_state);
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
        // D s_12_0: read-var res:i
        let s_12_0: i128 = fn_state.res;
        // D s_12_1: read-var row:i64
        let s_12_1: i64 = fn_state.row;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: read-var dim:i64
        let s_12_3: i64 = fn_state.dim;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: mul s_12_2 s_12_4
        let s_12_5: i128 = ((s_12_2) * (s_12_4));
        // D s_12_6: cast reint s_12_5 -> i64
        let s_12_6: i64 = (s_12_5 as i64);
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: read-var col:i64
        let s_12_8: i64 = fn_state.col;
        // D s_12_9: cast zx s_12_8 -> i
        let s_12_9: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_10: add s_12_7 s_12_9
        let s_12_10: i128 = (s_12_7 + s_12_9);
        // D s_12_11: cast reint s_12_10 -> i64
        let s_12_11: i64 = (s_12_10 as i64);
        // D s_12_12: read-var esize:i64
        let s_12_12: i64 = fn_state.esize;
        // D s_12_13: cast zx s_12_12 -> i
        let s_12_13: i128 = (i128::try_from(s_12_12).unwrap());
        // D s_12_14: cast reint s_12_13 -> i64
        let s_12_14: i64 = (s_12_13 as i64);
        // D s_12_15: read-var element3:bv
        let s_12_15: Bits = fn_state.element3;
        // D s_12_16: cast cvt s_12_0 -> bv
        let s_12_16: Bits = Bits::new(s_12_0 as u128, 128);
        // D s_12_17: add s_12_15 s_12_16
        let s_12_17: Bits = (s_12_15 + s_12_16);
        // D s_12_18: cast reint s_12_17 -> u32
        let s_12_18: u32 = (s_12_17.value() as u32);
        // D s_12_19: cast zx s_12_11 -> i
        let s_12_19: i128 = (i128::try_from(s_12_11).unwrap());
        // D s_12_20: cast zx s_12_14 -> i
        let s_12_20: i128 = (i128::try_from(s_12_14).unwrap());
        // D s_12_21: cast zx s_12_18 -> bv
        let s_12_21: Bits = Bits::new(s_12_18 as u128, 32u16);
        // D s_12_22: read-var result:bv
        let s_12_22: Bits = fn_state.result;
        // D s_12_23: call Elem_set(s_12_22, s_12_19, s_12_20, s_12_21)
        let s_12_23: Bits = Elem_set(state, tracer, s_12_22, s_12_19, s_12_20, s_12_21);
        // D s_12_24: write-var result <= s_12_23
        fn_state.result = s_12_23;
        // N s_12_25: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var res:i
        let s_13_0: i128 = fn_state.res;
        // D s_13_1: neg s_13_0
        let s_13_1: i128 = -s_13_0;
        // D s_13_2: write-var res <= s_13_1
        fn_state.res = s_13_1;
        // N s_13_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var col:i64
        let s_14_0: i64 = fn_state.col;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var esize:i64
        let s_14_2: i64 = fn_state.esize;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: read-var mask2:bv
        let s_14_4: Bits = fn_state.mask2;
        // D s_14_5: call ActivePredicateElement(s_14_4, s_14_1, s_14_3)
        let s_14_5: bool = ActivePredicateElement(state, tracer, s_14_4, s_14_1, s_14_3);
        // D s_14_6: write-var gs#288946 <= s_14_5
        fn_state.gs_288946 = s_14_5;
        // N s_14_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var row:i64
        let s_15_0: i64 = fn_state.row;
        // C s_15_1: const #1s : i64
        let s_15_1: i64 = 1;
        // D s_15_2: add s_15_0 s_15_1
        let s_15_2: i64 = (s_15_0 + s_15_1);
        // D s_15_3: write-var row <= s_15_2
        fn_state.row = s_15_2;
        // N s_15_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var VLshadow#6817:i64
        let s_16_0: i64 = fn_state.VLshadow_6817;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: cast reint s_16_1 -> i64
        let s_16_2: i64 = (s_16_1 as i64);
        // D s_16_3: read-var dim_dim_esizeshadow#6815:i64
        let s_16_3: i64 = fn_state.dim_dim_esizeshadow_6815;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: cast reint s_16_4 -> i64
        let s_16_5: i64 = (s_16_4 as i64);
        // D s_16_6: cast zx s_16_2 -> i
        let s_16_6: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_7: read-var da:i64
        let s_16_7: i64 = fn_state.da;
        // D s_16_8: cast zx s_16_7 -> i
        let s_16_8: i128 = (i128::try_from(s_16_7).unwrap());
        // D s_16_9: read-var esize:i64
        let s_16_9: i64 = fn_state.esize;
        // D s_16_10: cast zx s_16_9 -> i
        let s_16_10: i128 = (i128::try_from(s_16_9).unwrap());
        // D s_16_11: cast zx s_16_5 -> i
        let s_16_11: i128 = (i128::try_from(s_16_5).unwrap());
        // D s_16_12: read-var result:bv
        let s_16_12: Bits = fn_state.result;
        // D s_16_13: call ZAtile_set(s_16_6, s_16_8, s_16_10, s_16_11, s_16_12)
        let s_16_13: () = ZAtile_set(
            state,
            tracer,
            s_16_6,
            s_16_8,
            s_16_10,
            s_16_11,
            s_16_12,
        );
        // N s_16_14: return
        return;
    }
}
