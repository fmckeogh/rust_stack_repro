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
use integer_subrange::*;
use common::*;
pub fn execute_aarch64_instrs_integer_arithmetic_mul_widening_32_64<T: Tracer>(
    state: &mut State,
    tracer: &T,
    a: i64,
    d: i64,
    datasize: i64,
    destsize: i64,
    m: i64,
    n: i64,
    sub_op: bool,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
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
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        a,
        d,
        datasize,
        destsize,
        m,
        n,
        sub_op,
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
        // D s_0_3: read-var n:i64
        let s_0_3: i64 = fn_state.n;
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: call X_read(s_0_4, s_0_2)
        let s_0_5: Bits = X_read(state, tracer, s_0_4, s_0_2);
        // D s_0_6: write-var operand1 <= s_0_5
        fn_state.operand1 = s_0_5;
        // D s_0_7: read-var datasize:i64
        let s_0_7: i64 = fn_state.datasize;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // D s_0_10: read-var m:i64
        let s_0_10: i64 = fn_state.m;
        // D s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_12: call X_read(s_0_11, s_0_9)
        let s_0_12: Bits = X_read(state, tracer, s_0_11, s_0_9);
        // D s_0_13: write-var operand2 <= s_0_12
        fn_state.operand2 = s_0_12;
        // D s_0_14: read-var destsize:i64
        let s_0_14: i64 = fn_state.destsize;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: cast reint s_0_15 -> i64
        let s_0_16: i64 = (s_0_15 as i64);
        // D s_0_17: read-var a:i64
        let s_0_17: i64 = fn_state.a;
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_19: call X_read(s_0_18, s_0_16)
        let s_0_19: Bits = X_read(state, tracer, s_0_18, s_0_16);
        // D s_0_20: write-var operand3 <= s_0_19
        fn_state.operand3 = s_0_19;
        // D s_0_21: read-var sub_op:u8
        let s_0_21: bool = fn_state.sub_op;
        // N s_0_22: branch s_0_21 b3 b1
        if s_0_21 {
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
        // D s_1_1: read-var is_unsigned:u8
        let s_1_1: bool = fn_state.is_unsigned;
        // D s_1_2: call asl_Int(s_1_0, s_1_1)
        let s_1_2: i128 = asl_Int(state, tracer, s_1_0, s_1_1);
        // D s_1_3: read-var operand1:bv
        let s_1_3: Bits = fn_state.operand1;
        // D s_1_4: read-var is_unsigned:u8
        let s_1_4: bool = fn_state.is_unsigned;
        // D s_1_5: call asl_Int(s_1_3, s_1_4)
        let s_1_5: i128 = asl_Int(state, tracer, s_1_3, s_1_4);
        // D s_1_6: read-var operand2:bv
        let s_1_6: Bits = fn_state.operand2;
        // D s_1_7: read-var is_unsigned:u8
        let s_1_7: bool = fn_state.is_unsigned;
        // D s_1_8: call asl_Int(s_1_6, s_1_7)
        let s_1_8: i128 = asl_Int(state, tracer, s_1_6, s_1_7);
        // D s_1_9: mul s_1_5 s_1_8
        let s_1_9: i128 = ((s_1_5) * (s_1_8));
        // D s_1_10: add s_1_2 s_1_9
        let s_1_10: i128 = (s_1_2 + s_1_9);
        // D s_1_11: write-var result <= s_1_10
        fn_state.result = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var result:i
        let s_2_0: i128 = fn_state.result;
        // C s_2_1: const #64s : i64
        let s_2_1: i64 = 64;
        // C s_2_2: const #63s : i
        let s_2_2: i128 = 63;
        // C s_2_3: const #0s : i
        let s_2_3: i128 = 0;
        // D s_2_4: call integer_subrange(s_2_0, s_2_2, s_2_3)
        let s_2_4: Bits = integer_subrange(state, tracer, s_2_0, s_2_2, s_2_3);
        // D s_2_5: cast reint s_2_4 -> u64
        let s_2_5: u64 = (s_2_4.value() as u64);
        // D s_2_6: read-var d:i64
        let s_2_6: i64 = fn_state.d;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: cast zx s_2_5 -> bv
        let s_2_8: Bits = Bits::new(s_2_5 as u128, 64u16);
        // D s_2_9: call X_set(s_2_7, s_2_1, s_2_8)
        let s_2_9: () = X_set(state, tracer, s_2_7, s_2_1, s_2_8);
        // N s_2_10: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var operand3:bv
        let s_3_0: Bits = fn_state.operand3;
        // D s_3_1: read-var is_unsigned:u8
        let s_3_1: bool = fn_state.is_unsigned;
        // D s_3_2: call asl_Int(s_3_0, s_3_1)
        let s_3_2: i128 = asl_Int(state, tracer, s_3_0, s_3_1);
        // D s_3_3: read-var operand1:bv
        let s_3_3: Bits = fn_state.operand1;
        // D s_3_4: read-var is_unsigned:u8
        let s_3_4: bool = fn_state.is_unsigned;
        // D s_3_5: call asl_Int(s_3_3, s_3_4)
        let s_3_5: i128 = asl_Int(state, tracer, s_3_3, s_3_4);
        // D s_3_6: read-var operand2:bv
        let s_3_6: Bits = fn_state.operand2;
        // D s_3_7: read-var is_unsigned:u8
        let s_3_7: bool = fn_state.is_unsigned;
        // D s_3_8: call asl_Int(s_3_6, s_3_7)
        let s_3_8: i128 = asl_Int(state, tracer, s_3_6, s_3_7);
        // D s_3_9: mul s_3_5 s_3_8
        let s_3_9: i128 = ((s_3_5) * (s_3_8));
        // D s_3_10: sub s_3_2 s_3_9
        let s_3_10: i128 = ((s_3_2) - (s_3_9));
        // D s_3_11: write-var result <= s_3_10
        fn_state.result = s_3_10;
        // N s_3_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
