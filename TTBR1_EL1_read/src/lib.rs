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
use TTBR1_read::*;
use common::*;
pub fn TTBR1_EL1_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_8250: (),
) -> ProductType782ac6922b48c20d {
    #[derive(Default)]
    struct FunctionState {
        gs_8250: (),
    }
    let fn_state = FunctionState {
        gs_8250,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType782ac6922b48c20d {
        // C s_0_0: const #14168u : u32
        let s_0_0: u32 = 14168;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType782ac6922b48c20d = {
            let value = state
                .read_register::<ProductType782ac6922b48c20d>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call TTBR1_read(s_0_2)
        let s_0_3: ProductType5c790c8ef59cc8b2 = TTBR1_read(state, tracer, s_0_2);
        // N s_0_4: return s_0_1
        return s_0_1;
    }
}
