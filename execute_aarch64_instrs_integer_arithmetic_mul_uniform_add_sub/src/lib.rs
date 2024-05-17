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
use X_set::*;
use X_read::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch64_instrs_integer_arithmetic_mul_uniform_add_sub<T: Tracer>(
    state: &mut State,
    tracer: &T,
    a: i64,
    d: i64,
    datasize: i64,
    destsize: i64,
    m: i64,
    n: i64,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        destsizeshadow_1749: i64,
        result: i128,
        operand1: Bits,
        operand3: Bits,
        operand2: Bits,
        a: i64,
        d: i64,
        datasize: i64,
        destsize: i64,
        m: i64,
        n: i64,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        a,
        d,
        datasize,
        destsize,
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
        // D s_0_0: read-var destsize:i64
        let s_0_0: i64 = fn_state.destsize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var destsizeshadow#1749 <= s_0_2
        fn_state.destsizeshadow_1749 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: read-var n:i64
        let s_0_9: i64 = fn_state.n;
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_11: call X_read(s_0_10, s_0_8)
        let s_0_11: Bits = X_read(state, tracer, s_0_10, s_0_8);
        // D s_0_12: write-var operand1 <= s_0_11
        fn_state.operand1 = s_0_11;
        // D s_0_13: cast zx s_0_6 -> i
        let s_0_13: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_14: cast reint s_0_13 -> i64
        let s_0_14: i64 = (s_0_13 as i64);
        // D s_0_15: read-var m:i64
        let s_0_15: i64 = fn_state.m;
        // D s_0_16: cast zx s_0_15 -> i
        let s_0_16: i128 = (i128::try_from(s_0_15).unwrap());
        // D s_0_17: call X_read(s_0_16, s_0_14)
        let s_0_17: Bits = X_read(state, tracer, s_0_16, s_0_14);
        // D s_0_18: write-var operand2 <= s_0_17
        fn_state.operand2 = s_0_17;
        // D s_0_19: read-var destsizeshadow#1749:i64
        let s_0_19: i64 = fn_state.destsizeshadow_1749;
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: cast reint s_0_20 -> i64
        let s_0_21: i64 = (s_0_20 as i64);
        // D s_0_22: read-var a:i64
        let s_0_22: i64 = fn_state.a;
        // D s_0_23: cast zx s_0_22 -> i
        let s_0_23: i128 = (i128::try_from(s_0_22).unwrap());
        // D s_0_24: call X_read(s_0_23, s_0_21)
        let s_0_24: Bits = X_read(state, tracer, s_0_23, s_0_21);
        // D s_0_25: write-var operand3 <= s_0_24
        fn_state.operand3 = s_0_24;
        // D s_0_26: read-var sub_op:u8
        let s_0_26: bool = fn_state.sub_op;
        // N s_0_27: branch s_0_26 b3 b1
        if s_0_26 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var operand3:bv
        let s_1_0: Bits = fn_state.operand3;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (s_1_0.value() as i128);
        // D s_1_2: read-var operand1:bv
        let s_1_2: Bits = fn_state.operand1;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (s_1_2.value() as i128);
        // D s_1_4: read-var operand2:bv
        let s_1_4: Bits = fn_state.operand2;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (s_1_4.value() as i128);
        // D s_1_6: mul s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) * (s_1_5));
        // D s_1_7: add s_1_1 s_1_6
        let s_1_7: i128 = (s_1_1 + s_1_6);
        // D s_1_8: write-var result <= s_1_7
        fn_state.result = s_1_7;
        // N s_1_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var result:i
        let s_2_0: i128 = fn_state.result;
        // D s_2_1: read-var destsizeshadow#1749:i64
        let s_2_1: i64 = fn_state.destsizeshadow_1749;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #1s : i
        let s_2_4: i128 = 1;
        // D s_2_5: read-var destsizeshadow#1749:i64
        let s_2_5: i64 = fn_state.destsizeshadow_1749;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: sub s_2_6 s_2_4
        let s_2_7: i128 = ((s_2_6) - (s_2_4));
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // C s_2_9: const #0s : i
        let s_2_9: i128 = 0;
        // D s_2_10: cast zx s_2_8 -> i
        let s_2_10: i128 = (i128::try_from(s_2_8).unwrap());
        // D s_2_11: call integer_subrange(s_2_0, s_2_10, s_2_9)
        let s_2_11: Bits = integer_subrange(state, tracer, s_2_0, s_2_10, s_2_9);
        // D s_2_12: read-var d:i64
        let s_2_12: i64 = fn_state.d;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_14: call X_set(s_2_13, s_2_3, s_2_11)
        let s_2_14: () = X_set(state, tracer, s_2_13, s_2_3, s_2_11);
        // N s_2_15: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var operand3:bv
        let s_3_0: Bits = fn_state.operand3;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (s_3_0.value() as i128);
        // D s_3_2: read-var operand1:bv
        let s_3_2: Bits = fn_state.operand1;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (s_3_2.value() as i128);
        // D s_3_4: read-var operand2:bv
        let s_3_4: Bits = fn_state.operand2;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (s_3_4.value() as i128);
        // D s_3_6: mul s_3_3 s_3_5
        let s_3_6: i128 = ((s_3_3) * (s_3_5));
        // D s_3_7: sub s_3_1 s_3_6
        let s_3_7: i128 = ((s_3_1) - (s_3_6));
        // D s_3_8: write-var result <= s_3_7
        fn_state.result = s_3_7;
        // N s_3_9: jump b2
        return block_2(state, tracer, fn_state);
    }
}
