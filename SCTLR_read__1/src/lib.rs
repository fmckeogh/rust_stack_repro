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
use S1TranslationRegime__1::*;
use SCTLR_read::*;
use common::*;
pub fn SCTLR_read__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_5511: (),
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        gs_5511: (),
    }
    let fn_state = FunctionState {
        gs_5511,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call S1TranslationRegime__1(s_0_0)
        let s_0_1: u8 = S1TranslationRegime__1(state, tracer, s_0_0);
        // S s_0_2: tail-call SCTLR_read(s_0_1)
        return SCTLR_read(state, tracer, s_0_1);
    }
}
