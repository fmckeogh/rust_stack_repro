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
use X_set::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use PAR_EL1_read::*;
use u_get_HFGRTR_EL2_Type_PAR_EL1::*;
use u__get_PAR_EL1::*;
use AArch64_SystemAccessTrap::*;
use IsFeatureImplemented::*;
use EL2Enabled::*;
use common::*;
pub fn PAR_EL1_SysRegRead_8d8d0b85bc9f516c<T: Tracer>(
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
        gs_63624: bool,
        ga_72082: ProductType782ac6922b48c20d,
        gs_63627: bool,
        ga_72094: ProductType782ac6922b48c20d,
        u__SCR_EL3_FGTEn: bool,
        u__HFGRTR_EL2_PAR_EL1: bool,
        gs_63626: bool,
        u__PSTATE_EL: u8,
        ga_72088: ProductType782ac6922b48c20d,
        gs_63625: bool,
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
        // C s_0_3: const #90704u : u32
        let s_0_3: u32 = 90704;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCR_EL3_Type_FGTEn(s_0_4)
        let s_0_5: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_4);
        // D s_0_6: write-var __SCR_EL3_FGTEn <= s_0_5
        fn_state.u__SCR_EL3_FGTEn = s_0_5;
        // C s_0_7: const #16592u : u32
        let s_0_7: u32 = 16592;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HFGRTR_EL2_Type_PAR_EL1(s_0_8)
        let s_0_9: bool = u_get_HFGRTR_EL2_Type_PAR_EL1(state, tracer, s_0_8);
        // D s_0_10: write-var __HFGRTR_EL2_PAR_EL1 <= s_0_9
        fn_state.u__HFGRTR_EL2_PAR_EL1 = s_0_9;
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
        // N s_0_17: branch s_0_16 b22 b1
        if s_0_16 {
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
        // C s_5_1: const #() : ()
        let s_5_1: () = ();
        // S s_5_2: call PAR_EL1_read(s_5_1)
        let s_5_2: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_5_1);
        // S s_5_3: call __get_PAR_EL1(s_5_2)
        let s_5_3: ProductType782ac6922b48c20d = u__get_PAR_EL1(state, tracer, s_5_2);
        // D s_5_4: write-var ga#72094 <= s_5_3
        fn_state.ga_72094 = s_5_3;
        // D s_5_5: read-var ga#72094.0:struct
        let s_5_5: u128 = fn_state.ga_72094._0;
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // D s_5_7: cast zx s_5_5 -> bv
        let s_5_7: Bits = Bits::new(s_5_5 as u128, 128u16);
        // C s_5_8: const #1s : i64
        let s_5_8: i64 = 1;
        // C s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // C s_5_10: const #63s : i
        let s_5_10: i128 = 63;
        // C s_5_11: add s_5_10 s_5_9
        let s_5_11: i128 = (s_5_10 + s_5_9);
        // D s_5_12: bit-extract s_5_7 s_5_6 s_5_11
        let s_5_12: Bits = (Bits::new(
            ((s_5_7) >> (s_5_6)).value(),
            u16::try_from(s_5_11).unwrap(),
        ));
        // D s_5_13: cast reint s_5_12 -> u64
        let s_5_13: u64 = (s_5_12.value() as u64);
        // D s_5_14: cast zx s_5_13 -> bv
        let s_5_14: Bits = Bits::new(s_5_13 as u128, 64u16);
        // D s_5_15: read-var t:i
        let s_5_15: i128 = fn_state.t;
        // D s_5_16: call X_set(s_5_15, s_5_0, s_5_14)
        let s_5_16: () = X_set(state, tracer, s_5_15, s_5_0, s_5_14);
        // N s_5_17: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #() : ()
        let s_6_1: () = ();
        // S s_6_2: call PAR_EL1_read(s_6_1)
        let s_6_2: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_6_1);
        // S s_6_3: call __get_PAR_EL1(s_6_2)
        let s_6_3: ProductType782ac6922b48c20d = u__get_PAR_EL1(state, tracer, s_6_2);
        // D s_6_4: write-var ga#72088 <= s_6_3
        fn_state.ga_72088 = s_6_3;
        // D s_6_5: read-var ga#72088.0:struct
        let s_6_5: u128 = fn_state.ga_72088._0;
        // C s_6_6: const #0s : i
        let s_6_6: i128 = 0;
        // D s_6_7: cast zx s_6_5 -> bv
        let s_6_7: Bits = Bits::new(s_6_5 as u128, 128u16);
        // C s_6_8: const #1s : i64
        let s_6_8: i64 = 1;
        // C s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // C s_6_10: const #63s : i
        let s_6_10: i128 = 63;
        // C s_6_11: add s_6_10 s_6_9
        let s_6_11: i128 = (s_6_10 + s_6_9);
        // D s_6_12: bit-extract s_6_7 s_6_6 s_6_11
        let s_6_12: Bits = (Bits::new(
            ((s_6_7) >> (s_6_6)).value(),
            u16::try_from(s_6_11).unwrap(),
        ));
        // D s_6_13: cast reint s_6_12 -> u64
        let s_6_13: u64 = (s_6_12.value() as u64);
        // D s_6_14: cast zx s_6_13 -> bv
        let s_6_14: Bits = Bits::new(s_6_13 as u128, 64u16);
        // D s_6_15: read-var t:i
        let s_6_15: i128 = fn_state.t;
        // D s_6_16: call X_set(s_6_15, s_6_0, s_6_14)
        let s_6_16: () = X_set(state, tracer, s_6_15, s_6_0, s_6_14);
        // N s_6_17: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call EL2Enabled(s_7_0)
        let s_7_1: bool = EL2Enabled(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b21 b8
        if s_7_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#63624 <= s_8_0
        fn_state.gs_63624 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#63624:u8
        let s_9_0: bool = fn_state.gs_63624;
        // N s_9_1: branch s_9_0 b17 b10
        if s_9_0 {
            return block_17(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#63626 <= s_10_0
        fn_state.gs_63626 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#63626:u8
        let s_11_0: bool = fn_state.gs_63626;
        // N s_11_1: branch s_11_0 b16 b12
        if s_11_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#63627 <= s_12_0
        fn_state.gs_63627 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#63627:u8
        let s_13_0: bool = fn_state.gs_63627;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
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
        // C s_14_0: const #64s : i64
        let s_14_0: i64 = 64;
        // C s_14_1: const #() : ()
        let s_14_1: () = ();
        // S s_14_2: call PAR_EL1_read(s_14_1)
        let s_14_2: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_14_1);
        // S s_14_3: call __get_PAR_EL1(s_14_2)
        let s_14_3: ProductType782ac6922b48c20d = u__get_PAR_EL1(state, tracer, s_14_2);
        // D s_14_4: write-var ga#72082 <= s_14_3
        fn_state.ga_72082 = s_14_3;
        // D s_14_5: read-var ga#72082.0:struct
        let s_14_5: u128 = fn_state.ga_72082._0;
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // D s_14_7: cast zx s_14_5 -> bv
        let s_14_7: Bits = Bits::new(s_14_5 as u128, 128u16);
        // C s_14_8: const #1s : i64
        let s_14_8: i64 = 1;
        // C s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // C s_14_10: const #63s : i
        let s_14_10: i128 = 63;
        // C s_14_11: add s_14_10 s_14_9
        let s_14_11: i128 = (s_14_10 + s_14_9);
        // D s_14_12: bit-extract s_14_7 s_14_6 s_14_11
        let s_14_12: Bits = (Bits::new(
            ((s_14_7) >> (s_14_6)).value(),
            u16::try_from(s_14_11).unwrap(),
        ));
        // D s_14_13: cast reint s_14_12 -> u64
        let s_14_13: u64 = (s_14_12.value() as u64);
        // D s_14_14: cast zx s_14_13 -> bv
        let s_14_14: Bits = Bits::new(s_14_13 as u128, 64u16);
        // D s_14_15: read-var t:i
        let s_14_15: i128 = fn_state.t;
        // D s_14_16: call X_set(s_14_15, s_14_0, s_14_14)
        let s_14_16: () = X_set(state, tracer, s_14_15, s_14_0, s_14_14);
        // N s_14_17: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #24u : u8
        let s_15_0: u8 = 24;
        // C s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 8u16);
        // C s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (s_15_1.value() as i128);
        // C s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // C s_15_5: const #432u : u32
        let s_15_5: u32 = 432;
        // D s_15_6: read-reg s_15_5:u8
        let s_15_6: u8 = {
            let value = state.read_register::<u8>(s_15_5 as isize);
            tracer.read_register(s_15_5 as isize, value);
            value
        };
        // D s_15_7: call AArch64_SystemAccessTrap(s_15_6, s_15_4)
        let s_15_7: () = AArch64_SystemAccessTrap(state, tracer, s_15_6, s_15_4);
        // N s_15_8: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var __HFGRTR_EL2_PAR_EL1:u8
        let s_16_0: bool = fn_state.u__HFGRTR_EL2_PAR_EL1;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: write-var gs#63627 <= s_16_4
        fn_state.gs_63627 = s_16_4;
        // N s_16_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #424u : u32
        let s_17_0: u32 = 424;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // D s_17_4: not s_17_3
        let s_17_4: bool = !s_17_3;
        // N s_17_5: branch s_17_4 b20 b18
        if s_17_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __SCR_EL3_FGTEn:u8
        let s_18_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var gs#63625 <= s_18_4
        fn_state.gs_63625 = s_18_4;
        // N s_18_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#63625:u8
        let s_19_0: bool = fn_state.gs_63625;
        // D s_19_1: write-var gs#63626 <= s_19_0
        fn_state.gs_63626 = s_19_0;
        // N s_19_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#63625 <= s_20_0
        fn_state.gs_63625 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #146u : u32
        let s_21_0: u32 = 146;
        // S s_21_1: call IsFeatureImplemented(s_21_0)
        let s_21_1: bool = IsFeatureImplemented(state, tracer, s_21_0);
        // D s_21_2: write-var gs#63624 <= s_21_1
        fn_state.gs_63624 = s_21_1;
        // N s_21_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
}