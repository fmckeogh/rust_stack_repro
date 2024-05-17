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
pub fn MemU_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i64,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        ga_24299: Bits,
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
        // C s_0_1: const #16975u : u32
        let s_0_1: u32 = 16975;
        // D s_0_2: read-reg s_0_1:u8
        let s_0_2: u8 = {
            let value = state.read_register::<u8>(s_0_1 as isize);
            tracer.read_register(s_0_1 as isize, value);
            value
        };
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // C s_0_4: const #448u : u32
        let s_0_4: u32 = 448;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 2u16);
        // D s_0_7: cmp-ne s_0_3 s_0_6
        let s_0_7: bool = ((s_0_3) != (s_0_6));
        // C s_0_8: const #0u : u8
        let s_0_8: bool = false;
        // C s_0_9: const #0u : u32
        let s_0_9: u32 = 0;
        // D s_0_10: call CreateAccDescGPR(s_0_9, s_0_0, s_0_7, s_0_8)
        let s_0_10: ProductType9878976b5bcce9c9 = CreateAccDescGPR(
            state,
            tracer,
            s_0_9,
            s_0_0,
            s_0_7,
            s_0_8,
        );
        // D s_0_11: read-var address:u32
        let s_0_11: u32 = fn_state.address;
        // D s_0_12: read-var size:i64
        let s_0_12: i64 = fn_state.size;
        // D s_0_13: call Mem_with_type_read(s_0_11, s_0_12, s_0_10)
        let s_0_13: Bits = Mem_with_type_read(state, tracer, s_0_11, s_0_12, s_0_10);
        // D s_0_14: write-var ga#24299 <= s_0_13
        fn_state.ga_24299 = s_0_13;
        // N s_0_15: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var ga#24299:bv
        let s_1_0: Bits = fn_state.ga_24299;
        // N s_1_1: return s_1_0
        return s_1_0;
    }
}
