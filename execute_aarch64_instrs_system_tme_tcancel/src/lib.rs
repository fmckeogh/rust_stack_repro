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
use FailTransaction__1::*;
use IsTMEEnabled::*;
use common::*;
pub fn execute_aarch64_instrs_system_tme_tcancel<T: Tracer>(
    state: &mut State,
    tracer: &T,
    reason: u16,
    retry: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        reason: u16,
        retry: bool,
    }
    let fn_state = FunctionState {
        reason,
        retry,
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
        // S s_0_1: call IsTMEEnabled(s_0_0)
        let s_0_1: bool = IsTMEEnabled(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b4 b1
        if s_0_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #100180u : u32
        let s_1_0: u32 = 100180;
        // D s_1_1: read-reg s_1_0:i
        let s_1_1: i128 = {
            let value = state.read_register::<i128>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #0s : i
        let s_1_2: i128 = 0;
        // D s_1_3: cmp-gt s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) > (s_1_2));
        // N s_1_4: branch s_1_3 b3 b2
        if s_1_3 {
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
        // N s_2_0: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u32
        let s_3_0: u32 = 0;
        // D s_3_1: read-var retry:u8
        let s_3_1: bool = fn_state.retry;
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // D s_3_3: read-var reason:u15
        let s_3_3: u16 = fn_state.reason;
        // D s_3_4: call FailTransaction__1(s_3_0, s_3_1, s_3_2, s_3_3)
        let s_3_4: () = FailTransaction__1(state, tracer, s_3_0, s_3_1, s_3_2, s_3_3);
        // N s_3_5: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}
