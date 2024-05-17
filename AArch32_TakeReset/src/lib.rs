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
use u_update_FPEXC_Type_EN::*;
use u_get_HSCTLR_Type_EE::*;
use AArch32_ResetSIMDFPRegisters::*;
use AArch32_ResetControlRegisters::*;
use FPEXC_read::*;
use u_get_SCTLR_Type_TE::*;
use ResetExternalDebugRegisters::*;
use AArch32_ResetSpecialRegisters::*;
use u_get_HSCTLR_Type_TE::*;
use SCTLR_read__2::*;
use AArch32_ResetGeneralRegisters::*;
use AArch32_WriteMode::*;
use HaveAArch64::*;
use u_get_SCTLR_Type_EE::*;
use u__IMPDEF_bits::*;
use FPEXC_write::*;
use BranchTo::*;
use HSCTLR_read::*;
use common::*;
pub fn AArch32_TakeReset<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cold_reset: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31749: bool,
        rv: u32,
        gs_31772: bool,
        gs_31773: bool,
        cold_reset: bool,
    }
    let fn_state = FunctionState {
        cold_reset,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveAArch64(s_0_0)
        let s_0_1: bool = HaveAArch64(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // C s_0_4: const #424u : u32
        let s_0_4: u32 = 424;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // C s_0_6: const #2u : u8
        let s_0_6: u8 = 2;
        // D s_0_7: cmp-lt s_0_5 s_0_6
        let s_0_7: bool = ((s_0_5) < (s_0_6));
        // N s_0_8: branch s_0_7 b22 b1
        if s_0_7 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #432u : u32
        let s_1_0: u32 = 432;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #2u : u8
        let s_1_2: u8 = 2;
        // D s_1_3: cmp-lt s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) < (s_1_2));
        // N s_1_4: branch s_1_3 b21 b2
        if s_1_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #376u : u32
        let s_2_0: u32 = 376;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call AArch32_WriteMode(s_2_1)
        let s_2_2: () = AArch32_WriteMode(state, tracer, s_2_1);
        // N s_2_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var cold_reset:u8
        let s_3_0: bool = fn_state.cold_reset;
        // D s_3_1: call AArch32_ResetControlRegisters(s_3_0)
        let s_3_1: () = AArch32_ResetControlRegisters(state, tracer, s_3_0);
        // C s_3_2: const #() : ()
        let s_3_2: () = ();
        // S s_3_3: call FPEXC_read(s_3_2)
        let s_3_3: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_3_2);
        // C s_3_4: const #0u : u8
        let s_3_4: bool = false;
        // S s_3_5: call _update_FPEXC_Type_EN(s_3_3, s_3_4)
        let s_3_5: ProductType700c18a878c5601b = u_update_FPEXC_Type_EN(
            state,
            tracer,
            s_3_3,
            s_3_4,
        );
        // S s_3_6: call FPEXC_write(s_3_5)
        let s_3_6: () = FPEXC_write(state, tracer, s_3_5);
        // C s_3_7: const #7u : u8
        let s_3_7: u8 = 7;
        // C s_3_8: const #2s : i
        let s_3_8: i128 = 2;
        // C s_3_9: cast zx s_3_7 -> bv
        let s_3_9: Bits = Bits::new(s_3_7 as u128, 3u16);
        // C s_3_10: const #1s : i64
        let s_3_10: i64 = 1;
        // C s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // C s_3_12: const #0s : i
        let s_3_12: i128 = 0;
        // C s_3_13: add s_3_12 s_3_11
        let s_3_13: i128 = (s_3_12 + s_3_11);
        // D s_3_14: bit-extract s_3_9 s_3_8 s_3_13
        let s_3_14: Bits = (Bits::new(
            ((s_3_9) >> (s_3_8)).value(),
            u16::try_from(s_3_13).unwrap(),
        ));
        // D s_3_15: cast reint s_3_14 -> u8
        let s_3_15: bool = ((s_3_14.value()) != 0);
        // C s_3_16: const #16968u : u32
        let s_3_16: u32 = 16968;
        // N s_3_17: write-reg s_3_16 <= s_3_15
        let s_3_17: () = {
            state.write_register::<bool>(s_3_16 as isize, s_3_15);
            tracer.write_register(s_3_16 as isize, s_3_15);
        };
        // C s_3_18: const #1s : i
        let s_3_18: i128 = 1;
        // C s_3_19: cast zx s_3_7 -> bv
        let s_3_19: Bits = Bits::new(s_3_7 as u128, 3u16);
        // C s_3_20: const #1s : i64
        let s_3_20: i64 = 1;
        // C s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // C s_3_22: const #0s : i
        let s_3_22: i128 = 0;
        // C s_3_23: add s_3_22 s_3_21
        let s_3_23: i128 = (s_3_22 + s_3_21);
        // D s_3_24: bit-extract s_3_19 s_3_18 s_3_23
        let s_3_24: Bits = (Bits::new(
            ((s_3_19) >> (s_3_18)).value(),
            u16::try_from(s_3_23).unwrap(),
        ));
        // D s_3_25: cast reint s_3_24 -> u8
        let s_3_25: bool = ((s_3_24.value()) != 0);
        // C s_3_26: const #16979u : u32
        let s_3_26: u32 = 16979;
        // N s_3_27: write-reg s_3_26 <= s_3_25
        let s_3_27: () = {
            state.write_register::<bool>(s_3_26 as isize, s_3_25);
            tracer.write_register(s_3_26 as isize, s_3_25);
        };
        // C s_3_28: const #0s : i
        let s_3_28: i128 = 0;
        // C s_3_29: cast zx s_3_7 -> bv
        let s_3_29: Bits = Bits::new(s_3_7 as u128, 3u16);
        // C s_3_30: const #1s : i64
        let s_3_30: i64 = 1;
        // C s_3_31: cast zx s_3_30 -> i
        let s_3_31: i128 = (i128::try_from(s_3_30).unwrap());
        // C s_3_32: const #0s : i
        let s_3_32: i128 = 0;
        // C s_3_33: add s_3_32 s_3_31
        let s_3_33: i128 = (s_3_32 + s_3_31);
        // D s_3_34: bit-extract s_3_29 s_3_28 s_3_33
        let s_3_34: Bits = (Bits::new(
            ((s_3_29) >> (s_3_28)).value(),
            u16::try_from(s_3_33).unwrap(),
        ));
        // D s_3_35: cast reint s_3_34 -> u8
        let s_3_35: bool = ((s_3_34.value()) != 0);
        // C s_3_36: const #16977u : u32
        let s_3_36: u32 = 16977;
        // N s_3_37: write-reg s_3_36 <= s_3_35
        let s_3_37: () = {
            state.write_register::<bool>(s_3_36 as isize, s_3_35);
            tracer.write_register(s_3_36 as isize, s_3_35);
        };
        // C s_3_38: const #0u : u8
        let s_3_38: u8 = 0;
        // C s_3_39: const #16981u : u32
        let s_3_39: u32 = 16981;
        // N s_3_40: write-reg s_3_39 <= s_3_38
        let s_3_40: () = {
            state.write_register::<u8>(s_3_39 as isize, s_3_38);
            tracer.write_register(s_3_39 as isize, s_3_38);
        };
        // C s_3_41: const #432u : u32
        let s_3_41: u32 = 432;
        // D s_3_42: read-reg s_3_41:u8
        let s_3_42: u8 = {
            let value = state.read_register::<u8>(s_3_41 as isize);
            tracer.read_register(s_3_41 as isize, value);
            value
        };
        // C s_3_43: const #2u : u8
        let s_3_43: u8 = 2;
        // D s_3_44: cmp-lt s_3_42 s_3_43
        let s_3_44: bool = ((s_3_42) < (s_3_43));
        // N s_3_45: branch s_3_44 b20 b4
        if s_3_44 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#31749 <= s_4_0
        fn_state.gs_31749 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#31749:u8
        let s_5_0: bool = fn_state.gs_31749;
        // N s_5_1: branch s_5_0 b19 b6
        if s_5_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call SCTLR_read__2(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_6_0);
        // S s_6_2: call _get_SCTLR_Type_TE(s_6_1)
        let s_6_2: bool = u_get_SCTLR_Type_TE(state, tracer, s_6_1);
        // C s_6_3: const #16993u : u32
        let s_6_3: u32 = 16993;
        // N s_6_4: write-reg s_6_3 <= s_6_2
        let s_6_4: () = {
            state.write_register::<bool>(s_6_3 as isize, s_6_2);
            tracer.write_register(s_6_3 as isize, s_6_2);
        };
        // C s_6_5: const #() : ()
        let s_6_5: () = ();
        // S s_6_6: call SCTLR_read__2(s_6_5)
        let s_6_6: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_6_5);
        // S s_6_7: call _get_SCTLR_Type_EE(s_6_6)
        let s_6_7: bool = u_get_SCTLR_Type_EE(state, tracer, s_6_6);
        // C s_6_8: const #16974u : u32
        let s_6_8: u32 = 16974;
        // N s_6_9: write-reg s_6_8 <= s_6_7
        let s_6_9: () = {
            state.write_register::<bool>(s_6_8 as isize, s_6_7);
            tracer.write_register(s_6_8 as isize, s_6_7);
        };
        // N s_6_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // C s_7_1: const #16980u : u32
        let s_7_1: u32 = 16980;
        // N s_7_2: write-reg s_7_1 <= s_7_0
        let s_7_2: () = {
            state.write_register::<bool>(s_7_1 as isize, s_7_0);
            tracer.write_register(s_7_1 as isize, s_7_0);
        };
        // C s_7_3: const #() : ()
        let s_7_3: () = ();
        // S s_7_4: call AArch32_ResetGeneralRegisters(s_7_3)
        let s_7_4: () = AArch32_ResetGeneralRegisters(state, tracer, s_7_3);
        // C s_7_5: const #() : ()
        let s_7_5: () = ();
        // S s_7_6: call AArch32_ResetSIMDFPRegisters(s_7_5)
        let s_7_6: () = AArch32_ResetSIMDFPRegisters(state, tracer, s_7_5);
        // C s_7_7: const #() : ()
        let s_7_7: () = ();
        // S s_7_8: call AArch32_ResetSpecialRegisters(s_7_7)
        let s_7_8: () = AArch32_ResetSpecialRegisters(state, tracer, s_7_7);
        // D s_7_9: read-var cold_reset:u8
        let s_7_9: bool = fn_state.cold_reset;
        // D s_7_10: call ResetExternalDebugRegisters(s_7_9)
        let s_7_10: () = ResetExternalDebugRegisters(state, tracer, s_7_9);
        // C s_7_11: const #424u : u32
        let s_7_11: u32 = 424;
        // D s_7_12: read-reg s_7_11:u8
        let s_7_12: u8 = {
            let value = state.read_register::<u8>(s_7_11 as isize);
            tracer.read_register(s_7_11 as isize, value);
            value
        };
        // C s_7_13: const #2u : u8
        let s_7_13: u8 = 2;
        // D s_7_14: cmp-lt s_7_12 s_7_13
        let s_7_14: bool = ((s_7_12) < (s_7_13));
        // N s_7_15: branch s_7_14 b16 b8
        if s_7_14 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1s : i
        let s_8_0: i128 = 1;
        // C s_8_1: const #15032u : u32
        let s_8_1: u32 = 15032;
        // D s_8_2: read-reg s_8_1:u32
        let s_8_2: u32 = {
            let value = state.read_register::<u32>(s_8_1 as isize);
            tracer.read_register(s_8_1 as isize, value);
            value
        };
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 32u16);
        // C s_8_4: const #1s : i64
        let s_8_4: i64 = 1;
        // C s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // C s_8_6: const #30s : i
        let s_8_6: i128 = 30;
        // C s_8_7: add s_8_6 s_8_5
        let s_8_7: i128 = (s_8_6 + s_8_5);
        // D s_8_8: bit-extract s_8_3 s_8_0 s_8_7
        let s_8_8: Bits = (Bits::new(
            ((s_8_3) >> (s_8_0)).value(),
            u16::try_from(s_8_7).unwrap(),
        ));
        // D s_8_9: cast reint s_8_8 -> u31
        let s_8_9: u32 = (s_8_8.value() as u32);
        // D s_8_10: cast zx s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 31u16);
        // C s_8_11: const #0u : u8
        let s_8_11: bool = false;
        // C s_8_12: cast zx s_8_11 -> bv
        let s_8_12: Bits = Bits::new(s_8_11 as u128, 1u16);
        // D s_8_13: cast reint s_8_10 -> u128
        let s_8_13: u128 = (s_8_10.value() as u128);
        // D s_8_14: size-of s_8_10
        let s_8_14: u16 = s_8_10.length();
        // C s_8_15: cast reint s_8_12 -> u128
        let s_8_15: u128 = (s_8_12.value() as u128);
        // D s_8_16: size-of s_8_12
        let s_8_16: u16 = s_8_12.length();
        // D s_8_17: lsl s_8_13 s_8_16
        let s_8_17: u128 = s_8_13 << s_8_16;
        // D s_8_18: or s_8_17 s_8_15
        let s_8_18: u128 = ((s_8_17) | (s_8_15));
        // D s_8_19: add s_8_14 s_8_16
        let s_8_19: u16 = (s_8_14 + s_8_16);
        // D s_8_20: create-bits s_8_18 s_8_19
        let s_8_20: Bits = Bits::new(s_8_18, s_8_19);
        // D s_8_21: cast reint s_8_20 -> u32
        let s_8_21: u32 = (s_8_20.value() as u32);
        // D s_8_22: write-var rv <= s_8_21
        fn_state.rv = s_8_21;
        // N s_8_23: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: read-var rv:u32
        let s_9_1: u32 = fn_state.rv;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 32u16);
        // C s_9_3: const #1u : u64
        let s_9_3: u64 = 1;
        // D s_9_4: bit-extract s_9_2 s_9_0 s_9_3
        let s_9_4: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_3).unwrap(),
        ));
        // D s_9_5: cast reint s_9_4 -> u8
        let s_9_5: bool = ((s_9_4.value()) != 0);
        // C s_9_6: const #0s : i
        let s_9_6: i128 = 0;
        // C s_9_7: const #0u : u64
        let s_9_7: u64 = 0;
        // D s_9_8: cast zx s_9_5 -> u64
        let s_9_8: u64 = (s_9_5 as u64);
        // C s_9_9: const #1u : u64
        let s_9_9: u64 = 1;
        // D s_9_10: and s_9_8 s_9_9
        let s_9_10: u64 = ((s_9_8) & (s_9_9));
        // D s_9_11: cmp-eq s_9_10 s_9_9
        let s_9_11: bool = ((s_9_10) == (s_9_9));
        // D s_9_12: lsl s_9_8 s_9_6
        let s_9_12: u64 = s_9_8 << s_9_6;
        // D s_9_13: or s_9_7 s_9_12
        let s_9_13: u64 = ((s_9_7) | (s_9_12));
        // D s_9_14: cmpl s_9_12
        let s_9_14: u64 = !s_9_12;
        // D s_9_15: and s_9_7 s_9_14
        let s_9_15: u64 = ((s_9_7) & (s_9_14));
        // D s_9_16: select s_9_11 s_9_13 s_9_15
        let s_9_16: u64 = if s_9_11 { s_9_13 } else { s_9_15 };
        // D s_9_17: cast trunc s_9_16 -> u8
        let s_9_17: bool = ((s_9_16) != 0);
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 1u16);
        // C s_9_19: const #0u : u8
        let s_9_19: bool = false;
        // C s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 1u16);
        // D s_9_21: cmp-eq s_9_18 s_9_20
        let s_9_21: bool = ((s_9_18) == (s_9_20));
        // N s_9_22: branch s_9_21 b12 b10
        if s_9_21 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#31773 <= s_10_0
        fn_state.gs_31773 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#31773:u8
        let s_11_0: bool = fn_state.gs_31773;
        // N s_11_1: assert s_11_0
        let s_11_1: () = assert!(s_11_0);
        // C s_11_2: const #0u : u8
        let s_11_2: bool = false;
        // D s_11_3: read-var rv:u32
        let s_11_3: u32 = fn_state.rv;
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 32u16);
        // C s_11_5: const #9u : u32
        let s_11_5: u32 = 9;
        // D s_11_6: call BranchTo(s_11_4, s_11_5, s_11_2)
        let s_11_6: () = BranchTo(state, tracer, s_11_4, s_11_5, s_11_2);
        // N s_11_7: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #16993u : u32
        let s_12_0: u32 = 16993;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: bool = {
            let value = state.read_register::<bool>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 1u16);
        // C s_12_3: const #1u : u8
        let s_12_3: bool = true;
        // C s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 1u16);
        // D s_12_5: cmp-eq s_12_2 s_12_4
        let s_12_5: bool = ((s_12_2) == (s_12_4));
        // N s_12_6: branch s_12_5 b15 b13
        if s_12_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1s : i
        let s_13_0: i128 = 1;
        // D s_13_1: read-var rv:u32
        let s_13_1: u32 = fn_state.rv;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 32u16);
        // C s_13_3: const #1u : u64
        let s_13_3: u64 = 1;
        // D s_13_4: bit-extract s_13_2 s_13_0 s_13_3
        let s_13_4: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_3).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: bool = ((s_13_4.value()) != 0);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // C s_13_7: const #0u : u64
        let s_13_7: u64 = 0;
        // D s_13_8: cast zx s_13_5 -> u64
        let s_13_8: u64 = (s_13_5 as u64);
        // C s_13_9: const #1u : u64
        let s_13_9: u64 = 1;
        // D s_13_10: and s_13_8 s_13_9
        let s_13_10: u64 = ((s_13_8) & (s_13_9));
        // D s_13_11: cmp-eq s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) == (s_13_9));
        // D s_13_12: lsl s_13_8 s_13_6
        let s_13_12: u64 = s_13_8 << s_13_6;
        // D s_13_13: or s_13_7 s_13_12
        let s_13_13: u64 = ((s_13_7) | (s_13_12));
        // D s_13_14: cmpl s_13_12
        let s_13_14: u64 = !s_13_12;
        // D s_13_15: and s_13_7 s_13_14
        let s_13_15: u64 = ((s_13_7) & (s_13_14));
        // D s_13_16: select s_13_11 s_13_13 s_13_15
        let s_13_16: u64 = if s_13_11 { s_13_13 } else { s_13_15 };
        // D s_13_17: cast trunc s_13_16 -> u8
        let s_13_17: bool = ((s_13_16) != 0);
        // D s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 1u16);
        // C s_13_19: const #0u : u8
        let s_13_19: bool = false;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // D s_13_21: cmp-eq s_13_18 s_13_20
        let s_13_21: bool = ((s_13_18) == (s_13_20));
        // D s_13_22: write-var gs#31772 <= s_13_21
        fn_state.gs_31772 = s_13_21;
        // N s_13_23: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#31772:u8
        let s_14_0: bool = fn_state.gs_31772;
        // D s_14_1: write-var gs#31773 <= s_14_0
        fn_state.gs_31773 = s_14_0;
        // N s_14_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#31772 <= s_15_0
        fn_state.gs_31772 = s_15_0;
        // N s_15_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #100208u : u32
        let s_16_0: u32 = 100208;
        // D s_16_1: read-reg s_16_0:u32
        let s_16_1: u32 = {
            let value = state.read_register::<u32>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // C s_16_2: const #0s : i
        let s_16_2: i128 = 0;
        // D s_16_3: cast zx s_16_1 -> bv
        let s_16_3: Bits = Bits::new(s_16_1 as u128, 32u16);
        // C s_16_4: const #1u : u64
        let s_16_4: u64 = 1;
        // D s_16_5: bit-extract s_16_3 s_16_2 s_16_4
        let s_16_5: Bits = (Bits::new(
            ((s_16_3) >> (s_16_2)).value(),
            u16::try_from(s_16_4).unwrap(),
        ));
        // D s_16_6: cast reint s_16_5 -> u8
        let s_16_6: bool = ((s_16_5.value()) != 0);
        // C s_16_7: const #0s : i
        let s_16_7: i128 = 0;
        // C s_16_8: const #0u : u64
        let s_16_8: u64 = 0;
        // D s_16_9: cast zx s_16_6 -> u64
        let s_16_9: u64 = (s_16_6 as u64);
        // C s_16_10: const #1u : u64
        let s_16_10: u64 = 1;
        // D s_16_11: and s_16_9 s_16_10
        let s_16_11: u64 = ((s_16_9) & (s_16_10));
        // D s_16_12: cmp-eq s_16_11 s_16_10
        let s_16_12: bool = ((s_16_11) == (s_16_10));
        // D s_16_13: lsl s_16_9 s_16_7
        let s_16_13: u64 = s_16_9 << s_16_7;
        // D s_16_14: or s_16_8 s_16_13
        let s_16_14: u64 = ((s_16_8) | (s_16_13));
        // D s_16_15: cmpl s_16_13
        let s_16_15: u64 = !s_16_13;
        // D s_16_16: and s_16_8 s_16_15
        let s_16_16: u64 = ((s_16_8) & (s_16_15));
        // D s_16_17: select s_16_12 s_16_14 s_16_16
        let s_16_17: u64 = if s_16_12 { s_16_14 } else { s_16_16 };
        // D s_16_18: cast trunc s_16_17 -> u8
        let s_16_18: bool = ((s_16_17) != 0);
        // D s_16_19: cast zx s_16_18 -> bv
        let s_16_19: Bits = Bits::new(s_16_18 as u128, 1u16);
        // C s_16_20: const #1u : u8
        let s_16_20: bool = true;
        // C s_16_21: cast zx s_16_20 -> bv
        let s_16_21: Bits = Bits::new(s_16_20 as u128, 1u16);
        // D s_16_22: cmp-eq s_16_19 s_16_21
        let s_16_22: bool = ((s_16_19) == (s_16_21));
        // N s_16_23: branch s_16_22 b18 b17
        if s_16_22 {
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
        // C s_17_0: const #32s : i64
        let s_17_0: i64 = 32;
        // C s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // C s_17_2: const #"reset vector address" : str
        let s_17_2: &'static str = "reset vector address";
        // S s_17_3: call __IMPDEF_bits(s_17_1, s_17_2)
        let s_17_3: Bits = u__IMPDEF_bits(state, tracer, s_17_1, s_17_2);
        // S s_17_4: cast reint s_17_3 -> u32
        let s_17_4: u32 = (s_17_3.value() as u32);
        // D s_17_5: write-var rv <= s_17_4
        fn_state.rv = s_17_4;
        // N s_17_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #100208u : u32
        let s_18_0: u32 = 100208;
        // D s_18_1: read-reg s_18_0:u32
        let s_18_1: u32 = {
            let value = state.read_register::<u32>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // C s_18_2: const #1s : i
        let s_18_2: i128 = 1;
        // D s_18_3: cast zx s_18_1 -> bv
        let s_18_3: Bits = Bits::new(s_18_1 as u128, 32u16);
        // C s_18_4: const #1s : i64
        let s_18_4: i64 = 1;
        // C s_18_5: cast zx s_18_4 -> i
        let s_18_5: i128 = (i128::try_from(s_18_4).unwrap());
        // C s_18_6: const #30s : i
        let s_18_6: i128 = 30;
        // C s_18_7: add s_18_6 s_18_5
        let s_18_7: i128 = (s_18_6 + s_18_5);
        // D s_18_8: bit-extract s_18_3 s_18_2 s_18_7
        let s_18_8: Bits = (Bits::new(
            ((s_18_3) >> (s_18_2)).value(),
            u16::try_from(s_18_7).unwrap(),
        ));
        // D s_18_9: cast reint s_18_8 -> u31
        let s_18_9: u32 = (s_18_8.value() as u32);
        // D s_18_10: cast zx s_18_9 -> bv
        let s_18_10: Bits = Bits::new(s_18_9 as u128, 31u16);
        // C s_18_11: const #0u : u8
        let s_18_11: bool = false;
        // C s_18_12: cast zx s_18_11 -> bv
        let s_18_12: Bits = Bits::new(s_18_11 as u128, 1u16);
        // D s_18_13: cast reint s_18_10 -> u128
        let s_18_13: u128 = (s_18_10.value() as u128);
        // D s_18_14: size-of s_18_10
        let s_18_14: u16 = s_18_10.length();
        // C s_18_15: cast reint s_18_12 -> u128
        let s_18_15: u128 = (s_18_12.value() as u128);
        // D s_18_16: size-of s_18_12
        let s_18_16: u16 = s_18_12.length();
        // D s_18_17: lsl s_18_13 s_18_16
        let s_18_17: u128 = s_18_13 << s_18_16;
        // D s_18_18: or s_18_17 s_18_15
        let s_18_18: u128 = ((s_18_17) | (s_18_15));
        // D s_18_19: add s_18_14 s_18_16
        let s_18_19: u16 = (s_18_14 + s_18_16);
        // D s_18_20: create-bits s_18_18 s_18_19
        let s_18_20: Bits = Bits::new(s_18_18, s_18_19);
        // D s_18_21: cast reint s_18_20 -> u32
        let s_18_21: u32 = (s_18_20.value() as u32);
        // D s_18_22: write-var rv <= s_18_21
        fn_state.rv = s_18_21;
        // N s_18_23: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HSCTLR_read(s_19_0)
        let s_19_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_19_0);
        // S s_19_2: call _get_HSCTLR_Type_TE(s_19_1)
        let s_19_2: bool = u_get_HSCTLR_Type_TE(state, tracer, s_19_1);
        // C s_19_3: const #16993u : u32
        let s_19_3: u32 = 16993;
        // N s_19_4: write-reg s_19_3 <= s_19_2
        let s_19_4: () = {
            state.write_register::<bool>(s_19_3 as isize, s_19_2);
            tracer.write_register(s_19_3 as isize, s_19_2);
        };
        // C s_19_5: const #() : ()
        let s_19_5: () = ();
        // S s_19_6: call HSCTLR_read(s_19_5)
        let s_19_6: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_19_5);
        // S s_19_7: call _get_HSCTLR_Type_EE(s_19_6)
        let s_19_7: bool = u_get_HSCTLR_Type_EE(state, tracer, s_19_6);
        // C s_19_8: const #16974u : u32
        let s_19_8: u32 = 16974;
        // N s_19_9: write-reg s_19_8 <= s_19_7
        let s_19_9: () = {
            state.write_register::<bool>(s_19_8 as isize, s_19_7);
            tracer.write_register(s_19_8 as isize, s_19_7);
        };
        // N s_19_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #424u : u32
        let s_20_0: u32 = 424;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // D s_20_3: cmp-lt s_20_1 s_20_2
        let s_20_3: bool = ((s_20_1) < (s_20_2));
        // D s_20_4: not s_20_3
        let s_20_4: bool = !s_20_3;
        // D s_20_5: write-var gs#31749 <= s_20_4
        fn_state.gs_31749 = s_20_4;
        // N s_20_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #400u : u32
        let s_21_0: u32 = 400;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: u8 = {
            let value = state.read_register::<u8>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call AArch32_WriteMode(s_21_1)
        let s_21_2: () = AArch32_WriteMode(state, tracer, s_21_1);
        // N s_21_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #376u : u32
        let s_22_0: u32 = 376;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call AArch32_WriteMode(s_22_1)
        let s_22_2: () = AArch32_WriteMode(state, tracer, s_22_1);
        // C s_22_3: const #20920u : u32
        let s_22_3: u32 = 20920;
        // D s_22_4: read-reg s_22_3:struct
        let s_22_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_22_3 as isize);
            tracer.read_register(s_22_3 as isize, value);
            value
        };
        // C s_22_5: const #20920u : u32
        let s_22_5: u32 = 20920;
        // N s_22_6: write-reg s_22_5 <= s_22_4
        let s_22_6: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_22_5 as isize, s_22_4);
            tracer.write_register(s_22_5 as isize, s_22_4);
        };
        // N s_22_7: jump b3
        return block_3(state, tracer, fn_state);
    }
}
