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
pub fn ERXMISC3_write<T: Tracer>(state: &mut State, tracer: &T, value_name: u32) -> () {
    #[derive(Default)]
    struct FunctionState {
        value_name: u32,
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
        // D s_0_0: read-var value_name:u32
        let s_0_0: u32 = fn_state.value_name;
        // C s_0_1: const #0s : i
        let s_0_1: i128 = 0;
        // C s_0_2: const #32s : i
        let s_0_2: i128 = 32;
        // D s_0_3: cast zx s_0_0 -> bv
        let s_0_3: Bits = Bits::new(s_0_0 as u128, 32u16);
        // D s_0_4: bit-extract s_0_3 s_0_1 s_0_2
        let s_0_4: Bits = (Bits::new(
            ((s_0_3) >> (s_0_1)).value(),
            u16::try_from(s_0_2).unwrap(),
        ));
        // D s_0_5: cast reint s_0_4 -> u32
        let s_0_5: u32 = (s_0_4.value() as u32);
        // C s_0_6: const #32s : i
        let s_0_6: i128 = 32;
        // C s_0_7: const #1400u : u32
        let s_0_7: u32 = 1400;
        // D s_0_8: read-reg s_0_7:u64
        let s_0_8: u64 = {
            let value = state.read_register::<u64>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 64u16);
        // D s_0_10: cast zx s_0_5 -> bv
        let s_0_10: Bits = Bits::new(s_0_5 as u128, 32u16);
        // C s_0_11: const #31s : i
        let s_0_11: i128 = 31;
        // C s_0_12: const #1u : u64
        let s_0_12: u64 = 1;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 64u16);
        // C s_0_14: lsl s_0_13 s_0_11
        let s_0_14: Bits = s_0_13 << s_0_11;
        // C s_0_15: sub s_0_14 s_0_13
        let s_0_15: Bits = ((s_0_14) - (s_0_13));
        // D s_0_16: and s_0_10 s_0_15
        let s_0_16: Bits = ((s_0_10) & (s_0_15));
        // D s_0_17: lsl s_0_16 s_0_6
        let s_0_17: Bits = s_0_16 << s_0_6;
        // C s_0_18: lsl s_0_15 s_0_6
        let s_0_18: Bits = s_0_15 << s_0_6;
        // C s_0_19: cmpl s_0_18
        let s_0_19: Bits = !s_0_18;
        // D s_0_20: and s_0_9 s_0_19
        let s_0_20: Bits = ((s_0_9) & (s_0_19));
        // D s_0_21: or s_0_20 s_0_17
        let s_0_21: Bits = ((s_0_20) | (s_0_17));
        // D s_0_22: cast reint s_0_21 -> u64
        let s_0_22: u64 = (s_0_21.value() as u64);
        // C s_0_23: const #1400u : u32
        let s_0_23: u32 = 1400;
        // N s_0_24: write-reg s_0_23 <= s_0_22
        let s_0_24: () = {
            state.write_register::<u64>(s_0_23 as isize, s_0_22);
            tracer.write_register(s_0_23 as isize, s_0_22);
        };
        // C s_0_25: const #17080u : u32
        let s_0_25: u32 = 17080;
        // N s_0_26: write-reg s_0_25 <= s_0_0
        let s_0_26: () = {
            state.write_register::<u32>(s_0_25 as isize, s_0_0);
            tracer.write_register(s_0_25 as isize, s_0_0);
        };
        // N s_0_27: return
        return;
    }
}
