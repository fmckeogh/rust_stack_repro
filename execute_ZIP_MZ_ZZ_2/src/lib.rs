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
pub fn execute_ZIP_MZ_ZZ_2<T: Tracer>(
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
        p: i64,
        VLshadow_6791: i64,
        base: i64,
        operand0: Bits,
        gs_288294: i64,
        pairs: i64,
        esizeshadow_6789: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_6790: i64,
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
        // D s_0_3: write-var esizeshadow#6789 <= s_0_2
        fn_state.esizeshadow_6789 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6790 <= s_0_6
        fn_state.VLshadow_6790 = s_0_6;
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
        // D s_1_0: read-var VLshadow#6790:i64
        let s_1_0: i64 = fn_state.VLshadow_6790;
        // D s_1_1: write-var VLshadow#6791 <= s_1_0
        fn_state.VLshadow_6791 = s_1_0;
        // C s_1_2: const #2s : i
        let s_1_2: i128 = 2;
        // D s_1_3: read-var esizeshadow#6789:i64
        let s_1_3: i64 = fn_state.esizeshadow_6789;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: mul s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) * (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#6791:i64
        let s_1_7: i64 = fn_state.VLshadow_6791;
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
        // D s_1_13: read-var VLshadow#6791:i64
        let s_1_13: i64 = fn_state.VLshadow_6791;
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
        // D s_1_20: write-var operand0 <= s_1_19
        fn_state.operand0 = s_1_19;
        // D s_1_21: read-var VLshadow#6791:i64
        let s_1_21: i64 = fn_state.VLshadow_6791;
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
        // D s_1_28: write-var operand1 <= s_1_27
        fn_state.operand1 = s_1_27;
        // C s_1_29: const #0s : i64
        let s_1_29: i64 = 0;
        // D s_1_30: write-var r <= s_1_29
        fn_state.r = s_1_29;
        // N s_1_31: jump b2
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
        // C s_3_7: const #0s : i64
        let s_3_7: i64 = 0;
        // C s_3_8: const #1s : i
        let s_3_8: i128 = 1;
        // D s_3_9: read-var pairs:i64
        let s_3_9: i64 = fn_state.pairs;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: sub s_3_10 s_3_8
        let s_3_11: i128 = ((s_3_10) - (s_3_8));
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: write-var gs#288294 <= s_3_12
        fn_state.gs_288294 = s_3_12;
        // D s_3_14: write-var p <= s_3_7
        fn_state.p = s_3_7;
        // N s_3_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var p:i64
        let s_4_0: i64 = fn_state.p;
        // D s_4_1: read-var gs#288294:i64
        let s_4_1: i64 = fn_state.gs_288294;
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
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var p:i64
        let s_5_1: i64 = fn_state.p;
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
        // D s_5_9: read-var esizeshadow#6789:i64
        let s_5_9: i64 = fn_state.esizeshadow_6789;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: read-var base:i64
        let s_5_12: i64 = fn_state.base;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: read-var p:i64
        let s_5_14: i64 = fn_state.p;
        // D s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_16: add s_5_13 s_5_15
        let s_5_16: i128 = (s_5_13 + s_5_15);
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // D s_5_18: read-var esizeshadow#6789:i64
        let s_5_18: i64 = fn_state.esizeshadow_6789;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: cast reint s_5_19 -> i64
        let s_5_20: i64 = (s_5_19 as i64);
        // D s_5_21: cast zx s_5_17 -> i
        let s_5_21: i128 = (i128::try_from(s_5_17).unwrap());
        // D s_5_22: cast zx s_5_20 -> i
        let s_5_22: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_23: read-var operand0:bv
        let s_5_23: Bits = fn_state.operand0;
        // D s_5_24: call Elem_read(s_5_23, s_5_21, s_5_22)
        let s_5_24: Bits = Elem_read(state, tracer, s_5_23, s_5_21, s_5_22);
        // D s_5_25: cast zx s_5_8 -> i
        let s_5_25: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_26: cast zx s_5_11 -> i
        let s_5_26: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_27: read-var result:bv
        let s_5_27: Bits = fn_state.result;
        // D s_5_28: call Elem_set(s_5_27, s_5_25, s_5_26, s_5_24)
        let s_5_28: Bits = Elem_set(state, tracer, s_5_27, s_5_25, s_5_26, s_5_24);
        // D s_5_29: write-var result <= s_5_28
        fn_state.result = s_5_28;
        // C s_5_30: const #2s : i
        let s_5_30: i128 = 2;
        // D s_5_31: read-var p:i64
        let s_5_31: i64 = fn_state.p;
        // D s_5_32: cast zx s_5_31 -> i
        let s_5_32: i128 = (i128::try_from(s_5_31).unwrap());
        // D s_5_33: mul s_5_30 s_5_32
        let s_5_33: i128 = ((s_5_30) * (s_5_32));
        // D s_5_34: cast reint s_5_33 -> i64
        let s_5_34: i64 = (s_5_33 as i64);
        // C s_5_35: const #1s : i
        let s_5_35: i128 = 1;
        // D s_5_36: cast zx s_5_34 -> i
        let s_5_36: i128 = (i128::try_from(s_5_34).unwrap());
        // D s_5_37: add s_5_36 s_5_35
        let s_5_37: i128 = (s_5_36 + s_5_35);
        // D s_5_38: cast reint s_5_37 -> i64
        let s_5_38: i64 = (s_5_37 as i64);
        // D s_5_39: read-var esizeshadow#6789:i64
        let s_5_39: i64 = fn_state.esizeshadow_6789;
        // D s_5_40: cast zx s_5_39 -> i
        let s_5_40: i128 = (i128::try_from(s_5_39).unwrap());
        // D s_5_41: cast reint s_5_40 -> i64
        let s_5_41: i64 = (s_5_40 as i64);
        // D s_5_42: read-var base:i64
        let s_5_42: i64 = fn_state.base;
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: read-var p:i64
        let s_5_44: i64 = fn_state.p;
        // D s_5_45: cast zx s_5_44 -> i
        let s_5_45: i128 = (i128::try_from(s_5_44).unwrap());
        // D s_5_46: add s_5_43 s_5_45
        let s_5_46: i128 = (s_5_43 + s_5_45);
        // D s_5_47: cast reint s_5_46 -> i64
        let s_5_47: i64 = (s_5_46 as i64);
        // D s_5_48: read-var esizeshadow#6789:i64
        let s_5_48: i64 = fn_state.esizeshadow_6789;
        // D s_5_49: cast zx s_5_48 -> i
        let s_5_49: i128 = (i128::try_from(s_5_48).unwrap());
        // D s_5_50: cast reint s_5_49 -> i64
        let s_5_50: i64 = (s_5_49 as i64);
        // D s_5_51: cast zx s_5_47 -> i
        let s_5_51: i128 = (i128::try_from(s_5_47).unwrap());
        // D s_5_52: cast zx s_5_50 -> i
        let s_5_52: i128 = (i128::try_from(s_5_50).unwrap());
        // D s_5_53: read-var operand1:bv
        let s_5_53: Bits = fn_state.operand1;
        // D s_5_54: call Elem_read(s_5_53, s_5_51, s_5_52)
        let s_5_54: Bits = Elem_read(state, tracer, s_5_53, s_5_51, s_5_52);
        // D s_5_55: cast zx s_5_38 -> i
        let s_5_55: i128 = (i128::try_from(s_5_38).unwrap());
        // D s_5_56: cast zx s_5_41 -> i
        let s_5_56: i128 = (i128::try_from(s_5_41).unwrap());
        // D s_5_57: read-var result:bv
        let s_5_57: Bits = fn_state.result;
        // D s_5_58: call Elem_set(s_5_57, s_5_55, s_5_56, s_5_54)
        let s_5_58: Bits = Elem_set(state, tracer, s_5_57, s_5_55, s_5_56, s_5_54);
        // D s_5_59: write-var result <= s_5_58
        fn_state.result = s_5_58;
        // D s_5_60: read-var p:i64
        let s_5_60: i64 = fn_state.p;
        // C s_5_61: const #1s : i64
        let s_5_61: i64 = 1;
        // D s_5_62: add s_5_60 s_5_61
        let s_5_62: i64 = (s_5_60 + s_5_61);
        // D s_5_63: write-var p <= s_5_62
        fn_state.p = s_5_62;
        // N s_5_64: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var d:i64
        let s_6_0: i64 = fn_state.d;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var r:i64
        let s_6_2: i64 = fn_state.r;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: add s_6_1 s_6_3
        let s_6_4: i128 = (s_6_1 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: read-var VLshadow#6791:i64
        let s_6_6: i64 = fn_state.VLshadow_6791;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: cast reint s_6_7 -> i64
        let s_6_8: i64 = (s_6_7 as i64);
        // D s_6_9: cast zx s_6_5 -> i
        let s_6_9: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_10: cast zx s_6_8 -> i
        let s_6_10: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_11: read-var result:bv
        let s_6_11: Bits = fn_state.result;
        // D s_6_12: call Z_set(s_6_9, s_6_10, s_6_11)
        let s_6_12: () = Z_set(state, tracer, s_6_9, s_6_10, s_6_11);
        // D s_6_13: read-var r:i64
        let s_6_13: i64 = fn_state.r;
        // C s_6_14: const #1s : i64
        let s_6_14: i64 = 1;
        // D s_6_15: add s_6_13 s_6_14
        let s_6_15: i64 = (s_6_13 + s_6_14);
        // D s_6_16: write-var r <= s_6_15
        fn_state.r = s_6_15;
        // N s_6_17: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
}
