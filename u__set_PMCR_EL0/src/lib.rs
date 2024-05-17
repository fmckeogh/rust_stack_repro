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
use u_get_PMCR_EL0_Type_IDCODE::*;
use Bit::*;
use IsFeatureImplemented::*;
use u_get_PMCR_EL0_Type_P::*;
use u_get_PMCR_EL0_Type_IMP::*;
use u_get_PMCR_EL0_Type_C::*;
use common::*;
pub fn u__set_PMCR_EL0<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        value_name: ProductType5c790c8ef59cc8b2,
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
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // D s_0_2: read-var r.0:struct
        let s_0_2: u64 = fn_state.r._0;
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // C s_0_4: const #1s : i
        let s_0_4: i128 = 1;
        // D s_0_5: cast zx s_0_2 -> bv
        let s_0_5: Bits = Bits::new(s_0_2 as u128, 64u16);
        // D s_0_6: bit-extract s_0_5 s_0_3 s_0_4
        let s_0_6: Bits = (Bits::new(
            ((s_0_5) >> (s_0_3)).value(),
            u16::try_from(s_0_4).unwrap(),
        ));
        // D s_0_7: cast reint s_0_6 -> u8
        let s_0_7: bool = ((s_0_6.value()) != 0);
        // D s_0_8: call Bit(s_0_7)
        let s_0_8: bool = Bit(state, tracer, s_0_7);
        // C s_0_9: const #21016u : u32
        let s_0_9: u32 = 21016;
        // D s_0_10: read-reg s_0_9:struct
        let s_0_10: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // C s_0_11: const #21016u : u32
        let s_0_11: u32 = 21016;
        // N s_0_12: write-reg s_0_11 <= s_0_10
        let s_0_12: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize, s_0_10);
            tracer.write_register(s_0_11 as isize, s_0_10);
        };
        // C s_0_13: const #21016u : u32
        let s_0_13: u32 = 21016;
        // D s_0_14: read-reg s_0_13:struct
        let s_0_14: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // C s_0_15: const #21016u : u32
        let s_0_15: u32 = 21016;
        // N s_0_16: write-reg s_0_15 <= s_0_14
        let s_0_16: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize, s_0_14);
            tracer.write_register(s_0_15 as isize, s_0_14);
        };
        // C s_0_17: const #21016u : u32
        let s_0_17: u32 = 21016;
        // D s_0_18: read-reg s_0_17:struct
        let s_0_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // C s_0_19: const #21016u : u32
        let s_0_19: u32 = 21016;
        // N s_0_20: write-reg s_0_19 <= s_0_18
        let s_0_20: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize, s_0_18);
            tracer.write_register(s_0_19 as isize, s_0_18);
        };
        // C s_0_21: const #162u : u32
        let s_0_21: u32 = 162;
        // S s_0_22: call IsFeatureImplemented(s_0_21)
        let s_0_22: bool = IsFeatureImplemented(state, tracer, s_0_21);
        // S s_0_23: not s_0_22
        let s_0_23: bool = !s_0_22;
        // S s_0_24: not s_0_23
        let s_0_24: bool = !s_0_23;
        // N s_0_25: branch s_0_24 b6 b1
        if s_0_24 {
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
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #21016u : u32
        let s_2_0: u32 = 21016;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_PMCR_EL0_Type_IMP(s_2_1)
        let s_2_2: u8 = u_get_PMCR_EL0_Type_IMP(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 8u16);
        // C s_2_4: const #0u : u8
        let s_2_4: u8 = 0;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 8u16);
        // D s_2_6: cmp-ne s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) != (s_2_5));
        // D s_2_7: not s_2_6
        let s_2_7: bool = !s_2_6;
        // N s_2_8: branch s_2_7 b5 b3
        if s_2_7 {
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
        // D s_4_0: read-var r:struct
        let s_4_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_4_1: call _get_PMCR_EL0_Type_C(s_4_0)
        let s_4_1: bool = u_get_PMCR_EL0_Type_C(state, tracer, s_4_0);
        // C s_4_2: const #21016u : u32
        let s_4_2: u32 = 21016;
        // D s_4_3: read-reg s_4_2:struct
        let s_4_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // C s_4_4: const #21016u : u32
        let s_4_4: u32 = 21016;
        // N s_4_5: write-reg s_4_4 <= s_4_3
        let s_4_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_4 as isize, s_4_3);
            tracer.write_register(s_4_4 as isize, s_4_3);
        };
        // D s_4_6: read-var r:struct
        let s_4_6: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_4_7: call _get_PMCR_EL0_Type_P(s_4_6)
        let s_4_7: bool = u_get_PMCR_EL0_Type_P(state, tracer, s_4_6);
        // C s_4_8: const #21016u : u32
        let s_4_8: u32 = 21016;
        // D s_4_9: read-reg s_4_8:struct
        let s_4_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_8 as isize);
            tracer.read_register(s_4_8 as isize, value);
            value
        };
        // C s_4_10: const #21016u : u32
        let s_4_10: u32 = 21016;
        // N s_4_11: write-reg s_4_10 <= s_4_9
        let s_4_11: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_10 as isize, s_4_9);
            tracer.write_register(s_4_10 as isize, s_4_9);
        };
        // N s_4_12: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var r:struct
        let s_5_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_5_1: call _get_PMCR_EL0_Type_IDCODE(s_5_0)
        let s_5_1: u8 = u_get_PMCR_EL0_Type_IDCODE(state, tracer, s_5_0);
        // C s_5_2: const #21016u : u32
        let s_5_2: u32 = 21016;
        // D s_5_3: read-reg s_5_2:struct
        let s_5_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // C s_5_4: const #21016u : u32
        let s_5_4: u32 = 21016;
        // N s_5_5: write-reg s_5_4 <= s_5_3
        let s_5_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_4 as isize, s_5_3);
            tracer.write_register(s_5_4 as isize, s_5_3);
        };
        // N s_5_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var r:struct
        let s_6_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_6_1: call _get_PMCR_EL0_Type_IMP(s_6_0)
        let s_6_1: u8 = u_get_PMCR_EL0_Type_IMP(state, tracer, s_6_0);
        // C s_6_2: const #21016u : u32
        let s_6_2: u32 = 21016;
        // D s_6_3: read-reg s_6_2:struct
        let s_6_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // C s_6_4: const #21016u : u32
        let s_6_4: u32 = 21016;
        // N s_6_5: write-reg s_6_4 <= s_6_3
        let s_6_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_4 as isize, s_6_3);
            tracer.write_register(s_6_4 as isize, s_6_3);
        };
        // N s_6_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
