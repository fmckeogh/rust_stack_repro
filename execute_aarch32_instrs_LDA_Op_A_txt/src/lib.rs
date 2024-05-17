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
use MemO_read::*;
use R_read::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_LDA_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_913423: Bits,
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
        // C s_0_9: const #4s : i64
        let s_0_9: i64 = 4;
        // D s_0_10: call MemO_read(s_0_2, s_0_9)
        let s_0_10: Bits = MemO_read(state, tracer, s_0_2, s_0_9);
        // D s_0_11: write-var gs#913423 <= s_0_10
        fn_state.gs_913423 = s_0_10;
        // N s_0_12: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var gs#913423:bv
        let s_1_0: Bits = fn_state.gs_913423;
        // D s_1_1: cast reint s_1_0 -> u32
        let s_1_1: u32 = (s_1_0.value() as u32);
        // D s_1_2: read-var t:i64
        let s_1_2: i64 = fn_state.t;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: call R_set(s_1_3, s_1_1)
        let s_1_4: () = R_set(state, tracer, s_1_3, s_1_1);
        // N s_1_5: return
        return;
    }
}
