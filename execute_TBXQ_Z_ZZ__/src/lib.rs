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
pub fn execute_TBXQ_Z_ZZ__<T: Tracer>(
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
        gs_220297: i64,
        e: i64,
        gs_220303: i64,
        VLshadow_4358: i64,
        esizeshadow_4357: i64,
        VLshadow_4359: i64,
        idx: i128,
        s: i64,
        elements: i64,
        result: Bits,
        operand1: Bits,
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
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#4357 <= s_0_2
        fn_state.esizeshadow_4357 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#4358 <= s_0_6
        fn_state.VLshadow_4358 = s_0_6;
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
        // D s_1_0: read-var VLshadow#4358:i64
        let s_1_0: i64 = fn_state.VLshadow_4358;
        // D s_1_1: write-var VLshadow#4359 <= s_1_0
        fn_state.VLshadow_4359 = s_1_0;
        // C s_1_2: const #128s : i
        let s_1_2: i128 = 128;
        // D s_1_3: read-var VLshadow#4359:i64
        let s_1_3: i64 = fn_state.VLshadow_4359;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // C s_1_7: const #128s : i
        let s_1_7: i128 = 128;
        // D s_1_8: read-var esizeshadow#4357:i64
        let s_1_8: i64 = fn_state.esizeshadow_4357;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: div s_1_7 s_1_9
        let s_1_10: i128 = ((s_1_7) / (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var elements <= s_1_11
        fn_state.elements = s_1_11;
        // D s_1_13: read-var VLshadow#4359:i64
        let s_1_13: i64 = fn_state.VLshadow_4359;
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
        // D s_1_21: read-var VLshadow#4359:i64
        let s_1_21: i64 = fn_state.VLshadow_4359;
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
        // D s_1_29: read-var VLshadow#4359:i64
        let s_1_29: i64 = fn_state.VLshadow_4359;
        // D s_1_30: cast zx s_1_29 -> i
        let s_1_30: i128 = (i128::try_from(s_1_29).unwrap());
        // D s_1_31: cast reint s_1_30 -> i64
        let s_1_31: i64 = (s_1_30 as i64);
        // D s_1_32: read-var d:i64
        let s_1_32: i64 = fn_state.d;
        // D s_1_33: cast zx s_1_32 -> i
        let s_1_33: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_34: cast zx s_1_31 -> i
        let s_1_34: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_35: call Z_read(s_1_33, s_1_34)
        let s_1_35: Bits = Z_read(state, tracer, s_1_33, s_1_34);
        // D s_1_36: write-var result <= s_1_35
        fn_state.result = s_1_35;
        // C s_1_37: const #0s : i64
        let s_1_37: i64 = 0;
        // C s_1_38: const #1s : i
        let s_1_38: i128 = 1;
        // D s_1_39: cast zx s_1_6 -> i
        let s_1_39: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_40: sub s_1_39 s_1_38
        let s_1_40: i128 = ((s_1_39) - (s_1_38));
        // D s_1_41: cast reint s_1_40 -> i64
        let s_1_41: i64 = (s_1_40 as i64);
        // D s_1_42: write-var gs#220297 <= s_1_41
        fn_state.gs_220297 = s_1_41;
        // D s_1_43: write-var s <= s_1_37
        fn_state.s = s_1_37;
        // N s_1_44: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var s:i64
        let s_2_0: i64 = fn_state.s;
        // D s_2_1: read-var gs#220297:i64
        let s_2_1: i64 = fn_state.gs_220297;
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
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#220303 <= s_3_5
        fn_state.gs_220303 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#220303:i64
        let s_4_1: i64 = fn_state.gs_220303;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
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
        // D s_5_7: read-var e:i64
        let s_5_7: i64 = fn_state.e;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: add s_5_6 s_5_8
        let s_5_9: i128 = (s_5_6 + s_5_8);
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: read-var esizeshadow#4357:i64
        let s_5_11: i64 = fn_state.esizeshadow_4357;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: cast zx s_5_10 -> i
        let s_5_14: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_15: cast zx s_5_13 -> i
        let s_5_15: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_16: read-var operand2:bv
        let s_5_16: Bits = fn_state.operand2;
        // D s_5_17: call Elem_read(s_5_16, s_5_14, s_5_15)
        let s_5_17: Bits = Elem_read(state, tracer, s_5_16, s_5_14, s_5_15);
        // D s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (s_5_17.value() as i128);
        // D s_5_19: write-var idx <= s_5_18
        fn_state.idx = s_5_18;
        // D s_5_20: read-var elements:i64
        let s_5_20: i64 = fn_state.elements;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: read-var idx:i
        let s_5_22: i128 = fn_state.idx;
        // D s_5_23: cmp-lt s_5_22 s_5_21
        let s_5_23: bool = ((s_5_22) < (s_5_21));
        // N s_5_24: branch s_5_23 b8 b6
        if s_5_23 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var e <= s_7_2
        fn_state.e = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // D s_8_7: read-var e:i64
        let s_8_7: i64 = fn_state.e;
        // D s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_9: add s_8_6 s_8_8
        let s_8_9: i128 = (s_8_6 + s_8_8);
        // D s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: read-var esizeshadow#4357:i64
        let s_8_11: i64 = fn_state.esizeshadow_4357;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: cast reint s_8_12 -> i64
        let s_8_13: i64 = (s_8_12 as i64);
        // D s_8_14: read-var s:i64
        let s_8_14: i64 = fn_state.s;
        // D s_8_15: cast zx s_8_14 -> i
        let s_8_15: i128 = (i128::try_from(s_8_14).unwrap());
        // D s_8_16: read-var elements:i64
        let s_8_16: i64 = fn_state.elements;
        // D s_8_17: cast zx s_8_16 -> i
        let s_8_17: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_18: mul s_8_15 s_8_17
        let s_8_18: i128 = ((s_8_15) * (s_8_17));
        // D s_8_19: cast reint s_8_18 -> i64
        let s_8_19: i64 = (s_8_18 as i64);
        // D s_8_20: cast zx s_8_19 -> i
        let s_8_20: i128 = (i128::try_from(s_8_19).unwrap());
        // D s_8_21: read-var idx:i
        let s_8_21: i128 = fn_state.idx;
        // D s_8_22: add s_8_20 s_8_21
        let s_8_22: i128 = (s_8_20 + s_8_21);
        // D s_8_23: cast reint s_8_22 -> i64
        let s_8_23: i64 = (s_8_22 as i64);
        // D s_8_24: read-var esizeshadow#4357:i64
        let s_8_24: i64 = fn_state.esizeshadow_4357;
        // D s_8_25: cast zx s_8_24 -> i
        let s_8_25: i128 = (i128::try_from(s_8_24).unwrap());
        // D s_8_26: cast reint s_8_25 -> i64
        let s_8_26: i64 = (s_8_25 as i64);
        // D s_8_27: cast zx s_8_23 -> i
        let s_8_27: i128 = (i128::try_from(s_8_23).unwrap());
        // D s_8_28: cast zx s_8_26 -> i
        let s_8_28: i128 = (i128::try_from(s_8_26).unwrap());
        // D s_8_29: read-var operand1:bv
        let s_8_29: Bits = fn_state.operand1;
        // D s_8_30: call Elem_read(s_8_29, s_8_27, s_8_28)
        let s_8_30: Bits = Elem_read(state, tracer, s_8_29, s_8_27, s_8_28);
        // D s_8_31: cast zx s_8_10 -> i
        let s_8_31: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_32: cast zx s_8_13 -> i
        let s_8_32: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_33: read-var result:bv
        let s_8_33: Bits = fn_state.result;
        // D s_8_34: call Elem_set(s_8_33, s_8_31, s_8_32, s_8_30)
        let s_8_34: Bits = Elem_set(state, tracer, s_8_33, s_8_31, s_8_32, s_8_30);
        // D s_8_35: write-var result <= s_8_34
        fn_state.result = s_8_34;
        // N s_8_36: jump b7
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
        // D s_10_0: read-var VLshadow#4359:i64
        let s_10_0: i64 = fn_state.VLshadow_4359;
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
