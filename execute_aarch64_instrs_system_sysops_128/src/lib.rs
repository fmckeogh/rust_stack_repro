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
use AArch64_SysInstr128::*;
use common::*;
pub fn execute_aarch64_instrs_system_sysops_128<T: Tracer>(
    state: &mut State,
    tracer: &T,
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
        sys_crm: i64,
        sys_crn: i64,
        sys_op0: i64,
        sys_op1: i64,
        sys_op2: i64,
        t: i64,
        t2: i64,
    }
    let fn_state = FunctionState {
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
        // D s_0_0: read-var sys_op0:i64
        let s_0_0: i64 = fn_state.sys_op0;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: read-var sys_op1:i64
        let s_0_2: i64 = fn_state.sys_op1;
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: read-var sys_crn:i64
        let s_0_4: i64 = fn_state.sys_crn;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: read-var sys_crm:i64
        let s_0_6: i64 = fn_state.sys_crm;
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: read-var sys_op2:i64
        let s_0_8: i64 = fn_state.sys_op2;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: read-var t:i64
        let s_0_10: i64 = fn_state.t;
        // D s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_12: read-var t2:i64
        let s_0_12: i64 = fn_state.t2;
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (i128::try_from(s_0_12).unwrap());
        // D s_0_14: call AArch64_SysInstr128(s_0_1, s_0_3, s_0_5, s_0_7, s_0_9, s_0_11, s_0_13)
        let s_0_14: () = AArch64_SysInstr128(
            state,
            tracer,
            s_0_1,
            s_0_3,
            s_0_5,
            s_0_7,
            s_0_9,
            s_0_11,
            s_0_13,
        );
        // N s_0_15: return
        return;
    }
}
