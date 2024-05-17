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
use set_subrange_zeros::*;
use common::*;
pub fn SPESampleGeneralPurposeLoadStore<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_26270: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_26270: (),
    }
    let fn_state = FunctionState {
        gs_26270,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: u8 = 1;
        // C s_0_1: const #17136u : u32
        let s_0_1: u32 = 17136;
        // N s_0_2: write-reg s_0_1 <= s_0_0
        let s_0_2: () = {
            state.write_register::<u8>(s_0_1 as isize, s_0_0);
            tracer.write_register(s_0_1 as isize, s_0_0);
        };
        // C s_0_3: const #8s : i
        let s_0_3: i128 = 8;
        // C s_0_4: const #7s : i
        let s_0_4: i128 = 7;
        // C s_0_5: const #1s : i
        let s_0_5: i128 = 1;
        // C s_0_6: const #13528u : u32
        let s_0_6: u32 = 13528;
        // D s_0_7: read-reg s_0_6:u8
        let s_0_7: u8 = {
            let value = state.read_register::<u8>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 8u16);
        // D s_0_9: call set_subrange_zeros(s_0_3, s_0_8, s_0_4, s_0_5)
        let s_0_9: Bits = set_subrange_zeros(state, tracer, s_0_3, s_0_8, s_0_4, s_0_5);
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: u8 = (s_0_9.value() as u8);
        // C s_0_11: const #13528u : u32
        let s_0_11: u32 = 13528;
        // N s_0_12: write-reg s_0_11 <= s_0_10
        let s_0_12: () = {
            state.write_register::<u8>(s_0_11 as isize, s_0_10);
            tracer.write_register(s_0_11 as isize, s_0_10);
        };
        // C s_0_13: const #1u : u8
        let s_0_13: bool = true;
        // C s_0_14: const #11528u : u32
        let s_0_14: u32 = 11528;
        // N s_0_15: write-reg s_0_14 <= s_0_13
        let s_0_15: () = {
            state.write_register::<bool>(s_0_14 as isize, s_0_13);
            tracer.write_register(s_0_14 as isize, s_0_13);
        };
        // N s_0_16: return
        return;
    }
}
