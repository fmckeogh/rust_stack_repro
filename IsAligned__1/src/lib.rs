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
use Align_bits::*;
use common::*;
pub fn IsAligned__1<T: Tracer>(state: &mut State, tracer: &T, x: Bits, y: i128) -> bool {
    #[derive(Default)]
    struct FunctionState {
        x: Bits,
        y: i128,
    }
    let fn_state = FunctionState {
        x,
        y,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var x:bv
        let s_0_0: Bits = fn_state.x;
        // D s_0_1: read-var y:i
        let s_0_1: i128 = fn_state.y;
        // D s_0_2: call Align_bits(s_0_0, s_0_1)
        let s_0_2: Bits = Align_bits(state, tracer, s_0_0, s_0_1);
        // D s_0_3: read-var x:bv
        let s_0_3: Bits = fn_state.x;
        // D s_0_4: cmp-eq s_0_3 s_0_2
        let s_0_4: bool = ((s_0_3) == (s_0_2));
        // N s_0_5: return s_0_4
        return s_0_4;
    }
}
