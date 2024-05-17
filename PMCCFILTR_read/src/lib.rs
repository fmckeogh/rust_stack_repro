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
pub fn PMCCFILTR_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_2387: (),
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        gs_2387: (),
    }
    let fn_state = FunctionState {
        gs_2387,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_0_0: const #16952u : u32
        let s_0_0: u32 = 16952;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // N s_0_2: return s_0_1
        return s_0_1;
    }
}
