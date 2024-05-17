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
pub fn u__get_HVBAR<T: Tracer>(state: &mut State, tracer: &T, value_name: u32) -> u32 {
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
        // C s_0_1: const #32s : i
        let s_0_1: i128 = 32;
        // C s_0_2: const #31u : u8
        let s_0_2: u8 = 31;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 8u16);
        // D s_0_4: bits-cast zx s_0_3 -> bv length s_0_1
        let s_0_4: Bits = s_0_3.zero_extend(s_0_1);
        // D s_0_5: cast reint s_0_4 -> u32
        let s_0_5: u32 = (s_0_4.value() as u32);
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 32u16);
        // D s_0_7: not s_0_6
        let s_0_7: Bits = !s_0_6;
        // D s_0_8: cast reint s_0_7 -> u32
        let s_0_8: u32 = (s_0_7.value() as u32);
        // D s_0_9: cast zx s_0_0 -> bv
        let s_0_9: Bits = Bits::new(s_0_0 as u128, 32u16);
        // D s_0_10: cast zx s_0_8 -> bv
        let s_0_10: Bits = Bits::new(s_0_8 as u128, 32u16);
        // D s_0_11: and s_0_9 s_0_10
        let s_0_11: Bits = ((s_0_9) & (s_0_10));
        // D s_0_12: cast reint s_0_11 -> u32
        let s_0_12: u32 = (s_0_11.value() as u32);
        // N s_0_13: return s_0_12
        return s_0_12;
    }
}
