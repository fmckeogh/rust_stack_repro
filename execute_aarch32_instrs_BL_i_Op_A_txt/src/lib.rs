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
use BranchWritePC::*;
use Align_bits::*;
use CurrentInstrSet::*;
use LR_write::*;
use PC_read__1::*;
use SelectInstrSet::*;
use common::*;
pub fn execute_aarch32_instrs_BL_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm32: u32,
    targetInstrSet: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        targetAddress: u32,
        imm32: u32,
        targetInstrSet: u32,
    }
    let fn_state = FunctionState {
        imm32,
        targetInstrSet,
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
        // S s_0_1: call CurrentInstrSet(s_0_0)
        let s_0_1: u32 = CurrentInstrSet(state, tracer, s_0_0);
        // C s_0_2: const #1u : u32
        let s_0_2: u32 = 1;
        // S s_0_3: cmp-eq s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) == (s_0_2));
        // N s_0_4: branch s_0_3 b6 b1
        if s_0_3 {
            return block_6(state, tracer, fn_state);
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
        // C s_1_2: const #1s : i
        let s_1_2: i128 = 1;
        // S s_1_3: cast zx s_1_1 -> bv
        let s_1_3: Bits = Bits::new(s_1_1 as u128, 32u16);
        // C s_1_4: const #1s : i64
        let s_1_4: i64 = 1;
        // C s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // C s_1_6: const #30s : i
        let s_1_6: i128 = 30;
        // C s_1_7: add s_1_6 s_1_5
        let s_1_7: i128 = (s_1_6 + s_1_5);
        // D s_1_8: bit-extract s_1_3 s_1_2 s_1_7
        let s_1_8: Bits = (Bits::new(
            ((s_1_3) >> (s_1_2)).value(),
            u16::try_from(s_1_7).unwrap(),
        ));
        // D s_1_9: cast reint s_1_8 -> u31
        let s_1_9: u32 = (s_1_8.value() as u32);
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 31u16);
        // C s_1_11: const #1u : u8
        let s_1_11: bool = true;
        // C s_1_12: cast zx s_1_11 -> bv
        let s_1_12: Bits = Bits::new(s_1_11 as u128, 1u16);
        // D s_1_13: cast reint s_1_10 -> u128
        let s_1_13: u128 = (s_1_10.value() as u128);
        // D s_1_14: size-of s_1_10
        let s_1_14: u16 = s_1_10.length();
        // C s_1_15: cast reint s_1_12 -> u128
        let s_1_15: u128 = (s_1_12.value() as u128);
        // D s_1_16: size-of s_1_12
        let s_1_16: u16 = s_1_12.length();
        // D s_1_17: lsl s_1_13 s_1_16
        let s_1_17: u128 = s_1_13 << s_1_16;
        // D s_1_18: or s_1_17 s_1_15
        let s_1_18: u128 = ((s_1_17) | (s_1_15));
        // D s_1_19: add s_1_14 s_1_16
        let s_1_19: u16 = (s_1_14 + s_1_16);
        // D s_1_20: create-bits s_1_18 s_1_19
        let s_1_20: Bits = Bits::new(s_1_18, s_1_19);
        // D s_1_21: cast reint s_1_20 -> u32
        let s_1_21: u32 = (s_1_20.value() as u32);
        // D s_1_22: call LR_write(s_1_21)
        let s_1_22: () = LR_write(state, tracer, s_1_21);
        // N s_1_23: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var targetInstrSet:u32
        let s_2_0: u32 = fn_state.targetInstrSet;
        // C s_2_1: const #1u : u32
        let s_2_1: u32 = 1;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b5 b3
        if s_2_2 {
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
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call PC_read__1(s_3_0)
        let s_3_1: u32 = PC_read__1(state, tracer, s_3_0);
        // S s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 32u16);
        // D s_3_3: read-var imm32:u32
        let s_3_3: u32 = fn_state.imm32;
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 32u16);
        // D s_3_5: add s_3_2 s_3_4
        let s_3_5: Bits = (s_3_2 + s_3_4);
        // D s_3_6: cast reint s_3_5 -> u32
        let s_3_6: u32 = (s_3_5.value() as u32);
        // D s_3_7: write-var targetAddress <= s_3_6
        fn_state.targetAddress = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var targetInstrSet:u32
        let s_4_0: u32 = fn_state.targetInstrSet;
        // D s_4_1: call SelectInstrSet(s_4_0)
        let s_4_1: () = SelectInstrSet(state, tracer, s_4_0);
        // D s_4_2: read-var targetAddress:u32
        let s_4_2: u32 = fn_state.targetAddress;
        // C s_4_3: const #0u : u32
        let s_4_3: u32 = 0;
        // D s_4_4: call BranchWritePC(s_4_2, s_4_3)
        let s_4_4: () = BranchWritePC(state, tracer, s_4_2, s_4_3);
        // N s_4_5: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call PC_read__1(s_5_0)
        let s_5_1: u32 = PC_read__1(state, tracer, s_5_0);
        // C s_5_2: const #4s : i
        let s_5_2: i128 = 4;
        // S s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 32u16);
        // S s_5_4: call Align_bits(s_5_3, s_5_2)
        let s_5_4: Bits = Align_bits(state, tracer, s_5_3, s_5_2);
        // S s_5_5: cast reint s_5_4 -> u32
        let s_5_5: u32 = (s_5_4.value() as u32);
        // S s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 32u16);
        // D s_5_7: read-var imm32:u32
        let s_5_7: u32 = fn_state.imm32;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 32u16);
        // D s_5_9: add s_5_6 s_5_8
        let s_5_9: Bits = (s_5_6 + s_5_8);
        // D s_5_10: cast reint s_5_9 -> u32
        let s_5_10: u32 = (s_5_9.value() as u32);
        // D s_5_11: write-var targetAddress <= s_5_10
        fn_state.targetAddress = s_5_10;
        // N s_5_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call PC_read__1(s_6_0)
        let s_6_1: u32 = PC_read__1(state, tracer, s_6_0);
        // C s_6_2: const #4s : i
        let s_6_2: i128 = 4;
        // S s_6_3: cast zx s_6_1 -> bv
        let s_6_3: Bits = Bits::new(s_6_1 as u128, 32u16);
        // C s_6_4: cast cvt s_6_2 -> bv
        let s_6_4: Bits = Bits::new(s_6_2 as u128, 128);
        // S s_6_5: sub s_6_3 s_6_4
        let s_6_5: Bits = ((s_6_3) - (s_6_4));
        // S s_6_6: cast reint s_6_5 -> u32
        let s_6_6: u32 = (s_6_5.value() as u32);
        // S s_6_7: call LR_write(s_6_6)
        let s_6_7: () = LR_write(state, tracer, s_6_6);
        // N s_6_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
