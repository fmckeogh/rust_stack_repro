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
use HaveIESB::*;
use u__IMPDEF_boolean::*;
use common::*;
pub fn InsertIESBBeforeException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_6340: bool,
        el: u8,
    }
    let fn_state = FunctionState {
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveIESB(s_0_0)
        let s_0_1: bool = HaveIESB(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#6340 <= s_1_0
        fn_state.gs_6340 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#6340:u8
        let s_2_0: bool = fn_state.gs_6340;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #"Has Implicit Error Synchronization Barrier before Exception" : str
        let s_3_0: &'static str = "Has Implicit Error Synchronization Barrier before Exception";
        // S s_3_1: call __IMPDEF_boolean(s_3_0)
        let s_3_1: bool = u__IMPDEF_boolean(state, tracer, s_3_0);
        // D s_3_2: write-var gs#6340 <= s_3_1
        fn_state.gs_6340 = s_3_1;
        // N s_3_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
