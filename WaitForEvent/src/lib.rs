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
use HaveFeatWFxT::*;
use EnterLowPowerState::*;
use IsEventRegisterSet::*;
use LocalTimeoutEvent::*;
use common::*;
pub fn WaitForEvent<T: Tracer>(state: &mut State, tracer: &T, localtimeout: i128) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_6646: bool,
        gs_6647: bool,
        localtimeout: i128,
    }
    let fn_state = FunctionState {
        localtimeout,
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
        // S s_0_1: call IsEventRegisterSet(s_0_0)
        let s_0_1: bool = IsEventRegisterSet(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b8 b1
        if s_0_1 {
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
        // S s_1_1: call HaveFeatWFxT(s_1_0)
        let s_1_1: bool = HaveFeatWFxT(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b7 b2
        if s_1_1 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#6646 <= s_2_0
        fn_state.gs_6646 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#6646:u8
        let s_3_0: bool = fn_state.gs_6646;
        // D s_3_1: write-var gs#6647 <= s_3_0
        fn_state.gs_6647 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#6647:u8
        let s_4_0: bool = fn_state.gs_6647;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b6 b5
        if s_4_1 {
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
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // C s_6_1: const #11848u : u32
        let s_6_1: u32 = 11848;
        // N s_6_2: write-reg s_6_1 <= s_6_0
        let s_6_2: () = {
            state.write_register::<bool>(s_6_1 as isize, s_6_0);
            tracer.write_register(s_6_1 as isize, s_6_0);
        };
        // C s_6_3: const #21848u : u32
        let s_6_3: u32 = 21848;
        // D s_6_4: read-reg s_6_3:struct
        let s_6_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // C s_6_5: const #21848u : u32
        let s_6_5: u32 = 21848;
        // N s_6_6: write-reg s_6_5 <= s_6_4
        let s_6_6: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_6_5 as isize, s_6_4);
            tracer.write_register(s_6_5 as isize, s_6_4);
        };
        // C s_6_7: const #() : ()
        let s_6_7: () = ();
        // S s_6_8: call EnterLowPowerState(s_6_7)
        let s_6_8: () = EnterLowPowerState(state, tracer, s_6_7);
        // N s_6_9: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var localtimeout:i
        let s_7_0: i128 = fn_state.localtimeout;
        // D s_7_1: call LocalTimeoutEvent(s_7_0)
        let s_7_1: bool = LocalTimeoutEvent(state, tracer, s_7_0);
        // D s_7_2: write-var gs#6646 <= s_7_1
        fn_state.gs_6646 = s_7_1;
        // N s_7_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#6647 <= s_8_0
        fn_state.gs_6647 = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
