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
use S1TranslationRegime::*;
use common::*;
pub fn S1TranslationRegime__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_3008: (),
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        gs_3008: (),
    }
    let fn_state = FunctionState {
        gs_3008,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: tail-call S1TranslationRegime(s_0_1)
        return S1TranslationRegime(state, tracer, s_0_1);
    }
}
