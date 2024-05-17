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
use sail_ones::*;
use sail_mask::*;
use common::*;
pub fn slice_mask<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    i: i128,
    l: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        return_value: Bits,
        n: i128,
        i: i128,
        l: i128,
    }
    let fn_state = FunctionState {
        n,
        i,
        l,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var l:i
        let s_0_0: i128 = fn_state.l;
        // D s_0_1: read-var n:i
        let s_0_1: i128 = fn_state.n;
        // D s_0_2: cmp-ge s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) >= (s_0_1));
        // N s_0_3: branch s_0_2 b3 b1
        if s_0_2 {
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
        // D s_1_0: read-var n:i
        let s_1_0: i128 = fn_state.n;
        // C s_1_1: const #1u : u8
        let s_1_1: bool = true;
        // C s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 1u16);
        // D s_1_3: call sail_mask(s_1_0, s_1_2)
        let s_1_3: Bits = sail_mask(state, tracer, s_1_0, s_1_2);
        // D s_1_4: read-var l:i
        let s_1_4: i128 = fn_state.l;
        // D s_1_5: lsl s_1_3 s_1_4
        let s_1_5: Bits = s_1_3 << s_1_4;
        // D s_1_6: sub s_1_5 s_1_3
        let s_1_6: Bits = ((s_1_5) - (s_1_3));
        // D s_1_7: read-var i:i
        let s_1_7: i128 = fn_state.i;
        // D s_1_8: lsl s_1_6 s_1_7
        let s_1_8: Bits = s_1_6 << s_1_7;
        // D s_1_9: write-var return_value <= s_1_8
        fn_state.return_value = s_1_8;
        // N s_1_10: jump b2
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
        // D s_3_0: read-var n:i
        let s_3_0: i128 = fn_state.n;
        // D s_3_1: call sail_ones(s_3_0)
        let s_3_1: Bits = sail_ones(state, tracer, s_3_0);
        // D s_3_2: read-var i:i
        let s_3_2: i128 = fn_state.i;
        // D s_3_3: lsl s_3_1 s_3_2
        let s_3_3: Bits = s_3_1 << s_3_2;
        // D s_3_4: write-var return_value <= s_3_3
        fn_state.return_value = s_3_3;
        // N s_3_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
