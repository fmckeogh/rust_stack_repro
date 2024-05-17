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
use PC_read__1::*;
use BranchWritePC::*;
use common::*;
pub fn execute_aarch32_instrs_B_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm32: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
    }
    let fn_state = FunctionState {
        imm32,
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
        // S s_0_1: call PC_read__1(s_0_0)
        let s_0_1: u32 = PC_read__1(state, tracer, s_0_0);
        // S s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 32u16);
        // D s_0_3: read-var imm32:u32
        let s_0_3: u32 = fn_state.imm32;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 32u16);
        // D s_0_5: add s_0_2 s_0_4
        let s_0_5: Bits = (s_0_2 + s_0_4);
        // D s_0_6: cast reint s_0_5 -> u32
        let s_0_6: u32 = (s_0_5.value() as u32);
        // C s_0_7: const #5u : u32
        let s_0_7: u32 = 5;
        // D s_0_8: call BranchWritePC(s_0_6, s_0_7)
        let s_0_8: () = BranchWritePC(state, tracer, s_0_6, s_0_7);
        // N s_0_9: return
        return;
    }
}
