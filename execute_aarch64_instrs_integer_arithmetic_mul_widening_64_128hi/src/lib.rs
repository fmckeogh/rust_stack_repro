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
pub fn execute_aarch64_instrs_integer_arithmetic_mul_widening_64_128hi<T: Tracer>(
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
        // D s_0_3: read-var n:i64
        let s_0_3: i64 = fn_state.n;
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: call X_read(s_0_4, s_0_2)
        let s_0_5: Bits = X_read(state, tracer, s_0_4, s_0_2);
        // D s_0_6: read-var datasize:i64
        let s_0_6: i64 = fn_state.datasize;
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: read-var m:i64
        let s_0_9: i64 = fn_state.m;
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_11: call X_read(s_0_10, s_0_8)
        let s_0_11: Bits = X_read(state, tracer, s_0_10, s_0_8);
        // D s_0_12: read-var is_unsigned:u8
        let s_0_12: bool = fn_state.is_unsigned;
        // D s_0_13: call asl_Int(s_0_5, s_0_12)
        let s_0_13: i128 = asl_Int(state, tracer, s_0_5, s_0_12);
        // D s_0_14: read-var is_unsigned:u8
        let s_0_14: bool = fn_state.is_unsigned;
        // D s_0_15: call asl_Int(s_0_11, s_0_14)
        let s_0_15: i128 = asl_Int(state, tracer, s_0_11, s_0_14);
        // D s_0_16: mul s_0_13 s_0_15
        let s_0_16: i128 = ((s_0_13) * (s_0_15));
        // C s_0_17: const #64s : i64
        let s_0_17: i64 = 64;
        // C s_0_18: const #127s : i
        let s_0_18: i128 = 127;
        // C s_0_19: const #64s : i
        let s_0_19: i128 = 64;
        // D s_0_20: call integer_subrange(s_0_16, s_0_18, s_0_19)
        let s_0_20: Bits = integer_subrange(state, tracer, s_0_16, s_0_18, s_0_19);
        // D s_0_21: cast reint s_0_20 -> u64
        let s_0_21: u64 = (s_0_20.value() as u64);
        // D s_0_22: read-var d:i64
        let s_0_22: i64 = fn_state.d;
        // D s_0_23: cast zx s_0_22 -> i
        let s_0_23: i128 = (i128::try_from(s_0_22).unwrap());
        // D s_0_24: cast zx s_0_21 -> bv
        let s_0_24: Bits = Bits::new(s_0_21 as u128, 64u16);
        // D s_0_25: call X_set(s_0_23, s_0_17, s_0_24)
        let s_0_25: () = X_set(state, tracer, s_0_23, s_0_17, s_0_24);
        // N s_0_26: return
        return;
    }
}
