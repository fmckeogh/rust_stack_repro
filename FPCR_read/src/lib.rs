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
use Mk_FPCRType::*;
use u__get_FPCR::*;
use common::*;
pub fn FPCR_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_1819: (),
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        ga_840: ProductType5c790c8ef59cc8b2,
        gs_1819: (),
    }
    let fn_state = FunctionState {
        gs_1819,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_0_0: const #12920u : u32
        let s_0_0: u32 = 12920;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call __get_FPCR(s_0_1)
        let s_0_2: ProductType5c790c8ef59cc8b2 = u__get_FPCR(state, tracer, s_0_1);
        // D s_0_3: write-var ga#840 <= s_0_2
        fn_state.ga_840 = s_0_2;
        // D s_0_4: read-var ga#840.0:struct
        let s_0_4: u64 = fn_state.ga_840._0;
        // D s_0_5: tail-call Mk_FPCRType(s_0_4)
        return Mk_FPCRType(state, tracer, s_0_4);
    }
}
