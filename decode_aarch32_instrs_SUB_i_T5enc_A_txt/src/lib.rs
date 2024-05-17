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
use execute_aarch32_instrs_SUB_i_Op_A_txt::*;
use LastInITBlock::*;
use ConditionPassed::*;
use IsZero::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_SUB_i_T5enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        n: i64,
        gs_303190: bool,
        gs_303197: bool,
        Rn: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        Rn,
        imm8,
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
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #14u : u8
        let s_2_2: u8 = 14;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b14 b3
        if s_2_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#303190 <= s_3_0
        fn_state.gs_303190 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#303190:u8
        let s_4_0: bool = fn_state.gs_303190;
        // N s_4_1: branch s_4_0 b13 b5
        if s_4_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rn:u8
        let s_5_0: u8 = fn_state.Rn;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: write-var n <= s_5_3
        fn_state.n = s_5_3;
        // C s_5_5: const #32s : i
        let s_5_5: i128 = 32;
        // D s_5_6: read-var imm8:u8
        let s_5_6: u8 = fn_state.imm8;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 8u16);
        // D s_5_8: bits-cast zx s_5_7 -> bv length s_5_5
        let s_5_8: Bits = s_5_7.zero_extend(s_5_5);
        // D s_5_9: cast reint s_5_8 -> u32
        let s_5_9: u32 = (s_5_8.value() as u32);
        // D s_5_10: write-var imm32 <= s_5_9
        fn_state.imm32 = s_5_9;
        // C s_5_11: const #14s : i
        let s_5_11: i128 = 14;
        // D s_5_12: read-var n:i64
        let s_5_12: i64 = fn_state.n;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: call neq_int(s_5_13, s_5_11)
        let s_5_14: bool = neq_int(state, tracer, s_5_13, s_5_11);
        // N s_5_15: branch s_5_14 b12 b6
        if s_5_14 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call InITBlock(s_6_0)
        let s_6_1: bool = InITBlock(state, tracer, s_6_0);
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
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#303197 <= s_7_0
        fn_state.gs_303197 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#303197:u8
        let s_8_0: bool = fn_state.gs_303197;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #15s : i64
        let s_9_0: i64 = 15;
        // D s_9_1: read-var imm32:u32
        let s_9_1: u32 = fn_state.imm32;
        // D s_9_2: read-var n:i64
        let s_9_2: i64 = fn_state.n;
        // C s_9_3: const #1u : u8
        let s_9_3: bool = true;
        // D s_9_4: call execute_aarch32_instrs_SUB_i_Op_A_txt(s_9_0, s_9_1, s_9_2, s_9_3)
        let s_9_4: () = execute_aarch32_instrs_SUB_i_Op_A_txt(
            state,
            tracer,
            s_9_0,
            s_9_1,
            s_9_2,
            s_9_3,
        );
        // N s_9_5: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call LastInITBlock(s_11_0)
        let s_11_1: bool = LastInITBlock(state, tracer, s_11_0);
        // S s_11_2: not s_11_1
        let s_11_2: bool = !s_11_1;
        // D s_11_3: write-var gs#303197 <= s_11_2
        fn_state.gs_303197 = s_11_2;
        // N s_11_4: jump b8
        return block_8(state, tracer, fn_state);
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
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var imm8:u8
        let s_14_0: u8 = fn_state.imm8;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 8u16);
        // D s_14_2: call IsZero(s_14_1)
        let s_14_2: bool = IsZero(state, tracer, s_14_1);
        // D s_14_3: write-var gs#303190 <= s_14_2
        fn_state.gs_303190 = s_14_2;
        // N s_14_4: jump b4
        return block_4(state, tracer, fn_state);
    }
}
