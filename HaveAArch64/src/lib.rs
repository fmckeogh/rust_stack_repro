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
use IsFeatureImplemented::*;
use common::*;
pub fn HaveAArch64<T: Tracer>(state: &mut State, tracer: &T, gs_1644: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_1647: bool,
        gs_1646: bool,
        gs_1645: bool,
        gs_1644: (),
    }
    let fn_state = FunctionState {
        gs_1644,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #4u : u32
        let s_0_0: u32 = 4;
        // S s_0_1: call IsFeatureImplemented(s_0_0)
        let s_0_1: bool = IsFeatureImplemented(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b9 b1
        if s_0_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #5u : u32
        let s_1_0: u32 = 5;
        // S s_1_1: call IsFeatureImplemented(s_1_0)
        let s_1_1: bool = IsFeatureImplemented(state, tracer, s_1_0);
        // D s_1_2: write-var gs#1645 <= s_1_1
        fn_state.gs_1645 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#1645:u8
        let s_2_0: bool = fn_state.gs_1645;
        // N s_2_1: branch s_2_0 b8 b3
        if s_2_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #6u : u32
        let s_3_0: u32 = 6;
        // S s_3_1: call IsFeatureImplemented(s_3_0)
        let s_3_1: bool = IsFeatureImplemented(state, tracer, s_3_0);
        // D s_3_2: write-var gs#1646 <= s_3_1
        fn_state.gs_1646 = s_3_1;
        // N s_3_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#1646:u8
        let s_4_0: bool = fn_state.gs_1646;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #7u : u32
        let s_5_0: u32 = 7;
        // S s_5_1: call IsFeatureImplemented(s_5_0)
        let s_5_1: bool = IsFeatureImplemented(state, tracer, s_5_0);
        // D s_5_2: write-var gs#1647 <= s_5_1
        fn_state.gs_1647 = s_5_1;
        // N s_5_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#1647:u8
        let s_6_0: bool = fn_state.gs_1647;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#1647 <= s_7_0
        fn_state.gs_1647 = s_7_0;
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#1646 <= s_8_0
        fn_state.gs_1646 = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#1645 <= s_9_0
        fn_state.gs_1645 = s_9_0;
        // N s_9_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
