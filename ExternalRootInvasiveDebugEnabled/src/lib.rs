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
use ExternalSecureInvasiveDebugEnabled::*;
use HaveSecureEL2Ext::*;
use HaveRME::*;
use ExternalInvasiveDebugEnabled::*;
use ExternalRealmInvasiveDebugEnabled::*;
use common::*;
pub fn ExternalRootInvasiveDebugEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_2971: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_2973: bool,
        gs_2976: bool,
        gs_2974: bool,
        gs_2975: bool,
        return_value: bool,
        gs_2971: (),
    }
    let fn_state = FunctionState {
        gs_2971,
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
        // S s_0_1: call HaveRME(s_0_0)
        let s_0_1: bool = HaveRME(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b15 b1
        if s_0_2 {
            return block_15(state, tracer, fn_state);
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
        // S s_1_1: call ExternalInvasiveDebugEnabled(s_1_0)
        let s_1_1: bool = ExternalInvasiveDebugEnabled(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b11 b2
        if s_1_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#2974 <= s_2_0
        fn_state.gs_2974 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#2974:u8
        let s_3_0: bool = fn_state.gs_2974;
        // N s_3_1: branch s_3_0 b10 b4
        if s_3_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#2975 <= s_4_0
        fn_state.gs_2975 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#2975:u8
        let s_5_0: bool = fn_state.gs_2975;
        // N s_5_1: branch s_5_0 b9 b6
        if s_5_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#2976 <= s_6_0
        fn_state.gs_2976 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#2976:u8
        let s_7_0: bool = fn_state.gs_2976;
        // D s_7_1: write-var return_value <= s_7_0
        fn_state.return_value = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var return_value:u8
        let s_8_0: bool = fn_state.return_value;
        // N s_8_1: return s_8_0
        return s_8_0;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #15480u : u32
        let s_9_0: u32 = 15480;
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
        // D s_9_4: write-var gs#2976 <= s_9_3
        fn_state.gs_2976 = s_9_3;
        // N s_9_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call ExternalRealmInvasiveDebugEnabled(s_10_0)
        let s_10_1: bool = ExternalRealmInvasiveDebugEnabled(state, tracer, s_10_0);
        // D s_10_2: write-var gs#2975 <= s_10_1
        fn_state.gs_2975 = s_10_1;
        // N s_10_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HaveSecureEL2Ext(s_11_0)
        let s_11_1: bool = HaveSecureEL2Ext(state, tracer, s_11_0);
        // S s_11_2: not s_11_1
        let s_11_2: bool = !s_11_1;
        // N s_11_3: branch s_11_2 b14 b12
        if s_11_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call ExternalSecureInvasiveDebugEnabled(s_12_0)
        let s_12_1: bool = ExternalSecureInvasiveDebugEnabled(state, tracer, s_12_0);
        // D s_12_2: write-var gs#2973 <= s_12_1
        fn_state.gs_2973 = s_12_1;
        // N s_12_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#2973:u8
        let s_13_0: bool = fn_state.gs_2973;
        // D s_13_1: write-var gs#2974 <= s_13_0
        fn_state.gs_2974 = s_13_0;
        // N s_13_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#2973 <= s_14_0
        fn_state.gs_2973 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var return_value <= s_15_0
        fn_state.return_value = s_15_0;
        // N s_15_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
