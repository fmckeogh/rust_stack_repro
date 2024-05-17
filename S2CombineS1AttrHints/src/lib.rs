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
pub fn S2CombineS1AttrHints<T: Tracer>(
    state: &mut State,
    tracer: &T,
    s1_attrhints: ProductType183e6678e5239c85,
    s2_attrhints: ProductType183e6678e5239c85,
) -> ProductType183e6678e5239c85 {
    #[derive(Default)]
    struct FunctionState {
        s1_attrhints: ProductType183e6678e5239c85,
        s2_attrhints: ProductType183e6678e5239c85,
    }
    let fn_state = FunctionState {
        s1_attrhints,
        s2_attrhints,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType183e6678e5239c85 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // N s_0_1: panic s_0_0
        panic!("{:?}", (s_0_0));
    }
}
