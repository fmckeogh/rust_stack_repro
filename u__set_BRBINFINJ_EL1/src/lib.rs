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
use u_get_BRBINFINJ_EL1_Type_T::*;
use u_get_BRBINFINJ_EL1_Type_CC::*;
use u_get_BRBINFINJ_EL1_Type_VALID::*;
use u_get_BRBINFINJ_EL1_Type_MPRED::*;
use u_get_BRBINFINJ_EL1_Type_CCU::*;
use u_get_BRBINFINJ_EL1_Type_LASTFAILED::*;
use u_get_BRBINFINJ_EL1_Type_TYPE::*;
use u_get_BRBINFINJ_EL1_Type_EL::*;
use common::*;
pub fn u__set_BRBINFINJ_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        gs_33985: bool,
        gs_33986: bool,
        gs_33984: bool,
        gs_33989: bool,
        gs_33983: bool,
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
        // C s_0_2: const #10168u : u32
        let s_0_2: u32 = 10168;
        // D s_0_3: read-reg s_0_2:struct
        let s_0_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #10168u : u32
        let s_0_4: u32 = 10168;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // C s_0_6: const #10168u : u32
        let s_0_6: u32 = 10168;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // C s_0_8: const #10168u : u32
        let s_0_8: u32 = 10168;
        // N s_0_9: write-reg s_0_8 <= s_0_7
        let s_0_9: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_8 as isize, s_0_7);
            tracer.write_register(s_0_8 as isize, s_0_7);
        };
        // C s_0_10: const #10168u : u32
        let s_0_10: u32 = 10168;
        // D s_0_11: read-reg s_0_10:struct
        let s_0_11: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // C s_0_12: const #10168u : u32
        let s_0_12: u32 = 10168;
        // N s_0_13: write-reg s_0_12 <= s_0_11
        let s_0_13: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_12 as isize, s_0_11);
            tracer.write_register(s_0_12 as isize, s_0_11);
        };
        // C s_0_14: const #10168u : u32
        let s_0_14: u32 = 10168;
        // D s_0_15: read-reg s_0_14:struct
        let s_0_15: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // C s_0_16: const #10168u : u32
        let s_0_16: u32 = 10168;
        // N s_0_17: write-reg s_0_16 <= s_0_15
        let s_0_17: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_16 as isize, s_0_15);
            tracer.write_register(s_0_16 as isize, s_0_15);
        };
        // C s_0_18: const #10168u : u32
        let s_0_18: u32 = 10168;
        // D s_0_19: read-reg s_0_18:struct
        let s_0_19: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_18 as isize);
            tracer.read_register(s_0_18 as isize, value);
            value
        };
        // D s_0_20: call _get_BRBINFINJ_EL1_Type_VALID(s_0_19)
        let s_0_20: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_0_19);
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 2u16);
        // C s_0_22: const #0u : u8
        let s_0_22: u8 = 0;
        // C s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 2u16);
        // D s_0_24: cmp-eq s_0_21 s_0_23
        let s_0_24: bool = ((s_0_21) == (s_0_23));
        // D s_0_25: not s_0_24
        let s_0_25: bool = !s_0_24;
        // N s_0_26: branch s_0_25 b35 b1
        if s_0_25 {
            return block_35(state, tracer, fn_state);
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
        // C s_2_0: const #10168u : u32
        let s_2_0: u32 = 10168;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_BRBINFINJ_EL1_Type_CCU(s_2_1)
        let s_2_2: bool = u_get_BRBINFINJ_EL1_Type_CCU(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b34 b3
        if s_2_6 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #10168u : u32
        let s_3_0: u32 = 10168;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_BRBINFINJ_EL1_Type_VALID(s_3_1)
        let s_3_2: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_3_1);
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // C s_3_4: const #0u : u8
        let s_3_4: u8 = 0;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // D s_3_7: write-var gs#33983 <= s_3_6
        fn_state.gs_33983 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#33983:u8
        let s_4_0: bool = fn_state.gs_33983;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b33 b5
        if s_4_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #10168u : u32
        let s_6_0: u32 = 10168;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_BRBINFINJ_EL1_Type_VALID(s_6_1)
        let s_6_2: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_6_1);
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // C s_6_4: const #0u : u8
        let s_6_4: u8 = 0;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 2u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // D s_6_7: not s_6_6
        let s_6_7: bool = !s_6_6;
        // N s_6_8: branch s_6_7 b32 b7
        if s_6_7 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #10168u : u32
        let s_8_0: u32 = 10168;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_BRBINFINJ_EL1_Type_VALID(s_8_1)
        let s_8_2: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // C s_8_4: const #0u : u8
        let s_8_4: u8 = 0;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 2u16);
        // D s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // N s_8_7: branch s_8_6 b31 b9
        if s_8_6 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #10168u : u32
        let s_9_0: u32 = 10168;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_BRBINFINJ_EL1_Type_VALID(s_9_1)
        let s_9_2: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_9_1);
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 2u16);
        // C s_9_4: const #1u : u8
        let s_9_4: u8 = 1;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 2u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // D s_9_7: write-var gs#33984 <= s_9_6
        fn_state.gs_33984 = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#33984:u8
        let s_10_0: bool = fn_state.gs_33984;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // N s_10_2: branch s_10_1 b30 b11
        if s_10_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #10168u : u32
        let s_12_0: u32 = 10168;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_BRBINFINJ_EL1_Type_VALID(s_12_1)
        let s_12_2: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_12_1);
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // C s_12_4: const #0u : u8
        let s_12_4: u8 = 0;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 2u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // D s_12_7: not s_12_6
        let s_12_7: bool = !s_12_6;
        // N s_12_8: branch s_12_7 b29 b13
        if s_12_7 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #10168u : u32
        let s_14_0: u32 = 10168;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_BRBINFINJ_EL1_Type_VALID(s_14_1)
        let s_14_2: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 2u16);
        // C s_14_4: const #0u : u8
        let s_14_4: u8 = 0;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 2u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // N s_14_7: branch s_14_6 b28 b15
        if s_14_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #10168u : u32
        let s_15_0: u32 = 10168;
        // D s_15_1: read-reg s_15_0:struct
        let s_15_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call _get_BRBINFINJ_EL1_Type_VALID(s_15_1)
        let s_15_2: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_15_1);
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // C s_15_4: const #2u : u8
        let s_15_4: u8 = 2;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 2u16);
        // D s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // D s_15_7: write-var gs#33985 <= s_15_6
        fn_state.gs_33985 = s_15_6;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#33985:u8
        let s_16_0: bool = fn_state.gs_33985;
        // D s_16_1: not s_16_0
        let s_16_1: bool = !s_16_0;
        // N s_16_2: branch s_16_1 b27 b17
        if s_16_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #10168u : u32
        let s_18_0: u32 = 10168;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_BRBINFINJ_EL1_Type_VALID(s_18_1)
        let s_18_2: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // C s_18_4: const #0u : u8
        let s_18_4: u8 = 0;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 2u16);
        // D s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // N s_18_7: branch s_18_6 b26 b19
        if s_18_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #10168u : u32
        let s_19_0: u32 = 10168;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_BRBINFINJ_EL1_Type_VALID(s_19_1)
        let s_19_2: u8 = u_get_BRBINFINJ_EL1_Type_VALID(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 2u16);
        // C s_19_4: const #1u : u8
        let s_19_4: u8 = 1;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 2u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // D s_19_7: write-var gs#33986 <= s_19_6
        fn_state.gs_33986 = s_19_6;
        // N s_19_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#33986:u8
        let s_20_0: bool = fn_state.gs_33986;
        // N s_20_1: branch s_20_0 b25 b21
        if s_20_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #10168u : u32
        let s_21_0: u32 = 10168;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_BRBINFINJ_EL1_Type_TYPE(s_21_1)
        let s_21_2: u8 = u_get_BRBINFINJ_EL1_Type_TYPE(state, tracer, s_21_1);
        // C s_21_3: const #5s : i
        let s_21_3: i128 = 5;
        // D s_21_4: cast zx s_21_2 -> bv
        let s_21_4: Bits = Bits::new(s_21_2 as u128, 6u16);
        // C s_21_5: const #1u : u64
        let s_21_5: u64 = 1;
        // D s_21_6: bit-extract s_21_4 s_21_3 s_21_5
        let s_21_6: Bits = (Bits::new(
            ((s_21_4) >> (s_21_3)).value(),
            u16::try_from(s_21_5).unwrap(),
        ));
        // D s_21_7: cast reint s_21_6 -> u8
        let s_21_7: bool = ((s_21_6.value()) != 0);
        // C s_21_8: const #0s : i
        let s_21_8: i128 = 0;
        // C s_21_9: const #0u : u64
        let s_21_9: u64 = 0;
        // D s_21_10: cast zx s_21_7 -> u64
        let s_21_10: u64 = (s_21_7 as u64);
        // C s_21_11: const #1u : u64
        let s_21_11: u64 = 1;
        // D s_21_12: and s_21_10 s_21_11
        let s_21_12: u64 = ((s_21_10) & (s_21_11));
        // D s_21_13: cmp-eq s_21_12 s_21_11
        let s_21_13: bool = ((s_21_12) == (s_21_11));
        // D s_21_14: lsl s_21_10 s_21_8
        let s_21_14: u64 = s_21_10 << s_21_8;
        // D s_21_15: or s_21_9 s_21_14
        let s_21_15: u64 = ((s_21_9) | (s_21_14));
        // D s_21_16: cmpl s_21_14
        let s_21_16: u64 = !s_21_14;
        // D s_21_17: and s_21_9 s_21_16
        let s_21_17: u64 = ((s_21_9) & (s_21_16));
        // D s_21_18: select s_21_13 s_21_15 s_21_17
        let s_21_18: u64 = if s_21_13 { s_21_15 } else { s_21_17 };
        // D s_21_19: cast trunc s_21_18 -> u8
        let s_21_19: bool = ((s_21_18) != 0);
        // D s_21_20: cast zx s_21_19 -> bv
        let s_21_20: Bits = Bits::new(s_21_19 as u128, 1u16);
        // C s_21_21: const #1u : u8
        let s_21_21: bool = true;
        // C s_21_22: cast zx s_21_21 -> bv
        let s_21_22: Bits = Bits::new(s_21_21 as u128, 1u16);
        // D s_21_23: cmp-eq s_21_20 s_21_22
        let s_21_23: bool = ((s_21_20) == (s_21_22));
        // D s_21_24: write-var gs#33989 <= s_21_23
        fn_state.gs_33989 = s_21_23;
        // N s_21_25: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#33989:u8
        let s_22_0: bool = fn_state.gs_33989;
        // D s_22_1: not s_22_0
        let s_22_1: bool = !s_22_0;
        // N s_22_2: branch s_22_1 b24 b23
        if s_22_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var r:struct
        let s_24_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_24_1: call _get_BRBINFINJ_EL1_Type_MPRED(s_24_0)
        let s_24_1: bool = u_get_BRBINFINJ_EL1_Type_MPRED(state, tracer, s_24_0);
        // C s_24_2: const #10168u : u32
        let s_24_2: u32 = 10168;
        // D s_24_3: read-reg s_24_2:struct
        let s_24_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_2 as isize);
            tracer.read_register(s_24_2 as isize, value);
            value
        };
        // C s_24_4: const #10168u : u32
        let s_24_4: u32 = 10168;
        // N s_24_5: write-reg s_24_4 <= s_24_3
        let s_24_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_24_4 as isize, s_24_3);
            tracer.write_register(s_24_4 as isize, s_24_3);
        };
        // N s_24_6: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#33989 <= s_25_0
        fn_state.gs_33989 = s_25_0;
        // N s_25_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#33986 <= s_26_0
        fn_state.gs_33986 = s_26_0;
        // N s_26_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var r:struct
        let s_27_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_27_1: call _get_BRBINFINJ_EL1_Type_EL(s_27_0)
        let s_27_1: u8 = u_get_BRBINFINJ_EL1_Type_EL(state, tracer, s_27_0);
        // C s_27_2: const #10168u : u32
        let s_27_2: u32 = 10168;
        // D s_27_3: read-reg s_27_2:struct
        let s_27_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_2 as isize);
            tracer.read_register(s_27_2 as isize, value);
            value
        };
        // C s_27_4: const #10168u : u32
        let s_27_4: u32 = 10168;
        // N s_27_5: write-reg s_27_4 <= s_27_3
        let s_27_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_27_4 as isize, s_27_3);
            tracer.write_register(s_27_4 as isize, s_27_3);
        };
        // N s_27_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#33985 <= s_28_0
        fn_state.gs_33985 = s_28_0;
        // N s_28_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var r:struct
        let s_29_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_29_1: call _get_BRBINFINJ_EL1_Type_TYPE(s_29_0)
        let s_29_1: u8 = u_get_BRBINFINJ_EL1_Type_TYPE(state, tracer, s_29_0);
        // C s_29_2: const #10168u : u32
        let s_29_2: u32 = 10168;
        // D s_29_3: read-reg s_29_2:struct
        let s_29_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_2 as isize);
            tracer.read_register(s_29_2 as isize, value);
            value
        };
        // C s_29_4: const #10168u : u32
        let s_29_4: u32 = 10168;
        // N s_29_5: write-reg s_29_4 <= s_29_3
        let s_29_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_29_4 as isize, s_29_3);
            tracer.write_register(s_29_4 as isize, s_29_3);
        };
        // N s_29_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var r:struct
        let s_30_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_30_1: call _get_BRBINFINJ_EL1_Type_T(s_30_0)
        let s_30_1: bool = u_get_BRBINFINJ_EL1_Type_T(state, tracer, s_30_0);
        // C s_30_2: const #10168u : u32
        let s_30_2: u32 = 10168;
        // D s_30_3: read-reg s_30_2:struct
        let s_30_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_2 as isize);
            tracer.read_register(s_30_2 as isize, value);
            value
        };
        // C s_30_4: const #10168u : u32
        let s_30_4: u32 = 10168;
        // N s_30_5: write-reg s_30_4 <= s_30_3
        let s_30_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_30_4 as isize, s_30_3);
            tracer.write_register(s_30_4 as isize, s_30_3);
        };
        // N s_30_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#33984 <= s_31_0
        fn_state.gs_33984 = s_31_0;
        // N s_31_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var r:struct
        let s_32_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_32_1: call _get_BRBINFINJ_EL1_Type_LASTFAILED(s_32_0)
        let s_32_1: bool = u_get_BRBINFINJ_EL1_Type_LASTFAILED(state, tracer, s_32_0);
        // C s_32_2: const #10168u : u32
        let s_32_2: u32 = 10168;
        // D s_32_3: read-reg s_32_2:struct
        let s_32_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_32_2 as isize);
            tracer.read_register(s_32_2 as isize, value);
            value
        };
        // C s_32_4: const #10168u : u32
        let s_32_4: u32 = 10168;
        // N s_32_5: write-reg s_32_4 <= s_32_3
        let s_32_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_32_4 as isize, s_32_3);
            tracer.write_register(s_32_4 as isize, s_32_3);
        };
        // N s_32_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var r:struct
        let s_33_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_33_1: call _get_BRBINFINJ_EL1_Type_CC(s_33_0)
        let s_33_1: u16 = u_get_BRBINFINJ_EL1_Type_CC(state, tracer, s_33_0);
        // C s_33_2: const #10168u : u32
        let s_33_2: u32 = 10168;
        // D s_33_3: read-reg s_33_2:struct
        let s_33_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_33_2 as isize);
            tracer.read_register(s_33_2 as isize, value);
            value
        };
        // C s_33_4: const #10168u : u32
        let s_33_4: u32 = 10168;
        // N s_33_5: write-reg s_33_4 <= s_33_3
        let s_33_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_33_4 as isize, s_33_3);
            tracer.write_register(s_33_4 as isize, s_33_3);
        };
        // N s_33_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#33983 <= s_34_0
        fn_state.gs_33983 = s_34_0;
        // N s_34_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var r:struct
        let s_35_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_35_1: call _get_BRBINFINJ_EL1_Type_CCU(s_35_0)
        let s_35_1: bool = u_get_BRBINFINJ_EL1_Type_CCU(state, tracer, s_35_0);
        // C s_35_2: const #10168u : u32
        let s_35_2: u32 = 10168;
        // D s_35_3: read-reg s_35_2:struct
        let s_35_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_2 as isize);
            tracer.read_register(s_35_2 as isize, value);
            value
        };
        // C s_35_4: const #10168u : u32
        let s_35_4: u32 = 10168;
        // N s_35_5: write-reg s_35_4 <= s_35_3
        let s_35_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_35_4 as isize, s_35_3);
            tracer.write_register(s_35_4 as isize, s_35_3);
        };
        // N s_35_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
