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
pub fn AArch64_ResetControlRegisters<T: Tracer>(
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
        // C s_0_1: const #16998u : u32
        let s_0_1: u32 = 16998;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<bool>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // C s_0_3: const #0u : u8
        let s_0_3: bool = false;
        // C s_0_4: const #16989u : u32
        let s_0_4: u32 = 16989;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<bool>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // C s_0_6: const #0u : u8
        let s_0_6: bool = false;
        // C s_0_7: const #16969u : u32
        let s_0_7: u32 = 16969;
        // N s_0_8: write-reg s_0_7 <= s_0_6
        let s_0_8: () = {
            state.write_register::<bool>(s_0_7 as isize, s_0_6);
            tracer.write_register(s_0_7 as isize, s_0_6);
        };
        // C s_0_9: const #0u : u8
        let s_0_9: bool = false;
        // C s_0_10: const #16976u : u32
        let s_0_10: u32 = 16976;
        // N s_0_11: write-reg s_0_10 <= s_0_9
        let s_0_11: () = {
            state.write_register::<bool>(s_0_10 as isize, s_0_9);
            tracer.write_register(s_0_10 as isize, s_0_9);
        };
        // D s_0_12: read-var cold_reset:u8
        let s_0_12: bool = fn_state.cold_reset;
        // D s_0_13: call ResetControlRegisters(s_0_12)
        let s_0_13: () = ResetControlRegisters(state, tracer, s_0_12);
        // C s_0_14: const #() : ()
        let s_0_14: () = ();
        // S s_0_15: call _TLBInvalidate(s_0_14)
        let s_0_15: () = u_TLBInvalidate(state, tracer, s_0_14);
        // N s_0_16: return
        return;
    }
}
