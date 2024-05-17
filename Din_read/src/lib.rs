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
use common::*;
pub fn Din_read<T: Tracer>(state: &mut State, tracer: &T, n: i128) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        gs_31428: bool,
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
    ) -> u64 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var n:i
        let s_0_1: i128 = fn_state.n;
        // D s_0_2: cmp-ge s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) >= (s_0_0));
        // N s_0_3: branch s_0_2 b3 b1
        if s_0_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#31428 <= s_1_0
        fn_state.gs_31428 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var gs#31428:u8
        let s_2_0: bool = fn_state.gs_31428;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #12976u : u32
        let s_2_2: u32 = 12976;
        // D s_2_3: read-reg s_2_2:[u64; 32]
        let s_2_3: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: read-var n:i
        let s_2_4: i128 = fn_state.n;
        // D s_2_5: read-element s_2_3[s_2_4]
        let s_2_5: u64 = s_2_3[(s_2_4) as usize];
        // N s_2_6: return s_2_5
        return s_2_5;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_3_0: const #31s : i
        let s_3_0: i128 = 31;
        // D s_3_1: read-var n:i
        let s_3_1: i128 = fn_state.n;
        // D s_3_2: cmp-le s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) <= (s_3_0));
        // D s_3_3: write-var gs#31428 <= s_3_2
        fn_state.gs_31428 = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
