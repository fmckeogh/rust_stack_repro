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
use Z_set::*;
use Elem_set::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use integer_subrange::*;
use common::*;
pub fn execute_HISTSEG_Z_ZZ__<T: Tracer>(
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
        e: i64,
        VLshadow_4114: i64,
        gs_216160: i64,
        gs_216154: i64,
        s: i64,
        element1: Bits,
        gs_216168: i64,
        count: i128,
        b: i64,
        result: Bits,
        i: i64,
        operand1: Bits,
        eltspersegment: i64,
        VLshadow_4113: i64,
        operand2: Bits,
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
        // D s_0_3: write-var VLshadow#4113 <= s_0_2
        fn_state.VLshadow_4113 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4113:i64
        let s_1_0: i64 = fn_state.VLshadow_4113;
        // D s_1_1: write-var VLshadow#4114 <= s_1_0
        fn_state.VLshadow_4114 = s_1_0;
        // C s_1_2: const #128s : i
        let s_1_2: i128 = 128;
        // D s_1_3: read-var VLshadow#4114:i64
        let s_1_3: i64 = fn_state.VLshadow_4114;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #128s : i
        let s_1_7: i128 = 128;
        // D s_1_8: read-var esize:i64
        let s_1_8: i64 = fn_state.esize;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: div s_1_7 s_1_9
        let s_1_10: i128 = ((s_1_7) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var eltspersegment <= s_1_11
        fn_state.eltspersegment = s_1_11;
        // D s_1_13: read-var VLshadow#4114:i64
        let s_1_13: i64 = fn_state.VLshadow_4114;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var n:i64
        let s_1_16: i64 = fn_state.n;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call Z_read(s_1_17, s_1_18)
        let s_1_19: Bits = Z_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var operand1 <= s_1_19
        fn_state.operand1 = s_1_19;
        // D s_1_21: read-var VLshadow#4114:i64
        let s_1_21: i64 = fn_state.VLshadow_4114;
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var m:i64
        let s_1_24: i64 = fn_state.m;
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_26: cast zx s_1_23 -> i
        let s_1_26: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_27: call Z_read(s_1_25, s_1_26)
        let s_1_27: Bits = Z_read(state, tracer, s_1_25, s_1_26);
        // D s_1_28: write-var operand2 <= s_1_27
        fn_state.operand2 = s_1_27;
        // C s_1_29: const #0s : i64
        let s_1_29: i64 = 0;
        // C s_1_30: const #1s : i
        let s_1_30: i128 = 1;
        // D s_1_31: cast zx s_1_6 -> i
        let s_1_31: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_32: sub s_1_31 s_1_30
        let s_1_32: i128 = ((s_1_31) - (s_1_30));
        // D s_1_33: cast reint s_1_32 -> i64
        let s_1_33: i64 = (s_1_32 as i64);
        // D s_1_34: write-var gs#216154 <= s_1_33
        fn_state.gs_216154 = s_1_33;
        // D s_1_35: write-var b <= s_1_29
        fn_state.b = s_1_29;
        // N s_1_36: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var b:i64
        let s_2_0: i64 = fn_state.b;
        // D s_2_1: read-var gs#216154:i64
        let s_2_1: i64 = fn_state.gs_216154;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b13 b3
        if s_2_2 {
            return block_13(state, tracer, fn_state);
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
        // D s_3_2: read-var eltspersegment:i64
        let s_3_2: i64 = fn_state.eltspersegment;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#216160 <= s_3_5
        fn_state.gs_216160 = s_3_5;
        // D s_3_7: write-var s <= s_3_0
        fn_state.s = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var s:i64
        let s_4_0: i64 = fn_state.s;
        // D s_4_1: read-var gs#216160:i64
        let s_4_1: i64 = fn_state.gs_216160;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b12 b5
        if s_4_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // D s_5_1: write-var count <= s_5_0
        fn_state.count = s_5_0;
        // D s_5_2: read-var eltspersegment:i64
        let s_5_2: i64 = fn_state.eltspersegment;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var b:i64
        let s_5_4: i64 = fn_state.b;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: mul s_5_3 s_5_5
        let s_5_6: i128 = ((s_5_3) * (s_5_5));
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: read-var s:i64
        let s_5_9: i64 = fn_state.s;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: add s_5_8 s_5_10
        let s_5_11: i128 = (s_5_8 + s_5_10);
        // D s_5_12: cast reint s_5_11 -> i64
        let s_5_12: i64 = (s_5_11 as i64);
        // D s_5_13: write-var e <= s_5_12
        fn_state.e = s_5_12;
        // D s_5_14: read-var esize:i64
        let s_5_14: i64 = fn_state.esize;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: cast reint s_5_15 -> i64
        let s_5_16: i64 = (s_5_15 as i64);
        // D s_5_17: read-var e:i64
        let s_5_17: i64 = fn_state.e;
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_19: cast zx s_5_16 -> i
        let s_5_19: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_20: read-var operand1:bv
        let s_5_20: Bits = fn_state.operand1;
        // D s_5_21: call Elem_read(s_5_20, s_5_18, s_5_19)
        let s_5_21: Bits = Elem_read(state, tracer, s_5_20, s_5_18, s_5_19);
        // D s_5_22: write-var element1 <= s_5_21
        fn_state.element1 = s_5_21;
        // C s_5_23: const #0s : i64
        let s_5_23: i64 = 0;
        // C s_5_24: const #1s : i
        let s_5_24: i128 = 1;
        // D s_5_25: read-var eltspersegment:i64
        let s_5_25: i64 = fn_state.eltspersegment;
        // D s_5_26: cast zx s_5_25 -> i
        let s_5_26: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_27: sub s_5_26 s_5_24
        let s_5_27: i128 = ((s_5_26) - (s_5_24));
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // D s_5_29: write-var gs#216168 <= s_5_28
        fn_state.gs_216168 = s_5_28;
        // D s_5_30: write-var i <= s_5_23
        fn_state.i = s_5_23;
        // N s_5_31: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var i:i64
        let s_6_0: i64 = fn_state.i;
        // D s_6_1: read-var gs#216168:i64
        let s_6_1: i64 = fn_state.gs_216168;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b11 b7
        if s_6_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var eltspersegment:i64
        let s_7_0: i64 = fn_state.eltspersegment;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var b:i64
        let s_7_2: i64 = fn_state.b;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: mul s_7_1 s_7_3
        let s_7_4: i128 = ((s_7_1) * (s_7_3));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: read-var i:i64
        let s_7_7: i64 = fn_state.i;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: add s_7_6 s_7_8
        let s_7_9: i128 = (s_7_6 + s_7_8);
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: read-var esize:i64
        let s_7_11: i64 = fn_state.esize;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // D s_7_14: cast zx s_7_10 -> i
        let s_7_14: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_15: cast zx s_7_13 -> i
        let s_7_15: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_16: read-var operand2:bv
        let s_7_16: Bits = fn_state.operand2;
        // D s_7_17: call Elem_read(s_7_16, s_7_14, s_7_15)
        let s_7_17: Bits = Elem_read(state, tracer, s_7_16, s_7_14, s_7_15);
        // D s_7_18: read-var element1:bv
        let s_7_18: Bits = fn_state.element1;
        // D s_7_19: cmp-eq s_7_18 s_7_17
        let s_7_19: bool = ((s_7_18) == (s_7_17));
        // N s_7_20: branch s_7_19 b10 b8
        if s_7_19 {
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
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var i:i64
        let s_9_0: i64 = fn_state.i;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var i <= s_9_2
        fn_state.i = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1s : i
        let s_10_0: i128 = 1;
        // D s_10_1: read-var count:i
        let s_10_1: i128 = fn_state.count;
        // D s_10_2: add s_10_1 s_10_0
        let s_10_2: i128 = (s_10_1 + s_10_0);
        // D s_10_3: write-var count <= s_10_2
        fn_state.count = s_10_2;
        // N s_10_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var count:i
        let s_11_0: i128 = fn_state.count;
        // D s_11_1: read-var esize:i64
        let s_11_1: i64 = fn_state.esize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #1s : i
        let s_11_4: i128 = 1;
        // D s_11_5: read-var esize:i64
        let s_11_5: i64 = fn_state.esize;
        // D s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_7: sub s_11_6 s_11_4
        let s_11_7: i128 = ((s_11_6) - (s_11_4));
        // D s_11_8: cast reint s_11_7 -> i64
        let s_11_8: i64 = (s_11_7 as i64);
        // C s_11_9: const #0s : i
        let s_11_9: i128 = 0;
        // D s_11_10: cast zx s_11_8 -> i
        let s_11_10: i128 = (i128::try_from(s_11_8).unwrap());
        // D s_11_11: call integer_subrange(s_11_0, s_11_10, s_11_9)
        let s_11_11: Bits = integer_subrange(state, tracer, s_11_0, s_11_10, s_11_9);
        // D s_11_12: read-var e:i64
        let s_11_12: i64 = fn_state.e;
        // D s_11_13: cast zx s_11_12 -> i
        let s_11_13: i128 = (i128::try_from(s_11_12).unwrap());
        // D s_11_14: cast zx s_11_3 -> i
        let s_11_14: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_15: read-var result:bv
        let s_11_15: Bits = fn_state.result;
        // D s_11_16: call Elem_set(s_11_15, s_11_13, s_11_14, s_11_11)
        let s_11_16: Bits = Elem_set(state, tracer, s_11_15, s_11_13, s_11_14, s_11_11);
        // D s_11_17: write-var result <= s_11_16
        fn_state.result = s_11_16;
        // D s_11_18: read-var s:i64
        let s_11_18: i64 = fn_state.s;
        // C s_11_19: const #1s : i64
        let s_11_19: i64 = 1;
        // D s_11_20: add s_11_18 s_11_19
        let s_11_20: i64 = (s_11_18 + s_11_19);
        // D s_11_21: write-var s <= s_11_20
        fn_state.s = s_11_20;
        // N s_11_22: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var b:i64
        let s_12_0: i64 = fn_state.b;
        // C s_12_1: const #1s : i64
        let s_12_1: i64 = 1;
        // D s_12_2: add s_12_0 s_12_1
        let s_12_2: i64 = (s_12_0 + s_12_1);
        // D s_12_3: write-var b <= s_12_2
        fn_state.b = s_12_2;
        // N s_12_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VLshadow#4114:i64
        let s_13_0: i64 = fn_state.VLshadow_4114;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // D s_13_3: read-var d:i64
        let s_13_3: i64 = fn_state.d;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: cast zx s_13_2 -> i
        let s_13_5: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_6: read-var result:bv
        let s_13_6: Bits = fn_state.result;
        // D s_13_7: call Z_set(s_13_4, s_13_5, s_13_6)
        let s_13_7: () = Z_set(state, tracer, s_13_4, s_13_5, s_13_6);
        // N s_13_8: return
        return;
    }
}
