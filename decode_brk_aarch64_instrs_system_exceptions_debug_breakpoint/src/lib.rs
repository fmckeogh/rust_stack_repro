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
use execute_aarch64_instrs_system_exceptions_debug_breakpoint::*;
use SetBTypeCompatible::*;
use HaveBTIExt::*;
use common::*;
pub fn decode_brk_aarch64_instrs_system_exceptions_debug_breakpoint<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm16: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        comment: u16,
        imm16: u16,
    }
    let fn_state = FunctionState {
        imm16,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var imm16:u16
        let s_0_0: u16 = fn_state.imm16;
        // D s_0_1: write-var comment <= s_0_0
        fn_state.comment = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call HaveBTIExt(s_0_2)
        let s_0_3: bool = HaveBTIExt(state, tracer, s_0_2);
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
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
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var comment:u16
        let s_2_0: u16 = fn_state.comment;
        // D s_2_1: call execute_aarch64_instrs_system_exceptions_debug_breakpoint(s_2_0)
        let s_2_1: () = execute_aarch64_instrs_system_exceptions_debug_breakpoint(
            state,
            tracer,
            s_2_0,
        );
        // N s_2_2: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // S s_3_1: call SetBTypeCompatible(s_3_0)
        let s_3_1: () = SetBTypeCompatible(state, tracer, s_3_0);
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
