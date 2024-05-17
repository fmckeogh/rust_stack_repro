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
use ThisInstrAddr::*;
use AArch64_PCAlignmentFault::*;
use common::*;
pub fn AArch64_CheckPCAlignment<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25321: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25321: (),
    }
    let fn_state = FunctionState {
        gs_25321,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // C s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // S s_0_2: call ThisInstrAddr(s_0_1)
        let s_0_2: Bits = ThisInstrAddr(state, tracer, s_0_1);
        // S s_0_3: cast reint s_0_2 -> u64
        let s_0_3: u64 = (s_0_2.value() as u64);
        // C s_0_4: const #0s : i
        let s_0_4: i128 = 0;
        // S s_0_5: cast zx s_0_3 -> bv
        let s_0_5: Bits = Bits::new(s_0_3 as u128, 64u16);
        // C s_0_6: const #1s : i64
        let s_0_6: i64 = 1;
        // C s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // C s_0_8: const #1s : i
        let s_0_8: i128 = 1;
        // C s_0_9: add s_0_8 s_0_7
        let s_0_9: i128 = (s_0_8 + s_0_7);
        // D s_0_10: bit-extract s_0_5 s_0_4 s_0_9
        let s_0_10: Bits = (Bits::new(
            ((s_0_5) >> (s_0_4)).value(),
            u16::try_from(s_0_9).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u8
        let s_0_11: u8 = (s_0_10.value() as u8);
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #0u : u8
        let s_0_13: u8 = 0;
        // C s_0_14: cast zx s_0_13 -> bv
        let s_0_14: Bits = Bits::new(s_0_13 as u128, 2u16);
        // D s_0_15: cmp-ne s_0_12 s_0_14
        let s_0_15: bool = ((s_0_12) != (s_0_14));
        // N s_0_16: branch s_0_15 b2 b1
        if s_0_15 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call AArch64_PCAlignmentFault(s_2_0)
        let s_2_1: () = AArch64_PCAlignmentFault(state, tracer, s_2_0);
        // N s_2_2: return
        return;
    }
}
