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
pub fn u__set_CNTV_CTL_EL0<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #17200u : u32
        let s_0_0: u32 = 17200;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #17200u : u32
        let s_0_2: u32 = 17200;
        // N s_0_3: write-reg s_0_2 <= s_0_1
        let s_0_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize, s_0_1);
            tracer.write_register(s_0_2 as isize, s_0_1);
        };
        // C s_0_4: const #17200u : u32
        let s_0_4: u32 = 17200;
        // D s_0_5: read-reg s_0_4:struct
        let s_0_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // C s_0_6: const #17200u : u32
        let s_0_6: u32 = 17200;
        // N s_0_7: write-reg s_0_6 <= s_0_5
        let s_0_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize, s_0_5);
            tracer.write_register(s_0_6 as isize, s_0_5);
        };
        // N s_0_8: return
        return;
    }
}
