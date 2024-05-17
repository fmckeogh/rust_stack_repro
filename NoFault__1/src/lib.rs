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
use GPCNoFault::*;
use common::*;
pub fn NoFault__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        fault: ProductType1d757adad216cdef,
        gs_10022: bool,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: write-var fault.16 <= s_0_0
        fn_state.fault._16 = s_0_0;
        // D s_0_2: read-var accdesc:struct
        let s_0_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_3: write-var fault.0 <= s_0_2
        fn_state.fault._0 = s_0_2;
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // D s_0_5: write-var fault.15 <= s_0_4
        fn_state.fault._15 = s_0_4;
        // C s_0_6: const #0u : u8
        let s_0_6: bool = false;
        // D s_0_7: write-var fault.14 <= s_0_6
        fn_state.fault._14 = s_0_6;
        // C s_0_8: const #0u : u8
        let s_0_8: bool = false;
        // D s_0_9: write-var fault.3 <= s_0_8
        fn_state.fault._3 = s_0_8;
        // C s_0_10: const #0u : u8
        let s_0_10: bool = false;
        // D s_0_11: write-var fault.11 <= s_0_10
        fn_state.fault._11 = s_0_10;
        // C s_0_12: const #0u : u8
        let s_0_12: bool = false;
        // D s_0_13: write-var fault.18 <= s_0_12
        fn_state.fault._18 = s_0_12;
        // C s_0_14: const #0u : u8
        let s_0_14: bool = false;
        // D s_0_15: write-var fault.1 <= s_0_14
        fn_state.fault._1 = s_0_14;
        // C s_0_16: const #0u : u8
        let s_0_16: bool = false;
        // D s_0_17: write-var fault.13 <= s_0_16
        fn_state.fault._13 = s_0_16;
        // C s_0_18: const #0u : u8
        let s_0_18: bool = false;
        // D s_0_19: write-var fault.17 <= s_0_18
        fn_state.fault._17 = s_0_18;
        // D s_0_20: read-var accdesc.23:struct
        let s_0_20: bool = fn_state.accdesc._23;
        // D s_0_21: not s_0_20
        let s_0_21: bool = !s_0_20;
        // N s_0_22: branch s_0_21 b3 b1
        if s_0_21 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#10022 <= s_1_0
        fn_state.gs_10022 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_2_0: read-var gs#10022:u8
        let s_2_0: bool = fn_state.gs_10022;
        // D s_2_1: write-var fault.19 <= s_2_0
        fn_state.fault._19 = s_2_0;
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // D s_2_3: write-var fault.7 <= s_2_2
        fn_state.fault._7 = s_2_2;
        // C s_2_4: const #() : ()
        let s_2_4: () = ();
        // S s_2_5: call GPCNoFault(s_2_4)
        let s_2_5: ProductType396b95aa74979079 = GPCNoFault(state, tracer, s_2_4);
        // D s_2_6: write-var fault.6 <= s_2_5
        fn_state.fault._6 = s_2_5;
        // D s_2_7: read-var fault:struct
        let s_2_7: ProductType1d757adad216cdef = fn_state.fault;
        // N s_2_8: return s_2_7
        return s_2_7;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_3_0: read-var accdesc.32:struct
        let s_3_0: bool = fn_state.accdesc._32;
        // D s_3_1: write-var gs#10022 <= s_3_0
        fn_state.gs_10022 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
