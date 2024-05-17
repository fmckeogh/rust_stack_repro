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
use TranslationRegime::*;
use HasUnprivileged::*;
use common::*;
pub fn PtrHasUpperAndLowerAddRanges<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_13904: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_13904: (),
    }
    let fn_state = FunctionState {
        gs_13904,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call TranslationRegime(s_0_1)
        let s_0_2: u32 = TranslationRegime(state, tracer, s_0_1);
        // D s_0_3: tail-call HasUnprivileged(s_0_2)
        return HasUnprivileged(state, tracer, s_0_2);
    }
}
