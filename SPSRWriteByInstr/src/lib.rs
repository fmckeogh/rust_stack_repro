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
use SPSR_set::*;
use SPSR_read::*;
use common::*;
pub fn SPSRWriteByInstr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: u32,
    bytemask: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        new_spsr: u32,
        value_name: u32,
        bytemask: u8,
    }
    let fn_state = FunctionState {
        value_name,
        bytemask,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #32s : i
        let s_0_0: i128 = 32;
        // S s_0_1: call SPSR_read(s_0_0)
        let s_0_1: Bits = SPSR_read(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u32
        let s_0_2: u32 = (s_0_1.value() as u32);
        // D s_0_3: write-var new_spsr <= s_0_2
        fn_state.new_spsr = s_0_2;
        // C s_0_4: const #3s : i
        let s_0_4: i128 = 3;
        // D s_0_5: read-var bytemask:u8
        let s_0_5: u8 = fn_state.bytemask;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 4u16);
        // C s_0_7: const #1u : u64
        let s_0_7: u64 = 1;
        // D s_0_8: bit-extract s_0_6 s_0_4 s_0_7
        let s_0_8: Bits = (Bits::new(
            ((s_0_6) >> (s_0_4)).value(),
            u16::try_from(s_0_7).unwrap(),
        ));
        // D s_0_9: cast reint s_0_8 -> u8
        let s_0_9: bool = ((s_0_8.value()) != 0);
        // C s_0_10: const #0s : i
        let s_0_10: i128 = 0;
        // C s_0_11: const #0u : u64
        let s_0_11: u64 = 0;
        // D s_0_12: cast zx s_0_9 -> u64
        let s_0_12: u64 = (s_0_9 as u64);
        // C s_0_13: const #1u : u64
        let s_0_13: u64 = 1;
        // D s_0_14: and s_0_12 s_0_13
        let s_0_14: u64 = ((s_0_12) & (s_0_13));
        // D s_0_15: cmp-eq s_0_14 s_0_13
        let s_0_15: bool = ((s_0_14) == (s_0_13));
        // D s_0_16: lsl s_0_12 s_0_10
        let s_0_16: u64 = s_0_12 << s_0_10;
        // D s_0_17: or s_0_11 s_0_16
        let s_0_17: u64 = ((s_0_11) | (s_0_16));
        // D s_0_18: cmpl s_0_16
        let s_0_18: u64 = !s_0_16;
        // D s_0_19: and s_0_11 s_0_18
        let s_0_19: u64 = ((s_0_11) & (s_0_18));
        // D s_0_20: select s_0_15 s_0_17 s_0_19
        let s_0_20: u64 = if s_0_15 { s_0_17 } else { s_0_19 };
        // D s_0_21: cast trunc s_0_20 -> u8
        let s_0_21: bool = ((s_0_20) != 0);
        // D s_0_22: cast zx s_0_21 -> bv
        let s_0_22: Bits = Bits::new(s_0_21 as u128, 1u16);
        // C s_0_23: const #1u : u8
        let s_0_23: bool = true;
        // C s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 1u16);
        // D s_0_25: cmp-eq s_0_22 s_0_24
        let s_0_25: bool = ((s_0_22) == (s_0_24));
        // N s_0_26: branch s_0_25 b12 b1
        if s_0_25 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #2s : i
        let s_2_0: i128 = 2;
        // D s_2_1: read-var bytemask:u8
        let s_2_1: u8 = fn_state.bytemask;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 4u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // C s_2_19: const #1u : u8
        let s_2_19: bool = true;
        // C s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cmp-eq s_2_18 s_2_20
        let s_2_21: bool = ((s_2_18) == (s_2_20));
        // N s_2_22: branch s_2_21 b11 b3
        if s_2_21 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1s : i
        let s_4_0: i128 = 1;
        // D s_4_1: read-var bytemask:u8
        let s_4_1: u8 = fn_state.bytemask;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // C s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_21: cmp-eq s_4_18 s_4_20
        let s_4_21: bool = ((s_4_18) == (s_4_20));
        // N s_4_22: branch s_4_21 b10 b5
        if s_4_21 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // D s_6_1: read-var bytemask:u8
        let s_6_1: u8 = fn_state.bytemask;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 4u16);
        // C s_6_3: const #1u : u64
        let s_6_3: u64 = 1;
        // D s_6_4: bit-extract s_6_2 s_6_0 s_6_3
        let s_6_4: Bits = (Bits::new(
            ((s_6_2) >> (s_6_0)).value(),
            u16::try_from(s_6_3).unwrap(),
        ));
        // D s_6_5: cast reint s_6_4 -> u8
        let s_6_5: bool = ((s_6_4.value()) != 0);
        // C s_6_6: const #0s : i
        let s_6_6: i128 = 0;
        // C s_6_7: const #0u : u64
        let s_6_7: u64 = 0;
        // D s_6_8: cast zx s_6_5 -> u64
        let s_6_8: u64 = (s_6_5 as u64);
        // C s_6_9: const #1u : u64
        let s_6_9: u64 = 1;
        // D s_6_10: and s_6_8 s_6_9
        let s_6_10: u64 = ((s_6_8) & (s_6_9));
        // D s_6_11: cmp-eq s_6_10 s_6_9
        let s_6_11: bool = ((s_6_10) == (s_6_9));
        // D s_6_12: lsl s_6_8 s_6_6
        let s_6_12: u64 = s_6_8 << s_6_6;
        // D s_6_13: or s_6_7 s_6_12
        let s_6_13: u64 = ((s_6_7) | (s_6_12));
        // D s_6_14: cmpl s_6_12
        let s_6_14: u64 = !s_6_12;
        // D s_6_15: and s_6_7 s_6_14
        let s_6_15: u64 = ((s_6_7) & (s_6_14));
        // D s_6_16: select s_6_11 s_6_13 s_6_15
        let s_6_16: u64 = if s_6_11 { s_6_13 } else { s_6_15 };
        // D s_6_17: cast trunc s_6_16 -> u8
        let s_6_17: bool = ((s_6_16) != 0);
        // D s_6_18: cast zx s_6_17 -> bv
        let s_6_18: Bits = Bits::new(s_6_17 as u128, 1u16);
        // C s_6_19: const #1u : u8
        let s_6_19: bool = true;
        // C s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 1u16);
        // D s_6_21: cmp-eq s_6_18 s_6_20
        let s_6_21: bool = ((s_6_18) == (s_6_20));
        // N s_6_22: branch s_6_21 b9 b7
        if s_6_21 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #32s : i
        let s_8_0: i128 = 32;
        // D s_8_1: read-var new_spsr:u32
        let s_8_1: u32 = fn_state.new_spsr;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 32u16);
        // D s_8_3: call SPSR_set(s_8_0, s_8_2)
        let s_8_3: () = SPSR_set(state, tracer, s_8_0, s_8_2);
        // N s_8_4: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: read-var value_name:u32
        let s_9_1: u32 = fn_state.value_name;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 32u16);
        // C s_9_3: const #1s : i64
        let s_9_3: i64 = 1;
        // C s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // C s_9_5: const #7s : i
        let s_9_5: i128 = 7;
        // C s_9_6: add s_9_5 s_9_4
        let s_9_6: i128 = (s_9_5 + s_9_4);
        // D s_9_7: bit-extract s_9_2 s_9_0 s_9_6
        let s_9_7: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_6).unwrap(),
        ));
        // D s_9_8: cast reint s_9_7 -> u8
        let s_9_8: u8 = (s_9_7.value() as u8);
        // C s_9_9: const #0s : i
        let s_9_9: i128 = 0;
        // D s_9_10: read-var new_spsr:u32
        let s_9_10: u32 = fn_state.new_spsr;
        // D s_9_11: cast zx s_9_10 -> bv
        let s_9_11: Bits = Bits::new(s_9_10 as u128, 32u16);
        // D s_9_12: cast zx s_9_8 -> bv
        let s_9_12: Bits = Bits::new(s_9_8 as u128, 8u16);
        // C s_9_13: const #7s : i
        let s_9_13: i128 = 7;
        // C s_9_14: const #1u : u64
        let s_9_14: u64 = 1;
        // C s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 64u16);
        // C s_9_16: lsl s_9_15 s_9_13
        let s_9_16: Bits = s_9_15 << s_9_13;
        // C s_9_17: sub s_9_16 s_9_15
        let s_9_17: Bits = ((s_9_16) - (s_9_15));
        // D s_9_18: and s_9_12 s_9_17
        let s_9_18: Bits = ((s_9_12) & (s_9_17));
        // D s_9_19: lsl s_9_18 s_9_9
        let s_9_19: Bits = s_9_18 << s_9_9;
        // C s_9_20: lsl s_9_17 s_9_9
        let s_9_20: Bits = s_9_17 << s_9_9;
        // C s_9_21: cmpl s_9_20
        let s_9_21: Bits = !s_9_20;
        // D s_9_22: and s_9_11 s_9_21
        let s_9_22: Bits = ((s_9_11) & (s_9_21));
        // D s_9_23: or s_9_22 s_9_19
        let s_9_23: Bits = ((s_9_22) | (s_9_19));
        // D s_9_24: cast reint s_9_23 -> u32
        let s_9_24: u32 = (s_9_23.value() as u32);
        // D s_9_25: write-var new_spsr <= s_9_24
        fn_state.new_spsr = s_9_24;
        // N s_9_26: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #8s : i
        let s_10_0: i128 = 8;
        // D s_10_1: read-var value_name:u32
        let s_10_1: u32 = fn_state.value_name;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 32u16);
        // C s_10_3: const #1s : i64
        let s_10_3: i64 = 1;
        // C s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // C s_10_5: const #7s : i
        let s_10_5: i128 = 7;
        // C s_10_6: add s_10_5 s_10_4
        let s_10_6: i128 = (s_10_5 + s_10_4);
        // D s_10_7: bit-extract s_10_2 s_10_0 s_10_6
        let s_10_7: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_6).unwrap(),
        ));
        // D s_10_8: cast reint s_10_7 -> u8
        let s_10_8: u8 = (s_10_7.value() as u8);
        // C s_10_9: const #8s : i
        let s_10_9: i128 = 8;
        // D s_10_10: read-var new_spsr:u32
        let s_10_10: u32 = fn_state.new_spsr;
        // D s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 32u16);
        // D s_10_12: cast zx s_10_8 -> bv
        let s_10_12: Bits = Bits::new(s_10_8 as u128, 8u16);
        // C s_10_13: const #7s : i
        let s_10_13: i128 = 7;
        // C s_10_14: const #1u : u64
        let s_10_14: u64 = 1;
        // C s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 64u16);
        // C s_10_16: lsl s_10_15 s_10_13
        let s_10_16: Bits = s_10_15 << s_10_13;
        // C s_10_17: sub s_10_16 s_10_15
        let s_10_17: Bits = ((s_10_16) - (s_10_15));
        // D s_10_18: and s_10_12 s_10_17
        let s_10_18: Bits = ((s_10_12) & (s_10_17));
        // D s_10_19: lsl s_10_18 s_10_9
        let s_10_19: Bits = s_10_18 << s_10_9;
        // C s_10_20: lsl s_10_17 s_10_9
        let s_10_20: Bits = s_10_17 << s_10_9;
        // C s_10_21: cmpl s_10_20
        let s_10_21: Bits = !s_10_20;
        // D s_10_22: and s_10_11 s_10_21
        let s_10_22: Bits = ((s_10_11) & (s_10_21));
        // D s_10_23: or s_10_22 s_10_19
        let s_10_23: Bits = ((s_10_22) | (s_10_19));
        // D s_10_24: cast reint s_10_23 -> u32
        let s_10_24: u32 = (s_10_23.value() as u32);
        // D s_10_25: write-var new_spsr <= s_10_24
        fn_state.new_spsr = s_10_24;
        // N s_10_26: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #16s : i
        let s_11_0: i128 = 16;
        // D s_11_1: read-var value_name:u32
        let s_11_1: u32 = fn_state.value_name;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 32u16);
        // C s_11_3: const #1s : i64
        let s_11_3: i64 = 1;
        // C s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // C s_11_5: const #7s : i
        let s_11_5: i128 = 7;
        // C s_11_6: add s_11_5 s_11_4
        let s_11_6: i128 = (s_11_5 + s_11_4);
        // D s_11_7: bit-extract s_11_2 s_11_0 s_11_6
        let s_11_7: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_6).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: u8 = (s_11_7.value() as u8);
        // C s_11_9: const #16s : i
        let s_11_9: i128 = 16;
        // D s_11_10: read-var new_spsr:u32
        let s_11_10: u32 = fn_state.new_spsr;
        // D s_11_11: cast zx s_11_10 -> bv
        let s_11_11: Bits = Bits::new(s_11_10 as u128, 32u16);
        // D s_11_12: cast zx s_11_8 -> bv
        let s_11_12: Bits = Bits::new(s_11_8 as u128, 8u16);
        // C s_11_13: const #7s : i
        let s_11_13: i128 = 7;
        // C s_11_14: const #1u : u64
        let s_11_14: u64 = 1;
        // C s_11_15: cast zx s_11_14 -> bv
        let s_11_15: Bits = Bits::new(s_11_14 as u128, 64u16);
        // C s_11_16: lsl s_11_15 s_11_13
        let s_11_16: Bits = s_11_15 << s_11_13;
        // C s_11_17: sub s_11_16 s_11_15
        let s_11_17: Bits = ((s_11_16) - (s_11_15));
        // D s_11_18: and s_11_12 s_11_17
        let s_11_18: Bits = ((s_11_12) & (s_11_17));
        // D s_11_19: lsl s_11_18 s_11_9
        let s_11_19: Bits = s_11_18 << s_11_9;
        // C s_11_20: lsl s_11_17 s_11_9
        let s_11_20: Bits = s_11_17 << s_11_9;
        // C s_11_21: cmpl s_11_20
        let s_11_21: Bits = !s_11_20;
        // D s_11_22: and s_11_11 s_11_21
        let s_11_22: Bits = ((s_11_11) & (s_11_21));
        // D s_11_23: or s_11_22 s_11_19
        let s_11_23: Bits = ((s_11_22) | (s_11_19));
        // D s_11_24: cast reint s_11_23 -> u32
        let s_11_24: u32 = (s_11_23.value() as u32);
        // D s_11_25: write-var new_spsr <= s_11_24
        fn_state.new_spsr = s_11_24;
        // N s_11_26: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #24s : i
        let s_12_0: i128 = 24;
        // D s_12_1: read-var value_name:u32
        let s_12_1: u32 = fn_state.value_name;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 32u16);
        // C s_12_3: const #1s : i64
        let s_12_3: i64 = 1;
        // C s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // C s_12_5: const #7s : i
        let s_12_5: i128 = 7;
        // C s_12_6: add s_12_5 s_12_4
        let s_12_6: i128 = (s_12_5 + s_12_4);
        // D s_12_7: bit-extract s_12_2 s_12_0 s_12_6
        let s_12_7: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_6).unwrap(),
        ));
        // D s_12_8: cast reint s_12_7 -> u8
        let s_12_8: u8 = (s_12_7.value() as u8);
        // C s_12_9: const #24s : i
        let s_12_9: i128 = 24;
        // D s_12_10: read-var new_spsr:u32
        let s_12_10: u32 = fn_state.new_spsr;
        // D s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 32u16);
        // D s_12_12: cast zx s_12_8 -> bv
        let s_12_12: Bits = Bits::new(s_12_8 as u128, 8u16);
        // C s_12_13: const #7s : i
        let s_12_13: i128 = 7;
        // C s_12_14: const #1u : u64
        let s_12_14: u64 = 1;
        // C s_12_15: cast zx s_12_14 -> bv
        let s_12_15: Bits = Bits::new(s_12_14 as u128, 64u16);
        // C s_12_16: lsl s_12_15 s_12_13
        let s_12_16: Bits = s_12_15 << s_12_13;
        // C s_12_17: sub s_12_16 s_12_15
        let s_12_17: Bits = ((s_12_16) - (s_12_15));
        // D s_12_18: and s_12_12 s_12_17
        let s_12_18: Bits = ((s_12_12) & (s_12_17));
        // D s_12_19: lsl s_12_18 s_12_9
        let s_12_19: Bits = s_12_18 << s_12_9;
        // C s_12_20: lsl s_12_17 s_12_9
        let s_12_20: Bits = s_12_17 << s_12_9;
        // C s_12_21: cmpl s_12_20
        let s_12_21: Bits = !s_12_20;
        // D s_12_22: and s_12_11 s_12_21
        let s_12_22: Bits = ((s_12_11) & (s_12_21));
        // D s_12_23: or s_12_22 s_12_19
        let s_12_23: Bits = ((s_12_22) | (s_12_19));
        // D s_12_24: cast reint s_12_23 -> u32
        let s_12_24: u32 = (s_12_23.value() as u32);
        // D s_12_25: write-var new_spsr <= s_12_24
        fn_state.new_spsr = s_12_24;
        // N s_12_26: jump b2
        return block_2(state, tracer, fn_state);
    }
}
