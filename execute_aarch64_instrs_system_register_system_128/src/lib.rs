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
use AArch64_SysRegRead128::*;
use AArch64_SysRegWrite128::*;
use common::*;
pub fn execute_aarch64_instrs_system_register_system_128<T: Tracer>(
    state: &mut State,
    tracer: &T,
    read: bool,
    sys_crm: i64,
    sys_crn: i64,
    sys_op0: i64,
    sys_op1: i64,
    sys_op2: i64,
    t: i64,
    t2: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        read: bool,
        sys_crm: i64,
        sys_crn: i64,
        sys_op0: i64,
        sys_op1: i64,
        sys_op2: i64,
        t: i64,
        t2: i64,
    }
    let fn_state = FunctionState {
        read,
        sys_crm,
        sys_crn,
        sys_op0,
        sys_op1,
        sys_op2,
        t,
        t2,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var read:u8
        let s_0_0: bool = fn_state.read;
        // N s_0_1: branch s_0_0 b2 b1
        if s_0_0 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var sys_op0:i64
        let s_1_0: i64 = fn_state.sys_op0;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: read-var sys_op1:i64
        let s_1_2: i64 = fn_state.sys_op1;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var sys_crn:i64
        let s_1_4: i64 = fn_state.sys_crn;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: read-var sys_crm:i64
        let s_1_6: i64 = fn_state.sys_crm;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: read-var sys_op2:i64
        let s_1_8: i64 = fn_state.sys_op2;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var t:i64
        let s_1_10: i64 = fn_state.t;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: read-var t2:i64
        let s_1_12: i64 = fn_state.t2;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: call AArch64_SysRegWrite128(s_1_1, s_1_3, s_1_5, s_1_7, s_1_9, s_1_11, s_1_13)
        let s_1_14: () = AArch64_SysRegWrite128(
            state,
            tracer,
            s_1_1,
            s_1_3,
            s_1_5,
            s_1_7,
            s_1_9,
            s_1_11,
            s_1_13,
        );
        // N s_1_15: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var sys_op0:i64
        let s_2_0: i64 = fn_state.sys_op0;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: read-var sys_op1:i64
        let s_2_2: i64 = fn_state.sys_op1;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: read-var sys_crn:i64
        let s_2_4: i64 = fn_state.sys_crn;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: read-var sys_crm:i64
        let s_2_6: i64 = fn_state.sys_crm;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: read-var sys_op2:i64
        let s_2_8: i64 = fn_state.sys_op2;
        // D s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (i128::try_from(s_2_8).unwrap());
        // D s_2_10: read-var t:i64
        let s_2_10: i64 = fn_state.t;
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_12: read-var t2:i64
        let s_2_12: i64 = fn_state.t2;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_14: call AArch64_SysRegRead128(s_2_1, s_2_3, s_2_5, s_2_7, s_2_9, s_2_11, s_2_13)
        let s_2_14: () = AArch64_SysRegRead128(
            state,
            tracer,
            s_2_1,
            s_2_3,
            s_2_5,
            s_2_7,
            s_2_9,
            s_2_11,
            s_2_13,
        );
        // N s_2_15: return
        return;
    }
}
