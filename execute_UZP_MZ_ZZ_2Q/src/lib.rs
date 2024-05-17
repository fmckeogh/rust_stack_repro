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
use CheckStreamingSVEEnabled::*;
use Elem_read::*;
use Elem_set::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_UZP_MZ_ZZ_2Q<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        r: i64,
        p: i64,
        base: i64,
        gs_288475: i64,
        result1: Bits,
        VLshadow_6797: i64,
        result0: Bits,
        VLshadow_6798: i64,
        pairs: i64,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        n,
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
        // D s_0_3: write-var VLshadow#6797 <= s_0_2
        fn_state.VLshadow_6797 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#6797:i64
        let s_1_0: i64 = fn_state.VLshadow_6797;
        // D s_1_1: write-var VLshadow#6798 <= s_1_0
        fn_state.VLshadow_6798 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esize:i64
        let s_1_3: i64 = fn_state.esize;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) * (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#6798:i64
        let s_1_7: i64 = fn_state.VLshadow_6798;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: cmp-lt s_1_8 s_1_9
        let s_1_10: bool = ((s_1_8) < (s_1_9));
        // N s_1_11: branch s_1_10 b12 b2
        if s_1_10 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #2s : i
        let s_2_0: i128 = 2;
        // D s_2_1: read-var esize:i64
        let s_2_1: i64 = fn_state.esize;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: mul s_2_2 s_2_0
        let s_2_3: i128 = ((s_2_2) * (s_2_0));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: read-var VLshadow#6798:i64
        let s_2_5: i64 = fn_state.VLshadow_6798;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: cast zx s_2_4 -> i
        let s_2_7: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_8: div s_2_6 s_2_7
        let s_2_8: i128 = ((s_2_6) / (s_2_7));
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var pairs <= s_2_9
        fn_state.pairs = s_2_9;
        // C s_2_11: const #0s : i64
        let s_2_11: i64 = 0;
        // D s_2_12: write-var r <= s_2_11
        fn_state.r = s_2_11;
        // N s_2_13: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var r:i64
        let s_3_0: i64 = fn_state.r;
        // C s_3_1: const #1s : i64
        let s_3_1: i64 = 1;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b11 b4
        if s_3_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:i64
        let s_4_0: i64 = fn_state.r;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var pairs:i64
        let s_4_2: i64 = fn_state.pairs;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: mul s_4_1 s_4_3
        let s_4_4: i128 = ((s_4_1) * (s_4_3));
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: write-var base <= s_4_5
        fn_state.base = s_4_5;
        // C s_4_7: const #0s : i
        let s_4_7: i128 = 0;
        // D s_4_8: read-var r:i64
        let s_4_8: i64 = fn_state.r;
        // D s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_10: cmp-eq s_4_9 s_4_7
        let s_4_10: bool = ((s_4_9) == (s_4_7));
        // N s_4_11: branch s_4_10 b10 b5
        if s_4_10 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VLshadow#6798:i64
        let s_5_0: i64 = fn_state.VLshadow_6798;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var m:i64
        let s_5_3: i64 = fn_state.m;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: call Z_read(s_5_4, s_5_5)
        let s_5_6: Bits = Z_read(state, tracer, s_5_4, s_5_5);
        // D s_5_7: write-var operand <= s_5_6
        fn_state.operand = s_5_6;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i64
        let s_6_0: i64 = 0;
        // C s_6_1: const #1s : i
        let s_6_1: i128 = 1;
        // D s_6_2: read-var pairs:i64
        let s_6_2: i64 = fn_state.pairs;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: sub s_6_3 s_6_1
        let s_6_4: i128 = ((s_6_3) - (s_6_1));
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: write-var gs#288475 <= s_6_5
        fn_state.gs_288475 = s_6_5;
        // D s_6_7: write-var p <= s_6_0
        fn_state.p = s_6_0;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var p:i64
        let s_7_0: i64 = fn_state.p;
        // D s_7_1: read-var gs#288475:i64
        let s_7_1: i64 = fn_state.gs_288475;
        // D s_7_2: cmp-gt s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) > (s_7_1));
        // N s_7_3: branch s_7_2 b9 b8
        if s_7_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var base:i64
        let s_8_0: i64 = fn_state.base;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var p:i64
        let s_8_2: i64 = fn_state.p;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: add s_8_1 s_8_3
        let s_8_4: i128 = (s_8_1 + s_8_3);
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: read-var esize:i64
        let s_8_6: i64 = fn_state.esize;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: cast reint s_8_7 -> i64
        let s_8_8: i64 = (s_8_7 as i64);
        // C s_8_9: const #2s : i
        let s_8_9: i128 = 2;
        // D s_8_10: read-var p:i64
        let s_8_10: i64 = fn_state.p;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: mul s_8_9 s_8_11
        let s_8_12: i128 = ((s_8_9) * (s_8_11));
        // D s_8_13: cast reint s_8_12 -> i64
        let s_8_13: i64 = (s_8_12 as i64);
        // C s_8_14: const #0s : i
        let s_8_14: i128 = 0;
        // D s_8_15: cast zx s_8_13 -> i
        let s_8_15: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_16: add s_8_15 s_8_14
        let s_8_16: i128 = (s_8_15 + s_8_14);
        // D s_8_17: cast reint s_8_16 -> i64
        let s_8_17: i64 = (s_8_16 as i64);
        // D s_8_18: read-var esize:i64
        let s_8_18: i64 = fn_state.esize;
        // D s_8_19: cast zx s_8_18 -> i
        let s_8_19: i128 = (i128::try_from(s_8_18).unwrap());
        // D s_8_20: cast reint s_8_19 -> i64
        let s_8_20: i64 = (s_8_19 as i64);
        // D s_8_21: cast zx s_8_17 -> i
        let s_8_21: i128 = (i128::try_from(s_8_17).unwrap());
        // D s_8_22: cast zx s_8_20 -> i
        let s_8_22: i128 = (i128::try_from(s_8_20).unwrap());
        // D s_8_23: read-var operand:bv
        let s_8_23: Bits = fn_state.operand;
        // D s_8_24: call Elem_read(s_8_23, s_8_21, s_8_22)
        let s_8_24: Bits = Elem_read(state, tracer, s_8_23, s_8_21, s_8_22);
        // D s_8_25: cast reint s_8_24 -> u128
        let s_8_25: u128 = (s_8_24.value() as u128);
        // D s_8_26: cast zx s_8_5 -> i
        let s_8_26: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_27: cast zx s_8_8 -> i
        let s_8_27: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_28: cast zx s_8_25 -> bv
        let s_8_28: Bits = Bits::new(s_8_25 as u128, 128u16);
        // D s_8_29: read-var result0:bv
        let s_8_29: Bits = fn_state.result0;
        // D s_8_30: call Elem_set(s_8_29, s_8_26, s_8_27, s_8_28)
        let s_8_30: Bits = Elem_set(state, tracer, s_8_29, s_8_26, s_8_27, s_8_28);
        // D s_8_31: write-var result0 <= s_8_30
        fn_state.result0 = s_8_30;
        // D s_8_32: read-var base:i64
        let s_8_32: i64 = fn_state.base;
        // D s_8_33: cast zx s_8_32 -> i
        let s_8_33: i128 = (i128::try_from(s_8_32).unwrap());
        // D s_8_34: read-var p:i64
        let s_8_34: i64 = fn_state.p;
        // D s_8_35: cast zx s_8_34 -> i
        let s_8_35: i128 = (i128::try_from(s_8_34).unwrap());
        // D s_8_36: add s_8_33 s_8_35
        let s_8_36: i128 = (s_8_33 + s_8_35);
        // D s_8_37: cast reint s_8_36 -> i64
        let s_8_37: i64 = (s_8_36 as i64);
        // D s_8_38: read-var esize:i64
        let s_8_38: i64 = fn_state.esize;
        // D s_8_39: cast zx s_8_38 -> i
        let s_8_39: i128 = (i128::try_from(s_8_38).unwrap());
        // D s_8_40: cast reint s_8_39 -> i64
        let s_8_40: i64 = (s_8_39 as i64);
        // C s_8_41: const #2s : i
        let s_8_41: i128 = 2;
        // D s_8_42: read-var p:i64
        let s_8_42: i64 = fn_state.p;
        // D s_8_43: cast zx s_8_42 -> i
        let s_8_43: i128 = (i128::try_from(s_8_42).unwrap());
        // D s_8_44: mul s_8_41 s_8_43
        let s_8_44: i128 = ((s_8_41) * (s_8_43));
        // D s_8_45: cast reint s_8_44 -> i64
        let s_8_45: i64 = (s_8_44 as i64);
        // C s_8_46: const #1s : i
        let s_8_46: i128 = 1;
        // D s_8_47: cast zx s_8_45 -> i
        let s_8_47: i128 = (i128::try_from(s_8_45).unwrap());
        // D s_8_48: add s_8_47 s_8_46
        let s_8_48: i128 = (s_8_47 + s_8_46);
        // D s_8_49: cast reint s_8_48 -> i64
        let s_8_49: i64 = (s_8_48 as i64);
        // D s_8_50: read-var esize:i64
        let s_8_50: i64 = fn_state.esize;
        // D s_8_51: cast zx s_8_50 -> i
        let s_8_51: i128 = (i128::try_from(s_8_50).unwrap());
        // D s_8_52: cast reint s_8_51 -> i64
        let s_8_52: i64 = (s_8_51 as i64);
        // D s_8_53: cast zx s_8_49 -> i
        let s_8_53: i128 = (i128::try_from(s_8_49).unwrap());
        // D s_8_54: cast zx s_8_52 -> i
        let s_8_54: i128 = (i128::try_from(s_8_52).unwrap());
        // D s_8_55: read-var operand:bv
        let s_8_55: Bits = fn_state.operand;
        // D s_8_56: call Elem_read(s_8_55, s_8_53, s_8_54)
        let s_8_56: Bits = Elem_read(state, tracer, s_8_55, s_8_53, s_8_54);
        // D s_8_57: cast reint s_8_56 -> u128
        let s_8_57: u128 = (s_8_56.value() as u128);
        // D s_8_58: cast zx s_8_37 -> i
        let s_8_58: i128 = (i128::try_from(s_8_37).unwrap());
        // D s_8_59: cast zx s_8_40 -> i
        let s_8_59: i128 = (i128::try_from(s_8_40).unwrap());
        // D s_8_60: cast zx s_8_57 -> bv
        let s_8_60: Bits = Bits::new(s_8_57 as u128, 128u16);
        // D s_8_61: read-var result1:bv
        let s_8_61: Bits = fn_state.result1;
        // D s_8_62: call Elem_set(s_8_61, s_8_58, s_8_59, s_8_60)
        let s_8_62: Bits = Elem_set(state, tracer, s_8_61, s_8_58, s_8_59, s_8_60);
        // D s_8_63: write-var result1 <= s_8_62
        fn_state.result1 = s_8_62;
        // D s_8_64: read-var p:i64
        let s_8_64: i64 = fn_state.p;
        // C s_8_65: const #1s : i64
        let s_8_65: i64 = 1;
        // D s_8_66: add s_8_64 s_8_65
        let s_8_66: i64 = (s_8_64 + s_8_65);
        // D s_8_67: write-var p <= s_8_66
        fn_state.p = s_8_66;
        // N s_8_68: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r:i64
        let s_9_0: i64 = fn_state.r;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var r <= s_9_2
        fn_state.r = s_9_2;
        // N s_9_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#6798:i64
        let s_10_0: i64 = fn_state.VLshadow_6798;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var n:i64
        let s_10_3: i64 = fn_state.n;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: call Z_read(s_10_4, s_10_5)
        let s_10_6: Bits = Z_read(state, tracer, s_10_4, s_10_5);
        // D s_10_7: write-var operand <= s_10_6
        fn_state.operand = s_10_6;
        // N s_10_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var d:i64
        let s_11_1: i64 = fn_state.d;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: add s_11_2 s_11_0
        let s_11_3: i128 = (s_11_2 + s_11_0);
        // D s_11_4: cast reint s_11_3 -> i64
        let s_11_4: i64 = (s_11_3 as i64);
        // D s_11_5: read-var VLshadow#6798:i64
        let s_11_5: i64 = fn_state.VLshadow_6798;
        // D s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_7: cast reint s_11_6 -> i64
        let s_11_7: i64 = (s_11_6 as i64);
        // D s_11_8: cast zx s_11_4 -> i
        let s_11_8: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_9: cast zx s_11_7 -> i
        let s_11_9: i128 = (i128::try_from(s_11_7).unwrap());
        // D s_11_10: read-var result0:bv
        let s_11_10: Bits = fn_state.result0;
        // D s_11_11: call Z_set(s_11_8, s_11_9, s_11_10)
        let s_11_11: () = Z_set(state, tracer, s_11_8, s_11_9, s_11_10);
        // C s_11_12: const #1s : i
        let s_11_12: i128 = 1;
        // D s_11_13: read-var d:i64
        let s_11_13: i64 = fn_state.d;
        // D s_11_14: cast zx s_11_13 -> i
        let s_11_14: i128 = (i128::try_from(s_11_13).unwrap());
        // D s_11_15: add s_11_14 s_11_12
        let s_11_15: i128 = (s_11_14 + s_11_12);
        // D s_11_16: cast reint s_11_15 -> i64
        let s_11_16: i64 = (s_11_15 as i64);
        // D s_11_17: read-var VLshadow#6798:i64
        let s_11_17: i64 = fn_state.VLshadow_6798;
        // D s_11_18: cast zx s_11_17 -> i
        let s_11_18: i128 = (i128::try_from(s_11_17).unwrap());
        // D s_11_19: cast reint s_11_18 -> i64
        let s_11_19: i64 = (s_11_18 as i64);
        // D s_11_20: cast zx s_11_16 -> i
        let s_11_20: i128 = (i128::try_from(s_11_16).unwrap());
        // D s_11_21: cast zx s_11_19 -> i
        let s_11_21: i128 = (i128::try_from(s_11_19).unwrap());
        // D s_11_22: read-var result1:bv
        let s_11_22: Bits = fn_state.result1;
        // D s_11_23: call Z_set(s_11_20, s_11_21, s_11_22)
        let s_11_23: () = Z_set(state, tracer, s_11_20, s_11_21, s_11_22);
        // N s_11_24: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
}
