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
pub fn ICV_AP0R_read<T: Tracer>(state: &mut State, tracer: &T, n: i64) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #20808u : u32
        let s_0_0: u32 = 20808;
        // D s_0_1: read-reg s_0_0:[u32; 4]
        let s_0_1: [u32; 4usize] = {
            let value = state.read_register::<[u32; 4usize]>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: read-var n:i64
        let s_0_2: i64 = fn_state.n;
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: read-element s_0_1[s_0_3]
        let s_0_4: u32 = s_0_1[(s_0_3) as usize];
        // C s_0_5: const #21152u : u32
        let s_0_5: u32 = 21152;
        // D s_0_6: read-reg s_0_5:[u64; 4]
        let s_0_6: [u64; 4usize] = {
            let value = state.read_register::<[u64; 4usize]>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: read-var n:i64
        let s_0_7: i64 = fn_state.n;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: read-element s_0_6[s_0_8]
        let s_0_9: u64 = s_0_6[(s_0_8) as usize];
        // C s_0_10: const #0s : i
        let s_0_10: i128 = 0;
        // C s_0_11: const #32s : i
        let s_0_11: i128 = 32;
        // D s_0_12: cast zx s_0_9 -> bv
        let s_0_12: Bits = Bits::new(s_0_9 as u128, 64u16);
        // D s_0_13: bit-extract s_0_12 s_0_10 s_0_11
        let s_0_13: Bits = (Bits::new(
            ((s_0_12) >> (s_0_10)).value(),
            u16::try_from(s_0_11).unwrap(),
        ));
        // D s_0_14: cast reint s_0_13 -> u32
        let s_0_14: u32 = (s_0_13.value() as u32);
        // C s_0_15: const #0s : i
        let s_0_15: i128 = 0;
        // D s_0_16: cast zx s_0_4 -> bv
        let s_0_16: Bits = Bits::new(s_0_4 as u128, 32u16);
        // D s_0_17: cast zx s_0_14 -> bv
        let s_0_17: Bits = Bits::new(s_0_14 as u128, 32u16);
        // C s_0_18: const #31s : i
        let s_0_18: i128 = 31;
        // C s_0_19: const #1u : u64
        let s_0_19: u64 = 1;
        // C s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 64u16);
        // C s_0_21: lsl s_0_20 s_0_18
        let s_0_21: Bits = s_0_20 << s_0_18;
        // C s_0_22: sub s_0_21 s_0_20
        let s_0_22: Bits = ((s_0_21) - (s_0_20));
        // D s_0_23: and s_0_17 s_0_22
        let s_0_23: Bits = ((s_0_17) & (s_0_22));
        // D s_0_24: lsl s_0_23 s_0_15
        let s_0_24: Bits = s_0_23 << s_0_15;
        // C s_0_25: lsl s_0_22 s_0_15
        let s_0_25: Bits = s_0_22 << s_0_15;
        // C s_0_26: cmpl s_0_25
        let s_0_26: Bits = !s_0_25;
        // D s_0_27: and s_0_16 s_0_26
        let s_0_27: Bits = ((s_0_16) & (s_0_26));
        // D s_0_28: or s_0_27 s_0_24
        let s_0_28: Bits = ((s_0_27) | (s_0_24));
        // D s_0_29: cast reint s_0_28 -> u32
        let s_0_29: u32 = (s_0_28.value() as u32);
        // N s_0_30: return s_0_29
        return s_0_29;
    }
}
