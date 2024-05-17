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
use HIFAR_read::*;
use common::*;
pub fn IFAR_S_read<T: Tracer>(state: &mut State, tracer: &T, gs_36697: ()) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        r: u32,
        gs_36697: (),
    }
    let fn_state = FunctionState {
        gs_36697,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #14720u : u32
        let s_0_0: u32 = 14720;
        // D s_0_1: read-reg s_0_0:u32
        let s_0_1: u32 = {
            let value = state.read_register::<u32>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var r <= s_0_1
        fn_state.r = s_0_1;
        // C s_0_3: const #432u : u32
        let s_0_3: u32 = 432;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // C s_0_5: const #2u : u8
        let s_0_5: u8 = 2;
        // D s_0_6: cmp-lt s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) < (s_0_5));
        // N s_0_7: branch s_0_6 b3 b1
        if s_0_6 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HIFAR_read(s_1_0)
        let s_1_1: u32 = HIFAR_read(state, tracer, s_1_0);
        // C s_1_2: const #0s : i
        let s_1_2: i128 = 0;
        // C s_1_3: const #32s : i
        let s_1_3: i128 = 32;
        // S s_1_4: cast zx s_1_1 -> bv
        let s_1_4: Bits = Bits::new(s_1_1 as u128, 32u16);
        // D s_1_5: bit-extract s_1_4 s_1_2 s_1_3
        let s_1_5: Bits = (Bits::new(
            ((s_1_4) >> (s_1_2)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_6: cast reint s_1_5 -> u32
        let s_1_6: u32 = (s_1_5.value() as u32);
        // C s_1_7: const #0s : i
        let s_1_7: i128 = 0;
        // D s_1_8: read-var r:u32
        let s_1_8: u32 = fn_state.r;
        // D s_1_9: cast zx s_1_8 -> bv
        let s_1_9: Bits = Bits::new(s_1_8 as u128, 32u16);
        // D s_1_10: cast zx s_1_6 -> bv
        let s_1_10: Bits = Bits::new(s_1_6 as u128, 32u16);
        // C s_1_11: const #31s : i
        let s_1_11: i128 = 31;
        // C s_1_12: const #1u : u64
        let s_1_12: u64 = 1;
        // C s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 64u16);
        // C s_1_14: lsl s_1_13 s_1_11
        let s_1_14: Bits = s_1_13 << s_1_11;
        // C s_1_15: sub s_1_14 s_1_13
        let s_1_15: Bits = ((s_1_14) - (s_1_13));
        // D s_1_16: and s_1_10 s_1_15
        let s_1_16: Bits = ((s_1_10) & (s_1_15));
        // D s_1_17: lsl s_1_16 s_1_7
        let s_1_17: Bits = s_1_16 << s_1_7;
        // C s_1_18: lsl s_1_15 s_1_7
        let s_1_18: Bits = s_1_15 << s_1_7;
        // C s_1_19: cmpl s_1_18
        let s_1_19: Bits = !s_1_18;
        // D s_1_20: and s_1_9 s_1_19
        let s_1_20: Bits = ((s_1_9) & (s_1_19));
        // D s_1_21: or s_1_20 s_1_17
        let s_1_21: Bits = ((s_1_20) | (s_1_17));
        // D s_1_22: cast reint s_1_21 -> u32
        let s_1_22: u32 = (s_1_21.value() as u32);
        // D s_1_23: write-var r <= s_1_22
        fn_state.r = s_1_22;
        // N s_1_24: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var r:u32
        let s_2_0: u32 = fn_state.r;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #32s : i
        let s_3_0: i128 = 32;
        // C s_3_1: const #32s : i
        let s_3_1: i128 = 32;
        // C s_3_2: const #102616u : u32
        let s_3_2: u32 = 102616;
        // D s_3_3: read-reg s_3_2:u64
        let s_3_3: u64 = {
            let value = state.read_register::<u64>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 64u16);
        // D s_3_5: bit-extract s_3_4 s_3_0 s_3_1
        let s_3_5: Bits = (Bits::new(
            ((s_3_4) >> (s_3_0)).value(),
            u16::try_from(s_3_1).unwrap(),
        ));
        // D s_3_6: cast reint s_3_5 -> u32
        let s_3_6: u32 = (s_3_5.value() as u32);
        // C s_3_7: const #0s : i
        let s_3_7: i128 = 0;
        // D s_3_8: read-var r:u32
        let s_3_8: u32 = fn_state.r;
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 32u16);
        // D s_3_10: cast zx s_3_6 -> bv
        let s_3_10: Bits = Bits::new(s_3_6 as u128, 32u16);
        // C s_3_11: const #31s : i
        let s_3_11: i128 = 31;
        // C s_3_12: const #1u : u64
        let s_3_12: u64 = 1;
        // C s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 64u16);
        // C s_3_14: lsl s_3_13 s_3_11
        let s_3_14: Bits = s_3_13 << s_3_11;
        // C s_3_15: sub s_3_14 s_3_13
        let s_3_15: Bits = ((s_3_14) - (s_3_13));
        // D s_3_16: and s_3_10 s_3_15
        let s_3_16: Bits = ((s_3_10) & (s_3_15));
        // D s_3_17: lsl s_3_16 s_3_7
        let s_3_17: Bits = s_3_16 << s_3_7;
        // C s_3_18: lsl s_3_15 s_3_7
        let s_3_18: Bits = s_3_15 << s_3_7;
        // C s_3_19: cmpl s_3_18
        let s_3_19: Bits = !s_3_18;
        // D s_3_20: and s_3_9 s_3_19
        let s_3_20: Bits = ((s_3_9) & (s_3_19));
        // D s_3_21: or s_3_20 s_3_17
        let s_3_21: Bits = ((s_3_20) | (s_3_17));
        // D s_3_22: cast reint s_3_21 -> u32
        let s_3_22: u32 = (s_3_21.value() as u32);
        // D s_3_23: write-var r <= s_3_22
        fn_state.r = s_3_22;
        // N s_3_24: jump b2
        return block_2(state, tracer, fn_state);
    }
}
