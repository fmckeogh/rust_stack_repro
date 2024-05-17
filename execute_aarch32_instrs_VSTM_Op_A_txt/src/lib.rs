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
use CheckVFPEnabled::*;
use R_read::*;
use R_set::*;
use BigEndian::*;
use MemA_set::*;
use D_read::*;
use S_read::*;
use common::*;
pub fn execute_aarch32_instrs_VSTM_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    d: i64,
    imm32: u32,
    n: i64,
    regs: i128,
    single_regs: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_322129: i64,
        ga_362909: u32,
        address: u32,
        add: bool,
        d: i64,
        imm32: u32,
        n: i64,
        regs: i128,
        single_regs: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        add,
        d,
        imm32,
        n,
        regs,
        single_regs,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var add:u8
        let s_1_0: bool = fn_state.add;
        // N s_1_1: branch s_1_0 b21 b2
        if s_1_0 {
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
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call R_read(s_2_1)
        let s_2_2: u32 = R_read(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 32u16);
        // D s_2_4: read-var imm32:u32
        let s_2_4: u32 = fn_state.imm32;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 32u16);
        // D s_2_6: sub s_2_3 s_2_5
        let s_2_6: Bits = ((s_2_3) - (s_2_5));
        // D s_2_7: cast reint s_2_6 -> u32
        let s_2_7: u32 = (s_2_6.value() as u32);
        // D s_2_8: write-var address <= s_2_7
        fn_state.address = s_2_7;
        // N s_2_9: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var regs:i
        let s_3_2: i128 = fn_state.regs;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#322129 <= s_3_4
        fn_state.gs_322129 = s_3_4;
        // D s_3_6: write-var r <= s_3_0
        fn_state.r = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:i64
        let s_4_0: i64 = fn_state.r;
        // D s_4_1: read-var gs#322129:i64
        let s_4_1: i64 = fn_state.gs_322129;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b15 b5
        if s_4_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var single_regs:u8
        let s_5_0: bool = fn_state.single_regs;
        // N s_5_1: branch s_5_0 b13 b6
        if s_5_0 {
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
        // C s_6_0: const #2u : u32
        let s_6_0: u32 = 2;
        // S s_6_1: call BigEndian(s_6_0)
        let s_6_1: bool = BigEndian(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b11 b7
        if s_6_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var d:i64
        let s_7_0: i64 = fn_state.d;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var r:i64
        let s_7_2: i64 = fn_state.r;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: i128 = (s_7_1 + s_7_3);
        // D s_7_5: call D_read(s_7_4)
        let s_7_5: u64 = D_read(state, tracer, s_7_4);
        // C s_7_6: const #0s : i
        let s_7_6: i128 = 0;
        // D s_7_7: cast zx s_7_5 -> bv
        let s_7_7: Bits = Bits::new(s_7_5 as u128, 64u16);
        // C s_7_8: const #1s : i64
        let s_7_8: i64 = 1;
        // C s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // C s_7_10: const #31s : i
        let s_7_10: i128 = 31;
        // C s_7_11: add s_7_10 s_7_9
        let s_7_11: i128 = (s_7_10 + s_7_9);
        // D s_7_12: bit-extract s_7_7 s_7_6 s_7_11
        let s_7_12: Bits = (Bits::new(
            ((s_7_7) >> (s_7_6)).value(),
            u16::try_from(s_7_11).unwrap(),
        ));
        // D s_7_13: cast reint s_7_12 -> u32
        let s_7_13: u32 = (s_7_12.value() as u32);
        // C s_7_14: const #4s : i
        let s_7_14: i128 = 4;
        // D s_7_15: cast zx s_7_13 -> bv
        let s_7_15: Bits = Bits::new(s_7_13 as u128, 32u16);
        // D s_7_16: read-var address:u32
        let s_7_16: u32 = fn_state.address;
        // D s_7_17: call MemA_set(s_7_16, s_7_14, s_7_15)
        let s_7_17: () = MemA_set(state, tracer, s_7_16, s_7_14, s_7_15);
        // N s_7_18: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #4s : i
        let s_8_0: i128 = 4;
        // D s_8_1: read-var address:u32
        let s_8_1: u32 = fn_state.address;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 32u16);
        // C s_8_3: cast cvt s_8_0 -> bv
        let s_8_3: Bits = Bits::new(s_8_0 as u128, 128);
        // D s_8_4: add s_8_2 s_8_3
        let s_8_4: Bits = (s_8_2 + s_8_3);
        // D s_8_5: cast reint s_8_4 -> u32
        let s_8_5: u32 = (s_8_4.value() as u32);
        // D s_8_6: read-var d:i64
        let s_8_6: i64 = fn_state.d;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: read-var r:i64
        let s_8_8: i64 = fn_state.r;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: add s_8_7 s_8_9
        let s_8_10: i128 = (s_8_7 + s_8_9);
        // D s_8_11: call D_read(s_8_10)
        let s_8_11: u64 = D_read(state, tracer, s_8_10);
        // C s_8_12: const #32s : i
        let s_8_12: i128 = 32;
        // D s_8_13: cast zx s_8_11 -> bv
        let s_8_13: Bits = Bits::new(s_8_11 as u128, 64u16);
        // C s_8_14: const #1s : i64
        let s_8_14: i64 = 1;
        // C s_8_15: cast zx s_8_14 -> i
        let s_8_15: i128 = (i128::try_from(s_8_14).unwrap());
        // C s_8_16: const #31s : i
        let s_8_16: i128 = 31;
        // C s_8_17: add s_8_16 s_8_15
        let s_8_17: i128 = (s_8_16 + s_8_15);
        // D s_8_18: bit-extract s_8_13 s_8_12 s_8_17
        let s_8_18: Bits = (Bits::new(
            ((s_8_13) >> (s_8_12)).value(),
            u16::try_from(s_8_17).unwrap(),
        ));
        // D s_8_19: cast reint s_8_18 -> u32
        let s_8_19: u32 = (s_8_18.value() as u32);
        // C s_8_20: const #4s : i
        let s_8_20: i128 = 4;
        // D s_8_21: cast zx s_8_19 -> bv
        let s_8_21: Bits = Bits::new(s_8_19 as u128, 32u16);
        // D s_8_22: call MemA_set(s_8_5, s_8_20, s_8_21)
        let s_8_22: () = MemA_set(state, tracer, s_8_5, s_8_20, s_8_21);
        // N s_8_23: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #8s : i
        let s_9_0: i128 = 8;
        // D s_9_1: read-var address:u32
        let s_9_1: u32 = fn_state.address;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 32u16);
        // C s_9_3: cast cvt s_9_0 -> bv
        let s_9_3: Bits = Bits::new(s_9_0 as u128, 128);
        // D s_9_4: add s_9_2 s_9_3
        let s_9_4: Bits = (s_9_2 + s_9_3);
        // D s_9_5: cast reint s_9_4 -> u32
        let s_9_5: u32 = (s_9_4.value() as u32);
        // D s_9_6: write-var address <= s_9_5
        fn_state.address = s_9_5;
        // N s_9_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var r:i64
        let s_10_0: i64 = fn_state.r;
        // C s_10_1: const #1s : i64
        let s_10_1: i64 = 1;
        // D s_10_2: add s_10_0 s_10_1
        let s_10_2: i64 = (s_10_0 + s_10_1);
        // D s_10_3: write-var r <= s_10_2
        fn_state.r = s_10_2;
        // N s_10_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var d:i64
        let s_11_0: i64 = fn_state.d;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: read-var r:i64
        let s_11_2: i64 = fn_state.r;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: add s_11_1 s_11_3
        let s_11_4: i128 = (s_11_1 + s_11_3);
        // D s_11_5: call D_read(s_11_4)
        let s_11_5: u64 = D_read(state, tracer, s_11_4);
        // C s_11_6: const #32s : i
        let s_11_6: i128 = 32;
        // D s_11_7: cast zx s_11_5 -> bv
        let s_11_7: Bits = Bits::new(s_11_5 as u128, 64u16);
        // C s_11_8: const #1s : i64
        let s_11_8: i64 = 1;
        // C s_11_9: cast zx s_11_8 -> i
        let s_11_9: i128 = (i128::try_from(s_11_8).unwrap());
        // C s_11_10: const #31s : i
        let s_11_10: i128 = 31;
        // C s_11_11: add s_11_10 s_11_9
        let s_11_11: i128 = (s_11_10 + s_11_9);
        // D s_11_12: bit-extract s_11_7 s_11_6 s_11_11
        let s_11_12: Bits = (Bits::new(
            ((s_11_7) >> (s_11_6)).value(),
            u16::try_from(s_11_11).unwrap(),
        ));
        // D s_11_13: cast reint s_11_12 -> u32
        let s_11_13: u32 = (s_11_12.value() as u32);
        // C s_11_14: const #4s : i
        let s_11_14: i128 = 4;
        // D s_11_15: cast zx s_11_13 -> bv
        let s_11_15: Bits = Bits::new(s_11_13 as u128, 32u16);
        // D s_11_16: read-var address:u32
        let s_11_16: u32 = fn_state.address;
        // D s_11_17: call MemA_set(s_11_16, s_11_14, s_11_15)
        let s_11_17: () = MemA_set(state, tracer, s_11_16, s_11_14, s_11_15);
        // N s_11_18: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #4s : i
        let s_12_0: i128 = 4;
        // D s_12_1: read-var address:u32
        let s_12_1: u32 = fn_state.address;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 32u16);
        // C s_12_3: cast cvt s_12_0 -> bv
        let s_12_3: Bits = Bits::new(s_12_0 as u128, 128);
        // D s_12_4: add s_12_2 s_12_3
        let s_12_4: Bits = (s_12_2 + s_12_3);
        // D s_12_5: cast reint s_12_4 -> u32
        let s_12_5: u32 = (s_12_4.value() as u32);
        // D s_12_6: read-var d:i64
        let s_12_6: i64 = fn_state.d;
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: read-var r:i64
        let s_12_8: i64 = fn_state.r;
        // D s_12_9: cast zx s_12_8 -> i
        let s_12_9: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_10: add s_12_7 s_12_9
        let s_12_10: i128 = (s_12_7 + s_12_9);
        // D s_12_11: call D_read(s_12_10)
        let s_12_11: u64 = D_read(state, tracer, s_12_10);
        // C s_12_12: const #0s : i
        let s_12_12: i128 = 0;
        // D s_12_13: cast zx s_12_11 -> bv
        let s_12_13: Bits = Bits::new(s_12_11 as u128, 64u16);
        // C s_12_14: const #1s : i64
        let s_12_14: i64 = 1;
        // C s_12_15: cast zx s_12_14 -> i
        let s_12_15: i128 = (i128::try_from(s_12_14).unwrap());
        // C s_12_16: const #31s : i
        let s_12_16: i128 = 31;
        // C s_12_17: add s_12_16 s_12_15
        let s_12_17: i128 = (s_12_16 + s_12_15);
        // D s_12_18: bit-extract s_12_13 s_12_12 s_12_17
        let s_12_18: Bits = (Bits::new(
            ((s_12_13) >> (s_12_12)).value(),
            u16::try_from(s_12_17).unwrap(),
        ));
        // D s_12_19: cast reint s_12_18 -> u32
        let s_12_19: u32 = (s_12_18.value() as u32);
        // C s_12_20: const #4s : i
        let s_12_20: i128 = 4;
        // D s_12_21: cast zx s_12_19 -> bv
        let s_12_21: Bits = Bits::new(s_12_19 as u128, 32u16);
        // D s_12_22: call MemA_set(s_12_5, s_12_20, s_12_21)
        let s_12_22: () = MemA_set(state, tracer, s_12_5, s_12_20, s_12_21);
        // N s_12_23: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var d:i64
        let s_13_0: i64 = fn_state.d;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var r:i64
        let s_13_2: i64 = fn_state.r;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: add s_13_1 s_13_3
        let s_13_4: i128 = (s_13_1 + s_13_3);
        // D s_13_5: call S_read(s_13_4)
        let s_13_5: u32 = S_read(state, tracer, s_13_4);
        // C s_13_6: const #4s : i
        let s_13_6: i128 = 4;
        // D s_13_7: cast zx s_13_5 -> bv
        let s_13_7: Bits = Bits::new(s_13_5 as u128, 32u16);
        // D s_13_8: read-var address:u32
        let s_13_8: u32 = fn_state.address;
        // D s_13_9: call MemA_set(s_13_8, s_13_6, s_13_7)
        let s_13_9: () = MemA_set(state, tracer, s_13_8, s_13_6, s_13_7);
        // N s_13_10: jump b14
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
        // N s_14_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var wback:u8
        let s_15_0: bool = fn_state.wback;
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
        // D s_17_0: read-var add:u8
        let s_17_0: bool = fn_state.add;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call R_read(s_18_1)
        let s_18_2: u32 = R_read(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 32u16);
        // D s_18_4: read-var imm32:u32
        let s_18_4: u32 = fn_state.imm32;
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 32u16);
        // D s_18_6: sub s_18_3 s_18_5
        let s_18_6: Bits = ((s_18_3) - (s_18_5));
        // D s_18_7: cast reint s_18_6 -> u32
        let s_18_7: u32 = (s_18_6.value() as u32);
        // D s_18_8: write-var ga#362909 <= s_18_7
        fn_state.ga_362909 = s_18_7;
        // N s_18_9: jump b19
        return block_19(state, tracer, fn_state);
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
        // D s_19_2: read-var ga#362909:u32
        let s_19_2: u32 = fn_state.ga_362909;
        // D s_19_3: call R_set(s_19_1, s_19_2)
        let s_19_3: () = R_set(state, tracer, s_19_1, s_19_2);
        // N s_19_4: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var n:i64
        let s_20_0: i64 = fn_state.n;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call R_read(s_20_1)
        let s_20_2: u32 = R_read(state, tracer, s_20_1);
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 32u16);
        // D s_20_4: read-var imm32:u32
        let s_20_4: u32 = fn_state.imm32;
        // D s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 32u16);
        // D s_20_6: add s_20_3 s_20_5
        let s_20_6: Bits = (s_20_3 + s_20_5);
        // D s_20_7: cast reint s_20_6 -> u32
        let s_20_7: u32 = (s_20_6.value() as u32);
        // D s_20_8: write-var ga#362909 <= s_20_7
        fn_state.ga_362909 = s_20_7;
        // N s_20_9: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var n:i64
        let s_21_0: i64 = fn_state.n;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call R_read(s_21_1)
        let s_21_2: u32 = R_read(state, tracer, s_21_1);
        // D s_21_3: write-var address <= s_21_2
        fn_state.address = s_21_2;
        // N s_21_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
