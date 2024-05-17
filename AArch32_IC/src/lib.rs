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
use u__UNKNOWN_bits::*;
use AArch32_IC__1::*;
use common::*;
pub fn AArch32_IC<T: Tracer>(state: &mut State, tracer: &T, opscope: u32) -> () {
    #[derive(Default)]
    struct FunctionState {
        opscope: u32,
    }
    let fn_state = FunctionState {
        opscope,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #32s : i64
        let s_0_0: i64 = 32;
        // C s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // S s_0_2: call __UNKNOWN_bits(s_0_1)
        let s_0_2: Bits = u__UNKNOWN_bits(state, tracer, s_0_1);
        // S s_0_3: cast reint s_0_2 -> u32
        let s_0_3: u32 = (s_0_2.value() as u32);
        // D s_0_4: read-var opscope:u32
        let s_0_4: u32 = fn_state.opscope;
        // D s_0_5: call AArch32_IC__1(s_0_3, s_0_4)
        let s_0_5: () = AArch32_IC__1(state, tracer, s_0_3, s_0_4);
        // N s_0_6: return
        return;
    }
}
