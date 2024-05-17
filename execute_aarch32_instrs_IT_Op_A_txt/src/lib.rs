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
use AArch32_CheckITEnabled::*;
use common::*;
pub fn execute_aarch32_instrs_IT_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    firstcond: u8,
    mask: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        firstcond: u8,
        mask: u8,
    }
    let fn_state = FunctionState {
        firstcond,
        mask,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var mask:u8
        let s_0_0: u8 = fn_state.mask;
        // D s_0_1: call AArch32_CheckITEnabled(s_0_0)
        let s_0_1: () = AArch32_CheckITEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16968u : u32
        let s_1_0: u32 = 16968;
        // D s_1_1: read-reg s_1_0:struct
        let s_1_1: ProductTypec98939056e929b9c = {
            let value = state
                .read_register::<ProductTypec98939056e929b9c>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #16968u : u32
        let s_1_2: u32 = 16968;
        // N s_1_3: write-reg s_1_2 <= s_1_1
        let s_1_3: () = {
            state.write_register::<ProductTypec98939056e929b9c>(s_1_2 as isize, s_1_1);
            tracer.write_register(s_1_2 as isize, s_1_1);
        };
        // C s_1_4: const #0u : u8
        let s_1_4: bool = false;
        // C s_1_5: const #104792u : u32
        let s_1_5: u32 = 104792;
        // N s_1_6: write-reg s_1_5 <= s_1_4
        let s_1_6: () = {
            state.write_register::<bool>(s_1_5 as isize, s_1_4);
            tracer.write_register(s_1_5 as isize, s_1_4);
        };
        // N s_1_7: return
        return;
    }
}
