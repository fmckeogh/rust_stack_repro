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
use IsSCTLR2EL2Enabled::*;
use HaveFeatMEC::*;
use u_get_SCTLR2_EL2_Type_EMEC::*;
use AArch64_NSS2TTWParams::*;
use common::*;
pub fn AArch64_RLS2TTWParams<T: Tracer>(
    state: &mut State,
    tracer: &T,
    s1aarch64: bool,
) -> ProductTypeb05ce25a107f0c5e {
    #[derive(Default)]
    struct FunctionState {
        gs_18304: bool,
        walkparams: ProductTypeb05ce25a107f0c5e,
        s1aarch64: bool,
    }
    let fn_state = FunctionState {
        s1aarch64,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_0_0: read-var s1aarch64:u8
        let s_0_0: bool = fn_state.s1aarch64;
        // D s_0_1: call AArch64_NSS2TTWParams(s_0_0)
        let s_0_1: ProductTypeb05ce25a107f0c5e = AArch64_NSS2TTWParams(
            state,
            tracer,
            s_0_0,
        );
        // D s_0_2: write-var walkparams <= s_0_1
        fn_state.walkparams = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveFeatMEC(s_0_3)
        let s_0_4: bool = HaveFeatMEC(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b6 b1
        if s_0_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#18304 <= s_1_0
        fn_state.gs_18304 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_2_0: read-var gs#18304:u8
        let s_2_0: bool = fn_state.gs_18304;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var walkparams.5 <= s_3_0
        fn_state.walkparams._5 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // D s_4_0: read-var walkparams:struct
        let s_4_0: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_5_0: const #102680u : u32
        let s_5_0: u32 = 102680;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_SCTLR2_EL2_Type_EMEC(s_5_1)
        let s_5_2: bool = u_get_SCTLR2_EL2_Type_EMEC(state, tracer, s_5_1);
        // D s_5_3: write-var walkparams.5 <= s_5_2
        fn_state.walkparams._5 = s_5_2;
        // N s_5_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb05ce25a107f0c5e {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call IsSCTLR2EL2Enabled(s_6_0)
        let s_6_1: bool = IsSCTLR2EL2Enabled(state, tracer, s_6_0);
        // D s_6_2: write-var gs#18304 <= s_6_1
        fn_state.gs_18304 = s_6_1;
        // N s_6_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
