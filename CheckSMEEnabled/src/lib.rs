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
use u_get_CPACR_EL1_Type_FPEN::*;
use u_get_CPTR_EL2_Type_TFP::*;
use u_get_HCR_EL2_Type_E2H::*;
use IsInHost::*;
use SMEAccessTrap::*;
use u_get_CPTR_EL3_Type_ESM::*;
use u_get_CPTR_EL3_Type_TFP::*;
use u_get_CPTR_EL2_Type_FPEN::*;
use u_get_CPTR_EL2_Type_SMEN::*;
use AArch64_AdvSIMDFPAccessTrap::*;
use u_get_CPTR_EL2_Type_TSM::*;
use u_get_CPACR_EL1_Type_SMEN::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use HaveVirtHostExt::*;
use common::*;
pub fn CheckSMEEnabled<T: Tracer>(state: &mut State, tracer: &T, gs_21208: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_21213: bool,
        gs_21209: bool,
        gs_21210: bool,
        gs_21218: bool,
        gs_21225: bool,
        gs_21212: bool,
        gs_21232: bool,
        gs_21211: bool,
        ga_16628: u8,
        disabled: bool,
        ga_16615: u8,
        ga_16633: u8,
        ga_16612: u8,
        gs_21208: (),
    }
    let fn_state = FunctionState {
        gs_21208,
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
        // N s_0_7: branch s_0_6 b77 b1
        if s_0_6 {
            return block_77(state, tracer, fn_state);
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
        // D s_1_7: write-var gs#21209 <= s_1_6
        fn_state.gs_21209 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#21209:u8
        let s_2_0: bool = fn_state.gs_21209;
        // N s_2_1: branch s_2_0 b76 b3
        if s_2_0 {
            return block_76(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#21210 <= s_3_0
        fn_state.gs_21210 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#21210:u8
        let s_4_0: bool = fn_state.gs_21210;
        // N s_4_1: branch s_4_0 b59 b5
        if s_4_0 {
            return block_59(state, tracer, fn_state);
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
        // N s_6_7: branch s_6_6 b58 b7
        if s_6_6 {
            return block_58(state, tracer, fn_state);
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
        // N s_7_7: branch s_7_6 b57 b8
        if s_7_6 {
            return block_57(state, tracer, fn_state);
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
        // D s_8_7: write-var gs#21211 <= s_8_6
        fn_state.gs_21211 = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#21211:u8
        let s_9_0: bool = fn_state.gs_21211;
        // D s_9_1: write-var gs#21212 <= s_9_0
        fn_state.gs_21212 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#21212:u8
        let s_10_0: bool = fn_state.gs_21212;
        // N s_10_1: branch s_10_0 b56 b11
        if s_10_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#21213 <= s_11_0
        fn_state.gs_21213 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#21213:u8
        let s_12_0: bool = fn_state.gs_21213;
        // N s_12_1: branch s_12_0 b22 b13
        if s_12_0 {
            return block_22(state, tracer, fn_state);
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
        // N s_16_7: branch s_16_6 b21 b17
        if s_16_6 {
            return block_21(state, tracer, fn_state);
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
        // C s_18_0: const #16840u : u32
        let s_18_0: u32 = 16840;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_CPTR_EL3_Type_TFP(s_18_1)
        let s_18_2: bool = u_get_CPTR_EL3_Type_TFP(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // D s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // N s_18_7: branch s_18_6 b20 b19
        if s_18_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: return
        return;
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
        // D s_20_2: call AArch64_AdvSIMDFPAccessTrap(s_20_1)
        let s_20_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_20_1);
        // N s_20_3: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u32
        let s_21_0: u32 = 0;
        // C s_21_1: const #424u : u32
        let s_21_1: u32 = 424;
        // D s_21_2: read-reg s_21_1:u8
        let s_21_2: u8 = {
            let value = state.read_register::<u8>(s_21_1 as isize);
            tracer.read_register(s_21_1 as isize, value);
            value
        };
        // D s_21_3: call SMEAccessTrap(s_21_0, s_21_2)
        let s_21_3: () = SMEAccessTrap(state, tracer, s_21_0, s_21_2);
        // N s_21_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call HaveVirtHostExt(s_22_0)
        let s_22_1: bool = HaveVirtHostExt(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b55 b23
        if s_22_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#21218 <= s_23_0
        fn_state.gs_21218 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#21218:u8
        let s_24_0: bool = fn_state.gs_21218;
        // N s_24_1: branch s_24_0 b32 b25
        if s_24_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #11088u : u32
        let s_25_0: u32 = 11088;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_CPTR_EL2_Type_TSM(s_25_1)
        let s_25_2: bool = u_get_CPTR_EL2_Type_TSM(state, tracer, s_25_1);
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // D s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // N s_25_7: branch s_25_6 b31 b26
        if s_25_6 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #11088u : u32
        let s_27_0: u32 = 11088;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_CPTR_EL2_Type_TFP(s_27_1)
        let s_27_2: bool = u_get_CPTR_EL2_Type_TFP(state, tracer, s_27_1);
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // C s_27_4: const #1u : u8
        let s_27_4: bool = true;
        // C s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // D s_27_6: cmp-eq s_27_3 s_27_5
        let s_27_6: bool = ((s_27_3) == (s_27_5));
        // N s_27_7: branch s_27_6 b30 b28
        if s_27_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #432u : u32
        let s_30_0: u32 = 432;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call AArch64_AdvSIMDFPAccessTrap(s_30_1)
        let s_30_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_30_1);
        // N s_30_3: jump b29
        return block_29(state, tracer, fn_state);
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
        // N s_31_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #11088u : u32
        let s_32_0: u32 = 11088;
        // D s_32_1: read-reg s_32_0:struct
        let s_32_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call _get_CPTR_EL2_Type_SMEN(s_32_1)
        let s_32_2: u8 = u_get_CPTR_EL2_Type_SMEN(state, tracer, s_32_1);
        // D s_32_3: write-var ga#16628 <= s_32_2
        fn_state.ga_16628 = s_32_2;
        // D s_32_4: read-var ga#16628:u8
        let s_32_4: u8 = fn_state.ga_16628;
        // C s_32_5: const #0s : i
        let s_32_5: i128 = 0;
        // D s_32_6: cast zx s_32_4 -> bv
        let s_32_6: Bits = Bits::new(s_32_4 as u128, 2u16);
        // C s_32_7: const #1s : i64
        let s_32_7: i64 = 1;
        // C s_32_8: cast zx s_32_7 -> i
        let s_32_8: i128 = (i128::try_from(s_32_7).unwrap());
        // C s_32_9: const #0s : i
        let s_32_9: i128 = 0;
        // C s_32_10: add s_32_9 s_32_8
        let s_32_10: i128 = (s_32_9 + s_32_8);
        // D s_32_11: bit-extract s_32_6 s_32_5 s_32_10
        let s_32_11: Bits = (Bits::new(
            ((s_32_6) >> (s_32_5)).value(),
            u16::try_from(s_32_10).unwrap(),
        ));
        // D s_32_12: cast reint s_32_11 -> u8
        let s_32_12: bool = ((s_32_11.value()) != 0);
        // D s_32_13: cast zx s_32_12 -> bv
        let s_32_13: Bits = Bits::new(s_32_12 as u128, 1u16);
        // C s_32_14: const #0u : u8
        let s_32_14: bool = false;
        // C s_32_15: cast zx s_32_14 -> bv
        let s_32_15: Bits = Bits::new(s_32_14 as u128, 1u16);
        // D s_32_16: cmp-eq s_32_13 s_32_15
        let s_32_16: bool = ((s_32_13) == (s_32_15));
        // D s_32_17: not s_32_16
        let s_32_17: bool = !s_32_16;
        // N s_32_18: branch s_32_17 b49 b33
        if s_32_17 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var disabled <= s_33_0
        fn_state.disabled = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var disabled:u8
        let s_34_0: bool = fn_state.disabled;
        // N s_34_1: branch s_34_0 b48 b35
        if s_34_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #11088u : u32
        let s_36_0: u32 = 11088;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_CPTR_EL2_Type_FPEN(s_36_1)
        let s_36_2: u8 = u_get_CPTR_EL2_Type_FPEN(state, tracer, s_36_1);
        // D s_36_3: write-var ga#16633 <= s_36_2
        fn_state.ga_16633 = s_36_2;
        // D s_36_4: read-var ga#16633:u8
        let s_36_4: u8 = fn_state.ga_16633;
        // C s_36_5: const #0s : i
        let s_36_5: i128 = 0;
        // D s_36_6: cast zx s_36_4 -> bv
        let s_36_6: Bits = Bits::new(s_36_4 as u128, 2u16);
        // C s_36_7: const #1s : i64
        let s_36_7: i64 = 1;
        // C s_36_8: cast zx s_36_7 -> i
        let s_36_8: i128 = (i128::try_from(s_36_7).unwrap());
        // C s_36_9: const #0s : i
        let s_36_9: i128 = 0;
        // C s_36_10: add s_36_9 s_36_8
        let s_36_10: i128 = (s_36_9 + s_36_8);
        // D s_36_11: bit-extract s_36_6 s_36_5 s_36_10
        let s_36_11: Bits = (Bits::new(
            ((s_36_6) >> (s_36_5)).value(),
            u16::try_from(s_36_10).unwrap(),
        ));
        // D s_36_12: cast reint s_36_11 -> u8
        let s_36_12: bool = ((s_36_11.value()) != 0);
        // D s_36_13: cast zx s_36_12 -> bv
        let s_36_13: Bits = Bits::new(s_36_12 as u128, 1u16);
        // C s_36_14: const #0u : u8
        let s_36_14: bool = false;
        // C s_36_15: cast zx s_36_14 -> bv
        let s_36_15: Bits = Bits::new(s_36_14 as u128, 1u16);
        // D s_36_16: cmp-eq s_36_13 s_36_15
        let s_36_16: bool = ((s_36_13) == (s_36_15));
        // D s_36_17: not s_36_16
        let s_36_17: bool = !s_36_16;
        // N s_36_18: branch s_36_17 b42 b37
        if s_36_17 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var disabled <= s_37_0
        fn_state.disabled = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var disabled:u8
        let s_38_0: bool = fn_state.disabled;
        // N s_38_1: branch s_38_0 b41 b39
        if s_38_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_40_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #432u : u32
        let s_41_0: u32 = 432;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: u8 = {
            let value = state.read_register::<u8>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call AArch64_AdvSIMDFPAccessTrap(s_41_1)
        let s_41_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_41_1);
        // N s_41_3: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var ga#16633:u8
        let s_42_0: u8 = fn_state.ga_16633;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 2u16);
        // C s_42_2: const #1u : u8
        let s_42_2: u8 = 1;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 2u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: not s_42_4
        let s_42_5: bool = !s_42_4;
        // N s_42_6: branch s_42_5 b47 b43
        if s_42_5 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #16975u : u32
        let s_43_0: u32 = 16975;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: cast zx s_43_1 -> bv
        let s_43_2: Bits = Bits::new(s_43_1 as u128, 2u16);
        // C s_43_3: const #448u : u32
        let s_43_3: u32 = 448;
        // D s_43_4: read-reg s_43_3:u8
        let s_43_4: u8 = {
            let value = state.read_register::<u8>(s_43_3 as isize);
            tracer.read_register(s_43_3 as isize, value);
            value
        };
        // D s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 2u16);
        // D s_43_6: cmp-eq s_43_2 s_43_5
        let s_43_6: bool = ((s_43_2) == (s_43_5));
        // N s_43_7: branch s_43_6 b46 b44
        if s_43_6 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#21232 <= s_44_0
        fn_state.gs_21232 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#21232:u8
        let s_45_0: bool = fn_state.gs_21232;
        // D s_45_1: write-var disabled <= s_45_0
        fn_state.disabled = s_45_0;
        // N s_45_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #102552u : u32
        let s_46_0: u32 = 102552;
        // D s_46_1: read-reg s_46_0:struct
        let s_46_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call _get_HCR_EL2_Type_TGE(s_46_1)
        let s_46_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_46_1);
        // D s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // C s_46_4: const #1u : u8
        let s_46_4: bool = true;
        // C s_46_5: cast zx s_46_4 -> bv
        let s_46_5: Bits = Bits::new(s_46_4 as u128, 1u16);
        // D s_46_6: cmp-eq s_46_3 s_46_5
        let s_46_6: bool = ((s_46_3) == (s_46_5));
        // D s_46_7: write-var gs#21232 <= s_46_6
        fn_state.gs_21232 = s_46_6;
        // N s_46_8: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var disabled <= s_47_0
        fn_state.disabled = s_47_0;
        // N s_47_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u32
        let s_48_0: u32 = 0;
        // C s_48_1: const #432u : u32
        let s_48_1: u32 = 432;
        // D s_48_2: read-reg s_48_1:u8
        let s_48_2: u8 = {
            let value = state.read_register::<u8>(s_48_1 as isize);
            tracer.read_register(s_48_1 as isize, value);
            value
        };
        // D s_48_3: call SMEAccessTrap(s_48_0, s_48_2)
        let s_48_3: () = SMEAccessTrap(state, tracer, s_48_0, s_48_2);
        // N s_48_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var ga#16628:u8
        let s_49_0: u8 = fn_state.ga_16628;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 2u16);
        // C s_49_2: const #1u : u8
        let s_49_2: u8 = 1;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 2u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: not s_49_4
        let s_49_5: bool = !s_49_4;
        // N s_49_6: branch s_49_5 b54 b50
        if s_49_5 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #16975u : u32
        let s_50_0: u32 = 16975;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: cast zx s_50_1 -> bv
        let s_50_2: Bits = Bits::new(s_50_1 as u128, 2u16);
        // C s_50_3: const #448u : u32
        let s_50_3: u32 = 448;
        // D s_50_4: read-reg s_50_3:u8
        let s_50_4: u8 = {
            let value = state.read_register::<u8>(s_50_3 as isize);
            tracer.read_register(s_50_3 as isize, value);
            value
        };
        // D s_50_5: cast zx s_50_4 -> bv
        let s_50_5: Bits = Bits::new(s_50_4 as u128, 2u16);
        // D s_50_6: cmp-eq s_50_2 s_50_5
        let s_50_6: bool = ((s_50_2) == (s_50_5));
        // N s_50_7: branch s_50_6 b53 b51
        if s_50_6 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#21225 <= s_51_0
        fn_state.gs_21225 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#21225:u8
        let s_52_0: bool = fn_state.gs_21225;
        // D s_52_1: write-var disabled <= s_52_0
        fn_state.disabled = s_52_0;
        // N s_52_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #102552u : u32
        let s_53_0: u32 = 102552;
        // D s_53_1: read-reg s_53_0:struct
        let s_53_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: call _get_HCR_EL2_Type_TGE(s_53_1)
        let s_53_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_53_1);
        // D s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // C s_53_4: const #1u : u8
        let s_53_4: bool = true;
        // C s_53_5: cast zx s_53_4 -> bv
        let s_53_5: Bits = Bits::new(s_53_4 as u128, 1u16);
        // D s_53_6: cmp-eq s_53_3 s_53_5
        let s_53_6: bool = ((s_53_3) == (s_53_5));
        // D s_53_7: write-var gs#21225 <= s_53_6
        fn_state.gs_21225 = s_53_6;
        // N s_53_8: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var disabled <= s_54_0
        fn_state.disabled = s_54_0;
        // N s_54_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #102552u : u32
        let s_55_0: u32 = 102552;
        // D s_55_1: read-reg s_55_0:struct
        let s_55_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call _get_HCR_EL2_Type_E2H(s_55_1)
        let s_55_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_55_1);
        // D s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // C s_55_4: const #1u : u8
        let s_55_4: bool = true;
        // C s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 1u16);
        // D s_55_6: cmp-eq s_55_3 s_55_5
        let s_55_6: bool = ((s_55_3) == (s_55_5));
        // D s_55_7: write-var gs#21218 <= s_55_6
        fn_state.gs_21218 = s_55_6;
        // N s_55_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call EL2Enabled(s_56_0)
        let s_56_1: bool = EL2Enabled(state, tracer, s_56_0);
        // D s_56_2: write-var gs#21213 <= s_56_1
        fn_state.gs_21213 = s_56_1;
        // N s_56_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#21211 <= s_57_0
        fn_state.gs_21211 = s_57_0;
        // N s_57_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#21212 <= s_58_0
        fn_state.gs_21212 = s_58_0;
        // N s_58_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #12088u : u32
        let s_59_0: u32 = 12088;
        // D s_59_1: read-reg s_59_0:struct
        let s_59_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call _get_CPACR_EL1_Type_SMEN(s_59_1)
        let s_59_2: u8 = u_get_CPACR_EL1_Type_SMEN(state, tracer, s_59_1);
        // D s_59_3: write-var ga#16612 <= s_59_2
        fn_state.ga_16612 = s_59_2;
        // D s_59_4: read-var ga#16612:u8
        let s_59_4: u8 = fn_state.ga_16612;
        // C s_59_5: const #0s : i
        let s_59_5: i128 = 0;
        // D s_59_6: cast zx s_59_4 -> bv
        let s_59_6: Bits = Bits::new(s_59_4 as u128, 2u16);
        // C s_59_7: const #1s : i64
        let s_59_7: i64 = 1;
        // C s_59_8: cast zx s_59_7 -> i
        let s_59_8: i128 = (i128::try_from(s_59_7).unwrap());
        // C s_59_9: const #0s : i
        let s_59_9: i128 = 0;
        // C s_59_10: add s_59_9 s_59_8
        let s_59_10: i128 = (s_59_9 + s_59_8);
        // D s_59_11: bit-extract s_59_6 s_59_5 s_59_10
        let s_59_11: Bits = (Bits::new(
            ((s_59_6) >> (s_59_5)).value(),
            u16::try_from(s_59_10).unwrap(),
        ));
        // D s_59_12: cast reint s_59_11 -> u8
        let s_59_12: bool = ((s_59_11.value()) != 0);
        // D s_59_13: cast zx s_59_12 -> bv
        let s_59_13: Bits = Bits::new(s_59_12 as u128, 1u16);
        // C s_59_14: const #0u : u8
        let s_59_14: bool = false;
        // C s_59_15: cast zx s_59_14 -> bv
        let s_59_15: Bits = Bits::new(s_59_14 as u128, 1u16);
        // D s_59_16: cmp-eq s_59_13 s_59_15
        let s_59_16: bool = ((s_59_13) == (s_59_15));
        // D s_59_17: not s_59_16
        let s_59_17: bool = !s_59_16;
        // N s_59_18: branch s_59_17 b73 b60
        if s_59_17 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var disabled <= s_60_0
        fn_state.disabled = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var disabled:u8
        let s_61_0: bool = fn_state.disabled;
        // N s_61_1: branch s_61_0 b72 b62
        if s_61_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_62_0: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #12088u : u32
        let s_63_0: u32 = 12088;
        // D s_63_1: read-reg s_63_0:struct
        let s_63_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call _get_CPACR_EL1_Type_FPEN(s_63_1)
        let s_63_2: u8 = u_get_CPACR_EL1_Type_FPEN(state, tracer, s_63_1);
        // D s_63_3: write-var ga#16615 <= s_63_2
        fn_state.ga_16615 = s_63_2;
        // D s_63_4: read-var ga#16615:u8
        let s_63_4: u8 = fn_state.ga_16615;
        // C s_63_5: const #0s : i
        let s_63_5: i128 = 0;
        // D s_63_6: cast zx s_63_4 -> bv
        let s_63_6: Bits = Bits::new(s_63_4 as u128, 2u16);
        // C s_63_7: const #1s : i64
        let s_63_7: i64 = 1;
        // C s_63_8: cast zx s_63_7 -> i
        let s_63_8: i128 = (i128::try_from(s_63_7).unwrap());
        // C s_63_9: const #0s : i
        let s_63_9: i128 = 0;
        // C s_63_10: add s_63_9 s_63_8
        let s_63_10: i128 = (s_63_9 + s_63_8);
        // D s_63_11: bit-extract s_63_6 s_63_5 s_63_10
        let s_63_11: Bits = (Bits::new(
            ((s_63_6) >> (s_63_5)).value(),
            u16::try_from(s_63_10).unwrap(),
        ));
        // D s_63_12: cast reint s_63_11 -> u8
        let s_63_12: bool = ((s_63_11.value()) != 0);
        // D s_63_13: cast zx s_63_12 -> bv
        let s_63_13: Bits = Bits::new(s_63_12 as u128, 1u16);
        // C s_63_14: const #0u : u8
        let s_63_14: bool = false;
        // C s_63_15: cast zx s_63_14 -> bv
        let s_63_15: Bits = Bits::new(s_63_14 as u128, 1u16);
        // D s_63_16: cmp-eq s_63_13 s_63_15
        let s_63_16: bool = ((s_63_13) == (s_63_15));
        // D s_63_17: not s_63_16
        let s_63_17: bool = !s_63_16;
        // N s_63_18: branch s_63_17 b69 b64
        if s_63_17 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var disabled <= s_64_0
        fn_state.disabled = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var disabled:u8
        let s_65_0: bool = fn_state.disabled;
        // N s_65_1: branch s_65_0 b68 b66
        if s_65_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_66_0: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #440u : u32
        let s_68_0: u32 = 440;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call AArch64_AdvSIMDFPAccessTrap(s_68_1)
        let s_68_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_68_1);
        // N s_68_3: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var ga#16615:u8
        let s_69_0: u8 = fn_state.ga_16615;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 2u16);
        // C s_69_2: const #1u : u8
        let s_69_2: u8 = 1;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 2u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: not s_69_4
        let s_69_5: bool = !s_69_4;
        // N s_69_6: branch s_69_5 b71 b70
        if s_69_5 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #16975u : u32
        let s_70_0: u32 = 16975;
        // D s_70_1: read-reg s_70_0:u8
        let s_70_1: u8 = {
            let value = state.read_register::<u8>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: cast zx s_70_1 -> bv
        let s_70_2: Bits = Bits::new(s_70_1 as u128, 2u16);
        // C s_70_3: const #448u : u32
        let s_70_3: u32 = 448;
        // D s_70_4: read-reg s_70_3:u8
        let s_70_4: u8 = {
            let value = state.read_register::<u8>(s_70_3 as isize);
            tracer.read_register(s_70_3 as isize, value);
            value
        };
        // D s_70_5: cast zx s_70_4 -> bv
        let s_70_5: Bits = Bits::new(s_70_4 as u128, 2u16);
        // D s_70_6: cmp-eq s_70_2 s_70_5
        let s_70_6: bool = ((s_70_2) == (s_70_5));
        // D s_70_7: write-var disabled <= s_70_6
        fn_state.disabled = s_70_6;
        // N s_70_8: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var disabled <= s_71_0
        fn_state.disabled = s_71_0;
        // N s_71_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u32
        let s_72_0: u32 = 0;
        // C s_72_1: const #440u : u32
        let s_72_1: u32 = 440;
        // D s_72_2: read-reg s_72_1:u8
        let s_72_2: u8 = {
            let value = state.read_register::<u8>(s_72_1 as isize);
            tracer.read_register(s_72_1 as isize, value);
            value
        };
        // D s_72_3: call SMEAccessTrap(s_72_0, s_72_2)
        let s_72_3: () = SMEAccessTrap(state, tracer, s_72_0, s_72_2);
        // N s_72_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var ga#16612:u8
        let s_73_0: u8 = fn_state.ga_16612;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 2u16);
        // C s_73_2: const #1u : u8
        let s_73_2: u8 = 1;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 2u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: not s_73_4
        let s_73_5: bool = !s_73_4;
        // N s_73_6: branch s_73_5 b75 b74
        if s_73_5 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #16975u : u32
        let s_74_0: u32 = 16975;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: cast zx s_74_1 -> bv
        let s_74_2: Bits = Bits::new(s_74_1 as u128, 2u16);
        // C s_74_3: const #448u : u32
        let s_74_3: u32 = 448;
        // D s_74_4: read-reg s_74_3:u8
        let s_74_4: u8 = {
            let value = state.read_register::<u8>(s_74_3 as isize);
            tracer.read_register(s_74_3 as isize, value);
            value
        };
        // D s_74_5: cast zx s_74_4 -> bv
        let s_74_5: Bits = Bits::new(s_74_4 as u128, 2u16);
        // D s_74_6: cmp-eq s_74_2 s_74_5
        let s_74_6: bool = ((s_74_2) == (s_74_5));
        // D s_74_7: write-var disabled <= s_74_6
        fn_state.disabled = s_74_6;
        // N s_74_8: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var disabled <= s_75_0
        fn_state.disabled = s_75_0;
        // N s_75_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call IsInHost(s_76_0)
        let s_76_1: bool = IsInHost(state, tracer, s_76_0);
        // S s_76_2: not s_76_1
        let s_76_2: bool = !s_76_1;
        // D s_76_3: write-var gs#21210 <= s_76_2
        fn_state.gs_21210 = s_76_2;
        // N s_76_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #1u : u8
        let s_77_0: bool = true;
        // D s_77_1: write-var gs#21209 <= s_77_0
        fn_state.gs_21209 = s_77_0;
        // N s_77_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
