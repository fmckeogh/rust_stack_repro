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
pub fn execute_aarch64_instrs_integer_flags_cfinv<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_145921: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_145921: (),
    }
    let fn_state = FunctionState {
        gs_145921,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16971u : u32
        let s_0_0: u32 = 16971;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 1u16);
        // D s_0_3: not s_0_2
        let s_0_3: Bits = !s_0_2;
        // D s_0_4: cast reint s_0_3 -> u8
        let s_0_4: bool = ((s_0_3.value()) != 0);
        // C s_0_5: const #16971u : u32
        let s_0_5: u32 = 16971;
        // N s_0_6: write-reg s_0_5 <= s_0_4
        let s_0_6: () = {
            state.write_register::<bool>(s_0_5 as isize, s_0_4);
            tracer.write_register(s_0_5 as isize, s_0_4);
        };
        // N s_0_7: return
        return;
    }
}
