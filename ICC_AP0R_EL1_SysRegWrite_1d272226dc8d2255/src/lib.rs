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
use ICC_SRE_EL1_read::*;
use u_get_ICC_SRE_EL1_Type_SRE::*;
use u_get_ICC_SRE_EL2_Type_SRE::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_ICH_HCR_EL2_Type_TALL0::*;
use u_get_SCR_EL3_Type_FIQ::*;
use u_get_ICC_SRE_EL3_Type_SRE::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_FMO::*;
use common::*;
pub fn ICC_AP0R_EL1_SysRegWrite_1d272226dc8d2255<T: Tracer>(
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call EDSCR_read(s_0_0)
        let s_0_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_0);
        // S s_0_2: call _get_EDSCR_Type_SDD(s_0_1)
        let s_0_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_1);
        // C s_0_3: const #90704u : u32
        let s_0_3: u32 = 90704;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCR_EL3_Type_FIQ(s_0_4)
        let s_0_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_0_4);
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call ICC_SRE_EL1_read(s_0_6)
        let s_0_7: ProductType5c790c8ef59cc8b2 = ICC_SRE_EL1_read(state, tracer, s_0_6);
        // S s_0_8: call _get_ICC_SRE_EL1_Type_SRE(s_0_7)
        let s_0_8: bool = u_get_ICC_SRE_EL1_Type_SRE(state, tracer, s_0_7);
        // C s_0_9: const #20992u : u32
        let s_0_9: u32 = 20992;
        // D s_0_10: read-reg s_0_9:struct
        let s_0_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: call _get_ICH_HCR_EL2_Type_TALL0(s_0_10)
        let s_0_11: bool = u_get_ICH_HCR_EL2_Type_TALL0(state, tracer, s_0_10);
        // C s_0_12: const #102552u : u32
        let s_0_12: u32 = 102552;
        // D s_0_13: read-reg s_0_12:struct
        let s_0_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_12 as isize);
            tracer.read_register(s_0_12 as isize, value);
            value
        };
        // D s_0_14: call _get_HCR_EL2_Type_FMO(s_0_13)
        let s_0_14: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_0_13);
        // C s_0_15: const #16368u : u32
        let s_0_15: u32 = 16368;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_ICC_SRE_EL2_Type_SRE(s_0_16)
        let s_0_17: bool = u_get_ICC_SRE_EL2_Type_SRE(state, tracer, s_0_16);
        // C s_0_18: const #10200u : u32
        let s_0_18: u32 = 10200;
        // D s_0_19: read-reg s_0_18:struct
        let s_0_19: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_18 as isize);
            tracer.read_register(s_0_18 as isize, value);
            value
        };
        // D s_0_20: call _get_ICC_SRE_EL3_Type_SRE(s_0_19)
        let s_0_20: bool = u_get_ICC_SRE_EL3_Type_SRE(state, tracer, s_0_19);
        // N s_0_21: panic
        panic!("{:?}", ());
        // N s_0_22: return
        return;
    }
}
