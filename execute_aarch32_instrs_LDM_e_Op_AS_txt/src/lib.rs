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
use MemS_read::*;
use u__UNKNOWN_bits::*;
use R_read::*;
use R_set::*;
use u__id::*;
use AArch32_ExceptionReturn::*;
use BitCount::*;
use SPSR_read::*;
use common::*;
pub fn execute_aarch32_instrs_LDM_e_Op_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    increment_name: bool,
    n: i64,
    registers: u16,
    wback: bool,
    wordhigher: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_323221: bool,
        gs_323246: bool,
        address: u32,
        gs_323244: bool,
        ga_363953: u32,
        lengthshadow_7885: i128,
        gs_912354: Bits,
        gs_912359: Bits,
        i: i64,
        gs_323248: bool,
        new_pc_valueshadow_7886: u32,
        increment_name: bool,
        n: i64,
        registers: u16,
        wback: bool,
        wordhigher: bool,
    }
    let fn_state = FunctionState {
        increment_name,
        n,
        registers,
        wback,
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
        // N s_0_7: branch s_0_6 b39 b1
        if s_0_6 {
            return block_39(state, tracer, fn_state);
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
        // N s_1_7: branch s_1_6 b38 b2
        if s_1_6 {
            return block_38(state, tracer, fn_state);
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
        // D s_2_7: write-var gs#323221 <= s_2_6
        fn_state.gs_323221 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#323221:u8
        let s_3_0: bool = fn_state.gs_323221;
        // N s_3_1: branch s_3_0 b37 b4
        if s_3_0 {
            return block_37(state, tracer, fn_state);
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
        // C s_4_5: const #4s : i
        let s_4_5: i128 = 4;
        // D s_4_6: add s_4_4 s_4_5
        let s_4_6: i128 = (s_4_4 + s_4_5);
        // D s_4_7: write-var lengthshadow#7885 <= s_4_6
        fn_state.lengthshadow_7885 = s_4_6;
        // D s_4_8: read-var increment_name:u8
        let s_4_8: bool = fn_state.increment_name;
        // N s_4_9: branch s_4_8 b36 b5
        if s_4_8 {
            return block_36(state, tracer, fn_state);
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
        // D s_5_4: read-var lengthshadow#7885:i
        let s_5_4: i128 = fn_state.lengthshadow_7885;
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
        // N s_6_1: branch s_6_0 b35 b7
        if s_6_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_13_3: write-var gs#912354 <= s_13_2
        fn_state.gs_912354 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#912354:bv
        let s_14_0: Bits = fn_state.gs_912354;
        // D s_14_1: cast reint s_14_0 -> u32
        let s_14_1: u32 = (s_14_0.value() as u32);
        // D s_14_2: read-var i:i64
        let s_14_2: i64 = fn_state.i;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: call R_set(s_14_3, s_14_1)
        let s_14_4: () = R_set(state, tracer, s_14_3, s_14_1);
        // C s_14_5: const #4s : i
        let s_14_5: i128 = 4;
        // D s_14_6: read-var address:u32
        let s_14_6: u32 = fn_state.address;
        // D s_14_7: cast zx s_14_6 -> bv
        let s_14_7: Bits = Bits::new(s_14_6 as u128, 32u16);
        // C s_14_8: cast cvt s_14_5 -> bv
        let s_14_8: Bits = Bits::new(s_14_5 as u128, 128);
        // D s_14_9: add s_14_7 s_14_8
        let s_14_9: Bits = (s_14_7 + s_14_8);
        // D s_14_10: cast reint s_14_9 -> u32
        let s_14_10: u32 = (s_14_9.value() as u32);
        // D s_14_11: write-var address <= s_14_10
        fn_state.address = s_14_10;
        // N s_14_12: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #4s : i64
        let s_15_0: i64 = 4;
        // D s_15_1: read-var address:u32
        let s_15_1: u32 = fn_state.address;
        // D s_15_2: call MemS_read(s_15_1, s_15_0)
        let s_15_2: Bits = MemS_read(state, tracer, s_15_1, s_15_0);
        // D s_15_3: write-var gs#912359 <= s_15_2
        fn_state.gs_912359 = s_15_2;
        // N s_15_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#912359:bv
        let s_16_0: Bits = fn_state.gs_912359;
        // D s_16_1: cast reint s_16_0 -> u32
        let s_16_1: u32 = (s_16_0.value() as u32);
        // D s_16_2: write-var new_pc_valueshadow#7886 <= s_16_1
        fn_state.new_pc_valueshadow_7886 = s_16_1;
        // D s_16_3: read-var n:i64
        let s_16_3: i64 = fn_state.n;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: call __id(s_16_4)
        let s_16_5: i128 = u__id(state, tracer, s_16_4);
        // D s_16_6: cast reint s_16_5 -> i64
        let s_16_6: i64 = (s_16_5 as i64);
        // C s_16_7: const #0s : i
        let s_16_7: i128 = 0;
        // D s_16_8: cast zx s_16_6 -> i
        let s_16_8: i128 = (i128::try_from(s_16_6).unwrap());
        // D s_16_9: cmp-le s_16_7 s_16_8
        let s_16_9: bool = ((s_16_7) <= (s_16_8));
        // N s_16_10: branch s_16_9 b34 b17
        if s_16_9 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#323244 <= s_17_0
        fn_state.gs_323244 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#323244:u8
        let s_18_0: bool = fn_state.gs_323244;
        // N s_18_1: assert s_18_0
        let s_18_1: () = assert!(s_18_0);
        // D s_18_2: read-var wback:u8
        let s_18_2: bool = fn_state.wback;
        // N s_18_3: branch s_18_2 b33 b19
        if s_18_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#323246 <= s_19_0
        fn_state.gs_323246 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#323246:u8
        let s_20_0: bool = fn_state.gs_323246;
        // N s_20_1: branch s_20_0 b29 b21
        if s_20_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var wback:u8
        let s_22_0: bool = fn_state.wback;
        // N s_22_1: branch s_22_0 b28 b23
        if s_22_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#323248 <= s_23_0
        fn_state.gs_323248 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#323248:u8
        let s_24_0: bool = fn_state.gs_323248;
        // N s_24_1: branch s_24_0 b27 b25
        if s_24_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #32s : i
        let s_26_0: i128 = 32;
        // S s_26_1: call SPSR_read(s_26_0)
        let s_26_1: Bits = SPSR_read(state, tracer, s_26_0);
        // S s_26_2: cast reint s_26_1 -> u32
        let s_26_2: u32 = (s_26_1.value() as u32);
        // D s_26_3: read-var new_pc_valueshadow#7886:u32
        let s_26_3: u32 = fn_state.new_pc_valueshadow_7886;
        // D s_26_4: call AArch32_ExceptionReturn(s_26_3, s_26_2)
        let s_26_4: () = AArch32_ExceptionReturn(state, tracer, s_26_3, s_26_2);
        // N s_26_5: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #32s : i64
        let s_27_0: i64 = 32;
        // C s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // S s_27_2: call __UNKNOWN_bits(s_27_1)
        let s_27_2: Bits = u__UNKNOWN_bits(state, tracer, s_27_1);
        // S s_27_3: cast reint s_27_2 -> u32
        let s_27_3: u32 = (s_27_2.value() as u32);
        // D s_27_4: read-var n:i64
        let s_27_4: i64 = fn_state.n;
        // D s_27_5: cast zx s_27_4 -> i
        let s_27_5: i128 = (i128::try_from(s_27_4).unwrap());
        // D s_27_6: call R_set(s_27_5, s_27_3)
        let s_27_6: () = R_set(state, tracer, s_27_5, s_27_3);
        // N s_27_7: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var registers:u15
        let s_28_0: u16 = fn_state.registers;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 15u16);
        // D s_28_2: read-var n:i64
        let s_28_2: i64 = fn_state.n;
        // D s_28_3: cast zx s_28_2 -> i
        let s_28_3: i128 = (i128::try_from(s_28_2).unwrap());
        // C s_28_4: const #1u : u64
        let s_28_4: u64 = 1;
        // D s_28_5: bit-extract s_28_1 s_28_3 s_28_4
        let s_28_5: Bits = (Bits::new(
            ((s_28_1) >> (s_28_3)).value(),
            u16::try_from(s_28_4).unwrap(),
        ));
        // D s_28_6: cast reint s_28_5 -> u8
        let s_28_6: bool = ((s_28_5.value()) != 0);
        // C s_28_7: const #0s : i
        let s_28_7: i128 = 0;
        // C s_28_8: const #0u : u64
        let s_28_8: u64 = 0;
        // D s_28_9: cast zx s_28_6 -> u64
        let s_28_9: u64 = (s_28_6 as u64);
        // C s_28_10: const #1u : u64
        let s_28_10: u64 = 1;
        // D s_28_11: and s_28_9 s_28_10
        let s_28_11: u64 = ((s_28_9) & (s_28_10));
        // D s_28_12: cmp-eq s_28_11 s_28_10
        let s_28_12: bool = ((s_28_11) == (s_28_10));
        // D s_28_13: lsl s_28_9 s_28_7
        let s_28_13: u64 = s_28_9 << s_28_7;
        // D s_28_14: or s_28_8 s_28_13
        let s_28_14: u64 = ((s_28_8) | (s_28_13));
        // D s_28_15: cmpl s_28_13
        let s_28_15: u64 = !s_28_13;
        // D s_28_16: and s_28_8 s_28_15
        let s_28_16: u64 = ((s_28_8) & (s_28_15));
        // D s_28_17: select s_28_12 s_28_14 s_28_16
        let s_28_17: u64 = if s_28_12 { s_28_14 } else { s_28_16 };
        // D s_28_18: cast trunc s_28_17 -> u8
        let s_28_18: bool = ((s_28_17) != 0);
        // D s_28_19: cast zx s_28_18 -> bv
        let s_28_19: Bits = Bits::new(s_28_18 as u128, 1u16);
        // C s_28_20: const #1u : u8
        let s_28_20: bool = true;
        // C s_28_21: cast zx s_28_20 -> bv
        let s_28_21: Bits = Bits::new(s_28_20 as u128, 1u16);
        // D s_28_22: cmp-eq s_28_19 s_28_21
        let s_28_22: bool = ((s_28_19) == (s_28_21));
        // D s_28_23: write-var gs#323248 <= s_28_22
        fn_state.gs_323248 = s_28_22;
        // N s_28_24: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var increment_name:u8
        let s_29_0: bool = fn_state.increment_name;
        // N s_29_1: branch s_29_0 b32 b30
        if s_29_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var n:i64
        let s_30_0: i64 = fn_state.n;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: call R_read(s_30_1)
        let s_30_2: u32 = R_read(state, tracer, s_30_1);
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 32u16);
        // D s_30_4: read-var lengthshadow#7885:i
        let s_30_4: i128 = fn_state.lengthshadow_7885;
        // D s_30_5: cast cvt s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 128);
        // D s_30_6: sub s_30_3 s_30_5
        let s_30_6: Bits = ((s_30_3) - (s_30_5));
        // D s_30_7: cast reint s_30_6 -> u32
        let s_30_7: u32 = (s_30_6.value() as u32);
        // D s_30_8: write-var ga#363953 <= s_30_7
        fn_state.ga_363953 = s_30_7;
        // N s_30_9: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var n:i64
        let s_31_0: i64 = fn_state.n;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: read-var ga#363953:u32
        let s_31_2: u32 = fn_state.ga_363953;
        // D s_31_3: call R_set(s_31_1, s_31_2)
        let s_31_3: () = R_set(state, tracer, s_31_1, s_31_2);
        // N s_31_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var n:i64
        let s_32_0: i64 = fn_state.n;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: call R_read(s_32_1)
        let s_32_2: u32 = R_read(state, tracer, s_32_1);
        // D s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 32u16);
        // D s_32_4: read-var lengthshadow#7885:i
        let s_32_4: i128 = fn_state.lengthshadow_7885;
        // D s_32_5: cast cvt s_32_4 -> bv
        let s_32_5: Bits = Bits::new(s_32_4 as u128, 128);
        // D s_32_6: add s_32_3 s_32_5
        let s_32_6: Bits = (s_32_3 + s_32_5);
        // D s_32_7: cast reint s_32_6 -> u32
        let s_32_7: u32 = (s_32_6.value() as u32);
        // D s_32_8: write-var ga#363953 <= s_32_7
        fn_state.ga_363953 = s_32_7;
        // N s_32_9: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var registers:u15
        let s_33_0: u16 = fn_state.registers;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 15u16);
        // D s_33_2: read-var n:i64
        let s_33_2: i64 = fn_state.n;
        // D s_33_3: cast zx s_33_2 -> i
        let s_33_3: i128 = (i128::try_from(s_33_2).unwrap());
        // C s_33_4: const #1u : u64
        let s_33_4: u64 = 1;
        // D s_33_5: bit-extract s_33_1 s_33_3 s_33_4
        let s_33_5: Bits = (Bits::new(
            ((s_33_1) >> (s_33_3)).value(),
            u16::try_from(s_33_4).unwrap(),
        ));
        // D s_33_6: cast reint s_33_5 -> u8
        let s_33_6: bool = ((s_33_5.value()) != 0);
        // C s_33_7: const #0s : i
        let s_33_7: i128 = 0;
        // C s_33_8: const #0u : u64
        let s_33_8: u64 = 0;
        // D s_33_9: cast zx s_33_6 -> u64
        let s_33_9: u64 = (s_33_6 as u64);
        // C s_33_10: const #1u : u64
        let s_33_10: u64 = 1;
        // D s_33_11: and s_33_9 s_33_10
        let s_33_11: u64 = ((s_33_9) & (s_33_10));
        // D s_33_12: cmp-eq s_33_11 s_33_10
        let s_33_12: bool = ((s_33_11) == (s_33_10));
        // D s_33_13: lsl s_33_9 s_33_7
        let s_33_13: u64 = s_33_9 << s_33_7;
        // D s_33_14: or s_33_8 s_33_13
        let s_33_14: u64 = ((s_33_8) | (s_33_13));
        // D s_33_15: cmpl s_33_13
        let s_33_15: u64 = !s_33_13;
        // D s_33_16: and s_33_8 s_33_15
        let s_33_16: u64 = ((s_33_8) & (s_33_15));
        // D s_33_17: select s_33_12 s_33_14 s_33_16
        let s_33_17: u64 = if s_33_12 { s_33_14 } else { s_33_16 };
        // D s_33_18: cast trunc s_33_17 -> u8
        let s_33_18: bool = ((s_33_17) != 0);
        // D s_33_19: cast zx s_33_18 -> bv
        let s_33_19: Bits = Bits::new(s_33_18 as u128, 1u16);
        // C s_33_20: const #0u : u8
        let s_33_20: bool = false;
        // C s_33_21: cast zx s_33_20 -> bv
        let s_33_21: Bits = Bits::new(s_33_20 as u128, 1u16);
        // D s_33_22: cmp-eq s_33_19 s_33_21
        let s_33_22: bool = ((s_33_19) == (s_33_21));
        // D s_33_23: write-var gs#323246 <= s_33_22
        fn_state.gs_323246 = s_33_22;
        // N s_33_24: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var n:i64
        let s_34_0: i64 = fn_state.n;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: call __id(s_34_1)
        let s_34_2: i128 = u__id(state, tracer, s_34_1);
        // D s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: const #15s : i
        let s_34_4: i128 = 15;
        // D s_34_5: cast zx s_34_3 -> i
        let s_34_5: i128 = (i128::try_from(s_34_3).unwrap());
        // D s_34_6: cmp-lt s_34_5 s_34_4
        let s_34_6: bool = ((s_34_5) < (s_34_4));
        // D s_34_7: write-var gs#323244 <= s_34_6
        fn_state.gs_323244 = s_34_6;
        // N s_34_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #4s : i
        let s_35_0: i128 = 4;
        // D s_35_1: read-var address:u32
        let s_35_1: u32 = fn_state.address;
        // D s_35_2: cast zx s_35_1 -> bv
        let s_35_2: Bits = Bits::new(s_35_1 as u128, 32u16);
        // C s_35_3: cast cvt s_35_0 -> bv
        let s_35_3: Bits = Bits::new(s_35_0 as u128, 128);
        // D s_35_4: add s_35_2 s_35_3
        let s_35_4: Bits = (s_35_2 + s_35_3);
        // D s_35_5: cast reint s_35_4 -> u32
        let s_35_5: u32 = (s_35_4.value() as u32);
        // D s_35_6: write-var address <= s_35_5
        fn_state.address = s_35_5;
        // N s_35_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var n:i64
        let s_36_0: i64 = fn_state.n;
        // D s_36_1: cast zx s_36_0 -> i
        let s_36_1: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_2: call R_read(s_36_1)
        let s_36_2: u32 = R_read(state, tracer, s_36_1);
        // D s_36_3: write-var address <= s_36_2
        fn_state.address = s_36_2;
        // N s_36_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: panic
        panic!("{:?}", ());
        // N s_37_1: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#323221 <= s_38_0
        fn_state.gs_323221 = s_38_0;
        // N s_38_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: panic
        panic!("{:?}", ());
        // N s_39_1: return
        return;
    }
}
