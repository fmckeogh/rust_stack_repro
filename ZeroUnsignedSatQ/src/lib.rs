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
use vector_update_subrange_from_integer_subrange::*;
use u__id::*;
use common::*;
pub fn ZeroUnsignedSatQ<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: i128,
    N: i128,
    M: i128,
) -> ProductTypef506aa96a892fe52 {
    #[derive(Default)]
    struct FunctionState {
        saturated: bool,
        result: i128,
        Mshadow_1004: i128,
        i: i128,
        N: i128,
        M: i128,
    }
    let fn_state = FunctionState {
        i,
        N,
        M,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef506aa96a892fe52 {
        // D s_0_0: read-var M:i
        let s_0_0: i128 = fn_state.M;
        // D s_0_1: write-var Mshadow#1004 <= s_0_0
        fn_state.Mshadow_1004 = s_0_0;
        // D s_0_2: read-var N:i
        let s_0_2: i128 = fn_state.N;
        // D s_0_3: pow2 s_0_2
        let s_0_3: i128 = (s_0_2).pow(2);
        // C s_0_4: const #1s : i
        let s_0_4: i128 = 1;
        // D s_0_5: sub s_0_3 s_0_4
        let s_0_5: i128 = ((s_0_3) - (s_0_4));
        // D s_0_6: read-var i:i
        let s_0_6: i128 = fn_state.i;
        // D s_0_7: cmp-gt s_0_6 s_0_5
        let s_0_7: bool = ((s_0_6) > (s_0_5));
        // N s_0_8: branch s_0_7 b5 b1
        if s_0_7 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef506aa96a892fe52 {
        // C s_1_0: const #0s : i
        let s_1_0: i128 = 0;
        // D s_1_1: read-var i:i
        let s_1_1: i128 = fn_state.i;
        // D s_1_2: cmp-lt s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) < (s_1_0));
        // N s_1_3: branch s_1_2 b4 b2
        if s_1_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef506aa96a892fe52 {
        // D s_2_0: read-var i:i
        let s_2_0: i128 = fn_state.i;
        // D s_2_1: write-var result <= s_2_0
        fn_state.result = s_2_0;
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // D s_2_3: write-var saturated <= s_2_2
        fn_state.saturated = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef506aa96a892fe52 {
        // D s_3_0: read-var result:i
        let s_3_0: i128 = fn_state.result;
        // D s_3_1: read-var Mshadow#1004:i
        let s_3_1: i128 = fn_state.Mshadow_1004;
        // D s_3_2: call __id(s_3_1)
        let s_3_2: i128 = u__id(state, tracer, s_3_1);
        // D s_3_3: call Zeros(s_3_2)
        let s_3_3: Bits = Zeros(state, tracer, s_3_2);
        // D s_3_4: read-var Mshadow#1004:i
        let s_3_4: i128 = fn_state.Mshadow_1004;
        // D s_3_5: call __id(s_3_4)
        let s_3_5: i128 = u__id(state, tracer, s_3_4);
        // C s_3_6: const #1s : i
        let s_3_6: i128 = 1;
        // D s_3_7: read-var N:i
        let s_3_7: i128 = fn_state.N;
        // D s_3_8: sub s_3_7 s_3_6
        let s_3_8: i128 = ((s_3_7) - (s_3_6));
        // C s_3_9: const #1s : i
        let s_3_9: i128 = 1;
        // D s_3_10: read-var N:i
        let s_3_10: i128 = fn_state.N;
        // D s_3_11: sub s_3_10 s_3_9
        let s_3_11: i128 = ((s_3_10) - (s_3_9));
        // C s_3_12: const #0s : i
        let s_3_12: i128 = 0;
        // C s_3_13: const #0s : i
        let s_3_13: i128 = 0;
        // D s_3_14: call vector_update_subrange_from_integer_subrange(s_3_5, s_3_3, s_3_8, s_3_12, s_3_0, s_3_11, s_3_13)
        let s_3_14: Bits = vector_update_subrange_from_integer_subrange(
            state,
            tracer,
            s_3_5,
            s_3_3,
            s_3_8,
            s_3_12,
            s_3_0,
            s_3_11,
            s_3_13,
        );
        // D s_3_15: read-var saturated:u8
        let s_3_15: bool = fn_state.saturated;
        // D s_3_16: create-product struct = ["s_3_14", "s_3_15"]
        let s_3_16: ProductTypef506aa96a892fe52 = ProductTypef506aa96a892fe52 {
            _0: s_3_14,
            _1: s_3_15,
        };
        // N s_3_17: return s_3_16
        return s_3_16;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef506aa96a892fe52 {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: write-var result <= s_4_0
        fn_state.result = s_4_0;
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // D s_4_3: write-var saturated <= s_4_2
        fn_state.saturated = s_4_2;
        // N s_4_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef506aa96a892fe52 {
        // D s_5_0: read-var N:i
        let s_5_0: i128 = fn_state.N;
        // D s_5_1: pow2 s_5_0
        let s_5_1: i128 = (s_5_0).pow(2);
        // C s_5_2: const #1s : i
        let s_5_2: i128 = 1;
        // D s_5_3: sub s_5_1 s_5_2
        let s_5_3: i128 = ((s_5_1) - (s_5_2));
        // D s_5_4: write-var result <= s_5_3
        fn_state.result = s_5_3;
        // C s_5_5: const #1u : u8
        let s_5_5: bool = true;
        // D s_5_6: write-var saturated <= s_5_5
        fn_state.saturated = s_5_5;
        // N s_5_7: jump b3
        return block_3(state, tracer, fn_state);
    }
}
