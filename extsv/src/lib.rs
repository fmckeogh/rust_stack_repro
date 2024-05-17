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
pub fn extsv<T: Tracer>(state: &mut State, tracer: &T, m: i128, v: Bits) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        return_value: Bits,
        m: i128,
        v: Bits,
    }
    let fn_state = FunctionState {
        m,
        v,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var v:bv
        let s_0_0: Bits = fn_state.v;
        // D s_0_1: size-of s_0_0
        let s_0_1: u16 = s_0_0.length();
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: read-var m:i
        let s_0_3: i128 = fn_state.m;
        // D s_0_4: cmp-lt s_0_3 s_0_2
        let s_0_4: bool = ((s_0_3) < (s_0_2));
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var v:bv
        let s_1_0: Bits = fn_state.v;
        // D s_1_1: read-var m:i
        let s_1_1: i128 = fn_state.m;
        // D s_1_2: bits-cast sx s_1_0 -> bv length s_1_1
        let s_1_2: Bits = s_1_0.sign_extend(s_1_1);
        // D s_1_3: write-var return_value <= s_1_2
        fn_state.return_value = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var return_value:bv
        let s_2_0: Bits = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_3_0: read-var v:bv
        let s_3_0: Bits = fn_state.v;
        // D s_3_1: read-var m:i
        let s_3_1: i128 = fn_state.m;
        // D s_3_2: bits-cast trunc s_3_0 -> bv length s_3_1
        let s_3_2: Bits = s_3_0.truncate(s_3_1);
        // D s_3_3: write-var return_value <= s_3_2
        fn_state.return_value = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
