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
use execute_aarch64_instrs_branch_conditional_hinted::*;
use HaveFeatHBC::*;
use common::*;
pub fn decode_bc_cond_aarch64_instrs_branch_conditional_hinted<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    imm19: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        cond: u8,
        imm19: u32,
    }
    let fn_state = FunctionState {
        cond,
        imm19,
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
        // S s_0_1: call HaveFeatHBC(s_0_0)
        let s_0_1: bool = HaveFeatHBC(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b2 b1
        if s_0_2 {
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
        // D s_1_0: read-var imm19:u19
        let s_1_0: u32 = fn_state.imm19;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 19u16);
        // C s_1_2: const #0u : u8
        let s_1_2: u8 = 0;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cast reint s_1_1 -> u128
        let s_1_4: u128 = (s_1_1.value() as u128);
        // D s_1_5: size-of s_1_1
        let s_1_5: u16 = s_1_1.length();
        // C s_1_6: cast reint s_1_3 -> u128
        let s_1_6: u128 = (s_1_3.value() as u128);
        // D s_1_7: size-of s_1_3
        let s_1_7: u16 = s_1_3.length();
        // D s_1_8: lsl s_1_4 s_1_7
        let s_1_8: u128 = s_1_4 << s_1_7;
        // D s_1_9: or s_1_8 s_1_6
        let s_1_9: u128 = ((s_1_8) | (s_1_6));
        // D s_1_10: add s_1_5 s_1_7
        let s_1_10: u16 = (s_1_5 + s_1_7);
        // D s_1_11: create-bits s_1_9 s_1_10
        let s_1_11: Bits = Bits::new(s_1_9, s_1_10);
        // D s_1_12: cast reint s_1_11 -> u21
        let s_1_12: u32 = (s_1_11.value() as u32);
        // C s_1_13: const #64s : i
        let s_1_13: i128 = 64;
        // D s_1_14: cast zx s_1_12 -> bv
        let s_1_14: Bits = Bits::new(s_1_12 as u128, 21u16);
        // D s_1_15: bits-cast sx s_1_14 -> bv length s_1_13
        let s_1_15: Bits = s_1_14.sign_extend(s_1_13);
        // D s_1_16: cast reint s_1_15 -> u64
        let s_1_16: u64 = (s_1_15.value() as u64);
        // D s_1_17: read-var cond:u8
        let s_1_17: u8 = fn_state.cond;
        // D s_1_18: call execute_aarch64_instrs_branch_conditional_hinted(s_1_17, s_1_16)
        let s_1_18: () = execute_aarch64_instrs_branch_conditional_hinted(
            state,
            tracer,
            s_1_17,
            s_1_16,
        );
        // N s_1_19: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
