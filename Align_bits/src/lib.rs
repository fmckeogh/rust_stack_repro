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
use Align_int::*;
use Slice_int::*;
use common::*;
pub fn Align_bits<T: Tracer>(state: &mut State, tracer: &T, x: Bits, y: i128) -> Bits {
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
    ) -> Bits {
        // D s_0_0: read-var x:bv
        let s_0_0: Bits = fn_state.x;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (s_0_0.value() as i128);
        // D s_0_2: read-var y:i
        let s_0_2: i128 = fn_state.y;
        // D s_0_3: call Align_int(s_0_1, s_0_2)
        let s_0_3: i128 = Align_int(state, tracer, s_0_1, s_0_2);
        // D s_0_4: read-var x:bv
        let s_0_4: Bits = fn_state.x;
        // D s_0_5: size-of s_0_4
        let s_0_5: u16 = s_0_4.length();
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #0s : i
        let s_0_7: i128 = 0;
        // D s_0_8: tail-call Slice_int(s_0_3, s_0_7, s_0_6)
        return Slice_int(state, tracer, s_0_3, s_0_7, s_0_6);
    }
}
