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
use u_get_EDSCR_Type_SDD::*;
use u_get_CPTR_EL3_Type_TAM::*;
use u_get_SCR_EL3_Type_AMVOFFEN::*;
use u_get_HCR_EL2_Type_NV::*;
use EDSCR_read::*;
use common::*;
pub fn AMEVCNTVOFF0_EL2_SysRegRead_32a7e4332477435d<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #102552u : u32
        let s_0_0: u32 = 102552;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_HCR_EL2_Type_NV(s_0_1)
        let s_0_2: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_1);
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call EDSCR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_EDSCR_Type_SDD(s_0_4)
        let s_0_5: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_4);
        // C s_0_6: const #16840u : u32
        let s_0_6: u32 = 16840;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: call _get_CPTR_EL3_Type_TAM(s_0_7)
        let s_0_8: bool = u_get_CPTR_EL3_Type_TAM(state, tracer, s_0_7);
        // C s_0_9: const #90704u : u32
        let s_0_9: u32 = 90704;
        // D s_0_10: read-reg s_0_9:struct
        let s_0_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: call _get_SCR_EL3_Type_AMVOFFEN(s_0_10)
        let s_0_11: bool = u_get_SCR_EL3_Type_AMVOFFEN(state, tracer, s_0_10);
        // N s_0_12: panic
        panic!("{:?}", ());
        // N s_0_13: return
        return;
    }
}
