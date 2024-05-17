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
use AArch32_EnterModeInDebugState::*;
use Halted::*;
use u_get_SCTLR_Type_SPAN::*;
use u_get_SCTLR_Type_TE::*;
use HavePANExt::*;
use GetPSRFromPSTATE::*;
use SPSR_set::*;
use SCTLR_read__2::*;
use R_set::*;
use ELUsingAArch32::*;
use HaveSSBSExt::*;
use AArch32_WriteMode::*;
use ExcVectorBase::*;
use integer_subrange::*;
use SynchronizeContext::*;
use u_get_SCTLR_Type_EE::*;
use CheckExceptionCatch::*;
use BranchTo::*;
use EndOfInstruction::*;
use u_get_SCTLR_Type_DSSBS::*;
use common::*;
pub fn AArch32_EnterMode<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_mode: u8,
    preferred_exception_return: u32,
    lr_offset: i128,
    vect_offset: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        spsr: u32,
        gs_9067: bool,
        gs_9060: bool,
        gs_9052: bool,
        target_mode: u8,
        preferred_exception_return: u32,
        lr_offset: i128,
        vect_offset: i128,
    }
    let fn_state = FunctionState {
        target_mode,
        preferred_exception_return,
        lr_offset,
        vect_offset,
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
        // S s_0_1: call SynchronizeContext(s_0_0)
        let s_0_1: () = SynchronizeContext(state, tracer, s_0_0);
        // C s_0_2: const #440u : u32
        let s_0_2: u32 = 440;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: call ELUsingAArch32(s_0_3)
        let s_0_4: bool = ELUsingAArch32(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b26 b1
        if s_0_4 {
            return block_26(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#9052 <= s_1_0
        fn_state.gs_9052 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9052:u8
        let s_2_0: bool = fn_state.gs_9052;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call Halted(s_2_2)
        let s_2_3: bool = Halted(state, tracer, s_2_2);
        // N s_2_4: branch s_2_3 b25 b3
        if s_2_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #32s : i64
        let s_3_0: i64 = 32;
        // C s_3_1: const #0u : u32
        let s_3_1: u32 = 0;
        // S s_3_2: call GetPSRFromPSTATE(s_3_1, s_3_0)
        let s_3_2: Bits = GetPSRFromPSTATE(state, tracer, s_3_1, s_3_0);
        // S s_3_3: cast reint s_3_2 -> u32
        let s_3_3: u32 = (s_3_2.value() as u32);
        // D s_3_4: write-var spsr <= s_3_3
        fn_state.spsr = s_3_3;
        // C s_3_5: const #16983u : u32
        let s_3_5: u32 = 16983;
        // D s_3_6: read-reg s_3_5:u8
        let s_3_6: u8 = {
            let value = state.read_register::<u8>(s_3_5 as isize);
            tracer.read_register(s_3_5 as isize, value);
            value
        };
        // D s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 5u16);
        // C s_3_8: const #384u : u32
        let s_3_8: u32 = 384;
        // D s_3_9: read-reg s_3_8:u8
        let s_3_9: u8 = {
            let value = state.read_register::<u8>(s_3_8 as isize);
            tracer.read_register(s_3_8 as isize, value);
            value
        };
        // D s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 5u16);
        // D s_3_11: cmp-eq s_3_7 s_3_10
        let s_3_11: bool = ((s_3_7) == (s_3_10));
        // N s_3_12: branch s_3_11 b24 b4
        if s_3_11 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var target_mode:u8
        let s_5_0: u8 = fn_state.target_mode;
        // D s_5_1: call AArch32_WriteMode(s_5_0)
        let s_5_1: () = AArch32_WriteMode(state, tracer, s_5_0);
        // C s_5_2: const #32s : i
        let s_5_2: i128 = 32;
        // D s_5_3: read-var spsr:u32
        let s_5_3: u32 = fn_state.spsr;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 32u16);
        // D s_5_5: call SPSR_set(s_5_2, s_5_4)
        let s_5_5: () = SPSR_set(state, tracer, s_5_2, s_5_4);
        // D s_5_6: read-var preferred_exception_return:u32
        let s_5_6: u32 = fn_state.preferred_exception_return;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 32u16);
        // D s_5_8: read-var lr_offset:i
        let s_5_8: i128 = fn_state.lr_offset;
        // D s_5_9: cast cvt s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 128);
        // D s_5_10: add s_5_7 s_5_9
        let s_5_10: Bits = (s_5_7 + s_5_9);
        // D s_5_11: cast reint s_5_10 -> u32
        let s_5_11: u32 = (s_5_10.value() as u32);
        // C s_5_12: const #14s : i
        let s_5_12: i128 = 14;
        // D s_5_13: call R_set(s_5_12, s_5_11)
        let s_5_13: () = R_set(state, tracer, s_5_12, s_5_11);
        // C s_5_14: const #() : ()
        let s_5_14: () = ();
        // S s_5_15: call SCTLR_read__2(s_5_14)
        let s_5_15: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_5_14);
        // S s_5_16: call _get_SCTLR_Type_TE(s_5_15)
        let s_5_16: bool = u_get_SCTLR_Type_TE(state, tracer, s_5_15);
        // C s_5_17: const #16993u : u32
        let s_5_17: u32 = 16993;
        // N s_5_18: write-reg s_5_17 <= s_5_16
        let s_5_18: () = {
            state.write_register::<bool>(s_5_17 as isize, s_5_16);
            tracer.write_register(s_5_17 as isize, s_5_16);
        };
        // C s_5_19: const #0u : u8
        let s_5_19: bool = false;
        // C s_5_20: const #16991u : u32
        let s_5_20: u32 = 16991;
        // N s_5_21: write-reg s_5_20 <= s_5_19
        let s_5_21: () = {
            state.write_register::<bool>(s_5_20 as isize, s_5_19);
            tracer.write_register(s_5_20 as isize, s_5_19);
        };
        // D s_5_22: read-var target_mode:u8
        let s_5_22: u8 = fn_state.target_mode;
        // D s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 5u16);
        // C s_5_24: const #360u : u32
        let s_5_24: u32 = 360;
        // D s_5_25: read-reg s_5_24:u8
        let s_5_25: u8 = {
            let value = state.read_register::<u8>(s_5_24 as isize);
            tracer.read_register(s_5_24 as isize, value);
            value
        };
        // D s_5_26: cast zx s_5_25 -> bv
        let s_5_26: Bits = Bits::new(s_5_25 as u128, 5u16);
        // D s_5_27: cmp-eq s_5_23 s_5_26
        let s_5_27: bool = ((s_5_23) == (s_5_26));
        // N s_5_28: branch s_5_27 b23 b6
        if s_5_27 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var target_mode:u8
        let s_6_0: u8 = fn_state.target_mode;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 5u16);
        // C s_6_2: const #392u : u32
        let s_6_2: u32 = 392;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 5u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b22 b7
        if s_6_5 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var target_mode:u8
        let s_7_0: u8 = fn_state.target_mode;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 5u16);
        // C s_7_2: const #368u : u32
        let s_7_2: u32 = 368;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 5u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // D s_7_6: write-var gs#9067 <= s_7_5
        fn_state.gs_9067 = s_7_5;
        // N s_7_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#9067:u8
        let s_8_0: bool = fn_state.gs_9067;
        // N s_8_1: branch s_8_0 b21 b9
        if s_8_0 {
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
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // C s_9_1: const #16979u : u32
        let s_9_1: u32 = 16979;
        // N s_9_2: write-reg s_9_1 <= s_9_0
        let s_9_2: () = {
            state.write_register::<bool>(s_9_1 as isize, s_9_0);
            tracer.write_register(s_9_1 as isize, s_9_0);
        };
        // N s_9_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call SCTLR_read__2(s_10_0)
        let s_10_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_10_0);
        // S s_10_2: call _get_SCTLR_Type_EE(s_10_1)
        let s_10_2: bool = u_get_SCTLR_Type_EE(state, tracer, s_10_1);
        // C s_10_3: const #16974u : u32
        let s_10_3: u32 = 16974;
        // N s_10_4: write-reg s_10_3 <= s_10_2
        let s_10_4: () = {
            state.write_register::<bool>(s_10_3 as isize, s_10_2);
            tracer.write_register(s_10_3 as isize, s_10_2);
        };
        // C s_10_5: const #0u : u8
        let s_10_5: bool = false;
        // C s_10_6: const #16980u : u32
        let s_10_6: u32 = 16980;
        // N s_10_7: write-reg s_10_6 <= s_10_5
        let s_10_7: () = {
            state.write_register::<bool>(s_10_6 as isize, s_10_5);
            tracer.write_register(s_10_6 as isize, s_10_5);
        };
        // C s_10_8: const #0u : u8
        let s_10_8: u8 = 0;
        // C s_10_9: const #16981u : u32
        let s_10_9: u32 = 16981;
        // N s_10_10: write-reg s_10_9 <= s_10_8
        let s_10_10: () = {
            state.write_register::<u8>(s_10_9 as isize, s_10_8);
            tracer.write_register(s_10_9 as isize, s_10_8);
        };
        // C s_10_11: const #() : ()
        let s_10_11: () = ();
        // S s_10_12: call HavePANExt(s_10_11)
        let s_10_12: bool = HavePANExt(state, tracer, s_10_11);
        // N s_10_13: branch s_10_12 b20 b11
        if s_10_12 {
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
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#9060 <= s_11_0
        fn_state.gs_9060 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#9060:u8
        let s_12_0: bool = fn_state.gs_9060;
        // N s_12_1: branch s_12_0 b19 b13
        if s_12_0 {
            return block_19(state, tracer, fn_state);
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
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveSSBSExt(s_14_0)
        let s_14_1: bool = HaveSSBSExt(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b18 b15
        if s_14_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // C s_16_1: const #() : ()
        let s_16_1: () = ();
        // S s_16_2: call ExcVectorBase(s_16_1)
        let s_16_2: u32 = ExcVectorBase(state, tracer, s_16_1);
        // C s_16_3: const #5s : i
        let s_16_3: i128 = 5;
        // S s_16_4: cast zx s_16_2 -> bv
        let s_16_4: Bits = Bits::new(s_16_2 as u128, 32u16);
        // C s_16_5: const #1s : i64
        let s_16_5: i64 = 1;
        // C s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // C s_16_7: const #26s : i
        let s_16_7: i128 = 26;
        // C s_16_8: add s_16_7 s_16_6
        let s_16_8: i128 = (s_16_7 + s_16_6);
        // D s_16_9: bit-extract s_16_4 s_16_3 s_16_8
        let s_16_9: Bits = (Bits::new(
            ((s_16_4) >> (s_16_3)).value(),
            u16::try_from(s_16_8).unwrap(),
        ));
        // D s_16_10: cast reint s_16_9 -> u27
        let s_16_10: u32 = (s_16_9.value() as u32);
        // C s_16_11: const #4s : i
        let s_16_11: i128 = 4;
        // C s_16_12: const #0s : i
        let s_16_12: i128 = 0;
        // D s_16_13: read-var vect_offset:i
        let s_16_13: i128 = fn_state.vect_offset;
        // D s_16_14: call integer_subrange(s_16_13, s_16_11, s_16_12)
        let s_16_14: Bits = integer_subrange(state, tracer, s_16_13, s_16_11, s_16_12);
        // D s_16_15: cast reint s_16_14 -> u8
        let s_16_15: u8 = (s_16_14.value() as u8);
        // D s_16_16: cast zx s_16_10 -> bv
        let s_16_16: Bits = Bits::new(s_16_10 as u128, 27u16);
        // D s_16_17: cast zx s_16_15 -> bv
        let s_16_17: Bits = Bits::new(s_16_15 as u128, 5u16);
        // D s_16_18: cast reint s_16_16 -> u128
        let s_16_18: u128 = (s_16_16.value() as u128);
        // D s_16_19: size-of s_16_16
        let s_16_19: u16 = s_16_16.length();
        // D s_16_20: cast reint s_16_17 -> u128
        let s_16_20: u128 = (s_16_17.value() as u128);
        // D s_16_21: size-of s_16_17
        let s_16_21: u16 = s_16_17.length();
        // D s_16_22: lsl s_16_18 s_16_21
        let s_16_22: u128 = s_16_18 << s_16_21;
        // D s_16_23: or s_16_22 s_16_20
        let s_16_23: u128 = ((s_16_22) | (s_16_20));
        // D s_16_24: add s_16_19 s_16_21
        let s_16_24: u16 = (s_16_19 + s_16_21);
        // D s_16_25: create-bits s_16_23 s_16_24
        let s_16_25: Bits = Bits::new(s_16_23, s_16_24);
        // D s_16_26: cast reint s_16_25 -> u32
        let s_16_26: u32 = (s_16_25.value() as u32);
        // D s_16_27: cast zx s_16_26 -> bv
        let s_16_27: Bits = Bits::new(s_16_26 as u128, 32u16);
        // C s_16_28: const #7u : u32
        let s_16_28: u32 = 7;
        // D s_16_29: call BranchTo(s_16_27, s_16_28, s_16_0)
        let s_16_29: () = BranchTo(state, tracer, s_16_27, s_16_28, s_16_0);
        // C s_16_30: const #1u : u8
        let s_16_30: bool = true;
        // S s_16_31: call CheckExceptionCatch(s_16_30)
        let s_16_31: () = CheckExceptionCatch(state, tracer, s_16_30);
        // N s_16_32: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call EndOfInstruction(s_17_0)
        let s_17_1: () = EndOfInstruction(state, tracer, s_17_0);
        // N s_17_2: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call SCTLR_read__2(s_18_0)
        let s_18_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_18_0);
        // S s_18_2: call _get_SCTLR_Type_DSSBS(s_18_1)
        let s_18_2: bool = u_get_SCTLR_Type_DSSBS(state, tracer, s_18_1);
        // C s_18_3: const #16992u : u32
        let s_18_3: u32 = 16992;
        // N s_18_4: write-reg s_18_3 <= s_18_2
        let s_18_4: () = {
            state.write_register::<bool>(s_18_3 as isize, s_18_2);
            tracer.write_register(s_18_3 as isize, s_18_2);
        };
        // N s_18_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // C s_19_1: const #16985u : u32
        let s_19_1: u32 = 16985;
        // N s_19_2: write-reg s_19_1 <= s_19_0
        let s_19_2: () = {
            state.write_register::<bool>(s_19_1 as isize, s_19_0);
            tracer.write_register(s_19_1 as isize, s_19_0);
        };
        // N s_19_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call SCTLR_read__2(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_20_0);
        // S s_20_2: call _get_SCTLR_Type_SPAN(s_20_1)
        let s_20_2: bool = u_get_SCTLR_Type_SPAN(state, tracer, s_20_1);
        // S s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // C s_20_4: const #0u : u8
        let s_20_4: bool = false;
        // C s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 1u16);
        // S s_20_6: cmp-eq s_20_3 s_20_5
        let s_20_6: bool = ((s_20_3) == (s_20_5));
        // D s_20_7: write-var gs#9060 <= s_20_6
        fn_state.gs_9060 = s_20_6;
        // N s_20_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #3u : u8
        let s_21_0: u8 = 3;
        // C s_21_1: const #1s : i
        let s_21_1: i128 = 1;
        // C s_21_2: cast zx s_21_0 -> bv
        let s_21_2: Bits = Bits::new(s_21_0 as u128, 2u16);
        // C s_21_3: const #1s : i64
        let s_21_3: i64 = 1;
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // C s_21_5: const #0s : i
        let s_21_5: i128 = 0;
        // C s_21_6: add s_21_5 s_21_4
        let s_21_6: i128 = (s_21_5 + s_21_4);
        // D s_21_7: bit-extract s_21_2 s_21_1 s_21_6
        let s_21_7: Bits = (Bits::new(
            ((s_21_2) >> (s_21_1)).value(),
            u16::try_from(s_21_6).unwrap(),
        ));
        // D s_21_8: cast reint s_21_7 -> u8
        let s_21_8: bool = ((s_21_7.value()) != 0);
        // C s_21_9: const #16968u : u32
        let s_21_9: u32 = 16968;
        // N s_21_10: write-reg s_21_9 <= s_21_8
        let s_21_10: () = {
            state.write_register::<bool>(s_21_9 as isize, s_21_8);
            tracer.write_register(s_21_9 as isize, s_21_8);
        };
        // C s_21_11: const #0s : i
        let s_21_11: i128 = 0;
        // C s_21_12: cast zx s_21_0 -> bv
        let s_21_12: Bits = Bits::new(s_21_0 as u128, 2u16);
        // C s_21_13: const #1s : i64
        let s_21_13: i64 = 1;
        // C s_21_14: cast zx s_21_13 -> i
        let s_21_14: i128 = (i128::try_from(s_21_13).unwrap());
        // C s_21_15: const #0s : i
        let s_21_15: i128 = 0;
        // C s_21_16: add s_21_15 s_21_14
        let s_21_16: i128 = (s_21_15 + s_21_14);
        // D s_21_17: bit-extract s_21_12 s_21_11 s_21_16
        let s_21_17: Bits = (Bits::new(
            ((s_21_12) >> (s_21_11)).value(),
            u16::try_from(s_21_16).unwrap(),
        ));
        // D s_21_18: cast reint s_21_17 -> u8
        let s_21_18: bool = ((s_21_17.value()) != 0);
        // C s_21_19: const #16979u : u32
        let s_21_19: u32 = 16979;
        // N s_21_20: write-reg s_21_19 <= s_21_18
        let s_21_20: () = {
            state.write_register::<bool>(s_21_19 as isize, s_21_18);
            tracer.write_register(s_21_19 as isize, s_21_18);
        };
        // N s_21_21: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#9067 <= s_22_0
        fn_state.gs_9067 = s_22_0;
        // N s_22_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #7u : u8
        let s_23_0: u8 = 7;
        // C s_23_1: const #2s : i
        let s_23_1: i128 = 2;
        // C s_23_2: cast zx s_23_0 -> bv
        let s_23_2: Bits = Bits::new(s_23_0 as u128, 3u16);
        // C s_23_3: const #1s : i64
        let s_23_3: i64 = 1;
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #0s : i
        let s_23_5: i128 = 0;
        // C s_23_6: add s_23_5 s_23_4
        let s_23_6: i128 = (s_23_5 + s_23_4);
        // D s_23_7: bit-extract s_23_2 s_23_1 s_23_6
        let s_23_7: Bits = (Bits::new(
            ((s_23_2) >> (s_23_1)).value(),
            u16::try_from(s_23_6).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: bool = ((s_23_7.value()) != 0);
        // C s_23_9: const #16968u : u32
        let s_23_9: u32 = 16968;
        // N s_23_10: write-reg s_23_9 <= s_23_8
        let s_23_10: () = {
            state.write_register::<bool>(s_23_9 as isize, s_23_8);
            tracer.write_register(s_23_9 as isize, s_23_8);
        };
        // C s_23_11: const #1s : i
        let s_23_11: i128 = 1;
        // C s_23_12: cast zx s_23_0 -> bv
        let s_23_12: Bits = Bits::new(s_23_0 as u128, 3u16);
        // C s_23_13: const #1s : i64
        let s_23_13: i64 = 1;
        // C s_23_14: cast zx s_23_13 -> i
        let s_23_14: i128 = (i128::try_from(s_23_13).unwrap());
        // C s_23_15: const #0s : i
        let s_23_15: i128 = 0;
        // C s_23_16: add s_23_15 s_23_14
        let s_23_16: i128 = (s_23_15 + s_23_14);
        // D s_23_17: bit-extract s_23_12 s_23_11 s_23_16
        let s_23_17: Bits = (Bits::new(
            ((s_23_12) >> (s_23_11)).value(),
            u16::try_from(s_23_16).unwrap(),
        ));
        // D s_23_18: cast reint s_23_17 -> u8
        let s_23_18: bool = ((s_23_17.value()) != 0);
        // C s_23_19: const #16979u : u32
        let s_23_19: u32 = 16979;
        // N s_23_20: write-reg s_23_19 <= s_23_18
        let s_23_20: () = {
            state.write_register::<bool>(s_23_19 as isize, s_23_18);
            tracer.write_register(s_23_19 as isize, s_23_18);
        };
        // C s_23_21: const #0s : i
        let s_23_21: i128 = 0;
        // C s_23_22: cast zx s_23_0 -> bv
        let s_23_22: Bits = Bits::new(s_23_0 as u128, 3u16);
        // C s_23_23: const #1s : i64
        let s_23_23: i64 = 1;
        // C s_23_24: cast zx s_23_23 -> i
        let s_23_24: i128 = (i128::try_from(s_23_23).unwrap());
        // C s_23_25: const #0s : i
        let s_23_25: i128 = 0;
        // C s_23_26: add s_23_25 s_23_24
        let s_23_26: i128 = (s_23_25 + s_23_24);
        // D s_23_27: bit-extract s_23_22 s_23_21 s_23_26
        let s_23_27: Bits = (Bits::new(
            ((s_23_22) >> (s_23_21)).value(),
            u16::try_from(s_23_26).unwrap(),
        ));
        // D s_23_28: cast reint s_23_27 -> u8
        let s_23_28: bool = ((s_23_27.value()) != 0);
        // C s_23_29: const #16977u : u32
        let s_23_29: u32 = 16977;
        // N s_23_30: write-reg s_23_29 <= s_23_28
        let s_23_30: () = {
            state.write_register::<bool>(s_23_29 as isize, s_23_28);
            tracer.write_register(s_23_29 as isize, s_23_28);
        };
        // N s_23_31: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #20920u : u32
        let s_24_0: u32 = 20920;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // C s_24_2: const #20920u : u32
        let s_24_2: u32 = 20920;
        // N s_24_3: write-reg s_24_2 <= s_24_1
        let s_24_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_24_2 as isize, s_24_1);
            tracer.write_register(s_24_2 as isize, s_24_1);
        };
        // N s_24_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var target_mode:u8
        let s_25_0: u8 = fn_state.target_mode;
        // D s_25_1: call AArch32_EnterModeInDebugState(s_25_0)
        let s_25_1: () = AArch32_EnterModeInDebugState(state, tracer, s_25_0);
        // N s_25_2: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #16975u : u32
        let s_26_0: u32 = 16975;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 2u16);
        // C s_26_3: const #432u : u32
        let s_26_3: u32 = 432;
        // D s_26_4: read-reg s_26_3:u8
        let s_26_4: u8 = {
            let value = state.read_register::<u8>(s_26_3 as isize);
            tracer.read_register(s_26_3 as isize, value);
            value
        };
        // D s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 2u16);
        // D s_26_6: cmp-ne s_26_2 s_26_5
        let s_26_6: bool = ((s_26_2) != (s_26_5));
        // D s_26_7: write-var gs#9052 <= s_26_6
        fn_state.gs_9052 = s_26_6;
        // N s_26_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
