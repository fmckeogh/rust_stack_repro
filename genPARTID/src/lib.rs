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
use getMPAM_PARTID::*;
use MAP_vPARTID::*;
use MPAMisVirtual::*;
use u_get_MPAMIDR_EL1_Type_PARTID_MAX::*;
use common::*;
pub fn genPARTID<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    InD: bool,
    InSM: bool,
) -> ProductType4d3ef3a5cd661176 {
    #[derive(Default)]
    struct FunctionState {
        partidel: u16,
        return_value: ProductType4d3ef3a5cd661176,
        el: u8,
        InD: bool,
        InSM: bool,
    }
    let fn_state = FunctionState {
        el,
        InD,
        InSM,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_0_0: read-var el:u8
        let s_0_0: u8 = fn_state.el;
        // D s_0_1: read-var InD:u8
        let s_0_1: bool = fn_state.InD;
        // D s_0_2: read-var InSM:u8
        let s_0_2: bool = fn_state.InSM;
        // D s_0_3: call getMPAM_PARTID(s_0_0, s_0_1, s_0_2)
        let s_0_3: u16 = getMPAM_PARTID(state, tracer, s_0_0, s_0_1, s_0_2);
        // D s_0_4: write-var partidel <= s_0_3
        fn_state.partidel = s_0_3;
        // C s_0_5: const #11032u : u32
        let s_0_5: u32 = 11032;
        // D s_0_6: read-reg s_0_5:struct
        let s_0_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: call _get_MPAMIDR_EL1_Type_PARTID_MAX(s_0_6)
        let s_0_7: u16 = u_get_MPAMIDR_EL1_Type_PARTID_MAX(state, tracer, s_0_6);
        // D s_0_8: read-var partidel:u16
        let s_0_8: u16 = fn_state.partidel;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 16u16);
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (s_0_9.value() as i128);
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: cast zx s_0_7 -> bv
        let s_0_12: Bits = Bits::new(s_0_7 as u128, 16u16);
        // D s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (s_0_12.value() as i128);
        // D s_0_14: cast reint s_0_13 -> i64
        let s_0_14: i64 = (s_0_13 as i64);
        // D s_0_15: cast zx s_0_11 -> i
        let s_0_15: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_16: cast zx s_0_14 -> i
        let s_0_16: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_17: cmp-gt s_0_15 s_0_16
        let s_0_17: bool = ((s_0_15) > (s_0_16));
        // N s_0_18: branch s_0_17 b5 b1
        if s_0_17 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_1_0: read-var el:u8
        let s_1_0: u8 = fn_state.el;
        // D s_1_1: call MPAMisVirtual(s_1_0)
        let s_1_1: bool = MPAMisVirtual(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b4 b2
        if s_1_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_2_0: read-var partidel:u16
        let s_2_0: u16 = fn_state.partidel;
        // C s_2_1: const #0u : u8
        let s_2_1: bool = false;
        // D s_2_2: create-product struct = ["s_2_0", "s_2_1"]
        let s_2_2: ProductType4d3ef3a5cd661176 = ProductType4d3ef3a5cd661176 {
            _0: s_2_0,
            _1: s_2_1,
        };
        // D s_2_3: write-var return_value <= s_2_2
        fn_state.return_value = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_3_0: read-var return_value:struct
        let s_3_0: ProductType4d3ef3a5cd661176 = fn_state.return_value;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // D s_4_0: read-var partidel:u16
        let s_4_0: u16 = fn_state.partidel;
        // D s_4_1: call MAP_vPARTID(s_4_0)
        let s_4_1: ProductType4d3ef3a5cd661176 = MAP_vPARTID(state, tracer, s_4_0);
        // D s_4_2: write-var return_value <= s_4_1
        fn_state.return_value = s_4_1;
        // N s_4_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType4d3ef3a5cd661176 {
        // C s_5_0: const #768u : u32
        let s_5_0: u32 = 768;
        // D s_5_1: read-reg s_5_0:u16
        let s_5_1: u16 = {
            let value = state.read_register::<u16>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #1u : u8
        let s_5_2: bool = true;
        // D s_5_3: create-product struct = ["s_5_1", "s_5_2"]
        let s_5_3: ProductType4d3ef3a5cd661176 = ProductType4d3ef3a5cd661176 {
            _0: s_5_1,
            _1: s_5_2,
        };
        // D s_5_4: write-var return_value <= s_5_3
        fn_state.return_value = s_5_3;
        // N s_5_5: jump b3
        return block_3(state, tracer, fn_state);
    }
}
