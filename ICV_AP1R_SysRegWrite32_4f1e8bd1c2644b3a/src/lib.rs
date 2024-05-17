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
use u_get_ICC_MSRE_Type_SRE::*;
use u_get_SCR_Type_IRQ::*;
use u_get_ICC_SRE_Type_SRE::*;
use HCR_read::*;
use u_get_SCR_EL3_Type_IRQ::*;
use u_get_ICC_HSRE_Type_SRE::*;
use ICC_HSRE_read::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use ICC_SRE_read::*;
use u_get_SCR_Type_NS::*;
use u_get_ICH_HCR_EL2_Type_TALL1::*;
use u_get_ICH_HCR_Type_TALL1::*;
use u_get_HCR_EL2_Type_IMO::*;
use ICH_HCR_read::*;
use u_get_HCR_Type_IMO::*;
use u_get_HSTR_Type_T12::*;
use common::*;
pub fn ICV_AP1R_SysRegWrite32_4f1e8bd1c2644b3a<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRn: u8,
    opc2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        el: u8,
        coproc: u8,
        opc1: u8,
        CRn: u8,
        opc2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRn,
        opc2,
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
        // C s_0_0: const #90704u : u32
        let s_0_0: u32 = 90704;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_SCR_EL3_Type_IRQ(s_0_1)
        let s_0_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_0_1);
        // C s_0_3: const #20920u : u32
        let s_0_3: u32 = 20920;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCR_Type_IRQ(s_0_4)
        let s_0_5: bool = u_get_SCR_Type_IRQ(state, tracer, s_0_4);
        // C s_0_6: const #104936u : u32
        let s_0_6: u32 = 104936;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: call _get_HSTR_EL2_Type_T12(s_0_7)
        let s_0_8: bool = u_get_HSTR_EL2_Type_T12(state, tracer, s_0_7);
        // C s_0_9: const #() : ()
        let s_0_9: () = ();
        // S s_0_10: call HSTR_read(s_0_9)
        let s_0_10: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_9);
        // S s_0_11: call _get_HSTR_Type_T12(s_0_10)
        let s_0_11: bool = u_get_HSTR_Type_T12(state, tracer, s_0_10);
        // C s_0_12: const #() : ()
        let s_0_12: () = ();
        // S s_0_13: call ICC_SRE_read(s_0_12)
        let s_0_13: ProductType700c18a878c5601b = ICC_SRE_read(state, tracer, s_0_12);
        // S s_0_14: call _get_ICC_SRE_Type_SRE(s_0_13)
        let s_0_14: bool = u_get_ICC_SRE_Type_SRE(state, tracer, s_0_13);
        // C s_0_15: const #20992u : u32
        let s_0_15: u32 = 20992;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_ICH_HCR_EL2_Type_TALL1(s_0_16)
        let s_0_17: bool = u_get_ICH_HCR_EL2_Type_TALL1(state, tracer, s_0_16);
        // C s_0_18: const #() : ()
        let s_0_18: () = ();
        // S s_0_19: call ICH_HCR_read(s_0_18)
        let s_0_19: ProductType700c18a878c5601b = ICH_HCR_read(state, tracer, s_0_18);
        // S s_0_20: call _get_ICH_HCR_Type_TALL1(s_0_19)
        let s_0_20: bool = u_get_ICH_HCR_Type_TALL1(state, tracer, s_0_19);
        // C s_0_21: const #102552u : u32
        let s_0_21: u32 = 102552;
        // D s_0_22: read-reg s_0_21:struct
        let s_0_22: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: call _get_HCR_EL2_Type_IMO(s_0_22)
        let s_0_23: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_0_22);
        // C s_0_24: const #() : ()
        let s_0_24: () = ();
        // S s_0_25: call HCR_read(s_0_24)
        let s_0_25: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_24);
        // S s_0_26: call _get_HCR_Type_IMO(s_0_25)
        let s_0_26: bool = u_get_HCR_Type_IMO(state, tracer, s_0_25);
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call ICC_HSRE_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = ICC_HSRE_read(state, tracer, s_0_27);
        // S s_0_29: call _get_ICC_HSRE_Type_SRE(s_0_28)
        let s_0_29: bool = u_get_ICC_HSRE_Type_SRE(state, tracer, s_0_28);
        // C s_0_30: const #19992u : u32
        let s_0_30: u32 = 19992;
        // D s_0_31: read-reg s_0_30:struct
        let s_0_31: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_30 as isize);
            tracer.read_register(s_0_30 as isize, value);
            value
        };
        // D s_0_32: call _get_ICC_MSRE_Type_SRE(s_0_31)
        let s_0_32: bool = u_get_ICC_MSRE_Type_SRE(state, tracer, s_0_31);
        // C s_0_33: const #20920u : u32
        let s_0_33: u32 = 20920;
        // D s_0_34: read-reg s_0_33:struct
        let s_0_34: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_33 as isize);
            tracer.read_register(s_0_33 as isize, value);
            value
        };
        // D s_0_35: call _get_SCR_Type_NS(s_0_34)
        let s_0_35: bool = u_get_SCR_Type_NS(state, tracer, s_0_34);
        // N s_0_36: panic
        panic!("{:?}", ());
        // N s_0_37: return
        return;
    }
}
