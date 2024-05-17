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
use common::*;
pub fn u__get_DBGDEVID2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: u32,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        value_name: u32,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_0_0: read-var value_name:u32
        let s_0_0: u32 = fn_state.value_name;
        // C s_0_1: const #4294967295u : u32
        let s_0_1: u32 = 4294967295;
        // C s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 32u16);
        // C s_0_3: not s_0_2
        let s_0_3: Bits = !s_0_2;
        // C s_0_4: cast reint s_0_3 -> u32
        let s_0_4: u32 = (s_0_3.value() as u32);
        // D s_0_5: cast zx s_0_0 -> bv
        let s_0_5: Bits = Bits::new(s_0_0 as u128, 32u16);
        // C s_0_6: cast zx s_0_4 -> bv
        let s_0_6: Bits = Bits::new(s_0_4 as u128, 32u16);
        // D s_0_7: and s_0_5 s_0_6
        let s_0_7: Bits = ((s_0_5) & (s_0_6));
        // D s_0_8: cast reint s_0_7 -> u32
        let s_0_8: u32 = (s_0_7.value() as u32);
        // N s_0_9: return s_0_8
        return s_0_8;
    }
}
