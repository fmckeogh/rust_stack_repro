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
use neq_int::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_CPTR_EL3_Type_TAM::*;
use u_get_SCR_EL3_Type_AMVOFFEN::*;
use u_get_HCR_EL2_Type_NV::*;
use EDSCR_read::*;
use common::*;
pub fn AMEVCNTVOFF0_EL2_SysRegWrite_9d4925d599bf21f3<T: Tracer>(
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
        gs_80358: bool,
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
        // C s_0_12: const #1s : i
        let s_0_12: i128 = 1;
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // S s_0_14: call neq_int(s_0_12, s_0_13)
        let s_0_14: bool = neq_int(state, tracer, s_0_12, s_0_13);
        // N s_0_15: branch s_0_14 b6 b1
        if s_0_14 {
            return block_6(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#80358 <= s_1_0
        fn_state.gs_80358 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#80358:u8
        let s_2_0: bool = fn_state.gs_80358;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // C s_5_1: const #3s : i
        let s_5_1: i128 = 3;
        // S s_5_2: call neq_int(s_5_0, s_5_1)
        let s_5_2: bool = neq_int(state, tracer, s_5_0, s_5_1);
        // N s_5_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // C s_6_1: const #2s : i
        let s_6_1: i128 = 2;
        // S s_6_2: call neq_int(s_6_0, s_6_1)
        let s_6_2: bool = neq_int(state, tracer, s_6_0, s_6_1);
        // D s_6_3: write-var gs#80358 <= s_6_2
        fn_state.gs_80358 = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
