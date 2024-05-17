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
use IsExternalSyncAbort::*;
use common::*;
pub fn IsExternalSyncAbort__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: ProductType1d757adad216cdef,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_5981: ProductType396b95aa74979079,
        gs_8606: bool,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var fault.16:struct
        let s_0_0: u32 = fn_state.fault._16;
        // D s_0_1: call IsExternalSyncAbort(s_0_0)
        let s_0_1: bool = IsExternalSyncAbort(state, tracer, s_0_0);
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
        // D s_1_0: read-var fault.6:struct
        let s_1_0: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_1_1: write-var ga#5981 <= s_1_0
        fn_state.ga_5981 = s_1_0;
        // D s_1_2: read-var ga#5981.0:struct
        let s_1_2: u32 = fn_state.ga_5981._0;
        // C s_1_3: const #3u : u32
        let s_1_3: u32 = 3;
        // D s_1_4: cmp-eq s_1_2 s_1_3
        let s_1_4: bool = ((s_1_2) == (s_1_3));
        // D s_1_5: write-var gs#8606 <= s_1_4
        fn_state.gs_8606 = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#8606:u8
        let s_2_0: bool = fn_state.gs_8606;
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
        // D s_3_1: write-var gs#8606 <= s_3_0
        fn_state.gs_8606 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
