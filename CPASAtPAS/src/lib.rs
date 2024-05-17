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
pub fn CPASAtPAS<T: Tracer>(state: &mut State, tracer: &T, pas: u32) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        ga_4971: u32,
        return_value: u32,
        pas: u32,
    }
    let fn_state = FunctionState {
        pas,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: read-var pas:u32
        let s_0_1: u32 = fn_state.pas;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // D s_1_1: write-var return_value <= s_1_0
        fn_state.return_value = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var return_value:u32
        let s_2_0: u32 = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #1u : u32
        let s_3_0: u32 = 1;
        // D s_3_1: read-var pas:u32
        let s_3_1: u32 = fn_state.pas;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_4_0: const #6u : u32
        let s_4_0: u32 = 6;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_5_0: const #2u : u32
        let s_5_0: u32 = 2;
        // D s_5_1: read-var pas:u32
        let s_5_1: u32 = fn_state.pas;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_6_0: const #4u : u32
        let s_6_0: u32 = 4;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #3u : u32
        let s_7_0: u32 = 3;
        // D s_7_1: read-var pas:u32
        let s_7_1: u32 = fn_state.pas;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_8_0: const #3u : u32
        let s_8_0: u32 = 3;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_9_0: read-var ga#4971:u32
        let s_9_0: u32 = fn_state.ga_4971;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
