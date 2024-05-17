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
use integer_subrange::*;
use AArch64_AutoGen_SysOpsWrite::*;
use common::*;
pub fn AArch64_SysInstr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op0: i128,
    op1: i128,
    crn: i128,
    crm: i128,
    op2: i128,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        op0: i128,
        op1: i128,
        crn: i128,
        crm: i128,
        op2: i128,
        t: i128,
    }
    let fn_state = FunctionState {
        op0,
        op1,
        crn,
        crm,
        op2,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #1s : i
        let s_0_2: i128 = 1;
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // D s_0_4: read-var op0:i
        let s_0_4: i128 = fn_state.op0;
        // D s_0_5: call integer_subrange(s_0_4, s_0_2, s_0_3)
        let s_0_5: Bits = integer_subrange(state, tracer, s_0_4, s_0_2, s_0_3);
        // D s_0_6: cast reint s_0_5 -> u8
        let s_0_6: u8 = (s_0_5.value() as u8);
        // C s_0_7: const #2s : i
        let s_0_7: i128 = 2;
        // C s_0_8: const #0s : i
        let s_0_8: i128 = 0;
        // D s_0_9: read-var op1:i
        let s_0_9: i128 = fn_state.op1;
        // D s_0_10: call integer_subrange(s_0_9, s_0_7, s_0_8)
        let s_0_10: Bits = integer_subrange(state, tracer, s_0_9, s_0_7, s_0_8);
        // D s_0_11: cast reint s_0_10 -> u8
        let s_0_11: u8 = (s_0_10.value() as u8);
        // C s_0_12: const #3s : i
        let s_0_12: i128 = 3;
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // D s_0_14: read-var crn:i
        let s_0_14: i128 = fn_state.crn;
        // D s_0_15: call integer_subrange(s_0_14, s_0_12, s_0_13)
        let s_0_15: Bits = integer_subrange(state, tracer, s_0_14, s_0_12, s_0_13);
        // D s_0_16: cast reint s_0_15 -> u8
        let s_0_16: u8 = (s_0_15.value() as u8);
        // C s_0_17: const #2s : i
        let s_0_17: i128 = 2;
        // C s_0_18: const #0s : i
        let s_0_18: i128 = 0;
        // D s_0_19: read-var op2:i
        let s_0_19: i128 = fn_state.op2;
        // D s_0_20: call integer_subrange(s_0_19, s_0_17, s_0_18)
        let s_0_20: Bits = integer_subrange(state, tracer, s_0_19, s_0_17, s_0_18);
        // D s_0_21: cast reint s_0_20 -> u8
        let s_0_21: u8 = (s_0_20.value() as u8);
        // C s_0_22: const #3s : i
        let s_0_22: i128 = 3;
        // C s_0_23: const #0s : i
        let s_0_23: i128 = 0;
        // D s_0_24: read-var crm:i
        let s_0_24: i128 = fn_state.crm;
        // D s_0_25: call integer_subrange(s_0_24, s_0_22, s_0_23)
        let s_0_25: Bits = integer_subrange(state, tracer, s_0_24, s_0_22, s_0_23);
        // D s_0_26: cast reint s_0_25 -> u8
        let s_0_26: u8 = (s_0_25.value() as u8);
        // D s_0_27: read-var t:i
        let s_0_27: i128 = fn_state.t;
        // D s_0_28: call AArch64_AutoGen_SysOpsWrite(s_0_1, s_0_6, s_0_11, s_0_16, s_0_21, s_0_26, s_0_27)
        let s_0_28: () = AArch64_AutoGen_SysOpsWrite(
            state,
            tracer,
            s_0_1,
            s_0_6,
            s_0_11,
            s_0_16,
            s_0_21,
            s_0_26,
            s_0_27,
        );
        // N s_0_29: return
        return;
    }
}
