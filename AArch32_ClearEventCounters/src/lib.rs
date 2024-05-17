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
use HaveAArch64::*;
use AArch32_GetNumEventCountersAccessible::*;
use u__id::*;
use Zeros::*;
use PMEVCNTR_set::*;
use AArch64_ClearEventCounters::*;
use common::*;
pub fn AArch32_ClearEventCounters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_32186: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_32195: i64,
        idx: i64,
        counters: i128,
        gs_32186: (),
    }
    let fn_state = FunctionState {
        gs_32186,
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
        // S s_0_1: call HaveAArch64(s_0_0)
        let s_0_1: bool = HaveAArch64(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b7 b1
        if s_0_1 {
            return block_7(state, tracer, fn_state);
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
        // S s_1_1: call AArch32_GetNumEventCountersAccessible(s_1_0)
        let s_1_1: i128 = AArch32_GetNumEventCountersAccessible(state, tracer, s_1_0);
        // D s_1_2: write-var counters <= s_1_1
        fn_state.counters = s_1_1;
        // C s_1_3: const #0s : i
        let s_1_3: i128 = 0;
        // D s_1_4: read-var counters:i
        let s_1_4: i128 = fn_state.counters;
        // D s_1_5: call neq_int(s_1_4, s_1_3)
        let s_1_5: bool = neq_int(state, tracer, s_1_4, s_1_3);
        // N s_1_6: branch s_1_5 b3 b2
        if s_1_5 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var counters:i
        let s_3_2: i128 = fn_state.counters;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#32195 <= s_3_4
        fn_state.gs_32195 = s_3_4;
        // D s_3_6: write-var idx <= s_3_0
        fn_state.idx = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var idx:i64
        let s_4_0: i64 = fn_state.idx;
        // D s_4_1: read-var gs#32195:i64
        let s_4_1: i64 = fn_state.gs_32195;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var idx:i64
        let s_5_0: i64 = fn_state.idx;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call __id(s_5_1)
        let s_5_2: i128 = u__id(state, tracer, s_5_1);
        // C s_5_3: const #0s : i
        let s_5_3: i128 = 0;
        // D s_5_4: cmp-le s_5_3 s_5_2
        let s_5_4: bool = ((s_5_3) <= (s_5_2));
        // N s_5_5: assert s_5_4
        let s_5_5: () = assert!(s_5_4);
        // D s_5_6: read-var idx:i64
        let s_5_6: i64 = fn_state.idx;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: call __id(s_5_7)
        let s_5_8: i128 = u__id(state, tracer, s_5_7);
        // C s_5_9: const #31s : i
        let s_5_9: i128 = 31;
        // D s_5_10: cmp-lt s_5_8 s_5_9
        let s_5_10: bool = ((s_5_8) < (s_5_9));
        // N s_5_11: assert s_5_10
        let s_5_11: () = assert!(s_5_10);
        // C s_5_12: const #32s : i
        let s_5_12: i128 = 32;
        // S s_5_13: call Zeros(s_5_12)
        let s_5_13: Bits = Zeros(state, tracer, s_5_12);
        // S s_5_14: cast reint s_5_13 -> u32
        let s_5_14: u32 = (s_5_13.value() as u32);
        // D s_5_15: read-var idx:i64
        let s_5_15: i64 = fn_state.idx;
        // D s_5_16: call PMEVCNTR_set(s_5_15, s_5_14)
        let s_5_16: () = PMEVCNTR_set(state, tracer, s_5_15, s_5_14);
        // D s_5_17: read-var idx:i64
        let s_5_17: i64 = fn_state.idx;
        // C s_5_18: const #1s : i64
        let s_5_18: i64 = 1;
        // D s_5_19: add s_5_17 s_5_18
        let s_5_19: i64 = (s_5_17 + s_5_18);
        // D s_5_20: write-var idx <= s_5_19
        fn_state.idx = s_5_19;
        // N s_5_21: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call AArch64_ClearEventCounters(s_7_0)
        let s_7_1: () = AArch64_ClearEventCounters(state, tracer, s_7_0);
        // N s_7_2: return
        return;
    }
}
