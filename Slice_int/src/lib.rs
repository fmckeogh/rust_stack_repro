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
pub fn Slice_int<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: i128,
    l: i128,
    n: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        i: i128,
        l: i128,
        n: i128,
    }
    let fn_state = FunctionState {
        i,
        l,
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
        // D s_0_1: read-var i:i
        let s_0_1: i128 = fn_state.i;
        // D s_0_2: read-var l:i
        let s_0_2: i128 = fn_state.l;
        // D s_0_3: bit-extract s_0_1 s_0_2 s_0_0
        let s_0_3: i128 = ((s_0_1 >> s_0_2)
            & ((1 as i128).checked_shl(s_0_0 as u32).map(|x| x - 1).unwrap_or(!0)));
        // D s_0_4: cast reint s_0_3 -> u128
        let s_0_4: u128 = (s_0_3 as u128);
        // D s_0_5: cast reint s_0_0 -> u16
        let s_0_5: u16 = (s_0_0 as u16);
        // D s_0_6: create-bits s_0_4 s_0_5
        let s_0_6: Bits = Bits::new(s_0_4, s_0_5);
        // N s_0_7: return s_0_6
        return s_0_6;
    }
}
