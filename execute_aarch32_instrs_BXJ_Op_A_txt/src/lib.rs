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
use BXWritePC::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_BXJ_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
    }
    let fn_state = FunctionState {
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m:i64
        let s_0_0: i64 = fn_state.m;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #6u : u32
        let s_0_3: u32 = 6;
        // D s_0_4: call BXWritePC(s_0_2, s_0_3)
        let s_0_4: () = BXWritePC(state, tracer, s_0_2, s_0_3);
        // N s_0_5: return
        return;
    }
}
