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
pub fn integer_subrange<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: i128,
    hi: i128,
    lo: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        i: i128,
        hi: i128,
        lo: i128,
    }
    let fn_state = FunctionState {
        i,
        hi,
        lo,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var hi:i
        let s_0_0: i128 = fn_state.hi;
        // D s_0_1: read-var lo:i
        let s_0_1: i128 = fn_state.lo;
        // D s_0_2: sub s_0_0 s_0_1
        let s_0_2: i128 = ((s_0_0) - (s_0_1));
        // C s_0_3: const #1s : i
        let s_0_3: i128 = 1;
        // D s_0_4: add s_0_2 s_0_3
        let s_0_4: i128 = (s_0_2 + s_0_3);
        // D s_0_5: read-var i:i
        let s_0_5: i128 = fn_state.i;
        // D s_0_6: read-var lo:i
        let s_0_6: i128 = fn_state.lo;
        // D s_0_7: bit-extract s_0_5 s_0_6 s_0_4
        let s_0_7: i128 = ((s_0_5 >> s_0_6)
            & ((1 as i128).checked_shl(s_0_4 as u32).map(|x| x - 1).unwrap_or(!0)));
        // D s_0_8: cast reint s_0_7 -> u128
        let s_0_8: u128 = (s_0_7 as u128);
        // D s_0_9: cast reint s_0_4 -> u16
        let s_0_9: u16 = (s_0_4 as u16);
        // D s_0_10: create-bits s_0_8 s_0_9
        let s_0_10: Bits = Bits::new(s_0_8, s_0_9);
        // N s_0_11: return s_0_10
        return s_0_10;
    }
}
