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
use Mem_with_type_read__1::*;
use common::*;
pub fn Mem_with_type_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i64,
    accdesc: ProductType9878976b5bcce9c9,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        ga_24235: Bits,
        address: u32,
        size: i64,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc,
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
        // D s_0_1: read-var address:u32
        let s_0_1: u32 = fn_state.address;
        // D s_0_2: read-var size:i64
        let s_0_2: i64 = fn_state.size;
        // D s_0_3: read-var accdesc:struct
        let s_0_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_4: call Mem_with_type_read__1(s_0_1, s_0_2, s_0_3, s_0_0)
        let s_0_4: Bits = Mem_with_type_read__1(
            state,
            tracer,
            s_0_1,
            s_0_2,
            s_0_3,
            s_0_0,
        );
        // D s_0_5: write-var ga#24235 <= s_0_4
        fn_state.ga_24235 = s_0_4;
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var ga#24235:bv
        let s_1_0: Bits = fn_state.ga_24235;
        // N s_1_1: return s_1_0
        return s_1_0;
    }
}
