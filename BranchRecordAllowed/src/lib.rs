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
use u_get_MDCR_EL3_Type_E3BREW::*;
use HaveBRBEv1p1::*;
use u_get_BRBCR_EL2_Type_E2BRE::*;
use u_get_BRBCR_EL1_Type_E0BRE::*;
use ELUsingAArch32::*;
use u_get_BRBFCR_EL1_Type_PAUSED::*;
use u_get_MDCR_EL3_Type_SBRBE::*;
use CurrentSecurityState::*;
use u_get_MDCR_EL3_Type_E3BREC::*;
use u_get_BRBCR_EL1_Type_E1BRE::*;
use u_get_BRBCR_EL2_Type_E0HBRE::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn BranchRecordAllowed<T: Tracer>(state: &mut State, tracer: &T, el: u8) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_2979: bool,
        gs_2981: bool,
        return_value: bool,
        gs_2980: bool,
        ga_1797: bool,
        gs_2990: bool,
        gs_2978: bool,
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
        // D s_0_0: read-var el:u8
        let s_0_0: u8 = fn_state.el;
        // D s_0_1: call ELUsingAArch32(s_0_0)
        let s_0_1: bool = ELUsingAArch32(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b34 b1
        if s_0_1 {
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
        // C s_1_0: const #16536u : u32
        let s_1_0: u32 = 16536;
        // D s_1_1: read-reg s_1_0:struct
        let s_1_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call _get_BRBFCR_EL1_Type_PAUSED(s_1_1)
        let s_1_2: bool = u_get_BRBFCR_EL1_Type_PAUSED(state, tracer, s_1_1);
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // C s_1_4: const #1u : u8
        let s_1_4: bool = true;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // D s_1_6: cmp-eq s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) == (s_1_5));
        // N s_1_7: branch s_1_6 b33 b2
        if s_1_6 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var el:u8
        let s_2_0: u8 = fn_state.el;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #424u : u32
        let s_2_2: u32 = 424;
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
        // N s_2_6: branch s_2_5 b32 b3
        if s_2_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#2978 <= s_3_0
        fn_state.gs_2978 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#2978:u8
        let s_4_0: bool = fn_state.gs_2978;
        // N s_4_1: branch s_4_0 b31 b5
        if s_4_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #424u : u32
        let s_5_0: u32 = 424;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // D s_5_3: cmp-lt s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) < (s_5_2));
        // N s_5_4: branch s_5_3 b24 b6
        if s_5_3 {
            return block_24(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#2981 <= s_6_0
        fn_state.gs_2981 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#2981:u8
        let s_7_0: bool = fn_state.gs_2981;
        // N s_7_1: branch s_7_0 b23 b8
        if s_7_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var el:u8
        let s_8_0: u8 = fn_state.el;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #424u : u32
        let s_8_2: u32 = 424;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 2u16);
        // D s_8_5: cmp-eq s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) == (s_8_4));
        // D s_8_6: not s_8_5
        let s_8_6: bool = !s_8_5;
        // N s_8_7: branch s_8_6 b11 b9
        if s_8_6 {
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
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var return_value:u8
        let s_10_0: bool = fn_state.return_value;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var el:u8
        let s_11_0: u8 = fn_state.el;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #432u : u32
        let s_11_2: u32 = 432;
        // D s_11_3: read-reg s_11_2:u8
        let s_11_3: u8 = {
            let value = state.read_register::<u8>(s_11_2 as isize);
            tracer.read_register(s_11_2 as isize, value);
            value
        };
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 2u16);
        // D s_11_5: cmp-eq s_11_1 s_11_4
        let s_11_5: bool = ((s_11_1) == (s_11_4));
        // D s_11_6: not s_11_5
        let s_11_6: bool = !s_11_5;
        // N s_11_7: branch s_11_6 b13 b12
        if s_11_6 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #18272u : u32
        let s_12_0: u32 = 18272;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_BRBCR_EL2_Type_E2BRE(s_12_1)
        let s_12_2: bool = u_get_BRBCR_EL2_Type_E2BRE(state, tracer, s_12_1);
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #1u : u8
        let s_12_4: bool = true;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // D s_12_7: write-var return_value <= s_12_6
        fn_state.return_value = s_12_6;
        // N s_12_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var el:u8
        let s_13_0: u8 = fn_state.el;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #440u : u32
        let s_13_2: u32 = 440;
        // D s_13_3: read-reg s_13_2:u8
        let s_13_3: u8 = {
            let value = state.read_register::<u8>(s_13_2 as isize);
            tracer.read_register(s_13_2 as isize, value);
            value
        };
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 2u16);
        // D s_13_5: cmp-eq s_13_1 s_13_4
        let s_13_5: bool = ((s_13_1) == (s_13_4));
        // D s_13_6: not s_13_5
        let s_13_6: bool = !s_13_5;
        // N s_13_7: branch s_13_6 b15 b14
        if s_13_6 {
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
        // C s_14_0: const #90640u : u32
        let s_14_0: u32 = 90640;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_BRBCR_EL1_Type_E1BRE(s_14_1)
        let s_14_2: bool = u_get_BRBCR_EL1_Type_E1BRE(state, tracer, s_14_1);
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
        // N s_14_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var el:u8
        let s_15_0: u8 = fn_state.el;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // C s_15_2: const #448u : u32
        let s_15_2: u32 = 448;
        // D s_15_3: read-reg s_15_2:u8
        let s_15_3: u8 = {
            let value = state.read_register::<u8>(s_15_2 as isize);
            tracer.read_register(s_15_2 as isize, value);
            value
        };
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 2u16);
        // D s_15_5: cmp-eq s_15_1 s_15_4
        let s_15_5: bool = ((s_15_1) == (s_15_4));
        // D s_15_6: not s_15_5
        let s_15_6: bool = !s_15_5;
        // N s_15_7: branch s_15_6 b22 b16
        if s_15_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call EL2Enabled(s_16_0)
        let s_16_1: bool = EL2Enabled(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b21 b17
        if s_16_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#2990 <= s_17_0
        fn_state.gs_2990 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#2990:u8
        let s_18_0: bool = fn_state.gs_2990;
        // N s_18_1: branch s_18_0 b20 b19
        if s_18_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #90640u : u32
        let s_19_0: u32 = 90640;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_BRBCR_EL1_Type_E0BRE(s_19_1)
        let s_19_2: bool = u_get_BRBCR_EL1_Type_E0BRE(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #1u : u8
        let s_19_4: bool = true;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // D s_19_7: write-var return_value <= s_19_6
        fn_state.return_value = s_19_6;
        // N s_19_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #18272u : u32
        let s_20_0: u32 = 18272;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_BRBCR_EL2_Type_E0HBRE(s_20_1)
        let s_20_2: bool = u_get_BRBCR_EL2_Type_E0HBRE(state, tracer, s_20_1);
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
        // N s_20_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #102552u : u32
        let s_21_0: u32 = 102552;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_HCR_EL2_Type_TGE(s_21_1)
        let s_21_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: write-var gs#2990 <= s_21_6
        fn_state.gs_2990 = s_21_6;
        // N s_21_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var ga#1797:u8
        let s_22_0: bool = fn_state.ga_1797;
        // D s_22_1: write-var return_value <= s_22_0
        fn_state.return_value = s_22_0;
        // N s_22_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var return_value <= s_23_0
        fn_state.return_value = s_23_0;
        // N s_23_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #22712u : u32
        let s_24_0: u32 = 22712;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_MDCR_EL3_Type_SBRBE(s_24_1)
        let s_24_2: u8 = u_get_MDCR_EL3_Type_SBRBE(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 2u16);
        // C s_24_4: const #0u : u8
        let s_24_4: u8 = 0;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 2u16);
        // D s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // N s_24_7: branch s_24_6 b30 b25
        if s_24_6 {
            return block_30(state, tracer, fn_state);
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
        // S s_25_1: call CurrentSecurityState(s_25_0)
        let s_25_1: u32 = CurrentSecurityState(state, tracer, s_25_0);
        // C s_25_2: const #3u : u32
        let s_25_2: u32 = 3;
        // S s_25_3: cmp-eq s_25_1 s_25_2
        let s_25_3: bool = ((s_25_1) == (s_25_2));
        // N s_25_4: branch s_25_3 b29 b26
        if s_25_3 {
            return block_29(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#2979 <= s_26_0
        fn_state.gs_2979 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var gs#2979:u8
        let s_27_0: bool = fn_state.gs_2979;
        // D s_27_1: write-var gs#2980 <= s_27_0
        fn_state.gs_2980 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var gs#2980:u8
        let s_28_0: bool = fn_state.gs_2980;
        // D s_28_1: write-var gs#2981 <= s_28_0
        fn_state.gs_2981 = s_28_0;
        // N s_28_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #22712u : u32
        let s_29_0: u32 = 22712;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_MDCR_EL3_Type_SBRBE(s_29_1)
        let s_29_2: u8 = u_get_MDCR_EL3_Type_SBRBE(state, tracer, s_29_1);
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 2u16);
        // C s_29_4: const #1u : u8
        let s_29_4: u8 = 1;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 2u16);
        // D s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // D s_29_7: write-var gs#2979 <= s_29_6
        fn_state.gs_2979 = s_29_6;
        // N s_29_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#2980 <= s_30_0
        fn_state.gs_2980 = s_30_0;
        // N s_30_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #22712u : u32
        let s_31_0: u32 = 22712;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_MDCR_EL3_Type_E3BREC(s_31_1)
        let s_31_2: bool = u_get_MDCR_EL3_Type_E3BREC(state, tracer, s_31_1);
        // C s_31_3: const #22712u : u32
        let s_31_3: u32 = 22712;
        // D s_31_4: read-reg s_31_3:struct
        let s_31_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_3 as isize);
            tracer.read_register(s_31_3 as isize, value);
            value
        };
        // D s_31_5: call _get_MDCR_EL3_Type_E3BREW(s_31_4)
        let s_31_5: bool = u_get_MDCR_EL3_Type_E3BREW(state, tracer, s_31_4);
        // D s_31_6: cast zx s_31_2 -> bv
        let s_31_6: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_7: cast zx s_31_5 -> bv
        let s_31_7: Bits = Bits::new(s_31_5 as u128, 1u16);
        // D s_31_8: cmp-ne s_31_6 s_31_7
        let s_31_8: bool = ((s_31_6) != (s_31_7));
        // D s_31_9: write-var return_value <= s_31_8
        fn_state.return_value = s_31_8;
        // N s_31_10: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call HaveBRBEv1p1(s_32_0)
        let s_32_1: bool = HaveBRBEv1p1(state, tracer, s_32_0);
        // D s_32_2: write-var gs#2978 <= s_32_1
        fn_state.gs_2978 = s_32_1;
        // N s_32_3: jump b4
        return block_4(state, tracer, fn_state);
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
        // N s_33_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var return_value <= s_34_0
        fn_state.return_value = s_34_0;
        // N s_34_2: jump b10
        return block_10(state, tracer, fn_state);
    }
}
