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
use AArch64_MemSingle_set__1::*;
use common::*;
pub fn AArch64_MemSingle_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    size: i64,
    accdesc: ProductType9878976b5bcce9c9,
    aligned: bool,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u64,
        size: i64,
        accdesc: ProductType9878976b5bcce9c9,
        aligned: bool,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc,
        aligned,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: read-var address:u64
        let s_0_1: u64 = fn_state.address;
        // D s_0_2: read-var size:i64
        let s_0_2: i64 = fn_state.size;
        // D s_0_3: read-var accdesc:struct
        let s_0_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_4: read-var aligned:u8
        let s_0_4: bool = fn_state.aligned;
        // D s_0_5: read-var value_name:bv
        let s_0_5: Bits = fn_state.value_name;
        // D s_0_6: call AArch64_MemSingle_set__1(s_0_1, s_0_2, s_0_3, s_0_4, s_0_0, s_0_5)
        let s_0_6: () = AArch64_MemSingle_set__1(
            state,
            tracer,
            s_0_1,
            s_0_2,
            s_0_3,
            s_0_4,
            s_0_0,
            s_0_5,
        );
        // N s_0_7: return
        return;
    }
}
