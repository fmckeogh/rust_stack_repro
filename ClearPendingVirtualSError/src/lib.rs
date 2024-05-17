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
use u_update_HCR_Type_VA::*;
use HCR_write::*;
use ELUsingAArch32::*;
use HCR_read::*;
use common::*;
pub fn ClearPendingVirtualSError<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_10056: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_10056: (),
    }
    let fn_state = FunctionState {
        gs_10056,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #432u : u32
        let s_0_0: u32 = 432;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call ELUsingAArch32(s_0_1)
        let s_0_2: bool = ELUsingAArch32(state, tracer, s_0_1);
        // N s_0_3: branch s_0_2 b2 b1
        if s_0_2 {
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
        // C s_1_0: const #102552u : u32
        let s_1_0: u32 = 102552;
        // D s_1_1: read-reg s_1_0:struct
        let s_1_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #102552u : u32
        let s_1_2: u32 = 102552;
        // N s_1_3: write-reg s_1_2 <= s_1_1
        let s_1_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_1_2 as isize, s_1_1);
            tracer.write_register(s_1_2 as isize, s_1_1);
        };
        // N s_1_4: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HCR_read(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_2_0);
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // S s_2_3: call _update_HCR_Type_VA(s_2_1, s_2_2)
        let s_2_3: ProductType700c18a878c5601b = u_update_HCR_Type_VA(
            state,
            tracer,
            s_2_1,
            s_2_2,
        );
        // S s_2_4: call HCR_write(s_2_3)
        let s_2_4: () = HCR_write(state, tracer, s_2_3);
        // N s_2_5: return
        return;
    }
}
