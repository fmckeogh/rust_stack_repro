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
pub fn CreatePhysMemRetStatus<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: u32,
) -> ProductTypef8c3639b88223255 {
    #[derive(Default)]
    struct FunctionState {
        fault: u32,
    }
    let fn_state = FunctionState {
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // C s_0_0: const #64s : i
        let s_0_0: i128 = 64;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u64
        let s_0_2: u64 = (s_0_1.value() as u64);
        // C s_0_3: const #0u : u8
        let s_0_3: bool = false;
        // C s_0_4: const #5u : u32
        let s_0_4: u32 = 5;
        // D s_0_5: read-var fault:u32
        let s_0_5: u32 = fn_state.fault;
        // D s_0_6: create-product struct = ["s_0_3", "s_0_4", "s_0_5", "s_0_2"]
        let s_0_6: ProductTypef8c3639b88223255 = ProductTypef8c3639b88223255 {
            _0: s_0_3,
            _1: s_0_4,
            _2: s_0_5,
            _3: s_0_2,
        };
        // N s_0_7: return s_0_6
        return s_0_6;
    }
}
