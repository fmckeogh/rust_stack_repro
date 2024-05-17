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
use CheckVFPEnabled::*;
use S_set::*;
use R_read::*;
use S_read::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_VMOV_ss_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
    t: i64,
    t2: i64,
    to_arm_registers: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        t: i64,
        t2: i64,
        to_arm_registers: bool,
    }
    let fn_state = FunctionState {
        m,
        t,
        t2,
        to_arm_registers,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var to_arm_registers:u8
        let s_1_0: bool = fn_state.to_arm_registers;
        // N s_1_1: branch s_1_0 b3 b2
        if s_1_0 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var t:i64
        let s_2_0: i64 = fn_state.t;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call R_read(s_2_1)
        let s_2_2: u32 = R_read(state, tracer, s_2_1);
        // D s_2_3: read-var m:i64
        let s_2_3: i64 = fn_state.m;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: call S_set(s_2_4, s_2_2)
        let s_2_5: () = S_set(state, tracer, s_2_4, s_2_2);
        // C s_2_6: const #1s : i
        let s_2_6: i128 = 1;
        // D s_2_7: read-var m:i64
        let s_2_7: i64 = fn_state.m;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: add s_2_8 s_2_6
        let s_2_9: i128 = (s_2_8 + s_2_6);
        // D s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // D s_2_11: read-var t2:i64
        let s_2_11: i64 = fn_state.t2;
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: call R_read(s_2_12)
        let s_2_13: u32 = R_read(state, tracer, s_2_12);
        // D s_2_14: cast zx s_2_10 -> i
        let s_2_14: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_15: call S_set(s_2_14, s_2_13)
        let s_2_15: () = S_set(state, tracer, s_2_14, s_2_13);
        // N s_2_16: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var m:i64
        let s_3_0: i64 = fn_state.m;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call S_read(s_3_1)
        let s_3_2: u32 = S_read(state, tracer, s_3_1);
        // D s_3_3: read-var t:i64
        let s_3_3: i64 = fn_state.t;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: call R_set(s_3_4, s_3_2)
        let s_3_5: () = R_set(state, tracer, s_3_4, s_3_2);
        // C s_3_6: const #1s : i
        let s_3_6: i128 = 1;
        // D s_3_7: read-var m:i64
        let s_3_7: i64 = fn_state.m;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: add s_3_8 s_3_6
        let s_3_9: i128 = (s_3_8 + s_3_6);
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: call S_read(s_3_11)
        let s_3_12: u32 = S_read(state, tracer, s_3_11);
        // D s_3_13: read-var t2:i64
        let s_3_13: i64 = fn_state.t2;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: call R_set(s_3_14, s_3_12)
        let s_3_15: () = R_set(state, tracer, s_3_14, s_3_12);
        // N s_3_16: return
        return;
    }
}
