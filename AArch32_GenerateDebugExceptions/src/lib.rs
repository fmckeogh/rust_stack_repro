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
use CurrentSecurityState::*;
use AArch32_GenerateDebugExceptionsFrom::*;
use common::*;
pub fn AArch32_GenerateDebugExceptions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_5101: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_5101: (),
    }
    let fn_state = FunctionState {
        gs_5101,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CurrentSecurityState(s_0_0)
        let s_0_1: u32 = CurrentSecurityState(state, tracer, s_0_0);
        // C s_0_2: const #16975u : u32
        let s_0_2: u32 = 16975;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: tail-call AArch32_GenerateDebugExceptionsFrom(s_0_3, s_0_1)
        return AArch32_GenerateDebugExceptionsFrom(state, tracer, s_0_3, s_0_1);
    }
}
