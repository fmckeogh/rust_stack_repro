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
pub fn sail_mask<T: Tracer>(state: &mut State, tracer: &T, len: i128, v: Bits) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        lenshadow_0: i128,
        return_value: Bits,
        len: i128,
        v: Bits,
    }
    let fn_state = FunctionState {
        len,
        v,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var len:i
        let s_0_0: i128 = fn_state.len;
        // D s_0_1: write-var lenshadow#0 <= s_0_0
        fn_state.lenshadow_0 = s_0_0;
        // D s_0_2: read-var v:bv
        let s_0_2: Bits = fn_state.v;
        // D s_0_3: size-of s_0_2
        let s_0_3: u16 = s_0_2.length();
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: read-var lenshadow#0:i
        let s_0_5: i128 = fn_state.lenshadow_0;
        // D s_0_6: cmp-le s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) <= (s_0_4));
        // N s_0_7: branch s_0_6 b3 b1
        if s_0_6 {
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
        // D s_1_1: read-var lenshadow#0:i
        let s_1_1: i128 = fn_state.lenshadow_0;
        // D s_1_2: bits-cast zx s_1_0 -> bv length s_1_1
        let s_1_2: Bits = s_1_0.zero_extend(s_1_1);
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
        // D s_3_1: read-var lenshadow#0:i
        let s_3_1: i128 = fn_state.lenshadow_0;
        // D s_3_2: bits-cast trunc s_3_0 -> bv length s_3_1
        let s_3_2: Bits = s_3_0.truncate(s_3_1);
        // D s_3_3: write-var return_value <= s_3_2
        fn_state.return_value = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
