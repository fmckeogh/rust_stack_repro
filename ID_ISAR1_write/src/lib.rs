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
pub fn ID_ISAR1_write<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        value_name: ProductType700c18a878c5601b,
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
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // C s_0_1: const #14384u : u32
        let s_0_1: u32 = 14384;
        // D s_0_2: read-reg s_0_1:struct
        let s_0_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_1 as isize);
            tracer.read_register(s_0_1 as isize, value);
            value
        };
        // C s_0_3: const #14384u : u32
        let s_0_3: u32 = 14384;
        // N s_0_4: write-reg s_0_3 <= s_0_2
        let s_0_4: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize, s_0_2);
            tracer.write_register(s_0_3 as isize, s_0_2);
        };
        // C s_0_5: const #17008u : u32
        let s_0_5: u32 = 17008;
        // N s_0_6: write-reg s_0_5 <= s_0_0
        let s_0_6: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_5 as isize, s_0_0);
            tracer.write_register(s_0_5 as isize, s_0_0);
        };
        // N s_0_7: return
        return;
    }
}
