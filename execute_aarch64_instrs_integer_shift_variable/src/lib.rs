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
use ShiftReg::*;
use common::*;
pub fn execute_aarch64_instrs_integer_shift_variable<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    m: i64,
    n: i64,
    shift_type: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        datasize: i64,
        m: i64,
        n: i64,
        shift_type: u32,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        m,
        n,
        shift_type,
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
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: cast reint s_0_3 -> i64
        let s_0_4: i64 = (s_0_3 as i64);
        // D s_0_5: read-var m:i64
        let s_0_5: i64 = fn_state.m;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: call X_read(s_0_6, s_0_4)
        let s_0_7: Bits = X_read(state, tracer, s_0_6, s_0_4);
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (s_0_7.value() as i128);
        // D s_0_9: cast zx s_0_2 -> i
        let s_0_9: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_10: mod s_0_8 s_0_9
        let s_0_10: i128 = ((s_0_8) % (s_0_9));
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: cast zx s_0_2 -> i
        let s_0_12: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: read-var n:i64
        let s_0_14: i64 = fn_state.n;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: cast zx s_0_11 -> i
        let s_0_16: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_17: read-var shift_type:u32
        let s_0_17: u32 = fn_state.shift_type;
        // D s_0_18: call ShiftReg(s_0_15, s_0_17, s_0_16, s_0_13)
        let s_0_18: Bits = ShiftReg(state, tracer, s_0_15, s_0_17, s_0_16, s_0_13);
        // D s_0_19: cast zx s_0_2 -> i
        let s_0_19: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_20: cast reint s_0_19 -> i64
        let s_0_20: i64 = (s_0_19 as i64);
        // D s_0_21: read-var d:i64
        let s_0_21: i64 = fn_state.d;
        // D s_0_22: cast zx s_0_21 -> i
        let s_0_22: i128 = (i128::try_from(s_0_21).unwrap());
        // D s_0_23: call X_set(s_0_22, s_0_20, s_0_18)
        let s_0_23: () = X_set(state, tracer, s_0_22, s_0_20, s_0_18);
        // N s_0_24: return
        return;
    }
}
