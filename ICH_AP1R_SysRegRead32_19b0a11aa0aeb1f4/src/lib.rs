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
use u_get_ICC_HSRE_Type_SRE::*;
use ICC_HSRE_read::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use u_get_HSTR_Type_T12::*;
use u_get_ICC_MSRE_Type_SRE::*;
use common::*;
pub fn ICH_AP1R_SysRegRead32_19b0a11aa0aeb1f4<T: Tracer>(
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
        // C s_0_0: const #104936u : u32
        let s_0_0: u32 = 104936;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_HSTR_EL2_Type_T12(s_0_1)
        let s_0_2: bool = u_get_HSTR_EL2_Type_T12(state, tracer, s_0_1);
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HSTR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_HSTR_Type_T12(s_0_4)
        let s_0_5: bool = u_get_HSTR_Type_T12(state, tracer, s_0_4);
        // C s_0_6: const #() : ()
        let s_0_6: () = ();
        // S s_0_7: call ICC_HSRE_read(s_0_6)
        let s_0_7: ProductType700c18a878c5601b = ICC_HSRE_read(state, tracer, s_0_6);
        // S s_0_8: call _get_ICC_HSRE_Type_SRE(s_0_7)
        let s_0_8: bool = u_get_ICC_HSRE_Type_SRE(state, tracer, s_0_7);
        // C s_0_9: const #19992u : u32
        let s_0_9: u32 = 19992;
        // D s_0_10: read-reg s_0_9:struct
        let s_0_10: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: call _get_ICC_MSRE_Type_SRE(s_0_10)
        let s_0_11: bool = u_get_ICC_MSRE_Type_SRE(state, tracer, s_0_10);
        // N s_0_12: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
