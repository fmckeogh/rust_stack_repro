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
use CurrentInstrSet::*;
use BranchWritePC::*;
use common::*;
pub fn ALUWritePC<T: Tracer>(state: &mut State, tracer: &T, address: u32) -> () {
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CurrentInstrSet(s_0_0)
        let s_0_1: u32 = CurrentInstrSet(state, tracer, s_0_0);
        // C s_0_2: const #1u : u32
        let s_0_2: u32 = 1;
        // S s_0_3: cmp-eq s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) == (s_0_2));
        // N s_0_4: branch s_0_3 b2 b1
        if s_0_3 {
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
        // D s_1_0: read-var address:u32
        let s_1_0: u32 = fn_state.address;
        // C s_1_1: const #6u : u32
        let s_1_1: u32 = 6;
        // D s_1_2: call BranchWritePC(s_1_0, s_1_1)
        let s_1_2: () = BranchWritePC(state, tracer, s_1_0, s_1_1);
        // N s_1_3: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var address:u32
        let s_2_0: u32 = fn_state.address;
        // C s_2_1: const #6u : u32
        let s_2_1: u32 = 6;
        // D s_2_2: call BXWritePC(s_2_0, s_2_1)
        let s_2_2: () = BXWritePC(state, tracer, s_2_0, s_2_1);
        // N s_2_3: return
        return;
    }
}
