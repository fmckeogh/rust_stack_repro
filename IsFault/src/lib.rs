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
pub fn IsFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    addrdesc: ProductTypece7c66ccb2cab13e,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_9134: ProductType1d757adad216cdef,
        addrdesc: ProductTypece7c66ccb2cab13e,
    }
    let fn_state = FunctionState {
        addrdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var addrdesc.0:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_0_1: write-var ga#9134 <= s_0_0
        fn_state.ga_9134 = s_0_0;
        // D s_0_2: read-var ga#9134.16:struct
        let s_0_2: u32 = fn_state.ga_9134._16;
        // C s_0_3: const #0u : u32
        let s_0_3: u32 = 0;
        // D s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // N s_0_5: return s_0_4
        return s_0_4;
    }
}
