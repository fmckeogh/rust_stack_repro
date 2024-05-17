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
use AArch32_SysRegWrite::*;
use ThisInstr::*;
use common::*;
pub fn execute_aarch32_instrs_MCR_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cp: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        cp: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        cp,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ThisInstr(s_0_0)
        let s_0_1: u32 = ThisInstr(state, tracer, s_0_0);
        // D s_0_2: read-var cp:i64
        let s_0_2: i64 = fn_state.cp;
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: read-var t:i64
        let s_0_4: i64 = fn_state.t;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: call AArch32_SysRegWrite(s_0_3, s_0_1, s_0_5)
        let s_0_6: () = AArch32_SysRegWrite(state, tracer, s_0_3, s_0_1, s_0_5);
        // N s_0_7: return
        return;
    }
}
