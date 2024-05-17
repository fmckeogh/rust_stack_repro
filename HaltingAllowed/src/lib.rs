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
use ExternalRootInvasiveDebugEnabled::*;
use ExternalRealmInvasiveDebugEnabled::*;
use CurrentSecurityState::*;
use DoubleLockStatus::*;
use Halted::*;
use ExternalInvasiveDebugEnabled::*;
use ExternalSecureInvasiveDebugEnabled::*;
use common::*;
pub fn HaltingAllowed<T: Tracer>(state: &mut State, tracer: &T, gs_4678: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ss: u32,
        gs_4679: bool,
        ga_3065: bool,
        return_value: bool,
        gs_4678: (),
    }
    let fn_state = FunctionState {
        gs_4678,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call Halted(s_0_0)
        let s_0_1: bool = Halted(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b14 b1
        if s_0_1 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call DoubleLockStatus(s_1_0)
        let s_1_1: bool = DoubleLockStatus(state, tracer, s_1_0);
        // D s_1_2: write-var gs#4679 <= s_1_1
        fn_state.gs_4679 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#4679:u8
        let s_2_0: bool = fn_state.gs_4679;
        // N s_2_1: branch s_2_0 b13 b3
        if s_2_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call CurrentSecurityState(s_3_0)
        let s_3_1: u32 = CurrentSecurityState(state, tracer, s_3_0);
        // D s_3_2: write-var ss <= s_3_1
        fn_state.ss = s_3_1;
        // C s_3_3: const #0u : u32
        let s_3_3: u32 = 0;
        // D s_3_4: read-var ss:u32
        let s_3_4: u32 = fn_state.ss;
        // D s_3_5: cmp-eq s_3_3 s_3_4
        let s_3_5: bool = ((s_3_3) == (s_3_4));
        // D s_3_6: not s_3_5
        let s_3_6: bool = !s_3_5;
        // N s_3_7: branch s_3_6 b6 b4
        if s_3_6 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call ExternalInvasiveDebugEnabled(s_4_0)
        let s_4_1: bool = ExternalInvasiveDebugEnabled(state, tracer, s_4_0);
        // D s_4_2: write-var return_value <= s_4_1
        fn_state.return_value = s_4_1;
        // N s_4_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var return_value:u8
        let s_5_0: bool = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #3u : u32
        let s_6_0: u32 = 3;
        // D s_6_1: read-var ss:u32
        let s_6_1: u32 = fn_state.ss;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call ExternalSecureInvasiveDebugEnabled(s_7_0)
        let s_7_1: bool = ExternalSecureInvasiveDebugEnabled(state, tracer, s_7_0);
        // D s_7_2: write-var return_value <= s_7_1
        fn_state.return_value = s_7_1;
        // N s_7_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #1u : u32
        let s_8_0: u32 = 1;
        // D s_8_1: read-var ss:u32
        let s_8_1: u32 = fn_state.ss;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b10 b9
        if s_8_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call ExternalRootInvasiveDebugEnabled(s_9_0)
        let s_9_1: bool = ExternalRootInvasiveDebugEnabled(state, tracer, s_9_0);
        // D s_9_2: write-var return_value <= s_9_1
        fn_state.return_value = s_9_1;
        // N s_9_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #2u : u32
        let s_10_0: u32 = 2;
        // D s_10_1: read-var ss:u32
        let s_10_1: u32 = fn_state.ss;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b12 b11
        if s_10_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call ExternalRealmInvasiveDebugEnabled(s_11_0)
        let s_11_1: bool = ExternalRealmInvasiveDebugEnabled(state, tracer, s_11_0);
        // D s_11_2: write-var return_value <= s_11_1
        fn_state.return_value = s_11_1;
        // N s_11_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var ga#3065:u8
        let s_12_0: bool = fn_state.ga_3065;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var return_value <= s_13_0
        fn_state.return_value = s_13_0;
        // N s_13_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#4679 <= s_14_0
        fn_state.gs_4679 = s_14_0;
        // N s_14_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
