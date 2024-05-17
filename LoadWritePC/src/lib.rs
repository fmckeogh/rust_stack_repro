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
use common::*;
pub fn LoadWritePC<T: Tracer>(state: &mut State, tracer: &T, address: u32) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u32,
    }
    let fn_state = FunctionState {
        address,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var address:u32
        let s_0_0: u32 = fn_state.address;
        // C s_0_1: const #6u : u32
        let s_0_1: u32 = 6;
        // D s_0_2: call BXWritePC(s_0_0, s_0_1)
        let s_0_2: () = BXWritePC(state, tracer, s_0_0, s_0_1);
        // N s_0_3: return
        return;
    }
}
