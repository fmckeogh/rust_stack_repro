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
use NoFault::*;
use common::*;
pub fn CreateAddressDescriptor<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u64,
    pa: ProductTypeda0231e9dc169f81,
    memattrs: ProductTypef170cab34335b70c,
) -> ProductTypece7c66ccb2cab13e {
    #[derive(Default)]
    struct FunctionState {
        addrdesc: ProductTypece7c66ccb2cab13e,
        va: u64,
        pa: ProductTypeda0231e9dc169f81,
        memattrs: ProductTypef170cab34335b70c,
    }
    let fn_state = FunctionState {
        va,
        pa,
        memattrs,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_0_0: read-var pa:struct
        let s_0_0: ProductTypeda0231e9dc169f81 = fn_state.pa;
        // D s_0_1: write-var addrdesc.3 <= s_0_0
        fn_state.addrdesc._3 = s_0_0;
        // D s_0_2: read-var va:u64
        let s_0_2: u64 = fn_state.va;
        // D s_0_3: write-var addrdesc.7 <= s_0_2
        fn_state.addrdesc._7 = s_0_2;
        // D s_0_4: read-var memattrs:struct
        let s_0_4: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_0_5: write-var addrdesc.2 <= s_0_4
        fn_state.addrdesc._2 = s_0_4;
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call NoFault(s_0_6)
        let s_0_7: ProductType1d757adad216cdef = NoFault(state, tracer, s_0_6);
        // D s_0_8: write-var addrdesc.0 <= s_0_7
        fn_state.addrdesc._0 = s_0_7;
        // C s_0_9: const #0u : u8
        let s_0_9: bool = false;
        // D s_0_10: write-var addrdesc.4 <= s_0_9
        fn_state.addrdesc._4 = s_0_9;
        // D s_0_11: read-var addrdesc:struct
        let s_0_11: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // N s_0_12: return s_0_11
        return s_0_11;
    }
}
