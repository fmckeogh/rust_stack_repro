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
pub fn execute_aarch64_instrs_integer_flags_axflag<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_143923: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_143923: (),
    }
    let fn_state = FunctionState {
        gs_143923,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // C s_0_1: const #16997u : u32
        let s_0_1: u32 = 16997;
        // D s_0_2: read-reg s_0_1:u8
        let s_0_2: bool = {
            let value = state.read_register::<bool>(s_0_1 as isize);
            tracer.read_register(s_0_1 as isize, value);
            value
        };
        // C s_0_3: const #16996u : u32
        let s_0_3: u32 = 16996;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: bool = {
            let value = state.read_register::<bool>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_2 -> bv
        let s_0_5: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_6: cast zx s_0_4 -> bv
        let s_0_6: Bits = Bits::new(s_0_4 as u128, 1u16);
        // D s_0_7: or s_0_5 s_0_6
        let s_0_7: Bits = ((s_0_5) | (s_0_6));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: bool = ((s_0_7.value()) != 0);
        // C s_0_9: const #16971u : u32
        let s_0_9: u32 = 16971;
        // D s_0_10: read-reg s_0_9:u8
        let s_0_10: bool = {
            let value = state.read_register::<bool>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // C s_0_11: const #16996u : u32
        let s_0_11: u32 = 16996;
        // D s_0_12: read-reg s_0_11:u8
        let s_0_12: bool = {
            let value = state.read_register::<bool>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: not s_0_13
        let s_0_14: Bits = !s_0_13;
        // D s_0_15: cast reint s_0_14 -> u8
        let s_0_15: bool = ((s_0_14.value()) != 0);
        // D s_0_16: cast zx s_0_10 -> bv
        let s_0_16: Bits = Bits::new(s_0_10 as u128, 1u16);
        // D s_0_17: cast zx s_0_15 -> bv
        let s_0_17: Bits = Bits::new(s_0_15 as u128, 1u16);
        // D s_0_18: and s_0_16 s_0_17
        let s_0_18: Bits = ((s_0_16) & (s_0_17));
        // D s_0_19: cast reint s_0_18 -> u8
        let s_0_19: bool = ((s_0_18.value()) != 0);
        // C s_0_20: const #0u : u8
        let s_0_20: bool = false;
        // C s_0_21: const #16984u : u32
        let s_0_21: u32 = 16984;
        // N s_0_22: write-reg s_0_21 <= s_0_0
        let s_0_22: () = {
            state.write_register::<bool>(s_0_21 as isize, s_0_0);
            tracer.write_register(s_0_21 as isize, s_0_0);
        };
        // C s_0_23: const #16997u : u32
        let s_0_23: u32 = 16997;
        // N s_0_24: write-reg s_0_23 <= s_0_8
        let s_0_24: () = {
            state.write_register::<bool>(s_0_23 as isize, s_0_8);
            tracer.write_register(s_0_23 as isize, s_0_8);
        };
        // C s_0_25: const #16971u : u32
        let s_0_25: u32 = 16971;
        // N s_0_26: write-reg s_0_25 <= s_0_19
        let s_0_26: () = {
            state.write_register::<bool>(s_0_25 as isize, s_0_19);
            tracer.write_register(s_0_25 as isize, s_0_19);
        };
        // C s_0_27: const #16996u : u32
        let s_0_27: u32 = 16996;
        // N s_0_28: write-reg s_0_27 <= s_0_20
        let s_0_28: () = {
            state.write_register::<bool>(s_0_27 as isize, s_0_20);
            tracer.write_register(s_0_27 as isize, s_0_20);
        };
        // N s_0_29: return
        return;
    }
}
