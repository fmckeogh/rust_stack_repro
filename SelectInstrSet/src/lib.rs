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
use CurrentInstrSet::*;
use common::*;
pub fn SelectInstrSet<T: Tracer>(state: &mut State, tracer: &T, iset: u32) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31392: bool,
        gs_31393: bool,
        iset: u32,
    }
    let fn_state = FunctionState {
        iset,
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
        // S s_0_1: call CurrentInstrSet(s_0_0)
        let s_0_1: u32 = CurrentInstrSet(state, tracer, s_0_0);
        // C s_0_2: const #1u : u32
        let s_0_2: u32 = 1;
        // S s_0_3: cmp-eq s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) == (s_0_2));
        // N s_0_4: branch s_0_3 b8 b1
        if s_0_3 {
            return block_8(state, tracer, fn_state);
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
        // S s_1_1: call CurrentInstrSet(s_1_0)
        let s_1_1: u32 = CurrentInstrSet(state, tracer, s_1_0);
        // C s_1_2: const #2u : u32
        let s_1_2: u32 = 2;
        // S s_1_3: cmp-eq s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) == (s_1_2));
        // D s_1_4: write-var gs#31392 <= s_1_3
        fn_state.gs_31392 = s_1_3;
        // N s_1_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31392:u8
        let s_2_0: bool = fn_state.gs_31392;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var iset:u32
        let s_2_2: u32 = fn_state.iset;
        // C s_2_3: const #1u : u32
        let s_2_3: u32 = 1;
        // D s_2_4: cmp-eq s_2_2 s_2_3
        let s_2_4: bool = ((s_2_2) == (s_2_3));
        // N s_2_5: branch s_2_4 b7 b3
        if s_2_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var iset:u32
        let s_3_0: u32 = fn_state.iset;
        // C s_3_1: const #2u : u32
        let s_3_1: u32 = 2;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: write-var gs#31393 <= s_3_2
        fn_state.gs_31393 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#31393:u8
        let s_4_0: bool = fn_state.gs_31393;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var iset:u32
        let s_4_2: u32 = fn_state.iset;
        // C s_4_3: const #1u : u32
        let s_4_3: u32 = 1;
        // D s_4_4: cmp-eq s_4_2 s_4_3
        let s_4_4: bool = ((s_4_2) == (s_4_3));
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
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
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // C s_5_1: const #16993u : u32
        let s_5_1: u32 = 16993;
        // N s_5_2: write-reg s_5_1 <= s_5_0
        let s_5_2: () = {
            state.write_register::<bool>(s_5_1 as isize, s_5_0);
            tracer.write_register(s_5_1 as isize, s_5_0);
        };
        // N s_5_3: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // C s_6_1: const #16993u : u32
        let s_6_1: u32 = 16993;
        // N s_6_2: write-reg s_6_1 <= s_6_0
        let s_6_2: () = {
            state.write_register::<bool>(s_6_1 as isize, s_6_0);
            tracer.write_register(s_6_1 as isize, s_6_0);
        };
        // N s_6_3: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#31393 <= s_7_0
        fn_state.gs_31393 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#31392 <= s_8_0
        fn_state.gs_31392 = s_8_0;
        // N s_8_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
