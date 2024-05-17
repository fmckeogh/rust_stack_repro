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
use u_TLBInvalidate::*;
use ResetControlRegisters::*;
use common::*;
pub fn AArch32_ResetControlRegisters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cold_reset: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        cold_reset: bool,
    }
    let fn_state = FunctionState {
        cold_reset,
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
        // C s_0_1: const #16991u : u32
        let s_0_1: u32 = 16991;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<bool>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // D s_0_3: read-var cold_reset:u8
        let s_0_3: bool = fn_state.cold_reset;
        // D s_0_4: call ResetControlRegisters(s_0_3)
        let s_0_4: () = ResetControlRegisters(state, tracer, s_0_3);
        // C s_0_5: const #() : ()
        let s_0_5: () = ();
        // S s_0_6: call _TLBInvalidate(s_0_5)
        let s_0_6: () = u_TLBInvalidate(state, tracer, s_0_5);
        // N s_0_7: return
        return;
    }
}
