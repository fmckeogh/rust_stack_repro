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
use BRBEBranch::*;
use PC_read::*;
use common::*;
pub fn BRBEISB<T: Tracer>(state: &mut State, tracer: &T, gs_26375: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_26375: (),
    }
    let fn_state = FunctionState {
        gs_26375,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // C s_0_1: const #() : ()
        let s_0_1: () = ();
        // S s_0_2: call PC_read(s_0_1)
        let s_0_2: u64 = PC_read(state, tracer, s_0_1);
        // C s_0_3: const #4s : i
        let s_0_3: i128 = 4;
        // S s_0_4: cast zx s_0_2 -> bv
        let s_0_4: Bits = Bits::new(s_0_2 as u128, 64u16);
        // C s_0_5: cast cvt s_0_3 -> bv
        let s_0_5: Bits = Bits::new(s_0_3 as u128, 128);
        // S s_0_6: add s_0_4 s_0_5
        let s_0_6: Bits = (s_0_4 + s_0_5);
        // S s_0_7: cast reint s_0_6 -> u64
        let s_0_7: u64 = (s_0_6.value() as u64);
        // C s_0_8: const #5u : u32
        let s_0_8: u32 = 5;
        // S s_0_9: call BRBEBranch(s_0_8, s_0_0, s_0_7)
        let s_0_9: () = BRBEBranch(state, tracer, s_0_8, s_0_0, s_0_7);
        // N s_0_10: return
        return;
    }
}
