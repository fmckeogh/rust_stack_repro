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
use SMEAccessTrap::*;
use AArch64_CheckFPEnabled::*;
use IsFullA64Enabled::*;
use HaveSME::*;
use common::*;
pub fn AArch64_CheckFPAdvSIMDEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25088: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25089: bool,
        gs_25090: bool,
        gs_25088: (),
    }
    let fn_state = FunctionState {
        gs_25088,
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
        // S s_0_1: call AArch64_CheckFPEnabled(s_0_0)
        let s_0_1: () = AArch64_CheckFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveSME(s_1_0)
        let s_1_1: bool = HaveSME(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b9 b2
        if s_1_1 {
            return block_9(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#25089 <= s_2_0
        fn_state.gs_25089 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#25089:u8
        let s_3_0: bool = fn_state.gs_25089;
        // N s_3_1: branch s_3_0 b8 b4
        if s_3_0 {
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
        // D s_4_1: write-var gs#25090 <= s_4_0
        fn_state.gs_25090 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#25090:u8
        let s_5_0: bool = fn_state.gs_25090;
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
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16975u : u32
        let s_7_0: u32 = 16975;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #1u : u32
        let s_7_2: u32 = 1;
        // D s_7_3: call SMEAccessTrap(s_7_2, s_7_1)
        let s_7_3: () = SMEAccessTrap(state, tracer, s_7_2, s_7_1);
        // N s_7_4: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call IsFullA64Enabled(s_8_0)
        let s_8_1: bool = IsFullA64Enabled(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // D s_8_3: write-var gs#25090 <= s_8_2
        fn_state.gs_25090 = s_8_2;
        // N s_8_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16989u : u32
        let s_9_0: u32 = 16989;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: bool = {
            let value = state.read_register::<bool>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 1u16);
        // C s_9_3: const #1u : u8
        let s_9_3: bool = true;
        // C s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 1u16);
        // D s_9_5: cmp-eq s_9_2 s_9_4
        let s_9_5: bool = ((s_9_2) == (s_9_4));
        // D s_9_6: write-var gs#25089 <= s_9_5
        fn_state.gs_25089 = s_9_5;
        // N s_9_7: jump b3
        return block_3(state, tracer, fn_state);
    }
}
