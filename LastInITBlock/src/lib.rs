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
pub fn LastInITBlock<T: Tracer>(state: &mut State, tracer: &T, gs_31563: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_31563: (),
    }
    let fn_state = FunctionState {
        gs_31563,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #16981u : u32
        let s_0_0: u32 = 16981;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #0s : i
        let s_0_2: i128 = 0;
        // D s_0_3: cast zx s_0_1 -> bv
        let s_0_3: Bits = Bits::new(s_0_1 as u128, 8u16);
        // C s_0_4: const #1s : i64
        let s_0_4: i64 = 1;
        // C s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // C s_0_6: const #3s : i
        let s_0_6: i128 = 3;
        // C s_0_7: add s_0_6 s_0_5
        let s_0_7: i128 = (s_0_6 + s_0_5);
        // D s_0_8: bit-extract s_0_3 s_0_2 s_0_7
        let s_0_8: Bits = (Bits::new(
            ((s_0_3) >> (s_0_2)).value(),
            u16::try_from(s_0_7).unwrap(),
        ));
        // D s_0_9: cast reint s_0_8 -> u8
        let s_0_9: u8 = (s_0_8.value() as u8);
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 4u16);
        // C s_0_11: const #8u : u8
        let s_0_11: u8 = 8;
        // C s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 4u16);
        // D s_0_13: cmp-eq s_0_10 s_0_12
        let s_0_13: bool = ((s_0_10) == (s_0_12));
        // N s_0_14: return s_0_13
        return s_0_13;
    }
}
