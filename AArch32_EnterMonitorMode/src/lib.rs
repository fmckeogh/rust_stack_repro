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
use AArch32_EnterMonitorModeInDebugState::*;
use Halted::*;
use u_get_SCTLR_Type_SPAN::*;
use u_get_SCTLR_Type_TE::*;
use CurrentSecurityState::*;
use HavePANExt::*;
use GetPSRFromPSTATE::*;
use SPSR_set::*;
use SCTLR_read__2::*;
use R_set::*;
use HaveSSBSExt::*;
use integer_subrange::*;
use AArch32_WriteMode::*;
use CheckExceptionCatch::*;
use EndOfInstruction::*;
use SynchronizeContext::*;
use u_get_SCTLR_Type_EE::*;
use u_get_SCTLR_Type_DSSBS::*;
use BranchTo::*;
use ELUsingAArch32::*;
use common::*;
pub fn AArch32_EnterMonitorMode<T: Tracer>(
    state: &mut State,
    tracer: &T,
    preferred_exception_return: u32,
    lr_offset: i128,
    vect_offset: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        spsr: u32,
        gs_9136: bool,
        from_secure: bool,
        preferred_exception_return: u32,
        lr_offset: i128,
        vect_offset: i128,
    }
    let fn_state = FunctionState {
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
        // C s_0_2: const #424u : u32
        let s_0_2: u32 = 424;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #2u : u8
        let s_0_4: u8 = 2;
        // D s_0_5: cmp-lt s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) < (s_0_4));
        // N s_0_6: branch s_0_5 b19 b1
        if s_0_5 {
            return block_19(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#9136 <= s_1_0
        fn_state.gs_9136 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9136:u8
        let s_2_0: bool = fn_state.gs_9136;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call CurrentSecurityState(s_2_2)
        let s_2_3: u32 = CurrentSecurityState(state, tracer, s_2_2);
        // C s_2_4: const #3u : u32
        let s_2_4: u32 = 3;
        // S s_2_5: cmp-eq s_2_3 s_2_4
        let s_2_5: bool = ((s_2_3) == (s_2_4));
        // D s_2_6: write-var from_secure <= s_2_5
        fn_state.from_secure = s_2_5;
        // C s_2_7: const #() : ()
        let s_2_7: () = ();
        // S s_2_8: call Halted(s_2_7)
        let s_2_8: bool = Halted(state, tracer, s_2_7);
        // N s_2_9: branch s_2_8 b18 b3
        if s_2_8 {
            return block_18(state, tracer, fn_state);
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
        // N s_3_12: branch s_3_11 b17 b4
        if s_3_11 {
            return block_17(state, tracer, fn_state);
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
        // C s_5_0: const #384u : u32
        let s_5_0: u32 = 384;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call AArch32_WriteMode(s_5_1)
        let s_5_2: () = AArch32_WriteMode(state, tracer, s_5_1);
        // C s_5_3: const #32s : i
        let s_5_3: i128 = 32;
        // D s_5_4: read-var spsr:u32
        let s_5_4: u32 = fn_state.spsr;
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 32u16);
        // D s_5_6: call SPSR_set(s_5_3, s_5_5)
        let s_5_6: () = SPSR_set(state, tracer, s_5_3, s_5_5);
        // D s_5_7: read-var preferred_exception_return:u32
        let s_5_7: u32 = fn_state.preferred_exception_return;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 32u16);
        // D s_5_9: read-var lr_offset:i
        let s_5_9: i128 = fn_state.lr_offset;
        // D s_5_10: cast cvt s_5_9 -> bv
        let s_5_10: Bits = Bits::new(s_5_9 as u128, 128);
        // D s_5_11: add s_5_8 s_5_10
        let s_5_11: Bits = (s_5_8 + s_5_10);
        // D s_5_12: cast reint s_5_11 -> u32
        let s_5_12: u32 = (s_5_11.value() as u32);
        // C s_5_13: const #14s : i
        let s_5_13: i128 = 14;
        // D s_5_14: call R_set(s_5_13, s_5_12)
        let s_5_14: () = R_set(state, tracer, s_5_13, s_5_12);
        // C s_5_15: const #() : ()
        let s_5_15: () = ();
        // S s_5_16: call SCTLR_read__2(s_5_15)
        let s_5_16: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_5_15);
        // S s_5_17: call _get_SCTLR_Type_TE(s_5_16)
        let s_5_17: bool = u_get_SCTLR_Type_TE(state, tracer, s_5_16);
        // C s_5_18: const #16993u : u32
        let s_5_18: u32 = 16993;
        // N s_5_19: write-reg s_5_18 <= s_5_17
        let s_5_19: () = {
            state.write_register::<bool>(s_5_18 as isize, s_5_17);
            tracer.write_register(s_5_18 as isize, s_5_17);
        };
        // C s_5_20: const #0u : u8
        let s_5_20: bool = false;
        // C s_5_21: const #16991u : u32
        let s_5_21: u32 = 16991;
        // N s_5_22: write-reg s_5_21 <= s_5_20
        let s_5_22: () = {
            state.write_register::<bool>(s_5_21 as isize, s_5_20);
            tracer.write_register(s_5_21 as isize, s_5_20);
        };
        // C s_5_23: const #7u : u8
        let s_5_23: u8 = 7;
        // C s_5_24: const #2s : i
        let s_5_24: i128 = 2;
        // C s_5_25: cast zx s_5_23 -> bv
        let s_5_25: Bits = Bits::new(s_5_23 as u128, 3u16);
        // C s_5_26: const #1s : i64
        let s_5_26: i64 = 1;
        // C s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // C s_5_28: const #0s : i
        let s_5_28: i128 = 0;
        // C s_5_29: add s_5_28 s_5_27
        let s_5_29: i128 = (s_5_28 + s_5_27);
        // D s_5_30: bit-extract s_5_25 s_5_24 s_5_29
        let s_5_30: Bits = (Bits::new(
            ((s_5_25) >> (s_5_24)).value(),
            u16::try_from(s_5_29).unwrap(),
        ));
        // D s_5_31: cast reint s_5_30 -> u8
        let s_5_31: bool = ((s_5_30.value()) != 0);
        // C s_5_32: const #16968u : u32
        let s_5_32: u32 = 16968;
        // N s_5_33: write-reg s_5_32 <= s_5_31
        let s_5_33: () = {
            state.write_register::<bool>(s_5_32 as isize, s_5_31);
            tracer.write_register(s_5_32 as isize, s_5_31);
        };
        // C s_5_34: const #1s : i
        let s_5_34: i128 = 1;
        // C s_5_35: cast zx s_5_23 -> bv
        let s_5_35: Bits = Bits::new(s_5_23 as u128, 3u16);
        // C s_5_36: const #1s : i64
        let s_5_36: i64 = 1;
        // C s_5_37: cast zx s_5_36 -> i
        let s_5_37: i128 = (i128::try_from(s_5_36).unwrap());
        // C s_5_38: const #0s : i
        let s_5_38: i128 = 0;
        // C s_5_39: add s_5_38 s_5_37
        let s_5_39: i128 = (s_5_38 + s_5_37);
        // D s_5_40: bit-extract s_5_35 s_5_34 s_5_39
        let s_5_40: Bits = (Bits::new(
            ((s_5_35) >> (s_5_34)).value(),
            u16::try_from(s_5_39).unwrap(),
        ));
        // D s_5_41: cast reint s_5_40 -> u8
        let s_5_41: bool = ((s_5_40.value()) != 0);
        // C s_5_42: const #16979u : u32
        let s_5_42: u32 = 16979;
        // N s_5_43: write-reg s_5_42 <= s_5_41
        let s_5_43: () = {
            state.write_register::<bool>(s_5_42 as isize, s_5_41);
            tracer.write_register(s_5_42 as isize, s_5_41);
        };
        // C s_5_44: const #0s : i
        let s_5_44: i128 = 0;
        // C s_5_45: cast zx s_5_23 -> bv
        let s_5_45: Bits = Bits::new(s_5_23 as u128, 3u16);
        // C s_5_46: const #1s : i64
        let s_5_46: i64 = 1;
        // C s_5_47: cast zx s_5_46 -> i
        let s_5_47: i128 = (i128::try_from(s_5_46).unwrap());
        // C s_5_48: const #0s : i
        let s_5_48: i128 = 0;
        // C s_5_49: add s_5_48 s_5_47
        let s_5_49: i128 = (s_5_48 + s_5_47);
        // D s_5_50: bit-extract s_5_45 s_5_44 s_5_49
        let s_5_50: Bits = (Bits::new(
            ((s_5_45) >> (s_5_44)).value(),
            u16::try_from(s_5_49).unwrap(),
        ));
        // D s_5_51: cast reint s_5_50 -> u8
        let s_5_51: bool = ((s_5_50.value()) != 0);
        // C s_5_52: const #16977u : u32
        let s_5_52: u32 = 16977;
        // N s_5_53: write-reg s_5_52 <= s_5_51
        let s_5_53: () = {
            state.write_register::<bool>(s_5_52 as isize, s_5_51);
            tracer.write_register(s_5_52 as isize, s_5_51);
        };
        // C s_5_54: const #() : ()
        let s_5_54: () = ();
        // S s_5_55: call SCTLR_read__2(s_5_54)
        let s_5_55: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_5_54);
        // S s_5_56: call _get_SCTLR_Type_EE(s_5_55)
        let s_5_56: bool = u_get_SCTLR_Type_EE(state, tracer, s_5_55);
        // C s_5_57: const #16974u : u32
        let s_5_57: u32 = 16974;
        // N s_5_58: write-reg s_5_57 <= s_5_56
        let s_5_58: () = {
            state.write_register::<bool>(s_5_57 as isize, s_5_56);
            tracer.write_register(s_5_57 as isize, s_5_56);
        };
        // C s_5_59: const #0u : u8
        let s_5_59: bool = false;
        // C s_5_60: const #16980u : u32
        let s_5_60: u32 = 16980;
        // N s_5_61: write-reg s_5_60 <= s_5_59
        let s_5_61: () = {
            state.write_register::<bool>(s_5_60 as isize, s_5_59);
            tracer.write_register(s_5_60 as isize, s_5_59);
        };
        // C s_5_62: const #0u : u8
        let s_5_62: u8 = 0;
        // C s_5_63: const #16981u : u32
        let s_5_63: u32 = 16981;
        // N s_5_64: write-reg s_5_63 <= s_5_62
        let s_5_64: () = {
            state.write_register::<u8>(s_5_63 as isize, s_5_62);
            tracer.write_register(s_5_63 as isize, s_5_62);
        };
        // C s_5_65: const #() : ()
        let s_5_65: () = ();
        // S s_5_66: call HavePANExt(s_5_65)
        let s_5_66: bool = HavePANExt(state, tracer, s_5_65);
        // N s_5_67: branch s_5_66 b12 b6
        if s_5_66 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveSSBSExt(s_7_0)
        let s_7_1: bool = HaveSSBSExt(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b11 b8
        if s_7_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // C s_9_1: const #100208u : u32
        let s_9_1: u32 = 100208;
        // D s_9_2: read-reg s_9_1:u32
        let s_9_2: u32 = {
            let value = state.read_register::<u32>(s_9_1 as isize);
            tracer.read_register(s_9_1 as isize, value);
            value
        };
        // C s_9_3: const #5s : i
        let s_9_3: i128 = 5;
        // D s_9_4: cast zx s_9_2 -> bv
        let s_9_4: Bits = Bits::new(s_9_2 as u128, 32u16);
        // C s_9_5: const #1s : i64
        let s_9_5: i64 = 1;
        // C s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // C s_9_7: const #26s : i
        let s_9_7: i128 = 26;
        // C s_9_8: add s_9_7 s_9_6
        let s_9_8: i128 = (s_9_7 + s_9_6);
        // D s_9_9: bit-extract s_9_4 s_9_3 s_9_8
        let s_9_9: Bits = (Bits::new(
            ((s_9_4) >> (s_9_3)).value(),
            u16::try_from(s_9_8).unwrap(),
        ));
        // D s_9_10: cast reint s_9_9 -> u27
        let s_9_10: u32 = (s_9_9.value() as u32);
        // C s_9_11: const #4s : i
        let s_9_11: i128 = 4;
        // C s_9_12: const #0s : i
        let s_9_12: i128 = 0;
        // D s_9_13: read-var vect_offset:i
        let s_9_13: i128 = fn_state.vect_offset;
        // D s_9_14: call integer_subrange(s_9_13, s_9_11, s_9_12)
        let s_9_14: Bits = integer_subrange(state, tracer, s_9_13, s_9_11, s_9_12);
        // D s_9_15: cast reint s_9_14 -> u8
        let s_9_15: u8 = (s_9_14.value() as u8);
        // D s_9_16: cast zx s_9_10 -> bv
        let s_9_16: Bits = Bits::new(s_9_10 as u128, 27u16);
        // D s_9_17: cast zx s_9_15 -> bv
        let s_9_17: Bits = Bits::new(s_9_15 as u128, 5u16);
        // D s_9_18: cast reint s_9_16 -> u128
        let s_9_18: u128 = (s_9_16.value() as u128);
        // D s_9_19: size-of s_9_16
        let s_9_19: u16 = s_9_16.length();
        // D s_9_20: cast reint s_9_17 -> u128
        let s_9_20: u128 = (s_9_17.value() as u128);
        // D s_9_21: size-of s_9_17
        let s_9_21: u16 = s_9_17.length();
        // D s_9_22: lsl s_9_18 s_9_21
        let s_9_22: u128 = s_9_18 << s_9_21;
        // D s_9_23: or s_9_22 s_9_20
        let s_9_23: u128 = ((s_9_22) | (s_9_20));
        // D s_9_24: add s_9_19 s_9_21
        let s_9_24: u16 = (s_9_19 + s_9_21);
        // D s_9_25: create-bits s_9_23 s_9_24
        let s_9_25: Bits = Bits::new(s_9_23, s_9_24);
        // D s_9_26: cast reint s_9_25 -> u32
        let s_9_26: u32 = (s_9_25.value() as u32);
        // D s_9_27: cast zx s_9_26 -> bv
        let s_9_27: Bits = Bits::new(s_9_26 as u128, 32u16);
        // C s_9_28: const #7u : u32
        let s_9_28: u32 = 7;
        // D s_9_29: call BranchTo(s_9_27, s_9_28, s_9_0)
        let s_9_29: () = BranchTo(state, tracer, s_9_27, s_9_28, s_9_0);
        // C s_9_30: const #1u : u8
        let s_9_30: bool = true;
        // S s_9_31: call CheckExceptionCatch(s_9_30)
        let s_9_31: () = CheckExceptionCatch(state, tracer, s_9_30);
        // N s_9_32: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call EndOfInstruction(s_10_0)
        let s_10_1: () = EndOfInstruction(state, tracer, s_10_0);
        // N s_10_2: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call SCTLR_read__2(s_11_0)
        let s_11_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_11_0);
        // S s_11_2: call _get_SCTLR_Type_DSSBS(s_11_1)
        let s_11_2: bool = u_get_SCTLR_Type_DSSBS(state, tracer, s_11_1);
        // C s_11_3: const #16992u : u32
        let s_11_3: u32 = 16992;
        // N s_11_4: write-reg s_11_3 <= s_11_2
        let s_11_4: () = {
            state.write_register::<bool>(s_11_3 as isize, s_11_2);
            tracer.write_register(s_11_3 as isize, s_11_2);
        };
        // N s_11_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var from_secure:u8
        let s_12_0: bool = fn_state.from_secure;
        // D s_12_1: not s_12_0
        let s_12_1: bool = !s_12_0;
        // N s_12_2: branch s_12_1 b16 b13
        if s_12_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call SCTLR_read__2(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_13_0);
        // S s_13_2: call _get_SCTLR_Type_SPAN(s_13_1)
        let s_13_2: bool = u_get_SCTLR_Type_SPAN(state, tracer, s_13_1);
        // S s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: const #0u : u8
        let s_13_4: bool = false;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // S s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // N s_13_7: branch s_13_6 b15 b14
        if s_13_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // C s_15_1: const #16985u : u32
        let s_15_1: u32 = 16985;
        // N s_15_2: write-reg s_15_1 <= s_15_0
        let s_15_2: () = {
            state.write_register::<bool>(s_15_1 as isize, s_15_0);
            tracer.write_register(s_15_1 as isize, s_15_0);
        };
        // N s_15_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // C s_16_1: const #16985u : u32
        let s_16_1: u32 = 16985;
        // N s_16_2: write-reg s_16_1 <= s_16_0
        let s_16_2: () = {
            state.write_register::<bool>(s_16_1 as isize, s_16_0);
            tracer.write_register(s_16_1 as isize, s_16_0);
        };
        // N s_16_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #20920u : u32
        let s_17_0: u32 = 20920;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #20920u : u32
        let s_17_2: u32 = 20920;
        // N s_17_3: write-reg s_17_2 <= s_17_1
        let s_17_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_17_2 as isize, s_17_1);
            tracer.write_register(s_17_2 as isize, s_17_1);
        };
        // N s_17_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call AArch32_EnterMonitorModeInDebugState(s_18_0)
        let s_18_1: () = AArch32_EnterMonitorModeInDebugState(state, tracer, s_18_0);
        // N s_18_2: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #424u : u32
        let s_19_0: u32 = 424;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call ELUsingAArch32(s_19_1)
        let s_19_2: bool = ELUsingAArch32(state, tracer, s_19_1);
        // D s_19_3: write-var gs#9136 <= s_19_2
        fn_state.gs_9136 = s_19_2;
        // N s_19_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
