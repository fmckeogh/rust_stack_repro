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
use CreateAccDescA32LSMD::*;
use Mem_with_type_read::*;
use common::*;
pub fn MemS_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i64,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        ga_24304: Bits,
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
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // S s_0_1: call CreateAccDescA32LSMD(s_0_0)
        let s_0_1: ProductType9878976b5bcce9c9 = CreateAccDescA32LSMD(
            state,
            tracer,
            s_0_0,
        );
        // D s_0_2: read-var address:u32
        let s_0_2: u32 = fn_state.address;
        // D s_0_3: read-var size:i64
        let s_0_3: i64 = fn_state.size;
        // D s_0_4: call Mem_with_type_read(s_0_2, s_0_3, s_0_1)
        let s_0_4: Bits = Mem_with_type_read(state, tracer, s_0_2, s_0_3, s_0_1);
        // D s_0_5: write-var ga#24304 <= s_0_4
        fn_state.ga_24304 = s_0_4;
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var ga#24304:bv
        let s_1_0: Bits = fn_state.ga_24304;
        // N s_1_1: return s_1_0
        return s_1_0;
    }
}
