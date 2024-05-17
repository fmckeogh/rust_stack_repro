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
use CreateAccDescGPR::*;
use Mem_with_type_set::*;
use common::*;
pub fn MemU_unpriv_set<T: Tracer>(
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
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // C s_0_3: const #1u : u32
        let s_0_3: u32 = 1;
        // S s_0_4: call CreateAccDescGPR(s_0_3, s_0_0, s_0_1, s_0_2)
        let s_0_4: ProductType9878976b5bcce9c9 = CreateAccDescGPR(
            state,
            tracer,
            s_0_3,
            s_0_0,
            s_0_1,
            s_0_2,
        );
        // D s_0_5: read-var address:u32
        let s_0_5: u32 = fn_state.address;
        // D s_0_6: read-var size:i
        let s_0_6: i128 = fn_state.size;
        // D s_0_7: read-var value_name:bv
        let s_0_7: Bits = fn_state.value_name;
        // D s_0_8: call Mem_with_type_set(s_0_5, s_0_6, s_0_4, s_0_7)
        let s_0_8: () = Mem_with_type_set(state, tracer, s_0_5, s_0_6, s_0_4, s_0_7);
        // N s_0_9: return
        return;
    }
}
