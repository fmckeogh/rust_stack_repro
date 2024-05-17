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
use common::*;
pub fn pa_bits<T: Tracer>(state: &mut State, tracer: &T, bv: u64) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        bv: u64,
    }
    let fn_state = FunctionState {
        bv,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #64s : i
        let s_0_0: i128 = 64;
        // D s_0_1: read-var bv:u56
        let s_0_1: u64 = fn_state.bv;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 56u16);
        // D s_0_3: bits-cast zx s_0_2 -> bv length s_0_0
        let s_0_3: Bits = s_0_2.zero_extend(s_0_0);
        // D s_0_4: cast reint s_0_3 -> u64
        let s_0_4: u64 = (s_0_3.value() as u64);
        // N s_0_5: return s_0_4
        return s_0_4;
    }
}
