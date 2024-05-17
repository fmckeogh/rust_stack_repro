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
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_E2H::*;
use TranslationRegime::*;
use u_get_SCR_EL3_Type_ATA::*;
use u_get_SCTLR_EL3_Type_ATA::*;
use u_get_SCTLR_EL1_Type_ATA::*;
use u_get_HCR_EL2_Type_ATA::*;
use u_get_SCTLR_EL2_Type_ATA::*;
use u_get_SCTLR_EL2_Type_ATA0::*;
use u_get_SCTLR_EL1_Type_ATA0::*;
use Unreachable::*;
use common::*;
pub fn AArch64_AllocationTagAccessIsEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_15738: bool,
        gs_15740: bool,
        gs_15737: bool,
        gs_15739: bool,
        gs_15735: bool,
        return_value: bool,
        gs_15734: bool,
        regime: u32,
        gs_15736: bool,
        ga_11691: bool,
        ga_11695: bool,
        ga_11696: bool,
        el: u8,
    }
    let fn_state = FunctionState {
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #90704u : u32
        let s_0_0: u32 = 90704;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_SCR_EL3_Type_ATA(s_0_1)
        let s_0_2: bool = u_get_SCR_EL3_Type_ATA(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // D s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: branch s_0_6 b34 b1
        if s_0_6 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#15736 <= s_1_0
        fn_state.gs_15736 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#15736:u8
        let s_2_0: bool = fn_state.gs_15736;
        // N s_2_1: branch s_2_0 b33 b3
        if s_2_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #102552u : u32
        let s_3_0: u32 = 102552;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_HCR_EL2_Type_ATA(s_3_1)
        let s_3_2: bool = u_get_HCR_EL2_Type_ATA(state, tracer, s_3_1);
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // C s_3_4: const #0u : u8
        let s_3_4: bool = false;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 1u16);
        // D s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // N s_3_7: branch s_3_6 b29 b4
        if s_3_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#15738 <= s_4_0
        fn_state.gs_15738 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#15738:u8
        let s_5_0: bool = fn_state.gs_15738;
        // N s_5_1: branch s_5_0 b28 b6
        if s_5_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#15739 <= s_6_0
        fn_state.gs_15739 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#15739:u8
        let s_7_0: bool = fn_state.gs_15739;
        // N s_7_1: branch s_7_0 b27 b8
        if s_7_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#15740 <= s_8_0
        fn_state.gs_15740 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#15740:u8
        let s_9_0: bool = fn_state.gs_15740;
        // N s_9_1: branch s_9_0 b26 b10
        if s_9_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var el:u8
        let s_10_0: u8 = fn_state.el;
        // D s_10_1: call TranslationRegime(s_10_0)
        let s_10_1: u32 = TranslationRegime(state, tracer, s_10_0);
        // D s_10_2: write-var regime <= s_10_1
        fn_state.regime = s_10_1;
        // C s_10_3: const #0u : u32
        let s_10_3: u32 = 0;
        // D s_10_4: read-var regime:u32
        let s_10_4: u32 = fn_state.regime;
        // D s_10_5: cmp-eq s_10_3 s_10_4
        let s_10_5: bool = ((s_10_3) == (s_10_4));
        // D s_10_6: not s_10_5
        let s_10_6: bool = !s_10_5;
        // N s_10_7: branch s_10_6 b13 b11
        if s_10_6 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #17072u : u32
        let s_11_0: u32 = 17072;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_SCTLR_EL3_Type_ATA(s_11_1)
        let s_11_2: bool = u_get_SCTLR_EL3_Type_ATA(state, tracer, s_11_1);
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // D s_11_7: write-var return_value <= s_11_6
        fn_state.return_value = s_11_6;
        // N s_11_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var return_value:u8
        let s_12_0: bool = fn_state.return_value;
        // N s_12_1: return s_12_0
        return s_12_0;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #2u : u32
        let s_13_0: u32 = 2;
        // D s_13_1: read-var regime:u32
        let s_13_1: u32 = fn_state.regime;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #20784u : u32
        let s_14_0: u32 = 20784;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_SCTLR_EL2_Type_ATA(s_14_1)
        let s_14_2: bool = u_get_SCTLR_EL2_Type_ATA(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #1u : u8
        let s_14_4: bool = true;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // D s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // D s_14_7: write-var return_value <= s_14_6
        fn_state.return_value = s_14_6;
        // N s_14_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #3u : u32
        let s_15_0: u32 = 3;
        // D s_15_1: read-var regime:u32
        let s_15_1: u32 = fn_state.regime;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // N s_15_4: branch s_15_3 b20 b16
        if s_15_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var el:u8
        let s_16_0: u8 = fn_state.el;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 2u16);
        // C s_16_2: const #448u : u32
        let s_16_2: u32 = 448;
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
        // N s_16_6: branch s_16_5 b19 b17
        if s_16_5 {
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
        // C s_17_0: const #20784u : u32
        let s_17_0: u32 = 20784;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_SCTLR_EL2_Type_ATA(s_17_1)
        let s_17_2: bool = u_get_SCTLR_EL2_Type_ATA(state, tracer, s_17_1);
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // C s_17_4: const #1u : u8
        let s_17_4: bool = true;
        // C s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 1u16);
        // D s_17_6: cmp-eq s_17_3 s_17_5
        let s_17_6: bool = ((s_17_3) == (s_17_5));
        // D s_17_7: write-var ga#11691 <= s_17_6
        fn_state.ga_11691 = s_17_6;
        // N s_17_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var ga#11691:u8
        let s_18_0: bool = fn_state.ga_11691;
        // D s_18_1: write-var return_value <= s_18_0
        fn_state.return_value = s_18_0;
        // N s_18_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #20784u : u32
        let s_19_0: u32 = 20784;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_SCTLR_EL2_Type_ATA0(s_19_1)
        let s_19_2: bool = u_get_SCTLR_EL2_Type_ATA0(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #1u : u8
        let s_19_4: bool = true;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // D s_19_7: write-var ga#11691 <= s_19_6
        fn_state.ga_11691 = s_19_6;
        // N s_19_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #4u : u32
        let s_20_0: u32 = 4;
        // D s_20_1: read-var regime:u32
        let s_20_1: u32 = fn_state.regime;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b25 b21
        if s_20_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var el:u8
        let s_21_0: u8 = fn_state.el;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 2u16);
        // C s_21_2: const #448u : u32
        let s_21_2: u32 = 448;
        // D s_21_3: read-reg s_21_2:u8
        let s_21_3: u8 = {
            let value = state.read_register::<u8>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 2u16);
        // D s_21_5: cmp-eq s_21_1 s_21_4
        let s_21_5: bool = ((s_21_1) == (s_21_4));
        // N s_21_6: branch s_21_5 b24 b22
        if s_21_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #90272u : u32
        let s_22_0: u32 = 90272;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_SCTLR_EL1_Type_ATA(s_22_1)
        let s_22_2: bool = u_get_SCTLR_EL1_Type_ATA(state, tracer, s_22_1);
        // D s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // C s_22_4: const #1u : u8
        let s_22_4: bool = true;
        // C s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 1u16);
        // D s_22_6: cmp-eq s_22_3 s_22_5
        let s_22_6: bool = ((s_22_3) == (s_22_5));
        // D s_22_7: write-var ga#11695 <= s_22_6
        fn_state.ga_11695 = s_22_6;
        // N s_22_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var ga#11695:u8
        let s_23_0: bool = fn_state.ga_11695;
        // D s_23_1: write-var return_value <= s_23_0
        fn_state.return_value = s_23_0;
        // N s_23_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #90272u : u32
        let s_24_0: u32 = 90272;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_SCTLR_EL1_Type_ATA0(s_24_1)
        let s_24_2: bool = u_get_SCTLR_EL1_Type_ATA0(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // D s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var ga#11695 <= s_24_6
        fn_state.ga_11695 = s_24_6;
        // N s_24_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call Unreachable(s_25_0)
        let s_25_1: () = Unreachable(state, tracer, s_25_0);
        // D s_25_2: read-var ga#11696:u8
        let s_25_2: bool = fn_state.ga_11696;
        // D s_25_3: write-var return_value <= s_25_2
        fn_state.return_value = s_25_2;
        // N s_25_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
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
        // D s_27_21: write-var gs#15740 <= s_27_20
        fn_state.gs_15740 = s_27_20;
        // N s_27_22: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EL2Enabled(s_28_0)
        let s_28_1: bool = EL2Enabled(state, tracer, s_28_0);
        // D s_28_2: write-var gs#15739 <= s_28_1
        fn_state.gs_15739 = s_28_1;
        // N s_28_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var el:u8
        let s_29_0: u8 = fn_state.el;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 2u16);
        // C s_29_2: const #448u : u32
        let s_29_2: u32 = 448;
        // D s_29_3: read-reg s_29_2:u8
        let s_29_3: u8 = {
            let value = state.read_register::<u8>(s_29_2 as isize);
            tracer.read_register(s_29_2 as isize, value);
            value
        };
        // D s_29_4: cast zx s_29_3 -> bv
        let s_29_4: Bits = Bits::new(s_29_3 as u128, 2u16);
        // D s_29_5: cmp-eq s_29_1 s_29_4
        let s_29_5: bool = ((s_29_1) == (s_29_4));
        // N s_29_6: branch s_29_5 b32 b30
        if s_29_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var el:u8
        let s_30_0: u8 = fn_state.el;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 2u16);
        // C s_30_2: const #440u : u32
        let s_30_2: u32 = 440;
        // D s_30_3: read-reg s_30_2:u8
        let s_30_3: u8 = {
            let value = state.read_register::<u8>(s_30_2 as isize);
            tracer.read_register(s_30_2 as isize, value);
            value
        };
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 2u16);
        // D s_30_5: cmp-eq s_30_1 s_30_4
        let s_30_5: bool = ((s_30_1) == (s_30_4));
        // D s_30_6: write-var gs#15737 <= s_30_5
        fn_state.gs_15737 = s_30_5;
        // N s_30_7: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_31_0: read-var gs#15737:u8
        let s_31_0: bool = fn_state.gs_15737;
        // D s_31_1: write-var gs#15738 <= s_31_0
        fn_state.gs_15738 = s_31_0;
        // N s_31_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#15737 <= s_32_0
        fn_state.gs_15737 = s_32_0;
        // N s_32_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var return_value <= s_33_0
        fn_state.return_value = s_33_0;
        // N s_33_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_34_0: read-var el:u8
        let s_34_0: u8 = fn_state.el;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 2u16);
        // C s_34_2: const #448u : u32
        let s_34_2: u32 = 448;
        // D s_34_3: read-reg s_34_2:u8
        let s_34_3: u8 = {
            let value = state.read_register::<u8>(s_34_2 as isize);
            tracer.read_register(s_34_2 as isize, value);
            value
        };
        // D s_34_4: cast zx s_34_3 -> bv
        let s_34_4: Bits = Bits::new(s_34_3 as u128, 2u16);
        // D s_34_5: cmp-eq s_34_1 s_34_4
        let s_34_5: bool = ((s_34_1) == (s_34_4));
        // N s_34_6: branch s_34_5 b40 b35
        if s_34_5 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_35_0: read-var el:u8
        let s_35_0: u8 = fn_state.el;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 2u16);
        // C s_35_2: const #440u : u32
        let s_35_2: u32 = 440;
        // D s_35_3: read-reg s_35_2:u8
        let s_35_3: u8 = {
            let value = state.read_register::<u8>(s_35_2 as isize);
            tracer.read_register(s_35_2 as isize, value);
            value
        };
        // D s_35_4: cast zx s_35_3 -> bv
        let s_35_4: Bits = Bits::new(s_35_3 as u128, 2u16);
        // D s_35_5: cmp-eq s_35_1 s_35_4
        let s_35_5: bool = ((s_35_1) == (s_35_4));
        // N s_35_6: branch s_35_5 b39 b36
        if s_35_5 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var el:u8
        let s_36_0: u8 = fn_state.el;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 2u16);
        // C s_36_2: const #432u : u32
        let s_36_2: u32 = 432;
        // D s_36_3: read-reg s_36_2:u8
        let s_36_3: u8 = {
            let value = state.read_register::<u8>(s_36_2 as isize);
            tracer.read_register(s_36_2 as isize, value);
            value
        };
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 2u16);
        // D s_36_5: cmp-eq s_36_1 s_36_4
        let s_36_5: bool = ((s_36_1) == (s_36_4));
        // D s_36_6: write-var gs#15734 <= s_36_5
        fn_state.gs_15734 = s_36_5;
        // N s_36_7: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_37_0: read-var gs#15734:u8
        let s_37_0: bool = fn_state.gs_15734;
        // D s_37_1: write-var gs#15735 <= s_37_0
        fn_state.gs_15735 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_38_0: read-var gs#15735:u8
        let s_38_0: bool = fn_state.gs_15735;
        // D s_38_1: write-var gs#15736 <= s_38_0
        fn_state.gs_15736 = s_38_0;
        // N s_38_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#15734 <= s_39_0
        fn_state.gs_15734 = s_39_0;
        // N s_39_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#15735 <= s_40_0
        fn_state.gs_15735 = s_40_0;
        // N s_40_2: jump b38
        return block_38(state, tracer, fn_state);
    }
}
