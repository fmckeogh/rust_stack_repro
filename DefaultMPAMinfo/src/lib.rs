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
use common::*;
pub fn DefaultMPAMinfo<T: Tracer>(
    state: &mut State,
    tracer: &T,
    partidspace: u32,
) -> ProductTypee79b4310dbe79c8c {
    #[derive(Default)]
    struct FunctionState {
        DefaultInfo: ProductTypee79b4310dbe79c8c,
        partidspace: u32,
    }
    let fn_state = FunctionState {
        partidspace,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypee79b4310dbe79c8c {
        // D s_0_0: read-var partidspace:u32
        let s_0_0: u32 = fn_state.partidspace;
        // D s_0_1: write-var DefaultInfo.0 <= s_0_0
        fn_state.DefaultInfo._0 = s_0_0;
        // C s_0_2: const #768u : u32
        let s_0_2: u32 = 768;
        // D s_0_3: read-reg s_0_2:u16
        let s_0_3: u16 = {
            let value = state.read_register::<u16>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: write-var DefaultInfo.1 <= s_0_3
        fn_state.DefaultInfo._1 = s_0_3;
        // C s_0_5: const #776u : u32
        let s_0_5: u32 = 776;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: write-var DefaultInfo.2 <= s_0_6
        fn_state.DefaultInfo._2 = s_0_6;
        // D s_0_8: read-var DefaultInfo:struct
        let s_0_8: ProductTypee79b4310dbe79c8c = fn_state.DefaultInfo;
        // N s_0_9: return s_0_8
        return s_0_8;
    }
}
