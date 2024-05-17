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
use execute_aarch32_instrs_STR_i_OpT_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STR_i_T3enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    Rt: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        imm32: u32,
        n: i64,
        Rn: u8,
        Rt: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        Rn,
        Rt,
        imm12,
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
        // D s_3_0: read-var Rt:u8
        let s_3_0: u8 = fn_state.Rt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var t <= s_3_3
        fn_state.t = s_3_3;
        // D s_3_5: read-var Rn:u8
        let s_3_5: u8 = fn_state.Rn;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 4u16);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (s_3_6.value() as i128);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: write-var n <= s_3_8
        fn_state.n = s_3_8;
        // C s_3_10: const #32s : i
        let s_3_10: i128 = 32;
        // D s_3_11: read-var imm12:u12
        let s_3_11: u16 = fn_state.imm12;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 12u16);
        // D s_3_13: bits-cast zx s_3_12 -> bv length s_3_10
        let s_3_13: Bits = s_3_12.zero_extend(s_3_10);
        // D s_3_14: cast reint s_3_13 -> u32
        let s_3_14: u32 = (s_3_13.value() as u32);
        // D s_3_15: write-var imm32 <= s_3_14
        fn_state.imm32 = s_3_14;
        // C s_3_16: const #15s : i
        let s_3_16: i128 = 15;
        // D s_3_17: read-var t:i64
        let s_3_17: i64 = fn_state.t;
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: cmp-eq s_3_18 s_3_16
        let s_3_19: bool = ((s_3_18) == (s_3_16));
        // N s_3_20: branch s_3_19 b5 b4
        if s_3_19 {
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
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // D s_4_3: read-var n:i64
        let s_4_3: i64 = fn_state.n;
        // D s_4_4: read-var t:i64
        let s_4_4: i64 = fn_state.t;
        // C s_4_5: const #0u : u8
        let s_4_5: bool = false;
        // D s_4_6: call execute_aarch32_instrs_STR_i_OpT_A_txt(s_4_0, s_4_1, s_4_2, s_4_3, s_4_4, s_4_5)
        let s_4_6: () = execute_aarch32_instrs_STR_i_OpT_A_txt(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
            s_4_3,
            s_4_4,
            s_4_5,
        );
        // N s_4_7: return
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
