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
use SPEBranch__1::*;
use common::*;
pub fn SPEBranch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target: Bits,
    branch_type: u32,
    conditional: bool,
    taken_flag: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        target: Bits,
        branch_type: u32,
        conditional: bool,
        taken_flag: bool,
    }
    let fn_state = FunctionState {
        target,
        branch_type,
        conditional,
        taken_flag,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: read-var target:bv
        let s_0_1: Bits = fn_state.target;
        // D s_0_2: read-var branch_type:u32
        let s_0_2: u32 = fn_state.branch_type;
        // D s_0_3: read-var conditional:u8
        let s_0_3: bool = fn_state.conditional;
        // D s_0_4: read-var taken_flag:u8
        let s_0_4: bool = fn_state.taken_flag;
        // D s_0_5: call SPEBranch__1(s_0_1, s_0_2, s_0_3, s_0_4, s_0_0)
        let s_0_5: () = SPEBranch__1(state, tracer, s_0_1, s_0_2, s_0_3, s_0_4, s_0_0);
        // N s_0_6: return
        return;
    }
}
