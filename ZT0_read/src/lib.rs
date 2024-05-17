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
pub fn ZT0_read<T: Tracer>(state: &mut State, tracer: &T, width: i64) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        width: i64,
    }
    let fn_state = FunctionState {
        width,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_0_0: const #512s : i
        let s_0_0: i128 = 512;
        // D s_0_1: read-var width:i64
        let s_0_1: i64 = fn_state.width;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: assert s_0_3
        let s_0_4: () = assert!(s_0_3);
        // C s_0_5: const #0s : i
        let s_0_5: i128 = 0;
        // C s_0_6: const #512s : i
        let s_0_6: i128 = 512;
        // C s_0_7: const #90520u : u32
        let s_0_7: u32 = 90520;
        // D s_0_8: read-reg s_0_7:u512
        let s_0_8: u64 = {
            let value = state.read_register::<u64>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 512u16);
        // D s_0_10: bit-extract s_0_9 s_0_5 s_0_6
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_5)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u512
        let s_0_11: u64 = (s_0_10.value() as u64);
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 512u16);
        // N s_0_13: return s_0_12
        return s_0_12;
    }
}
