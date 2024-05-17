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
use AArch32_MemSingle_read__1::*;
use common::*;
pub fn AArch32_MemSingle_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i64,
    accdesc: ProductType9878976b5bcce9c9,
    aligned: bool,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        ga_24208: Bits,
        address: u32,
        size: i64,
        accdesc: ProductType9878976b5bcce9c9,
        aligned: bool,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc,
        aligned,
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
        // D s_0_4: read-var aligned:u8
        let s_0_4: bool = fn_state.aligned;
        // D s_0_5: call AArch32_MemSingle_read__1(s_0_1, s_0_2, s_0_3, s_0_4, s_0_0)
        let s_0_5: Bits = AArch32_MemSingle_read__1(
            state,
            tracer,
            s_0_1,
            s_0_2,
            s_0_3,
            s_0_4,
            s_0_0,
        );
        // D s_0_6: write-var ga#24208 <= s_0_5
        fn_state.ga_24208 = s_0_5;
        // N s_0_7: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var ga#24208:bv
        let s_1_0: Bits = fn_state.ga_24208;
        // N s_1_1: return s_1_0
        return s_1_0;
    }
}
