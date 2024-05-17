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
use Mem_with_type_read::*;
use CreateAccDescExLDST::*;
use common::*;
pub fn MemA_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i64,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        ga_24298: Bits,
        address: u32,
        size: i64,
    }
    let fn_state = FunctionState {
        address,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // C s_0_1: const #0u : u8
        let s_0_1: bool = false;
        // C s_0_2: const #0u : u32
        let s_0_2: u32 = 0;
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
        // D s_0_5: read-var size:i64
        let s_0_5: i64 = fn_state.size;
        // D s_0_6: call Mem_with_type_read(s_0_4, s_0_5, s_0_3)
        let s_0_6: Bits = Mem_with_type_read(state, tracer, s_0_4, s_0_5, s_0_3);
        // D s_0_7: write-var ga#24298 <= s_0_6
        fn_state.ga_24298 = s_0_6;
        // N s_0_8: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var ga#24298:bv
        let s_1_0: Bits = fn_state.ga_24298;
        // N s_1_1: return s_1_0
        return s_1_0;
    }
}
