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
use CommitTransactionalWrites::*;
use IsTMEEnabled::*;
use ClearExclusiveLocal::*;
use ProcessorID::*;
use common::*;
pub fn execute_aarch64_instrs_system_tme_tcommit<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_173972: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_173972: (),
    }
    let fn_state = FunctionState {
        gs_173972,
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
        // S s_0_1: call IsTMEEnabled(s_0_0)
        let s_0_1: bool = IsTMEEnabled(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b7 b1
        if s_0_2 {
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
        // C s_1_0: const #100180u : u32
        let s_1_0: u32 = 100180;
        // D s_1_1: read-reg s_1_0:i
        let s_1_1: i128 = {
            let value = state.read_register::<i128>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #0s : i
        let s_1_2: i128 = 0;
        // D s_1_3: cmp-eq s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) == (s_1_2));
        // N s_1_4: branch s_1_3 b6 b2
        if s_1_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #100180u : u32
        let s_2_0: u32 = 100180;
        // D s_2_1: read-reg s_2_0:i
        let s_2_1: i128 = {
            let value = state.read_register::<i128>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // C s_2_2: const #1s : i
        let s_2_2: i128 = 1;
        // D s_2_3: cmp-eq s_2_1 s_2_2
        let s_2_3: bool = ((s_2_1) == (s_2_2));
        // N s_2_4: branch s_2_3 b5 b3
        if s_2_3 {
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
        // C s_4_0: const #100180u : u32
        let s_4_0: u32 = 100180;
        // D s_4_1: read-reg s_4_0:i
        let s_4_1: i128 = {
            let value = state.read_register::<i128>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // C s_4_2: const #1s : i
        let s_4_2: i128 = 1;
        // D s_4_3: sub s_4_1 s_4_2
        let s_4_3: i128 = ((s_4_1) - (s_4_2));
        // C s_4_4: const #100180u : u32
        let s_4_4: u32 = 100180;
        // N s_4_5: write-reg s_4_4 <= s_4_3
        let s_4_5: () = {
            state.write_register::<i128>(s_4_4 as isize, s_4_3);
            tracer.write_register(s_4_4 as isize, s_4_3);
        };
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call CommitTransactionalWrites(s_5_0)
        let s_5_1: () = CommitTransactionalWrites(state, tracer, s_5_0);
        // C s_5_2: const #() : ()
        let s_5_2: () = ();
        // S s_5_3: call ProcessorID(s_5_2)
        let s_5_3: i128 = ProcessorID(state, tracer, s_5_2);
        // S s_5_4: call ClearExclusiveLocal(s_5_3)
        let s_5_4: () = ClearExclusiveLocal(state, tracer, s_5_3);
        // N s_5_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
}
