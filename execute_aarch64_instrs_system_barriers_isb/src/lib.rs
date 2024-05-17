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
use BRBEISB::*;
use HaveStatisticalProfiling::*;
use SPEISB::*;
use BRBEBranchOnISB::*;
use InstructionSynchronizationBarrier::*;
use HaveBRBExt::*;
use common::*;
pub fn execute_aarch64_instrs_system_barriers_isb<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_155185: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_155186: bool,
        gs_155185: (),
    }
    let fn_state = FunctionState {
        gs_155185,
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
        // S s_0_1: call InstructionSynchronizationBarrier(s_0_0)
        let s_0_1: () = InstructionSynchronizationBarrier(state, tracer, s_0_0);
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call HaveBRBExt(s_0_2)
        let s_0_3: bool = HaveBRBExt(state, tracer, s_0_2);
        // N s_0_4: branch s_0_3 b8 b1
        if s_0_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#155186 <= s_1_0
        fn_state.gs_155186 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#155186:u8
        let s_2_0: bool = fn_state.gs_155186;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call HaveStatisticalProfiling(s_4_0)
        let s_4_1: bool = HaveStatisticalProfiling(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b6 b5
        if s_4_1 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call SPEISB(s_6_0)
        let s_6_1: () = SPEISB(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call BRBEISB(s_7_0)
        let s_7_1: () = BRBEISB(state, tracer, s_7_0);
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call BRBEBranchOnISB(s_8_0)
        let s_8_1: bool = BRBEBranchOnISB(state, tracer, s_8_0);
        // D s_8_2: write-var gs#155186 <= s_8_1
        fn_state.gs_155186 = s_8_1;
        // N s_8_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
