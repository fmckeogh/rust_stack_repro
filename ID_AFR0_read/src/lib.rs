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
pub fn ID_AFR0_read<T: Tracer>(state: &mut State, tracer: &T, gs_35226: ()) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_35226: (),
    }
    let fn_state = FunctionState {
        gs_35226,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #102808u : u32
        let s_0_0: u32 = 102808;
        // D s_0_1: read-reg s_0_0:u32
        let s_0_1: u32 = {
            let value = state.read_register::<u32>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #0s : i
        let s_0_2: i128 = 0;
        // C s_0_3: const #32s : i
        let s_0_3: i128 = 32;
        // C s_0_4: const #21080u : u32
        let s_0_4: u32 = 21080;
        // D s_0_5: read-reg s_0_4:u64
        let s_0_5: u64 = {
            let value = state.read_register::<u64>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 64u16);
        // D s_0_7: bit-extract s_0_6 s_0_2 s_0_3
        let s_0_7: Bits = (Bits::new(
            ((s_0_6) >> (s_0_2)).value(),
            u16::try_from(s_0_3).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u32
        let s_0_8: u32 = (s_0_7.value() as u32);
        // C s_0_9: const #0s : i
        let s_0_9: i128 = 0;
        // D s_0_10: cast zx s_0_1 -> bv
        let s_0_10: Bits = Bits::new(s_0_1 as u128, 32u16);
        // D s_0_11: cast zx s_0_8 -> bv
        let s_0_11: Bits = Bits::new(s_0_8 as u128, 32u16);
        // C s_0_12: const #31s : i
        let s_0_12: i128 = 31;
        // C s_0_13: const #1u : u64
        let s_0_13: u64 = 1;
        // C s_0_14: cast zx s_0_13 -> bv
        let s_0_14: Bits = Bits::new(s_0_13 as u128, 64u16);
        // C s_0_15: lsl s_0_14 s_0_12
        let s_0_15: Bits = s_0_14 << s_0_12;
        // C s_0_16: sub s_0_15 s_0_14
        let s_0_16: Bits = ((s_0_15) - (s_0_14));
        // D s_0_17: and s_0_11 s_0_16
        let s_0_17: Bits = ((s_0_11) & (s_0_16));
        // D s_0_18: lsl s_0_17 s_0_9
        let s_0_18: Bits = s_0_17 << s_0_9;
        // C s_0_19: lsl s_0_16 s_0_9
        let s_0_19: Bits = s_0_16 << s_0_9;
        // C s_0_20: cmpl s_0_19
        let s_0_20: Bits = !s_0_19;
        // D s_0_21: and s_0_10 s_0_20
        let s_0_21: Bits = ((s_0_10) & (s_0_20));
        // D s_0_22: or s_0_21 s_0_18
        let s_0_22: Bits = ((s_0_21) | (s_0_18));
        // D s_0_23: cast reint s_0_22 -> u32
        let s_0_23: u32 = (s_0_22.value() as u32);
        // N s_0_24: return s_0_23
        return s_0_23;
    }
}
