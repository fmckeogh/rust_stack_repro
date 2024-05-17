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
use u_get_CPACR_EL1_Type_ZEN::*;
use IsInHost::*;
use EL3SDDUndefPriority::*;
use u_get_CPTR_EL2_Type_ZEN::*;
use u_get_CPTR_EL2_Type_TZ::*;
use u_get_CPTR_EL2_Type_FPEN::*;
use u_get_CPTR_EL3_Type_TFP::*;
use SVEAccessTrap::*;
use u_get_CPTR_EL3_Type_EZ::*;
use AArch64_AdvSIMDFPAccessTrap::*;
use HaveSVE::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EL3SDDUndef::*;
use HaveVirtHostExt::*;
use common::*;
pub fn CheckOriginalSVEEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_21125: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_21132: bool,
        gs_21153: bool,
        ga_16561: u8,
        gs_21131: bool,
        gs_21129: bool,
        gs_21134: bool,
        ga_16545: u8,
        gs_21139: bool,
        ga_16566: u8,
        gs_21130: bool,
        disabled: bool,
        gs_21146: bool,
        gs_21133: bool,
        gs_21127: bool,
        gs_21128: bool,
        ga_16548: u8,
        gs_21125: (),
    }
    let fn_state = FunctionState {
        gs_21125,
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
        // S s_0_1: call HaveSVE(s_0_0)
        let s_0_1: bool = HaveSVE(state, tracer, s_0_0);
        // N s_0_2: assert s_0_1
        let s_0_2: () = assert!(s_0_1);
        // C s_0_3: const #424u : u32
        let s_0_3: u32 = 424;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // C s_0_5: const #2u : u8
        let s_0_5: u8 = 2;
        // D s_0_6: cmp-lt s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) < (s_0_5));
        // N s_0_7: branch s_0_6 b89 b1
        if s_0_6 {
            return block_89(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#21128 <= s_1_0
        fn_state.gs_21128 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#21128:u8
        let s_2_0: bool = fn_state.gs_21128;
        // N s_2_1: branch s_2_0 b88 b3
        if s_2_0 {
            return block_88(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#21129 <= s_3_0
        fn_state.gs_21129 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#21129:u8
        let s_4_0: bool = fn_state.gs_21129;
        // N s_4_1: branch s_4_0 b87 b5
        if s_4_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16975u : u32
        let s_5_0: u32 = 16975;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 2u16);
        // C s_5_3: const #448u : u32
        let s_5_3: u32 = 448;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_2 s_5_5
        let s_5_6: bool = ((s_5_2) == (s_5_5));
        // N s_5_7: branch s_5_6 b86 b6
        if s_5_6 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
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
        // C s_6_3: const #440u : u32
        let s_6_3: u32 = 440;
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
        // D s_6_7: write-var gs#21130 <= s_6_6
        fn_state.gs_21130 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#21130:u8
        let s_7_0: bool = fn_state.gs_21130;
        // N s_7_1: branch s_7_0 b85 b8
        if s_7_0 {
            return block_85(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#21131 <= s_8_0
        fn_state.gs_21131 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#21131:u8
        let s_9_0: bool = fn_state.gs_21131;
        // N s_9_1: branch s_9_0 b68 b10
        if s_9_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #16975u : u32
        let s_11_0: u32 = 16975;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 2u16);
        // C s_11_3: const #448u : u32
        let s_11_3: u32 = 448;
        // D s_11_4: read-reg s_11_3:u8
        let s_11_4: u8 = {
            let value = state.read_register::<u8>(s_11_3 as isize);
            tracer.read_register(s_11_3 as isize, value);
            value
        };
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 2u16);
        // D s_11_6: cmp-eq s_11_2 s_11_5
        let s_11_6: bool = ((s_11_2) == (s_11_5));
        // N s_11_7: branch s_11_6 b67 b12
        if s_11_6 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #16975u : u32
        let s_12_0: u32 = 16975;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 2u16);
        // C s_12_3: const #440u : u32
        let s_12_3: u32 = 440;
        // D s_12_4: read-reg s_12_3:u8
        let s_12_4: u8 = {
            let value = state.read_register::<u8>(s_12_3 as isize);
            tracer.read_register(s_12_3 as isize, value);
            value
        };
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 2u16);
        // D s_12_6: cmp-eq s_12_2 s_12_5
        let s_12_6: bool = ((s_12_2) == (s_12_5));
        // N s_12_7: branch s_12_6 b66 b13
        if s_12_6 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #16975u : u32
        let s_13_0: u32 = 16975;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 2u16);
        // C s_13_3: const #432u : u32
        let s_13_3: u32 = 432;
        // D s_13_4: read-reg s_13_3:u8
        let s_13_4: u8 = {
            let value = state.read_register::<u8>(s_13_3 as isize);
            tracer.read_register(s_13_3 as isize, value);
            value
        };
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 2u16);
        // D s_13_6: cmp-eq s_13_2 s_13_5
        let s_13_6: bool = ((s_13_2) == (s_13_5));
        // D s_13_7: write-var gs#21132 <= s_13_6
        fn_state.gs_21132 = s_13_6;
        // N s_13_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#21132:u8
        let s_14_0: bool = fn_state.gs_21132;
        // D s_14_1: write-var gs#21133 <= s_14_0
        fn_state.gs_21133 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#21133:u8
        let s_15_0: bool = fn_state.gs_21133;
        // N s_15_1: branch s_15_0 b65 b16
        if s_15_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#21134 <= s_16_0
        fn_state.gs_21134 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#21134:u8
        let s_17_0: bool = fn_state.gs_21134;
        // N s_17_1: branch s_17_0 b31 b18
        if s_17_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
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
        // C s_19_2: const #2u : u8
        let s_19_2: u8 = 2;
        // D s_19_3: cmp-lt s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) < (s_19_2));
        // N s_19_4: branch s_19_3 b21 b20
        if s_19_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #16840u : u32
        let s_21_0: u32 = 16840;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_CPTR_EL3_Type_EZ(s_21_1)
        let s_21_2: bool = u_get_CPTR_EL3_Type_EZ(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #0u : u8
        let s_21_4: bool = false;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // N s_21_7: branch s_21_6 b28 b22
        if s_21_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #16840u : u32
        let s_23_0: u32 = 16840;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_CPTR_EL3_Type_TFP(s_23_1)
        let s_23_2: bool = u_get_CPTR_EL3_Type_TFP(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // N s_23_7: branch s_23_6 b25 b24
        if s_23_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call EL3SDDUndef(s_25_0)
        let s_25_1: bool = EL3SDDUndef(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b27 b26
        if s_25_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #424u : u32
        let s_26_0: u32 = 424;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call AArch64_AdvSIMDFPAccessTrap(s_26_1)
        let s_26_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_26_1);
        // N s_26_3: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EL3SDDUndef(s_28_0)
        let s_28_1: bool = EL3SDDUndef(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b30 b29
        if s_28_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #424u : u32
        let s_29_0: u32 = 424;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call SVEAccessTrap(s_29_1)
        let s_29_2: () = SVEAccessTrap(state, tracer, s_29_1);
        // N s_29_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call HaveVirtHostExt(s_31_0)
        let s_31_1: bool = HaveVirtHostExt(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b64 b32
        if s_31_1 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#21139 <= s_32_0
        fn_state.gs_21139 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#21139:u8
        let s_33_0: bool = fn_state.gs_21139;
        // N s_33_1: branch s_33_0 b41 b34
        if s_33_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #11088u : u32
        let s_34_0: u32 = 11088;
        // D s_34_1: read-reg s_34_0:struct
        let s_34_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call _get_CPTR_EL2_Type_TZ(s_34_1)
        let s_34_2: bool = u_get_CPTR_EL2_Type_TZ(state, tracer, s_34_1);
        // D s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // C s_34_4: const #1u : u8
        let s_34_4: bool = true;
        // C s_34_5: cast zx s_34_4 -> bv
        let s_34_5: Bits = Bits::new(s_34_4 as u128, 1u16);
        // D s_34_6: cmp-eq s_34_3 s_34_5
        let s_34_6: bool = ((s_34_3) == (s_34_5));
        // N s_34_7: branch s_34_6 b40 b35
        if s_34_6 {
            return block_40(state, tracer, fn_state);
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
        // D s_36_2: call _get_CPTR_EL2_Type_TFP(s_36_1)
        let s_36_2: bool = u_get_CPTR_EL2_Type_TFP(state, tracer, s_36_1);
        // D s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // D s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // N s_36_7: branch s_36_6 b39 b37
        if s_36_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #432u : u32
        let s_39_0: u32 = 432;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: u8 = {
            let value = state.read_register::<u8>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call AArch64_AdvSIMDFPAccessTrap(s_39_1)
        let s_39_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_39_1);
        // N s_39_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #432u : u32
        let s_40_0: u32 = 432;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: call SVEAccessTrap(s_40_1)
        let s_40_2: () = SVEAccessTrap(state, tracer, s_40_1);
        // N s_40_3: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #11088u : u32
        let s_41_0: u32 = 11088;
        // D s_41_1: read-reg s_41_0:struct
        let s_41_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call _get_CPTR_EL2_Type_ZEN(s_41_1)
        let s_41_2: u8 = u_get_CPTR_EL2_Type_ZEN(state, tracer, s_41_1);
        // D s_41_3: write-var ga#16561 <= s_41_2
        fn_state.ga_16561 = s_41_2;
        // D s_41_4: read-var ga#16561:u8
        let s_41_4: u8 = fn_state.ga_16561;
        // C s_41_5: const #0s : i
        let s_41_5: i128 = 0;
        // D s_41_6: cast zx s_41_4 -> bv
        let s_41_6: Bits = Bits::new(s_41_4 as u128, 2u16);
        // C s_41_7: const #1s : i64
        let s_41_7: i64 = 1;
        // C s_41_8: cast zx s_41_7 -> i
        let s_41_8: i128 = (i128::try_from(s_41_7).unwrap());
        // C s_41_9: const #0s : i
        let s_41_9: i128 = 0;
        // C s_41_10: add s_41_9 s_41_8
        let s_41_10: i128 = (s_41_9 + s_41_8);
        // D s_41_11: bit-extract s_41_6 s_41_5 s_41_10
        let s_41_11: Bits = (Bits::new(
            ((s_41_6) >> (s_41_5)).value(),
            u16::try_from(s_41_10).unwrap(),
        ));
        // D s_41_12: cast reint s_41_11 -> u8
        let s_41_12: bool = ((s_41_11.value()) != 0);
        // D s_41_13: cast zx s_41_12 -> bv
        let s_41_13: Bits = Bits::new(s_41_12 as u128, 1u16);
        // C s_41_14: const #0u : u8
        let s_41_14: bool = false;
        // C s_41_15: cast zx s_41_14 -> bv
        let s_41_15: Bits = Bits::new(s_41_14 as u128, 1u16);
        // D s_41_16: cmp-eq s_41_13 s_41_15
        let s_41_16: bool = ((s_41_13) == (s_41_15));
        // D s_41_17: not s_41_16
        let s_41_17: bool = !s_41_16;
        // N s_41_18: branch s_41_17 b58 b42
        if s_41_17 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var disabled <= s_42_0
        fn_state.disabled = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var disabled:u8
        let s_43_0: bool = fn_state.disabled;
        // N s_43_1: branch s_43_0 b57 b44
        if s_43_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #11088u : u32
        let s_45_0: u32 = 11088;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_CPTR_EL2_Type_FPEN(s_45_1)
        let s_45_2: u8 = u_get_CPTR_EL2_Type_FPEN(state, tracer, s_45_1);
        // D s_45_3: write-var ga#16566 <= s_45_2
        fn_state.ga_16566 = s_45_2;
        // D s_45_4: read-var ga#16566:u8
        let s_45_4: u8 = fn_state.ga_16566;
        // C s_45_5: const #0s : i
        let s_45_5: i128 = 0;
        // D s_45_6: cast zx s_45_4 -> bv
        let s_45_6: Bits = Bits::new(s_45_4 as u128, 2u16);
        // C s_45_7: const #1s : i64
        let s_45_7: i64 = 1;
        // C s_45_8: cast zx s_45_7 -> i
        let s_45_8: i128 = (i128::try_from(s_45_7).unwrap());
        // C s_45_9: const #0s : i
        let s_45_9: i128 = 0;
        // C s_45_10: add s_45_9 s_45_8
        let s_45_10: i128 = (s_45_9 + s_45_8);
        // D s_45_11: bit-extract s_45_6 s_45_5 s_45_10
        let s_45_11: Bits = (Bits::new(
            ((s_45_6) >> (s_45_5)).value(),
            u16::try_from(s_45_10).unwrap(),
        ));
        // D s_45_12: cast reint s_45_11 -> u8
        let s_45_12: bool = ((s_45_11.value()) != 0);
        // D s_45_13: cast zx s_45_12 -> bv
        let s_45_13: Bits = Bits::new(s_45_12 as u128, 1u16);
        // C s_45_14: const #0u : u8
        let s_45_14: bool = false;
        // C s_45_15: cast zx s_45_14 -> bv
        let s_45_15: Bits = Bits::new(s_45_14 as u128, 1u16);
        // D s_45_16: cmp-eq s_45_13 s_45_15
        let s_45_16: bool = ((s_45_13) == (s_45_15));
        // D s_45_17: not s_45_16
        let s_45_17: bool = !s_45_16;
        // N s_45_18: branch s_45_17 b51 b46
        if s_45_17 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var disabled <= s_46_0
        fn_state.disabled = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var disabled:u8
        let s_47_0: bool = fn_state.disabled;
        // N s_47_1: branch s_47_0 b50 b48
        if s_47_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_49_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #432u : u32
        let s_50_0: u32 = 432;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call AArch64_AdvSIMDFPAccessTrap(s_50_1)
        let s_50_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_50_1);
        // N s_50_3: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var ga#16566:u8
        let s_51_0: u8 = fn_state.ga_16566;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 2u16);
        // C s_51_2: const #1u : u8
        let s_51_2: u8 = 1;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 2u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: not s_51_4
        let s_51_5: bool = !s_51_4;
        // N s_51_6: branch s_51_5 b56 b52
        if s_51_5 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #16975u : u32
        let s_52_0: u32 = 16975;
        // D s_52_1: read-reg s_52_0:u8
        let s_52_1: u8 = {
            let value = state.read_register::<u8>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: cast zx s_52_1 -> bv
        let s_52_2: Bits = Bits::new(s_52_1 as u128, 2u16);
        // C s_52_3: const #448u : u32
        let s_52_3: u32 = 448;
        // D s_52_4: read-reg s_52_3:u8
        let s_52_4: u8 = {
            let value = state.read_register::<u8>(s_52_3 as isize);
            tracer.read_register(s_52_3 as isize, value);
            value
        };
        // D s_52_5: cast zx s_52_4 -> bv
        let s_52_5: Bits = Bits::new(s_52_4 as u128, 2u16);
        // D s_52_6: cmp-eq s_52_2 s_52_5
        let s_52_6: bool = ((s_52_2) == (s_52_5));
        // N s_52_7: branch s_52_6 b55 b53
        if s_52_6 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#21153 <= s_53_0
        fn_state.gs_21153 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#21153:u8
        let s_54_0: bool = fn_state.gs_21153;
        // D s_54_1: write-var disabled <= s_54_0
        fn_state.disabled = s_54_0;
        // N s_54_2: jump b47
        return block_47(state, tracer, fn_state);
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
        // D s_55_2: call _get_HCR_EL2_Type_TGE(s_55_1)
        let s_55_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_55_1);
        // D s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // C s_55_4: const #1u : u8
        let s_55_4: bool = true;
        // C s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 1u16);
        // D s_55_6: cmp-eq s_55_3 s_55_5
        let s_55_6: bool = ((s_55_3) == (s_55_5));
        // D s_55_7: write-var gs#21153 <= s_55_6
        fn_state.gs_21153 = s_55_6;
        // N s_55_8: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var disabled <= s_56_0
        fn_state.disabled = s_56_0;
        // N s_56_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #432u : u32
        let s_57_0: u32 = 432;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call SVEAccessTrap(s_57_1)
        let s_57_2: () = SVEAccessTrap(state, tracer, s_57_1);
        // N s_57_3: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var ga#16561:u8
        let s_58_0: u8 = fn_state.ga_16561;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 2u16);
        // C s_58_2: const #1u : u8
        let s_58_2: u8 = 1;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 2u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: not s_58_4
        let s_58_5: bool = !s_58_4;
        // N s_58_6: branch s_58_5 b63 b59
        if s_58_5 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #16975u : u32
        let s_59_0: u32 = 16975;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: u8 = {
            let value = state.read_register::<u8>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: cast zx s_59_1 -> bv
        let s_59_2: Bits = Bits::new(s_59_1 as u128, 2u16);
        // C s_59_3: const #448u : u32
        let s_59_3: u32 = 448;
        // D s_59_4: read-reg s_59_3:u8
        let s_59_4: u8 = {
            let value = state.read_register::<u8>(s_59_3 as isize);
            tracer.read_register(s_59_3 as isize, value);
            value
        };
        // D s_59_5: cast zx s_59_4 -> bv
        let s_59_5: Bits = Bits::new(s_59_4 as u128, 2u16);
        // D s_59_6: cmp-eq s_59_2 s_59_5
        let s_59_6: bool = ((s_59_2) == (s_59_5));
        // N s_59_7: branch s_59_6 b62 b60
        if s_59_6 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#21146 <= s_60_0
        fn_state.gs_21146 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#21146:u8
        let s_61_0: bool = fn_state.gs_21146;
        // D s_61_1: write-var disabled <= s_61_0
        fn_state.disabled = s_61_0;
        // N s_61_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #102552u : u32
        let s_62_0: u32 = 102552;
        // D s_62_1: read-reg s_62_0:struct
        let s_62_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call _get_HCR_EL2_Type_TGE(s_62_1)
        let s_62_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_62_1);
        // D s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // C s_62_4: const #1u : u8
        let s_62_4: bool = true;
        // C s_62_5: cast zx s_62_4 -> bv
        let s_62_5: Bits = Bits::new(s_62_4 as u128, 1u16);
        // D s_62_6: cmp-eq s_62_3 s_62_5
        let s_62_6: bool = ((s_62_3) == (s_62_5));
        // D s_62_7: write-var gs#21146 <= s_62_6
        fn_state.gs_21146 = s_62_6;
        // N s_62_8: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var disabled <= s_63_0
        fn_state.disabled = s_63_0;
        // N s_63_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #102552u : u32
        let s_64_0: u32 = 102552;
        // D s_64_1: read-reg s_64_0:struct
        let s_64_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call _get_HCR_EL2_Type_E2H(s_64_1)
        let s_64_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_64_1);
        // D s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // C s_64_4: const #1u : u8
        let s_64_4: bool = true;
        // C s_64_5: cast zx s_64_4 -> bv
        let s_64_5: Bits = Bits::new(s_64_4 as u128, 1u16);
        // D s_64_6: cmp-eq s_64_3 s_64_5
        let s_64_6: bool = ((s_64_3) == (s_64_5));
        // D s_64_7: write-var gs#21139 <= s_64_6
        fn_state.gs_21139 = s_64_6;
        // N s_64_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call EL2Enabled(s_65_0)
        let s_65_1: bool = EL2Enabled(state, tracer, s_65_0);
        // D s_65_2: write-var gs#21134 <= s_65_1
        fn_state.gs_21134 = s_65_1;
        // N s_65_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#21132 <= s_66_0
        fn_state.gs_21132 = s_66_0;
        // N s_66_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#21133 <= s_67_0
        fn_state.gs_21133 = s_67_0;
        // N s_67_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #12088u : u32
        let s_68_0: u32 = 12088;
        // D s_68_1: read-reg s_68_0:struct
        let s_68_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call _get_CPACR_EL1_Type_ZEN(s_68_1)
        let s_68_2: u8 = u_get_CPACR_EL1_Type_ZEN(state, tracer, s_68_1);
        // D s_68_3: write-var ga#16545 <= s_68_2
        fn_state.ga_16545 = s_68_2;
        // D s_68_4: read-var ga#16545:u8
        let s_68_4: u8 = fn_state.ga_16545;
        // C s_68_5: const #0s : i
        let s_68_5: i128 = 0;
        // D s_68_6: cast zx s_68_4 -> bv
        let s_68_6: Bits = Bits::new(s_68_4 as u128, 2u16);
        // C s_68_7: const #1s : i64
        let s_68_7: i64 = 1;
        // C s_68_8: cast zx s_68_7 -> i
        let s_68_8: i128 = (i128::try_from(s_68_7).unwrap());
        // C s_68_9: const #0s : i
        let s_68_9: i128 = 0;
        // C s_68_10: add s_68_9 s_68_8
        let s_68_10: i128 = (s_68_9 + s_68_8);
        // D s_68_11: bit-extract s_68_6 s_68_5 s_68_10
        let s_68_11: Bits = (Bits::new(
            ((s_68_6) >> (s_68_5)).value(),
            u16::try_from(s_68_10).unwrap(),
        ));
        // D s_68_12: cast reint s_68_11 -> u8
        let s_68_12: bool = ((s_68_11.value()) != 0);
        // D s_68_13: cast zx s_68_12 -> bv
        let s_68_13: Bits = Bits::new(s_68_12 as u128, 1u16);
        // C s_68_14: const #0u : u8
        let s_68_14: bool = false;
        // C s_68_15: cast zx s_68_14 -> bv
        let s_68_15: Bits = Bits::new(s_68_14 as u128, 1u16);
        // D s_68_16: cmp-eq s_68_13 s_68_15
        let s_68_16: bool = ((s_68_13) == (s_68_15));
        // D s_68_17: not s_68_16
        let s_68_17: bool = !s_68_16;
        // N s_68_18: branch s_68_17 b82 b69
        if s_68_17 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var disabled <= s_69_0
        fn_state.disabled = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var disabled:u8
        let s_70_0: bool = fn_state.disabled;
        // N s_70_1: branch s_70_0 b81 b71
        if s_70_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_71_0: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #12088u : u32
        let s_72_0: u32 = 12088;
        // D s_72_1: read-reg s_72_0:struct
        let s_72_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // D s_72_2: call _get_CPACR_EL1_Type_FPEN(s_72_1)
        let s_72_2: u8 = u_get_CPACR_EL1_Type_FPEN(state, tracer, s_72_1);
        // D s_72_3: write-var ga#16548 <= s_72_2
        fn_state.ga_16548 = s_72_2;
        // D s_72_4: read-var ga#16548:u8
        let s_72_4: u8 = fn_state.ga_16548;
        // C s_72_5: const #0s : i
        let s_72_5: i128 = 0;
        // D s_72_6: cast zx s_72_4 -> bv
        let s_72_6: Bits = Bits::new(s_72_4 as u128, 2u16);
        // C s_72_7: const #1s : i64
        let s_72_7: i64 = 1;
        // C s_72_8: cast zx s_72_7 -> i
        let s_72_8: i128 = (i128::try_from(s_72_7).unwrap());
        // C s_72_9: const #0s : i
        let s_72_9: i128 = 0;
        // C s_72_10: add s_72_9 s_72_8
        let s_72_10: i128 = (s_72_9 + s_72_8);
        // D s_72_11: bit-extract s_72_6 s_72_5 s_72_10
        let s_72_11: Bits = (Bits::new(
            ((s_72_6) >> (s_72_5)).value(),
            u16::try_from(s_72_10).unwrap(),
        ));
        // D s_72_12: cast reint s_72_11 -> u8
        let s_72_12: bool = ((s_72_11.value()) != 0);
        // D s_72_13: cast zx s_72_12 -> bv
        let s_72_13: Bits = Bits::new(s_72_12 as u128, 1u16);
        // C s_72_14: const #0u : u8
        let s_72_14: bool = false;
        // C s_72_15: cast zx s_72_14 -> bv
        let s_72_15: Bits = Bits::new(s_72_14 as u128, 1u16);
        // D s_72_16: cmp-eq s_72_13 s_72_15
        let s_72_16: bool = ((s_72_13) == (s_72_15));
        // D s_72_17: not s_72_16
        let s_72_17: bool = !s_72_16;
        // N s_72_18: branch s_72_17 b78 b73
        if s_72_17 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var disabled <= s_73_0
        fn_state.disabled = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var disabled:u8
        let s_74_0: bool = fn_state.disabled;
        // N s_74_1: branch s_74_0 b77 b75
        if s_74_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_75_0: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #440u : u32
        let s_77_0: u32 = 440;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: u8 = {
            let value = state.read_register::<u8>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call AArch64_AdvSIMDFPAccessTrap(s_77_1)
        let s_77_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_77_1);
        // N s_77_3: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var ga#16548:u8
        let s_78_0: u8 = fn_state.ga_16548;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 2u16);
        // C s_78_2: const #1u : u8
        let s_78_2: u8 = 1;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 2u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: not s_78_4
        let s_78_5: bool = !s_78_4;
        // N s_78_6: branch s_78_5 b80 b79
        if s_78_5 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #16975u : u32
        let s_79_0: u32 = 16975;
        // D s_79_1: read-reg s_79_0:u8
        let s_79_1: u8 = {
            let value = state.read_register::<u8>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // D s_79_2: cast zx s_79_1 -> bv
        let s_79_2: Bits = Bits::new(s_79_1 as u128, 2u16);
        // C s_79_3: const #448u : u32
        let s_79_3: u32 = 448;
        // D s_79_4: read-reg s_79_3:u8
        let s_79_4: u8 = {
            let value = state.read_register::<u8>(s_79_3 as isize);
            tracer.read_register(s_79_3 as isize, value);
            value
        };
        // D s_79_5: cast zx s_79_4 -> bv
        let s_79_5: Bits = Bits::new(s_79_4 as u128, 2u16);
        // D s_79_6: cmp-eq s_79_2 s_79_5
        let s_79_6: bool = ((s_79_2) == (s_79_5));
        // D s_79_7: write-var disabled <= s_79_6
        fn_state.disabled = s_79_6;
        // N s_79_8: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var disabled <= s_80_0
        fn_state.disabled = s_80_0;
        // N s_80_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #440u : u32
        let s_81_0: u32 = 440;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call SVEAccessTrap(s_81_1)
        let s_81_2: () = SVEAccessTrap(state, tracer, s_81_1);
        // N s_81_3: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var ga#16545:u8
        let s_82_0: u8 = fn_state.ga_16545;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 2u16);
        // C s_82_2: const #1u : u8
        let s_82_2: u8 = 1;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 2u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: not s_82_4
        let s_82_5: bool = !s_82_4;
        // N s_82_6: branch s_82_5 b84 b83
        if s_82_5 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #16975u : u32
        let s_83_0: u32 = 16975;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: cast zx s_83_1 -> bv
        let s_83_2: Bits = Bits::new(s_83_1 as u128, 2u16);
        // C s_83_3: const #448u : u32
        let s_83_3: u32 = 448;
        // D s_83_4: read-reg s_83_3:u8
        let s_83_4: u8 = {
            let value = state.read_register::<u8>(s_83_3 as isize);
            tracer.read_register(s_83_3 as isize, value);
            value
        };
        // D s_83_5: cast zx s_83_4 -> bv
        let s_83_5: Bits = Bits::new(s_83_4 as u128, 2u16);
        // D s_83_6: cmp-eq s_83_2 s_83_5
        let s_83_6: bool = ((s_83_2) == (s_83_5));
        // D s_83_7: write-var disabled <= s_83_6
        fn_state.disabled = s_83_6;
        // N s_83_8: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var disabled <= s_84_0
        fn_state.disabled = s_84_0;
        // N s_84_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #() : ()
        let s_85_0: () = ();
        // S s_85_1: call IsInHost(s_85_0)
        let s_85_1: bool = IsInHost(state, tracer, s_85_0);
        // S s_85_2: not s_85_1
        let s_85_2: bool = !s_85_1;
        // D s_85_3: write-var gs#21131 <= s_85_2
        fn_state.gs_21131 = s_85_2;
        // N s_85_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #1u : u8
        let s_86_0: bool = true;
        // D s_86_1: write-var gs#21130 <= s_86_0
        fn_state.gs_21130 = s_86_0;
        // N s_86_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_87_0: panic
        panic!("{:?}", ());
        // N s_87_1: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call EL3SDDUndefPriority(s_88_0)
        let s_88_1: bool = EL3SDDUndefPriority(state, tracer, s_88_0);
        // D s_88_2: write-var gs#21129 <= s_88_1
        fn_state.gs_21129 = s_88_1;
        // N s_88_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #16840u : u32
        let s_89_0: u32 = 16840;
        // D s_89_1: read-reg s_89_0:struct
        let s_89_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // D s_89_2: call _get_CPTR_EL3_Type_EZ(s_89_1)
        let s_89_2: bool = u_get_CPTR_EL3_Type_EZ(state, tracer, s_89_1);
        // D s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // C s_89_4: const #0u : u8
        let s_89_4: bool = false;
        // C s_89_5: cast zx s_89_4 -> bv
        let s_89_5: Bits = Bits::new(s_89_4 as u128, 1u16);
        // D s_89_6: cmp-eq s_89_3 s_89_5
        let s_89_6: bool = ((s_89_3) == (s_89_5));
        // N s_89_7: branch s_89_6 b92 b90
        if s_89_6 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #16840u : u32
        let s_90_0: u32 = 16840;
        // D s_90_1: read-reg s_90_0:struct
        let s_90_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call _get_CPTR_EL3_Type_TFP(s_90_1)
        let s_90_2: bool = u_get_CPTR_EL3_Type_TFP(state, tracer, s_90_1);
        // D s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // C s_90_4: const #1u : u8
        let s_90_4: bool = true;
        // C s_90_5: cast zx s_90_4 -> bv
        let s_90_5: Bits = Bits::new(s_90_4 as u128, 1u16);
        // D s_90_6: cmp-eq s_90_3 s_90_5
        let s_90_6: bool = ((s_90_3) == (s_90_5));
        // D s_90_7: write-var gs#21127 <= s_90_6
        fn_state.gs_21127 = s_90_6;
        // N s_90_8: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#21127:u8
        let s_91_0: bool = fn_state.gs_21127;
        // D s_91_1: write-var gs#21128 <= s_91_0
        fn_state.gs_21128 = s_91_0;
        // N s_91_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #1u : u8
        let s_92_0: bool = true;
        // D s_92_1: write-var gs#21127 <= s_92_0
        fn_state.gs_21127 = s_92_0;
        // N s_92_2: jump b91
        return block_91(state, tracer, fn_state);
    }
}
