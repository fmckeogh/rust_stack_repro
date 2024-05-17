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
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_E2H::*;
use common::*;
pub fn DAIF_SysRegWrite_523f21b562245414<T: Tracer>(
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
        gs_82109: bool,
        gs_82093: bool,
        gs_82094: bool,
        u__PSTATE_EL: u8,
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
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // C s_5_4: const #6s : i
        let s_5_4: i128 = 6;
        // D s_5_5: cast zx s_5_3 -> bv
        let s_5_5: Bits = Bits::new(s_5_3 as u128, 64u16);
        // C s_5_6: const #1s : i64
        let s_5_6: i64 = 1;
        // C s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // C s_5_8: const #3s : i
        let s_5_8: i128 = 3;
        // C s_5_9: add s_5_8 s_5_7
        let s_5_9: i128 = (s_5_8 + s_5_7);
        // D s_5_10: bit-extract s_5_5 s_5_4 s_5_9
        let s_5_10: Bits = (Bits::new(
            ((s_5_5) >> (s_5_4)).value(),
            u16::try_from(s_5_9).unwrap(),
        ));
        // D s_5_11: cast reint s_5_10 -> u8
        let s_5_11: u8 = (s_5_10.value() as u8);
        // C s_5_12: const #3s : i
        let s_5_12: i128 = 3;
        // D s_5_13: cast zx s_5_11 -> bv
        let s_5_13: Bits = Bits::new(s_5_11 as u128, 4u16);
        // C s_5_14: const #1s : i64
        let s_5_14: i64 = 1;
        // C s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // C s_5_16: const #0s : i
        let s_5_16: i128 = 0;
        // C s_5_17: add s_5_16 s_5_15
        let s_5_17: i128 = (s_5_16 + s_5_15);
        // D s_5_18: bit-extract s_5_13 s_5_12 s_5_17
        let s_5_18: Bits = (Bits::new(
            ((s_5_13) >> (s_5_12)).value(),
            u16::try_from(s_5_17).unwrap(),
        ));
        // D s_5_19: cast reint s_5_18 -> u8
        let s_5_19: bool = ((s_5_18.value()) != 0);
        // C s_5_20: const #16972u : u32
        let s_5_20: u32 = 16972;
        // N s_5_21: write-reg s_5_20 <= s_5_19
        let s_5_21: () = {
            state.write_register::<bool>(s_5_20 as isize, s_5_19);
            tracer.write_register(s_5_20 as isize, s_5_19);
        };
        // C s_5_22: const #2s : i
        let s_5_22: i128 = 2;
        // D s_5_23: cast zx s_5_11 -> bv
        let s_5_23: Bits = Bits::new(s_5_11 as u128, 4u16);
        // C s_5_24: const #1s : i64
        let s_5_24: i64 = 1;
        // C s_5_25: cast zx s_5_24 -> i
        let s_5_25: i128 = (i128::try_from(s_5_24).unwrap());
        // C s_5_26: const #0s : i
        let s_5_26: i128 = 0;
        // C s_5_27: add s_5_26 s_5_25
        let s_5_27: i128 = (s_5_26 + s_5_25);
        // D s_5_28: bit-extract s_5_23 s_5_22 s_5_27
        let s_5_28: Bits = (Bits::new(
            ((s_5_23) >> (s_5_22)).value(),
            u16::try_from(s_5_27).unwrap(),
        ));
        // D s_5_29: cast reint s_5_28 -> u8
        let s_5_29: bool = ((s_5_28.value()) != 0);
        // C s_5_30: const #16968u : u32
        let s_5_30: u32 = 16968;
        // N s_5_31: write-reg s_5_30 <= s_5_29
        let s_5_31: () = {
            state.write_register::<bool>(s_5_30 as isize, s_5_29);
            tracer.write_register(s_5_30 as isize, s_5_29);
        };
        // C s_5_32: const #1s : i
        let s_5_32: i128 = 1;
        // D s_5_33: cast zx s_5_11 -> bv
        let s_5_33: Bits = Bits::new(s_5_11 as u128, 4u16);
        // C s_5_34: const #1s : i64
        let s_5_34: i64 = 1;
        // C s_5_35: cast zx s_5_34 -> i
        let s_5_35: i128 = (i128::try_from(s_5_34).unwrap());
        // C s_5_36: const #0s : i
        let s_5_36: i128 = 0;
        // C s_5_37: add s_5_36 s_5_35
        let s_5_37: i128 = (s_5_36 + s_5_35);
        // D s_5_38: bit-extract s_5_33 s_5_32 s_5_37
        let s_5_38: Bits = (Bits::new(
            ((s_5_33) >> (s_5_32)).value(),
            u16::try_from(s_5_37).unwrap(),
        ));
        // D s_5_39: cast reint s_5_38 -> u8
        let s_5_39: bool = ((s_5_38.value()) != 0);
        // C s_5_40: const #16979u : u32
        let s_5_40: u32 = 16979;
        // N s_5_41: write-reg s_5_40 <= s_5_39
        let s_5_41: () = {
            state.write_register::<bool>(s_5_40 as isize, s_5_39);
            tracer.write_register(s_5_40 as isize, s_5_39);
        };
        // C s_5_42: const #0s : i
        let s_5_42: i128 = 0;
        // D s_5_43: cast zx s_5_11 -> bv
        let s_5_43: Bits = Bits::new(s_5_11 as u128, 4u16);
        // C s_5_44: const #1s : i64
        let s_5_44: i64 = 1;
        // C s_5_45: cast zx s_5_44 -> i
        let s_5_45: i128 = (i128::try_from(s_5_44).unwrap());
        // C s_5_46: const #0s : i
        let s_5_46: i128 = 0;
        // C s_5_47: add s_5_46 s_5_45
        let s_5_47: i128 = (s_5_46 + s_5_45);
        // D s_5_48: bit-extract s_5_43 s_5_42 s_5_47
        let s_5_48: Bits = (Bits::new(
            ((s_5_43) >> (s_5_42)).value(),
            u16::try_from(s_5_47).unwrap(),
        ));
        // D s_5_49: cast reint s_5_48 -> u8
        let s_5_49: bool = ((s_5_48.value()) != 0);
        // C s_5_50: const #16977u : u32
        let s_5_50: u32 = 16977;
        // N s_5_51: write-reg s_5_50 <= s_5_49
        let s_5_51: () = {
            state.write_register::<bool>(s_5_50 as isize, s_5_49);
            tracer.write_register(s_5_50 as isize, s_5_49);
        };
        // N s_5_52: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: read-var t:i
        let s_6_1: i128 = fn_state.t;
        // D s_6_2: call X_read(s_6_1, s_6_0)
        let s_6_2: Bits = X_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // C s_6_4: const #6s : i
        let s_6_4: i128 = 6;
        // D s_6_5: cast zx s_6_3 -> bv
        let s_6_5: Bits = Bits::new(s_6_3 as u128, 64u16);
        // C s_6_6: const #1s : i64
        let s_6_6: i64 = 1;
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // C s_6_8: const #3s : i
        let s_6_8: i128 = 3;
        // C s_6_9: add s_6_8 s_6_7
        let s_6_9: i128 = (s_6_8 + s_6_7);
        // D s_6_10: bit-extract s_6_5 s_6_4 s_6_9
        let s_6_10: Bits = (Bits::new(
            ((s_6_5) >> (s_6_4)).value(),
            u16::try_from(s_6_9).unwrap(),
        ));
        // D s_6_11: cast reint s_6_10 -> u8
        let s_6_11: u8 = (s_6_10.value() as u8);
        // C s_6_12: const #3s : i
        let s_6_12: i128 = 3;
        // D s_6_13: cast zx s_6_11 -> bv
        let s_6_13: Bits = Bits::new(s_6_11 as u128, 4u16);
        // C s_6_14: const #1s : i64
        let s_6_14: i64 = 1;
        // C s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (i128::try_from(s_6_14).unwrap());
        // C s_6_16: const #0s : i
        let s_6_16: i128 = 0;
        // C s_6_17: add s_6_16 s_6_15
        let s_6_17: i128 = (s_6_16 + s_6_15);
        // D s_6_18: bit-extract s_6_13 s_6_12 s_6_17
        let s_6_18: Bits = (Bits::new(
            ((s_6_13) >> (s_6_12)).value(),
            u16::try_from(s_6_17).unwrap(),
        ));
        // D s_6_19: cast reint s_6_18 -> u8
        let s_6_19: bool = ((s_6_18.value()) != 0);
        // C s_6_20: const #16972u : u32
        let s_6_20: u32 = 16972;
        // N s_6_21: write-reg s_6_20 <= s_6_19
        let s_6_21: () = {
            state.write_register::<bool>(s_6_20 as isize, s_6_19);
            tracer.write_register(s_6_20 as isize, s_6_19);
        };
        // C s_6_22: const #2s : i
        let s_6_22: i128 = 2;
        // D s_6_23: cast zx s_6_11 -> bv
        let s_6_23: Bits = Bits::new(s_6_11 as u128, 4u16);
        // C s_6_24: const #1s : i64
        let s_6_24: i64 = 1;
        // C s_6_25: cast zx s_6_24 -> i
        let s_6_25: i128 = (i128::try_from(s_6_24).unwrap());
        // C s_6_26: const #0s : i
        let s_6_26: i128 = 0;
        // C s_6_27: add s_6_26 s_6_25
        let s_6_27: i128 = (s_6_26 + s_6_25);
        // D s_6_28: bit-extract s_6_23 s_6_22 s_6_27
        let s_6_28: Bits = (Bits::new(
            ((s_6_23) >> (s_6_22)).value(),
            u16::try_from(s_6_27).unwrap(),
        ));
        // D s_6_29: cast reint s_6_28 -> u8
        let s_6_29: bool = ((s_6_28.value()) != 0);
        // C s_6_30: const #16968u : u32
        let s_6_30: u32 = 16968;
        // N s_6_31: write-reg s_6_30 <= s_6_29
        let s_6_31: () = {
            state.write_register::<bool>(s_6_30 as isize, s_6_29);
            tracer.write_register(s_6_30 as isize, s_6_29);
        };
        // C s_6_32: const #1s : i
        let s_6_32: i128 = 1;
        // D s_6_33: cast zx s_6_11 -> bv
        let s_6_33: Bits = Bits::new(s_6_11 as u128, 4u16);
        // C s_6_34: const #1s : i64
        let s_6_34: i64 = 1;
        // C s_6_35: cast zx s_6_34 -> i
        let s_6_35: i128 = (i128::try_from(s_6_34).unwrap());
        // C s_6_36: const #0s : i
        let s_6_36: i128 = 0;
        // C s_6_37: add s_6_36 s_6_35
        let s_6_37: i128 = (s_6_36 + s_6_35);
        // D s_6_38: bit-extract s_6_33 s_6_32 s_6_37
        let s_6_38: Bits = (Bits::new(
            ((s_6_33) >> (s_6_32)).value(),
            u16::try_from(s_6_37).unwrap(),
        ));
        // D s_6_39: cast reint s_6_38 -> u8
        let s_6_39: bool = ((s_6_38.value()) != 0);
        // C s_6_40: const #16979u : u32
        let s_6_40: u32 = 16979;
        // N s_6_41: write-reg s_6_40 <= s_6_39
        let s_6_41: () = {
            state.write_register::<bool>(s_6_40 as isize, s_6_39);
            tracer.write_register(s_6_40 as isize, s_6_39);
        };
        // C s_6_42: const #0s : i
        let s_6_42: i128 = 0;
        // D s_6_43: cast zx s_6_11 -> bv
        let s_6_43: Bits = Bits::new(s_6_11 as u128, 4u16);
        // C s_6_44: const #1s : i64
        let s_6_44: i64 = 1;
        // C s_6_45: cast zx s_6_44 -> i
        let s_6_45: i128 = (i128::try_from(s_6_44).unwrap());
        // C s_6_46: const #0s : i
        let s_6_46: i128 = 0;
        // C s_6_47: add s_6_46 s_6_45
        let s_6_47: i128 = (s_6_46 + s_6_45);
        // D s_6_48: bit-extract s_6_43 s_6_42 s_6_47
        let s_6_48: Bits = (Bits::new(
            ((s_6_43) >> (s_6_42)).value(),
            u16::try_from(s_6_47).unwrap(),
        ));
        // D s_6_49: cast reint s_6_48 -> u8
        let s_6_49: bool = ((s_6_48.value()) != 0);
        // C s_6_50: const #16977u : u32
        let s_6_50: u32 = 16977;
        // N s_6_51: write-reg s_6_50 <= s_6_49
        let s_6_51: () = {
            state.write_register::<bool>(s_6_50 as isize, s_6_49);
            tracer.write_register(s_6_50 as isize, s_6_49);
        };
        // N s_6_52: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: read-var t:i
        let s_7_1: i128 = fn_state.t;
        // D s_7_2: call X_read(s_7_1, s_7_0)
        let s_7_2: Bits = X_read(state, tracer, s_7_1, s_7_0);
        // D s_7_3: cast reint s_7_2 -> u64
        let s_7_3: u64 = (s_7_2.value() as u64);
        // C s_7_4: const #6s : i
        let s_7_4: i128 = 6;
        // D s_7_5: cast zx s_7_3 -> bv
        let s_7_5: Bits = Bits::new(s_7_3 as u128, 64u16);
        // C s_7_6: const #1s : i64
        let s_7_6: i64 = 1;
        // C s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // C s_7_8: const #3s : i
        let s_7_8: i128 = 3;
        // C s_7_9: add s_7_8 s_7_7
        let s_7_9: i128 = (s_7_8 + s_7_7);
        // D s_7_10: bit-extract s_7_5 s_7_4 s_7_9
        let s_7_10: Bits = (Bits::new(
            ((s_7_5) >> (s_7_4)).value(),
            u16::try_from(s_7_9).unwrap(),
        ));
        // D s_7_11: cast reint s_7_10 -> u8
        let s_7_11: u8 = (s_7_10.value() as u8);
        // C s_7_12: const #3s : i
        let s_7_12: i128 = 3;
        // D s_7_13: cast zx s_7_11 -> bv
        let s_7_13: Bits = Bits::new(s_7_11 as u128, 4u16);
        // C s_7_14: const #1s : i64
        let s_7_14: i64 = 1;
        // C s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // C s_7_16: const #0s : i
        let s_7_16: i128 = 0;
        // C s_7_17: add s_7_16 s_7_15
        let s_7_17: i128 = (s_7_16 + s_7_15);
        // D s_7_18: bit-extract s_7_13 s_7_12 s_7_17
        let s_7_18: Bits = (Bits::new(
            ((s_7_13) >> (s_7_12)).value(),
            u16::try_from(s_7_17).unwrap(),
        ));
        // D s_7_19: cast reint s_7_18 -> u8
        let s_7_19: bool = ((s_7_18.value()) != 0);
        // C s_7_20: const #16972u : u32
        let s_7_20: u32 = 16972;
        // N s_7_21: write-reg s_7_20 <= s_7_19
        let s_7_21: () = {
            state.write_register::<bool>(s_7_20 as isize, s_7_19);
            tracer.write_register(s_7_20 as isize, s_7_19);
        };
        // C s_7_22: const #2s : i
        let s_7_22: i128 = 2;
        // D s_7_23: cast zx s_7_11 -> bv
        let s_7_23: Bits = Bits::new(s_7_11 as u128, 4u16);
        // C s_7_24: const #1s : i64
        let s_7_24: i64 = 1;
        // C s_7_25: cast zx s_7_24 -> i
        let s_7_25: i128 = (i128::try_from(s_7_24).unwrap());
        // C s_7_26: const #0s : i
        let s_7_26: i128 = 0;
        // C s_7_27: add s_7_26 s_7_25
        let s_7_27: i128 = (s_7_26 + s_7_25);
        // D s_7_28: bit-extract s_7_23 s_7_22 s_7_27
        let s_7_28: Bits = (Bits::new(
            ((s_7_23) >> (s_7_22)).value(),
            u16::try_from(s_7_27).unwrap(),
        ));
        // D s_7_29: cast reint s_7_28 -> u8
        let s_7_29: bool = ((s_7_28.value()) != 0);
        // C s_7_30: const #16968u : u32
        let s_7_30: u32 = 16968;
        // N s_7_31: write-reg s_7_30 <= s_7_29
        let s_7_31: () = {
            state.write_register::<bool>(s_7_30 as isize, s_7_29);
            tracer.write_register(s_7_30 as isize, s_7_29);
        };
        // C s_7_32: const #1s : i
        let s_7_32: i128 = 1;
        // D s_7_33: cast zx s_7_11 -> bv
        let s_7_33: Bits = Bits::new(s_7_11 as u128, 4u16);
        // C s_7_34: const #1s : i64
        let s_7_34: i64 = 1;
        // C s_7_35: cast zx s_7_34 -> i
        let s_7_35: i128 = (i128::try_from(s_7_34).unwrap());
        // C s_7_36: const #0s : i
        let s_7_36: i128 = 0;
        // C s_7_37: add s_7_36 s_7_35
        let s_7_37: i128 = (s_7_36 + s_7_35);
        // D s_7_38: bit-extract s_7_33 s_7_32 s_7_37
        let s_7_38: Bits = (Bits::new(
            ((s_7_33) >> (s_7_32)).value(),
            u16::try_from(s_7_37).unwrap(),
        ));
        // D s_7_39: cast reint s_7_38 -> u8
        let s_7_39: bool = ((s_7_38.value()) != 0);
        // C s_7_40: const #16979u : u32
        let s_7_40: u32 = 16979;
        // N s_7_41: write-reg s_7_40 <= s_7_39
        let s_7_41: () = {
            state.write_register::<bool>(s_7_40 as isize, s_7_39);
            tracer.write_register(s_7_40 as isize, s_7_39);
        };
        // C s_7_42: const #0s : i
        let s_7_42: i128 = 0;
        // D s_7_43: cast zx s_7_11 -> bv
        let s_7_43: Bits = Bits::new(s_7_11 as u128, 4u16);
        // C s_7_44: const #1s : i64
        let s_7_44: i64 = 1;
        // C s_7_45: cast zx s_7_44 -> i
        let s_7_45: i128 = (i128::try_from(s_7_44).unwrap());
        // C s_7_46: const #0s : i
        let s_7_46: i128 = 0;
        // C s_7_47: add s_7_46 s_7_45
        let s_7_47: i128 = (s_7_46 + s_7_45);
        // D s_7_48: bit-extract s_7_43 s_7_42 s_7_47
        let s_7_48: Bits = (Bits::new(
            ((s_7_43) >> (s_7_42)).value(),
            u16::try_from(s_7_47).unwrap(),
        ));
        // D s_7_49: cast reint s_7_48 -> u8
        let s_7_49: bool = ((s_7_48.value()) != 0);
        // C s_7_50: const #16977u : u32
        let s_7_50: u32 = 16977;
        // N s_7_51: write-reg s_7_50 <= s_7_49
        let s_7_51: () = {
            state.write_register::<bool>(s_7_50 as isize, s_7_49);
            tracer.write_register(s_7_50 as isize, s_7_49);
        };
        // N s_7_52: return
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
        // D s_9_1: write-var gs#82093 <= s_9_0
        fn_state.gs_82093 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#82093:u8
        let s_10_0: bool = fn_state.gs_82093;
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
        // D s_11_5: write-var gs#82094 <= s_11_4
        fn_state.gs_82094 = s_11_4;
        // N s_11_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#82094:u8
        let s_12_0: bool = fn_state.gs_82094;
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
        // D s_13_1: read-var t:i
        let s_13_1: i128 = fn_state.t;
        // D s_13_2: call X_read(s_13_1, s_13_0)
        let s_13_2: Bits = X_read(state, tracer, s_13_1, s_13_0);
        // D s_13_3: cast reint s_13_2 -> u64
        let s_13_3: u64 = (s_13_2.value() as u64);
        // C s_13_4: const #6s : i
        let s_13_4: i128 = 6;
        // D s_13_5: cast zx s_13_3 -> bv
        let s_13_5: Bits = Bits::new(s_13_3 as u128, 64u16);
        // C s_13_6: const #1s : i64
        let s_13_6: i64 = 1;
        // C s_13_7: cast zx s_13_6 -> i
        let s_13_7: i128 = (i128::try_from(s_13_6).unwrap());
        // C s_13_8: const #3s : i
        let s_13_8: i128 = 3;
        // C s_13_9: add s_13_8 s_13_7
        let s_13_9: i128 = (s_13_8 + s_13_7);
        // D s_13_10: bit-extract s_13_5 s_13_4 s_13_9
        let s_13_10: Bits = (Bits::new(
            ((s_13_5) >> (s_13_4)).value(),
            u16::try_from(s_13_9).unwrap(),
        ));
        // D s_13_11: cast reint s_13_10 -> u8
        let s_13_11: u8 = (s_13_10.value() as u8);
        // C s_13_12: const #3s : i
        let s_13_12: i128 = 3;
        // D s_13_13: cast zx s_13_11 -> bv
        let s_13_13: Bits = Bits::new(s_13_11 as u128, 4u16);
        // C s_13_14: const #1s : i64
        let s_13_14: i64 = 1;
        // C s_13_15: cast zx s_13_14 -> i
        let s_13_15: i128 = (i128::try_from(s_13_14).unwrap());
        // C s_13_16: const #0s : i
        let s_13_16: i128 = 0;
        // C s_13_17: add s_13_16 s_13_15
        let s_13_17: i128 = (s_13_16 + s_13_15);
        // D s_13_18: bit-extract s_13_13 s_13_12 s_13_17
        let s_13_18: Bits = (Bits::new(
            ((s_13_13) >> (s_13_12)).value(),
            u16::try_from(s_13_17).unwrap(),
        ));
        // D s_13_19: cast reint s_13_18 -> u8
        let s_13_19: bool = ((s_13_18.value()) != 0);
        // C s_13_20: const #16972u : u32
        let s_13_20: u32 = 16972;
        // N s_13_21: write-reg s_13_20 <= s_13_19
        let s_13_21: () = {
            state.write_register::<bool>(s_13_20 as isize, s_13_19);
            tracer.write_register(s_13_20 as isize, s_13_19);
        };
        // C s_13_22: const #2s : i
        let s_13_22: i128 = 2;
        // D s_13_23: cast zx s_13_11 -> bv
        let s_13_23: Bits = Bits::new(s_13_11 as u128, 4u16);
        // C s_13_24: const #1s : i64
        let s_13_24: i64 = 1;
        // C s_13_25: cast zx s_13_24 -> i
        let s_13_25: i128 = (i128::try_from(s_13_24).unwrap());
        // C s_13_26: const #0s : i
        let s_13_26: i128 = 0;
        // C s_13_27: add s_13_26 s_13_25
        let s_13_27: i128 = (s_13_26 + s_13_25);
        // D s_13_28: bit-extract s_13_23 s_13_22 s_13_27
        let s_13_28: Bits = (Bits::new(
            ((s_13_23) >> (s_13_22)).value(),
            u16::try_from(s_13_27).unwrap(),
        ));
        // D s_13_29: cast reint s_13_28 -> u8
        let s_13_29: bool = ((s_13_28.value()) != 0);
        // C s_13_30: const #16968u : u32
        let s_13_30: u32 = 16968;
        // N s_13_31: write-reg s_13_30 <= s_13_29
        let s_13_31: () = {
            state.write_register::<bool>(s_13_30 as isize, s_13_29);
            tracer.write_register(s_13_30 as isize, s_13_29);
        };
        // C s_13_32: const #1s : i
        let s_13_32: i128 = 1;
        // D s_13_33: cast zx s_13_11 -> bv
        let s_13_33: Bits = Bits::new(s_13_11 as u128, 4u16);
        // C s_13_34: const #1s : i64
        let s_13_34: i64 = 1;
        // C s_13_35: cast zx s_13_34 -> i
        let s_13_35: i128 = (i128::try_from(s_13_34).unwrap());
        // C s_13_36: const #0s : i
        let s_13_36: i128 = 0;
        // C s_13_37: add s_13_36 s_13_35
        let s_13_37: i128 = (s_13_36 + s_13_35);
        // D s_13_38: bit-extract s_13_33 s_13_32 s_13_37
        let s_13_38: Bits = (Bits::new(
            ((s_13_33) >> (s_13_32)).value(),
            u16::try_from(s_13_37).unwrap(),
        ));
        // D s_13_39: cast reint s_13_38 -> u8
        let s_13_39: bool = ((s_13_38.value()) != 0);
        // C s_13_40: const #16979u : u32
        let s_13_40: u32 = 16979;
        // N s_13_41: write-reg s_13_40 <= s_13_39
        let s_13_41: () = {
            state.write_register::<bool>(s_13_40 as isize, s_13_39);
            tracer.write_register(s_13_40 as isize, s_13_39);
        };
        // C s_13_42: const #0s : i
        let s_13_42: i128 = 0;
        // D s_13_43: cast zx s_13_11 -> bv
        let s_13_43: Bits = Bits::new(s_13_11 as u128, 4u16);
        // C s_13_44: const #1s : i64
        let s_13_44: i64 = 1;
        // C s_13_45: cast zx s_13_44 -> i
        let s_13_45: i128 = (i128::try_from(s_13_44).unwrap());
        // C s_13_46: const #0s : i
        let s_13_46: i128 = 0;
        // C s_13_47: add s_13_46 s_13_45
        let s_13_47: i128 = (s_13_46 + s_13_45);
        // D s_13_48: bit-extract s_13_43 s_13_42 s_13_47
        let s_13_48: Bits = (Bits::new(
            ((s_13_43) >> (s_13_42)).value(),
            u16::try_from(s_13_47).unwrap(),
        ));
        // D s_13_49: cast reint s_13_48 -> u8
        let s_13_49: bool = ((s_13_48.value()) != 0);
        // C s_13_50: const #16977u : u32
        let s_13_50: u32 = 16977;
        // N s_13_51: write-reg s_13_50 <= s_13_49
        let s_13_51: () = {
            state.write_register::<bool>(s_13_50 as isize, s_13_49);
            tracer.write_register(s_13_50 as isize, s_13_49);
        };
        // N s_13_52: return
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
        // D s_15_1: write-var gs#82109 <= s_15_0
        fn_state.gs_82109 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#82109:u8
        let s_16_0: bool = fn_state.gs_82109;
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
        // D s_19_5: write-var gs#82109 <= s_19_4
        fn_state.gs_82109 = s_19_4;
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
        // D s_20_1: write-var gs#82094 <= s_20_0
        fn_state.gs_82094 = s_20_0;
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
        // D s_21_21: write-var gs#82093 <= s_21_20
        fn_state.gs_82093 = s_21_20;
        // N s_21_22: jump b10
        return block_10(state, tracer, fn_state);
    }
}
