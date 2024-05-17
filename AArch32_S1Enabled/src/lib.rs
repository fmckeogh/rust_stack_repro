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
use ELStateUsingAArch32::*;
use SCTLR_read__2::*;
use AArch32_EL2Enabled::*;
use u_get_HCR_EL2_Type_DC::*;
use u_get_HCR_Type_DC::*;
use HCR_read::*;
use HaveAArch32EL::*;
use SCTLR_NS_read::*;
use u_get_HSCTLR_Type_M::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_SCTLR_Type_M::*;
use u_get_HCR_Type_TGE::*;
use HSCTLR_read::*;
use common::*;
pub fn AArch32_S1Enabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    ss: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_21397: bool,
        gs_27717: bool,
        return_value: bool,
        gs_27718: bool,
        ga_21385: bool,
        regime: u32,
        ss: u32,
    }
    let fn_state = FunctionState {
        regime,
        ss,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var regime:u32
        let s_0_0: u32 = fn_state.regime;
        // C s_0_1: const #2u : u32
        let s_0_1: u32 = 2;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b21 b1
        if s_0_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var regime:u32
        let s_1_0: u32 = fn_state.regime;
        // C s_1_1: const #1u : u32
        let s_1_1: u32 = 1;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b20 b2
        if s_1_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var ss:u32
        let s_2_0: u32 = fn_state.ss;
        // D s_2_1: call AArch32_EL2Enabled(s_2_0)
        let s_2_1: bool = AArch32_EL2Enabled(state, tracer, s_2_0);
        // D s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // N s_2_3: branch s_2_2 b16 b3
        if s_2_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var ss:u32
        let s_3_0: u32 = fn_state.ss;
        // C s_3_1: const #3u : u32
        let s_3_1: u32 = 3;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // C s_3_3: const #432u : u32
        let s_3_3: u32 = 432;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: call ELStateUsingAArch32(s_3_4, s_3_2)
        let s_3_5: bool = ELStateUsingAArch32(state, tracer, s_3_4, s_3_2);
        // N s_3_6: branch s_3_5 b9 b4
        if s_3_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #102552u : u32
        let s_4_0: u32 = 102552;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_HCR_EL2_Type_TGE(s_4_1)
        let s_4_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_4_1);
        // C s_4_3: const #102552u : u32
        let s_4_3: u32 = 102552;
        // D s_4_4: read-reg s_4_3:struct
        let s_4_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_3 as isize);
            tracer.read_register(s_4_3 as isize, value);
            value
        };
        // D s_4_5: call _get_HCR_EL2_Type_DC(s_4_4)
        let s_4_5: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_4_4);
        // D s_4_6: cast zx s_4_2 -> bv
        let s_4_6: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_7: cast zx s_4_5 -> bv
        let s_4_7: Bits = Bits::new(s_4_5 as u128, 1u16);
        // D s_4_8: cast reint s_4_6 -> u128
        let s_4_8: u128 = (s_4_6.value() as u128);
        // D s_4_9: size-of s_4_6
        let s_4_9: u16 = s_4_6.length();
        // D s_4_10: cast reint s_4_7 -> u128
        let s_4_10: u128 = (s_4_7.value() as u128);
        // D s_4_11: size-of s_4_7
        let s_4_11: u16 = s_4_7.length();
        // D s_4_12: lsl s_4_8 s_4_11
        let s_4_12: u128 = s_4_8 << s_4_11;
        // D s_4_13: or s_4_12 s_4_10
        let s_4_13: u128 = ((s_4_12) | (s_4_10));
        // D s_4_14: add s_4_9 s_4_11
        let s_4_14: u16 = (s_4_9 + s_4_11);
        // D s_4_15: create-bits s_4_13 s_4_14
        let s_4_15: Bits = Bits::new(s_4_13, s_4_14);
        // D s_4_16: cast reint s_4_15 -> u8
        let s_4_16: u8 = (s_4_15.value() as u8);
        // D s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 2u16);
        // C s_4_18: const #0u : u8
        let s_4_18: u8 = 0;
        // C s_4_19: cast zx s_4_18 -> bv
        let s_4_19: Bits = Bits::new(s_4_18 as u128, 2u16);
        // D s_4_20: cmp-eq s_4_17 s_4_19
        let s_4_20: bool = ((s_4_17) == (s_4_19));
        // N s_4_21: branch s_4_20 b8 b5
        if s_4_20 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#27717 <= s_5_0
        fn_state.gs_27717 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#27717:u8
        let s_6_0: bool = fn_state.gs_27717;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var return_value:u8
        let s_7_0: bool = fn_state.return_value;
        // N s_7_1: return s_7_0
        return s_7_0;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call SCTLR_read__2(s_8_0)
        let s_8_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_8_0);
        // S s_8_2: call _get_SCTLR_Type_M(s_8_1)
        let s_8_2: bool = u_get_SCTLR_Type_M(state, tracer, s_8_1);
        // S s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // S s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // D s_8_7: write-var gs#27717 <= s_8_6
        fn_state.gs_27717 = s_8_6;
        // N s_8_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call HCR_read(s_9_0)
        let s_9_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_9_0);
        // S s_9_2: call _get_HCR_Type_TGE(s_9_1)
        let s_9_2: bool = u_get_HCR_Type_TGE(state, tracer, s_9_1);
        // C s_9_3: const #() : ()
        let s_9_3: () = ();
        // S s_9_4: call HCR_read(s_9_3)
        let s_9_4: ProductType700c18a878c5601b = HCR_read(state, tracer, s_9_3);
        // S s_9_5: call _get_HCR_Type_DC(s_9_4)
        let s_9_5: bool = u_get_HCR_Type_DC(state, tracer, s_9_4);
        // S s_9_6: cast zx s_9_2 -> bv
        let s_9_6: Bits = Bits::new(s_9_2 as u128, 1u16);
        // S s_9_7: cast zx s_9_5 -> bv
        let s_9_7: Bits = Bits::new(s_9_5 as u128, 1u16);
        // S s_9_8: cast reint s_9_6 -> u128
        let s_9_8: u128 = (s_9_6.value() as u128);
        // D s_9_9: size-of s_9_6
        let s_9_9: u16 = s_9_6.length();
        // S s_9_10: cast reint s_9_7 -> u128
        let s_9_10: u128 = (s_9_7.value() as u128);
        // D s_9_11: size-of s_9_7
        let s_9_11: u16 = s_9_7.length();
        // D s_9_12: lsl s_9_8 s_9_11
        let s_9_12: u128 = s_9_8 << s_9_11;
        // D s_9_13: or s_9_12 s_9_10
        let s_9_13: u128 = ((s_9_12) | (s_9_10));
        // D s_9_14: add s_9_9 s_9_11
        let s_9_14: u16 = (s_9_9 + s_9_11);
        // D s_9_15: create-bits s_9_13 s_9_14
        let s_9_15: Bits = Bits::new(s_9_13, s_9_14);
        // D s_9_16: cast reint s_9_15 -> u8
        let s_9_16: u8 = (s_9_15.value() as u8);
        // D s_9_17: cast zx s_9_16 -> bv
        let s_9_17: Bits = Bits::new(s_9_16 as u128, 2u16);
        // C s_9_18: const #0u : u8
        let s_9_18: u8 = 0;
        // C s_9_19: cast zx s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 2u16);
        // D s_9_20: cmp-eq s_9_17 s_9_19
        let s_9_20: bool = ((s_9_17) == (s_9_19));
        // N s_9_21: branch s_9_20 b12 b10
        if s_9_20 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#27718 <= s_10_0
        fn_state.gs_27718 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#27718:u8
        let s_11_0: bool = fn_state.gs_27718;
        // D s_11_1: write-var return_value <= s_11_0
        fn_state.return_value = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #424u : u32
        let s_12_0: u32 = 424;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call HaveAArch32EL(s_12_1)
        let s_12_2: bool = HaveAArch32EL(state, tracer, s_12_1);
        // N s_12_3: branch s_12_2 b15 b13
        if s_12_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call SCTLR_read__2(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_13_0);
        // S s_13_2: call _get_SCTLR_Type_M(s_13_1)
        let s_13_2: bool = u_get_SCTLR_Type_M(state, tracer, s_13_1);
        // D s_13_3: write-var ga#21397 <= s_13_2
        fn_state.ga_21397 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var ga#21397:u8
        let s_14_0: bool = fn_state.ga_21397;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var gs#27718 <= s_14_4
        fn_state.gs_27718 = s_14_4;
        // N s_14_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call SCTLR_NS_read(s_15_0)
        let s_15_1: ProductType700c18a878c5601b = SCTLR_NS_read(state, tracer, s_15_0);
        // S s_15_2: call _get_SCTLR_Type_M(s_15_1)
        let s_15_2: bool = u_get_SCTLR_Type_M(state, tracer, s_15_1);
        // D s_15_3: write-var ga#21397 <= s_15_2
        fn_state.ga_21397 = s_15_2;
        // N s_15_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call HaveAArch32EL(s_16_1)
        let s_16_2: bool = HaveAArch32EL(state, tracer, s_16_1);
        // N s_16_3: branch s_16_2 b19 b17
        if s_16_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call SCTLR_read__2(s_17_0)
        let s_17_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_17_0);
        // S s_17_2: call _get_SCTLR_Type_M(s_17_1)
        let s_17_2: bool = u_get_SCTLR_Type_M(state, tracer, s_17_1);
        // D s_17_3: write-var ga#21385 <= s_17_2
        fn_state.ga_21385 = s_17_2;
        // N s_17_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var ga#21385:u8
        let s_18_0: bool = fn_state.ga_21385;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var return_value <= s_18_4
        fn_state.return_value = s_18_4;
        // N s_18_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call SCTLR_NS_read(s_19_0)
        let s_19_1: ProductType700c18a878c5601b = SCTLR_NS_read(state, tracer, s_19_0);
        // S s_19_2: call _get_SCTLR_Type_M(s_19_1)
        let s_19_2: bool = u_get_SCTLR_Type_M(state, tracer, s_19_1);
        // D s_19_3: write-var ga#21385 <= s_19_2
        fn_state.ga_21385 = s_19_2;
        // N s_19_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #16456u : u32
        let s_20_0: u32 = 16456;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_SCTLR_Type_M(s_20_1)
        let s_20_2: bool = u_get_SCTLR_Type_M(state, tracer, s_20_1);
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // C s_20_4: const #1u : u8
        let s_20_4: bool = true;
        // C s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 1u16);
        // D s_20_6: cmp-eq s_20_3 s_20_5
        let s_20_6: bool = ((s_20_3) == (s_20_5));
        // D s_20_7: write-var return_value <= s_20_6
        fn_state.return_value = s_20_6;
        // N s_20_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call HSCTLR_read(s_21_0)
        let s_21_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_21_0);
        // S s_21_2: call _get_HSCTLR_Type_M(s_21_1)
        let s_21_2: bool = u_get_HSCTLR_Type_M(state, tracer, s_21_1);
        // S s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // S s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: write-var return_value <= s_21_6
        fn_state.return_value = s_21_6;
        // N s_21_8: jump b7
        return block_7(state, tracer, fn_state);
    }
}
