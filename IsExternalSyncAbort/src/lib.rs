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
pub fn IsExternalSyncAbort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    statuscode: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_8603: bool,
        gs_8604: bool,
        gs_8605: bool,
        statuscode: u32,
    }
    let fn_state = FunctionState {
        statuscode,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var statuscode:u32
        let s_0_0: u32 = fn_state.statuscode;
        // C s_0_1: const #0u : u32
        let s_0_1: u32 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var statuscode:u32
        let s_0_4: u32 = fn_state.statuscode;
        // C s_0_5: const #8u : u32
        let s_0_5: u32 = 8;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // N s_0_7: branch s_0_6 b9 b1
        if s_0_6 {
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
        // D s_1_0: read-var statuscode:u32
        let s_1_0: u32 = fn_state.statuscode;
        // C s_1_1: const #10u : u32
        let s_1_1: u32 = 10;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b8 b2
        if s_1_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var statuscode:u32
        let s_2_0: u32 = fn_state.statuscode;
        // C s_2_1: const #9u : u32
        let s_2_1: u32 = 9;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var statuscode:u32
        let s_3_0: u32 = fn_state.statuscode;
        // C s_3_1: const #11u : u32
        let s_3_1: u32 = 11;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: write-var gs#8603 <= s_3_2
        fn_state.gs_8603 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#8603:u8
        let s_4_0: bool = fn_state.gs_8603;
        // D s_4_1: write-var gs#8604 <= s_4_0
        fn_state.gs_8604 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#8604:u8
        let s_5_0: bool = fn_state.gs_8604;
        // D s_5_1: write-var gs#8605 <= s_5_0
        fn_state.gs_8605 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#8605:u8
        let s_6_0: bool = fn_state.gs_8605;
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
        // D s_7_1: write-var gs#8603 <= s_7_0
        fn_state.gs_8603 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#8604 <= s_8_0
        fn_state.gs_8604 = s_8_0;
        // N s_8_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#8605 <= s_9_0
        fn_state.gs_8605 = s_9_0;
        // N s_9_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
