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
use NewAccDesc::*;
use common::*;
pub fn CreateAccDescGCSSS1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
) -> ProductType9878976b5bcce9c9 {
    #[derive(Default)]
    struct FunctionState {
        el: u8,
    }
    let fn_state = FunctionState {
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_0_0: const #11u : u32
        let s_0_0: u32 = 11;
        // S s_0_1: tail-call NewAccDesc(s_0_0)
        return NewAccDesc(state, tracer, s_0_0);
    }
}