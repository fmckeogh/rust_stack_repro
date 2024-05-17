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
use Zeros::*;
use common::*;
pub fn SPMEVFILTR_EL0_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    s: i128,
    n: i128,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        s: i128,
        n: i128,
    }
    let fn_state = FunctionState {
        s,
        n,
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
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u64
        let s_0_2: u64 = (s_0_1.value() as u64);
        // N s_0_3: return s_0_2
        return s_0_2;
    }
}
