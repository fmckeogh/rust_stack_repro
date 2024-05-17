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
use CreateAccDescExLDST::*;
use Mem_with_type_set::*;
use common::*;
pub fn MemA_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i128,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u32,
        size: i128,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        address,
        size,
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
        // C s_0_1: const #0u : u8
        let s_0_1: bool = false;
        // C s_0_2: const #1u : u32
        let s_0_2: u32 = 1;
        // S s_0_3: call CreateAccDescExLDST(s_0_2, s_0_0, s_0_1)
        let s_0_3: ProductType9878976b5bcce9c9 = CreateAccDescExLDST(
            state,
            tracer,
            s_0_2,
            s_0_0,
            s_0_1,
        );
        // D s_0_4: read-var address:u32
        let s_0_4: u32 = fn_state.address;
        // D s_0_5: read-var size:i
        let s_0_5: i128 = fn_state.size;
        // D s_0_6: read-var value_name:bv
        let s_0_6: Bits = fn_state.value_name;
        // D s_0_7: call Mem_with_type_set(s_0_4, s_0_5, s_0_3, s_0_6)
        let s_0_7: () = Mem_with_type_set(state, tracer, s_0_4, s_0_5, s_0_3, s_0_6);
        // N s_0_8: return
        return;
    }
}