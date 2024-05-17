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
use execute_aarch32_instrs_POP_Op_A_txt::*;
use ConditionPassed::*;
use LastInITBlock::*;
use BitCount::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_POP_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    P: bool,
    register_list: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        registers: u16,
        gs_299311: bool,
        gs_299310: bool,
        P: bool,
        register_list: u8,
    }
    let fn_state = FunctionState {
        P,
        register_list,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var P:u8
        let s_2_0: bool = fn_state.P;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 7u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // C s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 8u16);
        // D s_2_14: read-var register_list:u8
        let s_2_14: u8 = fn_state.register_list;
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 8u16);
        // D s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // D s_2_18: cast reint s_2_15 -> u128
        let s_2_18: u128 = (s_2_15.value() as u128);
        // D s_2_19: size-of s_2_15
        let s_2_19: u16 = s_2_15.length();
        // D s_2_20: lsl s_2_16 s_2_19
        let s_2_20: u128 = s_2_16 << s_2_19;
        // D s_2_21: or s_2_20 s_2_18
        let s_2_21: u128 = ((s_2_20) | (s_2_18));
        // D s_2_22: add s_2_17 s_2_19
        let s_2_22: u16 = (s_2_17 + s_2_19);
        // D s_2_23: create-bits s_2_21 s_2_22
        let s_2_23: Bits = Bits::new(s_2_21, s_2_22);
        // D s_2_24: cast reint s_2_23 -> u16
        let s_2_24: u16 = (s_2_23.value() as u16);
        // D s_2_25: write-var registers <= s_2_24
        fn_state.registers = s_2_24;
        // D s_2_26: read-var registers:u16
        let s_2_26: u16 = fn_state.registers;
        // D s_2_27: cast zx s_2_26 -> bv
        let s_2_27: Bits = Bits::new(s_2_26 as u128, 16u16);
        // D s_2_28: call BitCount(s_2_27)
        let s_2_28: i128 = BitCount(state, tracer, s_2_27);
        // C s_2_29: const #1s : i
        let s_2_29: i128 = 1;
        // D s_2_30: cmp-lt s_2_28 s_2_29
        let s_2_30: bool = ((s_2_28) < (s_2_29));
        // N s_2_31: branch s_2_30 b12 b3
        if s_2_30 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var registers:u16
        let s_3_1: u16 = fn_state.registers;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 16u16);
        // C s_3_3: const #1u : u64
        let s_3_3: u64 = 1;
        // D s_3_4: bit-extract s_3_2 s_3_0 s_3_3
        let s_3_4: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_3).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u8
        let s_3_5: bool = ((s_3_4.value()) != 0);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // C s_3_7: const #0u : u64
        let s_3_7: u64 = 0;
        // D s_3_8: cast zx s_3_5 -> u64
        let s_3_8: u64 = (s_3_5 as u64);
        // C s_3_9: const #1u : u64
        let s_3_9: u64 = 1;
        // D s_3_10: and s_3_8 s_3_9
        let s_3_10: u64 = ((s_3_8) & (s_3_9));
        // D s_3_11: cmp-eq s_3_10 s_3_9
        let s_3_11: bool = ((s_3_10) == (s_3_9));
        // D s_3_12: lsl s_3_8 s_3_6
        let s_3_12: u64 = s_3_8 << s_3_6;
        // D s_3_13: or s_3_7 s_3_12
        let s_3_13: u64 = ((s_3_7) | (s_3_12));
        // D s_3_14: cmpl s_3_12
        let s_3_14: u64 = !s_3_12;
        // D s_3_15: and s_3_7 s_3_14
        let s_3_15: u64 = ((s_3_7) & (s_3_14));
        // D s_3_16: select s_3_11 s_3_13 s_3_15
        let s_3_16: u64 = if s_3_11 { s_3_13 } else { s_3_15 };
        // D s_3_17: cast trunc s_3_16 -> u8
        let s_3_17: bool = ((s_3_16) != 0);
        // D s_3_18: cast zx s_3_17 -> bv
        let s_3_18: Bits = Bits::new(s_3_17 as u128, 1u16);
        // C s_3_19: const #1u : u8
        let s_3_19: bool = true;
        // C s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // D s_3_21: cmp-eq s_3_18 s_3_20
        let s_3_21: bool = ((s_3_18) == (s_3_20));
        // N s_3_22: branch s_3_21 b11 b4
        if s_3_21 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#299310 <= s_4_0
        fn_state.gs_299310 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#299310:u8
        let s_5_0: bool = fn_state.gs_299310;
        // N s_5_1: branch s_5_0 b10 b6
        if s_5_0 {
            return block_10(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#299311 <= s_6_0
        fn_state.gs_299311 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#299311:u8
        let s_7_0: bool = fn_state.gs_299311;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
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
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: read-var registers:u16
        let s_8_1: u16 = fn_state.registers;
        // D s_8_2: call execute_aarch32_instrs_POP_Op_A_txt(s_8_0, s_8_1)
        let s_8_2: () = execute_aarch32_instrs_POP_Op_A_txt(state, tracer, s_8_0, s_8_1);
        // N s_8_3: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call LastInITBlock(s_10_0)
        let s_10_1: bool = LastInITBlock(state, tracer, s_10_0);
        // S s_10_2: not s_10_1
        let s_10_2: bool = !s_10_1;
        // D s_10_3: write-var gs#299311 <= s_10_2
        fn_state.gs_299311 = s_10_2;
        // N s_10_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call InITBlock(s_11_0)
        let s_11_1: bool = InITBlock(state, tracer, s_11_0);
        // D s_11_2: write-var gs#299310 <= s_11_1
        fn_state.gs_299310 = s_11_1;
        // N s_11_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
}
