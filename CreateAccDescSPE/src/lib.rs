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
use GenMPAMatEL::*;
use common::*;
pub fn CreateAccDescSPE<T: Tracer>(
    state: &mut State,
    tracer: &T,
    owning_ss: u32,
    owning_el: u8,
) -> ProductType9878976b5bcce9c9 {
    #[derive(Default)]
    struct FunctionState {
        owning_ss: u32,
        owning_el: u8,
    }
    let fn_state = FunctionState {
        owning_ss,
        owning_el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_0_0: const #10u : u32
        let s_0_0: u32 = 10;
        // S s_0_1: call NewAccDesc(s_0_0)
        let s_0_1: ProductType9878976b5bcce9c9 = NewAccDesc(state, tracer, s_0_0);
        // C s_0_2: const #10u : u32
        let s_0_2: u32 = 10;
        // D s_0_3: read-var owning_el:u8
        let s_0_3: u8 = fn_state.owning_el;
        // D s_0_4: call GenMPAMatEL(s_0_2, s_0_3)
        let s_0_4: ProductTypee79b4310dbe79c8c = GenMPAMatEL(
            state,
            tracer,
            s_0_2,
            s_0_3,
        );
        // N s_0_5: return s_0_1
        return s_0_1;
    }
}
