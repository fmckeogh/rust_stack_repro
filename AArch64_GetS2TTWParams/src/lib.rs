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
use AArch64_NSS2TTWParams::*;
use HaveSecureEL2Ext::*;
use AArch64_RLS2TTWParams::*;
use AArch64_SS2TTWParams::*;
use Unreachable::*;
use common::*;
pub fn AArch64_GetS2TTWParams<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ss: u32,
    ipaspace: u32,
    s1aarch64: bool,
) -> ProductTypeb05ce25a107f0c5e {
    #[derive(Default)]
    struct FunctionState {
        walkparams: ProductTypeb05ce25a107f0c5e,
        gs_18362: bool,
        ss: u32,
        ipaspace: u32,
        s1aarch64: bool,
    }
    let fn_state = FunctionState {
        ss,
        ipaspace,
        s1aarch64,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_0_0: read-var ss:u32
        let s_0_0: u32 = fn_state.ss;
        // C s_0_1: const #0u : u32
        let s_0_1: u32 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b10 b1
        if s_0_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveSecureEL2Ext(s_1_0)
        let s_1_1: bool = HaveSecureEL2Ext(state, tracer, s_1_0);
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
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#18362 <= s_2_0
        fn_state.gs_18362 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_3_0: read-var gs#18362:u8
        let s_3_0: bool = fn_state.gs_18362;
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
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_4_0: read-var ss:u32
        let s_4_0: u32 = fn_state.ss;
        // C s_4_1: const #2u : u32
        let s_4_1: u32 = 2;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
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
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call Unreachable(s_5_0)
        let s_5_1: () = Unreachable(state, tracer, s_5_0);
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_6_0: read-var walkparams:struct
        let s_6_0: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_7_0: read-var s1aarch64:u8
        let s_7_0: bool = fn_state.s1aarch64;
        // D s_7_1: call AArch64_RLS2TTWParams(s_7_0)
        let s_7_1: ProductTypeb05ce25a107f0c5e = AArch64_RLS2TTWParams(
            state,
            tracer,
            s_7_0,
        );
        // D s_7_2: write-var walkparams <= s_7_1
        fn_state.walkparams = s_7_1;
        // N s_7_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_8_0: read-var ipaspace:u32
        let s_8_0: u32 = fn_state.ipaspace;
        // D s_8_1: read-var s1aarch64:u8
        let s_8_1: bool = fn_state.s1aarch64;
        // D s_8_2: call AArch64_SS2TTWParams(s_8_0, s_8_1)
        let s_8_2: ProductTypeb05ce25a107f0c5e = AArch64_SS2TTWParams(
            state,
            tracer,
            s_8_0,
            s_8_1,
        );
        // D s_8_3: write-var walkparams <= s_8_2
        fn_state.walkparams = s_8_2;
        // N s_8_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_9_0: read-var ss:u32
        let s_9_0: u32 = fn_state.ss;
        // C s_9_1: const #3u : u32
        let s_9_1: u32 = 3;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: write-var gs#18362 <= s_9_2
        fn_state.gs_18362 = s_9_2;
        // N s_9_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_10_0: read-var s1aarch64:u8
        let s_10_0: bool = fn_state.s1aarch64;
        // D s_10_1: call AArch64_NSS2TTWParams(s_10_0)
        let s_10_1: ProductTypeb05ce25a107f0c5e = AArch64_NSS2TTWParams(
            state,
            tracer,
            s_10_0,
        );
        // D s_10_2: write-var walkparams <= s_10_1
        fn_state.walkparams = s_10_1;
        // N s_10_3: jump b6
        return block_6(state, tracer, fn_state);
    }
}
