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
pub fn DBGDTRRXext_write<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
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
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // D s_0_2: read-var r.0:struct
        let s_0_2: u32 = fn_state.r._0;
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // C s_0_4: const #32s : i
        let s_0_4: i128 = 32;
        // D s_0_5: cast zx s_0_2 -> bv
        let s_0_5: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_6: bit-extract s_0_5 s_0_3 s_0_4
        let s_0_6: Bits = (Bits::new(
            ((s_0_5) >> (s_0_3)).value(),
            u16::try_from(s_0_4).unwrap(),
        ));
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // C s_0_8: const #0s : i
        let s_0_8: i128 = 0;
        // C s_0_9: const #15872u : u32
        let s_0_9: u32 = 15872;
        // D s_0_10: read-reg s_0_9:u64
        let s_0_10: u64 = {
            let value = state.read_register::<u64>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 64u16);
        // D s_0_12: cast zx s_0_7 -> bv
        let s_0_12: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_13: const #31s : i
        let s_0_13: i128 = 31;
        // C s_0_14: const #1u : u64
        let s_0_14: u64 = 1;
        // C s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 64u16);
        // C s_0_16: lsl s_0_15 s_0_13
        let s_0_16: Bits = s_0_15 << s_0_13;
        // C s_0_17: sub s_0_16 s_0_15
        let s_0_17: Bits = ((s_0_16) - (s_0_15));
        // D s_0_18: and s_0_12 s_0_17
        let s_0_18: Bits = ((s_0_12) & (s_0_17));
        // D s_0_19: lsl s_0_18 s_0_8
        let s_0_19: Bits = s_0_18 << s_0_8;
        // C s_0_20: lsl s_0_17 s_0_8
        let s_0_20: Bits = s_0_17 << s_0_8;
        // C s_0_21: cmpl s_0_20
        let s_0_21: Bits = !s_0_20;
        // D s_0_22: and s_0_11 s_0_21
        let s_0_22: Bits = ((s_0_11) & (s_0_21));
        // D s_0_23: or s_0_22 s_0_19
        let s_0_23: Bits = ((s_0_22) | (s_0_19));
        // D s_0_24: cast reint s_0_23 -> u64
        let s_0_24: u64 = (s_0_23.value() as u64);
        // C s_0_25: const #15872u : u32
        let s_0_25: u32 = 15872;
        // N s_0_26: write-reg s_0_25 <= s_0_24
        let s_0_26: () = {
            state.write_register::<u64>(s_0_25 as isize, s_0_24);
            tracer.write_register(s_0_25 as isize, s_0_24);
        };
        // C s_0_27: const #10152u : u32
        let s_0_27: u32 = 10152;
        // N s_0_28: write-reg s_0_27 <= s_0_0
        let s_0_28: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_27 as isize, s_0_0);
            tracer.write_register(s_0_27 as isize, s_0_0);
        };
        // N s_0_29: return
        return;
    }
}
