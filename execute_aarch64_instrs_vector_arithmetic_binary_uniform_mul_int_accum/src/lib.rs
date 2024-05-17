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
use Elem_read::*;
use V_set::*;
use integer_subrange::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_accum<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        product: Bits,
        e: i64,
        datasizeshadow_1753: i64,
        operand3: Bits,
        gs_165040: i64,
        result: Bits,
        operand1: Bits,
        esizeshadow_1752: i64,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
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
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#1752 <= s_0_2
        fn_state.esizeshadow_1752 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1753 <= s_0_6
        fn_state.datasizeshadow_1753 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1753:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1753;
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
        // D s_1_6: write-var operand1 <= s_1_5
        fn_state.operand1 = s_1_5;
        // D s_1_7: read-var datasizeshadow#1753:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1753;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var m:i64
        let s_1_10: i64 = fn_state.m;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call V_read(s_1_11, s_1_9)
        let s_1_12: Bits = V_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: write-var operand2 <= s_1_12
        fn_state.operand2 = s_1_12;
        // D s_1_14: read-var datasizeshadow#1753:i64
        let s_1_14: i64 = fn_state.datasizeshadow_1753;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var d:i64
        let s_1_17: i64 = fn_state.d;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: call V_read(s_1_18, s_1_16)
        let s_1_19: Bits = V_read(state, tracer, s_1_18, s_1_16);
        // D s_1_20: write-var operand3 <= s_1_19
        fn_state.operand3 = s_1_19;
        // C s_1_21: const #0s : i64
        let s_1_21: i64 = 0;
        // C s_1_22: const #1s : i
        let s_1_22: i128 = 1;
        // D s_1_23: read-var elements:i
        let s_1_23: i128 = fn_state.elements;
        // D s_1_24: sub s_1_23 s_1_22
        let s_1_24: i128 = ((s_1_23) - (s_1_22));
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var gs#165040 <= s_1_25
        fn_state.gs_165040 = s_1_25;
        // D s_1_27: write-var e <= s_1_21
        fn_state.e = s_1_21;
        // N s_1_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#165040:i64
        let s_2_1: i64 = fn_state.gs_165040;
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
        // D s_3_0: read-var esizeshadow#1752:i64
        let s_3_0: i64 = fn_state.esizeshadow_1752;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand1:bv
        let s_3_6: Bits = fn_state.operand1;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: read-var esizeshadow#1752:i64
        let s_3_8: i64 = fn_state.esizeshadow_1752;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: read-var e:i64
        let s_3_11: i64 = fn_state.e;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: cast zx s_3_10 -> i
        let s_3_13: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_14: read-var operand2:bv
        let s_3_14: Bits = fn_state.operand2;
        // D s_3_15: call Elem_read(s_3_14, s_3_12, s_3_13)
        let s_3_15: Bits = Elem_read(state, tracer, s_3_14, s_3_12, s_3_13);
        // D s_3_16: cast zx s_3_7 -> i
        let s_3_16: i128 = (s_3_7.value() as i128);
        // D s_3_17: cast zx s_3_15 -> i
        let s_3_17: i128 = (s_3_15.value() as i128);
        // D s_3_18: mul s_3_16 s_3_17
        let s_3_18: i128 = ((s_3_16) * (s_3_17));
        // C s_3_19: const #1s : i
        let s_3_19: i128 = 1;
        // D s_3_20: read-var esizeshadow#1752:i64
        let s_3_20: i64 = fn_state.esizeshadow_1752;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: sub s_3_21 s_3_19
        let s_3_22: i128 = ((s_3_21) - (s_3_19));
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // C s_3_24: const #0s : i
        let s_3_24: i128 = 0;
        // D s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_26: call integer_subrange(s_3_18, s_3_25, s_3_24)
        let s_3_26: Bits = integer_subrange(state, tracer, s_3_18, s_3_25, s_3_24);
        // D s_3_27: write-var product <= s_3_26
        fn_state.product = s_3_26;
        // D s_3_28: read-var sub_op:u8
        let s_3_28: bool = fn_state.sub_op;
        // N s_3_29: branch s_3_28 b6 b4
        if s_3_28 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esizeshadow#1752:i64
        let s_4_0: i64 = fn_state.esizeshadow_1752;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var esizeshadow#1752:i64
        let s_4_3: i64 = fn_state.esizeshadow_1752;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: read-var e:i64
        let s_4_6: i64 = fn_state.e;
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: cast zx s_4_5 -> i
        let s_4_8: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_9: read-var operand3:bv
        let s_4_9: Bits = fn_state.operand3;
        // D s_4_10: call Elem_read(s_4_9, s_4_7, s_4_8)
        let s_4_10: Bits = Elem_read(state, tracer, s_4_9, s_4_7, s_4_8);
        // D s_4_11: read-var product:bv
        let s_4_11: Bits = fn_state.product;
        // D s_4_12: add s_4_10 s_4_11
        let s_4_12: Bits = (s_4_10 + s_4_11);
        // D s_4_13: read-var e:i64
        let s_4_13: i64 = fn_state.e;
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_15: cast zx s_4_2 -> i
        let s_4_15: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_16: read-var result:bv
        let s_4_16: Bits = fn_state.result;
        // D s_4_17: call Elem_set(s_4_16, s_4_14, s_4_15, s_4_12)
        let s_4_17: Bits = Elem_set(state, tracer, s_4_16, s_4_14, s_4_15, s_4_12);
        // D s_4_18: write-var result <= s_4_17
        fn_state.result = s_4_17;
        // N s_4_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var e <= s_5_2
        fn_state.e = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#1752:i64
        let s_6_0: i64 = fn_state.esizeshadow_1752;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var esizeshadow#1752:i64
        let s_6_3: i64 = fn_state.esizeshadow_1752;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: read-var e:i64
        let s_6_6: i64 = fn_state.e;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: cast zx s_6_5 -> i
        let s_6_8: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_9: read-var operand3:bv
        let s_6_9: Bits = fn_state.operand3;
        // D s_6_10: call Elem_read(s_6_9, s_6_7, s_6_8)
        let s_6_10: Bits = Elem_read(state, tracer, s_6_9, s_6_7, s_6_8);
        // D s_6_11: read-var product:bv
        let s_6_11: Bits = fn_state.product;
        // D s_6_12: sub s_6_10 s_6_11
        let s_6_12: Bits = ((s_6_10) - (s_6_11));
        // D s_6_13: read-var e:i64
        let s_6_13: i64 = fn_state.e;
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: cast zx s_6_2 -> i
        let s_6_15: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_16: read-var result:bv
        let s_6_16: Bits = fn_state.result;
        // D s_6_17: call Elem_set(s_6_16, s_6_14, s_6_15, s_6_12)
        let s_6_17: Bits = Elem_set(state, tracer, s_6_16, s_6_14, s_6_15, s_6_12);
        // D s_6_18: write-var result <= s_6_17
        fn_state.result = s_6_17;
        // N s_6_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasizeshadow#1753:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1753;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var result:bv
        let s_7_5: Bits = fn_state.result;
        // D s_7_6: call V_set(s_7_4, s_7_2, s_7_5)
        let s_7_6: () = V_set(state, tracer, s_7_4, s_7_2, s_7_5);
        // N s_7_7: return
        return;
    }
}
