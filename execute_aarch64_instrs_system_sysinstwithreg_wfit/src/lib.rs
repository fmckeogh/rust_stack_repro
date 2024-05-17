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
use X_read::*;
use Hint_WFI::*;
use Halted::*;
use EndOfInstruction::*;
use ConstrainUnpredictableBool::*;
use common::*;
pub fn execute_aarch64_instrs_system_sysinstwithreg_wfit<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_174359: bool,
        localtimeout: i128,
        d: i64,
    }
    let fn_state = FunctionState {
        d,
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
        // D s_0_1: read-var d:i64
        let s_0_1: i64 = fn_state.d;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: call X_read(s_0_2, s_0_0)
        let s_0_3: Bits = X_read(state, tracer, s_0_2, s_0_0);
        // D s_0_4: cast reint s_0_3 -> u64
        let s_0_4: u64 = (s_0_3.value() as u64);
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 64u16);
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (s_0_5.value() as i128);
        // D s_0_7: write-var localtimeout <= s_0_6
        fn_state.localtimeout = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call Halted(s_0_8)
        let s_0_9: bool = Halted(state, tracer, s_0_8);
        // N s_0_10: branch s_0_9 b6 b1
        if s_0_9 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#174359 <= s_1_0
        fn_state.gs_174359 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#174359:u8
        let s_2_0: bool = fn_state.gs_174359;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var localtimeout:i
        let s_4_0: i128 = fn_state.localtimeout;
        // C s_4_1: const #3u : u32
        let s_4_1: u32 = 3;
        // D s_4_2: call Hint_WFI(s_4_0, s_4_1)
        let s_4_2: () = Hint_WFI(state, tracer, s_4_0, s_4_1);
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call EndOfInstruction(s_5_0)
        let s_5_1: () = EndOfInstruction(state, tracer, s_5_0);
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #65u : u32
        let s_6_0: u32 = 65;
        // S s_6_1: call ConstrainUnpredictableBool(s_6_0)
        let s_6_1: bool = ConstrainUnpredictableBool(state, tracer, s_6_0);
        // D s_6_2: write-var gs#174359 <= s_6_1
        fn_state.gs_174359 = s_6_1;
        // N s_6_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
