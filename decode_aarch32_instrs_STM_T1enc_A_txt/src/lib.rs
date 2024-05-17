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
use BitCount::*;
use execute_aarch32_instrs_STM_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STM_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rn: u8,
    register_list: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        registers: u16,
        Rn: u8,
        register_list: u8,
    }
    let fn_state = FunctionState {
        Rn,
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
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 3u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var n <= s_2_3
        fn_state.n = s_2_3;
        // C s_2_5: const #0u : u8
        let s_2_5: u8 = 0;
        // C s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 8u16);
        // D s_2_7: read-var register_list:u8
        let s_2_7: u8 = fn_state.register_list;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 8u16);
        // C s_2_9: cast reint s_2_6 -> u128
        let s_2_9: u128 = (s_2_6.value() as u128);
        // D s_2_10: size-of s_2_6
        let s_2_10: u16 = s_2_6.length();
        // D s_2_11: cast reint s_2_8 -> u128
        let s_2_11: u128 = (s_2_8.value() as u128);
        // D s_2_12: size-of s_2_8
        let s_2_12: u16 = s_2_8.length();
        // D s_2_13: lsl s_2_9 s_2_12
        let s_2_13: u128 = s_2_9 << s_2_12;
        // D s_2_14: or s_2_13 s_2_11
        let s_2_14: u128 = ((s_2_13) | (s_2_11));
        // D s_2_15: add s_2_10 s_2_12
        let s_2_15: u16 = (s_2_10 + s_2_12);
        // D s_2_16: create-bits s_2_14 s_2_15
        let s_2_16: Bits = Bits::new(s_2_14, s_2_15);
        // D s_2_17: cast reint s_2_16 -> u16
        let s_2_17: u16 = (s_2_16.value() as u16);
        // D s_2_18: write-var registers <= s_2_17
        fn_state.registers = s_2_17;
        // D s_2_19: read-var registers:u16
        let s_2_19: u16 = fn_state.registers;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 16u16);
        // D s_2_21: call BitCount(s_2_20)
        let s_2_21: i128 = BitCount(state, tracer, s_2_20);
        // C s_2_22: const #1s : i
        let s_2_22: i128 = 1;
        // D s_2_23: cmp-lt s_2_21 s_2_22
        let s_2_23: bool = ((s_2_21) < (s_2_22));
        // N s_2_24: branch s_2_23 b4 b3
        if s_2_23 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: read-var registers:u16
        let s_3_1: u16 = fn_state.registers;
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // D s_3_3: call execute_aarch32_instrs_STM_Op_A_txt(s_3_0, s_3_1, s_3_2)
        let s_3_3: () = execute_aarch32_instrs_STM_Op_A_txt(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
        );
        // N s_3_4: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}
