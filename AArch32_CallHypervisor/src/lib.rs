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
use AArch32_TakeHVCException::*;
use ELUsingAArch32::*;
use AArch64_CallHypervisor::*;
use common::*;
pub fn AArch32_CallHypervisor<T: Tracer>(
    state: &mut State,
    tracer: &T,
    immediate: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        immediate: u16,
    }
    let fn_state = FunctionState {
        immediate,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #432u : u32
        let s_0_0: u32 = 432;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // N s_0_4: assert s_0_3
        let s_0_4: () = assert!(s_0_3);
        // C s_0_5: const #432u : u32
        let s_0_5: u32 = 432;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: call ELUsingAArch32(s_0_6)
        let s_0_7: bool = ELUsingAArch32(state, tracer, s_0_6);
        // D s_0_8: not s_0_7
        let s_0_8: bool = !s_0_7;
        // N s_0_9: branch s_0_8 b2 b1
        if s_0_8 {
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
        // D s_1_0: read-var immediate:u16
        let s_1_0: u16 = fn_state.immediate;
        // D s_1_1: call AArch32_TakeHVCException(s_1_0)
        let s_1_1: () = AArch32_TakeHVCException(state, tracer, s_1_0);
        // N s_1_2: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var immediate:u16
        let s_2_0: u16 = fn_state.immediate;
        // D s_2_1: call AArch64_CallHypervisor(s_2_0)
        let s_2_1: () = AArch64_CallHypervisor(state, tracer, s_2_0);
        // N s_2_2: return
        return;
    }
}
