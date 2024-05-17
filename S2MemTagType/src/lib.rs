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
use common::*;
pub fn S2MemTagType<T: Tracer>(
    state: &mut State,
    tracer: &T,
    s2_memattrs: ProductTypef170cab34335b70c,
    s1_tagtype: u32,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        s2_memattrs: ProductTypef170cab34335b70c,
        s1_tagtype: u32,
    }
    let fn_state = FunctionState {
        s2_memattrs,
        s1_tagtype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
