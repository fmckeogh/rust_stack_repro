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
use ELUsingAArch32::*;
use SCTLR_NS_write::*;
use CurrentSecurityState::*;
use common::*;
pub fn SCTLR_write<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        gs_36503: bool,
        value_name: ProductType700c18a878c5601b,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #424u : u32
        let s_0_2: u32 = 424;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: call ELUsingAArch32(s_0_3)
        let s_0_4: bool = ELUsingAArch32(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b5 b1
        if s_0_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#36503 <= s_1_0
        fn_state.gs_36503 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#36503:u8
        let s_2_0: bool = fn_state.gs_36503;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var r:struct
        let s_3_0: ProductType700c18a878c5601b = fn_state.r;
        // D s_3_1: call SCTLR_NS_write(s_3_0)
        let s_3_1: () = SCTLR_NS_write(state, tracer, s_3_0);
        // N s_3_2: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:struct
        let s_4_0: ProductType700c18a878c5601b = fn_state.r;
        // C s_4_1: const #16456u : u32
        let s_4_1: u32 = 16456;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call CurrentSecurityState(s_5_0)
        let s_5_1: u32 = CurrentSecurityState(state, tracer, s_5_0);
        // C s_5_2: const #3u : u32
        let s_5_2: u32 = 3;
        // S s_5_3: cmp-eq s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) == (s_5_2));
        // D s_5_4: write-var gs#36503 <= s_5_3
        fn_state.gs_36503 = s_5_3;
        // N s_5_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
