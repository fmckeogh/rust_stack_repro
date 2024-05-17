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
use neq_int::*;
use AArch32_SysRegRead__1::*;
use ThisInstr::*;
use AArch32_SysRegReadCanWriteAPSR::*;
use common::*;
pub fn execute_aarch32_instrs_MRC_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cp: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_298727: bool,
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
        // C s_0_0: const #15s : i
        let s_0_0: i128 = 15;
        // D s_0_1: read-var t:i64
        let s_0_1: i64 = fn_state.t;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: call neq_int(s_0_2, s_0_0)
        let s_0_3: bool = neq_int(state, tracer, s_0_2, s_0_0);
        // N s_0_4: branch s_0_3 b5 b1
        if s_0_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call ThisInstr(s_1_0)
        let s_1_1: u32 = ThisInstr(state, tracer, s_1_0);
        // D s_1_2: read-var cp:i64
        let s_1_2: i64 = fn_state.cp;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: call AArch32_SysRegReadCanWriteAPSR(s_1_3, s_1_1)
        let s_1_4: bool = AArch32_SysRegReadCanWriteAPSR(state, tracer, s_1_3, s_1_1);
        // D s_1_5: write-var gs#298727 <= s_1_4
        fn_state.gs_298727 = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#298727:u8
        let s_2_0: bool = fn_state.gs_298727;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: panic
        panic!("{:?}", ());
        // N s_3_1: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call ThisInstr(s_4_0)
        let s_4_1: u32 = ThisInstr(state, tracer, s_4_0);
        // D s_4_2: read-var cp:i64
        let s_4_2: i64 = fn_state.cp;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var t:i64
        let s_4_4: i64 = fn_state.t;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: call AArch32_SysRegRead__1(s_4_3, s_4_1, s_4_5)
        let s_4_6: () = AArch32_SysRegRead__1(state, tracer, s_4_3, s_4_1, s_4_5);
        // N s_4_7: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#298727 <= s_5_0
        fn_state.gs_298727 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
