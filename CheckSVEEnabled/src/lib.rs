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
use HaveSVE::*;
use CheckStreamingSVEEnabled::*;
use CheckSMEEnabled::*;
use CheckOriginalSVEEnabled::*;
use HaveSME::*;
use common::*;
pub fn CheckSVEEnabled<T: Tracer>(state: &mut State, tracer: &T, gs_21259: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_21261: bool,
        gs_21260: bool,
        gs_21259: (),
    }
    let fn_state = FunctionState {
        gs_21259,
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
        // S s_0_1: call HaveSME(s_0_0)
        let s_0_1: bool = HaveSME(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b10 b1
        if s_0_1 {
            return block_10(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#21260 <= s_1_0
        fn_state.gs_21260 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#21260:u8
        let s_2_0: bool = fn_state.gs_21260;
        // N s_2_1: branch s_2_0 b9 b3
        if s_2_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HaveSME(s_3_0)
        let s_3_1: bool = HaveSME(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b8 b4
        if s_3_1 {
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
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#21261 <= s_4_0
        fn_state.gs_21261 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#21261:u8
        let s_5_0: bool = fn_state.gs_21261;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call CheckOriginalSVEEnabled(s_6_0)
        let s_6_1: () = CheckOriginalSVEEnabled(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call CheckStreamingSVEEnabled(s_7_0)
        let s_7_1: () = CheckStreamingSVEEnabled(state, tracer, s_7_0);
        // N s_7_2: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call HaveSVE(s_8_0)
        let s_8_1: bool = HaveSVE(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // D s_8_3: write-var gs#21261 <= s_8_2
        fn_state.gs_21261 = s_8_2;
        // N s_8_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call CheckSMEEnabled(s_9_0)
        let s_9_1: () = CheckSMEEnabled(state, tracer, s_9_0);
        // N s_9_2: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16989u : u32
        let s_10_0: u32 = 16989;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: bool = {
            let value = state.read_register::<bool>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 1u16);
        // C s_10_3: const #1u : u8
        let s_10_3: bool = true;
        // C s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 1u16);
        // D s_10_5: cmp-eq s_10_2 s_10_4
        let s_10_5: bool = ((s_10_2) == (s_10_4));
        // D s_10_6: write-var gs#21260 <= s_10_5
        fn_state.gs_21260 = s_10_5;
        // N s_10_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
