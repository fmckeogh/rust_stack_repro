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
use BigEndian::*;
use IsAligned__1::*;
use MemA_read::*;
use R_set::*;
use PC_read__1::*;
use Align_bits::*;
use common::*;
pub fn execute_aarch32_instrs_LDRD_l_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    imm32: u32,
    t: i64,
    t2: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_879779: Bits,
        gs_879784: Bits,
        address: u32,
        datashadow_7166: u64,
        gs_879787: Bits,
        add: bool,
        imm32: u32,
        t: i64,
        t2: i64,
    }
    let fn_state = FunctionState {
        add,
        imm32,
        t,
        t2,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var add:u8
        let s_0_0: bool = fn_state.add;
        // N s_0_1: branch s_0_0 b10 b1
        if s_0_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call PC_read__1(s_1_0)
        let s_1_1: u32 = PC_read__1(state, tracer, s_1_0);
        // C s_1_2: const #4s : i
        let s_1_2: i128 = 4;
        // S s_1_3: cast zx s_1_1 -> bv
        let s_1_3: Bits = Bits::new(s_1_1 as u128, 32u16);
        // S s_1_4: call Align_bits(s_1_3, s_1_2)
        let s_1_4: Bits = Align_bits(state, tracer, s_1_3, s_1_2);
        // S s_1_5: cast reint s_1_4 -> u32
        let s_1_5: u32 = (s_1_4.value() as u32);
        // S s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 32u16);
        // D s_1_7: read-var imm32:u32
        let s_1_7: u32 = fn_state.imm32;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 32u16);
        // D s_1_9: sub s_1_6 s_1_8
        let s_1_9: Bits = ((s_1_6) - (s_1_8));
        // D s_1_10: cast reint s_1_9 -> u32
        let s_1_10: u32 = (s_1_9.value() as u32);
        // D s_1_11: write-var address <= s_1_10
        fn_state.address = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #8s : i
        let s_2_0: i128 = 8;
        // D s_2_1: read-var address:u32
        let s_2_1: u32 = fn_state.address;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 32u16);
        // D s_2_3: call IsAligned__1(s_2_2, s_2_0)
        let s_2_3: bool = IsAligned__1(state, tracer, s_2_2, s_2_0);
        // N s_2_4: branch s_2_3 b6 b3
        if s_2_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #4s : i64
        let s_3_0: i64 = 4;
        // D s_3_1: read-var address:u32
        let s_3_1: u32 = fn_state.address;
        // D s_3_2: call MemA_read(s_3_1, s_3_0)
        let s_3_2: Bits = MemA_read(state, tracer, s_3_1, s_3_0);
        // D s_3_3: write-var gs#879779 <= s_3_2
        fn_state.gs_879779 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#879779:bv
        let s_4_0: Bits = fn_state.gs_879779;
        // D s_4_1: cast reint s_4_0 -> u32
        let s_4_1: u32 = (s_4_0.value() as u32);
        // D s_4_2: read-var t:i64
        let s_4_2: i64 = fn_state.t;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: call R_set(s_4_3, s_4_1)
        let s_4_4: () = R_set(state, tracer, s_4_3, s_4_1);
        // C s_4_5: const #4s : i
        let s_4_5: i128 = 4;
        // D s_4_6: read-var address:u32
        let s_4_6: u32 = fn_state.address;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 32u16);
        // C s_4_8: cast cvt s_4_5 -> bv
        let s_4_8: Bits = Bits::new(s_4_5 as u128, 128);
        // D s_4_9: add s_4_7 s_4_8
        let s_4_9: Bits = (s_4_7 + s_4_8);
        // D s_4_10: cast reint s_4_9 -> u32
        let s_4_10: u32 = (s_4_9.value() as u32);
        // C s_4_11: const #4s : i64
        let s_4_11: i64 = 4;
        // D s_4_12: call MemA_read(s_4_10, s_4_11)
        let s_4_12: Bits = MemA_read(state, tracer, s_4_10, s_4_11);
        // D s_4_13: write-var gs#879784 <= s_4_12
        fn_state.gs_879784 = s_4_12;
        // N s_4_14: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#879784:bv
        let s_5_0: Bits = fn_state.gs_879784;
        // D s_5_1: cast reint s_5_0 -> u32
        let s_5_1: u32 = (s_5_0.value() as u32);
        // D s_5_2: read-var t2:i64
        let s_5_2: i64 = fn_state.t2;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: call R_set(s_5_3, s_5_1)
        let s_5_4: () = R_set(state, tracer, s_5_3, s_5_1);
        // N s_5_5: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #8s : i64
        let s_6_0: i64 = 8;
        // D s_6_1: read-var address:u32
        let s_6_1: u32 = fn_state.address;
        // D s_6_2: call MemA_read(s_6_1, s_6_0)
        let s_6_2: Bits = MemA_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: write-var gs#879787 <= s_6_2
        fn_state.gs_879787 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#879787:bv
        let s_7_0: Bits = fn_state.gs_879787;
        // D s_7_1: cast reint s_7_0 -> u64
        let s_7_1: u64 = (s_7_0.value() as u64);
        // D s_7_2: write-var datashadow#7166 <= s_7_1
        fn_state.datashadow_7166 = s_7_1;
        // C s_7_3: const #1u : u32
        let s_7_3: u32 = 1;
        // S s_7_4: call BigEndian(s_7_3)
        let s_7_4: bool = BigEndian(state, tracer, s_7_3);
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: read-var datashadow#7166:u64
        let s_8_1: u64 = fn_state.datashadow_7166;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 64u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #31s : i
        let s_8_5: i128 = 31;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_0 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u32
        let s_8_8: u32 = (s_8_7.value() as u32);
        // D s_8_9: read-var t:i64
        let s_8_9: i64 = fn_state.t;
        // D s_8_10: cast zx s_8_9 -> i
        let s_8_10: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_11: call R_set(s_8_10, s_8_8)
        let s_8_11: () = R_set(state, tracer, s_8_10, s_8_8);
        // C s_8_12: const #32s : i
        let s_8_12: i128 = 32;
        // D s_8_13: read-var datashadow#7166:u64
        let s_8_13: u64 = fn_state.datashadow_7166;
        // D s_8_14: cast zx s_8_13 -> bv
        let s_8_14: Bits = Bits::new(s_8_13 as u128, 64u16);
        // C s_8_15: const #1s : i64
        let s_8_15: i64 = 1;
        // C s_8_16: cast zx s_8_15 -> i
        let s_8_16: i128 = (i128::try_from(s_8_15).unwrap());
        // C s_8_17: const #31s : i
        let s_8_17: i128 = 31;
        // C s_8_18: add s_8_17 s_8_16
        let s_8_18: i128 = (s_8_17 + s_8_16);
        // D s_8_19: bit-extract s_8_14 s_8_12 s_8_18
        let s_8_19: Bits = (Bits::new(
            ((s_8_14) >> (s_8_12)).value(),
            u16::try_from(s_8_18).unwrap(),
        ));
        // D s_8_20: cast reint s_8_19 -> u32
        let s_8_20: u32 = (s_8_19.value() as u32);
        // D s_8_21: read-var t2:i64
        let s_8_21: i64 = fn_state.t2;
        // D s_8_22: cast zx s_8_21 -> i
        let s_8_22: i128 = (i128::try_from(s_8_21).unwrap());
        // D s_8_23: call R_set(s_8_22, s_8_20)
        let s_8_23: () = R_set(state, tracer, s_8_22, s_8_20);
        // N s_8_24: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #32s : i
        let s_9_0: i128 = 32;
        // D s_9_1: read-var datashadow#7166:u64
        let s_9_1: u64 = fn_state.datashadow_7166;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 64u16);
        // C s_9_3: const #1s : i64
        let s_9_3: i64 = 1;
        // C s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // C s_9_5: const #31s : i
        let s_9_5: i128 = 31;
        // C s_9_6: add s_9_5 s_9_4
        let s_9_6: i128 = (s_9_5 + s_9_4);
        // D s_9_7: bit-extract s_9_2 s_9_0 s_9_6
        let s_9_7: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_6).unwrap(),
        ));
        // D s_9_8: cast reint s_9_7 -> u32
        let s_9_8: u32 = (s_9_7.value() as u32);
        // D s_9_9: read-var t:i64
        let s_9_9: i64 = fn_state.t;
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_11: call R_set(s_9_10, s_9_8)
        let s_9_11: () = R_set(state, tracer, s_9_10, s_9_8);
        // C s_9_12: const #0s : i
        let s_9_12: i128 = 0;
        // D s_9_13: read-var datashadow#7166:u64
        let s_9_13: u64 = fn_state.datashadow_7166;
        // D s_9_14: cast zx s_9_13 -> bv
        let s_9_14: Bits = Bits::new(s_9_13 as u128, 64u16);
        // C s_9_15: const #1s : i64
        let s_9_15: i64 = 1;
        // C s_9_16: cast zx s_9_15 -> i
        let s_9_16: i128 = (i128::try_from(s_9_15).unwrap());
        // C s_9_17: const #31s : i
        let s_9_17: i128 = 31;
        // C s_9_18: add s_9_17 s_9_16
        let s_9_18: i128 = (s_9_17 + s_9_16);
        // D s_9_19: bit-extract s_9_14 s_9_12 s_9_18
        let s_9_19: Bits = (Bits::new(
            ((s_9_14) >> (s_9_12)).value(),
            u16::try_from(s_9_18).unwrap(),
        ));
        // D s_9_20: cast reint s_9_19 -> u32
        let s_9_20: u32 = (s_9_19.value() as u32);
        // D s_9_21: read-var t2:i64
        let s_9_21: i64 = fn_state.t2;
        // D s_9_22: cast zx s_9_21 -> i
        let s_9_22: i128 = (i128::try_from(s_9_21).unwrap());
        // D s_9_23: call R_set(s_9_22, s_9_20)
        let s_9_23: () = R_set(state, tracer, s_9_22, s_9_20);
        // N s_9_24: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call PC_read__1(s_10_0)
        let s_10_1: u32 = PC_read__1(state, tracer, s_10_0);
        // C s_10_2: const #4s : i
        let s_10_2: i128 = 4;
        // S s_10_3: cast zx s_10_1 -> bv
        let s_10_3: Bits = Bits::new(s_10_1 as u128, 32u16);
        // S s_10_4: call Align_bits(s_10_3, s_10_2)
        let s_10_4: Bits = Align_bits(state, tracer, s_10_3, s_10_2);
        // S s_10_5: cast reint s_10_4 -> u32
        let s_10_5: u32 = (s_10_4.value() as u32);
        // S s_10_6: cast zx s_10_5 -> bv
        let s_10_6: Bits = Bits::new(s_10_5 as u128, 32u16);
        // D s_10_7: read-var imm32:u32
        let s_10_7: u32 = fn_state.imm32;
        // D s_10_8: cast zx s_10_7 -> bv
        let s_10_8: Bits = Bits::new(s_10_7 as u128, 32u16);
        // D s_10_9: add s_10_6 s_10_8
        let s_10_9: Bits = (s_10_6 + s_10_8);
        // D s_10_10: cast reint s_10_9 -> u32
        let s_10_10: u32 = (s_10_9.value() as u32);
        // D s_10_11: write-var address <= s_10_10
        fn_state.address = s_10_10;
        // N s_10_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
