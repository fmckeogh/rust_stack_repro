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
use asl_Int::*;
use X_read::*;
use IsZero::*;
use RoundTowardsZero::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch64_instrs_integer_arithmetic_div<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    m: i64,
    n: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: i128,
        operand1: Bits,
        datasizeshadow_1815: i64,
        operand2: Bits,
        d: i64,
        datasize: i64,
        m: i64,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        m,
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
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1815 <= s_0_2
        fn_state.datasizeshadow_1815 = s_0_2;
        // D s_0_4: read-var datasizeshadow#1815:i64
        let s_0_4: i64 = fn_state.datasizeshadow_1815;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: read-var n:i64
        let s_0_7: i64 = fn_state.n;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: call X_read(s_0_8, s_0_6)
        let s_0_9: Bits = X_read(state, tracer, s_0_8, s_0_6);
        // D s_0_10: write-var operand1 <= s_0_9
        fn_state.operand1 = s_0_9;
        // D s_0_11: read-var datasizeshadow#1815:i64
        let s_0_11: i64 = fn_state.datasizeshadow_1815;
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: read-var m:i64
        let s_0_14: i64 = fn_state.m;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: call X_read(s_0_15, s_0_13)
        let s_0_16: Bits = X_read(state, tracer, s_0_15, s_0_13);
        // D s_0_17: write-var operand2 <= s_0_16
        fn_state.operand2 = s_0_16;
        // D s_0_18: read-var operand2:bv
        let s_0_18: Bits = fn_state.operand2;
        // D s_0_19: call IsZero(s_0_18)
        let s_0_19: bool = IsZero(state, tracer, s_0_18);
        // N s_0_20: branch s_0_19 b3 b1
        if s_0_19 {
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
        // D s_1_0: read-var operand1:bv
        let s_1_0: Bits = fn_state.operand1;
        // D s_1_1: read-var is_unsigned:u8
        let s_1_1: bool = fn_state.is_unsigned;
        // D s_1_2: call asl_Int(s_1_0, s_1_1)
        let s_1_2: i128 = asl_Int(state, tracer, s_1_0, s_1_1);
        // D s_1_3: cast reint s_1_2 -> f64
        let s_1_3: f64 = (s_1_2 as f64);
        // D s_1_4: cast trunc s_1_3 -> f32
        let s_1_4: f32 = (s_1_3 as f32);
        // D s_1_5: read-var operand2:bv
        let s_1_5: Bits = fn_state.operand2;
        // D s_1_6: read-var is_unsigned:u8
        let s_1_6: bool = fn_state.is_unsigned;
        // D s_1_7: call asl_Int(s_1_5, s_1_6)
        let s_1_7: i128 = asl_Int(state, tracer, s_1_5, s_1_6);
        // D s_1_8: cast reint s_1_7 -> f64
        let s_1_8: f64 = (s_1_7 as f64);
        // D s_1_9: cast trunc s_1_8 -> f32
        let s_1_9: f32 = (s_1_8 as f32);
        // D s_1_10: div s_1_4 s_1_9
        let s_1_10: f32 = ((s_1_4) / (s_1_9));
        // D s_1_11: call RoundTowardsZero(s_1_10)
        let s_1_11: i128 = RoundTowardsZero(state, tracer, s_1_10);
        // D s_1_12: write-var result <= s_1_11
        fn_state.result = s_1_11;
        // N s_1_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var result:i
        let s_2_0: i128 = fn_state.result;
        // D s_2_1: read-var datasizeshadow#1815:i64
        let s_2_1: i64 = fn_state.datasizeshadow_1815;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #1s : i
        let s_2_4: i128 = 1;
        // D s_2_5: read-var datasizeshadow#1815:i64
        let s_2_5: i64 = fn_state.datasizeshadow_1815;
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
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: write-var result <= s_3_0
        fn_state.result = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
