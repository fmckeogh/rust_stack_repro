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
use ELStateUsingAArch32K::*;
use common::*;
pub fn ELStateUsingAArch32<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    secure: bool,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_762: ProductType8b847afc727d5818,
        el: u8,
        secure: bool,
    }
    let fn_state = FunctionState {
        el,
        secure,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var el:u8
        let s_0_0: u8 = fn_state.el;
        // D s_0_1: read-var secure:u8
        let s_0_1: bool = fn_state.secure;
        // D s_0_2: call ELStateUsingAArch32K(s_0_0, s_0_1)
        let s_0_2: ProductType8b847afc727d5818 = ELStateUsingAArch32K(
            state,
            tracer,
            s_0_0,
            s_0_1,
        );
        // D s_0_3: write-var ga#762 <= s_0_2
        fn_state.ga_762 = s_0_2;
        // D s_0_4: read-var ga#762.0:struct
        let s_0_4: bool = fn_state.ga_762._0;
        // D s_0_5: read-var ga#762.1:struct
        let s_0_5: bool = fn_state.ga_762._1;
        // N s_0_6: assert s_0_4
        let s_0_6: () = assert!(s_0_4);
        // N s_0_7: return s_0_5
        return s_0_5;
    }
}
