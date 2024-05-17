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
use SPEBranch__1::*;
use PC_read::*;
use common::*;
pub fn SPEISB<T: Tracer>(state: &mut State, tracer: &T, gs_26298: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_26298: (),
    }
    let fn_state = FunctionState {
        gs_26298,
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
        // S s_0_1: call PC_read(s_0_0)
        let s_0_1: u64 = PC_read(state, tracer, s_0_0);
        // C s_0_2: const #4s : i
        let s_0_2: i128 = 4;
        // S s_0_3: cast zx s_0_1 -> bv
        let s_0_3: Bits = Bits::new(s_0_1 as u128, 64u16);
        // C s_0_4: cast cvt s_0_2 -> bv
        let s_0_4: Bits = Bits::new(s_0_2 as u128, 128);
        // S s_0_5: add s_0_3 s_0_4
        let s_0_5: Bits = (s_0_3 + s_0_4);
        // S s_0_6: cast reint s_0_5 -> u64
        let s_0_6: u64 = (s_0_5.value() as u64);
        // C s_0_7: const #5u : u32
        let s_0_7: u32 = 5;
        // C s_0_8: const #0u : u8
        let s_0_8: bool = false;
        // C s_0_9: const #0u : u8
        let s_0_9: bool = false;
        // C s_0_10: const #1u : u8
        let s_0_10: bool = true;
        // S s_0_11: cast zx s_0_6 -> bv
        let s_0_11: Bits = Bits::new(s_0_6 as u128, 64u16);
        // S s_0_12: call SPEBranch__1(s_0_11, s_0_7, s_0_8, s_0_9, s_0_10)
        let s_0_12: () = SPEBranch__1(
            state,
            tracer,
            s_0_11,
            s_0_7,
            s_0_8,
            s_0_9,
            s_0_10,
        );
        // N s_0_13: return
        return;
    }
}
