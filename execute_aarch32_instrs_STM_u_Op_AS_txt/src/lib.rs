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
use Rmode_read::*;
use MemS_set::*;
use PCStoreValue::*;
use R_read::*;
use BitCount::*;
use common::*;
pub fn execute_aarch32_instrs_STM_u_Op_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    increment_name: bool,
    n: i64,
    registers: u16,
    wordhigher: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u32,
        i: i64,
        lengthshadow_7907: i128,
        gs_323623: bool,
        increment_name: bool,
        n: i64,
        registers: u16,
        wordhigher: bool,
    }
    let fn_state = FunctionState {
        increment_name,
        n,
        registers,
        wordhigher,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #432u : u32
        let s_0_3: u32 = 432;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b22 b1
        if s_0_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16983u : u32
        let s_1_0: u32 = 16983;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 5u16);
        // C s_1_3: const #352u : u32
        let s_1_3: u32 = 352;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 5u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b21 b2
        if s_1_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16983u : u32
        let s_2_0: u32 = 16983;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 5u16);
        // C s_2_3: const #416u : u32
        let s_2_3: u32 = 416;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 5u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // D s_2_7: write-var gs#323623 <= s_2_6
        fn_state.gs_323623 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#323623:u8
        let s_3_0: bool = fn_state.gs_323623;
        // N s_3_1: branch s_3_0 b20 b4
        if s_3_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var registers:u16
        let s_4_0: u16 = fn_state.registers;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 16u16);
        // D s_4_2: call BitCount(s_4_1)
        let s_4_2: i128 = BitCount(state, tracer, s_4_1);
        // C s_4_3: const #4s : i
        let s_4_3: i128 = 4;
        // D s_4_4: mul s_4_3 s_4_2
        let s_4_4: i128 = ((s_4_3) * (s_4_2));
        // D s_4_5: write-var lengthshadow#7907 <= s_4_4
        fn_state.lengthshadow_7907 = s_4_4;
        // D s_4_6: read-var increment_name:u8
        let s_4_6: bool = fn_state.increment_name;
        // N s_4_7: branch s_4_6 b19 b5
        if s_4_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call R_read(s_5_1)
        let s_5_2: u32 = R_read(state, tracer, s_5_1);
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_4: read-var lengthshadow#7907:i
        let s_5_4: i128 = fn_state.lengthshadow_7907;
        // D s_5_5: cast cvt s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 128);
        // D s_5_6: sub s_5_3 s_5_5
        let s_5_6: Bits = ((s_5_3) - (s_5_5));
        // D s_5_7: cast reint s_5_6 -> u32
        let s_5_7: u32 = (s_5_6.value() as u32);
        // D s_5_8: write-var address <= s_5_7
        fn_state.address = s_5_7;
        // N s_5_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var wordhigher:u8
        let s_6_0: bool = fn_state.wordhigher;
        // N s_6_1: branch s_6_0 b18 b7
        if s_6_0 {
            return block_18(state, tracer, fn_state);
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
        // C s_8_0: const #0s : i64
        let s_8_0: i64 = 0;
        // D s_8_1: write-var i <= s_8_0
        fn_state.i = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var i:i64
        let s_9_0: i64 = fn_state.i;
        // C s_9_1: const #14s : i64
        let s_9_1: i64 = 14;
        // D s_9_2: cmp-gt s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) > (s_9_1));
        // N s_9_3: branch s_9_2 b15 b10
        if s_9_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var registers:u16
        let s_10_0: u16 = fn_state.registers;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 16u16);
        // D s_10_2: read-var i:i64
        let s_10_2: i64 = fn_state.i;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // C s_10_4: const #1u : u64
        let s_10_4: u64 = 1;
        // D s_10_5: bit-extract s_10_1 s_10_3 s_10_4
        let s_10_5: Bits = (Bits::new(
            ((s_10_1) >> (s_10_3)).value(),
            u16::try_from(s_10_4).unwrap(),
        ));
        // D s_10_6: cast reint s_10_5 -> u8
        let s_10_6: bool = ((s_10_5.value()) != 0);
        // C s_10_7: const #0s : i
        let s_10_7: i128 = 0;
        // C s_10_8: const #0u : u64
        let s_10_8: u64 = 0;
        // D s_10_9: cast zx s_10_6 -> u64
        let s_10_9: u64 = (s_10_6 as u64);
        // C s_10_10: const #1u : u64
        let s_10_10: u64 = 1;
        // D s_10_11: and s_10_9 s_10_10
        let s_10_11: u64 = ((s_10_9) & (s_10_10));
        // D s_10_12: cmp-eq s_10_11 s_10_10
        let s_10_12: bool = ((s_10_11) == (s_10_10));
        // D s_10_13: lsl s_10_9 s_10_7
        let s_10_13: u64 = s_10_9 << s_10_7;
        // D s_10_14: or s_10_8 s_10_13
        let s_10_14: u64 = ((s_10_8) | (s_10_13));
        // D s_10_15: cmpl s_10_13
        let s_10_15: u64 = !s_10_13;
        // D s_10_16: and s_10_8 s_10_15
        let s_10_16: u64 = ((s_10_8) & (s_10_15));
        // D s_10_17: select s_10_12 s_10_14 s_10_16
        let s_10_17: u64 = if s_10_12 { s_10_14 } else { s_10_16 };
        // D s_10_18: cast trunc s_10_17 -> u8
        let s_10_18: bool = ((s_10_17) != 0);
        // D s_10_19: cast zx s_10_18 -> bv
        let s_10_19: Bits = Bits::new(s_10_18 as u128, 1u16);
        // C s_10_20: const #1u : u8
        let s_10_20: bool = true;
        // C s_10_21: cast zx s_10_20 -> bv
        let s_10_21: Bits = Bits::new(s_10_20 as u128, 1u16);
        // D s_10_22: cmp-eq s_10_19 s_10_21
        let s_10_22: bool = ((s_10_19) == (s_10_21));
        // N s_10_23: branch s_10_22 b13 b11
        if s_10_22 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var i:i64
        let s_12_0: i64 = fn_state.i;
        // C s_12_1: const #1s : i64
        let s_12_1: i64 = 1;
        // D s_12_2: add s_12_0 s_12_1
        let s_12_2: i64 = (s_12_0 + s_12_1);
        // D s_12_3: write-var i <= s_12_2
        fn_state.i = s_12_2;
        // N s_12_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var i:i64
        let s_13_0: i64 = fn_state.i;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // C s_13_2: const #352u : u32
        let s_13_2: u32 = 352;
        // D s_13_3: read-reg s_13_2:u8
        let s_13_3: u8 = {
            let value = state.read_register::<u8>(s_13_2 as isize);
            tracer.read_register(s_13_2 as isize, value);
            value
        };
        // D s_13_4: call Rmode_read(s_13_1, s_13_3)
        let s_13_4: u32 = Rmode_read(state, tracer, s_13_1, s_13_3);
        // C s_13_5: const #4s : i
        let s_13_5: i128 = 4;
        // D s_13_6: cast zx s_13_4 -> bv
        let s_13_6: Bits = Bits::new(s_13_4 as u128, 32u16);
        // D s_13_7: read-var address:u32
        let s_13_7: u32 = fn_state.address;
        // D s_13_8: call MemS_set(s_13_7, s_13_5, s_13_6)
        let s_13_8: () = MemS_set(state, tracer, s_13_7, s_13_5, s_13_6);
        // N s_13_9: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #4s : i
        let s_14_0: i128 = 4;
        // D s_14_1: read-var address:u32
        let s_14_1: u32 = fn_state.address;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 32u16);
        // C s_14_3: cast cvt s_14_0 -> bv
        let s_14_3: Bits = Bits::new(s_14_0 as u128, 128);
        // D s_14_4: add s_14_2 s_14_3
        let s_14_4: Bits = (s_14_2 + s_14_3);
        // D s_14_5: cast reint s_14_4 -> u32
        let s_14_5: u32 = (s_14_4.value() as u32);
        // D s_14_6: write-var address <= s_14_5
        fn_state.address = s_14_5;
        // N s_14_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #15s : i
        let s_15_0: i128 = 15;
        // D s_15_1: read-var registers:u16
        let s_15_1: u16 = fn_state.registers;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 16u16);
        // C s_15_3: const #1u : u64
        let s_15_3: u64 = 1;
        // D s_15_4: bit-extract s_15_2 s_15_0 s_15_3
        let s_15_4: Bits = (Bits::new(
            ((s_15_2) >> (s_15_0)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_5: cast reint s_15_4 -> u8
        let s_15_5: bool = ((s_15_4.value()) != 0);
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // C s_15_7: const #0u : u64
        let s_15_7: u64 = 0;
        // D s_15_8: cast zx s_15_5 -> u64
        let s_15_8: u64 = (s_15_5 as u64);
        // C s_15_9: const #1u : u64
        let s_15_9: u64 = 1;
        // D s_15_10: and s_15_8 s_15_9
        let s_15_10: u64 = ((s_15_8) & (s_15_9));
        // D s_15_11: cmp-eq s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) == (s_15_9));
        // D s_15_12: lsl s_15_8 s_15_6
        let s_15_12: u64 = s_15_8 << s_15_6;
        // D s_15_13: or s_15_7 s_15_12
        let s_15_13: u64 = ((s_15_7) | (s_15_12));
        // D s_15_14: cmpl s_15_12
        let s_15_14: u64 = !s_15_12;
        // D s_15_15: and s_15_7 s_15_14
        let s_15_15: u64 = ((s_15_7) & (s_15_14));
        // D s_15_16: select s_15_11 s_15_13 s_15_15
        let s_15_16: u64 = if s_15_11 { s_15_13 } else { s_15_15 };
        // D s_15_17: cast trunc s_15_16 -> u8
        let s_15_17: bool = ((s_15_16) != 0);
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 1u16);
        // C s_15_19: const #1u : u8
        let s_15_19: bool = true;
        // C s_15_20: cast zx s_15_19 -> bv
        let s_15_20: Bits = Bits::new(s_15_19 as u128, 1u16);
        // D s_15_21: cmp-eq s_15_18 s_15_20
        let s_15_21: bool = ((s_15_18) == (s_15_20));
        // N s_15_22: branch s_15_21 b17 b16
        if s_15_21 {
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
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call PCStoreValue(s_17_0)
        let s_17_1: u32 = PCStoreValue(state, tracer, s_17_0);
        // C s_17_2: const #4s : i
        let s_17_2: i128 = 4;
        // S s_17_3: cast zx s_17_1 -> bv
        let s_17_3: Bits = Bits::new(s_17_1 as u128, 32u16);
        // D s_17_4: read-var address:u32
        let s_17_4: u32 = fn_state.address;
        // D s_17_5: call MemS_set(s_17_4, s_17_2, s_17_3)
        let s_17_5: () = MemS_set(state, tracer, s_17_4, s_17_2, s_17_3);
        // N s_17_6: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #4s : i
        let s_18_0: i128 = 4;
        // D s_18_1: read-var address:u32
        let s_18_1: u32 = fn_state.address;
        // D s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 32u16);
        // C s_18_3: cast cvt s_18_0 -> bv
        let s_18_3: Bits = Bits::new(s_18_0 as u128, 128);
        // D s_18_4: add s_18_2 s_18_3
        let s_18_4: Bits = (s_18_2 + s_18_3);
        // D s_18_5: cast reint s_18_4 -> u32
        let s_18_5: u32 = (s_18_4.value() as u32);
        // D s_18_6: write-var address <= s_18_5
        fn_state.address = s_18_5;
        // N s_18_7: jump b8
        return block_8(state, tracer, fn_state);
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
        // D s_19_3: write-var address <= s_19_2
        fn_state.address = s_19_2;
        // N s_19_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#323623 <= s_21_0
        fn_state.gs_323623 = s_21_0;
        // N s_21_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
}
