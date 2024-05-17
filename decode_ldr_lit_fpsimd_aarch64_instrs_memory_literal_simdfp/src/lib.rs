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
use execute_aarch64_instrs_memory_literal_simdfp::*;
use common::*;
pub fn decode_ldr_lit_fpsimd_aarch64_instrs_memory_literal_simdfp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    imm19: u32,
    opc: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        size: i64,
        t: i64,
        Rt: u8,
        imm19: u32,
        opc: u8,
    }
    let fn_state = FunctionState {
        Rt,
        imm19,
        opc,
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
        // C s_0_5: const #4s : i64
        let s_0_5: i64 = 4;
        // D s_0_6: write-var size <= s_0_5
        fn_state.size = s_0_5;
        // D s_0_7: read-var opc:u8
        let s_0_7: u8 = fn_state.opc;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 2u16);
        // C s_0_9: const #0u : u8
        let s_0_9: u8 = 0;
        // C s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 2u16);
        // D s_0_11: cmp-eq s_0_8 s_0_10
        let s_0_11: bool = ((s_0_8) == (s_0_10));
        // D s_0_12: not s_0_11
        let s_0_12: bool = !s_0_11;
        // N s_0_13: branch s_0_12 b3 b1
        if s_0_12 {
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
        // C s_1_0: const #4s : i64
        let s_1_0: i64 = 4;
        // D s_1_1: write-var size <= s_1_0
        fn_state.size = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:i64
        let s_2_0: i64 = fn_state.size;
        // D s_2_1: read-var imm19:u19
        let s_2_1: u32 = fn_state.imm19;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 19u16);
        // C s_2_3: const #0u : u8
        let s_2_3: u8 = 0;
        // C s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cast reint s_2_2 -> u128
        let s_2_5: u128 = (s_2_2.value() as u128);
        // D s_2_6: size-of s_2_2
        let s_2_6: u16 = s_2_2.length();
        // C s_2_7: cast reint s_2_4 -> u128
        let s_2_7: u128 = (s_2_4.value() as u128);
        // D s_2_8: size-of s_2_4
        let s_2_8: u16 = s_2_4.length();
        // D s_2_9: lsl s_2_5 s_2_8
        let s_2_9: u128 = s_2_5 << s_2_8;
        // D s_2_10: or s_2_9 s_2_7
        let s_2_10: u128 = ((s_2_9) | (s_2_7));
        // D s_2_11: add s_2_6 s_2_8
        let s_2_11: u16 = (s_2_6 + s_2_8);
        // D s_2_12: create-bits s_2_10 s_2_11
        let s_2_12: Bits = Bits::new(s_2_10, s_2_11);
        // D s_2_13: cast reint s_2_12 -> u21
        let s_2_13: u32 = (s_2_12.value() as u32);
        // C s_2_14: const #64s : i
        let s_2_14: i128 = 64;
        // D s_2_15: cast zx s_2_13 -> bv
        let s_2_15: Bits = Bits::new(s_2_13 as u128, 21u16);
        // D s_2_16: bits-cast sx s_2_15 -> bv length s_2_14
        let s_2_16: Bits = s_2_15.sign_extend(s_2_14);
        // D s_2_17: cast reint s_2_16 -> u64
        let s_2_17: u64 = (s_2_16.value() as u64);
        // C s_2_18: const #0u : u8
        let s_2_18: bool = false;
        // C s_2_19: const #0u : u8
        let s_2_19: bool = false;
        // D s_2_20: read-var t:i64
        let s_2_20: i64 = fn_state.t;
        // D s_2_21: call execute_aarch64_instrs_memory_literal_simdfp(s_2_19, s_2_17, s_2_0, s_2_20, s_2_18)
        let s_2_21: () = execute_aarch64_instrs_memory_literal_simdfp(
            state,
            tracer,
            s_2_19,
            s_2_17,
            s_2_0,
            s_2_20,
            s_2_18,
        );
        // N s_2_22: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var opc:u8
        let s_3_0: u8 = fn_state.opc;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
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
        // C s_4_0: const #8s : i64
        let s_4_0: i64 = 8;
        // D s_4_1: write-var size <= s_4_0
        fn_state.size = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var opc:u8
        let s_5_0: u8 = fn_state.opc;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16s : i64
        let s_6_0: i64 = 16;
        // D s_6_1: write-var size <= s_6_0
        fn_state.size = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
}
