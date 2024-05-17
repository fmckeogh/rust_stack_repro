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
use SPEToCollectSample::*;
use PMUEvent::*;
use SPESampleAddTimeStamp::*;
use StatisticalProfilingEnabled::*;
use SPESampleAddAddressPCVirtual::*;
use SPESampleAddContext::*;
use SPEStartCounter::*;
use SPESampleAddOpOther__1::*;
use common::*;
pub fn SPEPreExecution<T: Tracer>(state: &mut State, tracer: &T, gs_25681: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25681: (),
    }
    let fn_state = FunctionState {
        gs_25681,
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
        // S s_0_1: call StatisticalProfilingEnabled(s_0_0)
        let s_0_1: bool = StatisticalProfilingEnabled(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
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
        // C s_2_0: const #152u : u32
        let s_2_0: u32 = 152;
        // D s_2_1: read-reg s_2_0:u16
        let s_2_1: u16 = {
            let value = state.read_register::<u16>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call PMUEvent(s_2_1)
        let s_2_2: () = PMUEvent(state, tracer, s_2_1);
        // C s_2_3: const #() : ()
        let s_2_3: () = ();
        // S s_2_4: call SPEToCollectSample(s_2_3)
        let s_2_4: bool = SPEToCollectSample(state, tracer, s_2_3);
        // N s_2_5: branch s_2_4 b4 b3
        if s_2_4 {
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
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #22416u : u32
        let s_4_0: u32 = 22416;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: bool = {
            let value = state.read_register::<bool>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: not s_4_1
        let s_4_2: bool = !s_4_1;
        // N s_4_3: branch s_4_2 b7 b5
        if s_4_2 {
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
        // C s_5_0: const #176u : u32
        let s_5_0: u32 = 176;
        // D s_5_1: read-reg s_5_0:u16
        let s_5_1: u16 = {
            let value = state.read_register::<u16>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call PMUEvent(s_5_1)
        let s_5_2: () = PMUEvent(state, tracer, s_5_1);
        // C s_5_3: const #13704u : u32
        let s_5_3: u32 = 13704;
        // D s_5_4: read-reg s_5_3:struct
        let s_5_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // C s_5_5: const #13704u : u32
        let s_5_5: u32 = 13704;
        // N s_5_6: write-reg s_5_5 <= s_5_4
        let s_5_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_5 as isize, s_5_4);
            tracer.write_register(s_5_5 as isize, s_5_4);
        };
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // S s_6_1: call SPESampleAddOpOther__1(s_6_0)
        let s_6_1: () = SPESampleAddOpOther__1(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // C s_7_1: const #22416u : u32
        let s_7_1: u32 = 22416;
        // N s_7_2: write-reg s_7_1 <= s_7_0
        let s_7_2: () = {
            state.write_register::<bool>(s_7_1 as isize, s_7_0);
            tracer.write_register(s_7_1 as isize, s_7_0);
        };
        // C s_7_3: const #1056u : u32
        let s_7_3: u32 = 1056;
        // D s_7_4: read-reg s_7_3:i64
        let s_7_4: i64 = {
            let value = state.read_register::<i64>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: call SPEStartCounter(s_7_5)
        let s_7_6: () = SPEStartCounter(state, tracer, s_7_5);
        // C s_7_7: const #1064u : u32
        let s_7_7: u32 = 1064;
        // D s_7_8: read-reg s_7_7:i64
        let s_7_8: i64 = {
            let value = state.read_register::<i64>(s_7_7 as isize);
            tracer.read_register(s_7_7 as isize, value);
            value
        };
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_10: call SPEStartCounter(s_7_9)
        let s_7_10: () = SPEStartCounter(state, tracer, s_7_9);
        // C s_7_11: const #() : ()
        let s_7_11: () = ();
        // S s_7_12: call SPESampleAddContext(s_7_11)
        let s_7_12: () = SPESampleAddContext(state, tracer, s_7_11);
        // C s_7_13: const #() : ()
        let s_7_13: () = ();
        // S s_7_14: call SPESampleAddAddressPCVirtual(s_7_13)
        let s_7_14: () = SPESampleAddAddressPCVirtual(state, tracer, s_7_13);
        // C s_7_15: const #() : ()
        let s_7_15: () = ();
        // S s_7_16: call SPESampleAddTimeStamp(s_7_15)
        let s_7_16: () = SPESampleAddTimeStamp(state, tracer, s_7_15);
        // N s_7_17: jump b6
        return block_6(state, tracer, fn_state);
    }
}
