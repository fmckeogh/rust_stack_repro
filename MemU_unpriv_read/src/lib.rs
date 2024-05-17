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
use CreateAccDescGPR::*;
use common::*;
pub fn MemU_unpriv_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i64,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        ga_24302: Bits,
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
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // C s_0_3: const #0u : u32
        let s_0_3: u32 = 0;
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
        // D s_0_6: read-var size:i64
        let s_0_6: i64 = fn_state.size;
        // D s_0_7: call Mem_with_type_read(s_0_5, s_0_6, s_0_4)
        let s_0_7: Bits = Mem_with_type_read(state, tracer, s_0_5, s_0_6, s_0_4);
        // D s_0_8: write-var ga#24302 <= s_0_7
        fn_state.ga_24302 = s_0_7;
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var ga#24302:bv
        let s_1_0: Bits = fn_state.ga_24302;
        // N s_1_1: return s_1_0
        return s_1_0;
    }
}
