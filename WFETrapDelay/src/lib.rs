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
use IsInHost::*;
use u_get_HCR_EL2_Type_TWEDEL::*;
use u_get_SCTLR_EL1_Type_TWEDEn::*;
use u_get_SCR_EL3_Type_TWEDEL::*;
use u_get_SCTLR_EL2_Type_TWEDEL::*;
use u_get_SCTLR_EL1_Type_TWEDEL::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TWEDEn::*;
use u_get_SCTLR_EL2_Type_TWEDEn::*;
use u_get_SCR_EL3_Type_TWEDEn::*;
use common::*;
pub fn WFETrapDelay<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_el: u8,
) -> ProductType6e608f0222d797fa {
    #[derive(Default)]
    struct FunctionState {
        delay: i128,
        delay_enabled: bool,
        target_el: u8,
    }
    let fn_state = FunctionState {
        target_el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType6e608f0222d797fa {
        // D s_0_0: read-var target_el:u8
        let s_0_0: u8 = fn_state.target_el;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 2u16);
        // C s_0_2: const #440u : u32
        let s_0_2: u32 = 440;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // D s_0_5: cmp-eq s_0_1 s_0_4
        let s_0_5: bool = ((s_0_1) == (s_0_4));
        // D s_0_6: not s_0_5
        let s_0_6: bool = !s_0_5;
        // N s_0_7: branch s_0_6 b5 b1
        if s_0_6 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType6e608f0222d797fa {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call IsInHost(s_1_0)
        let s_1_1: bool = IsInHost(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b4 b2
        if s_1_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType6e608f0222d797fa {
        // C s_2_0: const #20784u : u32
        let s_2_0: u32 = 20784;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_SCTLR_EL2_Type_TWEDEn(s_2_1)
        let s_2_2: bool = u_get_SCTLR_EL2_Type_TWEDEn(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // D s_2_7: write-var delay_enabled <= s_2_6
        fn_state.delay_enabled = s_2_6;
        // C s_2_8: const #20784u : u32
        let s_2_8: u32 = 20784;
        // D s_2_9: read-reg s_2_8:struct
        let s_2_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_8 as isize);
            tracer.read_register(s_2_8 as isize, value);
            value
        };
        // D s_2_10: call _get_SCTLR_EL2_Type_TWEDEL(s_2_9)
        let s_2_10: u8 = u_get_SCTLR_EL2_Type_TWEDEL(state, tracer, s_2_9);
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 4u16);
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (s_2_11.value() as i128);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // C s_2_14: const #8s : i
        let s_2_14: i128 = 8;
        // D s_2_15: cast zx s_2_13 -> i
        let s_2_15: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_16: add s_2_15 s_2_14
        let s_2_16: i128 = (s_2_15 + s_2_14);
        // D s_2_17: cast reint s_2_16 -> i64
        let s_2_17: i64 = (s_2_16 as i64);
        // C s_2_18: const #1s : i
        let s_2_18: i128 = 1;
        // D s_2_19: cast zx s_2_17 -> i
        let s_2_19: i128 = (i128::try_from(s_2_17).unwrap());
        // D s_2_20: lsl s_2_18 s_2_19
        let s_2_20: i128 = s_2_18 << s_2_19;
        // D s_2_21: write-var delay <= s_2_20
        fn_state.delay = s_2_20;
        // N s_2_22: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType6e608f0222d797fa {
        // D s_3_0: read-var delay:i
        let s_3_0: i128 = fn_state.delay;
        // D s_3_1: read-var delay_enabled:u8
        let s_3_1: bool = fn_state.delay_enabled;
        // D s_3_2: create-product struct = ["s_3_1", "s_3_0"]
        let s_3_2: ProductType6e608f0222d797fa = ProductType6e608f0222d797fa {
            _0: s_3_1,
            _1: s_3_0,
        };
        // N s_3_3: return s_3_2
        return s_3_2;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType6e608f0222d797fa {
        // C s_4_0: const #90272u : u32
        let s_4_0: u32 = 90272;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_SCTLR_EL1_Type_TWEDEn(s_4_1)
        let s_4_2: bool = u_get_SCTLR_EL1_Type_TWEDEn(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // D s_4_7: write-var delay_enabled <= s_4_6
        fn_state.delay_enabled = s_4_6;
        // C s_4_8: const #90272u : u32
        let s_4_8: u32 = 90272;
        // D s_4_9: read-reg s_4_8:struct
        let s_4_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_8 as isize);
            tracer.read_register(s_4_8 as isize, value);
            value
        };
        // D s_4_10: call _get_SCTLR_EL1_Type_TWEDEL(s_4_9)
        let s_4_10: u8 = u_get_SCTLR_EL1_Type_TWEDEL(state, tracer, s_4_9);
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 4u16);
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (s_4_11.value() as i128);
        // D s_4_13: cast reint s_4_12 -> i64
        let s_4_13: i64 = (s_4_12 as i64);
        // C s_4_14: const #8s : i
        let s_4_14: i128 = 8;
        // D s_4_15: cast zx s_4_13 -> i
        let s_4_15: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_16: add s_4_15 s_4_14
        let s_4_16: i128 = (s_4_15 + s_4_14);
        // D s_4_17: cast reint s_4_16 -> i64
        let s_4_17: i64 = (s_4_16 as i64);
        // C s_4_18: const #1s : i
        let s_4_18: i128 = 1;
        // D s_4_19: cast zx s_4_17 -> i
        let s_4_19: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_20: lsl s_4_18 s_4_19
        let s_4_20: i128 = s_4_18 << s_4_19;
        // D s_4_21: write-var delay <= s_4_20
        fn_state.delay = s_4_20;
        // N s_4_22: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType6e608f0222d797fa {
        // D s_5_0: read-var target_el:u8
        let s_5_0: u8 = fn_state.target_el;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #432u : u32
        let s_5_2: u32 = 432;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // D s_5_6: not s_5_5
        let s_5_6: bool = !s_5_5;
        // N s_5_7: branch s_5_6 b7 b6
        if s_5_6 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType6e608f0222d797fa {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call EL2Enabled(s_6_0)
        let s_6_1: bool = EL2Enabled(state, tracer, s_6_0);
        // N s_6_2: assert s_6_1
        let s_6_2: () = assert!(s_6_1);
        // C s_6_3: const #102552u : u32
        let s_6_3: u32 = 102552;
        // D s_6_4: read-reg s_6_3:struct
        let s_6_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // D s_6_5: call _get_HCR_EL2_Type_TWEDEn(s_6_4)
        let s_6_5: bool = u_get_HCR_EL2_Type_TWEDEn(state, tracer, s_6_4);
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 1u16);
        // C s_6_7: const #1u : u8
        let s_6_7: bool = true;
        // C s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 1u16);
        // D s_6_9: cmp-eq s_6_6 s_6_8
        let s_6_9: bool = ((s_6_6) == (s_6_8));
        // D s_6_10: write-var delay_enabled <= s_6_9
        fn_state.delay_enabled = s_6_9;
        // C s_6_11: const #102552u : u32
        let s_6_11: u32 = 102552;
        // D s_6_12: read-reg s_6_11:struct
        let s_6_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_11 as isize);
            tracer.read_register(s_6_11 as isize, value);
            value
        };
        // D s_6_13: call _get_HCR_EL2_Type_TWEDEL(s_6_12)
        let s_6_13: u8 = u_get_HCR_EL2_Type_TWEDEL(state, tracer, s_6_12);
        // D s_6_14: cast zx s_6_13 -> bv
        let s_6_14: Bits = Bits::new(s_6_13 as u128, 4u16);
        // D s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (s_6_14.value() as i128);
        // D s_6_16: cast reint s_6_15 -> i64
        let s_6_16: i64 = (s_6_15 as i64);
        // C s_6_17: const #8s : i
        let s_6_17: i128 = 8;
        // D s_6_18: cast zx s_6_16 -> i
        let s_6_18: i128 = (i128::try_from(s_6_16).unwrap());
        // D s_6_19: add s_6_18 s_6_17
        let s_6_19: i128 = (s_6_18 + s_6_17);
        // D s_6_20: cast reint s_6_19 -> i64
        let s_6_20: i64 = (s_6_19 as i64);
        // C s_6_21: const #1s : i
        let s_6_21: i128 = 1;
        // D s_6_22: cast zx s_6_20 -> i
        let s_6_22: i128 = (i128::try_from(s_6_20).unwrap());
        // D s_6_23: lsl s_6_21 s_6_22
        let s_6_23: i128 = s_6_21 << s_6_22;
        // D s_6_24: write-var delay <= s_6_23
        fn_state.delay = s_6_23;
        // N s_6_25: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType6e608f0222d797fa {
        // D s_7_0: read-var target_el:u8
        let s_7_0: u8 = fn_state.target_el;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #424u : u32
        let s_7_2: u32 = 424;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // D s_7_6: not s_7_5
        let s_7_6: bool = !s_7_5;
        // N s_7_7: branch s_7_6 b9 b8
        if s_7_6 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType6e608f0222d797fa {
        // C s_8_0: const #90704u : u32
        let s_8_0: u32 = 90704;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_SCR_EL3_Type_TWEDEn(s_8_1)
        let s_8_2: bool = u_get_SCR_EL3_Type_TWEDEn(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // D s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // D s_8_7: write-var delay_enabled <= s_8_6
        fn_state.delay_enabled = s_8_6;
        // C s_8_8: const #90704u : u32
        let s_8_8: u32 = 90704;
        // D s_8_9: read-reg s_8_8:struct
        let s_8_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_8 as isize);
            tracer.read_register(s_8_8 as isize, value);
            value
        };
        // D s_8_10: call _get_SCR_EL3_Type_TWEDEL(s_8_9)
        let s_8_10: u8 = u_get_SCR_EL3_Type_TWEDEL(state, tracer, s_8_9);
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 4u16);
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (s_8_11.value() as i128);
        // D s_8_13: cast reint s_8_12 -> i64
        let s_8_13: i64 = (s_8_12 as i64);
        // C s_8_14: const #8s : i
        let s_8_14: i128 = 8;
        // D s_8_15: cast zx s_8_13 -> i
        let s_8_15: i128 = (i128::try_from(s_8_13).unwrap());
        // D s_8_16: add s_8_15 s_8_14
        let s_8_16: i128 = (s_8_15 + s_8_14);
        // D s_8_17: cast reint s_8_16 -> i64
        let s_8_17: i64 = (s_8_16 as i64);
        // C s_8_18: const #1s : i
        let s_8_18: i128 = 1;
        // D s_8_19: cast zx s_8_17 -> i
        let s_8_19: i128 = (i128::try_from(s_8_17).unwrap());
        // D s_8_20: lsl s_8_18 s_8_19
        let s_8_20: i128 = s_8_18 << s_8_19;
        // D s_8_21: write-var delay <= s_8_20
        fn_state.delay = s_8_20;
        // N s_8_22: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType6e608f0222d797fa {
        // N s_9_0: jump b3
        return block_3(state, tracer, fn_state);
    }
}
