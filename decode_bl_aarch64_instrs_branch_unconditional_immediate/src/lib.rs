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
use execute_aarch64_instrs_branch_unconditional_immediate::*;
use common::*;
pub fn decode_bl_aarch64_instrs_branch_unconditional_immediate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm26: u32,
    op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        branch_type: u32,
        imm26: u32,
        op: bool,
    }
    let fn_state = FunctionState {
        imm26,
        op,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var op:u8
        let s_0_0: bool = fn_state.op;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
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
        // C s_1_0: const #5u : u32
        let s_1_0: u32 = 5;
        // D s_1_1: write-var branch_type <= s_1_0
        fn_state.branch_type = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var imm26:u26
        let s_2_0: u32 = fn_state.imm26;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 26u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
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
        // D s_2_12: cast reint s_2_11 -> u28
        let s_2_12: u32 = (s_2_11.value() as u32);
        // C s_2_13: const #64s : i
        let s_2_13: i128 = 64;
        // D s_2_14: cast zx s_2_12 -> bv
        let s_2_14: Bits = Bits::new(s_2_12 as u128, 28u16);
        // D s_2_15: bits-cast sx s_2_14 -> bv length s_2_13
        let s_2_15: Bits = s_2_14.sign_extend(s_2_13);
        // D s_2_16: cast reint s_2_15 -> u64
        let s_2_16: u64 = (s_2_15.value() as u64);
        // D s_2_17: read-var branch_type:u32
        let s_2_17: u32 = fn_state.branch_type;
        // D s_2_18: call execute_aarch64_instrs_branch_unconditional_immediate(s_2_17, s_2_16)
        let s_2_18: () = execute_aarch64_instrs_branch_unconditional_immediate(
            state,
            tracer,
            s_2_17,
            s_2_16,
        );
        // N s_2_19: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u32
        let s_3_0: u32 = 0;
        // D s_3_1: write-var branch_type <= s_3_0
        fn_state.branch_type = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
