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
use execute_aarch64_instrs_branch_conditional_compare::*;
use common::*;
pub fn decode_cbz_aarch64_instrs_branch_conditional_compare<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    imm19: u32,
    op: bool,
    sf: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        ga_251020: i64,
        Rt: u8,
        imm19: u32,
        op: bool,
        sf: bool,
    }
    let fn_state = FunctionState {
        Rt,
        imm19,
        op,
        sf,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rt:u8
        let s_0_0: u8 = fn_state.Rt;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var t <= s_0_3
        fn_state.t = s_0_3;
        // D s_0_5: read-var sf:u8
        let s_0_5: bool = fn_state.sf;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 1u16);
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // C s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 1u16);
        // D s_0_9: cmp-eq s_0_6 s_0_8
        let s_0_9: bool = ((s_0_6) == (s_0_8));
        // N s_0_10: branch s_0_9 b3 b1
        if s_0_9 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ga#251020 <= s_1_0
        fn_state.ga_251020 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#251020:i64
        let s_2_0: i64 = fn_state.ga_251020;
        // D s_2_1: read-var op:u8
        let s_2_1: bool = fn_state.op;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 1u16);
        // C s_2_3: const #0u : u8
        let s_2_3: bool = false;
        // C s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 1u16);
        // D s_2_5: cmp-eq s_2_2 s_2_4
        let s_2_5: bool = ((s_2_2) == (s_2_4));
        // D s_2_6: read-var imm19:u19
        let s_2_6: u32 = fn_state.imm19;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 19u16);
        // C s_2_8: const #0u : u8
        let s_2_8: u8 = 0;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 2u16);
        // D s_2_10: cast reint s_2_7 -> u128
        let s_2_10: u128 = (s_2_7.value() as u128);
        // D s_2_11: size-of s_2_7
        let s_2_11: u16 = s_2_7.length();
        // C s_2_12: cast reint s_2_9 -> u128
        let s_2_12: u128 = (s_2_9.value() as u128);
        // D s_2_13: size-of s_2_9
        let s_2_13: u16 = s_2_9.length();
        // D s_2_14: lsl s_2_10 s_2_13
        let s_2_14: u128 = s_2_10 << s_2_13;
        // D s_2_15: or s_2_14 s_2_12
        let s_2_15: u128 = ((s_2_14) | (s_2_12));
        // D s_2_16: add s_2_11 s_2_13
        let s_2_16: u16 = (s_2_11 + s_2_13);
        // D s_2_17: create-bits s_2_15 s_2_16
        let s_2_17: Bits = Bits::new(s_2_15, s_2_16);
        // D s_2_18: cast reint s_2_17 -> u21
        let s_2_18: u32 = (s_2_17.value() as u32);
        // C s_2_19: const #64s : i
        let s_2_19: i128 = 64;
        // D s_2_20: cast zx s_2_18 -> bv
        let s_2_20: Bits = Bits::new(s_2_18 as u128, 21u16);
        // D s_2_21: bits-cast sx s_2_20 -> bv length s_2_19
        let s_2_21: Bits = s_2_20.sign_extend(s_2_19);
        // D s_2_22: cast reint s_2_21 -> u64
        let s_2_22: u64 = (s_2_21.value() as u64);
        // D s_2_23: cast zx s_2_0 -> i
        let s_2_23: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_24: cast reint s_2_23 -> i64
        let s_2_24: i64 = (s_2_23 as i64);
        // D s_2_25: read-var t:i64
        let s_2_25: i64 = fn_state.t;
        // D s_2_26: call execute_aarch64_instrs_branch_conditional_compare(s_2_24, s_2_5, s_2_22, s_2_25)
        let s_2_26: () = execute_aarch64_instrs_branch_conditional_compare(
            state,
            tracer,
            s_2_24,
            s_2_5,
            s_2_22,
            s_2_25,
        );
        // N s_2_27: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#251020 <= s_3_0
        fn_state.ga_251020 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
