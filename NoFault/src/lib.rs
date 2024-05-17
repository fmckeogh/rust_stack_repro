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
use u__UNKNOWN_AccessDescriptor::*;
use common::*;
pub fn NoFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_10009: (),
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        fault: ProductType1d757adad216cdef,
        gs_10009: (),
    }
    let fn_state = FunctionState {
        gs_10009,
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
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call __UNKNOWN_AccessDescriptor(s_0_2)
        let s_0_3: ProductType9878976b5bcce9c9 = u__UNKNOWN_AccessDescriptor(
            state,
            tracer,
            s_0_2,
        );
        // D s_0_4: write-var fault.0 <= s_0_3
        fn_state.fault._0 = s_0_3;
        // C s_0_5: const #0u : u8
        let s_0_5: bool = false;
        // D s_0_6: write-var fault.15 <= s_0_5
        fn_state.fault._15 = s_0_5;
        // C s_0_7: const #0u : u8
        let s_0_7: bool = false;
        // D s_0_8: write-var fault.14 <= s_0_7
        fn_state.fault._14 = s_0_7;
        // C s_0_9: const #0u : u8
        let s_0_9: bool = false;
        // D s_0_10: write-var fault.3 <= s_0_9
        fn_state.fault._3 = s_0_9;
        // C s_0_11: const #0u : u8
        let s_0_11: bool = false;
        // D s_0_12: write-var fault.11 <= s_0_11
        fn_state.fault._11 = s_0_11;
        // C s_0_13: const #0u : u8
        let s_0_13: bool = false;
        // D s_0_14: write-var fault.18 <= s_0_13
        fn_state.fault._18 = s_0_13;
        // C s_0_15: const #0u : u8
        let s_0_15: bool = false;
        // D s_0_16: write-var fault.1 <= s_0_15
        fn_state.fault._1 = s_0_15;
        // C s_0_17: const #0u : u8
        let s_0_17: bool = false;
        // D s_0_18: write-var fault.13 <= s_0_17
        fn_state.fault._13 = s_0_17;
        // C s_0_19: const #0u : u8
        let s_0_19: bool = false;
        // D s_0_20: write-var fault.17 <= s_0_19
        fn_state.fault._17 = s_0_19;
        // C s_0_21: const #0u : u8
        let s_0_21: bool = false;
        // D s_0_22: write-var fault.7 <= s_0_21
        fn_state.fault._7 = s_0_21;
        // C s_0_23: const #() : ()
        let s_0_23: () = ();
        // S s_0_24: call GPCNoFault(s_0_23)
        let s_0_24: ProductType396b95aa74979079 = GPCNoFault(state, tracer, s_0_23);
        // D s_0_25: write-var fault.6 <= s_0_24
        fn_state.fault._6 = s_0_24;
        // D s_0_26: read-var fault:struct
        let s_0_26: ProductType1d757adad216cdef = fn_state.fault;
        // N s_0_27: return s_0_26
        return s_0_26;
    }
}
