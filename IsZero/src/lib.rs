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
pub fn IsZero<T: Tracer>(state: &mut State, tracer: &T, x: Bits) -> bool {
    #[derive(Default)]
    struct FunctionState {
        x: Bits,
    }
    let fn_state = FunctionState {
        x,
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
        // D s_0_1: size-of s_0_0
        let s_0_1: u16 = s_0_0.length();
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // C s_0_3: const #0u : u8
        let s_0_3: u8 = 0;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 8u16);
        // D s_0_5: bits-cast zx s_0_4 -> bv length s_0_2
        let s_0_5: Bits = s_0_4.zero_extend(s_0_2);
        // D s_0_6: read-var x:bv
        let s_0_6: Bits = fn_state.x;
        // D s_0_7: cmp-eq s_0_6 s_0_5
        let s_0_7: bool = ((s_0_6) == (s_0_5));
        // N s_0_8: return s_0_7
        return s_0_7;
    }
}
