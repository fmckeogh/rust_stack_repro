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
use ICC_AP1R_NS_read::*;
use u__id::*;
use ICC_AP1R_S_read::*;
use ELUsingAArch32::*;
use CurrentSecurityState::*;
use common::*;
pub fn ICC_AP1R_read<T: Tracer>(state: &mut State, tracer: &T, n: i128) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_35450: bool,
        r: u32,
        gs_35446: bool,
        gs_35454: bool,
        n: i128,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call ELUsingAArch32(s_0_1)
        let s_0_2: bool = ELUsingAArch32(state, tracer, s_0_1);
        // N s_0_3: branch s_0_2 b12 b1
        if s_0_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#35446 <= s_1_0
        fn_state.gs_35446 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var gs#35446:u8
        let s_2_0: bool = fn_state.gs_35446;
        // N s_2_1: branch s_2_0 b8 b3
        if s_2_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_3_0: read-var n:i
        let s_3_0: i128 = fn_state.n;
        // D s_3_1: call __id(s_3_0)
        let s_3_1: i128 = u__id(state, tracer, s_3_0);
        // C s_3_2: const #0s : i
        let s_3_2: i128 = 0;
        // D s_3_3: cmp-le s_3_2 s_3_1
        let s_3_3: bool = ((s_3_2) <= (s_3_1));
        // N s_3_4: branch s_3_3 b7 b4
        if s_3_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#35450 <= s_4_0
        fn_state.gs_35450 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_5_0: read-var gs#35450:u8
        let s_5_0: bool = fn_state.gs_35450;
        // N s_5_1: assert s_5_0
        let s_5_1: () = assert!(s_5_0);
        // D s_5_2: read-var n:i
        let s_5_2: i128 = fn_state.n;
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: call ICC_AP1R_NS_read(s_5_3)
        let s_5_4: u32 = ICC_AP1R_NS_read(state, tracer, s_5_3);
        // D s_5_5: write-var r <= s_5_4
        fn_state.r = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_6_0: read-var r:u32
        let s_6_0: u32 = fn_state.r;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_7_0: read-var n:i
        let s_7_0: i128 = fn_state.n;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // C s_7_2: const #4s : i
        let s_7_2: i128 = 4;
        // D s_7_3: cmp-lt s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) < (s_7_2));
        // D s_7_4: write-var gs#35450 <= s_7_3
        fn_state.gs_35450 = s_7_3;
        // N s_7_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_8_0: read-var n:i
        let s_8_0: i128 = fn_state.n;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #0s : i
        let s_8_2: i128 = 0;
        // D s_8_3: cmp-le s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) <= (s_8_1));
        // N s_8_4: branch s_8_3 b11 b9
        if s_8_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#35454 <= s_9_0
        fn_state.gs_35454 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_10_0: read-var gs#35454:u8
        let s_10_0: bool = fn_state.gs_35454;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // D s_10_2: read-var n:i
        let s_10_2: i128 = fn_state.n;
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // D s_10_4: call ICC_AP1R_S_read(s_10_3)
        let s_10_4: u32 = ICC_AP1R_S_read(state, tracer, s_10_3);
        // D s_10_5: write-var r <= s_10_4
        fn_state.r = s_10_4;
        // N s_10_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_11_0: read-var n:i
        let s_11_0: i128 = fn_state.n;
        // D s_11_1: call __id(s_11_0)
        let s_11_1: i128 = u__id(state, tracer, s_11_0);
        // C s_11_2: const #4s : i
        let s_11_2: i128 = 4;
        // D s_11_3: cmp-lt s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) < (s_11_2));
        // D s_11_4: write-var gs#35454 <= s_11_3
        fn_state.gs_35454 = s_11_3;
        // N s_11_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call CurrentSecurityState(s_12_0)
        let s_12_1: u32 = CurrentSecurityState(state, tracer, s_12_0);
        // C s_12_2: const #3u : u32
        let s_12_2: u32 = 3;
        // S s_12_3: cmp-eq s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) == (s_12_2));
        // D s_12_4: write-var gs#35446 <= s_12_3
        fn_state.gs_35446 = s_12_3;
        // N s_12_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
