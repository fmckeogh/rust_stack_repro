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
pub fn sail_ones<T: Tracer>(state: &mut State, tracer: &T, n: i128) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        n: i128,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var n:i
        let s_0_0: i128 = fn_state.n;
        // C s_0_1: const #0u : u8
        let s_0_1: u8 = 0;
        // C s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 8u16);
        // D s_0_3: bits-cast zx s_0_2 -> bv length s_0_0
        let s_0_3: Bits = s_0_2.zero_extend(s_0_0);
        // D s_0_4: not s_0_3
        let s_0_4: Bits = !s_0_3;
        // N s_0_5: return s_0_4
        return s_0_4;
    }
}
