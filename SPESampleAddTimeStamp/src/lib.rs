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
use GetTimestamp::*;
use CollectTimeStamp::*;
use common::*;
pub fn SPESampleAddTimeStamp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25585: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        timestamp: u32,
        gs_25585: (),
    }
    let fn_state = FunctionState {
        gs_25585,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CollectTimeStamp(s_0_0)
        let s_0_1: u32 = CollectTimeStamp(state, tracer, s_0_0);
        // D s_0_2: write-var timestamp <= s_0_1
        fn_state.timestamp = s_0_1;
        // C s_0_3: const #0u : u32
        let s_0_3: u32 = 0;
        // D s_0_4: read-var timestamp:u32
        let s_0_4: u32 = fn_state.timestamp;
        // D s_0_5: cmp-eq s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) == (s_0_4));
        // D s_0_6: not s_0_5
        let s_0_6: bool = !s_0_5;
        // N s_0_7: branch s_0_6 b2 b1
        if s_0_6 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // C s_1_1: const #17616u : u32
        let s_1_1: u32 = 17616;
        // N s_1_2: write-reg s_1_1 <= s_1_0
        let s_1_2: () = {
            state.write_register::<bool>(s_1_1 as isize, s_1_0);
            tracer.write_register(s_1_1 as isize, s_1_0);
        };
        // N s_1_3: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // C s_2_1: const #17616u : u32
        let s_2_1: u32 = 17616;
        // N s_2_2: write-reg s_2_1 <= s_2_0
        let s_2_2: () = {
            state.write_register::<bool>(s_2_1 as isize, s_2_0);
            tracer.write_register(s_2_1 as isize, s_2_0);
        };
        // D s_2_3: read-var timestamp:u32
        let s_2_3: u32 = fn_state.timestamp;
        // D s_2_4: call GetTimestamp(s_2_3)
        let s_2_4: u64 = GetTimestamp(state, tracer, s_2_3);
        // C s_2_5: const #13504u : u32
        let s_2_5: u32 = 13504;
        // N s_2_6: write-reg s_2_5 <= s_2_4
        let s_2_6: () = {
            state.write_register::<u64>(s_2_5 as isize, s_2_4);
            tracer.write_register(s_2_5 as isize, s_2_4);
        };
        // N s_2_7: return
        return;
    }
}
