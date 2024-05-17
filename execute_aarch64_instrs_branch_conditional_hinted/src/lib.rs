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
use PC_read::*;
use ConditionHolds::*;
use BranchNotTaken::*;
use BranchTo::*;
use common::*;
pub fn execute_aarch64_instrs_branch_conditional_hinted<T: Tracer>(
    state: &mut State,
    tracer: &T,
    condition: u8,
    offset: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        condition: u8,
        offset: u64,
    }
    let fn_state = FunctionState {
        condition,
        offset,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var condition:u8
        let s_0_0: u8 = fn_state.condition;
        // D s_0_1: call ConditionHolds(s_0_0)
        let s_0_1: bool = ConditionHolds(state, tracer, s_0_0);
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
        // C s_1_0: const #5u : u32
        let s_1_0: u32 = 5;
        // C s_1_1: const #1u : u8
        let s_1_1: bool = true;
        // S s_1_2: call BranchNotTaken(s_1_0, s_1_1)
        let s_1_2: () = BranchNotTaken(state, tracer, s_1_0, s_1_1);
        // N s_1_3: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call PC_read(s_2_0)
        let s_2_1: u64 = PC_read(state, tracer, s_2_0);
        // S s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 64u16);
        // D s_2_3: read-var offset:u64
        let s_2_3: u64 = fn_state.offset;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 64u16);
        // D s_2_5: add s_2_2 s_2_4
        let s_2_5: Bits = (s_2_2 + s_2_4);
        // D s_2_6: cast reint s_2_5 -> u64
        let s_2_6: u64 = (s_2_5.value() as u64);
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 64u16);
        // C s_2_8: const #5u : u32
        let s_2_8: u32 = 5;
        // C s_2_9: const #1u : u8
        let s_2_9: bool = true;
        // D s_2_10: call BranchTo(s_2_7, s_2_8, s_2_9)
        let s_2_10: () = BranchTo(state, tracer, s_2_7, s_2_8, s_2_9);
        // N s_2_11: return
        return;
    }
}
