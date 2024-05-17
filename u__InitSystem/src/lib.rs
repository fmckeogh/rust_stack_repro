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
use TakeReset::*;
use common::*;
pub fn u__InitSystem<T: Tracer>(state: &mut State, tracer: &T, gs_331011: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_331011: (),
    }
    let fn_state = FunctionState {
        gs_331011,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // C s_0_1: const #20440u : u32
        let s_0_1: u32 = 20440;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<u32>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // C s_0_3: const #1u : u8
        let s_0_3: bool = true;
        // S s_0_4: call TakeReset(s_0_3)
        let s_0_4: () = TakeReset(state, tracer, s_0_3);
        // C s_0_5: const #18288u : u32
        let s_0_5: u32 = 18288;
        // D s_0_6: read-reg s_0_5:struct
        let s_0_6: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // C s_0_7: const #18288u : u32
        let s_0_7: u32 = 18288;
        // N s_0_8: write-reg s_0_7 <= s_0_6
        let s_0_8: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_7 as isize, s_0_6);
            tracer.write_register(s_0_7 as isize, s_0_6);
        };
        // N s_0_9: return
        return;
    }
}
