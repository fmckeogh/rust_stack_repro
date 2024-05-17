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
pub fn execute_UZP_MZ_ZZ_2<T: Tracer>(
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
        result1: Bits,
        VLshadow_6793: i64,
        result0: Bits,
        gs_288353: i64,
        VLshadow_6794: i64,
        pairs: i64,
        esizeshadow_6792: i64,
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
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#6792 <= s_0_2
        fn_state.esizeshadow_6792 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6793 <= s_0_6
        fn_state.VLshadow_6793 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckStreamingSVEEnabled(s_0_8)
        let s_0_9: () = CheckStreamingSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#6793:i64
        let s_1_0: i64 = fn_state.VLshadow_6793;
        // D s_1_1: write-var VLshadow#6794 <= s_1_0
        fn_state.VLshadow_6794 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esizeshadow#6792:i64
        let s_1_3: i64 = fn_state.esizeshadow_6792;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) * (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#6794:i64
        let s_1_7: i64 = fn_state.VLshadow_6794;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: div s_1_8 s_1_9
        let s_1_10: i128 = ((s_1_8) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var pairs <= s_1_11
        fn_state.pairs = s_1_11;
        // C s_1_13: const #0s : i64
        let s_1_13: i64 = 0;
        // D s_1_14: write-var r <= s_1_13
        fn_state.r = s_1_13;
        // N s_1_15: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // C s_2_1: const #1s : i64
        let s_2_1: i64 = 1;
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
        // D s_3_0: read-var r:i64
        let s_3_0: i64 = fn_state.r;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var pairs:i64
        let s_3_2: i64 = fn_state.pairs;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: mul s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) * (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var base <= s_3_5
        fn_state.base = s_3_5;
        // C s_3_7: const #0s : i
        let s_3_7: i128 = 0;
        // D s_3_8: read-var r:i64
        let s_3_8: i64 = fn_state.r;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cmp-eq s_3_9 s_3_7
        let s_3_10: bool = ((s_3_9) == (s_3_7));
        // N s_3_11: branch s_3_10 b9 b4
        if s_3_10 {
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
        // D s_4_0: read-var VLshadow#6794:i64
        let s_4_0: i64 = fn_state.VLshadow_6794;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var m:i64
        let s_4_3: i64 = fn_state.m;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: call Z_read(s_4_4, s_4_5)
        let s_4_6: Bits = Z_read(state, tracer, s_4_4, s_4_5);
        // D s_4_7: write-var operand <= s_4_6
        fn_state.operand = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var pairs:i64
        let s_5_2: i64 = fn_state.pairs;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: sub s_5_3 s_5_1
        let s_5_4: i128 = ((s_5_3) - (s_5_1));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: write-var gs#288353 <= s_5_5
        fn_state.gs_288353 = s_5_5;
        // D s_5_7: write-var p <= s_5_0
        fn_state.p = s_5_0;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var p:i64
        let s_6_0: i64 = fn_state.p;
        // D s_6_1: read-var gs#288353:i64
        let s_6_1: i64 = fn_state.gs_288353;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b8 b7
        if s_6_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var base:i64
        let s_7_0: i64 = fn_state.base;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var p:i64
        let s_7_2: i64 = fn_state.p;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: i128 = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: read-var esizeshadow#6792:i64
        let s_7_6: i64 = fn_state.esizeshadow_6792;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // C s_7_9: const #2s : i
        let s_7_9: i128 = 2;
        // D s_7_10: read-var p:i64
        let s_7_10: i64 = fn_state.p;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: mul s_7_9 s_7_11
        let s_7_12: i128 = ((s_7_9) * (s_7_11));
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // C s_7_14: const #0s : i
        let s_7_14: i128 = 0;
        // D s_7_15: cast zx s_7_13 -> i
        let s_7_15: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_16: add s_7_15 s_7_14
        let s_7_16: i128 = (s_7_15 + s_7_14);
        // D s_7_17: cast reint s_7_16 -> i64
        let s_7_17: i64 = (s_7_16 as i64);
        // D s_7_18: read-var esizeshadow#6792:i64
        let s_7_18: i64 = fn_state.esizeshadow_6792;
        // D s_7_19: cast zx s_7_18 -> i
        let s_7_19: i128 = (i128::try_from(s_7_18).unwrap());
        // D s_7_20: cast reint s_7_19 -> i64
        let s_7_20: i64 = (s_7_19 as i64);
        // D s_7_21: cast zx s_7_17 -> i
        let s_7_21: i128 = (i128::try_from(s_7_17).unwrap());
        // D s_7_22: cast zx s_7_20 -> i
        let s_7_22: i128 = (i128::try_from(s_7_20).unwrap());
        // D s_7_23: read-var operand:bv
        let s_7_23: Bits = fn_state.operand;
        // D s_7_24: call Elem_read(s_7_23, s_7_21, s_7_22)
        let s_7_24: Bits = Elem_read(state, tracer, s_7_23, s_7_21, s_7_22);
        // D s_7_25: cast zx s_7_5 -> i
        let s_7_25: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_26: cast zx s_7_8 -> i
        let s_7_26: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_27: read-var result0:bv
        let s_7_27: Bits = fn_state.result0;
        // D s_7_28: call Elem_set(s_7_27, s_7_25, s_7_26, s_7_24)
        let s_7_28: Bits = Elem_set(state, tracer, s_7_27, s_7_25, s_7_26, s_7_24);
        // D s_7_29: write-var result0 <= s_7_28
        fn_state.result0 = s_7_28;
        // D s_7_30: read-var base:i64
        let s_7_30: i64 = fn_state.base;
        // D s_7_31: cast zx s_7_30 -> i
        let s_7_31: i128 = (i128::try_from(s_7_30).unwrap());
        // D s_7_32: read-var p:i64
        let s_7_32: i64 = fn_state.p;
        // D s_7_33: cast zx s_7_32 -> i
        let s_7_33: i128 = (i128::try_from(s_7_32).unwrap());
        // D s_7_34: add s_7_31 s_7_33
        let s_7_34: i128 = (s_7_31 + s_7_33);
        // D s_7_35: cast reint s_7_34 -> i64
        let s_7_35: i64 = (s_7_34 as i64);
        // D s_7_36: read-var esizeshadow#6792:i64
        let s_7_36: i64 = fn_state.esizeshadow_6792;
        // D s_7_37: cast zx s_7_36 -> i
        let s_7_37: i128 = (i128::try_from(s_7_36).unwrap());
        // D s_7_38: cast reint s_7_37 -> i64
        let s_7_38: i64 = (s_7_37 as i64);
        // C s_7_39: const #2s : i
        let s_7_39: i128 = 2;
        // D s_7_40: read-var p:i64
        let s_7_40: i64 = fn_state.p;
        // D s_7_41: cast zx s_7_40 -> i
        let s_7_41: i128 = (i128::try_from(s_7_40).unwrap());
        // D s_7_42: mul s_7_39 s_7_41
        let s_7_42: i128 = ((s_7_39) * (s_7_41));
        // D s_7_43: cast reint s_7_42 -> i64
        let s_7_43: i64 = (s_7_42 as i64);
        // C s_7_44: const #1s : i
        let s_7_44: i128 = 1;
        // D s_7_45: cast zx s_7_43 -> i
        let s_7_45: i128 = (i128::try_from(s_7_43).unwrap());
        // D s_7_46: add s_7_45 s_7_44
        let s_7_46: i128 = (s_7_45 + s_7_44);
        // D s_7_47: cast reint s_7_46 -> i64
        let s_7_47: i64 = (s_7_46 as i64);
        // D s_7_48: read-var esizeshadow#6792:i64
        let s_7_48: i64 = fn_state.esizeshadow_6792;
        // D s_7_49: cast zx s_7_48 -> i
        let s_7_49: i128 = (i128::try_from(s_7_48).unwrap());
        // D s_7_50: cast reint s_7_49 -> i64
        let s_7_50: i64 = (s_7_49 as i64);
        // D s_7_51: cast zx s_7_47 -> i
        let s_7_51: i128 = (i128::try_from(s_7_47).unwrap());
        // D s_7_52: cast zx s_7_50 -> i
        let s_7_52: i128 = (i128::try_from(s_7_50).unwrap());
        // D s_7_53: read-var operand:bv
        let s_7_53: Bits = fn_state.operand;
        // D s_7_54: call Elem_read(s_7_53, s_7_51, s_7_52)
        let s_7_54: Bits = Elem_read(state, tracer, s_7_53, s_7_51, s_7_52);
        // D s_7_55: cast zx s_7_35 -> i
        let s_7_55: i128 = (i128::try_from(s_7_35).unwrap());
        // D s_7_56: cast zx s_7_38 -> i
        let s_7_56: i128 = (i128::try_from(s_7_38).unwrap());
        // D s_7_57: read-var result1:bv
        let s_7_57: Bits = fn_state.result1;
        // D s_7_58: call Elem_set(s_7_57, s_7_55, s_7_56, s_7_54)
        let s_7_58: Bits = Elem_set(state, tracer, s_7_57, s_7_55, s_7_56, s_7_54);
        // D s_7_59: write-var result1 <= s_7_58
        fn_state.result1 = s_7_58;
        // D s_7_60: read-var p:i64
        let s_7_60: i64 = fn_state.p;
        // C s_7_61: const #1s : i64
        let s_7_61: i64 = 1;
        // D s_7_62: add s_7_60 s_7_61
        let s_7_62: i64 = (s_7_60 + s_7_61);
        // D s_7_63: write-var p <= s_7_62
        fn_state.p = s_7_62;
        // N s_7_64: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var r:i64
        let s_8_0: i64 = fn_state.r;
        // C s_8_1: const #1s : i64
        let s_8_1: i64 = 1;
        // D s_8_2: add s_8_0 s_8_1
        let s_8_2: i64 = (s_8_0 + s_8_1);
        // D s_8_3: write-var r <= s_8_2
        fn_state.r = s_8_2;
        // N s_8_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VLshadow#6794:i64
        let s_9_0: i64 = fn_state.VLshadow_6794;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var n:i64
        let s_9_3: i64 = fn_state.n;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast zx s_9_2 -> i
        let s_9_5: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_6: call Z_read(s_9_4, s_9_5)
        let s_9_6: Bits = Z_read(state, tracer, s_9_4, s_9_5);
        // D s_9_7: write-var operand <= s_9_6
        fn_state.operand = s_9_6;
        // N s_9_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var d:i64
        let s_10_1: i64 = fn_state.d;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: add s_10_2 s_10_0
        let s_10_3: i128 = (s_10_2 + s_10_0);
        // D s_10_4: cast reint s_10_3 -> i64
        let s_10_4: i64 = (s_10_3 as i64);
        // D s_10_5: read-var VLshadow#6794:i64
        let s_10_5: i64 = fn_state.VLshadow_6794;
        // D s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_7: cast reint s_10_6 -> i64
        let s_10_7: i64 = (s_10_6 as i64);
        // D s_10_8: cast zx s_10_4 -> i
        let s_10_8: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_9: cast zx s_10_7 -> i
        let s_10_9: i128 = (i128::try_from(s_10_7).unwrap());
        // D s_10_10: read-var result0:bv
        let s_10_10: Bits = fn_state.result0;
        // D s_10_11: call Z_set(s_10_8, s_10_9, s_10_10)
        let s_10_11: () = Z_set(state, tracer, s_10_8, s_10_9, s_10_10);
        // C s_10_12: const #1s : i
        let s_10_12: i128 = 1;
        // D s_10_13: read-var d:i64
        let s_10_13: i64 = fn_state.d;
        // D s_10_14: cast zx s_10_13 -> i
        let s_10_14: i128 = (i128::try_from(s_10_13).unwrap());
        // D s_10_15: add s_10_14 s_10_12
        let s_10_15: i128 = (s_10_14 + s_10_12);
        // D s_10_16: cast reint s_10_15 -> i64
        let s_10_16: i64 = (s_10_15 as i64);
        // D s_10_17: read-var VLshadow#6794:i64
        let s_10_17: i64 = fn_state.VLshadow_6794;
        // D s_10_18: cast zx s_10_17 -> i
        let s_10_18: i128 = (i128::try_from(s_10_17).unwrap());
        // D s_10_19: cast reint s_10_18 -> i64
        let s_10_19: i64 = (s_10_18 as i64);
        // D s_10_20: cast zx s_10_16 -> i
        let s_10_20: i128 = (i128::try_from(s_10_16).unwrap());
        // D s_10_21: cast zx s_10_19 -> i
        let s_10_21: i128 = (i128::try_from(s_10_19).unwrap());
        // D s_10_22: read-var result1:bv
        let s_10_22: Bits = fn_state.result1;
        // D s_10_23: call Z_set(s_10_20, s_10_21, s_10_22)
        let s_10_23: () = Z_set(state, tracer, s_10_20, s_10_21, s_10_22);
        // N s_10_24: return
        return;
    }
}
