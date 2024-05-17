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
use AArch64_PAMax::*;
use BranchTo::*;
use IsZero::*;
use AArch64_ResetControlRegisters::*;
use HaveAArch64::*;
use AArch64_ResetSpecialRegisters::*;
use ResetExternalDebugRegisters::*;
use is_zero_subrange::*;
use AArch64_ResetGeneralRegisters::*;
use HaveTME::*;
use AArch64_ResetSIMDFPRegisters::*;
use common::*;
pub fn AArch64_TakeReset<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cold_reset: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        rv: u64,
        gs_24602: bool,
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
        // N s_0_2: assert s_0_1
        let s_0_2: () = assert!(s_0_1);
        // C s_0_3: const #0u : u8
        let s_0_3: bool = false;
        // C s_0_4: const #16999u : u32
        let s_0_4: u32 = 16999;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<bool>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // C s_0_6: const #424u : u32
        let s_0_6: u32 = 424;
        // D s_0_7: read-reg s_0_6:u8
        let s_0_7: u8 = {
            let value = state.read_register::<u8>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // C s_0_8: const #2u : u8
        let s_0_8: u8 = 2;
        // D s_0_9: cmp-lt s_0_7 s_0_8
        let s_0_9: bool = ((s_0_7) < (s_0_8));
        // N s_0_10: branch s_0_9 b16 b1
        if s_0_9 {
            return block_16(state, tracer, fn_state);
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
        // N s_1_4: branch s_1_3 b15 b2
        if s_1_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #440u : u32
        let s_2_0: u32 = 440;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // C s_2_2: const #16975u : u32
        let s_2_2: u32 = 16975;
        // N s_2_3: write-reg s_2_2 <= s_2_1
        let s_2_3: () = {
            state.write_register::<u8>(s_2_2 as isize, s_2_1);
            tracer.write_register(s_2_2 as isize, s_2_1);
        };
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var cold_reset:u8
        let s_3_0: bool = fn_state.cold_reset;
        // D s_3_1: call AArch64_ResetControlRegisters(s_3_0)
        let s_3_1: () = AArch64_ResetControlRegisters(state, tracer, s_3_0);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // C s_3_3: const #16990u : u32
        let s_3_3: u32 = 16990;
        // N s_3_4: write-reg s_3_3 <= s_3_2
        let s_3_4: () = {
            state.write_register::<bool>(s_3_3 as isize, s_3_2);
            tracer.write_register(s_3_3 as isize, s_3_2);
        };
        // C s_3_5: const #15u : u8
        let s_3_5: u8 = 15;
        // C s_3_6: const #3s : i
        let s_3_6: i128 = 3;
        // C s_3_7: cast zx s_3_5 -> bv
        let s_3_7: Bits = Bits::new(s_3_5 as u128, 4u16);
        // C s_3_8: const #1s : i64
        let s_3_8: i64 = 1;
        // C s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // C s_3_10: const #0s : i
        let s_3_10: i128 = 0;
        // C s_3_11: add s_3_10 s_3_9
        let s_3_11: i128 = (s_3_10 + s_3_9);
        // D s_3_12: bit-extract s_3_7 s_3_6 s_3_11
        let s_3_12: Bits = (Bits::new(
            ((s_3_7) >> (s_3_6)).value(),
            u16::try_from(s_3_11).unwrap(),
        ));
        // D s_3_13: cast reint s_3_12 -> u8
        let s_3_13: bool = ((s_3_12.value()) != 0);
        // C s_3_14: const #16972u : u32
        let s_3_14: u32 = 16972;
        // N s_3_15: write-reg s_3_14 <= s_3_13
        let s_3_15: () = {
            state.write_register::<bool>(s_3_14 as isize, s_3_13);
            tracer.write_register(s_3_14 as isize, s_3_13);
        };
        // C s_3_16: const #2s : i
        let s_3_16: i128 = 2;
        // C s_3_17: cast zx s_3_5 -> bv
        let s_3_17: Bits = Bits::new(s_3_5 as u128, 4u16);
        // C s_3_18: const #1s : i64
        let s_3_18: i64 = 1;
        // C s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // C s_3_20: const #0s : i
        let s_3_20: i128 = 0;
        // C s_3_21: add s_3_20 s_3_19
        let s_3_21: i128 = (s_3_20 + s_3_19);
        // D s_3_22: bit-extract s_3_17 s_3_16 s_3_21
        let s_3_22: Bits = (Bits::new(
            ((s_3_17) >> (s_3_16)).value(),
            u16::try_from(s_3_21).unwrap(),
        ));
        // D s_3_23: cast reint s_3_22 -> u8
        let s_3_23: bool = ((s_3_22.value()) != 0);
        // C s_3_24: const #16968u : u32
        let s_3_24: u32 = 16968;
        // N s_3_25: write-reg s_3_24 <= s_3_23
        let s_3_25: () = {
            state.write_register::<bool>(s_3_24 as isize, s_3_23);
            tracer.write_register(s_3_24 as isize, s_3_23);
        };
        // C s_3_26: const #1s : i
        let s_3_26: i128 = 1;
        // C s_3_27: cast zx s_3_5 -> bv
        let s_3_27: Bits = Bits::new(s_3_5 as u128, 4u16);
        // C s_3_28: const #1s : i64
        let s_3_28: i64 = 1;
        // C s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // C s_3_30: const #0s : i
        let s_3_30: i128 = 0;
        // C s_3_31: add s_3_30 s_3_29
        let s_3_31: i128 = (s_3_30 + s_3_29);
        // D s_3_32: bit-extract s_3_27 s_3_26 s_3_31
        let s_3_32: Bits = (Bits::new(
            ((s_3_27) >> (s_3_26)).value(),
            u16::try_from(s_3_31).unwrap(),
        ));
        // D s_3_33: cast reint s_3_32 -> u8
        let s_3_33: bool = ((s_3_32.value()) != 0);
        // C s_3_34: const #16979u : u32
        let s_3_34: u32 = 16979;
        // N s_3_35: write-reg s_3_34 <= s_3_33
        let s_3_35: () = {
            state.write_register::<bool>(s_3_34 as isize, s_3_33);
            tracer.write_register(s_3_34 as isize, s_3_33);
        };
        // C s_3_36: const #0s : i
        let s_3_36: i128 = 0;
        // C s_3_37: cast zx s_3_5 -> bv
        let s_3_37: Bits = Bits::new(s_3_5 as u128, 4u16);
        // C s_3_38: const #1s : i64
        let s_3_38: i64 = 1;
        // C s_3_39: cast zx s_3_38 -> i
        let s_3_39: i128 = (i128::try_from(s_3_38).unwrap());
        // C s_3_40: const #0s : i
        let s_3_40: i128 = 0;
        // C s_3_41: add s_3_40 s_3_39
        let s_3_41: i128 = (s_3_40 + s_3_39);
        // D s_3_42: bit-extract s_3_37 s_3_36 s_3_41
        let s_3_42: Bits = (Bits::new(
            ((s_3_37) >> (s_3_36)).value(),
            u16::try_from(s_3_41).unwrap(),
        ));
        // D s_3_43: cast reint s_3_42 -> u8
        let s_3_43: bool = ((s_3_42.value()) != 0);
        // C s_3_44: const #16977u : u32
        let s_3_44: u32 = 16977;
        // N s_3_45: write-reg s_3_44 <= s_3_43
        let s_3_45: () = {
            state.write_register::<bool>(s_3_44 as isize, s_3_43);
            tracer.write_register(s_3_44 as isize, s_3_43);
        };
        // C s_3_46: const #0u : u8
        let s_3_46: bool = false;
        // C s_3_47: const #16991u : u32
        let s_3_47: u32 = 16991;
        // N s_3_48: write-reg s_3_47 <= s_3_46
        let s_3_48: () = {
            state.write_register::<bool>(s_3_47 as isize, s_3_46);
            tracer.write_register(s_3_47 as isize, s_3_46);
        };
        // C s_3_49: const #0u : u8
        let s_3_49: bool = false;
        // C s_3_50: const #16973u : u32
        let s_3_50: u32 = 16973;
        // N s_3_51: write-reg s_3_50 <= s_3_49
        let s_3_51: () = {
            state.write_register::<bool>(s_3_50 as isize, s_3_49);
            tracer.write_register(s_3_50 as isize, s_3_49);
        };
        // C s_3_52: const #0u : u8
        let s_3_52: bool = false;
        // C s_3_53: const #16980u : u32
        let s_3_53: u32 = 16980;
        // N s_3_54: write-reg s_3_53 <= s_3_52
        let s_3_54: () = {
            state.write_register::<bool>(s_3_53 as isize, s_3_52);
            tracer.write_register(s_3_53 as isize, s_3_52);
        };
        // C s_3_55: const #() : ()
        let s_3_55: () = ();
        // S s_3_56: call HaveTME(s_3_55)
        let s_3_56: bool = HaveTME(state, tracer, s_3_55);
        // N s_3_57: branch s_3_56 b14 b4
        if s_3_56 {
            return block_14(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call AArch64_ResetGeneralRegisters(s_5_0)
        let s_5_1: () = AArch64_ResetGeneralRegisters(state, tracer, s_5_0);
        // C s_5_2: const #() : ()
        let s_5_2: () = ();
        // S s_5_3: call AArch64_ResetSIMDFPRegisters(s_5_2)
        let s_5_3: () = AArch64_ResetSIMDFPRegisters(state, tracer, s_5_2);
        // C s_5_4: const #() : ()
        let s_5_4: () = ();
        // S s_5_5: call AArch64_ResetSpecialRegisters(s_5_4)
        let s_5_5: () = AArch64_ResetSpecialRegisters(state, tracer, s_5_4);
        // D s_5_6: read-var cold_reset:u8
        let s_5_6: bool = fn_state.cold_reset;
        // D s_5_7: call ResetExternalDebugRegisters(s_5_6)
        let s_5_7: () = ResetExternalDebugRegisters(state, tracer, s_5_6);
        // C s_5_8: const #424u : u32
        let s_5_8: u32 = 424;
        // D s_5_9: read-reg s_5_8:u8
        let s_5_9: u8 = {
            let value = state.read_register::<u8>(s_5_8 as isize);
            tracer.read_register(s_5_8 as isize, value);
            value
        };
        // C s_5_10: const #2u : u8
        let s_5_10: u8 = 2;
        // D s_5_11: cmp-lt s_5_9 s_5_10
        let s_5_11: bool = ((s_5_9) < (s_5_10));
        // N s_5_12: branch s_5_11 b13 b6
        if s_5_11 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #432u : u32
        let s_6_0: u32 = 432;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // D s_6_3: cmp-lt s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) < (s_6_2));
        // N s_6_4: branch s_6_3 b12 b7
        if s_6_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16264u : u32
        let s_7_0: u32 = 16264;
        // D s_7_1: read-reg s_7_0:u64
        let s_7_1: u64 = {
            let value = state.read_register::<u64>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: write-var rv <= s_7_1
        fn_state.rv = s_7_1;
        // N s_7_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call AArch64_PAMax(s_8_0)
        let s_8_1: i64 = AArch64_PAMax(state, tracer, s_8_0);
        // C s_8_2: const #63s : i
        let s_8_2: i128 = 63;
        // D s_8_3: read-var rv:u64
        let s_8_3: u64 = fn_state.rv;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 64u16);
        // S s_8_5: cast zx s_8_1 -> i
        let s_8_5: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_6: call is_zero_subrange(s_8_4, s_8_2, s_8_5)
        let s_8_6: bool = is_zero_subrange(state, tracer, s_8_4, s_8_2, s_8_5);
        // N s_8_7: branch s_8_6 b11 b9
        if s_8_6 {
            return block_11(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#24602 <= s_9_0
        fn_state.gs_24602 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#24602:u8
        let s_10_0: bool = fn_state.gs_24602;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // D s_10_3: read-var rv:u64
        let s_10_3: u64 = fn_state.rv;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 64u16);
        // C s_10_5: const #9u : u32
        let s_10_5: u32 = 9;
        // D s_10_6: call BranchTo(s_10_4, s_10_5, s_10_2)
        let s_10_6: () = BranchTo(state, tracer, s_10_4, s_10_5, s_10_2);
        // N s_10_7: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var rv:u64
        let s_11_1: u64 = fn_state.rv;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 64u16);
        // C s_11_3: const #1s : i64
        let s_11_3: i64 = 1;
        // C s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // C s_11_5: const #1s : i
        let s_11_5: i128 = 1;
        // C s_11_6: add s_11_5 s_11_4
        let s_11_6: i128 = (s_11_5 + s_11_4);
        // D s_11_7: bit-extract s_11_2 s_11_0 s_11_6
        let s_11_7: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_6).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: u8 = (s_11_7.value() as u8);
        // D s_11_9: cast zx s_11_8 -> bv
        let s_11_9: Bits = Bits::new(s_11_8 as u128, 2u16);
        // D s_11_10: call IsZero(s_11_9)
        let s_11_10: bool = IsZero(state, tracer, s_11_9);
        // D s_11_11: write-var gs#24602 <= s_11_10
        fn_state.gs_24602 = s_11_10;
        // N s_11_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #89592u : u32
        let s_12_0: u32 = 89592;
        // D s_12_1: read-reg s_12_0:u64
        let s_12_1: u64 = {
            let value = state.read_register::<u64>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: write-var rv <= s_12_1
        fn_state.rv = s_12_1;
        // N s_12_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #15768u : u32
        let s_13_0: u32 = 15768;
        // D s_13_1: read-reg s_13_0:u64
        let s_13_1: u64 = {
            let value = state.read_register::<u64>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: write-var rv <= s_13_1
        fn_state.rv = s_13_1;
        // N s_13_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i
        let s_14_0: i128 = 0;
        // C s_14_1: const #100180u : u32
        let s_14_1: u32 = 100180;
        // N s_14_2: write-reg s_14_1 <= s_14_0
        let s_14_2: () = {
            state.write_register::<i128>(s_14_1 as isize, s_14_0);
            tracer.write_register(s_14_1 as isize, s_14_0);
        };
        // N s_14_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #432u : u32
        let s_15_0: u32 = 432;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #16975u : u32
        let s_15_2: u32 = 16975;
        // N s_15_3: write-reg s_15_2 <= s_15_1
        let s_15_3: () = {
            state.write_register::<u8>(s_15_2 as isize, s_15_1);
            tracer.write_register(s_15_2 as isize, s_15_1);
        };
        // N s_15_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // C s_16_2: const #16975u : u32
        let s_16_2: u32 = 16975;
        // N s_16_3: write-reg s_16_2 <= s_16_1
        let s_16_3: () = {
            state.write_register::<u8>(s_16_2 as isize, s_16_1);
            tracer.write_register(s_16_2 as isize, s_16_1);
        };
        // N s_16_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
