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
pub fn execute_ZIP_MZ_ZZ_2Q<T: Tracer>(
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
        r: i64,
        gs_288415: i64,
        p: i64,
        VLshadow_6795: i64,
        base: i64,
        operand0: Bits,
        VLshadow_6796: i64,
        pairs: i64,
        result: Bits,
        operand1: Bits,
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
        // D s_0_3: write-var VLshadow#6795 <= s_0_2
        fn_state.VLshadow_6795 = s_0_2;
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
        // D s_1_0: read-var VLshadow#6795:i64
        let s_1_0: i64 = fn_state.VLshadow_6795;
        // D s_1_1: write-var VLshadow#6796 <= s_1_0
        fn_state.VLshadow_6796 = s_1_0;
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
        // D s_1_7: read-var VLshadow#6796:i64
        let s_1_7: i64 = fn_state.VLshadow_6796;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> i
        let s_1_9: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_10: cmp-lt s_1_8 s_1_9
        let s_1_10: bool = ((s_1_8) < (s_1_9));
        // N s_1_11: branch s_1_10 b9 b2
        if s_1_10 {
            return block_9(state, tracer, fn_state);
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
        // D s_2_5: read-var VLshadow#6796:i64
        let s_2_5: i64 = fn_state.VLshadow_6796;
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
        // D s_2_11: read-var VLshadow#6796:i64
        let s_2_11: i64 = fn_state.VLshadow_6796;
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: read-var n:i64
        let s_2_14: i64 = fn_state.n;
        // D s_2_15: cast zx s_2_14 -> i
        let s_2_15: i128 = (i128::try_from(s_2_14).unwrap());
        // D s_2_16: cast zx s_2_13 -> i
        let s_2_16: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_17: call Z_read(s_2_15, s_2_16)
        let s_2_17: Bits = Z_read(state, tracer, s_2_15, s_2_16);
        // D s_2_18: write-var operand0 <= s_2_17
        fn_state.operand0 = s_2_17;
        // D s_2_19: read-var VLshadow#6796:i64
        let s_2_19: i64 = fn_state.VLshadow_6796;
        // D s_2_20: cast zx s_2_19 -> i
        let s_2_20: i128 = (i128::try_from(s_2_19).unwrap());
        // D s_2_21: cast reint s_2_20 -> i64
        let s_2_21: i64 = (s_2_20 as i64);
        // D s_2_22: read-var m:i64
        let s_2_22: i64 = fn_state.m;
        // D s_2_23: cast zx s_2_22 -> i
        let s_2_23: i128 = (i128::try_from(s_2_22).unwrap());
        // D s_2_24: cast zx s_2_21 -> i
        let s_2_24: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_25: call Z_read(s_2_23, s_2_24)
        let s_2_25: Bits = Z_read(state, tracer, s_2_23, s_2_24);
        // D s_2_26: write-var operand1 <= s_2_25
        fn_state.operand1 = s_2_25;
        // C s_2_27: const #0s : i64
        let s_2_27: i64 = 0;
        // D s_2_28: write-var r <= s_2_27
        fn_state.r = s_2_27;
        // N s_2_29: jump b3
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
        // N s_3_3: branch s_3_2 b8 b4
        if s_3_2 {
            return block_8(state, tracer, fn_state);
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
        // C s_4_7: const #0s : i64
        let s_4_7: i64 = 0;
        // C s_4_8: const #1s : i
        let s_4_8: i128 = 1;
        // D s_4_9: read-var pairs:i64
        let s_4_9: i64 = fn_state.pairs;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: sub s_4_10 s_4_8
        let s_4_11: i128 = ((s_4_10) - (s_4_8));
        // D s_4_12: cast reint s_4_11 -> i64
        let s_4_12: i64 = (s_4_11 as i64);
        // D s_4_13: write-var gs#288415 <= s_4_12
        fn_state.gs_288415 = s_4_12;
        // D s_4_14: write-var p <= s_4_7
        fn_state.p = s_4_7;
        // N s_4_15: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var p:i64
        let s_5_0: i64 = fn_state.p;
        // D s_5_1: read-var gs#288415:i64
        let s_5_1: i64 = fn_state.gs_288415;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b7 b6
        if s_5_2 {
            return block_7(state, tracer, fn_state);
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
        // C s_6_5: const #0s : i
        let s_6_5: i128 = 0;
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
        // D s_6_12: read-var base:i64
        let s_6_12: i64 = fn_state.base;
        // D s_6_13: cast zx s_6_12 -> i
        let s_6_13: i128 = (i128::try_from(s_6_12).unwrap());
        // D s_6_14: read-var p:i64
        let s_6_14: i64 = fn_state.p;
        // D s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (i128::try_from(s_6_14).unwrap());
        // D s_6_16: add s_6_13 s_6_15
        let s_6_16: i128 = (s_6_13 + s_6_15);
        // D s_6_17: cast reint s_6_16 -> i64
        let s_6_17: i64 = (s_6_16 as i64);
        // D s_6_18: read-var esize:i64
        let s_6_18: i64 = fn_state.esize;
        // D s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (i128::try_from(s_6_18).unwrap());
        // D s_6_20: cast reint s_6_19 -> i64
        let s_6_20: i64 = (s_6_19 as i64);
        // D s_6_21: cast zx s_6_17 -> i
        let s_6_21: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_22: cast zx s_6_20 -> i
        let s_6_22: i128 = (i128::try_from(s_6_20).unwrap());
        // D s_6_23: read-var operand0:bv
        let s_6_23: Bits = fn_state.operand0;
        // D s_6_24: call Elem_read(s_6_23, s_6_21, s_6_22)
        let s_6_24: Bits = Elem_read(state, tracer, s_6_23, s_6_21, s_6_22);
        // D s_6_25: cast reint s_6_24 -> u128
        let s_6_25: u128 = (s_6_24.value() as u128);
        // D s_6_26: cast zx s_6_8 -> i
        let s_6_26: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_27: cast zx s_6_11 -> i
        let s_6_27: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_28: cast zx s_6_25 -> bv
        let s_6_28: Bits = Bits::new(s_6_25 as u128, 128u16);
        // D s_6_29: read-var result:bv
        let s_6_29: Bits = fn_state.result;
        // D s_6_30: call Elem_set(s_6_29, s_6_26, s_6_27, s_6_28)
        let s_6_30: Bits = Elem_set(state, tracer, s_6_29, s_6_26, s_6_27, s_6_28);
        // D s_6_31: write-var result <= s_6_30
        fn_state.result = s_6_30;
        // C s_6_32: const #2s : i
        let s_6_32: i128 = 2;
        // D s_6_33: read-var p:i64
        let s_6_33: i64 = fn_state.p;
        // D s_6_34: cast zx s_6_33 -> i
        let s_6_34: i128 = (i128::try_from(s_6_33).unwrap());
        // D s_6_35: mul s_6_32 s_6_34
        let s_6_35: i128 = ((s_6_32) * (s_6_34));
        // D s_6_36: cast reint s_6_35 -> i64
        let s_6_36: i64 = (s_6_35 as i64);
        // C s_6_37: const #1s : i
        let s_6_37: i128 = 1;
        // D s_6_38: cast zx s_6_36 -> i
        let s_6_38: i128 = (i128::try_from(s_6_36).unwrap());
        // D s_6_39: add s_6_38 s_6_37
        let s_6_39: i128 = (s_6_38 + s_6_37);
        // D s_6_40: cast reint s_6_39 -> i64
        let s_6_40: i64 = (s_6_39 as i64);
        // D s_6_41: read-var esize:i64
        let s_6_41: i64 = fn_state.esize;
        // D s_6_42: cast zx s_6_41 -> i
        let s_6_42: i128 = (i128::try_from(s_6_41).unwrap());
        // D s_6_43: cast reint s_6_42 -> i64
        let s_6_43: i64 = (s_6_42 as i64);
        // D s_6_44: read-var base:i64
        let s_6_44: i64 = fn_state.base;
        // D s_6_45: cast zx s_6_44 -> i
        let s_6_45: i128 = (i128::try_from(s_6_44).unwrap());
        // D s_6_46: read-var p:i64
        let s_6_46: i64 = fn_state.p;
        // D s_6_47: cast zx s_6_46 -> i
        let s_6_47: i128 = (i128::try_from(s_6_46).unwrap());
        // D s_6_48: add s_6_45 s_6_47
        let s_6_48: i128 = (s_6_45 + s_6_47);
        // D s_6_49: cast reint s_6_48 -> i64
        let s_6_49: i64 = (s_6_48 as i64);
        // D s_6_50: read-var esize:i64
        let s_6_50: i64 = fn_state.esize;
        // D s_6_51: cast zx s_6_50 -> i
        let s_6_51: i128 = (i128::try_from(s_6_50).unwrap());
        // D s_6_52: cast reint s_6_51 -> i64
        let s_6_52: i64 = (s_6_51 as i64);
        // D s_6_53: cast zx s_6_49 -> i
        let s_6_53: i128 = (i128::try_from(s_6_49).unwrap());
        // D s_6_54: cast zx s_6_52 -> i
        let s_6_54: i128 = (i128::try_from(s_6_52).unwrap());
        // D s_6_55: read-var operand1:bv
        let s_6_55: Bits = fn_state.operand1;
        // D s_6_56: call Elem_read(s_6_55, s_6_53, s_6_54)
        let s_6_56: Bits = Elem_read(state, tracer, s_6_55, s_6_53, s_6_54);
        // D s_6_57: cast reint s_6_56 -> u128
        let s_6_57: u128 = (s_6_56.value() as u128);
        // D s_6_58: cast zx s_6_40 -> i
        let s_6_58: i128 = (i128::try_from(s_6_40).unwrap());
        // D s_6_59: cast zx s_6_43 -> i
        let s_6_59: i128 = (i128::try_from(s_6_43).unwrap());
        // D s_6_60: cast zx s_6_57 -> bv
        let s_6_60: Bits = Bits::new(s_6_57 as u128, 128u16);
        // D s_6_61: read-var result:bv
        let s_6_61: Bits = fn_state.result;
        // D s_6_62: call Elem_set(s_6_61, s_6_58, s_6_59, s_6_60)
        let s_6_62: Bits = Elem_set(state, tracer, s_6_61, s_6_58, s_6_59, s_6_60);
        // D s_6_63: write-var result <= s_6_62
        fn_state.result = s_6_62;
        // D s_6_64: read-var p:i64
        let s_6_64: i64 = fn_state.p;
        // C s_6_65: const #1s : i64
        let s_6_65: i64 = 1;
        // D s_6_66: add s_6_64 s_6_65
        let s_6_66: i64 = (s_6_64 + s_6_65);
        // D s_6_67: write-var p <= s_6_66
        fn_state.p = s_6_66;
        // N s_6_68: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var d:i64
        let s_7_0: i64 = fn_state.d;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var r:i64
        let s_7_2: i64 = fn_state.r;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: i128 = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: read-var VLshadow#6796:i64
        let s_7_6: i64 = fn_state.VLshadow_6796;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // D s_7_9: cast zx s_7_5 -> i
        let s_7_9: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_10: cast zx s_7_8 -> i
        let s_7_10: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_11: read-var result:bv
        let s_7_11: Bits = fn_state.result;
        // D s_7_12: call Z_set(s_7_9, s_7_10, s_7_11)
        let s_7_12: () = Z_set(state, tracer, s_7_9, s_7_10, s_7_11);
        // D s_7_13: read-var r:i64
        let s_7_13: i64 = fn_state.r;
        // C s_7_14: const #1s : i64
        let s_7_14: i64 = 1;
        // D s_7_15: add s_7_13 s_7_14
        let s_7_15: i64 = (s_7_13 + s_7_14);
        // D s_7_16: write-var r <= s_7_15
        fn_state.r = s_7_15;
        // N s_7_17: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
}
