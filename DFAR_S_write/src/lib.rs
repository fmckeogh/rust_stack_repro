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
use HDFAR_read::*;
use HDFAR_write::*;
use common::*;
pub fn DFAR_S_write<T: Tracer>(state: &mut State, tracer: &T, value_name: u32) -> () {
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
        // C s_0_1: const #() : ()
        let s_0_1: () = ();
        // S s_0_2: call HDFAR_read(s_0_1)
        let s_0_2: u32 = HDFAR_read(state, tracer, s_0_1);
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // C s_0_4: const #32s : i
        let s_0_4: i128 = 32;
        // D s_0_5: cast zx s_0_0 -> bv
        let s_0_5: Bits = Bits::new(s_0_0 as u128, 32u16);
        // D s_0_6: bit-extract s_0_5 s_0_3 s_0_4
        let s_0_6: Bits = (Bits::new(
            ((s_0_5) >> (s_0_3)).value(),
            u16::try_from(s_0_4).unwrap(),
        ));
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // C s_0_8: const #32s : i
        let s_0_8: i128 = 32;
        // C s_0_9: const #0s : i
        let s_0_9: i128 = 0;
        // S s_0_10: cast zx s_0_2 -> bv
        let s_0_10: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_11: cast zx s_0_7 -> bv
        let s_0_11: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_12: const #1u : u64
        let s_0_12: u64 = 1;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 64u16);
        // C s_0_14: lsl s_0_13 s_0_8
        let s_0_14: Bits = s_0_13 << s_0_8;
        // C s_0_15: sub s_0_14 s_0_13
        let s_0_15: Bits = ((s_0_14) - (s_0_13));
        // D s_0_16: and s_0_11 s_0_15
        let s_0_16: Bits = ((s_0_11) & (s_0_15));
        // D s_0_17: lsl s_0_16 s_0_9
        let s_0_17: Bits = s_0_16 << s_0_9;
        // C s_0_18: lsl s_0_15 s_0_9
        let s_0_18: Bits = s_0_15 << s_0_9;
        // C s_0_19: cmpl s_0_18
        let s_0_19: Bits = !s_0_18;
        // S s_0_20: and s_0_10 s_0_19
        let s_0_20: Bits = ((s_0_10) & (s_0_19));
        // D s_0_21: or s_0_20 s_0_17
        let s_0_21: Bits = ((s_0_20) | (s_0_17));
        // D s_0_22: cast reint s_0_21 -> u32
        let s_0_22: u32 = (s_0_21.value() as u32);
        // D s_0_23: call HDFAR_write(s_0_22)
        let s_0_23: () = HDFAR_write(state, tracer, s_0_22);
        // C s_0_24: const #21752u : u32
        let s_0_24: u32 = 21752;
        // N s_0_25: write-reg s_0_24 <= s_0_0
        let s_0_25: () = {
            state.write_register::<u32>(s_0_24 as isize, s_0_0);
            tracer.write_register(s_0_24 as isize, s_0_0);
        };
        // N s_0_26: return
        return;
    }
}
