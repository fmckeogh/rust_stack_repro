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
use HaveNoninvasiveDebugAuth::*;
use ExternalNoninvasiveDebugEnabled::*;
use ExternalSecureInvasiveDebugEnabled::*;
use SecureOnlyImplementation::*;
use common::*;
pub fn ExternalSecureNoninvasiveDebugEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_2081: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        gs_2084: bool,
        gs_2085: bool,
        gs_2082: bool,
        gs_2081: (),
    }
    let fn_state = FunctionState {
        gs_2081,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // D s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b14 b1
        if s_0_4 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#2082 <= s_1_0
        fn_state.gs_2082 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#2082:u8
        let s_2_0: bool = fn_state.gs_2082;
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
        // S s_3_1: call HaveNoninvasiveDebugAuth(s_3_0)
        let s_3_1: bool = HaveNoninvasiveDebugAuth(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b6 b4
        if s_3_1 {
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
        // S s_4_1: call ExternalSecureInvasiveDebugEnabled(s_4_0)
        let s_4_1: bool = ExternalSecureInvasiveDebugEnabled(state, tracer, s_4_0);
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
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call ExternalNoninvasiveDebugEnabled(s_6_0)
        let s_6_1: bool = ExternalNoninvasiveDebugEnabled(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b9 b7
        if s_6_1 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#2085 <= s_7_0
        fn_state.gs_2085 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#2085:u8
        let s_8_0: bool = fn_state.gs_2085;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #12136u : u32
        let s_9_0: u32 = 12136;
        // D s_9_1: read-reg s_9_0:u32
        let s_9_1: u32 = {
            let value = state.read_register::<u32>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // C s_9_2: const #1u : u32
        let s_9_2: u32 = 1;
        // D s_9_3: cmp-eq s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) == (s_9_2));
        // N s_9_4: branch s_9_3 b12 b10
        if s_9_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #104920u : u32
        let s_10_0: u32 = 104920;
        // D s_10_1: read-reg s_10_0:u32
        let s_10_1: u32 = {
            let value = state.read_register::<u32>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #1u : u32
        let s_10_2: u32 = 1;
        // D s_10_3: cmp-eq s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) == (s_10_2));
        // D s_10_4: write-var gs#2084 <= s_10_3
        fn_state.gs_2084 = s_10_3;
        // N s_10_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#2084:u8
        let s_11_0: bool = fn_state.gs_2084;
        // D s_11_1: write-var gs#2085 <= s_11_0
        fn_state.gs_2085 = s_11_0;
        // N s_11_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#2084 <= s_12_0
        fn_state.gs_2084 = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
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
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call SecureOnlyImplementation(s_14_0)
        let s_14_1: bool = SecureOnlyImplementation(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // D s_14_3: write-var gs#2082 <= s_14_2
        fn_state.gs_2082 = s_14_2;
        // N s_14_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
