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
use u__UNKNOWN_bits::*;
use LoadWritePC::*;
use R_read::*;
use R_set::*;
use BitCount::*;
use MemS_read::*;
use common::*;
pub fn execute_aarch32_instrs_LDMDA_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    registers: u16,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_879233: Bits,
        gs_296876: bool,
        address: u32,
        gs_296874: bool,
        gs_879241: Bits,
        i: i64,
        n: i64,
        registers: u16,
        wback: bool,
    }
    let fn_state = FunctionState {
        n,
        registers,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: read-var registers:u16
        let s_0_3: u16 = fn_state.registers;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 16u16);
        // D s_0_5: call BitCount(s_0_4)
        let s_0_5: i128 = BitCount(state, tracer, s_0_4);
        // C s_0_6: const #4s : i
        let s_0_6: i128 = 4;
        // D s_0_7: mul s_0_6 s_0_5
        let s_0_7: i128 = ((s_0_6) * (s_0_5));
        // D s_0_8: cast zx s_0_2 -> bv
        let s_0_8: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_9: cast cvt s_0_7 -> bv
        let s_0_9: Bits = Bits::new(s_0_7 as u128, 128);
        // D s_0_10: sub s_0_8 s_0_9
        let s_0_10: Bits = ((s_0_8) - (s_0_9));
        // D s_0_11: cast reint s_0_10 -> u32
        let s_0_11: u32 = (s_0_10.value() as u32);
        // C s_0_12: const #4s : i
        let s_0_12: i128 = 4;
        // D s_0_13: cast zx s_0_11 -> bv
        let s_0_13: Bits = Bits::new(s_0_11 as u128, 32u16);
        // C s_0_14: cast cvt s_0_12 -> bv
        let s_0_14: Bits = Bits::new(s_0_12 as u128, 128);
        // D s_0_15: add s_0_13 s_0_14
        let s_0_15: Bits = (s_0_13 + s_0_14);
        // D s_0_16: cast reint s_0_15 -> u32
        let s_0_16: u32 = (s_0_15.value() as u32);
        // D s_0_17: write-var address <= s_0_16
        fn_state.address = s_0_16;
        // C s_0_18: const #0s : i64
        let s_0_18: i64 = 0;
        // D s_0_19: write-var i <= s_0_18
        fn_state.i = s_0_18;
        // N s_0_20: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // C s_1_1: const #14s : i64
        let s_1_1: i64 = 14;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b7 b2
        if s_1_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var registers:u16
        let s_2_0: u16 = fn_state.registers;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 16u16);
        // D s_2_2: read-var i:i64
        let s_2_2: i64 = fn_state.i;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // C s_2_4: const #1u : u64
        let s_2_4: u64 = 1;
        // D s_2_5: bit-extract s_2_1 s_2_3 s_2_4
        let s_2_5: Bits = (Bits::new(
            ((s_2_1) >> (s_2_3)).value(),
            u16::try_from(s_2_4).unwrap(),
        ));
        // D s_2_6: cast reint s_2_5 -> u8
        let s_2_6: bool = ((s_2_5.value()) != 0);
        // C s_2_7: const #0s : i
        let s_2_7: i128 = 0;
        // C s_2_8: const #0u : u64
        let s_2_8: u64 = 0;
        // D s_2_9: cast zx s_2_6 -> u64
        let s_2_9: u64 = (s_2_6 as u64);
        // C s_2_10: const #1u : u64
        let s_2_10: u64 = 1;
        // D s_2_11: and s_2_9 s_2_10
        let s_2_11: u64 = ((s_2_9) & (s_2_10));
        // D s_2_12: cmp-eq s_2_11 s_2_10
        let s_2_12: bool = ((s_2_11) == (s_2_10));
        // D s_2_13: lsl s_2_9 s_2_7
        let s_2_13: u64 = s_2_9 << s_2_7;
        // D s_2_14: or s_2_8 s_2_13
        let s_2_14: u64 = ((s_2_8) | (s_2_13));
        // D s_2_15: cmpl s_2_13
        let s_2_15: u64 = !s_2_13;
        // D s_2_16: and s_2_8 s_2_15
        let s_2_16: u64 = ((s_2_8) & (s_2_15));
        // D s_2_17: select s_2_12 s_2_14 s_2_16
        let s_2_17: u64 = if s_2_12 { s_2_14 } else { s_2_16 };
        // D s_2_18: cast trunc s_2_17 -> u8
        let s_2_18: bool = ((s_2_17) != 0);
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 1u16);
        // C s_2_20: const #1u : u8
        let s_2_20: bool = true;
        // C s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 1u16);
        // D s_2_22: cmp-eq s_2_19 s_2_21
        let s_2_22: bool = ((s_2_19) == (s_2_21));
        // N s_2_23: branch s_2_22 b5 b3
        if s_2_22 {
            return block_5(state, tracer, fn_state);
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
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #1s : i64
        let s_4_1: i64 = 1;
        // D s_4_2: add s_4_0 s_4_1
        let s_4_2: i64 = (s_4_0 + s_4_1);
        // D s_4_3: write-var i <= s_4_2
        fn_state.i = s_4_2;
        // N s_4_4: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #4s : i64
        let s_5_0: i64 = 4;
        // D s_5_1: read-var address:u32
        let s_5_1: u32 = fn_state.address;
        // D s_5_2: call MemS_read(s_5_1, s_5_0)
        let s_5_2: Bits = MemS_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: write-var gs#879233 <= s_5_2
        fn_state.gs_879233 = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#879233:bv
        let s_6_0: Bits = fn_state.gs_879233;
        // D s_6_1: cast reint s_6_0 -> u32
        let s_6_1: u32 = (s_6_0.value() as u32);
        // D s_6_2: read-var i:i64
        let s_6_2: i64 = fn_state.i;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: call R_set(s_6_3, s_6_1)
        let s_6_4: () = R_set(state, tracer, s_6_3, s_6_1);
        // C s_6_5: const #4s : i
        let s_6_5: i128 = 4;
        // D s_6_6: read-var address:u32
        let s_6_6: u32 = fn_state.address;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 32u16);
        // C s_6_8: cast cvt s_6_5 -> bv
        let s_6_8: Bits = Bits::new(s_6_5 as u128, 128);
        // D s_6_9: add s_6_7 s_6_8
        let s_6_9: Bits = (s_6_7 + s_6_8);
        // D s_6_10: cast reint s_6_9 -> u32
        let s_6_10: u32 = (s_6_9.value() as u32);
        // D s_6_11: write-var address <= s_6_10
        fn_state.address = s_6_10;
        // N s_6_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #15s : i
        let s_7_0: i128 = 15;
        // D s_7_1: read-var registers:u16
        let s_7_1: u16 = fn_state.registers;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 16u16);
        // C s_7_3: const #1u : u64
        let s_7_3: u64 = 1;
        // D s_7_4: bit-extract s_7_2 s_7_0 s_7_3
        let s_7_4: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_3).unwrap(),
        ));
        // D s_7_5: cast reint s_7_4 -> u8
        let s_7_5: bool = ((s_7_4.value()) != 0);
        // C s_7_6: const #0s : i
        let s_7_6: i128 = 0;
        // C s_7_7: const #0u : u64
        let s_7_7: u64 = 0;
        // D s_7_8: cast zx s_7_5 -> u64
        let s_7_8: u64 = (s_7_5 as u64);
        // C s_7_9: const #1u : u64
        let s_7_9: u64 = 1;
        // D s_7_10: and s_7_8 s_7_9
        let s_7_10: u64 = ((s_7_8) & (s_7_9));
        // D s_7_11: cmp-eq s_7_10 s_7_9
        let s_7_11: bool = ((s_7_10) == (s_7_9));
        // D s_7_12: lsl s_7_8 s_7_6
        let s_7_12: u64 = s_7_8 << s_7_6;
        // D s_7_13: or s_7_7 s_7_12
        let s_7_13: u64 = ((s_7_7) | (s_7_12));
        // D s_7_14: cmpl s_7_12
        let s_7_14: u64 = !s_7_12;
        // D s_7_15: and s_7_7 s_7_14
        let s_7_15: u64 = ((s_7_7) & (s_7_14));
        // D s_7_16: select s_7_11 s_7_13 s_7_15
        let s_7_16: u64 = if s_7_11 { s_7_13 } else { s_7_15 };
        // D s_7_17: cast trunc s_7_16 -> u8
        let s_7_17: bool = ((s_7_16) != 0);
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 1u16);
        // C s_7_19: const #1u : u8
        let s_7_19: bool = true;
        // C s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 1u16);
        // D s_7_21: cmp-eq s_7_18 s_7_20
        let s_7_21: bool = ((s_7_18) == (s_7_20));
        // N s_7_22: branch s_7_21 b21 b8
        if s_7_21 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var wback:u8
        let s_9_0: bool = fn_state.wback;
        // N s_9_1: branch s_9_0 b20 b10
        if s_9_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#296874 <= s_10_0
        fn_state.gs_296874 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#296874:u8
        let s_11_0: bool = fn_state.gs_296874;
        // N s_11_1: branch s_11_0 b19 b12
        if s_11_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var wback:u8
        let s_13_0: bool = fn_state.wback;
        // N s_13_1: branch s_13_0 b18 b14
        if s_13_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#296876 <= s_14_0
        fn_state.gs_296876 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#296876:u8
        let s_15_0: bool = fn_state.gs_296876;
        // N s_15_1: branch s_15_0 b17 b16
        if s_15_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #32s : i64
        let s_17_0: i64 = 32;
        // C s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // S s_17_2: call __UNKNOWN_bits(s_17_1)
        let s_17_2: Bits = u__UNKNOWN_bits(state, tracer, s_17_1);
        // S s_17_3: cast reint s_17_2 -> u32
        let s_17_3: u32 = (s_17_2.value() as u32);
        // D s_17_4: read-var n:i64
        let s_17_4: i64 = fn_state.n;
        // D s_17_5: cast zx s_17_4 -> i
        let s_17_5: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_6: call R_set(s_17_5, s_17_3)
        let s_17_6: () = R_set(state, tracer, s_17_5, s_17_3);
        // N s_17_7: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var registers:u16
        let s_18_0: u16 = fn_state.registers;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 16u16);
        // D s_18_2: read-var n:i64
        let s_18_2: i64 = fn_state.n;
        // D s_18_3: cast zx s_18_2 -> i
        let s_18_3: i128 = (i128::try_from(s_18_2).unwrap());
        // C s_18_4: const #1u : u64
        let s_18_4: u64 = 1;
        // D s_18_5: bit-extract s_18_1 s_18_3 s_18_4
        let s_18_5: Bits = (Bits::new(
            ((s_18_1) >> (s_18_3)).value(),
            u16::try_from(s_18_4).unwrap(),
        ));
        // D s_18_6: cast reint s_18_5 -> u8
        let s_18_6: bool = ((s_18_5.value()) != 0);
        // C s_18_7: const #0s : i
        let s_18_7: i128 = 0;
        // C s_18_8: const #0u : u64
        let s_18_8: u64 = 0;
        // D s_18_9: cast zx s_18_6 -> u64
        let s_18_9: u64 = (s_18_6 as u64);
        // C s_18_10: const #1u : u64
        let s_18_10: u64 = 1;
        // D s_18_11: and s_18_9 s_18_10
        let s_18_11: u64 = ((s_18_9) & (s_18_10));
        // D s_18_12: cmp-eq s_18_11 s_18_10
        let s_18_12: bool = ((s_18_11) == (s_18_10));
        // D s_18_13: lsl s_18_9 s_18_7
        let s_18_13: u64 = s_18_9 << s_18_7;
        // D s_18_14: or s_18_8 s_18_13
        let s_18_14: u64 = ((s_18_8) | (s_18_13));
        // D s_18_15: cmpl s_18_13
        let s_18_15: u64 = !s_18_13;
        // D s_18_16: and s_18_8 s_18_15
        let s_18_16: u64 = ((s_18_8) & (s_18_15));
        // D s_18_17: select s_18_12 s_18_14 s_18_16
        let s_18_17: u64 = if s_18_12 { s_18_14 } else { s_18_16 };
        // D s_18_18: cast trunc s_18_17 -> u8
        let s_18_18: bool = ((s_18_17) != 0);
        // D s_18_19: cast zx s_18_18 -> bv
        let s_18_19: Bits = Bits::new(s_18_18 as u128, 1u16);
        // C s_18_20: const #1u : u8
        let s_18_20: bool = true;
        // C s_18_21: cast zx s_18_20 -> bv
        let s_18_21: Bits = Bits::new(s_18_20 as u128, 1u16);
        // D s_18_22: cmp-eq s_18_19 s_18_21
        let s_18_22: bool = ((s_18_19) == (s_18_21));
        // D s_18_23: write-var gs#296876 <= s_18_22
        fn_state.gs_296876 = s_18_22;
        // N s_18_24: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var n:i64
        let s_19_0: i64 = fn_state.n;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: call R_read(s_19_1)
        let s_19_2: u32 = R_read(state, tracer, s_19_1);
        // D s_19_3: read-var registers:u16
        let s_19_3: u16 = fn_state.registers;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 16u16);
        // D s_19_5: call BitCount(s_19_4)
        let s_19_5: i128 = BitCount(state, tracer, s_19_4);
        // C s_19_6: const #4s : i
        let s_19_6: i128 = 4;
        // D s_19_7: mul s_19_6 s_19_5
        let s_19_7: i128 = ((s_19_6) * (s_19_5));
        // D s_19_8: cast zx s_19_2 -> bv
        let s_19_8: Bits = Bits::new(s_19_2 as u128, 32u16);
        // D s_19_9: cast cvt s_19_7 -> bv
        let s_19_9: Bits = Bits::new(s_19_7 as u128, 128);
        // D s_19_10: sub s_19_8 s_19_9
        let s_19_10: Bits = ((s_19_8) - (s_19_9));
        // D s_19_11: cast reint s_19_10 -> u32
        let s_19_11: u32 = (s_19_10.value() as u32);
        // D s_19_12: read-var n:i64
        let s_19_12: i64 = fn_state.n;
        // D s_19_13: cast zx s_19_12 -> i
        let s_19_13: i128 = (i128::try_from(s_19_12).unwrap());
        // D s_19_14: call R_set(s_19_13, s_19_11)
        let s_19_14: () = R_set(state, tracer, s_19_13, s_19_11);
        // N s_19_15: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var registers:u16
        let s_20_0: u16 = fn_state.registers;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 16u16);
        // D s_20_2: read-var n:i64
        let s_20_2: i64 = fn_state.n;
        // D s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // C s_20_4: const #1u : u64
        let s_20_4: u64 = 1;
        // D s_20_5: bit-extract s_20_1 s_20_3 s_20_4
        let s_20_5: Bits = (Bits::new(
            ((s_20_1) >> (s_20_3)).value(),
            u16::try_from(s_20_4).unwrap(),
        ));
        // D s_20_6: cast reint s_20_5 -> u8
        let s_20_6: bool = ((s_20_5.value()) != 0);
        // C s_20_7: const #0s : i
        let s_20_7: i128 = 0;
        // C s_20_8: const #0u : u64
        let s_20_8: u64 = 0;
        // D s_20_9: cast zx s_20_6 -> u64
        let s_20_9: u64 = (s_20_6 as u64);
        // C s_20_10: const #1u : u64
        let s_20_10: u64 = 1;
        // D s_20_11: and s_20_9 s_20_10
        let s_20_11: u64 = ((s_20_9) & (s_20_10));
        // D s_20_12: cmp-eq s_20_11 s_20_10
        let s_20_12: bool = ((s_20_11) == (s_20_10));
        // D s_20_13: lsl s_20_9 s_20_7
        let s_20_13: u64 = s_20_9 << s_20_7;
        // D s_20_14: or s_20_8 s_20_13
        let s_20_14: u64 = ((s_20_8) | (s_20_13));
        // D s_20_15: cmpl s_20_13
        let s_20_15: u64 = !s_20_13;
        // D s_20_16: and s_20_8 s_20_15
        let s_20_16: u64 = ((s_20_8) & (s_20_15));
        // D s_20_17: select s_20_12 s_20_14 s_20_16
        let s_20_17: u64 = if s_20_12 { s_20_14 } else { s_20_16 };
        // D s_20_18: cast trunc s_20_17 -> u8
        let s_20_18: bool = ((s_20_17) != 0);
        // D s_20_19: cast zx s_20_18 -> bv
        let s_20_19: Bits = Bits::new(s_20_18 as u128, 1u16);
        // C s_20_20: const #0u : u8
        let s_20_20: bool = false;
        // C s_20_21: cast zx s_20_20 -> bv
        let s_20_21: Bits = Bits::new(s_20_20 as u128, 1u16);
        // D s_20_22: cmp-eq s_20_19 s_20_21
        let s_20_22: bool = ((s_20_19) == (s_20_21));
        // D s_20_23: write-var gs#296874 <= s_20_22
        fn_state.gs_296874 = s_20_22;
        // N s_20_24: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #4s : i64
        let s_21_0: i64 = 4;
        // D s_21_1: read-var address:u32
        let s_21_1: u32 = fn_state.address;
        // D s_21_2: call MemS_read(s_21_1, s_21_0)
        let s_21_2: Bits = MemS_read(state, tracer, s_21_1, s_21_0);
        // D s_21_3: write-var gs#879241 <= s_21_2
        fn_state.gs_879241 = s_21_2;
        // N s_21_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#879241:bv
        let s_22_0: Bits = fn_state.gs_879241;
        // D s_22_1: cast reint s_22_0 -> u32
        let s_22_1: u32 = (s_22_0.value() as u32);
        // D s_22_2: call LoadWritePC(s_22_1)
        let s_22_2: () = LoadWritePC(state, tracer, s_22_1);
        // N s_22_3: jump b9
        return block_9(state, tracer, fn_state);
    }
}
