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
use AArch32_IMPDEFResets::*;
use HaveAArch64::*;
use AArch64_IMPDEFResets::*;
use AArch64_AutoGen_ArchitectureReset::*;
use AArch32_AutoGen_ArchitectureReset::*;
use u__Reset::*;
use common::*;
pub fn ResetControlRegisters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cold: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        cold: bool,
    }
    let fn_state = FunctionState {
        cold,
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
        // S s_0_1: call HaveAArch64(s_0_0)
        let s_0_1: bool = HaveAArch64(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
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
        // D s_1_0: read-var cold:u8
        let s_1_0: bool = fn_state.cold;
        // D s_1_1: call AArch32_AutoGen_ArchitectureReset(s_1_0)
        let s_1_1: () = AArch32_AutoGen_ArchitectureReset(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call AArch32_IMPDEFResets(s_2_0)
        let s_2_1: () = AArch32_IMPDEFResets(state, tracer, s_2_0);
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call AArch64_IMPDEFResets(s_2_2)
        let s_2_3: () = AArch64_IMPDEFResets(state, tracer, s_2_2);
        // D s_2_4: read-var cold:u8
        let s_2_4: bool = fn_state.cold;
        // D s_2_5: call __Reset(s_2_4)
        let s_2_5: () = u__Reset(state, tracer, s_2_4);
        // N s_2_6: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var cold:u8
        let s_3_0: bool = fn_state.cold;
        // D s_3_1: call AArch64_AutoGen_ArchitectureReset(s_3_0)
        let s_3_1: () = AArch64_AutoGen_ArchitectureReset(state, tracer, s_3_0);
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
