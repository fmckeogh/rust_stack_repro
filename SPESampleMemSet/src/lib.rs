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
pub fn SPESampleMemSet<T: Tracer>(state: &mut State, tracer: &T, gs_26292: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_26292: (),
    }
    let fn_state = FunctionState {
        gs_26292,
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
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // C s_0_4: const #13528u : u32
        let s_0_4: u32 = 13528;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 8u16);
        // C s_0_7: const #37u : u8
        let s_0_7: u8 = 37;
        // C s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 8u16);
        // C s_0_9: const #7s : i
        let s_0_9: i128 = 7;
        // C s_0_10: const #1u : u64
        let s_0_10: u64 = 1;
        // C s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 64u16);
        // C s_0_12: lsl s_0_11 s_0_9
        let s_0_12: Bits = s_0_11 << s_0_9;
        // C s_0_13: sub s_0_12 s_0_11
        let s_0_13: Bits = ((s_0_12) - (s_0_11));
        // C s_0_14: and s_0_8 s_0_13
        let s_0_14: Bits = ((s_0_8) & (s_0_13));
        // C s_0_15: lsl s_0_14 s_0_3
        let s_0_15: Bits = s_0_14 << s_0_3;
        // C s_0_16: lsl s_0_13 s_0_3
        let s_0_16: Bits = s_0_13 << s_0_3;
        // C s_0_17: cmpl s_0_16
        let s_0_17: Bits = !s_0_16;
        // D s_0_18: and s_0_6 s_0_17
        let s_0_18: Bits = ((s_0_6) & (s_0_17));
        // D s_0_19: or s_0_18 s_0_15
        let s_0_19: Bits = ((s_0_18) | (s_0_15));
        // D s_0_20: cast reint s_0_19 -> u8
        let s_0_20: u8 = (s_0_19.value() as u8);
        // C s_0_21: const #13528u : u32
        let s_0_21: u32 = 13528;
        // N s_0_22: write-reg s_0_21 <= s_0_20
        let s_0_22: () = {
            state.write_register::<u8>(s_0_21 as isize, s_0_20);
            tracer.write_register(s_0_21 as isize, s_0_20);
        };
        // C s_0_23: const #1u : u8
        let s_0_23: bool = true;
        // C s_0_24: const #11528u : u32
        let s_0_24: u32 = 11528;
        // N s_0_25: write-reg s_0_24 <= s_0_23
        let s_0_25: () = {
            state.write_register::<bool>(s_0_24 as isize, s_0_23);
            tracer.write_register(s_0_24 as isize, s_0_23);
        };
        // C s_0_26: const #1u : u32
        let s_0_26: u32 = 1;
        // C s_0_27: const #19040u : u32
        let s_0_27: u32 = 19040;
        // N s_0_28: write-reg s_0_27 <= s_0_26
        let s_0_28: () = {
            state.write_register::<u32>(s_0_27 as isize, s_0_26);
            tracer.write_register(s_0_27 as isize, s_0_26);
        };
        // N s_0_29: return
        return;
    }
}
