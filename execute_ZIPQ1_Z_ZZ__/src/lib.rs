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
pub fn execute_ZIPQ1_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        p: i64,
        VLshadow_4367: i64,
        base: i64,
        gs_220509: i64,
        VLshadow_4368: i64,
        gs_220516: i64,
        s: i64,
        esizeshadow_4366: i64,
        pairs: i64,
        elements: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        n,
        part,
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
        // D s_0_3: write-var esizeshadow#4366 <= s_0_2
        fn_state.esizeshadow_4366 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4367 <= s_0_6
        fn_state.VLshadow_4367 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4367:i64
        let s_1_0: i64 = fn_state.VLshadow_4367;
        // D s_1_1: write-var VLshadow#4368 <= s_1_0
        fn_state.VLshadow_4368 = s_1_0;
        // C s_1_2: const #128s : i
        let s_1_2: i128 = 128;
        // D s_1_3: read-var VLshadow#4368:i64
        let s_1_3: i64 = fn_state.VLshadow_4368;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #128s : i
        let s_1_7: i128 = 128;
        // D s_1_8: read-var esizeshadow#4366:i64
        let s_1_8: i64 = fn_state.esizeshadow_4366;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: div s_1_7 s_1_9
        let s_1_10: i128 = ((s_1_7) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var elements <= s_1_11
        fn_state.elements = s_1_11;
        // C s_1_13: const #2s : i
        let s_1_13: i128 = 2;
        // D s_1_14: read-var elements:i64
        let s_1_14: i64 = fn_state.elements;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: div s_1_15 s_1_13
        let s_1_16: i128 = ((s_1_15) / (s_1_13));
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: write-var pairs <= s_1_17
        fn_state.pairs = s_1_17;
        // D s_1_19: read-var VLshadow#4368:i64
        let s_1_19: i64 = fn_state.VLshadow_4368;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast reint s_1_20 -> i64
        let s_1_21: i64 = (s_1_20 as i64);
        // D s_1_22: read-var n:i64
        let s_1_22: i64 = fn_state.n;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast zx s_1_21 -> i
        let s_1_24: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_25: call Z_read(s_1_23, s_1_24)
        let s_1_25: Bits = Z_read(state, tracer, s_1_23, s_1_24);
        // D s_1_26: write-var operand1 <= s_1_25
        fn_state.operand1 = s_1_25;
        // D s_1_27: read-var VLshadow#4368:i64
        let s_1_27: i64 = fn_state.VLshadow_4368;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: cast reint s_1_28 -> i64
        let s_1_29: i64 = (s_1_28 as i64);
        // D s_1_30: read-var m:i64
        let s_1_30: i64 = fn_state.m;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: cast zx s_1_29 -> i
        let s_1_32: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_33: call Z_read(s_1_31, s_1_32)
        let s_1_33: Bits = Z_read(state, tracer, s_1_31, s_1_32);
        // D s_1_34: write-var operand2 <= s_1_33
        fn_state.operand2 = s_1_33;
        // C s_1_35: const #0s : i64
        let s_1_35: i64 = 0;
        // C s_1_36: const #1s : i
        let s_1_36: i128 = 1;
        // D s_1_37: cast zx s_1_6 -> i
        let s_1_37: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_38: sub s_1_37 s_1_36
        let s_1_38: i128 = ((s_1_37) - (s_1_36));
        // D s_1_39: cast reint s_1_38 -> i64
        let s_1_39: i64 = (s_1_38 as i64);
        // D s_1_40: write-var gs#220509 <= s_1_39
        fn_state.gs_220509 = s_1_39;
        // D s_1_41: write-var s <= s_1_35
        fn_state.s = s_1_35;
        // N s_1_42: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var s:i64
        let s_2_0: i64 = fn_state.s;
        // D s_2_1: read-var gs#220509:i64
        let s_2_1: i64 = fn_state.gs_220509;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var s:i64
        let s_3_0: i64 = fn_state.s;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: mul s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) * (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var part:i64
        let s_3_6: i64 = fn_state.part;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: read-var pairs:i64
        let s_3_8: i64 = fn_state.pairs;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: mul s_3_7 s_3_9
        let s_3_10: i128 = ((s_3_7) * (s_3_9));
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: cast zx s_3_5 -> i
        let s_3_12: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_13: cast zx s_3_11 -> i
        let s_3_13: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_14: add s_3_12 s_3_13
        let s_3_14: i128 = (s_3_12 + s_3_13);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: write-var base <= s_3_15
        fn_state.base = s_3_15;
        // C s_3_17: const #0s : i64
        let s_3_17: i64 = 0;
        // C s_3_18: const #1s : i
        let s_3_18: i128 = 1;
        // D s_3_19: read-var pairs:i64
        let s_3_19: i64 = fn_state.pairs;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: sub s_3_20 s_3_18
        let s_3_21: i128 = ((s_3_20) - (s_3_18));
        // D s_3_22: cast reint s_3_21 -> i64
        let s_3_22: i64 = (s_3_21 as i64);
        // D s_3_23: write-var gs#220516 <= s_3_22
        fn_state.gs_220516 = s_3_22;
        // D s_3_24: write-var p <= s_3_17
        fn_state.p = s_3_17;
        // N s_3_25: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var p:i64
        let s_4_0: i64 = fn_state.p;
        // D s_4_1: read-var gs#220516:i64
        let s_4_1: i64 = fn_state.gs_220516;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var s:i64
        let s_5_0: i64 = fn_state.s;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var elements:i64
        let s_5_2: i64 = fn_state.elements;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: mul s_5_1 s_5_3
        let s_5_4: i128 = ((s_5_1) * (s_5_3));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // C s_5_6: const #2s : i
        let s_5_6: i128 = 2;
        // D s_5_7: read-var p:i64
        let s_5_7: i64 = fn_state.p;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: mul s_5_6 s_5_8
        let s_5_9: i128 = ((s_5_6) * (s_5_8));
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_5 -> i
        let s_5_11: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_12: cast zx s_5_10 -> i
        let s_5_12: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_13: add s_5_11 s_5_12
        let s_5_13: i128 = (s_5_11 + s_5_12);
        // D s_5_14: cast reint s_5_13 -> i64
        let s_5_14: i64 = (s_5_13 as i64);
        // C s_5_15: const #0s : i
        let s_5_15: i128 = 0;
        // D s_5_16: cast zx s_5_14 -> i
        let s_5_16: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_17: add s_5_16 s_5_15
        let s_5_17: i128 = (s_5_16 + s_5_15);
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // D s_5_19: read-var esizeshadow#4366:i64
        let s_5_19: i64 = fn_state.esizeshadow_4366;
        // D s_5_20: cast zx s_5_19 -> i
        let s_5_20: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_21: cast reint s_5_20 -> i64
        let s_5_21: i64 = (s_5_20 as i64);
        // D s_5_22: read-var base:i64
        let s_5_22: i64 = fn_state.base;
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_24: read-var p:i64
        let s_5_24: i64 = fn_state.p;
        // D s_5_25: cast zx s_5_24 -> i
        let s_5_25: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_26: add s_5_23 s_5_25
        let s_5_26: i128 = (s_5_23 + s_5_25);
        // D s_5_27: cast reint s_5_26 -> i64
        let s_5_27: i64 = (s_5_26 as i64);
        // D s_5_28: read-var esizeshadow#4366:i64
        let s_5_28: i64 = fn_state.esizeshadow_4366;
        // D s_5_29: cast zx s_5_28 -> i
        let s_5_29: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_30: cast reint s_5_29 -> i64
        let s_5_30: i64 = (s_5_29 as i64);
        // D s_5_31: cast zx s_5_27 -> i
        let s_5_31: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_32: cast zx s_5_30 -> i
        let s_5_32: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_33: read-var operand1:bv
        let s_5_33: Bits = fn_state.operand1;
        // D s_5_34: call Elem_read(s_5_33, s_5_31, s_5_32)
        let s_5_34: Bits = Elem_read(state, tracer, s_5_33, s_5_31, s_5_32);
        // D s_5_35: cast zx s_5_18 -> i
        let s_5_35: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_36: cast zx s_5_21 -> i
        let s_5_36: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_37: read-var result:bv
        let s_5_37: Bits = fn_state.result;
        // D s_5_38: call Elem_set(s_5_37, s_5_35, s_5_36, s_5_34)
        let s_5_38: Bits = Elem_set(state, tracer, s_5_37, s_5_35, s_5_36, s_5_34);
        // D s_5_39: write-var result <= s_5_38
        fn_state.result = s_5_38;
        // D s_5_40: read-var s:i64
        let s_5_40: i64 = fn_state.s;
        // D s_5_41: cast zx s_5_40 -> i
        let s_5_41: i128 = (i128::try_from(s_5_40).unwrap());
        // D s_5_42: read-var elements:i64
        let s_5_42: i64 = fn_state.elements;
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: mul s_5_41 s_5_43
        let s_5_44: i128 = ((s_5_41) * (s_5_43));
        // D s_5_45: cast reint s_5_44 -> i64
        let s_5_45: i64 = (s_5_44 as i64);
        // C s_5_46: const #2s : i
        let s_5_46: i128 = 2;
        // D s_5_47: read-var p:i64
        let s_5_47: i64 = fn_state.p;
        // D s_5_48: cast zx s_5_47 -> i
        let s_5_48: i128 = (i128::try_from(s_5_47).unwrap());
        // D s_5_49: mul s_5_46 s_5_48
        let s_5_49: i128 = ((s_5_46) * (s_5_48));
        // D s_5_50: cast reint s_5_49 -> i64
        let s_5_50: i64 = (s_5_49 as i64);
        // D s_5_51: cast zx s_5_45 -> i
        let s_5_51: i128 = (i128::try_from(s_5_45).unwrap());
        // D s_5_52: cast zx s_5_50 -> i
        let s_5_52: i128 = (i128::try_from(s_5_50).unwrap());
        // D s_5_53: add s_5_51 s_5_52
        let s_5_53: i128 = (s_5_51 + s_5_52);
        // D s_5_54: cast reint s_5_53 -> i64
        let s_5_54: i64 = (s_5_53 as i64);
        // C s_5_55: const #1s : i
        let s_5_55: i128 = 1;
        // D s_5_56: cast zx s_5_54 -> i
        let s_5_56: i128 = (i128::try_from(s_5_54).unwrap());
        // D s_5_57: add s_5_56 s_5_55
        let s_5_57: i128 = (s_5_56 + s_5_55);
        // D s_5_58: cast reint s_5_57 -> i64
        let s_5_58: i64 = (s_5_57 as i64);
        // D s_5_59: read-var esizeshadow#4366:i64
        let s_5_59: i64 = fn_state.esizeshadow_4366;
        // D s_5_60: cast zx s_5_59 -> i
        let s_5_60: i128 = (i128::try_from(s_5_59).unwrap());
        // D s_5_61: cast reint s_5_60 -> i64
        let s_5_61: i64 = (s_5_60 as i64);
        // D s_5_62: read-var base:i64
        let s_5_62: i64 = fn_state.base;
        // D s_5_63: cast zx s_5_62 -> i
        let s_5_63: i128 = (i128::try_from(s_5_62).unwrap());
        // D s_5_64: read-var p:i64
        let s_5_64: i64 = fn_state.p;
        // D s_5_65: cast zx s_5_64 -> i
        let s_5_65: i128 = (i128::try_from(s_5_64).unwrap());
        // D s_5_66: add s_5_63 s_5_65
        let s_5_66: i128 = (s_5_63 + s_5_65);
        // D s_5_67: cast reint s_5_66 -> i64
        let s_5_67: i64 = (s_5_66 as i64);
        // D s_5_68: read-var esizeshadow#4366:i64
        let s_5_68: i64 = fn_state.esizeshadow_4366;
        // D s_5_69: cast zx s_5_68 -> i
        let s_5_69: i128 = (i128::try_from(s_5_68).unwrap());
        // D s_5_70: cast reint s_5_69 -> i64
        let s_5_70: i64 = (s_5_69 as i64);
        // D s_5_71: cast zx s_5_67 -> i
        let s_5_71: i128 = (i128::try_from(s_5_67).unwrap());
        // D s_5_72: cast zx s_5_70 -> i
        let s_5_72: i128 = (i128::try_from(s_5_70).unwrap());
        // D s_5_73: read-var operand2:bv
        let s_5_73: Bits = fn_state.operand2;
        // D s_5_74: call Elem_read(s_5_73, s_5_71, s_5_72)
        let s_5_74: Bits = Elem_read(state, tracer, s_5_73, s_5_71, s_5_72);
        // D s_5_75: cast zx s_5_58 -> i
        let s_5_75: i128 = (i128::try_from(s_5_58).unwrap());
        // D s_5_76: cast zx s_5_61 -> i
        let s_5_76: i128 = (i128::try_from(s_5_61).unwrap());
        // D s_5_77: read-var result:bv
        let s_5_77: Bits = fn_state.result;
        // D s_5_78: call Elem_set(s_5_77, s_5_75, s_5_76, s_5_74)
        let s_5_78: Bits = Elem_set(state, tracer, s_5_77, s_5_75, s_5_76, s_5_74);
        // D s_5_79: write-var result <= s_5_78
        fn_state.result = s_5_78;
        // D s_5_80: read-var p:i64
        let s_5_80: i64 = fn_state.p;
        // C s_5_81: const #1s : i64
        let s_5_81: i64 = 1;
        // D s_5_82: add s_5_80 s_5_81
        let s_5_82: i64 = (s_5_80 + s_5_81);
        // D s_5_83: write-var p <= s_5_82
        fn_state.p = s_5_82;
        // N s_5_84: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var s:i64
        let s_6_0: i64 = fn_state.s;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var s <= s_6_2
        fn_state.s = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VLshadow#4368:i64
        let s_7_0: i64 = fn_state.VLshadow_4368;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var result:bv
        let s_7_6: Bits = fn_state.result;
        // D s_7_7: call Z_set(s_7_4, s_7_5, s_7_6)
        let s_7_7: () = Z_set(state, tracer, s_7_4, s_7_5, s_7_6);
        // N s_7_8: return
        return;
    }
}
