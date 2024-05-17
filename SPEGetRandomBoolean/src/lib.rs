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
use SPEGetRandomInterval::*;
use common::*;
pub fn SPEGetRandomBoolean<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_26257: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_26257: (),
    }
    let fn_state = FunctionState {
        gs_26257,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call SPEGetRandomInterval(s_0_0)
        let s_0_1: u8 = SPEGetRandomInterval(state, tracer, s_0_0);
        // C s_0_2: const #0s : i
        let s_0_2: i128 = 0;
        // S s_0_3: cast zx s_0_1 -> bv
        let s_0_3: Bits = Bits::new(s_0_1 as u128, 8u16);
        // C s_0_4: const #1u : u64
        let s_0_4: u64 = 1;
        // D s_0_5: bit-extract s_0_3 s_0_2 s_0_4
        let s_0_5: Bits = (Bits::new(
            ((s_0_3) >> (s_0_2)).value(),
            u16::try_from(s_0_4).unwrap(),
        ));
        // D s_0_6: cast reint s_0_5 -> u8
        let s_0_6: bool = ((s_0_5.value()) != 0);
        // C s_0_7: const #0s : i
        let s_0_7: i128 = 0;
        // C s_0_8: const #0u : u64
        let s_0_8: u64 = 0;
        // D s_0_9: cast zx s_0_6 -> u64
        let s_0_9: u64 = (s_0_6 as u64);
        // C s_0_10: const #1u : u64
        let s_0_10: u64 = 1;
        // D s_0_11: and s_0_9 s_0_10
        let s_0_11: u64 = ((s_0_9) & (s_0_10));
        // D s_0_12: cmp-eq s_0_11 s_0_10
        let s_0_12: bool = ((s_0_11) == (s_0_10));
        // D s_0_13: lsl s_0_9 s_0_7
        let s_0_13: u64 = s_0_9 << s_0_7;
        // D s_0_14: or s_0_8 s_0_13
        let s_0_14: u64 = ((s_0_8) | (s_0_13));
        // D s_0_15: cmpl s_0_13
        let s_0_15: u64 = !s_0_13;
        // D s_0_16: and s_0_8 s_0_15
        let s_0_16: u64 = ((s_0_8) & (s_0_15));
        // D s_0_17: select s_0_12 s_0_14 s_0_16
        let s_0_17: u64 = if s_0_12 { s_0_14 } else { s_0_16 };
        // D s_0_18: cast trunc s_0_17 -> u8
        let s_0_18: bool = ((s_0_17) != 0);
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 1u16);
        // C s_0_20: const #1u : u8
        let s_0_20: bool = true;
        // C s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 1u16);
        // D s_0_22: cmp-eq s_0_19 s_0_21
        let s_0_22: bool = ((s_0_19) == (s_0_21));
        // N s_0_23: return s_0_22
        return s_0_22;
    }
}
