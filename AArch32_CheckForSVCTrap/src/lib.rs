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
use u_get_HCR_EL2_Type_E2H::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use ELUsingAArch32::*;
use AArch64_TakeException::*;
use HaveFGTExt::*;
use ExceptionSyndrome::*;
use u_get_HFGITR_EL2_Type_SVC_EL0::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use common::*;
pub fn AArch32_CheckForSVCTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    immediate: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        gs_31865: bool,
        gs_31863: bool,
        gs_31867: bool,
        gs_31864: bool,
        gs_31866: bool,
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
        // N s_2_9: branch s_2_8 b7 b3
        if s_2_8 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var route_to_el2:u8
        let s_4_0: bool = fn_state.route_to_el2;
        // N s_4_1: branch s_4_0 b6 b5
        if s_4_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #12u : u32
        let s_6_0: u32 = 12;
        // S s_6_1: call ExceptionSyndrome(s_6_0)
        let s_6_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_6_0);
        // C s_6_2: const #64s : i64
        let s_6_2: i64 = 64;
        // C s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // S s_6_4: call ThisInstrAddr(s_6_3)
        let s_6_4: Bits = ThisInstrAddr(state, tracer, s_6_3);
        // S s_6_5: cast reint s_6_4 -> u64
        let s_6_5: u64 = (s_6_4.value() as u64);
        // C s_6_6: const #0u : u8
        let s_6_6: u8 = 0;
        // C s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 4u16);
        // C s_6_8: cast zx s_6_7 -> i
        let s_6_8: i128 = (s_6_7.value() as i128);
        // C s_6_9: cast reint s_6_8 -> i64
        let s_6_9: i64 = (s_6_8 as i64);
        // C s_6_10: cast zx s_6_9 -> i
        let s_6_10: i128 = (i128::try_from(s_6_9).unwrap());
        // C s_6_11: const #432u : u32
        let s_6_11: u32 = 432;
        // D s_6_12: read-reg s_6_11:u8
        let s_6_12: u8 = {
            let value = state.read_register::<u8>(s_6_11 as isize);
            tracer.read_register(s_6_11 as isize, value);
            value
        };
        // D s_6_13: call AArch64_TakeException(s_6_12, s_6_1, s_6_5, s_6_10)
        let s_6_13: () = AArch64_TakeException(
            state,
            tracer,
            s_6_12,
            s_6_1,
            s_6_5,
            s_6_10,
        );
        // N s_6_14: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #440u : u32
        let s_7_0: u32 = 440;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call ELUsingAArch32(s_7_1)
        let s_7_2: bool = ELUsingAArch32(state, tracer, s_7_1);
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b22 b8
        if s_7_3 {
            return block_22(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#31863 <= s_8_0
        fn_state.gs_31863 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#31863:u8
        let s_9_0: bool = fn_state.gs_31863;
        // N s_9_1: branch s_9_0 b21 b10
        if s_9_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#31864 <= s_10_0
        fn_state.gs_31864 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#31864:u8
        let s_11_0: bool = fn_state.gs_31864;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#31867 <= s_12_0
        fn_state.gs_31867 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#31867:u8
        let s_13_0: bool = fn_state.gs_31867;
        // D s_13_1: write-var route_to_el2 <= s_13_0
        fn_state.route_to_el2 = s_13_0;
        // N s_13_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #102552u : u32
        let s_14_0: u32 = 102552;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_HCR_EL2_Type_E2H(s_14_1)
        let s_14_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_14_1);
        // C s_14_3: const #102552u : u32
        let s_14_3: u32 = 102552;
        // D s_14_4: read-reg s_14_3:struct
        let s_14_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_3 as isize);
            tracer.read_register(s_14_3 as isize, value);
            value
        };
        // D s_14_5: call _get_HCR_EL2_Type_TGE(s_14_4)
        let s_14_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_14_4);
        // D s_14_6: cast zx s_14_2 -> bv
        let s_14_6: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_7: cast zx s_14_5 -> bv
        let s_14_7: Bits = Bits::new(s_14_5 as u128, 1u16);
        // D s_14_8: cast reint s_14_6 -> u128
        let s_14_8: u128 = (s_14_6.value() as u128);
        // D s_14_9: size-of s_14_6
        let s_14_9: u16 = s_14_6.length();
        // D s_14_10: cast reint s_14_7 -> u128
        let s_14_10: u128 = (s_14_7.value() as u128);
        // D s_14_11: size-of s_14_7
        let s_14_11: u16 = s_14_7.length();
        // D s_14_12: lsl s_14_8 s_14_11
        let s_14_12: u128 = s_14_8 << s_14_11;
        // D s_14_13: or s_14_12 s_14_10
        let s_14_13: u128 = ((s_14_12) | (s_14_10));
        // D s_14_14: add s_14_9 s_14_11
        let s_14_14: u16 = (s_14_9 + s_14_11);
        // D s_14_15: create-bits s_14_13 s_14_14
        let s_14_15: Bits = Bits::new(s_14_13, s_14_14);
        // D s_14_16: cast reint s_14_15 -> u8
        let s_14_16: u8 = (s_14_15.value() as u8);
        // D s_14_17: cast zx s_14_16 -> bv
        let s_14_17: Bits = Bits::new(s_14_16 as u128, 2u16);
        // C s_14_18: const #3u : u8
        let s_14_18: u8 = 3;
        // C s_14_19: cast zx s_14_18 -> bv
        let s_14_19: Bits = Bits::new(s_14_18 as u128, 2u16);
        // D s_14_20: cmp-ne s_14_17 s_14_19
        let s_14_20: bool = ((s_14_17) != (s_14_19));
        // N s_14_21: branch s_14_20 b17 b15
        if s_14_20 {
            return block_17(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#31866 <= s_15_0
        fn_state.gs_31866 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#31866:u8
        let s_16_0: bool = fn_state.gs_31866;
        // D s_16_1: write-var gs#31867 <= s_16_0
        fn_state.gs_31867 = s_16_0;
        // N s_16_2: jump b13
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
        // C s_18_0: const #90704u : u32
        let s_18_0: u32 = 90704;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_SCR_EL3_Type_FGTEn(s_18_1)
        let s_18_2: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // D s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // D s_18_7: write-var gs#31865 <= s_18_6
        fn_state.gs_31865 = s_18_6;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#31865:u8
        let s_19_0: bool = fn_state.gs_31865;
        // D s_19_1: write-var gs#31866 <= s_19_0
        fn_state.gs_31866 = s_19_0;
        // N s_19_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#31865 <= s_20_0
        fn_state.gs_31865 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #13608u : u32
        let s_21_0: u32 = 13608;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_HFGITR_EL2_Type_SVC_EL0(s_21_1)
        let s_21_2: bool = u_get_HFGITR_EL2_Type_SVC_EL0(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: write-var gs#31864 <= s_21_6
        fn_state.gs_31864 = s_21_6;
        // N s_21_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call EL2Enabled(s_22_0)
        let s_22_1: bool = EL2Enabled(state, tracer, s_22_0);
        // D s_22_2: write-var gs#31863 <= s_22_1
        fn_state.gs_31863 = s_22_1;
        // N s_22_3: jump b9
        return block_9(state, tracer, fn_state);
    }
}
