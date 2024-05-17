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
use u__IMPDEF_boolean::*;
use HaveDoubleFaultExt::*;
use common::*;
pub fn IsSErrorEdgeTriggered<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_10060: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        gs_10060: (),
    }
    let fn_state = FunctionState {
        gs_10060,
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
        // S s_0_1: call HaveDoubleFaultExt(s_0_0)
        let s_0_1: bool = HaveDoubleFaultExt(state, tracer, s_0_0);
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
        // C s_1_0: const #"Edge-triggered SError" : str
        let s_1_0: &'static str = "Edge-triggered SError";
        // S s_1_1: call __IMPDEF_boolean(s_1_0)
        let s_1_1: bool = u__IMPDEF_boolean(state, tracer, s_1_0);
        // D s_1_2: write-var return_value <= s_1_1
        fn_state.return_value = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var return_value:u8
        let s_2_0: bool = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var return_value <= s_3_0
        fn_state.return_value = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
