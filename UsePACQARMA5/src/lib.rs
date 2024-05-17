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
use HavePACQARMA5Auth::*;
use HavePACQARMA5Generic::*;
use common::*;
pub fn UsePACQARMA5<T: Tracer>(state: &mut State, tracer: &T, isgeneric: bool) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_10768: bool,
        isgeneric: bool,
    }
    let fn_state = FunctionState {
        isgeneric,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var isgeneric:u8
        let s_0_0: bool = fn_state.isgeneric;
        // N s_0_1: branch s_0_0 b3 b1
        if s_0_0 {
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HavePACQARMA5Auth(s_1_0)
        let s_1_1: bool = HavePACQARMA5Auth(state, tracer, s_1_0);
        // D s_1_2: write-var ga#10768 <= s_1_1
        fn_state.ga_10768 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var ga#10768:u8
        let s_2_0: bool = fn_state.ga_10768;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HavePACQARMA5Generic(s_3_0)
        let s_3_1: bool = HavePACQARMA5Generic(state, tracer, s_3_0);
        // D s_3_2: write-var ga#10768 <= s_3_1
        fn_state.ga_10768 = s_3_1;
        // N s_3_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
