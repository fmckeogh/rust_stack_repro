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
use V_read::*;
use V_set::*;
use integer_subrange::*;
use asl_Int::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_add_pairwise<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acc: bool,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        gs_167073: i64,
        esizeshadow_1804: i64,
        e: i64,
        sum: Bits,
        datasizeshadow_1805: i64,
        result: Bits,
        acc: bool,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        acc,
        d,
        datasize,
        elements,
        esize,
        n,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#1804 <= s_0_2
        fn_state.esizeshadow_1804 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1805 <= s_0_6
        fn_state.datasizeshadow_1805 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1805:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1805;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand <= s_1_5
        fn_state.operand = s_1_5;
        // D s_1_7: read-var acc:u8
        let s_1_7: bool = fn_state.acc;
        // N s_1_8: branch s_1_7 b10 b2
        if s_1_7 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
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
        // D s_3_2: read-var elements:i
        let s_3_2: i128 = fn_state.elements;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#167073 <= s_3_4
        fn_state.gs_167073 = s_3_4;
        // D s_3_6: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#167073:i64
        let s_4_1: i64 = fn_state.gs_167073;
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
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var e:i64
        let s_5_1: i64 = fn_state.e;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_0 s_5_2
        let s_5_3: i128 = ((s_5_0) * (s_5_2));
        // C s_5_4: const #0s : i
        let s_5_4: i128 = 0;
        // D s_5_5: add s_5_3 s_5_4
        let s_5_5: i128 = (s_5_3 + s_5_4);
        // D s_5_6: read-var esizeshadow#1804:i64
        let s_5_6: i64 = fn_state.esizeshadow_1804;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: read-var operand:bv
        let s_5_10: Bits = fn_state.operand;
        // D s_5_11: call Elem_read(s_5_10, s_5_5, s_5_9)
        let s_5_11: Bits = Elem_read(state, tracer, s_5_10, s_5_5, s_5_9);
        // D s_5_12: read-var is_unsigned:u8
        let s_5_12: bool = fn_state.is_unsigned;
        // D s_5_13: call asl_Int(s_5_11, s_5_12)
        let s_5_13: i128 = asl_Int(state, tracer, s_5_11, s_5_12);
        // C s_5_14: const #2s : i
        let s_5_14: i128 = 2;
        // D s_5_15: read-var e:i64
        let s_5_15: i64 = fn_state.e;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: mul s_5_14 s_5_16
        let s_5_17: i128 = ((s_5_14) * (s_5_16));
        // C s_5_18: const #1s : i
        let s_5_18: i128 = 1;
        // D s_5_19: add s_5_17 s_5_18
        let s_5_19: i128 = (s_5_17 + s_5_18);
        // D s_5_20: read-var esizeshadow#1804:i64
        let s_5_20: i64 = fn_state.esizeshadow_1804;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: cast zx s_5_22 -> i
        let s_5_23: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_24: read-var operand:bv
        let s_5_24: Bits = fn_state.operand;
        // D s_5_25: call Elem_read(s_5_24, s_5_19, s_5_23)
        let s_5_25: Bits = Elem_read(state, tracer, s_5_24, s_5_19, s_5_23);
        // D s_5_26: read-var is_unsigned:u8
        let s_5_26: bool = fn_state.is_unsigned;
        // D s_5_27: call asl_Int(s_5_25, s_5_26)
        let s_5_27: i128 = asl_Int(state, tracer, s_5_25, s_5_26);
        // D s_5_28: add s_5_13 s_5_27
        let s_5_28: i128 = (s_5_13 + s_5_27);
        // C s_5_29: const #2s : i
        let s_5_29: i128 = 2;
        // D s_5_30: read-var esizeshadow#1804:i64
        let s_5_30: i64 = fn_state.esizeshadow_1804;
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_32: mul s_5_29 s_5_31
        let s_5_32: i128 = ((s_5_29) * (s_5_31));
        // D s_5_33: cast reint s_5_32 -> i64
        let s_5_33: i64 = (s_5_32 as i64);
        // C s_5_34: const #1s : i
        let s_5_34: i128 = 1;
        // D s_5_35: cast zx s_5_33 -> i
        let s_5_35: i128 = (i128::try_from(s_5_33).unwrap());
        // D s_5_36: sub s_5_35 s_5_34
        let s_5_36: i128 = ((s_5_35) - (s_5_34));
        // D s_5_37: cast reint s_5_36 -> i64
        let s_5_37: i64 = (s_5_36 as i64);
        // C s_5_38: const #0s : i
        let s_5_38: i128 = 0;
        // D s_5_39: cast zx s_5_37 -> i
        let s_5_39: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_40: call integer_subrange(s_5_28, s_5_39, s_5_38)
        let s_5_40: Bits = integer_subrange(state, tracer, s_5_28, s_5_39, s_5_38);
        // D s_5_41: write-var sum <= s_5_40
        fn_state.sum = s_5_40;
        // D s_5_42: read-var acc:u8
        let s_5_42: bool = fn_state.acc;
        // N s_5_43: branch s_5_42 b8 b6
        if s_5_42 {
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
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var esizeshadow#1804:i64
        let s_6_1: i64 = fn_state.esizeshadow_1804;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: mul s_6_0 s_6_2
        let s_6_3: i128 = ((s_6_0) * (s_6_2));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: cast reint s_6_5 -> i64
        let s_6_6: i64 = (s_6_5 as i64);
        // D s_6_7: read-var e:i64
        let s_6_7: i64 = fn_state.e;
        // D s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_9: cast zx s_6_6 -> i
        let s_6_9: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_10: read-var result:bv
        let s_6_10: Bits = fn_state.result;
        // D s_6_11: read-var sum:bv
        let s_6_11: Bits = fn_state.sum;
        // D s_6_12: call Elem_set(s_6_10, s_6_8, s_6_9, s_6_11)
        let s_6_12: Bits = Elem_set(state, tracer, s_6_10, s_6_8, s_6_9, s_6_11);
        // D s_6_13: write-var result <= s_6_12
        fn_state.result = s_6_12;
        // N s_6_14: jump b7
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
        // C s_8_0: const #2s : i
        let s_8_0: i128 = 2;
        // D s_8_1: read-var esizeshadow#1804:i64
        let s_8_1: i64 = fn_state.esizeshadow_1804;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: mul s_8_0 s_8_2
        let s_8_3: i128 = ((s_8_0) * (s_8_2));
        // D s_8_4: cast reint s_8_3 -> i64
        let s_8_4: i64 = (s_8_3 as i64);
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: cast reint s_8_5 -> i64
        let s_8_6: i64 = (s_8_5 as i64);
        // C s_8_7: const #2s : i
        let s_8_7: i128 = 2;
        // D s_8_8: read-var esizeshadow#1804:i64
        let s_8_8: i64 = fn_state.esizeshadow_1804;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: mul s_8_7 s_8_9
        let s_8_10: i128 = ((s_8_7) * (s_8_9));
        // D s_8_11: cast reint s_8_10 -> i64
        let s_8_11: i64 = (s_8_10 as i64);
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: cast reint s_8_12 -> i64
        let s_8_13: i64 = (s_8_12 as i64);
        // D s_8_14: read-var e:i64
        let s_8_14: i64 = fn_state.e;
        // D s_8_15: cast zx s_8_14 -> i
        let s_8_15: i128 = (i128::try_from(s_8_14).unwrap());
        // D s_8_16: cast zx s_8_13 -> i
        let s_8_16: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_17: read-var result:bv
        let s_8_17: Bits = fn_state.result;
        // D s_8_18: call Elem_read(s_8_17, s_8_15, s_8_16)
        let s_8_18: Bits = Elem_read(state, tracer, s_8_17, s_8_15, s_8_16);
        // D s_8_19: read-var sum:bv
        let s_8_19: Bits = fn_state.sum;
        // D s_8_20: add s_8_18 s_8_19
        let s_8_20: Bits = (s_8_18 + s_8_19);
        // D s_8_21: read-var e:i64
        let s_8_21: i64 = fn_state.e;
        // D s_8_22: cast zx s_8_21 -> i
        let s_8_22: i128 = (i128::try_from(s_8_21).unwrap());
        // D s_8_23: cast zx s_8_6 -> i
        let s_8_23: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_24: read-var result:bv
        let s_8_24: Bits = fn_state.result;
        // D s_8_25: call Elem_set(s_8_24, s_8_22, s_8_23, s_8_20)
        let s_8_25: Bits = Elem_set(state, tracer, s_8_24, s_8_22, s_8_23, s_8_20);
        // D s_8_26: write-var result <= s_8_25
        fn_state.result = s_8_25;
        // N s_8_27: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var datasizeshadow#1805:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1805;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var d:i64
        let s_9_3: i64 = fn_state.d;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: read-var result:bv
        let s_9_5: Bits = fn_state.result;
        // D s_9_6: call V_set(s_9_4, s_9_2, s_9_5)
        let s_9_6: () = V_set(state, tracer, s_9_4, s_9_2, s_9_5);
        // N s_9_7: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var datasizeshadow#1805:i64
        let s_10_0: i64 = fn_state.datasizeshadow_1805;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var d:i64
        let s_10_3: i64 = fn_state.d;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: call V_read(s_10_4, s_10_2)
        let s_10_5: Bits = V_read(state, tracer, s_10_4, s_10_2);
        // D s_10_6: write-var result <= s_10_5
        fn_state.result = s_10_5;
        // N s_10_7: jump b3
        return block_3(state, tracer, fn_state);
    }
}
