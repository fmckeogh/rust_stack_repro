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
pub fn execute_UZPQ2_Z_ZZ__<T: Tracer>(
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
        gs_220440: i64,
        p: i64,
        s: i64,
        pairs: i64,
        esizeshadow_4363: i64,
        elements: i64,
        result: Bits,
        operand1: Bits,
        u_5098: i64,
        gs_220449: i64,
        gs_220434: i64,
        VLshadow_4364: i64,
        VLshadow_4365: i64,
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
        // D s_0_3: write-var esizeshadow#4363 <= s_0_2
        fn_state.esizeshadow_4363 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4364 <= s_0_6
        fn_state.VLshadow_4364 = s_0_6;
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
        // D s_1_0: read-var VLshadow#4364:i64
        let s_1_0: i64 = fn_state.VLshadow_4364;
        // D s_1_1: write-var VLshadow#4365 <= s_1_0
        fn_state.VLshadow_4365 = s_1_0;
        // C s_1_2: const #128s : i
        let s_1_2: i128 = 128;
        // D s_1_3: read-var VLshadow#4365:i64
        let s_1_3: i64 = fn_state.VLshadow_4365;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #128s : i
        let s_1_7: i128 = 128;
        // D s_1_8: read-var esizeshadow#4363:i64
        let s_1_8: i64 = fn_state.esizeshadow_4363;
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
        // D s_1_19: read-var VLshadow#4365:i64
        let s_1_19: i64 = fn_state.VLshadow_4365;
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
        // D s_1_27: read-var VLshadow#4365:i64
        let s_1_27: i64 = fn_state.VLshadow_4365;
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
        // D s_1_40: write-var gs#220434 <= s_1_39
        fn_state.gs_220434 = s_1_39;
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
        // D s_2_1: read-var gs#220434:i64
        let s_2_1: i64 = fn_state.gs_220434;
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
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var pairs:i64
        let s_3_2: i64 = fn_state.pairs;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#220440 <= s_3_5
        fn_state.gs_220440 = s_3_5;
        // D s_3_7: write-var p <= s_3_0
        fn_state.p = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var p:i64
        let s_4_0: i64 = fn_state.p;
        // D s_4_1: read-var gs#220440:i64
        let s_4_1: i64 = fn_state.gs_220440;
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
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: read-var p:i64
        let s_5_7: i64 = fn_state.p;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: add s_5_6 s_5_8
        let s_5_9: i128 = (s_5_6 + s_5_8);
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: read-var esizeshadow#4363:i64
        let s_5_11: i64 = fn_state.esizeshadow_4363;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: read-var s:i64
        let s_5_14: i64 = fn_state.s;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: read-var elements:i64
        let s_5_16: i64 = fn_state.elements;
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_18: mul s_5_15 s_5_17
        let s_5_18: i128 = ((s_5_15) * (s_5_17));
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // C s_5_20: const #2s : i
        let s_5_20: i128 = 2;
        // D s_5_21: read-var p:i64
        let s_5_21: i64 = fn_state.p;
        // D s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_23: mul s_5_20 s_5_22
        let s_5_23: i128 = ((s_5_20) * (s_5_22));
        // D s_5_24: cast reint s_5_23 -> i64
        let s_5_24: i64 = (s_5_23 as i64);
        // D s_5_25: cast zx s_5_19 -> i
        let s_5_25: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_26: cast zx s_5_24 -> i
        let s_5_26: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_27: add s_5_25 s_5_26
        let s_5_27: i128 = (s_5_25 + s_5_26);
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // D s_5_29: cast zx s_5_28 -> i
        let s_5_29: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_30: read-var part:i64
        let s_5_30: i64 = fn_state.part;
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_32: add s_5_29 s_5_31
        let s_5_32: i128 = (s_5_29 + s_5_31);
        // D s_5_33: cast reint s_5_32 -> i64
        let s_5_33: i64 = (s_5_32 as i64);
        // D s_5_34: read-var esizeshadow#4363:i64
        let s_5_34: i64 = fn_state.esizeshadow_4363;
        // D s_5_35: cast zx s_5_34 -> i
        let s_5_35: i128 = (i128::try_from(s_5_34).unwrap());
        // D s_5_36: cast reint s_5_35 -> i64
        let s_5_36: i64 = (s_5_35 as i64);
        // D s_5_37: cast zx s_5_33 -> i
        let s_5_37: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_38: cast zx s_5_36 -> i
        let s_5_38: i128 = (i128::try_from(s_5_36).unwrap());
        // D s_5_39: read-var operand1:bv
        let s_5_39: Bits = fn_state.operand1;
        // D s_5_40: call Elem_read(s_5_39, s_5_37, s_5_38)
        let s_5_40: Bits = Elem_read(state, tracer, s_5_39, s_5_37, s_5_38);
        // D s_5_41: cast zx s_5_10 -> i
        let s_5_41: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_42: cast zx s_5_13 -> i
        let s_5_42: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_43: read-var result:bv
        let s_5_43: Bits = fn_state.result;
        // D s_5_44: call Elem_set(s_5_43, s_5_41, s_5_42, s_5_40)
        let s_5_44: Bits = Elem_set(state, tracer, s_5_43, s_5_41, s_5_42, s_5_40);
        // D s_5_45: write-var result <= s_5_44
        fn_state.result = s_5_44;
        // D s_5_46: read-var p:i64
        let s_5_46: i64 = fn_state.p;
        // C s_5_47: const #1s : i64
        let s_5_47: i64 = 1;
        // D s_5_48: add s_5_46 s_5_47
        let s_5_48: i64 = (s_5_46 + s_5_47);
        // D s_5_49: write-var p <= s_5_48
        fn_state.p = s_5_48;
        // N s_5_50: jump b4
        return block_4(state, tracer, fn_state);
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
        // D s_6_6: write-var gs#220449 <= s_6_5
        fn_state.gs_220449 = s_6_5;
        // D s_6_7: write-var u#5098 <= s_6_0
        fn_state.u_5098 = s_6_0;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var u#5098:i64
        let s_7_0: i64 = fn_state.u_5098;
        // D s_7_1: read-var gs#220449:i64
        let s_7_1: i64 = fn_state.gs_220449;
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
        // D s_8_0: read-var s:i64
        let s_8_0: i64 = fn_state.s;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var elements:i64
        let s_8_2: i64 = fn_state.elements;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: mul s_8_1 s_8_3
        let s_8_4: i128 = ((s_8_1) * (s_8_3));
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: read-var pairs:i64
        let s_8_7: i64 = fn_state.pairs;
        // D s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_9: add s_8_6 s_8_8
        let s_8_9: i128 = (s_8_6 + s_8_8);
        // D s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: read-var u#5098:i64
        let s_8_12: i64 = fn_state.u_5098;
        // D s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_14: add s_8_11 s_8_13
        let s_8_14: i128 = (s_8_11 + s_8_13);
        // D s_8_15: cast reint s_8_14 -> i64
        let s_8_15: i64 = (s_8_14 as i64);
        // D s_8_16: read-var esizeshadow#4363:i64
        let s_8_16: i64 = fn_state.esizeshadow_4363;
        // D s_8_17: cast zx s_8_16 -> i
        let s_8_17: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_18: cast reint s_8_17 -> i64
        let s_8_18: i64 = (s_8_17 as i64);
        // D s_8_19: read-var s:i64
        let s_8_19: i64 = fn_state.s;
        // D s_8_20: cast zx s_8_19 -> i
        let s_8_20: i128 = (i128::try_from(s_8_19).unwrap());
        // D s_8_21: read-var elements:i64
        let s_8_21: i64 = fn_state.elements;
        // D s_8_22: cast zx s_8_21 -> i
        let s_8_22: i128 = (i128::try_from(s_8_21).unwrap());
        // D s_8_23: mul s_8_20 s_8_22
        let s_8_23: i128 = ((s_8_20) * (s_8_22));
        // D s_8_24: cast reint s_8_23 -> i64
        let s_8_24: i64 = (s_8_23 as i64);
        // C s_8_25: const #2s : i
        let s_8_25: i128 = 2;
        // D s_8_26: read-var u#5098:i64
        let s_8_26: i64 = fn_state.u_5098;
        // D s_8_27: cast zx s_8_26 -> i
        let s_8_27: i128 = (i128::try_from(s_8_26).unwrap());
        // D s_8_28: mul s_8_25 s_8_27
        let s_8_28: i128 = ((s_8_25) * (s_8_27));
        // D s_8_29: cast reint s_8_28 -> i64
        let s_8_29: i64 = (s_8_28 as i64);
        // D s_8_30: cast zx s_8_24 -> i
        let s_8_30: i128 = (i128::try_from(s_8_24).unwrap());
        // D s_8_31: cast zx s_8_29 -> i
        let s_8_31: i128 = (i128::try_from(s_8_29).unwrap());
        // D s_8_32: add s_8_30 s_8_31
        let s_8_32: i128 = (s_8_30 + s_8_31);
        // D s_8_33: cast reint s_8_32 -> i64
        let s_8_33: i64 = (s_8_32 as i64);
        // D s_8_34: cast zx s_8_33 -> i
        let s_8_34: i128 = (i128::try_from(s_8_33).unwrap());
        // D s_8_35: read-var part:i64
        let s_8_35: i64 = fn_state.part;
        // D s_8_36: cast zx s_8_35 -> i
        let s_8_36: i128 = (i128::try_from(s_8_35).unwrap());
        // D s_8_37: add s_8_34 s_8_36
        let s_8_37: i128 = (s_8_34 + s_8_36);
        // D s_8_38: cast reint s_8_37 -> i64
        let s_8_38: i64 = (s_8_37 as i64);
        // D s_8_39: read-var esizeshadow#4363:i64
        let s_8_39: i64 = fn_state.esizeshadow_4363;
        // D s_8_40: cast zx s_8_39 -> i
        let s_8_40: i128 = (i128::try_from(s_8_39).unwrap());
        // D s_8_41: cast reint s_8_40 -> i64
        let s_8_41: i64 = (s_8_40 as i64);
        // D s_8_42: cast zx s_8_38 -> i
        let s_8_42: i128 = (i128::try_from(s_8_38).unwrap());
        // D s_8_43: cast zx s_8_41 -> i
        let s_8_43: i128 = (i128::try_from(s_8_41).unwrap());
        // D s_8_44: read-var operand2:bv
        let s_8_44: Bits = fn_state.operand2;
        // D s_8_45: call Elem_read(s_8_44, s_8_42, s_8_43)
        let s_8_45: Bits = Elem_read(state, tracer, s_8_44, s_8_42, s_8_43);
        // D s_8_46: cast zx s_8_15 -> i
        let s_8_46: i128 = (i128::try_from(s_8_15).unwrap());
        // D s_8_47: cast zx s_8_18 -> i
        let s_8_47: i128 = (i128::try_from(s_8_18).unwrap());
        // D s_8_48: read-var result:bv
        let s_8_48: Bits = fn_state.result;
        // D s_8_49: call Elem_set(s_8_48, s_8_46, s_8_47, s_8_45)
        let s_8_49: Bits = Elem_set(state, tracer, s_8_48, s_8_46, s_8_47, s_8_45);
        // D s_8_50: write-var result <= s_8_49
        fn_state.result = s_8_49;
        // D s_8_51: read-var u#5098:i64
        let s_8_51: i64 = fn_state.u_5098;
        // C s_8_52: const #1s : i64
        let s_8_52: i64 = 1;
        // D s_8_53: add s_8_51 s_8_52
        let s_8_53: i64 = (s_8_51 + s_8_52);
        // D s_8_54: write-var u#5098 <= s_8_53
        fn_state.u_5098 = s_8_53;
        // N s_8_55: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var s:i64
        let s_9_0: i64 = fn_state.s;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var s <= s_9_2
        fn_state.s = s_9_2;
        // N s_9_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#4365:i64
        let s_10_0: i64 = fn_state.VLshadow_4365;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var d:i64
        let s_10_3: i64 = fn_state.d;
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
