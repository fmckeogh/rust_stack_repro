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
use getMPAM_PMG::*;
use u_get_MPAMIDR_EL1_Type_PMG_MAX::*;
use common::*;
pub fn genPMG<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    InD: bool,
    InSM: bool,
    partid_err: bool,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        pmg_max: i64,
        groupel: u8,
        return_value: u8,
        el: u8,
        InD: bool,
        InSM: bool,
        partid_err: bool,
    }
    let fn_state = FunctionState {
        el,
        InD,
        InSM,
        partid_err,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #11032u : u32
        let s_0_0: u32 = 11032;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_MPAMIDR_EL1_Type_PMG_MAX(s_0_1)
        let s_0_2: u8 = u_get_MPAMIDR_EL1_Type_PMG_MAX(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 8u16);
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // D s_0_6: write-var pmg_max <= s_0_5
        fn_state.pmg_max = s_0_5;
        // D s_0_7: read-var partid_err:u8
        let s_0_7: bool = fn_state.partid_err;
        // N s_0_8: branch s_0_7 b5 b1
        if s_0_7 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_1_0: read-var el:u8
        let s_1_0: u8 = fn_state.el;
        // D s_1_1: read-var InD:u8
        let s_1_1: bool = fn_state.InD;
        // D s_1_2: read-var InSM:u8
        let s_1_2: bool = fn_state.InSM;
        // D s_1_3: call getMPAM_PMG(s_1_0, s_1_1, s_1_2)
        let s_1_3: u8 = getMPAM_PMG(state, tracer, s_1_0, s_1_1, s_1_2);
        // D s_1_4: write-var groupel <= s_1_3
        fn_state.groupel = s_1_3;
        // D s_1_5: read-var groupel:u8
        let s_1_5: u8 = fn_state.groupel;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 8u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var pmg_max:i64
        let s_1_10: i64 = fn_state.pmg_max;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cmp-le s_1_9 s_1_11
        let s_1_12: bool = ((s_1_9) <= (s_1_11));
        // N s_1_13: branch s_1_12 b4 b2
        if s_1_12 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_2_0: const #776u : u32
        let s_2_0: u32 = 776;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: write-var return_value <= s_2_1
        fn_state.return_value = s_2_1;
        // N s_2_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_3_0: read-var return_value:u8
        let s_3_0: u8 = fn_state.return_value;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var groupel:u8
        let s_4_0: u8 = fn_state.groupel;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #776u : u32
        let s_5_0: u32 = 776;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: write-var return_value <= s_5_1
        fn_state.return_value = s_5_1;
        // N s_5_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
