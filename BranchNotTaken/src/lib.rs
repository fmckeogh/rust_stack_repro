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
use SPEBranch::*;
use u__UNKNOWN_bits::*;
use HaveStatisticalProfiling::*;
use common::*;
pub fn BranchNotTaken<T: Tracer>(
    state: &mut State,
    tracer: &T,
    branchtype: u32,
    branch_conditional: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        branchtype: u32,
        branch_conditional: bool,
    }
    let fn_state = FunctionState {
        branchtype,
        branch_conditional,
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
        // S s_0_1: call HaveStatisticalProfiling(s_0_0)
        let s_0_1: bool = HaveStatisticalProfiling(state, tracer, s_0_0);
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
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // C s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // S s_2_2: call __UNKNOWN_bits(s_2_1)
        let s_2_2: Bits = u__UNKNOWN_bits(state, tracer, s_2_1);
        // S s_2_3: cast reint s_2_2 -> u64
        let s_2_3: u64 = (s_2_2.value() as u64);
        // S s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 64u16);
        // D s_2_5: read-var branchtype:u32
        let s_2_5: u32 = fn_state.branchtype;
        // D s_2_6: read-var branch_conditional:u8
        let s_2_6: bool = fn_state.branch_conditional;
        // C s_2_7: const #0u : u8
        let s_2_7: bool = false;
        // D s_2_8: call SPEBranch(s_2_4, s_2_5, s_2_6, s_2_7)
        let s_2_8: () = SPEBranch(state, tracer, s_2_4, s_2_5, s_2_6, s_2_7);
        // N s_2_9: return
        return;
    }
}
