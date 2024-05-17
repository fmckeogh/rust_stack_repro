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
use u_get_TCR_EL1_Type_TCMA1::*;
use u_get_TCR_EL2_Type_TCMA1::*;
use u_get_TCR_EL2_Type_TCMA0::*;
use u_get_TCR_EL1_Type_TCMA0::*;
use ELUsingAArch32::*;
use S1TranslationRegime::*;
use u_get_TCR_EL2_Type_TCMA::*;
use ELIsInHost::*;
use HaveVirtHostExt::*;
use u_get_TCR_EL3_Type_TCMA::*;
use common::*;
pub fn EffectiveTCMA<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    el: u8,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        tcma: bool,
        regime: u8,
        gs_6813: bool,
        address: u64,
        el: u8,
    }
    let fn_state = FunctionState {
        address,
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
        // N s_0_18: branch s_0_17 b6 b1
        if s_0_17 {
            return block_6(state, tracer, fn_state);
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
        // N s_1_22: branch s_1_21 b5 b2
        if s_1_21 {
            return block_5(state, tracer, fn_state);
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
        // D s_2_2: call _get_TCR_EL1_Type_TCMA0(s_2_1)
        let s_2_2: bool = u_get_TCR_EL1_Type_TCMA0(state, tracer, s_2_1);
        // D s_2_3: write-var tcma <= s_2_2
        fn_state.tcma = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var tcma:u8
        let s_4_0: bool = fn_state.tcma;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #22392u : u32
        let s_5_0: u32 = 22392;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_TCR_EL1_Type_TCMA1(s_5_1)
        let s_5_2: bool = u_get_TCR_EL1_Type_TCMA1(state, tracer, s_5_1);
        // D s_5_3: write-var tcma <= s_5_2
        fn_state.tcma = s_5_2;
        // N s_5_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var regime:u8
        let s_6_0: u8 = fn_state.regime;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #432u : u32
        let s_6_2: u32 = 432;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // D s_6_6: not s_6_5
        let s_6_6: bool = !s_6_5;
        // N s_6_7: branch s_6_6 b16 b7
        if s_6_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveVirtHostExt(s_7_0)
        let s_7_1: bool = HaveVirtHostExt(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b15 b8
        if s_7_1 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#6813 <= s_8_0
        fn_state.gs_6813 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#6813:u8
        let s_9_0: bool = fn_state.gs_6813;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #12816u : u32
        let s_10_0: u32 = 12816;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_TCR_EL2_Type_TCMA(s_10_1)
        let s_10_2: bool = u_get_TCR_EL2_Type_TCMA(state, tracer, s_10_1);
        // D s_10_3: write-var tcma <= s_10_2
        fn_state.tcma = s_10_2;
        // N s_10_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #55s : i
        let s_11_0: i128 = 55;
        // D s_11_1: read-var address:u64
        let s_11_1: u64 = fn_state.address;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 64u16);
        // C s_11_3: const #1u : u64
        let s_11_3: u64 = 1;
        // D s_11_4: bit-extract s_11_2 s_11_0 s_11_3
        let s_11_4: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_5: cast reint s_11_4 -> u8
        let s_11_5: bool = ((s_11_4.value()) != 0);
        // C s_11_6: const #0s : i
        let s_11_6: i128 = 0;
        // C s_11_7: const #0u : u64
        let s_11_7: u64 = 0;
        // D s_11_8: cast zx s_11_5 -> u64
        let s_11_8: u64 = (s_11_5 as u64);
        // C s_11_9: const #1u : u64
        let s_11_9: u64 = 1;
        // D s_11_10: and s_11_8 s_11_9
        let s_11_10: u64 = ((s_11_8) & (s_11_9));
        // D s_11_11: cmp-eq s_11_10 s_11_9
        let s_11_11: bool = ((s_11_10) == (s_11_9));
        // D s_11_12: lsl s_11_8 s_11_6
        let s_11_12: u64 = s_11_8 << s_11_6;
        // D s_11_13: or s_11_7 s_11_12
        let s_11_13: u64 = ((s_11_7) | (s_11_12));
        // D s_11_14: cmpl s_11_12
        let s_11_14: u64 = !s_11_12;
        // D s_11_15: and s_11_7 s_11_14
        let s_11_15: u64 = ((s_11_7) & (s_11_14));
        // D s_11_16: select s_11_11 s_11_13 s_11_15
        let s_11_16: u64 = if s_11_11 { s_11_13 } else { s_11_15 };
        // D s_11_17: cast trunc s_11_16 -> u8
        let s_11_17: bool = ((s_11_16) != 0);
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 1u16);
        // C s_11_19: const #1u : u8
        let s_11_19: bool = true;
        // C s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: cmp-eq s_11_18 s_11_20
        let s_11_21: bool = ((s_11_18) == (s_11_20));
        // N s_11_22: branch s_11_21 b14 b12
        if s_11_21 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #12816u : u32
        let s_12_0: u32 = 12816;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_TCR_EL2_Type_TCMA0(s_12_1)
        let s_12_2: bool = u_get_TCR_EL2_Type_TCMA0(state, tracer, s_12_1);
        // D s_12_3: write-var tcma <= s_12_2
        fn_state.tcma = s_12_2;
        // N s_12_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_13_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #12816u : u32
        let s_14_0: u32 = 12816;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_TCR_EL2_Type_TCMA1(s_14_1)
        let s_14_2: bool = u_get_TCR_EL2_Type_TCMA1(state, tracer, s_14_1);
        // D s_14_3: write-var tcma <= s_14_2
        fn_state.tcma = s_14_2;
        // N s_14_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var el:u8
        let s_15_0: u8 = fn_state.el;
        // D s_15_1: call ELIsInHost(s_15_0)
        let s_15_1: bool = ELIsInHost(state, tracer, s_15_0);
        // D s_15_2: write-var gs#6813 <= s_15_1
        fn_state.gs_6813 = s_15_1;
        // N s_15_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var regime:u8
        let s_16_0: u8 = fn_state.regime;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 2u16);
        // C s_16_2: const #424u : u32
        let s_16_2: u32 = 424;
        // D s_16_3: read-reg s_16_2:u8
        let s_16_3: u8 = {
            let value = state.read_register::<u8>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 2u16);
        // D s_16_5: cmp-eq s_16_1 s_16_4
        let s_16_5: bool = ((s_16_1) == (s_16_4));
        // D s_16_6: not s_16_5
        let s_16_6: bool = !s_16_5;
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
    ) -> bool {
        // C s_17_0: const #10736u : u32
        let s_17_0: u32 = 10736;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_TCR_EL3_Type_TCMA(s_17_1)
        let s_17_2: bool = u_get_TCR_EL3_Type_TCMA(state, tracer, s_17_1);
        // D s_17_3: write-var tcma <= s_17_2
        fn_state.tcma = s_17_2;
        // N s_17_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_18_0: jump b4
        return block_4(state, tracer, fn_state);
    }
}
