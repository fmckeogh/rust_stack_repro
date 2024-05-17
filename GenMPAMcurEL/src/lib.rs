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
use GenMPAMatEL::*;
use common::*;
pub fn GenMPAMcurEL<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acctype: u32,
) -> ProductTypee79b4310dbe79c8c {
    #[derive(Default)]
    struct FunctionState {
        acctype: u32,
    }
    let fn_state = FunctionState {
        acctype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: read-var acctype:u32
        let s_0_2: u32 = fn_state.acctype;
        // D s_0_3: tail-call GenMPAMatEL(s_0_2, s_0_1)
        return GenMPAMatEL(state, tracer, s_0_2, s_0_1);
    }
}
