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
use R_set::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_MOVT_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    imm16: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        imm16: u16,
    }
    let fn_state = FunctionState {
        d,
        imm16,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var d:i64
        let s_0_0: i64 = fn_state.d;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #16s : i
        let s_0_3: i128 = 16;
        // D s_0_4: cast zx s_0_2 -> bv
        let s_0_4: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_5: read-var imm16:u16
        let s_0_5: u16 = fn_state.imm16;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 16u16);
        // C s_0_7: const #15s : i
        let s_0_7: i128 = 15;
        // C s_0_8: const #1u : u64
        let s_0_8: u64 = 1;
        // C s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 64u16);
        // C s_0_10: lsl s_0_9 s_0_7
        let s_0_10: Bits = s_0_9 << s_0_7;
        // C s_0_11: sub s_0_10 s_0_9
        let s_0_11: Bits = ((s_0_10) - (s_0_9));
        // D s_0_12: and s_0_6 s_0_11
        let s_0_12: Bits = ((s_0_6) & (s_0_11));
        // D s_0_13: lsl s_0_12 s_0_3
        let s_0_13: Bits = s_0_12 << s_0_3;
        // C s_0_14: lsl s_0_11 s_0_3
        let s_0_14: Bits = s_0_11 << s_0_3;
        // C s_0_15: cmpl s_0_14
        let s_0_15: Bits = !s_0_14;
        // D s_0_16: and s_0_4 s_0_15
        let s_0_16: Bits = ((s_0_4) & (s_0_15));
        // D s_0_17: or s_0_16 s_0_13
        let s_0_17: Bits = ((s_0_16) | (s_0_13));
        // D s_0_18: cast reint s_0_17 -> u32
        let s_0_18: u32 = (s_0_17.value() as u32);
        // D s_0_19: read-var d:i64
        let s_0_19: i64 = fn_state.d;
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: call R_set(s_0_20, s_0_18)
        let s_0_21: () = R_set(state, tracer, s_0_20, s_0_18);
        // N s_0_22: return
        return;
    }
}
