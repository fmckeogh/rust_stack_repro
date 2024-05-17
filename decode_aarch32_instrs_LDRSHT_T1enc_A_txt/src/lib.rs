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
use ConditionPassed::*;
use execute_aarch32_instrs_LDRSHT_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDRSHT_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    Rt: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i128,
        t: i64,
        imm32: u32,
        mshadow_7200: i128,
        n: i64,
        Rn: u8,
        Rt: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        Rn,
        Rt,
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
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b6 b3
        if s_2_4 {
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
        // D s_3_0: read-var m:i
        let s_3_0: i128 = fn_state.m;
        // D s_3_1: write-var mshadow#7200 <= s_3_0
        fn_state.mshadow_7200 = s_3_0;
        // D s_3_2: read-var Rt:u8
        let s_3_2: u8 = fn_state.Rt;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (s_3_3.value() as i128);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var t <= s_3_5
        fn_state.t = s_3_5;
        // D s_3_7: read-var Rn:u8
        let s_3_7: u8 = fn_state.Rn;
        // D s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 4u16);
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (s_3_8.value() as i128);
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: write-var n <= s_3_10
        fn_state.n = s_3_10;
        // C s_3_12: const #32s : i
        let s_3_12: i128 = 32;
        // D s_3_13: read-var imm8:u8
        let s_3_13: u8 = fn_state.imm8;
        // D s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 8u16);
        // D s_3_15: bits-cast zx s_3_14 -> bv length s_3_12
        let s_3_15: Bits = s_3_14.zero_extend(s_3_12);
        // D s_3_16: cast reint s_3_15 -> u32
        let s_3_16: u32 = (s_3_15.value() as u32);
        // D s_3_17: write-var imm32 <= s_3_16
        fn_state.imm32 = s_3_16;
        // C s_3_18: const #15s : i
        let s_3_18: i128 = 15;
        // D s_3_19: read-var t:i64
        let s_3_19: i64 = fn_state.t;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: cmp-eq s_3_20 s_3_18
        let s_3_21: bool = ((s_3_20) == (s_3_18));
        // N s_3_22: branch s_3_21 b5 b4
        if s_3_21 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // D s_4_1: read-var imm32:u32
        let s_4_1: u32 = fn_state.imm32;
        // D s_4_2: read-var mshadow#7200:i
        let s_4_2: i128 = fn_state.mshadow_7200;
        // D s_4_3: read-var n:i64
        let s_4_3: i64 = fn_state.n;
        // C s_4_4: const #0u : u8
        let s_4_4: bool = false;
        // C s_4_5: const #0u : u8
        let s_4_5: bool = false;
        // D s_4_6: read-var t:i64
        let s_4_6: i64 = fn_state.t;
        // D s_4_7: call execute_aarch32_instrs_LDRSHT_Op_A_txt(s_4_0, s_4_1, s_4_2, s_4_3, s_4_4, s_4_5, s_4_6)
        let s_4_7: () = execute_aarch32_instrs_LDRSHT_Op_A_txt(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
            s_4_3,
            s_4_4,
            s_4_5,
            s_4_6,
        );
        // N s_4_8: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
}
