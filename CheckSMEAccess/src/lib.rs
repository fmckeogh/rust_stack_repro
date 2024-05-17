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
use u_get_CPTR_EL2_Type_SMEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use IsInHost::*;
use u_get_CPTR_EL2_Type_TSM::*;
use SMEAccessTrap::*;
use u_get_CPTR_EL3_Type_ESM::*;
use u_get_CPACR_EL1_Type_SMEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use HaveVirtHostExt::*;
use common::*;
pub fn CheckSMEAccess<T: Tracer>(state: &mut State, tracer: &T, gs_21286: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_21287: bool,
        ga_16697: u8,
        gs_21301: bool,
        disabled: bool,
        gs_21291: bool,
        gs_21290: bool,
        gs_21295: bool,
        gs_21289: bool,
        gs_21288: bool,
        ga_16710: u8,
        gs_21286: (),
    }
    let fn_state = FunctionState {
        gs_21286,
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
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b52 b1
        if s_0_6 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#21287 <= s_1_6
        fn_state.gs_21287 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#21287:u8
        let s_2_0: bool = fn_state.gs_21287;
        // N s_2_1: branch s_2_0 b51 b3
        if s_2_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#21288 <= s_3_0
        fn_state.gs_21288 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#21288:u8
        let s_4_0: bool = fn_state.gs_21288;
        // N s_4_1: branch s_4_0 b42 b5
        if s_4_0 {
            return block_42(state, tracer, fn_state);
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
        // C s_6_0: const #16975u : u32
        let s_6_0: u32 = 16975;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 2u16);
        // C s_6_3: const #448u : u32
        let s_6_3: u32 = 448;
        // D s_6_4: read-reg s_6_3:u8
        let s_6_4: u8 = {
            let value = state.read_register::<u8>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 2u16);
        // D s_6_6: cmp-eq s_6_2 s_6_5
        let s_6_6: bool = ((s_6_2) == (s_6_5));
        // N s_6_7: branch s_6_6 b41 b7
        if s_6_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16975u : u32
        let s_7_0: u32 = 16975;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 2u16);
        // C s_7_3: const #440u : u32
        let s_7_3: u32 = 440;
        // D s_7_4: read-reg s_7_3:u8
        let s_7_4: u8 = {
            let value = state.read_register::<u8>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 2u16);
        // D s_7_6: cmp-eq s_7_2 s_7_5
        let s_7_6: bool = ((s_7_2) == (s_7_5));
        // N s_7_7: branch s_7_6 b40 b8
        if s_7_6 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16975u : u32
        let s_8_0: u32 = 16975;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 2u16);
        // C s_8_3: const #432u : u32
        let s_8_3: u32 = 432;
        // D s_8_4: read-reg s_8_3:u8
        let s_8_4: u8 = {
            let value = state.read_register::<u8>(s_8_3 as isize);
            tracer.read_register(s_8_3 as isize, value);
            value
        };
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 2u16);
        // D s_8_6: cmp-eq s_8_2 s_8_5
        let s_8_6: bool = ((s_8_2) == (s_8_5));
        // D s_8_7: write-var gs#21289 <= s_8_6
        fn_state.gs_21289 = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#21289:u8
        let s_9_0: bool = fn_state.gs_21289;
        // D s_9_1: write-var gs#21290 <= s_9_0
        fn_state.gs_21290 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#21290:u8
        let s_10_0: bool = fn_state.gs_21290;
        // N s_10_1: branch s_10_0 b39 b11
        if s_10_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#21291 <= s_11_0
        fn_state.gs_21291 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#21291:u8
        let s_12_0: bool = fn_state.gs_21291;
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
        // C s_14_0: const #424u : u32
        let s_14_0: u32 = 424;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #2u : u8
        let s_14_2: u8 = 2;
        // D s_14_3: cmp-lt s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) < (s_14_2));
        // N s_14_4: branch s_14_3 b16 b15
        if s_14_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16840u : u32
        let s_16_0: u32 = 16840;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_CPTR_EL3_Type_ESM(s_16_1)
        let s_16_2: bool = u_get_CPTR_EL3_Type_ESM(state, tracer, s_16_1);
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #0u : u8
        let s_16_4: bool = false;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // N s_16_7: branch s_16_6 b18 b17
        if s_16_6 {
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
        // N s_17_0: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u32
        let s_18_0: u32 = 0;
        // C s_18_1: const #424u : u32
        let s_18_1: u32 = 424;
        // D s_18_2: read-reg s_18_1:u8
        let s_18_2: u8 = {
            let value = state.read_register::<u8>(s_18_1 as isize);
            tracer.read_register(s_18_1 as isize, value);
            value
        };
        // D s_18_3: call SMEAccessTrap(s_18_0, s_18_2)
        let s_18_3: () = SMEAccessTrap(state, tracer, s_18_0, s_18_2);
        // N s_18_4: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HaveVirtHostExt(s_19_0)
        let s_19_1: bool = HaveVirtHostExt(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b38 b20
        if s_19_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#21295 <= s_20_0
        fn_state.gs_21295 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#21295:u8
        let s_21_0: bool = fn_state.gs_21295;
        // N s_21_1: branch s_21_0 b26 b22
        if s_21_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #11088u : u32
        let s_22_0: u32 = 11088;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_CPTR_EL2_Type_TSM(s_22_1)
        let s_22_2: bool = u_get_CPTR_EL2_Type_TSM(state, tracer, s_22_1);
        // D s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // C s_22_4: const #1u : u8
        let s_22_4: bool = true;
        // C s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 1u16);
        // D s_22_6: cmp-eq s_22_3 s_22_5
        let s_22_6: bool = ((s_22_3) == (s_22_5));
        // N s_22_7: branch s_22_6 b25 b23
        if s_22_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u32
        let s_25_0: u32 = 0;
        // C s_25_1: const #432u : u32
        let s_25_1: u32 = 432;
        // D s_25_2: read-reg s_25_1:u8
        let s_25_2: u8 = {
            let value = state.read_register::<u8>(s_25_1 as isize);
            tracer.read_register(s_25_1 as isize, value);
            value
        };
        // D s_25_3: call SMEAccessTrap(s_25_0, s_25_2)
        let s_25_3: () = SMEAccessTrap(state, tracer, s_25_0, s_25_2);
        // N s_25_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #11088u : u32
        let s_26_0: u32 = 11088;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_CPTR_EL2_Type_SMEN(s_26_1)
        let s_26_2: u8 = u_get_CPTR_EL2_Type_SMEN(state, tracer, s_26_1);
        // D s_26_3: write-var ga#16710 <= s_26_2
        fn_state.ga_16710 = s_26_2;
        // D s_26_4: read-var ga#16710:u8
        let s_26_4: u8 = fn_state.ga_16710;
        // C s_26_5: const #0s : i
        let s_26_5: i128 = 0;
        // D s_26_6: cast zx s_26_4 -> bv
        let s_26_6: Bits = Bits::new(s_26_4 as u128, 2u16);
        // C s_26_7: const #1s : i64
        let s_26_7: i64 = 1;
        // C s_26_8: cast zx s_26_7 -> i
        let s_26_8: i128 = (i128::try_from(s_26_7).unwrap());
        // C s_26_9: const #0s : i
        let s_26_9: i128 = 0;
        // C s_26_10: add s_26_9 s_26_8
        let s_26_10: i128 = (s_26_9 + s_26_8);
        // D s_26_11: bit-extract s_26_6 s_26_5 s_26_10
        let s_26_11: Bits = (Bits::new(
            ((s_26_6) >> (s_26_5)).value(),
            u16::try_from(s_26_10).unwrap(),
        ));
        // D s_26_12: cast reint s_26_11 -> u8
        let s_26_12: bool = ((s_26_11.value()) != 0);
        // D s_26_13: cast zx s_26_12 -> bv
        let s_26_13: Bits = Bits::new(s_26_12 as u128, 1u16);
        // C s_26_14: const #0u : u8
        let s_26_14: bool = false;
        // C s_26_15: cast zx s_26_14 -> bv
        let s_26_15: Bits = Bits::new(s_26_14 as u128, 1u16);
        // D s_26_16: cmp-eq s_26_13 s_26_15
        let s_26_16: bool = ((s_26_13) == (s_26_15));
        // D s_26_17: not s_26_16
        let s_26_17: bool = !s_26_16;
        // N s_26_18: branch s_26_17 b32 b27
        if s_26_17 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var disabled <= s_27_0
        fn_state.disabled = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var disabled:u8
        let s_28_0: bool = fn_state.disabled;
        // N s_28_1: branch s_28_0 b31 b29
        if s_28_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u32
        let s_31_0: u32 = 0;
        // C s_31_1: const #432u : u32
        let s_31_1: u32 = 432;
        // D s_31_2: read-reg s_31_1:u8
        let s_31_2: u8 = {
            let value = state.read_register::<u8>(s_31_1 as isize);
            tracer.read_register(s_31_1 as isize, value);
            value
        };
        // D s_31_3: call SMEAccessTrap(s_31_0, s_31_2)
        let s_31_3: () = SMEAccessTrap(state, tracer, s_31_0, s_31_2);
        // N s_31_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var ga#16710:u8
        let s_32_0: u8 = fn_state.ga_16710;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 2u16);
        // C s_32_2: const #1u : u8
        let s_32_2: u8 = 1;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 2u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: not s_32_4
        let s_32_5: bool = !s_32_4;
        // N s_32_6: branch s_32_5 b37 b33
        if s_32_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #16975u : u32
        let s_33_0: u32 = 16975;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: cast zx s_33_1 -> bv
        let s_33_2: Bits = Bits::new(s_33_1 as u128, 2u16);
        // C s_33_3: const #448u : u32
        let s_33_3: u32 = 448;
        // D s_33_4: read-reg s_33_3:u8
        let s_33_4: u8 = {
            let value = state.read_register::<u8>(s_33_3 as isize);
            tracer.read_register(s_33_3 as isize, value);
            value
        };
        // D s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 2u16);
        // D s_33_6: cmp-eq s_33_2 s_33_5
        let s_33_6: bool = ((s_33_2) == (s_33_5));
        // N s_33_7: branch s_33_6 b36 b34
        if s_33_6 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#21301 <= s_34_0
        fn_state.gs_21301 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#21301:u8
        let s_35_0: bool = fn_state.gs_21301;
        // D s_35_1: write-var disabled <= s_35_0
        fn_state.disabled = s_35_0;
        // N s_35_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #102552u : u32
        let s_36_0: u32 = 102552;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_HCR_EL2_Type_TGE(s_36_1)
        let s_36_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_36_1);
        // D s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // D s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // D s_36_7: write-var gs#21301 <= s_36_6
        fn_state.gs_21301 = s_36_6;
        // N s_36_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var disabled <= s_37_0
        fn_state.disabled = s_37_0;
        // N s_37_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #102552u : u32
        let s_38_0: u32 = 102552;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call _get_HCR_EL2_Type_E2H(s_38_1)
        let s_38_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_38_1);
        // D s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #1u : u8
        let s_38_4: bool = true;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // D s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: write-var gs#21295 <= s_38_6
        fn_state.gs_21295 = s_38_6;
        // N s_38_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call EL2Enabled(s_39_0)
        let s_39_1: bool = EL2Enabled(state, tracer, s_39_0);
        // D s_39_2: write-var gs#21291 <= s_39_1
        fn_state.gs_21291 = s_39_1;
        // N s_39_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#21289 <= s_40_0
        fn_state.gs_21289 = s_40_0;
        // N s_40_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#21290 <= s_41_0
        fn_state.gs_21290 = s_41_0;
        // N s_41_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #12088u : u32
        let s_42_0: u32 = 12088;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_CPACR_EL1_Type_SMEN(s_42_1)
        let s_42_2: u8 = u_get_CPACR_EL1_Type_SMEN(state, tracer, s_42_1);
        // D s_42_3: write-var ga#16697 <= s_42_2
        fn_state.ga_16697 = s_42_2;
        // D s_42_4: read-var ga#16697:u8
        let s_42_4: u8 = fn_state.ga_16697;
        // C s_42_5: const #0s : i
        let s_42_5: i128 = 0;
        // D s_42_6: cast zx s_42_4 -> bv
        let s_42_6: Bits = Bits::new(s_42_4 as u128, 2u16);
        // C s_42_7: const #1s : i64
        let s_42_7: i64 = 1;
        // C s_42_8: cast zx s_42_7 -> i
        let s_42_8: i128 = (i128::try_from(s_42_7).unwrap());
        // C s_42_9: const #0s : i
        let s_42_9: i128 = 0;
        // C s_42_10: add s_42_9 s_42_8
        let s_42_10: i128 = (s_42_9 + s_42_8);
        // D s_42_11: bit-extract s_42_6 s_42_5 s_42_10
        let s_42_11: Bits = (Bits::new(
            ((s_42_6) >> (s_42_5)).value(),
            u16::try_from(s_42_10).unwrap(),
        ));
        // D s_42_12: cast reint s_42_11 -> u8
        let s_42_12: bool = ((s_42_11.value()) != 0);
        // D s_42_13: cast zx s_42_12 -> bv
        let s_42_13: Bits = Bits::new(s_42_12 as u128, 1u16);
        // C s_42_14: const #0u : u8
        let s_42_14: bool = false;
        // C s_42_15: cast zx s_42_14 -> bv
        let s_42_15: Bits = Bits::new(s_42_14 as u128, 1u16);
        // D s_42_16: cmp-eq s_42_13 s_42_15
        let s_42_16: bool = ((s_42_13) == (s_42_15));
        // D s_42_17: not s_42_16
        let s_42_17: bool = !s_42_16;
        // N s_42_18: branch s_42_17 b48 b43
        if s_42_17 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var disabled <= s_43_0
        fn_state.disabled = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var disabled:u8
        let s_44_0: bool = fn_state.disabled;
        // N s_44_1: branch s_44_0 b47 b45
        if s_44_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u32
        let s_47_0: u32 = 0;
        // C s_47_1: const #440u : u32
        let s_47_1: u32 = 440;
        // D s_47_2: read-reg s_47_1:u8
        let s_47_2: u8 = {
            let value = state.read_register::<u8>(s_47_1 as isize);
            tracer.read_register(s_47_1 as isize, value);
            value
        };
        // D s_47_3: call SMEAccessTrap(s_47_0, s_47_2)
        let s_47_3: () = SMEAccessTrap(state, tracer, s_47_0, s_47_2);
        // N s_47_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var ga#16697:u8
        let s_48_0: u8 = fn_state.ga_16697;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 2u16);
        // C s_48_2: const #1u : u8
        let s_48_2: u8 = 1;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 2u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: not s_48_4
        let s_48_5: bool = !s_48_4;
        // N s_48_6: branch s_48_5 b50 b49
        if s_48_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #16975u : u32
        let s_49_0: u32 = 16975;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: cast zx s_49_1 -> bv
        let s_49_2: Bits = Bits::new(s_49_1 as u128, 2u16);
        // C s_49_3: const #448u : u32
        let s_49_3: u32 = 448;
        // D s_49_4: read-reg s_49_3:u8
        let s_49_4: u8 = {
            let value = state.read_register::<u8>(s_49_3 as isize);
            tracer.read_register(s_49_3 as isize, value);
            value
        };
        // D s_49_5: cast zx s_49_4 -> bv
        let s_49_5: Bits = Bits::new(s_49_4 as u128, 2u16);
        // D s_49_6: cmp-eq s_49_2 s_49_5
        let s_49_6: bool = ((s_49_2) == (s_49_5));
        // D s_49_7: write-var disabled <= s_49_6
        fn_state.disabled = s_49_6;
        // N s_49_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var disabled <= s_50_0
        fn_state.disabled = s_50_0;
        // N s_50_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call IsInHost(s_51_0)
        let s_51_1: bool = IsInHost(state, tracer, s_51_0);
        // S s_51_2: not s_51_1
        let s_51_2: bool = !s_51_1;
        // D s_51_3: write-var gs#21288 <= s_51_2
        fn_state.gs_21288 = s_51_2;
        // N s_51_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#21287 <= s_52_0
        fn_state.gs_21287 = s_52_0;
        // N s_52_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
