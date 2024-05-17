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
use HavePACExt::*;
use u_get_TCR_EL2_Type_TBID::*;
use u_get_TCR_EL2_Type_TBID1::*;
use u_get_TCR_EL1_Type_TBID0::*;
use S1TranslationRegime::*;
use u_get_TCR_EL2_Type_TBI::*;
use ELIsInHost::*;
use u_get_TCR_EL3_Type_TBID::*;
use u_get_TCR_EL2_Type_TBI1::*;
use u_get_TCR_EL2_Type_TBI0::*;
use u_get_TCR_EL1_Type_TBI0::*;
use u_get_TCR_EL1_Type_TBID1::*;
use ELUsingAArch32::*;
use u_get_TCR_EL1_Type_TBI1::*;
use u_get_TCR_EL2_Type_TBID0::*;
use u_get_TCR_EL3_Type_TBI::*;
use HaveVirtHostExt::*;
use common::*;
pub fn EffectiveTBI<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    IsInstr: bool,
    el: u8,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_1934: bool,
        gs_3212: bool,
        tbi: bool,
        tbid: bool,
        gs_3210: bool,
        regime: u8,
        gs_3211: bool,
        gs_3194: bool,
        address: u64,
        IsInstr: bool,
        el: u8,
    }
    let fn_state = FunctionState {
        address,
        IsInstr,
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var el:u8
        let s_0_0: u8 = fn_state.el;
        // C s_0_1: const #2u : u8
        let s_0_1: u8 = 2;
        // D s_0_2: cmp-lt s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) < (s_0_1));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var el:u8
        let s_0_4: u8 = fn_state.el;
        // D s_0_5: call S1TranslationRegime(s_0_4)
        let s_0_5: u8 = S1TranslationRegime(state, tracer, s_0_4);
        // D s_0_6: write-var regime <= s_0_5
        fn_state.regime = s_0_5;
        // D s_0_7: read-var regime:u8
        let s_0_7: u8 = fn_state.regime;
        // D s_0_8: call ELUsingAArch32(s_0_7)
        let s_0_8: bool = ELUsingAArch32(state, tracer, s_0_7);
        // D s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: assert s_0_9
        let s_0_10: () = assert!(s_0_9);
        // D s_0_11: read-var regime:u8
        let s_0_11: u8 = fn_state.regime;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #440u : u32
        let s_0_13: u32 = 440;
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
        // D s_0_17: not s_0_16
        let s_0_17: bool = !s_0_16;
        // N s_0_18: branch s_0_17 b24 b1
        if s_0_17 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #55s : i
        let s_1_0: i128 = 55;
        // D s_1_1: read-var address:u64
        let s_1_1: u64 = fn_state.address;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 64u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_0 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // C s_1_19: const #1u : u8
        let s_1_19: bool = true;
        // C s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // D s_1_21: cmp-eq s_1_18 s_1_20
        let s_1_21: bool = ((s_1_18) == (s_1_20));
        // N s_1_22: branch s_1_21 b23 b2
        if s_1_21 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #22392u : u32
        let s_2_0: u32 = 22392;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_TCR_EL1_Type_TBI0(s_2_1)
        let s_2_2: bool = u_get_TCR_EL1_Type_TBI0(state, tracer, s_2_1);
        // D s_2_3: write-var tbi <= s_2_2
        fn_state.tbi = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HavePACExt(s_3_0)
        let s_3_1: bool = HavePACExt(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b19 b4
        if s_3_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var tbi:u8
        let s_6_0: bool = fn_state.tbi;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b12 b7
        if s_6_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#3212 <= s_7_0
        fn_state.gs_3212 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#3212:u8
        let s_8_0: bool = fn_state.gs_3212;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var ga#1934 <= s_9_0
        fn_state.ga_1934 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var ga#1934:u8
        let s_10_0: bool = fn_state.ga_1934;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var ga#1934 <= s_11_0
        fn_state.ga_1934 = s_11_0;
        // N s_11_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call HavePACExt(s_12_0)
        let s_12_1: bool = HavePACExt(state, tracer, s_12_0);
        // S s_12_2: not s_12_1
        let s_12_2: bool = !s_12_1;
        // N s_12_3: branch s_12_2 b18 b13
        if s_12_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var tbid:u8
        let s_13_0: bool = fn_state.tbid;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #0u : u8
        let s_13_2: bool = false;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var gs#3210 <= s_13_4
        fn_state.gs_3210 = s_13_4;
        // N s_13_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#3210:u8
        let s_14_0: bool = fn_state.gs_3210;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var IsInstr:u8
        let s_15_0: bool = fn_state.IsInstr;
        // D s_15_1: not s_15_0
        let s_15_1: bool = !s_15_0;
        // D s_15_2: write-var gs#3211 <= s_15_1
        fn_state.gs_3211 = s_15_1;
        // N s_15_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var gs#3211:u8
        let s_16_0: bool = fn_state.gs_3211;
        // D s_16_1: write-var gs#3212 <= s_16_0
        fn_state.gs_3212 = s_16_0;
        // N s_16_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#3211 <= s_17_0
        fn_state.gs_3211 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#3210 <= s_18_0
        fn_state.gs_3210 = s_18_0;
        // N s_18_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #55s : i
        let s_19_0: i128 = 55;
        // D s_19_1: read-var address:u64
        let s_19_1: u64 = fn_state.address;
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 64u16);
        // C s_19_3: const #1u : u64
        let s_19_3: u64 = 1;
        // D s_19_4: bit-extract s_19_2 s_19_0 s_19_3
        let s_19_4: Bits = (Bits::new(
            ((s_19_2) >> (s_19_0)).value(),
            u16::try_from(s_19_3).unwrap(),
        ));
        // D s_19_5: cast reint s_19_4 -> u8
        let s_19_5: bool = ((s_19_4.value()) != 0);
        // C s_19_6: const #0s : i
        let s_19_6: i128 = 0;
        // C s_19_7: const #0u : u64
        let s_19_7: u64 = 0;
        // D s_19_8: cast zx s_19_5 -> u64
        let s_19_8: u64 = (s_19_5 as u64);
        // C s_19_9: const #1u : u64
        let s_19_9: u64 = 1;
        // D s_19_10: and s_19_8 s_19_9
        let s_19_10: u64 = ((s_19_8) & (s_19_9));
        // D s_19_11: cmp-eq s_19_10 s_19_9
        let s_19_11: bool = ((s_19_10) == (s_19_9));
        // D s_19_12: lsl s_19_8 s_19_6
        let s_19_12: u64 = s_19_8 << s_19_6;
        // D s_19_13: or s_19_7 s_19_12
        let s_19_13: u64 = ((s_19_7) | (s_19_12));
        // D s_19_14: cmpl s_19_12
        let s_19_14: u64 = !s_19_12;
        // D s_19_15: and s_19_7 s_19_14
        let s_19_15: u64 = ((s_19_7) & (s_19_14));
        // D s_19_16: select s_19_11 s_19_13 s_19_15
        let s_19_16: u64 = if s_19_11 { s_19_13 } else { s_19_15 };
        // D s_19_17: cast trunc s_19_16 -> u8
        let s_19_17: bool = ((s_19_16) != 0);
        // D s_19_18: cast zx s_19_17 -> bv
        let s_19_18: Bits = Bits::new(s_19_17 as u128, 1u16);
        // C s_19_19: const #1u : u8
        let s_19_19: bool = true;
        // C s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 1u16);
        // D s_19_21: cmp-eq s_19_18 s_19_20
        let s_19_21: bool = ((s_19_18) == (s_19_20));
        // N s_19_22: branch s_19_21 b22 b20
        if s_19_21 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #22392u : u32
        let s_20_0: u32 = 22392;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_TCR_EL1_Type_TBID0(s_20_1)
        let s_20_2: bool = u_get_TCR_EL1_Type_TBID0(state, tracer, s_20_1);
        // D s_20_3: write-var tbid <= s_20_2
        fn_state.tbid = s_20_2;
        // N s_20_4: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_21_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #22392u : u32
        let s_22_0: u32 = 22392;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_TCR_EL1_Type_TBID1(s_22_1)
        let s_22_2: bool = u_get_TCR_EL1_Type_TBID1(state, tracer, s_22_1);
        // D s_22_3: write-var tbid <= s_22_2
        fn_state.tbid = s_22_2;
        // N s_22_4: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #22392u : u32
        let s_23_0: u32 = 22392;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_TCR_EL1_Type_TBI1(s_23_1)
        let s_23_2: bool = u_get_TCR_EL1_Type_TBI1(state, tracer, s_23_1);
        // D s_23_3: write-var tbi <= s_23_2
        fn_state.tbi = s_23_2;
        // N s_23_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var regime:u8
        let s_24_0: u8 = fn_state.regime;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 2u16);
        // C s_24_2: const #432u : u32
        let s_24_2: u32 = 432;
        // D s_24_3: read-reg s_24_2:u8
        let s_24_3: u8 = {
            let value = state.read_register::<u8>(s_24_2 as isize);
            tracer.read_register(s_24_2 as isize, value);
            value
        };
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 2u16);
        // D s_24_5: cmp-eq s_24_1 s_24_4
        let s_24_5: bool = ((s_24_1) == (s_24_4));
        // D s_24_6: not s_24_5
        let s_24_6: bool = !s_24_5;
        // N s_24_7: branch s_24_6 b43 b25
        if s_24_6 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call HaveVirtHostExt(s_25_0)
        let s_25_1: bool = HaveVirtHostExt(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b42 b26
        if s_25_1 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#3194 <= s_26_0
        fn_state.gs_3194 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var gs#3194:u8
        let s_27_0: bool = fn_state.gs_3194;
        // N s_27_1: branch s_27_0 b32 b28
        if s_27_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #12816u : u32
        let s_28_0: u32 = 12816;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_TCR_EL2_Type_TBI(s_28_1)
        let s_28_2: bool = u_get_TCR_EL2_Type_TBI(state, tracer, s_28_1);
        // D s_28_3: write-var tbi <= s_28_2
        fn_state.tbi = s_28_2;
        // C s_28_4: const #() : ()
        let s_28_4: () = ();
        // S s_28_5: call HavePACExt(s_28_4)
        let s_28_5: bool = HavePACExt(state, tracer, s_28_4);
        // N s_28_6: branch s_28_5 b31 b29
        if s_28_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_30_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #12816u : u32
        let s_31_0: u32 = 12816;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_TCR_EL2_Type_TBID(s_31_1)
        let s_31_2: bool = u_get_TCR_EL2_Type_TBID(state, tracer, s_31_1);
        // D s_31_3: write-var tbid <= s_31_2
        fn_state.tbid = s_31_2;
        // N s_31_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #55s : i
        let s_32_0: i128 = 55;
        // D s_32_1: read-var address:u64
        let s_32_1: u64 = fn_state.address;
        // D s_32_2: cast zx s_32_1 -> bv
        let s_32_2: Bits = Bits::new(s_32_1 as u128, 64u16);
        // C s_32_3: const #1u : u64
        let s_32_3: u64 = 1;
        // D s_32_4: bit-extract s_32_2 s_32_0 s_32_3
        let s_32_4: Bits = (Bits::new(
            ((s_32_2) >> (s_32_0)).value(),
            u16::try_from(s_32_3).unwrap(),
        ));
        // D s_32_5: cast reint s_32_4 -> u8
        let s_32_5: bool = ((s_32_4.value()) != 0);
        // C s_32_6: const #0s : i
        let s_32_6: i128 = 0;
        // C s_32_7: const #0u : u64
        let s_32_7: u64 = 0;
        // D s_32_8: cast zx s_32_5 -> u64
        let s_32_8: u64 = (s_32_5 as u64);
        // C s_32_9: const #1u : u64
        let s_32_9: u64 = 1;
        // D s_32_10: and s_32_8 s_32_9
        let s_32_10: u64 = ((s_32_8) & (s_32_9));
        // D s_32_11: cmp-eq s_32_10 s_32_9
        let s_32_11: bool = ((s_32_10) == (s_32_9));
        // D s_32_12: lsl s_32_8 s_32_6
        let s_32_12: u64 = s_32_8 << s_32_6;
        // D s_32_13: or s_32_7 s_32_12
        let s_32_13: u64 = ((s_32_7) | (s_32_12));
        // D s_32_14: cmpl s_32_12
        let s_32_14: u64 = !s_32_12;
        // D s_32_15: and s_32_7 s_32_14
        let s_32_15: u64 = ((s_32_7) & (s_32_14));
        // D s_32_16: select s_32_11 s_32_13 s_32_15
        let s_32_16: u64 = if s_32_11 { s_32_13 } else { s_32_15 };
        // D s_32_17: cast trunc s_32_16 -> u8
        let s_32_17: bool = ((s_32_16) != 0);
        // D s_32_18: cast zx s_32_17 -> bv
        let s_32_18: Bits = Bits::new(s_32_17 as u128, 1u16);
        // C s_32_19: const #1u : u8
        let s_32_19: bool = true;
        // C s_32_20: cast zx s_32_19 -> bv
        let s_32_20: Bits = Bits::new(s_32_19 as u128, 1u16);
        // D s_32_21: cmp-eq s_32_18 s_32_20
        let s_32_21: bool = ((s_32_18) == (s_32_20));
        // N s_32_22: branch s_32_21 b41 b33
        if s_32_21 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_33_0: const #12816u : u32
        let s_33_0: u32 = 12816;
        // D s_33_1: read-reg s_33_0:struct
        let s_33_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call _get_TCR_EL2_Type_TBI0(s_33_1)
        let s_33_2: bool = u_get_TCR_EL2_Type_TBI0(state, tracer, s_33_1);
        // D s_33_3: write-var tbi <= s_33_2
        fn_state.tbi = s_33_2;
        // N s_33_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call HavePACExt(s_34_0)
        let s_34_1: bool = HavePACExt(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b37 b35
        if s_34_1 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_36_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #55s : i
        let s_37_0: i128 = 55;
        // D s_37_1: read-var address:u64
        let s_37_1: u64 = fn_state.address;
        // D s_37_2: cast zx s_37_1 -> bv
        let s_37_2: Bits = Bits::new(s_37_1 as u128, 64u16);
        // C s_37_3: const #1u : u64
        let s_37_3: u64 = 1;
        // D s_37_4: bit-extract s_37_2 s_37_0 s_37_3
        let s_37_4: Bits = (Bits::new(
            ((s_37_2) >> (s_37_0)).value(),
            u16::try_from(s_37_3).unwrap(),
        ));
        // D s_37_5: cast reint s_37_4 -> u8
        let s_37_5: bool = ((s_37_4.value()) != 0);
        // C s_37_6: const #0s : i
        let s_37_6: i128 = 0;
        // C s_37_7: const #0u : u64
        let s_37_7: u64 = 0;
        // D s_37_8: cast zx s_37_5 -> u64
        let s_37_8: u64 = (s_37_5 as u64);
        // C s_37_9: const #1u : u64
        let s_37_9: u64 = 1;
        // D s_37_10: and s_37_8 s_37_9
        let s_37_10: u64 = ((s_37_8) & (s_37_9));
        // D s_37_11: cmp-eq s_37_10 s_37_9
        let s_37_11: bool = ((s_37_10) == (s_37_9));
        // D s_37_12: lsl s_37_8 s_37_6
        let s_37_12: u64 = s_37_8 << s_37_6;
        // D s_37_13: or s_37_7 s_37_12
        let s_37_13: u64 = ((s_37_7) | (s_37_12));
        // D s_37_14: cmpl s_37_12
        let s_37_14: u64 = !s_37_12;
        // D s_37_15: and s_37_7 s_37_14
        let s_37_15: u64 = ((s_37_7) & (s_37_14));
        // D s_37_16: select s_37_11 s_37_13 s_37_15
        let s_37_16: u64 = if s_37_11 { s_37_13 } else { s_37_15 };
        // D s_37_17: cast trunc s_37_16 -> u8
        let s_37_17: bool = ((s_37_16) != 0);
        // D s_37_18: cast zx s_37_17 -> bv
        let s_37_18: Bits = Bits::new(s_37_17 as u128, 1u16);
        // C s_37_19: const #1u : u8
        let s_37_19: bool = true;
        // C s_37_20: cast zx s_37_19 -> bv
        let s_37_20: Bits = Bits::new(s_37_19 as u128, 1u16);
        // D s_37_21: cmp-eq s_37_18 s_37_20
        let s_37_21: bool = ((s_37_18) == (s_37_20));
        // N s_37_22: branch s_37_21 b40 b38
        if s_37_21 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #12816u : u32
        let s_38_0: u32 = 12816;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call _get_TCR_EL2_Type_TBID0(s_38_1)
        let s_38_2: bool = u_get_TCR_EL2_Type_TBID0(state, tracer, s_38_1);
        // D s_38_3: write-var tbid <= s_38_2
        fn_state.tbid = s_38_2;
        // N s_38_4: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_39_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #12816u : u32
        let s_40_0: u32 = 12816;
        // D s_40_1: read-reg s_40_0:struct
        let s_40_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: call _get_TCR_EL2_Type_TBID1(s_40_1)
        let s_40_2: bool = u_get_TCR_EL2_Type_TBID1(state, tracer, s_40_1);
        // D s_40_3: write-var tbid <= s_40_2
        fn_state.tbid = s_40_2;
        // N s_40_4: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_41_0: const #12816u : u32
        let s_41_0: u32 = 12816;
        // D s_41_1: read-reg s_41_0:struct
        let s_41_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call _get_TCR_EL2_Type_TBI1(s_41_1)
        let s_41_2: bool = u_get_TCR_EL2_Type_TBI1(state, tracer, s_41_1);
        // D s_41_3: write-var tbi <= s_41_2
        fn_state.tbi = s_41_2;
        // N s_41_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_42_0: read-var el:u8
        let s_42_0: u8 = fn_state.el;
        // D s_42_1: call ELIsInHost(s_42_0)
        let s_42_1: bool = ELIsInHost(state, tracer, s_42_0);
        // D s_42_2: write-var gs#3194 <= s_42_1
        fn_state.gs_3194 = s_42_1;
        // N s_42_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_43_0: read-var regime:u8
        let s_43_0: u8 = fn_state.regime;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 2u16);
        // C s_43_2: const #424u : u32
        let s_43_2: u32 = 424;
        // D s_43_3: read-reg s_43_2:u8
        let s_43_3: u8 = {
            let value = state.read_register::<u8>(s_43_2 as isize);
            tracer.read_register(s_43_2 as isize, value);
            value
        };
        // D s_43_4: cast zx s_43_3 -> bv
        let s_43_4: Bits = Bits::new(s_43_3 as u128, 2u16);
        // D s_43_5: cmp-eq s_43_1 s_43_4
        let s_43_5: bool = ((s_43_1) == (s_43_4));
        // D s_43_6: not s_43_5
        let s_43_6: bool = !s_43_5;
        // N s_43_7: branch s_43_6 b48 b44
        if s_43_6 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_44_0: const #10736u : u32
        let s_44_0: u32 = 10736;
        // D s_44_1: read-reg s_44_0:struct
        let s_44_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call _get_TCR_EL3_Type_TBI(s_44_1)
        let s_44_2: bool = u_get_TCR_EL3_Type_TBI(state, tracer, s_44_1);
        // D s_44_3: write-var tbi <= s_44_2
        fn_state.tbi = s_44_2;
        // C s_44_4: const #() : ()
        let s_44_4: () = ();
        // S s_44_5: call HavePACExt(s_44_4)
        let s_44_5: bool = HavePACExt(state, tracer, s_44_4);
        // N s_44_6: branch s_44_5 b47 b45
        if s_44_5 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_46_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_47_0: const #10736u : u32
        let s_47_0: u32 = 10736;
        // D s_47_1: read-reg s_47_0:struct
        let s_47_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call _get_TCR_EL3_Type_TBID(s_47_1)
        let s_47_2: bool = u_get_TCR_EL3_Type_TBID(state, tracer, s_47_1);
        // D s_47_3: write-var tbid <= s_47_2
        fn_state.tbid = s_47_2;
        // N s_47_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_48_0: jump b6
        return block_6(state, tracer, fn_state);
    }
}
