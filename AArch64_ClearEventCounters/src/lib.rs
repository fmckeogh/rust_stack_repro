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
use u__id::*;
use Zeros::*;
use AArch64_GetNumEventCountersAccessible::*;
use common::*;
pub fn AArch64_ClearEventCounters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25396: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        idx: i64,
        gs_25403: i64,
        gs_25408: bool,
        counters: i128,
        gs_25396: (),
    }
    let fn_state = FunctionState {
        gs_25396,
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
        // S s_0_1: call AArch64_GetNumEventCountersAccessible(s_0_0)
        let s_0_1: i128 = AArch64_GetNumEventCountersAccessible(state, tracer, s_0_0);
        // D s_0_2: write-var counters <= s_0_1
        fn_state.counters = s_0_1;
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // D s_0_4: read-var counters:i
        let s_0_4: i128 = fn_state.counters;
        // D s_0_5: call neq_int(s_0_4, s_0_3)
        let s_0_5: bool = neq_int(state, tracer, s_0_4, s_0_3);
        // N s_0_6: branch s_0_5 b2 b1
        if s_0_5 {
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
        // C s_2_0: const #0s : i64
        let s_2_0: i64 = 0;
        // C s_2_1: const #1s : i
        let s_2_1: i128 = 1;
        // D s_2_2: read-var counters:i
        let s_2_2: i128 = fn_state.counters;
        // D s_2_3: sub s_2_2 s_2_1
        let s_2_3: i128 = ((s_2_2) - (s_2_1));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: write-var gs#25403 <= s_2_4
        fn_state.gs_25403 = s_2_4;
        // D s_2_6: write-var idx <= s_2_0
        fn_state.idx = s_2_0;
        // N s_2_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var idx:i64
        let s_3_0: i64 = fn_state.idx;
        // D s_3_1: read-var gs#25403:i64
        let s_3_1: i64 = fn_state.gs_25403;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b8 b4
        if s_3_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var idx:i64
        let s_4_0: i64 = fn_state.idx;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call __id(s_4_1)
        let s_4_2: i128 = u__id(state, tracer, s_4_1);
        // C s_4_3: const #0s : i
        let s_4_3: i128 = 0;
        // D s_4_4: cmp-le s_4_3 s_4_2
        let s_4_4: bool = ((s_4_3) <= (s_4_2));
        // N s_4_5: branch s_4_4 b7 b5
        if s_4_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#25408 <= s_5_0
        fn_state.gs_25408 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#25408:u8
        let s_6_0: bool = fn_state.gs_25408;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // C s_6_2: const #64s : i
        let s_6_2: i128 = 64;
        // S s_6_3: call Zeros(s_6_2)
        let s_6_3: Bits = Zeros(state, tracer, s_6_2);
        // S s_6_4: cast reint s_6_3 -> u64
        let s_6_4: u64 = (s_6_3.value() as u64);
        // C s_6_5: const #10744u : u32
        let s_6_5: u32 = 10744;
        // D s_6_6: read-reg s_6_5:[u64; 32]
        let s_6_6: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_6_5 as isize);
            tracer.read_register(s_6_5 as isize, value);
            value
        };
        // D s_6_7: read-var idx:i64
        let s_6_7: i64 = fn_state.idx;
        // D s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_9: mutate-element s_6_6[s_6_8] <= s_6_4
        let s_6_9: [u64; 32usize] = {
            let mut local = s_6_6.clone();
            local[(s_6_8) as usize] = s_6_4;
            local
        };
        // D s_6_10: cast cvt s_6_9 -> [u64; 0]
        let s_6_10: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_6_9);
        // D s_6_11: cast cvt s_6_10 -> [u64; 32]
        let s_6_11: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_6_10);
            buf
        };
        // C s_6_12: const #10744u : u32
        let s_6_12: u32 = 10744;
        // N s_6_13: write-reg s_6_12 <= s_6_11
        let s_6_13: () = {
            state.write_register::<[u64; 32usize]>(s_6_12 as isize, s_6_11);
            tracer.write_register(s_6_12 as isize, s_6_11);
        };
        // D s_6_14: read-var idx:i64
        let s_6_14: i64 = fn_state.idx;
        // C s_6_15: const #1s : i64
        let s_6_15: i64 = 1;
        // D s_6_16: add s_6_14 s_6_15
        let s_6_16: i64 = (s_6_14 + s_6_15);
        // D s_6_17: write-var idx <= s_6_16
        fn_state.idx = s_6_16;
        // N s_6_18: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var idx:i64
        let s_7_0: i64 = fn_state.idx;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call __id(s_7_1)
        let s_7_2: i128 = u__id(state, tracer, s_7_1);
        // C s_7_3: const #32s : i
        let s_7_3: i128 = 32;
        // D s_7_4: cmp-lt s_7_2 s_7_3
        let s_7_4: bool = ((s_7_2) < (s_7_3));
        // D s_7_5: write-var gs#25408 <= s_7_4
        fn_state.gs_25408 = s_7_4;
        // N s_7_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
}
