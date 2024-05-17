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
use TTBR0_EL2_read::*;
use HaveCommonNotPrivateTransExt::*;
use u_get_TTBR0_EL2_Type_CnP::*;
use common::*;
pub fn AArch64_TLBContextEL2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ss: u32,
    va: u64,
    tg: u32,
) -> ProductTypec0d0fb0603850c4c {
    #[derive(Default)]
    struct FunctionState {
        tlbcontext: ProductTypec0d0fb0603850c4c,
        ss: u32,
        va: u64,
        tg: u32,
    }
    let fn_state = FunctionState {
        ss,
        va,
        tg,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_0_0: read-var ss:u32
        let s_0_0: u32 = fn_state.ss;
        // D s_0_1: write-var tlbcontext.11 <= s_0_0
        fn_state.tlbcontext._11 = s_0_0;
        // C s_0_2: const #2u : u32
        let s_0_2: u32 = 2;
        // D s_0_3: write-var tlbcontext.10 <= s_0_2
        fn_state.tlbcontext._10 = s_0_2;
        // D s_0_4: read-var tg:u32
        let s_0_4: u32 = fn_state.tg;
        // D s_0_5: write-var tlbcontext.12 <= s_0_4
        fn_state.tlbcontext._12 = s_0_4;
        // D s_0_6: read-var va:u64
        let s_0_6: u64 = fn_state.va;
        // D s_0_7: write-var tlbcontext.2 <= s_0_6
        fn_state.tlbcontext._2 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call HaveCommonNotPrivateTransExt(s_0_8)
        let s_0_9: bool = HaveCommonNotPrivateTransExt(state, tracer, s_0_8);
        // N s_0_10: branch s_0_9 b3 b1
        if s_0_9 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var tlbcontext.1 <= s_1_0
        fn_state.tlbcontext._1 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_2_0: read-var tlbcontext:struct
        let s_2_0: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call TTBR0_EL2_read(s_3_0)
        let s_3_1: ProductType782ac6922b48c20d = TTBR0_EL2_read(state, tracer, s_3_0);
        // S s_3_2: call _get_TTBR0_EL2_Type_CnP(s_3_1)
        let s_3_2: bool = u_get_TTBR0_EL2_Type_CnP(state, tracer, s_3_1);
        // D s_3_3: write-var tlbcontext.1 <= s_3_2
        fn_state.tlbcontext._1 = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
