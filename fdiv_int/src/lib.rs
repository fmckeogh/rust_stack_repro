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
pub fn fdiv_int<T: Tracer>(state: &mut State, tracer: &T, n: i128, m: i128) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_4: bool,
        gs_7: bool,
        return_value: i128,
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
    ) -> i128 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var n:i
        let s_0_1: i128 = fn_state.n;
        // D s_0_2: cmp-lt s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) < (s_0_0));
        // N s_0_3: branch s_0_2 b11 b1
        if s_0_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#4 <= s_1_0
        fn_state.gs_4 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var gs#4:u8
        let s_2_0: bool = fn_state.gs_4;
        // N s_2_1: branch s_2_0 b10 b3
        if s_2_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var n:i
        let s_3_1: i128 = fn_state.n;
        // D s_3_2: cmp-gt s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) > (s_3_0));
        // N s_3_3: branch s_3_2 b9 b4
        if s_3_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#7 <= s_4_0
        fn_state.gs_7 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_5_0: read-var gs#7:u8
        let s_5_0: bool = fn_state.gs_7;
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_6_0: read-var n:i
        let s_6_0: i128 = fn_state.n;
        // D s_6_1: read-var m:i
        let s_6_1: i128 = fn_state.m;
        // D s_6_2: div s_6_0 s_6_1
        let s_6_2: i128 = ((s_6_0) / (s_6_1));
        // D s_6_3: write-var return_value <= s_6_2
        fn_state.return_value = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_7_0: read-var return_value:i
        let s_7_0: i128 = fn_state.return_value;
        // N s_7_1: return s_7_0
        return s_7_0;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_8_0: const #1s : i
        let s_8_0: i128 = 1;
        // D s_8_1: read-var n:i
        let s_8_1: i128 = fn_state.n;
        // D s_8_2: sub s_8_1 s_8_0
        let s_8_2: i128 = ((s_8_1) - (s_8_0));
        // D s_8_3: read-var m:i
        let s_8_3: i128 = fn_state.m;
        // D s_8_4: div s_8_2 s_8_3
        let s_8_4: i128 = ((s_8_2) / (s_8_3));
        // C s_8_5: const #1s : i
        let s_8_5: i128 = 1;
        // D s_8_6: sub s_8_4 s_8_5
        let s_8_6: i128 = ((s_8_4) - (s_8_5));
        // D s_8_7: write-var return_value <= s_8_6
        fn_state.return_value = s_8_6;
        // N s_8_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: read-var m:i
        let s_9_1: i128 = fn_state.m;
        // D s_9_2: cmp-lt s_9_1 s_9_0
        let s_9_2: bool = ((s_9_1) < (s_9_0));
        // D s_9_3: write-var gs#7 <= s_9_2
        fn_state.gs_7 = s_9_2;
        // N s_9_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_10_0: const #1s : i
        let s_10_0: i128 = 1;
        // D s_10_1: read-var n:i
        let s_10_1: i128 = fn_state.n;
        // D s_10_2: add s_10_1 s_10_0
        let s_10_2: i128 = (s_10_1 + s_10_0);
        // D s_10_3: read-var m:i
        let s_10_3: i128 = fn_state.m;
        // D s_10_4: div s_10_2 s_10_3
        let s_10_4: i128 = ((s_10_2) / (s_10_3));
        // C s_10_5: const #1s : i
        let s_10_5: i128 = 1;
        // D s_10_6: sub s_10_4 s_10_5
        let s_10_6: i128 = ((s_10_4) - (s_10_5));
        // D s_10_7: write-var return_value <= s_10_6
        fn_state.return_value = s_10_6;
        // N s_10_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var m:i
        let s_11_1: i128 = fn_state.m;
        // D s_11_2: cmp-gt s_11_1 s_11_0
        let s_11_2: bool = ((s_11_1) > (s_11_0));
        // D s_11_3: write-var gs#4 <= s_11_2
        fn_state.gs_4 = s_11_2;
        // N s_11_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
