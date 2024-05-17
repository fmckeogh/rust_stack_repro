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
use HaveBlockBBM::*;
use u__UNKNOWN_integer::*;
use u__IMPDEF_integer::*;
use common::*;
pub fn AArch64_BlockBBMSupportLevel<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_17519: (),
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        return_value: i128,
        gs_17519: (),
    }
    let fn_state = FunctionState {
        gs_17519,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveBlockBBM(s_0_0)
        let s_0_1: bool = HaveBlockBBM(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
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
    ) -> i128 {
        // C s_1_0: const #"Block BBM support level" : str
        let s_1_0: &'static str = "Block BBM support level";
        // S s_1_1: call __IMPDEF_integer(s_1_0)
        let s_1_1: i128 = u__IMPDEF_integer(state, tracer, s_1_0);
        // D s_1_2: write-var return_value <= s_1_1
        fn_state.return_value = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var return_value:i
        let s_2_0: i128 = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call __UNKNOWN_integer(s_3_0)
        let s_3_1: i128 = u__UNKNOWN_integer(state, tracer, s_3_0);
        // D s_3_2: write-var return_value <= s_3_1
        fn_state.return_value = s_3_1;
        // N s_3_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
