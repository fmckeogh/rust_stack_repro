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
use AArch32_SetLSInstructionSyndrome::*;
use MemO_set::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_STL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        n,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #4s : i
        let s_0_3: i128 = 4;
        // D s_0_4: read-var t:i64
        let s_0_4: i64 = fn_state.t;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // C s_0_6: const #0u : u8
        let s_0_6: bool = false;
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // D s_0_8: call AArch32_SetLSInstructionSyndrome(s_0_3, s_0_6, s_0_5, s_0_7)
        let s_0_8: () = AArch32_SetLSInstructionSyndrome(
            state,
            tracer,
            s_0_3,
            s_0_6,
            s_0_5,
            s_0_7,
        );
        // D s_0_9: read-var t:i64
        let s_0_9: i64 = fn_state.t;
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_11: call R_read(s_0_10)
        let s_0_11: u32 = R_read(state, tracer, s_0_10);
        // C s_0_12: const #4s : i
        let s_0_12: i128 = 4;
        // D s_0_13: cast zx s_0_11 -> bv
        let s_0_13: Bits = Bits::new(s_0_11 as u128, 32u16);
        // D s_0_14: call MemO_set(s_0_2, s_0_12, s_0_13)
        let s_0_14: () = MemO_set(state, tracer, s_0_2, s_0_12, s_0_13);
        // N s_0_15: return
        return;
    }
}
