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
use neq_int::*;
use u__UNKNOWN_bits::*;
use PCStoreValue::*;
use R_read::*;
use R_set::*;
use MemU_set::*;
use LowestSetBit::*;
use MemA_set::*;
use BitCount::*;
use common::*;
pub fn execute_aarch32_instrs_PUSH_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    UnalignedAllowed: bool,
    registers: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_299325: bool,
        address: u32,
        i: i64,
        UnalignedAllowed: bool,
        registers: u16,
    }
    let fn_state = FunctionState {
        UnalignedAllowed,
        registers,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #13s : i
        let s_0_0: i128 = 13;
        // S s_0_1: call R_read(s_0_0)
        let s_0_1: u32 = R_read(state, tracer, s_0_0);
        // D s_0_2: read-var registers:u16
        let s_0_2: u16 = fn_state.registers;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 16u16);
        // D s_0_4: call BitCount(s_0_3)
        let s_0_4: i128 = BitCount(state, tracer, s_0_3);
        // C s_0_5: const #4s : i
        let s_0_5: i128 = 4;
        // D s_0_6: mul s_0_5 s_0_4
        let s_0_6: i128 = ((s_0_5) * (s_0_4));
        // S s_0_7: cast zx s_0_1 -> bv
        let s_0_7: Bits = Bits::new(s_0_1 as u128, 32u16);
        // D s_0_8: cast cvt s_0_6 -> bv
        let s_0_8: Bits = Bits::new(s_0_6 as u128, 128);
        // D s_0_9: sub s_0_7 s_0_8
        let s_0_9: Bits = ((s_0_7) - (s_0_8));
        // D s_0_10: cast reint s_0_9 -> u32
        let s_0_10: u32 = (s_0_9.value() as u32);
        // D s_0_11: write-var address <= s_0_10
        fn_state.address = s_0_10;
        // C s_0_12: const #0s : i64
        let s_0_12: i64 = 0;
        // D s_0_13: write-var i <= s_0_12
        fn_state.i = s_0_12;
        // N s_0_14: jump b1
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
        // N s_1_3: branch s_1_2 b14 b2
        if s_1_2 {
            return block_14(state, tracer, fn_state);
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
        // C s_5_0: const #13s : i
        let s_5_0: i128 = 13;
        // D s_5_1: read-var i:i64
        let s_5_1: i64 = fn_state.i;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // N s_5_4: branch s_5_3 b13 b6
        if s_5_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#299325 <= s_6_0
        fn_state.gs_299325 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#299325:u8
        let s_7_0: bool = fn_state.gs_299325;
        // N s_7_1: branch s_7_0 b12 b8
        if s_7_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var UnalignedAllowed:u8
        let s_8_0: bool = fn_state.UnalignedAllowed;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var i:i64
        let s_9_0: i64 = fn_state.i;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call R_read(s_9_1)
        let s_9_2: u32 = R_read(state, tracer, s_9_1);
        // C s_9_3: const #4s : i
        let s_9_3: i128 = 4;
        // D s_9_4: cast zx s_9_2 -> bv
        let s_9_4: Bits = Bits::new(s_9_2 as u128, 32u16);
        // D s_9_5: read-var address:u32
        let s_9_5: u32 = fn_state.address;
        // D s_9_6: call MemA_set(s_9_5, s_9_3, s_9_4)
        let s_9_6: () = MemA_set(state, tracer, s_9_5, s_9_3, s_9_4);
        // N s_9_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #4s : i
        let s_10_0: i128 = 4;
        // D s_10_1: read-var address:u32
        let s_10_1: u32 = fn_state.address;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 32u16);
        // C s_10_3: cast cvt s_10_0 -> bv
        let s_10_3: Bits = Bits::new(s_10_0 as u128, 128);
        // D s_10_4: add s_10_2 s_10_3
        let s_10_4: Bits = (s_10_2 + s_10_3);
        // D s_10_5: cast reint s_10_4 -> u32
        let s_10_5: u32 = (s_10_4.value() as u32);
        // D s_10_6: write-var address <= s_10_5
        fn_state.address = s_10_5;
        // N s_10_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var i:i64
        let s_11_0: i64 = fn_state.i;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call R_read(s_11_1)
        let s_11_2: u32 = R_read(state, tracer, s_11_1);
        // C s_11_3: const #4s : i
        let s_11_3: i128 = 4;
        // D s_11_4: cast zx s_11_2 -> bv
        let s_11_4: Bits = Bits::new(s_11_2 as u128, 32u16);
        // D s_11_5: read-var address:u32
        let s_11_5: u32 = fn_state.address;
        // D s_11_6: call MemU_set(s_11_5, s_11_3, s_11_4)
        let s_11_6: () = MemU_set(state, tracer, s_11_5, s_11_3, s_11_4);
        // N s_11_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #32s : i64
        let s_12_0: i64 = 32;
        // C s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // S s_12_2: call __UNKNOWN_bits(s_12_1)
        let s_12_2: Bits = u__UNKNOWN_bits(state, tracer, s_12_1);
        // S s_12_3: cast reint s_12_2 -> u32
        let s_12_3: u32 = (s_12_2.value() as u32);
        // C s_12_4: const #4s : i
        let s_12_4: i128 = 4;
        // S s_12_5: cast zx s_12_3 -> bv
        let s_12_5: Bits = Bits::new(s_12_3 as u128, 32u16);
        // D s_12_6: read-var address:u32
        let s_12_6: u32 = fn_state.address;
        // D s_12_7: call MemA_set(s_12_6, s_12_4, s_12_5)
        let s_12_7: () = MemA_set(state, tracer, s_12_6, s_12_4, s_12_5);
        // N s_12_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var registers:u16
        let s_13_0: u16 = fn_state.registers;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 16u16);
        // D s_13_2: call LowestSetBit(s_13_1)
        let s_13_2: i128 = LowestSetBit(state, tracer, s_13_1);
        // D s_13_3: read-var i:i64
        let s_13_3: i64 = fn_state.i;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: call neq_int(s_13_4, s_13_2)
        let s_13_5: bool = neq_int(state, tracer, s_13_4, s_13_2);
        // D s_13_6: write-var gs#299325 <= s_13_5
        fn_state.gs_299325 = s_13_5;
        // N s_13_7: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #15s : i
        let s_14_0: i128 = 15;
        // D s_14_1: read-var registers:u16
        let s_14_1: u16 = fn_state.registers;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 16u16);
        // C s_14_3: const #1u : u64
        let s_14_3: u64 = 1;
        // D s_14_4: bit-extract s_14_2 s_14_0 s_14_3
        let s_14_4: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_5: cast reint s_14_4 -> u8
        let s_14_5: bool = ((s_14_4.value()) != 0);
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // C s_14_7: const #0u : u64
        let s_14_7: u64 = 0;
        // D s_14_8: cast zx s_14_5 -> u64
        let s_14_8: u64 = (s_14_5 as u64);
        // C s_14_9: const #1u : u64
        let s_14_9: u64 = 1;
        // D s_14_10: and s_14_8 s_14_9
        let s_14_10: u64 = ((s_14_8) & (s_14_9));
        // D s_14_11: cmp-eq s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) == (s_14_9));
        // D s_14_12: lsl s_14_8 s_14_6
        let s_14_12: u64 = s_14_8 << s_14_6;
        // D s_14_13: or s_14_7 s_14_12
        let s_14_13: u64 = ((s_14_7) | (s_14_12));
        // D s_14_14: cmpl s_14_12
        let s_14_14: u64 = !s_14_12;
        // D s_14_15: and s_14_7 s_14_14
        let s_14_15: u64 = ((s_14_7) & (s_14_14));
        // D s_14_16: select s_14_11 s_14_13 s_14_15
        let s_14_16: u64 = if s_14_11 { s_14_13 } else { s_14_15 };
        // D s_14_17: cast trunc s_14_16 -> u8
        let s_14_17: bool = ((s_14_16) != 0);
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 1u16);
        // C s_14_19: const #1u : u8
        let s_14_19: bool = true;
        // C s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 1u16);
        // D s_14_21: cmp-eq s_14_18 s_14_20
        let s_14_21: bool = ((s_14_18) == (s_14_20));
        // N s_14_22: branch s_14_21 b17 b15
        if s_14_21 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #13s : i
        let s_16_0: i128 = 13;
        // S s_16_1: call R_read(s_16_0)
        let s_16_1: u32 = R_read(state, tracer, s_16_0);
        // D s_16_2: read-var registers:u16
        let s_16_2: u16 = fn_state.registers;
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 16u16);
        // D s_16_4: call BitCount(s_16_3)
        let s_16_4: i128 = BitCount(state, tracer, s_16_3);
        // C s_16_5: const #4s : i
        let s_16_5: i128 = 4;
        // D s_16_6: mul s_16_5 s_16_4
        let s_16_6: i128 = ((s_16_5) * (s_16_4));
        // S s_16_7: cast zx s_16_1 -> bv
        let s_16_7: Bits = Bits::new(s_16_1 as u128, 32u16);
        // D s_16_8: cast cvt s_16_6 -> bv
        let s_16_8: Bits = Bits::new(s_16_6 as u128, 128);
        // D s_16_9: sub s_16_7 s_16_8
        let s_16_9: Bits = ((s_16_7) - (s_16_8));
        // D s_16_10: cast reint s_16_9 -> u32
        let s_16_10: u32 = (s_16_9.value() as u32);
        // C s_16_11: const #13s : i
        let s_16_11: i128 = 13;
        // D s_16_12: call R_set(s_16_11, s_16_10)
        let s_16_12: () = R_set(state, tracer, s_16_11, s_16_10);
        // N s_16_13: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var UnalignedAllowed:u8
        let s_17_0: bool = fn_state.UnalignedAllowed;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call PCStoreValue(s_18_0)
        let s_18_1: u32 = PCStoreValue(state, tracer, s_18_0);
        // C s_18_2: const #4s : i
        let s_18_2: i128 = 4;
        // S s_18_3: cast zx s_18_1 -> bv
        let s_18_3: Bits = Bits::new(s_18_1 as u128, 32u16);
        // D s_18_4: read-var address:u32
        let s_18_4: u32 = fn_state.address;
        // D s_18_5: call MemA_set(s_18_4, s_18_2, s_18_3)
        let s_18_5: () = MemA_set(state, tracer, s_18_4, s_18_2, s_18_3);
        // N s_18_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call PCStoreValue(s_19_0)
        let s_19_1: u32 = PCStoreValue(state, tracer, s_19_0);
        // C s_19_2: const #4s : i
        let s_19_2: i128 = 4;
        // S s_19_3: cast zx s_19_1 -> bv
        let s_19_3: Bits = Bits::new(s_19_1 as u128, 32u16);
        // D s_19_4: read-var address:u32
        let s_19_4: u32 = fn_state.address;
        // D s_19_5: call MemU_set(s_19_4, s_19_2, s_19_3)
        let s_19_5: () = MemU_set(state, tracer, s_19_4, s_19_2, s_19_3);
        // N s_19_6: jump b16
        return block_16(state, tracer, fn_state);
    }
}
