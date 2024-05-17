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
use LocalTimeoutEvent::*;
use common::*;
pub fn WaitForInterrupt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    localtimeout: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_6699: bool,
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
        // S s_0_1: call HaveFeatWFxT(s_0_0)
        let s_0_1: bool = HaveFeatWFxT(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b5 b1
        if s_0_1 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#6699 <= s_1_0
        fn_state.gs_6699 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#6699:u8
        let s_2_0: bool = fn_state.gs_6699;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b4 b3
        if s_2_1 {
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
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // C s_4_1: const #101200u : u32
        let s_4_1: u32 = 101200;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<bool>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // C s_4_3: const #21848u : u32
        let s_4_3: u32 = 21848;
        // D s_4_4: read-reg s_4_3:struct
        let s_4_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_4_3 as isize);
            tracer.read_register(s_4_3 as isize, value);
            value
        };
        // C s_4_5: const #21848u : u32
        let s_4_5: u32 = 21848;
        // N s_4_6: write-reg s_4_5 <= s_4_4
        let s_4_6: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_4_5 as isize, s_4_4);
            tracer.write_register(s_4_5 as isize, s_4_4);
        };
        // C s_4_7: const #() : ()
        let s_4_7: () = ();
        // S s_4_8: call EnterLowPowerState(s_4_7)
        let s_4_8: () = EnterLowPowerState(state, tracer, s_4_7);
        // N s_4_9: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var localtimeout:i
        let s_5_0: i128 = fn_state.localtimeout;
        // D s_5_1: call LocalTimeoutEvent(s_5_0)
        let s_5_1: bool = LocalTimeoutEvent(state, tracer, s_5_0);
        // D s_5_2: write-var gs#6699 <= s_5_1
        fn_state.gs_6699 = s_5_1;
        // N s_5_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
