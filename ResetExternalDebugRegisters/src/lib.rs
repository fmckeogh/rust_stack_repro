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
use EDSCR_write::*;
use u_update_EDSCR_Type_RW::*;
use u_update_EDSCR_Type_STATUS::*;
use EDSCR_read::*;
use common::*;
pub fn ResetExternalDebugRegisters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cold_reset: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        cold_reset: bool,
    }
    let fn_state = FunctionState {
        cold_reset,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #14040u : u32
        let s_0_0: u32 = 14040;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #14040u : u32
        let s_0_2: u32 = 14040;
        // N s_0_3: write-reg s_0_2 <= s_0_1
        let s_0_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_2 as isize, s_0_1);
            tracer.write_register(s_0_2 as isize, s_0_1);
        };
        // C s_0_4: const #14040u : u32
        let s_0_4: u32 = 14040;
        // D s_0_5: read-reg s_0_4:struct
        let s_0_5: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // C s_0_6: const #14040u : u32
        let s_0_6: u32 = 14040;
        // N s_0_7: write-reg s_0_6 <= s_0_5
        let s_0_7: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_6 as isize, s_0_5);
            tracer.write_register(s_0_6 as isize, s_0_5);
        };
        // C s_0_8: const #14040u : u32
        let s_0_8: u32 = 14040;
        // D s_0_9: read-reg s_0_8:struct
        let s_0_9: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // C s_0_10: const #14040u : u32
        let s_0_10: u32 = 14040;
        // N s_0_11: write-reg s_0_10 <= s_0_9
        let s_0_11: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_10 as isize, s_0_9);
            tracer.write_register(s_0_10 as isize, s_0_9);
        };
        // D s_0_12: read-var cold_reset:u8
        let s_0_12: bool = fn_state.cold_reset;
        // N s_0_13: branch s_0_12 b2 b1
        if s_0_12 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call EDSCR_read(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_2_0);
        // C s_2_2: const #2u : u8
        let s_2_2: u8 = 2;
        // S s_2_3: call _update_EDSCR_Type_STATUS(s_2_1, s_2_2)
        let s_2_3: ProductType700c18a878c5601b = u_update_EDSCR_Type_STATUS(
            state,
            tracer,
            s_2_1,
            s_2_2,
        );
        // S s_2_4: call EDSCR_write(s_2_3)
        let s_2_4: () = EDSCR_write(state, tracer, s_2_3);
        // C s_2_5: const #() : ()
        let s_2_5: () = ();
        // S s_2_6: call EDSCR_read(s_2_5)
        let s_2_6: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_2_5);
        // C s_2_7: const #15u : u8
        let s_2_7: u8 = 15;
        // S s_2_8: call _update_EDSCR_Type_RW(s_2_6, s_2_7)
        let s_2_8: ProductType700c18a878c5601b = u_update_EDSCR_Type_RW(
            state,
            tracer,
            s_2_6,
            s_2_7,
        );
        // S s_2_9: call EDSCR_write(s_2_8)
        let s_2_9: () = EDSCR_write(state, tracer, s_2_8);
        // N s_2_10: return
        return;
    }
}
