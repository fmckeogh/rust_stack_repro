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
pub fn AArch32_S2IASize<T: Tracer>(state: &mut State, tracer: &T, t0sz: u8) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        t0sz: u8,
    }
    let fn_state = FunctionState {
        t0sz,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_0_0: read-var t0sz:u8
        let s_0_0: u8 = fn_state.t0sz;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // D s_0_2: cast sx s_0_1 -> i
        let s_0_2: i128 = {
            let sign_bit = s_0_1.length() - 1;
            let mut result = s_0_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // C s_0_4: const #32s : i
        let s_0_4: i128 = 32;
        // D s_0_5: cast zx s_0_3 -> i
        let s_0_5: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_6: sub s_0_4 s_0_5
        let s_0_6: i128 = ((s_0_4) - (s_0_5));
        // D s_0_7: cast reint s_0_6 -> i64
        let s_0_7: i64 = (s_0_6 as i64);
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // N s_0_9: return s_0_8
        return s_0_8;
    }
}
