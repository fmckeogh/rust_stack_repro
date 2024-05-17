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
pub fn HasUnprivileged<T: Tracer>(state: &mut State, tracer: &T, regime: u32) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_13506: bool,
        gs_13505: bool,
        regime: u32,
    }
    let fn_state = FunctionState {
        regime,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var regime:u32
        let s_0_0: u32 = fn_state.regime;
        // C s_0_1: const #3u : u32
        let s_0_1: u32 = 3;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b6 b1
        if s_0_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var regime:u32
        let s_1_0: u32 = fn_state.regime;
        // C s_1_1: const #1u : u32
        let s_1_1: u32 = 1;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b5 b2
        if s_1_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var regime:u32
        let s_2_0: u32 = fn_state.regime;
        // C s_2_1: const #4u : u32
        let s_2_1: u32 = 4;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // D s_2_3: write-var gs#13505 <= s_2_2
        fn_state.gs_13505 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#13505:u8
        let s_3_0: bool = fn_state.gs_13505;
        // D s_3_1: write-var gs#13506 <= s_3_0
        fn_state.gs_13506 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#13506:u8
        let s_4_0: bool = fn_state.gs_13506;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#13505 <= s_5_0
        fn_state.gs_13505 = s_5_0;
        // N s_5_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#13506 <= s_6_0
        fn_state.gs_13506 = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
