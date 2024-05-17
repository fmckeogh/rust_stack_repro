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
use u_get_SCTLR_EL1_Type_UMA::*;
use X_set::*;
use u_get_HCR_EL2_Type_E2H::*;
use AArch64_SystemAccessTrap::*;
use Zeros::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use common::*;
pub fn DAIF_SysRegRead_102c97f1f2780854<T: Tracer>(
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
        u__SCTLR_EL1_UMA: bool,
        u__HCR_EL2_TGE: bool,
        gs_58297: bool,
        u__PSTATE_EL: u8,
        gs_58293: bool,
        gs_58292: bool,
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
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var __PSTATE_EL <= s_0_1
        fn_state.u__PSTATE_EL = s_0_1;
        // C s_0_3: const #90272u : u32
        let s_0_3: u32 = 90272;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCTLR_EL1_Type_UMA(s_0_4)
        let s_0_5: bool = u_get_SCTLR_EL1_Type_UMA(state, tracer, s_0_4);
        // D s_0_6: write-var __SCTLR_EL1_UMA <= s_0_5
        fn_state.u__SCTLR_EL1_UMA = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_TGE(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_TGE <= s_0_9
        fn_state.u__HCR_EL2_TGE = s_0_9;
        // D s_0_11: read-var __PSTATE_EL:u8
        let s_0_11: u8 = fn_state.u__PSTATE_EL;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #448u : u32
        let s_0_13: u32 = 448;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_12 s_0_15
        let s_0_16: bool = ((s_0_12) == (s_0_15));
        // N s_0_17: branch s_0_16 b8 b1
        if s_0_16 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var __PSTATE_EL:u8
        let s_1_0: u8 = fn_state.u__PSTATE_EL;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #440u : u32
        let s_1_2: u32 = 440;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // N s_1_6: branch s_1_5 b7 b2
        if s_1_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var __PSTATE_EL:u8
        let s_2_0: u8 = fn_state.u__PSTATE_EL;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #432u : u32
        let s_2_2: u32 = 432;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // N s_2_6: branch s_2_5 b6 b3
        if s_2_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var __PSTATE_EL:u8
        let s_3_0: u8 = fn_state.u__PSTATE_EL;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #424u : u32
        let s_3_2: u32 = 424;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // C s_5_1: const #54s : i
        let s_5_1: i128 = 54;
        // S s_5_2: call Zeros(s_5_1)
        let s_5_2: Bits = Zeros(state, tracer, s_5_1);
        // S s_5_3: cast reint s_5_2 -> u54
        let s_5_3: u64 = (s_5_2.value() as u64);
        // C s_5_4: const #16972u : u32
        let s_5_4: u32 = 16972;
        // D s_5_5: read-reg s_5_4:u8
        let s_5_5: bool = {
            let value = state.read_register::<bool>(s_5_4 as isize);
            tracer.read_register(s_5_4 as isize, value);
            value
        };
        // C s_5_6: const #16968u : u32
        let s_5_6: u32 = 16968;
        // D s_5_7: read-reg s_5_6:u8
        let s_5_7: bool = {
            let value = state.read_register::<bool>(s_5_6 as isize);
            tracer.read_register(s_5_6 as isize, value);
            value
        };
        // C s_5_8: const #16979u : u32
        let s_5_8: u32 = 16979;
        // D s_5_9: read-reg s_5_8:u8
        let s_5_9: bool = {
            let value = state.read_register::<bool>(s_5_8 as isize);
            tracer.read_register(s_5_8 as isize, value);
            value
        };
        // C s_5_10: const #16977u : u32
        let s_5_10: u32 = 16977;
        // D s_5_11: read-reg s_5_10:u8
        let s_5_11: bool = {
            let value = state.read_register::<bool>(s_5_10 as isize);
            tracer.read_register(s_5_10 as isize, value);
            value
        };
        // D s_5_12: cast zx s_5_9 -> bv
        let s_5_12: Bits = Bits::new(s_5_9 as u128, 1u16);
        // D s_5_13: cast zx s_5_11 -> bv
        let s_5_13: Bits = Bits::new(s_5_11 as u128, 1u16);
        // D s_5_14: cast reint s_5_12 -> u128
        let s_5_14: u128 = (s_5_12.value() as u128);
        // D s_5_15: size-of s_5_12
        let s_5_15: u16 = s_5_12.length();
        // D s_5_16: cast reint s_5_13 -> u128
        let s_5_16: u128 = (s_5_13.value() as u128);
        // D s_5_17: size-of s_5_13
        let s_5_17: u16 = s_5_13.length();
        // D s_5_18: lsl s_5_14 s_5_17
        let s_5_18: u128 = s_5_14 << s_5_17;
        // D s_5_19: or s_5_18 s_5_16
        let s_5_19: u128 = ((s_5_18) | (s_5_16));
        // D s_5_20: add s_5_15 s_5_17
        let s_5_20: u16 = (s_5_15 + s_5_17);
        // D s_5_21: create-bits s_5_19 s_5_20
        let s_5_21: Bits = Bits::new(s_5_19, s_5_20);
        // D s_5_22: cast reint s_5_21 -> u8
        let s_5_22: u8 = (s_5_21.value() as u8);
        // D s_5_23: cast zx s_5_7 -> bv
        let s_5_23: Bits = Bits::new(s_5_7 as u128, 1u16);
        // D s_5_24: cast zx s_5_22 -> bv
        let s_5_24: Bits = Bits::new(s_5_22 as u128, 2u16);
        // D s_5_25: cast reint s_5_23 -> u128
        let s_5_25: u128 = (s_5_23.value() as u128);
        // D s_5_26: size-of s_5_23
        let s_5_26: u16 = s_5_23.length();
        // D s_5_27: cast reint s_5_24 -> u128
        let s_5_27: u128 = (s_5_24.value() as u128);
        // D s_5_28: size-of s_5_24
        let s_5_28: u16 = s_5_24.length();
        // D s_5_29: lsl s_5_25 s_5_28
        let s_5_29: u128 = s_5_25 << s_5_28;
        // D s_5_30: or s_5_29 s_5_27
        let s_5_30: u128 = ((s_5_29) | (s_5_27));
        // D s_5_31: add s_5_26 s_5_28
        let s_5_31: u16 = (s_5_26 + s_5_28);
        // D s_5_32: create-bits s_5_30 s_5_31
        let s_5_32: Bits = Bits::new(s_5_30, s_5_31);
        // D s_5_33: cast reint s_5_32 -> u8
        let s_5_33: u8 = (s_5_32.value() as u8);
        // D s_5_34: cast zx s_5_5 -> bv
        let s_5_34: Bits = Bits::new(s_5_5 as u128, 1u16);
        // D s_5_35: cast zx s_5_33 -> bv
        let s_5_35: Bits = Bits::new(s_5_33 as u128, 3u16);
        // D s_5_36: cast reint s_5_34 -> u128
        let s_5_36: u128 = (s_5_34.value() as u128);
        // D s_5_37: size-of s_5_34
        let s_5_37: u16 = s_5_34.length();
        // D s_5_38: cast reint s_5_35 -> u128
        let s_5_38: u128 = (s_5_35.value() as u128);
        // D s_5_39: size-of s_5_35
        let s_5_39: u16 = s_5_35.length();
        // D s_5_40: lsl s_5_36 s_5_39
        let s_5_40: u128 = s_5_36 << s_5_39;
        // D s_5_41: or s_5_40 s_5_38
        let s_5_41: u128 = ((s_5_40) | (s_5_38));
        // D s_5_42: add s_5_37 s_5_39
        let s_5_42: u16 = (s_5_37 + s_5_39);
        // D s_5_43: create-bits s_5_41 s_5_42
        let s_5_43: Bits = Bits::new(s_5_41, s_5_42);
        // D s_5_44: cast reint s_5_43 -> u8
        let s_5_44: u8 = (s_5_43.value() as u8);
        // S s_5_45: cast zx s_5_3 -> bv
        let s_5_45: Bits = Bits::new(s_5_3 as u128, 54u16);
        // D s_5_46: cast zx s_5_44 -> bv
        let s_5_46: Bits = Bits::new(s_5_44 as u128, 4u16);
        // S s_5_47: cast reint s_5_45 -> u128
        let s_5_47: u128 = (s_5_45.value() as u128);
        // D s_5_48: size-of s_5_45
        let s_5_48: u16 = s_5_45.length();
        // D s_5_49: cast reint s_5_46 -> u128
        let s_5_49: u128 = (s_5_46.value() as u128);
        // D s_5_50: size-of s_5_46
        let s_5_50: u16 = s_5_46.length();
        // D s_5_51: lsl s_5_47 s_5_50
        let s_5_51: u128 = s_5_47 << s_5_50;
        // D s_5_52: or s_5_51 s_5_49
        let s_5_52: u128 = ((s_5_51) | (s_5_49));
        // D s_5_53: add s_5_48 s_5_50
        let s_5_53: u16 = (s_5_48 + s_5_50);
        // D s_5_54: create-bits s_5_52 s_5_53
        let s_5_54: Bits = Bits::new(s_5_52, s_5_53);
        // D s_5_55: cast reint s_5_54 -> u58
        let s_5_55: u64 = (s_5_54.value() as u64);
        // C s_5_56: const #6s : i
        let s_5_56: i128 = 6;
        // S s_5_57: call Zeros(s_5_56)
        let s_5_57: Bits = Zeros(state, tracer, s_5_56);
        // S s_5_58: cast reint s_5_57 -> u8
        let s_5_58: u8 = (s_5_57.value() as u8);
        // D s_5_59: cast zx s_5_55 -> bv
        let s_5_59: Bits = Bits::new(s_5_55 as u128, 58u16);
        // S s_5_60: cast zx s_5_58 -> bv
        let s_5_60: Bits = Bits::new(s_5_58 as u128, 6u16);
        // D s_5_61: cast reint s_5_59 -> u128
        let s_5_61: u128 = (s_5_59.value() as u128);
        // D s_5_62: size-of s_5_59
        let s_5_62: u16 = s_5_59.length();
        // S s_5_63: cast reint s_5_60 -> u128
        let s_5_63: u128 = (s_5_60.value() as u128);
        // D s_5_64: size-of s_5_60
        let s_5_64: u16 = s_5_60.length();
        // D s_5_65: lsl s_5_61 s_5_64
        let s_5_65: u128 = s_5_61 << s_5_64;
        // D s_5_66: or s_5_65 s_5_63
        let s_5_66: u128 = ((s_5_65) | (s_5_63));
        // D s_5_67: add s_5_62 s_5_64
        let s_5_67: u16 = (s_5_62 + s_5_64);
        // D s_5_68: create-bits s_5_66 s_5_67
        let s_5_68: Bits = Bits::new(s_5_66, s_5_67);
        // D s_5_69: cast reint s_5_68 -> u64
        let s_5_69: u64 = (s_5_68.value() as u64);
        // D s_5_70: cast zx s_5_69 -> bv
        let s_5_70: Bits = Bits::new(s_5_69 as u128, 64u16);
        // D s_5_71: read-var t:i
        let s_5_71: i128 = fn_state.t;
        // D s_5_72: call X_set(s_5_71, s_5_0, s_5_70)
        let s_5_72: () = X_set(state, tracer, s_5_71, s_5_0, s_5_70);
        // N s_5_73: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #54s : i
        let s_6_1: i128 = 54;
        // S s_6_2: call Zeros(s_6_1)
        let s_6_2: Bits = Zeros(state, tracer, s_6_1);
        // S s_6_3: cast reint s_6_2 -> u54
        let s_6_3: u64 = (s_6_2.value() as u64);
        // C s_6_4: const #16972u : u32
        let s_6_4: u32 = 16972;
        // D s_6_5: read-reg s_6_4:u8
        let s_6_5: bool = {
            let value = state.read_register::<bool>(s_6_4 as isize);
            tracer.read_register(s_6_4 as isize, value);
            value
        };
        // C s_6_6: const #16968u : u32
        let s_6_6: u32 = 16968;
        // D s_6_7: read-reg s_6_6:u8
        let s_6_7: bool = {
            let value = state.read_register::<bool>(s_6_6 as isize);
            tracer.read_register(s_6_6 as isize, value);
            value
        };
        // C s_6_8: const #16979u : u32
        let s_6_8: u32 = 16979;
        // D s_6_9: read-reg s_6_8:u8
        let s_6_9: bool = {
            let value = state.read_register::<bool>(s_6_8 as isize);
            tracer.read_register(s_6_8 as isize, value);
            value
        };
        // C s_6_10: const #16977u : u32
        let s_6_10: u32 = 16977;
        // D s_6_11: read-reg s_6_10:u8
        let s_6_11: bool = {
            let value = state.read_register::<bool>(s_6_10 as isize);
            tracer.read_register(s_6_10 as isize, value);
            value
        };
        // D s_6_12: cast zx s_6_9 -> bv
        let s_6_12: Bits = Bits::new(s_6_9 as u128, 1u16);
        // D s_6_13: cast zx s_6_11 -> bv
        let s_6_13: Bits = Bits::new(s_6_11 as u128, 1u16);
        // D s_6_14: cast reint s_6_12 -> u128
        let s_6_14: u128 = (s_6_12.value() as u128);
        // D s_6_15: size-of s_6_12
        let s_6_15: u16 = s_6_12.length();
        // D s_6_16: cast reint s_6_13 -> u128
        let s_6_16: u128 = (s_6_13.value() as u128);
        // D s_6_17: size-of s_6_13
        let s_6_17: u16 = s_6_13.length();
        // D s_6_18: lsl s_6_14 s_6_17
        let s_6_18: u128 = s_6_14 << s_6_17;
        // D s_6_19: or s_6_18 s_6_16
        let s_6_19: u128 = ((s_6_18) | (s_6_16));
        // D s_6_20: add s_6_15 s_6_17
        let s_6_20: u16 = (s_6_15 + s_6_17);
        // D s_6_21: create-bits s_6_19 s_6_20
        let s_6_21: Bits = Bits::new(s_6_19, s_6_20);
        // D s_6_22: cast reint s_6_21 -> u8
        let s_6_22: u8 = (s_6_21.value() as u8);
        // D s_6_23: cast zx s_6_7 -> bv
        let s_6_23: Bits = Bits::new(s_6_7 as u128, 1u16);
        // D s_6_24: cast zx s_6_22 -> bv
        let s_6_24: Bits = Bits::new(s_6_22 as u128, 2u16);
        // D s_6_25: cast reint s_6_23 -> u128
        let s_6_25: u128 = (s_6_23.value() as u128);
        // D s_6_26: size-of s_6_23
        let s_6_26: u16 = s_6_23.length();
        // D s_6_27: cast reint s_6_24 -> u128
        let s_6_27: u128 = (s_6_24.value() as u128);
        // D s_6_28: size-of s_6_24
        let s_6_28: u16 = s_6_24.length();
        // D s_6_29: lsl s_6_25 s_6_28
        let s_6_29: u128 = s_6_25 << s_6_28;
        // D s_6_30: or s_6_29 s_6_27
        let s_6_30: u128 = ((s_6_29) | (s_6_27));
        // D s_6_31: add s_6_26 s_6_28
        let s_6_31: u16 = (s_6_26 + s_6_28);
        // D s_6_32: create-bits s_6_30 s_6_31
        let s_6_32: Bits = Bits::new(s_6_30, s_6_31);
        // D s_6_33: cast reint s_6_32 -> u8
        let s_6_33: u8 = (s_6_32.value() as u8);
        // D s_6_34: cast zx s_6_5 -> bv
        let s_6_34: Bits = Bits::new(s_6_5 as u128, 1u16);
        // D s_6_35: cast zx s_6_33 -> bv
        let s_6_35: Bits = Bits::new(s_6_33 as u128, 3u16);
        // D s_6_36: cast reint s_6_34 -> u128
        let s_6_36: u128 = (s_6_34.value() as u128);
        // D s_6_37: size-of s_6_34
        let s_6_37: u16 = s_6_34.length();
        // D s_6_38: cast reint s_6_35 -> u128
        let s_6_38: u128 = (s_6_35.value() as u128);
        // D s_6_39: size-of s_6_35
        let s_6_39: u16 = s_6_35.length();
        // D s_6_40: lsl s_6_36 s_6_39
        let s_6_40: u128 = s_6_36 << s_6_39;
        // D s_6_41: or s_6_40 s_6_38
        let s_6_41: u128 = ((s_6_40) | (s_6_38));
        // D s_6_42: add s_6_37 s_6_39
        let s_6_42: u16 = (s_6_37 + s_6_39);
        // D s_6_43: create-bits s_6_41 s_6_42
        let s_6_43: Bits = Bits::new(s_6_41, s_6_42);
        // D s_6_44: cast reint s_6_43 -> u8
        let s_6_44: u8 = (s_6_43.value() as u8);
        // S s_6_45: cast zx s_6_3 -> bv
        let s_6_45: Bits = Bits::new(s_6_3 as u128, 54u16);
        // D s_6_46: cast zx s_6_44 -> bv
        let s_6_46: Bits = Bits::new(s_6_44 as u128, 4u16);
        // S s_6_47: cast reint s_6_45 -> u128
        let s_6_47: u128 = (s_6_45.value() as u128);
        // D s_6_48: size-of s_6_45
        let s_6_48: u16 = s_6_45.length();
        // D s_6_49: cast reint s_6_46 -> u128
        let s_6_49: u128 = (s_6_46.value() as u128);
        // D s_6_50: size-of s_6_46
        let s_6_50: u16 = s_6_46.length();
        // D s_6_51: lsl s_6_47 s_6_50
        let s_6_51: u128 = s_6_47 << s_6_50;
        // D s_6_52: or s_6_51 s_6_49
        let s_6_52: u128 = ((s_6_51) | (s_6_49));
        // D s_6_53: add s_6_48 s_6_50
        let s_6_53: u16 = (s_6_48 + s_6_50);
        // D s_6_54: create-bits s_6_52 s_6_53
        let s_6_54: Bits = Bits::new(s_6_52, s_6_53);
        // D s_6_55: cast reint s_6_54 -> u58
        let s_6_55: u64 = (s_6_54.value() as u64);
        // C s_6_56: const #6s : i
        let s_6_56: i128 = 6;
        // S s_6_57: call Zeros(s_6_56)
        let s_6_57: Bits = Zeros(state, tracer, s_6_56);
        // S s_6_58: cast reint s_6_57 -> u8
        let s_6_58: u8 = (s_6_57.value() as u8);
        // D s_6_59: cast zx s_6_55 -> bv
        let s_6_59: Bits = Bits::new(s_6_55 as u128, 58u16);
        // S s_6_60: cast zx s_6_58 -> bv
        let s_6_60: Bits = Bits::new(s_6_58 as u128, 6u16);
        // D s_6_61: cast reint s_6_59 -> u128
        let s_6_61: u128 = (s_6_59.value() as u128);
        // D s_6_62: size-of s_6_59
        let s_6_62: u16 = s_6_59.length();
        // S s_6_63: cast reint s_6_60 -> u128
        let s_6_63: u128 = (s_6_60.value() as u128);
        // D s_6_64: size-of s_6_60
        let s_6_64: u16 = s_6_60.length();
        // D s_6_65: lsl s_6_61 s_6_64
        let s_6_65: u128 = s_6_61 << s_6_64;
        // D s_6_66: or s_6_65 s_6_63
        let s_6_66: u128 = ((s_6_65) | (s_6_63));
        // D s_6_67: add s_6_62 s_6_64
        let s_6_67: u16 = (s_6_62 + s_6_64);
        // D s_6_68: create-bits s_6_66 s_6_67
        let s_6_68: Bits = Bits::new(s_6_66, s_6_67);
        // D s_6_69: cast reint s_6_68 -> u64
        let s_6_69: u64 = (s_6_68.value() as u64);
        // D s_6_70: cast zx s_6_69 -> bv
        let s_6_70: Bits = Bits::new(s_6_69 as u128, 64u16);
        // D s_6_71: read-var t:i
        let s_6_71: i128 = fn_state.t;
        // D s_6_72: call X_set(s_6_71, s_6_0, s_6_70)
        let s_6_72: () = X_set(state, tracer, s_6_71, s_6_0, s_6_70);
        // N s_6_73: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #54s : i
        let s_7_1: i128 = 54;
        // S s_7_2: call Zeros(s_7_1)
        let s_7_2: Bits = Zeros(state, tracer, s_7_1);
        // S s_7_3: cast reint s_7_2 -> u54
        let s_7_3: u64 = (s_7_2.value() as u64);
        // C s_7_4: const #16972u : u32
        let s_7_4: u32 = 16972;
        // D s_7_5: read-reg s_7_4:u8
        let s_7_5: bool = {
            let value = state.read_register::<bool>(s_7_4 as isize);
            tracer.read_register(s_7_4 as isize, value);
            value
        };
        // C s_7_6: const #16968u : u32
        let s_7_6: u32 = 16968;
        // D s_7_7: read-reg s_7_6:u8
        let s_7_7: bool = {
            let value = state.read_register::<bool>(s_7_6 as isize);
            tracer.read_register(s_7_6 as isize, value);
            value
        };
        // C s_7_8: const #16979u : u32
        let s_7_8: u32 = 16979;
        // D s_7_9: read-reg s_7_8:u8
        let s_7_9: bool = {
            let value = state.read_register::<bool>(s_7_8 as isize);
            tracer.read_register(s_7_8 as isize, value);
            value
        };
        // C s_7_10: const #16977u : u32
        let s_7_10: u32 = 16977;
        // D s_7_11: read-reg s_7_10:u8
        let s_7_11: bool = {
            let value = state.read_register::<bool>(s_7_10 as isize);
            tracer.read_register(s_7_10 as isize, value);
            value
        };
        // D s_7_12: cast zx s_7_9 -> bv
        let s_7_12: Bits = Bits::new(s_7_9 as u128, 1u16);
        // D s_7_13: cast zx s_7_11 -> bv
        let s_7_13: Bits = Bits::new(s_7_11 as u128, 1u16);
        // D s_7_14: cast reint s_7_12 -> u128
        let s_7_14: u128 = (s_7_12.value() as u128);
        // D s_7_15: size-of s_7_12
        let s_7_15: u16 = s_7_12.length();
        // D s_7_16: cast reint s_7_13 -> u128
        let s_7_16: u128 = (s_7_13.value() as u128);
        // D s_7_17: size-of s_7_13
        let s_7_17: u16 = s_7_13.length();
        // D s_7_18: lsl s_7_14 s_7_17
        let s_7_18: u128 = s_7_14 << s_7_17;
        // D s_7_19: or s_7_18 s_7_16
        let s_7_19: u128 = ((s_7_18) | (s_7_16));
        // D s_7_20: add s_7_15 s_7_17
        let s_7_20: u16 = (s_7_15 + s_7_17);
        // D s_7_21: create-bits s_7_19 s_7_20
        let s_7_21: Bits = Bits::new(s_7_19, s_7_20);
        // D s_7_22: cast reint s_7_21 -> u8
        let s_7_22: u8 = (s_7_21.value() as u8);
        // D s_7_23: cast zx s_7_7 -> bv
        let s_7_23: Bits = Bits::new(s_7_7 as u128, 1u16);
        // D s_7_24: cast zx s_7_22 -> bv
        let s_7_24: Bits = Bits::new(s_7_22 as u128, 2u16);
        // D s_7_25: cast reint s_7_23 -> u128
        let s_7_25: u128 = (s_7_23.value() as u128);
        // D s_7_26: size-of s_7_23
        let s_7_26: u16 = s_7_23.length();
        // D s_7_27: cast reint s_7_24 -> u128
        let s_7_27: u128 = (s_7_24.value() as u128);
        // D s_7_28: size-of s_7_24
        let s_7_28: u16 = s_7_24.length();
        // D s_7_29: lsl s_7_25 s_7_28
        let s_7_29: u128 = s_7_25 << s_7_28;
        // D s_7_30: or s_7_29 s_7_27
        let s_7_30: u128 = ((s_7_29) | (s_7_27));
        // D s_7_31: add s_7_26 s_7_28
        let s_7_31: u16 = (s_7_26 + s_7_28);
        // D s_7_32: create-bits s_7_30 s_7_31
        let s_7_32: Bits = Bits::new(s_7_30, s_7_31);
        // D s_7_33: cast reint s_7_32 -> u8
        let s_7_33: u8 = (s_7_32.value() as u8);
        // D s_7_34: cast zx s_7_5 -> bv
        let s_7_34: Bits = Bits::new(s_7_5 as u128, 1u16);
        // D s_7_35: cast zx s_7_33 -> bv
        let s_7_35: Bits = Bits::new(s_7_33 as u128, 3u16);
        // D s_7_36: cast reint s_7_34 -> u128
        let s_7_36: u128 = (s_7_34.value() as u128);
        // D s_7_37: size-of s_7_34
        let s_7_37: u16 = s_7_34.length();
        // D s_7_38: cast reint s_7_35 -> u128
        let s_7_38: u128 = (s_7_35.value() as u128);
        // D s_7_39: size-of s_7_35
        let s_7_39: u16 = s_7_35.length();
        // D s_7_40: lsl s_7_36 s_7_39
        let s_7_40: u128 = s_7_36 << s_7_39;
        // D s_7_41: or s_7_40 s_7_38
        let s_7_41: u128 = ((s_7_40) | (s_7_38));
        // D s_7_42: add s_7_37 s_7_39
        let s_7_42: u16 = (s_7_37 + s_7_39);
        // D s_7_43: create-bits s_7_41 s_7_42
        let s_7_43: Bits = Bits::new(s_7_41, s_7_42);
        // D s_7_44: cast reint s_7_43 -> u8
        let s_7_44: u8 = (s_7_43.value() as u8);
        // S s_7_45: cast zx s_7_3 -> bv
        let s_7_45: Bits = Bits::new(s_7_3 as u128, 54u16);
        // D s_7_46: cast zx s_7_44 -> bv
        let s_7_46: Bits = Bits::new(s_7_44 as u128, 4u16);
        // S s_7_47: cast reint s_7_45 -> u128
        let s_7_47: u128 = (s_7_45.value() as u128);
        // D s_7_48: size-of s_7_45
        let s_7_48: u16 = s_7_45.length();
        // D s_7_49: cast reint s_7_46 -> u128
        let s_7_49: u128 = (s_7_46.value() as u128);
        // D s_7_50: size-of s_7_46
        let s_7_50: u16 = s_7_46.length();
        // D s_7_51: lsl s_7_47 s_7_50
        let s_7_51: u128 = s_7_47 << s_7_50;
        // D s_7_52: or s_7_51 s_7_49
        let s_7_52: u128 = ((s_7_51) | (s_7_49));
        // D s_7_53: add s_7_48 s_7_50
        let s_7_53: u16 = (s_7_48 + s_7_50);
        // D s_7_54: create-bits s_7_52 s_7_53
        let s_7_54: Bits = Bits::new(s_7_52, s_7_53);
        // D s_7_55: cast reint s_7_54 -> u58
        let s_7_55: u64 = (s_7_54.value() as u64);
        // C s_7_56: const #6s : i
        let s_7_56: i128 = 6;
        // S s_7_57: call Zeros(s_7_56)
        let s_7_57: Bits = Zeros(state, tracer, s_7_56);
        // S s_7_58: cast reint s_7_57 -> u8
        let s_7_58: u8 = (s_7_57.value() as u8);
        // D s_7_59: cast zx s_7_55 -> bv
        let s_7_59: Bits = Bits::new(s_7_55 as u128, 58u16);
        // S s_7_60: cast zx s_7_58 -> bv
        let s_7_60: Bits = Bits::new(s_7_58 as u128, 6u16);
        // D s_7_61: cast reint s_7_59 -> u128
        let s_7_61: u128 = (s_7_59.value() as u128);
        // D s_7_62: size-of s_7_59
        let s_7_62: u16 = s_7_59.length();
        // S s_7_63: cast reint s_7_60 -> u128
        let s_7_63: u128 = (s_7_60.value() as u128);
        // D s_7_64: size-of s_7_60
        let s_7_64: u16 = s_7_60.length();
        // D s_7_65: lsl s_7_61 s_7_64
        let s_7_65: u128 = s_7_61 << s_7_64;
        // D s_7_66: or s_7_65 s_7_63
        let s_7_66: u128 = ((s_7_65) | (s_7_63));
        // D s_7_67: add s_7_62 s_7_64
        let s_7_67: u16 = (s_7_62 + s_7_64);
        // D s_7_68: create-bits s_7_66 s_7_67
        let s_7_68: Bits = Bits::new(s_7_66, s_7_67);
        // D s_7_69: cast reint s_7_68 -> u64
        let s_7_69: u64 = (s_7_68.value() as u64);
        // D s_7_70: cast zx s_7_69 -> bv
        let s_7_70: Bits = Bits::new(s_7_69 as u128, 64u16);
        // D s_7_71: read-var t:i
        let s_7_71: i128 = fn_state.t;
        // D s_7_72: call X_set(s_7_71, s_7_0, s_7_70)
        let s_7_72: () = X_set(state, tracer, s_7_71, s_7_0, s_7_70);
        // N s_7_73: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call EL2Enabled(s_8_0)
        let s_8_1: bool = EL2Enabled(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b21 b9
        if s_8_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#58292 <= s_9_0
        fn_state.gs_58292 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#58292:u8
        let s_10_0: bool = fn_state.gs_58292;
        // N s_10_1: branch s_10_0 b20 b11
        if s_10_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var __SCTLR_EL1_UMA:u8
        let s_11_0: bool = fn_state.u__SCTLR_EL1_UMA;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #0u : u8
        let s_11_2: bool = false;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: write-var gs#58293 <= s_11_4
        fn_state.gs_58293 = s_11_4;
        // N s_11_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#58293:u8
        let s_12_0: bool = fn_state.gs_58293;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #64s : i64
        let s_13_0: i64 = 64;
        // C s_13_1: const #54s : i
        let s_13_1: i128 = 54;
        // S s_13_2: call Zeros(s_13_1)
        let s_13_2: Bits = Zeros(state, tracer, s_13_1);
        // S s_13_3: cast reint s_13_2 -> u54
        let s_13_3: u64 = (s_13_2.value() as u64);
        // C s_13_4: const #16972u : u32
        let s_13_4: u32 = 16972;
        // D s_13_5: read-reg s_13_4:u8
        let s_13_5: bool = {
            let value = state.read_register::<bool>(s_13_4 as isize);
            tracer.read_register(s_13_4 as isize, value);
            value
        };
        // C s_13_6: const #16968u : u32
        let s_13_6: u32 = 16968;
        // D s_13_7: read-reg s_13_6:u8
        let s_13_7: bool = {
            let value = state.read_register::<bool>(s_13_6 as isize);
            tracer.read_register(s_13_6 as isize, value);
            value
        };
        // C s_13_8: const #16979u : u32
        let s_13_8: u32 = 16979;
        // D s_13_9: read-reg s_13_8:u8
        let s_13_9: bool = {
            let value = state.read_register::<bool>(s_13_8 as isize);
            tracer.read_register(s_13_8 as isize, value);
            value
        };
        // C s_13_10: const #16977u : u32
        let s_13_10: u32 = 16977;
        // D s_13_11: read-reg s_13_10:u8
        let s_13_11: bool = {
            let value = state.read_register::<bool>(s_13_10 as isize);
            tracer.read_register(s_13_10 as isize, value);
            value
        };
        // D s_13_12: cast zx s_13_9 -> bv
        let s_13_12: Bits = Bits::new(s_13_9 as u128, 1u16);
        // D s_13_13: cast zx s_13_11 -> bv
        let s_13_13: Bits = Bits::new(s_13_11 as u128, 1u16);
        // D s_13_14: cast reint s_13_12 -> u128
        let s_13_14: u128 = (s_13_12.value() as u128);
        // D s_13_15: size-of s_13_12
        let s_13_15: u16 = s_13_12.length();
        // D s_13_16: cast reint s_13_13 -> u128
        let s_13_16: u128 = (s_13_13.value() as u128);
        // D s_13_17: size-of s_13_13
        let s_13_17: u16 = s_13_13.length();
        // D s_13_18: lsl s_13_14 s_13_17
        let s_13_18: u128 = s_13_14 << s_13_17;
        // D s_13_19: or s_13_18 s_13_16
        let s_13_19: u128 = ((s_13_18) | (s_13_16));
        // D s_13_20: add s_13_15 s_13_17
        let s_13_20: u16 = (s_13_15 + s_13_17);
        // D s_13_21: create-bits s_13_19 s_13_20
        let s_13_21: Bits = Bits::new(s_13_19, s_13_20);
        // D s_13_22: cast reint s_13_21 -> u8
        let s_13_22: u8 = (s_13_21.value() as u8);
        // D s_13_23: cast zx s_13_7 -> bv
        let s_13_23: Bits = Bits::new(s_13_7 as u128, 1u16);
        // D s_13_24: cast zx s_13_22 -> bv
        let s_13_24: Bits = Bits::new(s_13_22 as u128, 2u16);
        // D s_13_25: cast reint s_13_23 -> u128
        let s_13_25: u128 = (s_13_23.value() as u128);
        // D s_13_26: size-of s_13_23
        let s_13_26: u16 = s_13_23.length();
        // D s_13_27: cast reint s_13_24 -> u128
        let s_13_27: u128 = (s_13_24.value() as u128);
        // D s_13_28: size-of s_13_24
        let s_13_28: u16 = s_13_24.length();
        // D s_13_29: lsl s_13_25 s_13_28
        let s_13_29: u128 = s_13_25 << s_13_28;
        // D s_13_30: or s_13_29 s_13_27
        let s_13_30: u128 = ((s_13_29) | (s_13_27));
        // D s_13_31: add s_13_26 s_13_28
        let s_13_31: u16 = (s_13_26 + s_13_28);
        // D s_13_32: create-bits s_13_30 s_13_31
        let s_13_32: Bits = Bits::new(s_13_30, s_13_31);
        // D s_13_33: cast reint s_13_32 -> u8
        let s_13_33: u8 = (s_13_32.value() as u8);
        // D s_13_34: cast zx s_13_5 -> bv
        let s_13_34: Bits = Bits::new(s_13_5 as u128, 1u16);
        // D s_13_35: cast zx s_13_33 -> bv
        let s_13_35: Bits = Bits::new(s_13_33 as u128, 3u16);
        // D s_13_36: cast reint s_13_34 -> u128
        let s_13_36: u128 = (s_13_34.value() as u128);
        // D s_13_37: size-of s_13_34
        let s_13_37: u16 = s_13_34.length();
        // D s_13_38: cast reint s_13_35 -> u128
        let s_13_38: u128 = (s_13_35.value() as u128);
        // D s_13_39: size-of s_13_35
        let s_13_39: u16 = s_13_35.length();
        // D s_13_40: lsl s_13_36 s_13_39
        let s_13_40: u128 = s_13_36 << s_13_39;
        // D s_13_41: or s_13_40 s_13_38
        let s_13_41: u128 = ((s_13_40) | (s_13_38));
        // D s_13_42: add s_13_37 s_13_39
        let s_13_42: u16 = (s_13_37 + s_13_39);
        // D s_13_43: create-bits s_13_41 s_13_42
        let s_13_43: Bits = Bits::new(s_13_41, s_13_42);
        // D s_13_44: cast reint s_13_43 -> u8
        let s_13_44: u8 = (s_13_43.value() as u8);
        // S s_13_45: cast zx s_13_3 -> bv
        let s_13_45: Bits = Bits::new(s_13_3 as u128, 54u16);
        // D s_13_46: cast zx s_13_44 -> bv
        let s_13_46: Bits = Bits::new(s_13_44 as u128, 4u16);
        // S s_13_47: cast reint s_13_45 -> u128
        let s_13_47: u128 = (s_13_45.value() as u128);
        // D s_13_48: size-of s_13_45
        let s_13_48: u16 = s_13_45.length();
        // D s_13_49: cast reint s_13_46 -> u128
        let s_13_49: u128 = (s_13_46.value() as u128);
        // D s_13_50: size-of s_13_46
        let s_13_50: u16 = s_13_46.length();
        // D s_13_51: lsl s_13_47 s_13_50
        let s_13_51: u128 = s_13_47 << s_13_50;
        // D s_13_52: or s_13_51 s_13_49
        let s_13_52: u128 = ((s_13_51) | (s_13_49));
        // D s_13_53: add s_13_48 s_13_50
        let s_13_53: u16 = (s_13_48 + s_13_50);
        // D s_13_54: create-bits s_13_52 s_13_53
        let s_13_54: Bits = Bits::new(s_13_52, s_13_53);
        // D s_13_55: cast reint s_13_54 -> u58
        let s_13_55: u64 = (s_13_54.value() as u64);
        // C s_13_56: const #6s : i
        let s_13_56: i128 = 6;
        // S s_13_57: call Zeros(s_13_56)
        let s_13_57: Bits = Zeros(state, tracer, s_13_56);
        // S s_13_58: cast reint s_13_57 -> u8
        let s_13_58: u8 = (s_13_57.value() as u8);
        // D s_13_59: cast zx s_13_55 -> bv
        let s_13_59: Bits = Bits::new(s_13_55 as u128, 58u16);
        // S s_13_60: cast zx s_13_58 -> bv
        let s_13_60: Bits = Bits::new(s_13_58 as u128, 6u16);
        // D s_13_61: cast reint s_13_59 -> u128
        let s_13_61: u128 = (s_13_59.value() as u128);
        // D s_13_62: size-of s_13_59
        let s_13_62: u16 = s_13_59.length();
        // S s_13_63: cast reint s_13_60 -> u128
        let s_13_63: u128 = (s_13_60.value() as u128);
        // D s_13_64: size-of s_13_60
        let s_13_64: u16 = s_13_60.length();
        // D s_13_65: lsl s_13_61 s_13_64
        let s_13_65: u128 = s_13_61 << s_13_64;
        // D s_13_66: or s_13_65 s_13_63
        let s_13_66: u128 = ((s_13_65) | (s_13_63));
        // D s_13_67: add s_13_62 s_13_64
        let s_13_67: u16 = (s_13_62 + s_13_64);
        // D s_13_68: create-bits s_13_66 s_13_67
        let s_13_68: Bits = Bits::new(s_13_66, s_13_67);
        // D s_13_69: cast reint s_13_68 -> u64
        let s_13_69: u64 = (s_13_68.value() as u64);
        // D s_13_70: cast zx s_13_69 -> bv
        let s_13_70: Bits = Bits::new(s_13_69 as u128, 64u16);
        // D s_13_71: read-var t:i
        let s_13_71: i128 = fn_state.t;
        // D s_13_72: call X_set(s_13_71, s_13_0, s_13_70)
        let s_13_72: () = X_set(state, tracer, s_13_71, s_13_0, s_13_70);
        // N s_13_73: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EL2Enabled(s_14_0)
        let s_14_1: bool = EL2Enabled(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b19 b15
        if s_14_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#58297 <= s_15_0
        fn_state.gs_58297 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#58297:u8
        let s_16_0: bool = fn_state.gs_58297;
        // N s_16_1: branch s_16_0 b18 b17
        if s_16_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #24u : u8
        let s_17_0: u8 = 24;
        // C s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 8u16);
        // C s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (s_17_1.value() as i128);
        // C s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #440u : u32
        let s_17_5: u32 = 440;
        // D s_17_6: read-reg s_17_5:u8
        let s_17_6: u8 = {
            let value = state.read_register::<u8>(s_17_5 as isize);
            tracer.read_register(s_17_5 as isize, value);
            value
        };
        // D s_17_7: call AArch64_SystemAccessTrap(s_17_6, s_17_4)
        let s_17_7: () = AArch64_SystemAccessTrap(state, tracer, s_17_6, s_17_4);
        // N s_17_8: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #24u : u8
        let s_18_0: u8 = 24;
        // C s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 8u16);
        // C s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (s_18_1.value() as i128);
        // C s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #432u : u32
        let s_18_5: u32 = 432;
        // D s_18_6: read-reg s_18_5:u8
        let s_18_6: u8 = {
            let value = state.read_register::<u8>(s_18_5 as isize);
            tracer.read_register(s_18_5 as isize, value);
            value
        };
        // D s_18_7: call AArch64_SystemAccessTrap(s_18_6, s_18_4)
        let s_18_7: () = AArch64_SystemAccessTrap(state, tracer, s_18_6, s_18_4);
        // N s_18_8: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var __HCR_EL2_TGE:u8
        let s_19_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #1u : u8
        let s_19_2: bool = true;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#58297 <= s_19_4
        fn_state.gs_58297 = s_19_4;
        // N s_19_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#58293 <= s_20_0
        fn_state.gs_58293 = s_20_0;
        // N s_20_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #102552u : u32
        let s_21_0: u32 = 102552;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_HCR_EL2_Type_E2H(s_21_1)
        let s_21_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_21_1);
        // C s_21_3: const #102552u : u32
        let s_21_3: u32 = 102552;
        // D s_21_4: read-reg s_21_3:struct
        let s_21_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_3 as isize);
            tracer.read_register(s_21_3 as isize, value);
            value
        };
        // D s_21_5: call _get_HCR_EL2_Type_TGE(s_21_4)
        let s_21_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_21_4);
        // D s_21_6: cast zx s_21_2 -> bv
        let s_21_6: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_7: cast zx s_21_5 -> bv
        let s_21_7: Bits = Bits::new(s_21_5 as u128, 1u16);
        // D s_21_8: cast reint s_21_6 -> u128
        let s_21_8: u128 = (s_21_6.value() as u128);
        // D s_21_9: size-of s_21_6
        let s_21_9: u16 = s_21_6.length();
        // D s_21_10: cast reint s_21_7 -> u128
        let s_21_10: u128 = (s_21_7.value() as u128);
        // D s_21_11: size-of s_21_7
        let s_21_11: u16 = s_21_7.length();
        // D s_21_12: lsl s_21_8 s_21_11
        let s_21_12: u128 = s_21_8 << s_21_11;
        // D s_21_13: or s_21_12 s_21_10
        let s_21_13: u128 = ((s_21_12) | (s_21_10));
        // D s_21_14: add s_21_9 s_21_11
        let s_21_14: u16 = (s_21_9 + s_21_11);
        // D s_21_15: create-bits s_21_13 s_21_14
        let s_21_15: Bits = Bits::new(s_21_13, s_21_14);
        // D s_21_16: cast reint s_21_15 -> u8
        let s_21_16: u8 = (s_21_15.value() as u8);
        // D s_21_17: cast zx s_21_16 -> bv
        let s_21_17: Bits = Bits::new(s_21_16 as u128, 2u16);
        // C s_21_18: const #3u : u8
        let s_21_18: u8 = 3;
        // C s_21_19: cast zx s_21_18 -> bv
        let s_21_19: Bits = Bits::new(s_21_18 as u128, 2u16);
        // D s_21_20: cmp-eq s_21_17 s_21_19
        let s_21_20: bool = ((s_21_17) == (s_21_19));
        // D s_21_21: write-var gs#58292 <= s_21_20
        fn_state.gs_58292 = s_21_20;
        // N s_21_22: jump b10
        return block_10(state, tracer, fn_state);
    }
}
