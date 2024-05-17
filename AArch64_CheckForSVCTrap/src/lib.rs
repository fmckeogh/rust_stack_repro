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
use ThisInstrAddr::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use AArch64_TakeException::*;
use u_get_HFGITR_EL2_Type_SVC_EL1::*;
use HaveFGTExt::*;
use ExceptionSyndrome::*;
use UsingAArch32::*;
use ELUsingAArch32::*;
use u_get_HFGITR_EL2_Type_SVC_EL0::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn AArch64_CheckForSVCTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    immediate: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        gs_24761: bool,
        gs_24764: bool,
        gs_24760: bool,
        gs_24765: bool,
        gs_24759: bool,
        gs_24758: bool,
        gs_24757: bool,
        gs_24762: bool,
        gs_24763: bool,
        immediate: u16,
    }
    let fn_state = FunctionState {
        immediate,
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
        // S s_0_1: call HaveFGTExt(s_0_0)
        let s_0_1: bool = HaveFGTExt(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var route_to_el2 <= s_2_0
        fn_state.route_to_el2 = s_2_0;
        // C s_2_2: const #16975u : u32
        let s_2_2: u32 = 16975;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // C s_2_5: const #448u : u32
        let s_2_5: u32 = 448;
        // D s_2_6: read-reg s_2_5:u8
        let s_2_6: u8 = {
            let value = state.read_register::<u8>(s_2_5 as isize);
            tracer.read_register(s_2_5 as isize, value);
            value
        };
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // D s_2_8: cmp-eq s_2_4 s_2_7
        let s_2_8: bool = ((s_2_4) == (s_2_7));
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
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #440u : u32
        let s_3_3: u32 = 440;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) == (s_3_5));
        // N s_3_7: branch s_3_6 b8 b4
        if s_3_6 {
            return block_8(state, tracer, fn_state);
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
        // D s_5_0: read-var route_to_el2:u8
        let s_5_0: bool = fn_state.route_to_el2;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #12u : u32
        let s_7_0: u32 = 12;
        // S s_7_1: call ExceptionSyndrome(s_7_0)
        let s_7_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_7_0);
        // C s_7_2: const #64s : i64
        let s_7_2: i64 = 64;
        // C s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // S s_7_4: call ThisInstrAddr(s_7_3)
        let s_7_4: Bits = ThisInstrAddr(state, tracer, s_7_3);
        // S s_7_5: cast reint s_7_4 -> u64
        let s_7_5: u64 = (s_7_4.value() as u64);
        // C s_7_6: const #0u : u8
        let s_7_6: u8 = 0;
        // C s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 4u16);
        // C s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (s_7_7.value() as i128);
        // C s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // C s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // C s_7_11: const #432u : u32
        let s_7_11: u32 = 432;
        // D s_7_12: read-reg s_7_11:u8
        let s_7_12: u8 = {
            let value = state.read_register::<u8>(s_7_11 as isize);
            tracer.read_register(s_7_11 as isize, value);
            value
        };
        // D s_7_13: call AArch64_TakeException(s_7_12, s_7_1, s_7_5, s_7_10)
        let s_7_13: () = AArch64_TakeException(
            state,
            tracer,
            s_7_12,
            s_7_1,
            s_7_5,
            s_7_10,
        );
        // N s_7_14: return
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
        // N s_8_2: branch s_8_1 b17 b9
        if s_8_1 {
            return block_17(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#24757 <= s_9_0
        fn_state.gs_24757 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#24757:u8
        let s_10_0: bool = fn_state.gs_24757;
        // N s_10_1: branch s_10_0 b13 b11
        if s_10_0 {
            return block_13(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#24759 <= s_11_0
        fn_state.gs_24759 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#24759:u8
        let s_12_0: bool = fn_state.gs_24759;
        // D s_12_1: write-var route_to_el2 <= s_12_0
        fn_state.route_to_el2 = s_12_0;
        // N s_12_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #424u : u32
        let s_13_0: u32 = 424;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // D s_13_3: cmp-lt s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) < (s_13_2));
        // D s_13_4: not s_13_3
        let s_13_4: bool = !s_13_3;
        // N s_13_5: branch s_13_4 b16 b14
        if s_13_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #90704u : u32
        let s_14_0: u32 = 90704;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_SCR_EL3_Type_FGTEn(s_14_1)
        let s_14_2: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // D s_14_7: write-var gs#24758 <= s_14_6
        fn_state.gs_24758 = s_14_6;
        // N s_14_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#24758:u8
        let s_15_0: bool = fn_state.gs_24758;
        // D s_15_1: write-var gs#24759 <= s_15_0
        fn_state.gs_24759 = s_15_0;
        // N s_15_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#24758 <= s_16_0
        fn_state.gs_24758 = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #13608u : u32
        let s_17_0: u32 = 13608;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_HFGITR_EL2_Type_SVC_EL1(s_17_1)
        let s_17_2: bool = u_get_HFGITR_EL2_Type_SVC_EL1(state, tracer, s_17_1);
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // C s_17_4: const #1u : u8
        let s_17_4: bool = true;
        // C s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 1u16);
        // D s_17_6: cmp-eq s_17_3 s_17_5
        let s_17_6: bool = ((s_17_3) == (s_17_5));
        // D s_17_7: write-var gs#24757 <= s_17_6
        fn_state.gs_24757 = s_17_6;
        // N s_17_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call UsingAArch32(s_18_0)
        let s_18_1: bool = UsingAArch32(state, tracer, s_18_0);
        // S s_18_2: not s_18_1
        let s_18_2: bool = !s_18_1;
        // N s_18_3: branch s_18_2 b36 b19
        if s_18_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#24760 <= s_19_0
        fn_state.gs_24760 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#24760:u8
        let s_20_0: bool = fn_state.gs_24760;
        // N s_20_1: branch s_20_0 b35 b21
        if s_20_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#24761 <= s_21_0
        fn_state.gs_24761 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#24761:u8
        let s_22_0: bool = fn_state.gs_24761;
        // N s_22_1: branch s_22_0 b34 b23
        if s_22_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#24762 <= s_23_0
        fn_state.gs_24762 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#24762:u8
        let s_24_0: bool = fn_state.gs_24762;
        // N s_24_1: branch s_24_0 b27 b25
        if s_24_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#24765 <= s_25_0
        fn_state.gs_24765 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#24765:u8
        let s_26_0: bool = fn_state.gs_24765;
        // D s_26_1: write-var route_to_el2 <= s_26_0
        fn_state.route_to_el2 = s_26_0;
        // N s_26_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #102552u : u32
        let s_27_0: u32 = 102552;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_HCR_EL2_Type_E2H(s_27_1)
        let s_27_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_27_1);
        // C s_27_3: const #102552u : u32
        let s_27_3: u32 = 102552;
        // D s_27_4: read-reg s_27_3:struct
        let s_27_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_3 as isize);
            tracer.read_register(s_27_3 as isize, value);
            value
        };
        // D s_27_5: call _get_HCR_EL2_Type_TGE(s_27_4)
        let s_27_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_27_4);
        // D s_27_6: cast zx s_27_2 -> bv
        let s_27_6: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_7: cast zx s_27_5 -> bv
        let s_27_7: Bits = Bits::new(s_27_5 as u128, 1u16);
        // D s_27_8: cast reint s_27_6 -> u128
        let s_27_8: u128 = (s_27_6.value() as u128);
        // D s_27_9: size-of s_27_6
        let s_27_9: u16 = s_27_6.length();
        // D s_27_10: cast reint s_27_7 -> u128
        let s_27_10: u128 = (s_27_7.value() as u128);
        // D s_27_11: size-of s_27_7
        let s_27_11: u16 = s_27_7.length();
        // D s_27_12: lsl s_27_8 s_27_11
        let s_27_12: u128 = s_27_8 << s_27_11;
        // D s_27_13: or s_27_12 s_27_10
        let s_27_13: u128 = ((s_27_12) | (s_27_10));
        // D s_27_14: add s_27_9 s_27_11
        let s_27_14: u16 = (s_27_9 + s_27_11);
        // D s_27_15: create-bits s_27_13 s_27_14
        let s_27_15: Bits = Bits::new(s_27_13, s_27_14);
        // D s_27_16: cast reint s_27_15 -> u8
        let s_27_16: u8 = (s_27_15.value() as u8);
        // D s_27_17: cast zx s_27_16 -> bv
        let s_27_17: Bits = Bits::new(s_27_16 as u128, 2u16);
        // C s_27_18: const #3u : u8
        let s_27_18: u8 = 3;
        // C s_27_19: cast zx s_27_18 -> bv
        let s_27_19: Bits = Bits::new(s_27_18 as u128, 2u16);
        // D s_27_20: cmp-ne s_27_17 s_27_19
        let s_27_20: bool = ((s_27_17) != (s_27_19));
        // N s_27_21: branch s_27_20 b30 b28
        if s_27_20 {
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
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#24764 <= s_28_0
        fn_state.gs_24764 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#24764:u8
        let s_29_0: bool = fn_state.gs_24764;
        // D s_29_1: write-var gs#24765 <= s_29_0
        fn_state.gs_24765 = s_29_0;
        // N s_29_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #424u : u32
        let s_30_0: u32 = 424;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // C s_30_2: const #2u : u8
        let s_30_2: u8 = 2;
        // D s_30_3: cmp-lt s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) < (s_30_2));
        // D s_30_4: not s_30_3
        let s_30_4: bool = !s_30_3;
        // N s_30_5: branch s_30_4 b33 b31
        if s_30_4 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #90704u : u32
        let s_31_0: u32 = 90704;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_SCR_EL3_Type_FGTEn(s_31_1)
        let s_31_2: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_31_1);
        // D s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // C s_31_4: const #1u : u8
        let s_31_4: bool = true;
        // C s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 1u16);
        // D s_31_6: cmp-eq s_31_3 s_31_5
        let s_31_6: bool = ((s_31_3) == (s_31_5));
        // D s_31_7: write-var gs#24763 <= s_31_6
        fn_state.gs_24763 = s_31_6;
        // N s_31_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#24763:u8
        let s_32_0: bool = fn_state.gs_24763;
        // D s_32_1: write-var gs#24764 <= s_32_0
        fn_state.gs_24764 = s_32_0;
        // N s_32_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#24763 <= s_33_0
        fn_state.gs_24763 = s_33_0;
        // N s_33_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #13608u : u32
        let s_34_0: u32 = 13608;
        // D s_34_1: read-reg s_34_0:struct
        let s_34_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call _get_HFGITR_EL2_Type_SVC_EL0(s_34_1)
        let s_34_2: bool = u_get_HFGITR_EL2_Type_SVC_EL0(state, tracer, s_34_1);
        // D s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // C s_34_4: const #1u : u8
        let s_34_4: bool = true;
        // C s_34_5: cast zx s_34_4 -> bv
        let s_34_5: Bits = Bits::new(s_34_4 as u128, 1u16);
        // D s_34_6: cmp-eq s_34_3 s_34_5
        let s_34_6: bool = ((s_34_3) == (s_34_5));
        // D s_34_7: write-var gs#24762 <= s_34_6
        fn_state.gs_24762 = s_34_6;
        // N s_34_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call EL2Enabled(s_35_0)
        let s_35_1: bool = EL2Enabled(state, tracer, s_35_0);
        // D s_35_2: write-var gs#24761 <= s_35_1
        fn_state.gs_24761 = s_35_1;
        // N s_35_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #440u : u32
        let s_36_0: u32 = 440;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call ELUsingAArch32(s_36_1)
        let s_36_2: bool = ELUsingAArch32(state, tracer, s_36_1);
        // D s_36_3: not s_36_2
        let s_36_3: bool = !s_36_2;
        // D s_36_4: write-var gs#24760 <= s_36_3
        fn_state.gs_24760 = s_36_3;
        // N s_36_5: jump b20
        return block_20(state, tracer, fn_state);
    }
}
