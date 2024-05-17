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
use extsv::*;
use u__id::*;
use common::*;
pub fn zext_ones<T: Tracer>(state: &mut State, tracer: &T, n: i128, m: i128) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        n: i128,
        m: i128,
    }
    let fn_state = FunctionState {
        n,
        m,
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
        // D s_0_1: call __id(s_0_0)
        let s_0_1: i128 = u__id(state, tracer, s_0_0);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: call extsv(s_0_1, s_0_3)
        let s_0_4: Bits = extsv(state, tracer, s_0_1, s_0_3);
        // D s_0_5: read-var n:i
        let s_0_5: i128 = fn_state.n;
        // D s_0_6: read-var m:i
        let s_0_6: i128 = fn_state.m;
        // D s_0_7: sub s_0_5 s_0_6
        let s_0_7: i128 = ((s_0_5) - (s_0_6));
        // D s_0_8: lsr s_0_4 s_0_7
        let s_0_8: Bits = s_0_4 >> s_0_7;
        // N s_0_9: return s_0_8
        return s_0_8;
    }
}
