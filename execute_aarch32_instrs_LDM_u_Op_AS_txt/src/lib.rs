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
use R_read::*;
use Rmode_set::*;
use BitCount::*;
use MemS_read::*;
use common::*;
pub fn execute_aarch32_instrs_LDM_u_Op_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    increment_name: bool,
    n: i64,
    registers: u16,
    wordhigher: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        lengthshadow_7887: i128,
        address: u32,
        gs_912419: Bits,
        gs_323264: bool,
        i: i64,
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
        // N s_0_7: branch s_0_6 b20 b1
        if s_0_6 {
            return block_20(state, tracer, fn_state);
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
        // N s_1_7: branch s_1_6 b19 b2
        if s_1_6 {
            return block_19(state, tracer, fn_state);
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
        // D s_2_7: write-var gs#323264 <= s_2_6
        fn_state.gs_323264 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#323264:u8
        let s_3_0: bool = fn_state.gs_323264;
        // N s_3_1: branch s_3_0 b18 b4
        if s_3_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var registers:u15
        let s_4_0: u16 = fn_state.registers;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 15u16);
        // D s_4_2: call BitCount(s_4_1)
        let s_4_2: i128 = BitCount(state, tracer, s_4_1);
        // C s_4_3: const #4s : i
        let s_4_3: i128 = 4;
        // D s_4_4: mul s_4_3 s_4_2
        let s_4_4: i128 = ((s_4_3) * (s_4_2));
        // D s_4_5: write-var lengthshadow#7887 <= s_4_4
        fn_state.lengthshadow_7887 = s_4_4;
        // D s_4_6: read-var increment_name:u8
        let s_4_6: bool = fn_state.increment_name;
        // N s_4_7: branch s_4_6 b17 b5
        if s_4_6 {
            return block_17(state, tracer, fn_state);
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
        // D s_5_4: read-var lengthshadow#7887:i
        let s_5_4: i128 = fn_state.lengthshadow_7887;
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
        // N s_6_1: branch s_6_0 b16 b7
        if s_6_0 {
            return block_16(state, tracer, fn_state);
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
        // D s_10_0: read-var registers:u15
        let s_10_0: u16 = fn_state.registers;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 15u16);
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
        // C s_13_0: const #4s : i64
        let s_13_0: i64 = 4;
        // D s_13_1: read-var address:u32
        let s_13_1: u32 = fn_state.address;
        // D s_13_2: call MemS_read(s_13_1, s_13_0)
        let s_13_2: Bits = MemS_read(state, tracer, s_13_1, s_13_0);
        // D s_13_3: write-var gs#912419 <= s_13_2
        fn_state.gs_912419 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#912419:bv
        let s_14_0: Bits = fn_state.gs_912419;
        // D s_14_1: cast reint s_14_0 -> u32
        let s_14_1: u32 = (s_14_0.value() as u32);
        // D s_14_2: read-var i:i64
        let s_14_2: i64 = fn_state.i;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // C s_14_4: const #352u : u32
        let s_14_4: u32 = 352;
        // D s_14_5: read-reg s_14_4:u8
        let s_14_5: u8 = {
            let value = state.read_register::<u8>(s_14_4 as isize);
            tracer.read_register(s_14_4 as isize, value);
            value
        };
        // D s_14_6: call Rmode_set(s_14_3, s_14_5, s_14_1)
        let s_14_6: () = Rmode_set(state, tracer, s_14_3, s_14_5, s_14_1);
        // C s_14_7: const #4s : i
        let s_14_7: i128 = 4;
        // D s_14_8: read-var address:u32
        let s_14_8: u32 = fn_state.address;
        // D s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 32u16);
        // C s_14_10: cast cvt s_14_7 -> bv
        let s_14_10: Bits = Bits::new(s_14_7 as u128, 128);
        // D s_14_11: add s_14_9 s_14_10
        let s_14_11: Bits = (s_14_9 + s_14_10);
        // D s_14_12: cast reint s_14_11 -> u32
        let s_14_12: u32 = (s_14_11.value() as u32);
        // D s_14_13: write-var address <= s_14_12
        fn_state.address = s_14_12;
        // N s_14_14: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #4s : i
        let s_16_0: i128 = 4;
        // D s_16_1: read-var address:u32
        let s_16_1: u32 = fn_state.address;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 32u16);
        // C s_16_3: cast cvt s_16_0 -> bv
        let s_16_3: Bits = Bits::new(s_16_0 as u128, 128);
        // D s_16_4: add s_16_2 s_16_3
        let s_16_4: Bits = (s_16_2 + s_16_3);
        // D s_16_5: cast reint s_16_4 -> u32
        let s_16_5: u32 = (s_16_4.value() as u32);
        // D s_16_6: write-var address <= s_16_5
        fn_state.address = s_16_5;
        // N s_16_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var n:i64
        let s_17_0: i64 = fn_state.n;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call R_read(s_17_1)
        let s_17_2: u32 = R_read(state, tracer, s_17_1);
        // D s_17_3: write-var address <= s_17_2
        fn_state.address = s_17_2;
        // N s_17_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#323264 <= s_19_0
        fn_state.gs_323264 = s_19_0;
        // N s_19_2: jump b3
        return block_3(state, tracer, fn_state);
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
}
