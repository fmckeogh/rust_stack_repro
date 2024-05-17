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
use HaveSecureEL2Ext::*;
use HCR_read::*;
use ELUsingAArch32::*;
use HDCR_read::*;
use u_get_HDCR_Type_TDE::*;
use HaveAArch64::*;
use u_get_MDCR_EL2_Type_TDE::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_SCR_EL3_Type_EEL2::*;
use u_get_HCR_Type_TGE::*;
use common::*;
pub fn DebugTargetFrom<T: Tracer>(state: &mut State, tracer: &T, from_state: u32) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        target: u8,
        gs_5079: bool,
        gs_5077: bool,
        gs_5086: bool,
        gs_5078: bool,
        gs_5080: bool,
        gs_5083: bool,
        gs_5085: bool,
        gs_5082: bool,
        from_state: u32,
    }
    let fn_state = FunctionState {
        from_state,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #432u : u32
        let s_0_0: u32 = 432;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // N s_0_4: branch s_0_3 b25 b1
        if s_0_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#5080 <= s_1_0
        fn_state.gs_5080 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var gs#5080:u8
        let s_2_0: bool = fn_state.gs_5080;
        // N s_2_1: branch s_2_0 b16 b3
        if s_2_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var route_to_el2 <= s_3_0
        fn_state.route_to_el2 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var route_to_el2:u8
        let s_4_0: bool = fn_state.route_to_el2;
        // N s_4_1: branch s_4_0 b15 b5
        if s_4_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
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
        // N s_5_4: branch s_5_3 b14 b6
        if s_5_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#5085 <= s_6_0
        fn_state.gs_5085 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_7_0: read-var gs#5085:u8
        let s_7_0: bool = fn_state.gs_5085;
        // N s_7_1: branch s_7_0 b13 b8
        if s_7_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#5086 <= s_8_0
        fn_state.gs_5086 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_9_0: read-var gs#5086:u8
        let s_9_0: bool = fn_state.gs_5086;
        // N s_9_1: branch s_9_0 b12 b10
        if s_9_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_10_0: const #440u : u32
        let s_10_0: u32 = 440;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: write-var target <= s_10_1
        fn_state.target = s_10_1;
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_11_0: read-var target:u8
        let s_11_0: u8 = fn_state.target;
        // N s_11_1: return s_11_0
        return s_11_0;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_12_0: const #424u : u32
        let s_12_0: u32 = 424;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: write-var target <= s_12_1
        fn_state.target = s_12_1;
        // N s_12_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_13_0: read-var from_state:u32
        let s_13_0: u32 = fn_state.from_state;
        // C s_13_1: const #3u : u32
        let s_13_1: u32 = 3;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: write-var gs#5086 <= s_13_2
        fn_state.gs_5086 = s_13_2;
        // N s_13_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveAArch64(s_14_0)
        let s_14_1: bool = HaveAArch64(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // D s_14_3: write-var gs#5085 <= s_14_2
        fn_state.gs_5085 = s_14_2;
        // N s_14_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_15_0: const #432u : u32
        let s_15_0: u32 = 432;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: write-var target <= s_15_1
        fn_state.target = s_15_1;
        // N s_15_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_16_0: const #432u : u32
        let s_16_0: u32 = 432;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call ELUsingAArch32(s_16_1)
        let s_16_2: bool = ELUsingAArch32(state, tracer, s_16_1);
        // N s_16_3: branch s_16_2 b21 b17
        if s_16_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_17_0: const #104880u : u32
        let s_17_0: u32 = 104880;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_MDCR_EL2_Type_TDE(s_17_1)
        let s_17_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_17_1);
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // C s_17_4: const #1u : u8
        let s_17_4: bool = true;
        // C s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 1u16);
        // D s_17_6: cmp-eq s_17_3 s_17_5
        let s_17_6: bool = ((s_17_3) == (s_17_5));
        // N s_17_7: branch s_17_6 b20 b18
        if s_17_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_18_0: const #102552u : u32
        let s_18_0: u32 = 102552;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_HCR_EL2_Type_TGE(s_18_1)
        let s_18_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // D s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // D s_18_7: write-var gs#5082 <= s_18_6
        fn_state.gs_5082 = s_18_6;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_19_0: read-var gs#5082:u8
        let s_19_0: bool = fn_state.gs_5082;
        // D s_19_1: write-var route_to_el2 <= s_19_0
        fn_state.route_to_el2 = s_19_0;
        // N s_19_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#5082 <= s_20_0
        fn_state.gs_5082 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call HDCR_read(s_21_0)
        let s_21_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_21_0);
        // S s_21_2: call _get_HDCR_Type_TDE(s_21_1)
        let s_21_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_21_1);
        // S s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // S s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // N s_21_7: branch s_21_6 b24 b22
        if s_21_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call HCR_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_22_0);
        // S s_22_2: call _get_HCR_Type_TGE(s_22_1)
        let s_22_2: bool = u_get_HCR_Type_TGE(state, tracer, s_22_1);
        // S s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // C s_22_4: const #1u : u8
        let s_22_4: bool = true;
        // C s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 1u16);
        // S s_22_6: cmp-eq s_22_3 s_22_5
        let s_22_6: bool = ((s_22_3) == (s_22_5));
        // D s_22_7: write-var gs#5083 <= s_22_6
        fn_state.gs_5083 = s_22_6;
        // N s_22_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_23_0: read-var gs#5083:u8
        let s_23_0: bool = fn_state.gs_5083;
        // D s_23_1: write-var route_to_el2 <= s_23_0
        fn_state.route_to_el2 = s_23_0;
        // N s_23_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#5083 <= s_24_0
        fn_state.gs_5083 = s_24_0;
        // N s_24_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_25_0: read-var from_state:u32
        let s_25_0: u32 = fn_state.from_state;
        // C s_25_1: const #3u : u32
        let s_25_1: u32 = 3;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // N s_25_3: branch s_25_2 b34 b26
        if s_25_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call HaveSecureEL2Ext(s_26_0)
        let s_26_1: bool = HaveSecureEL2Ext(state, tracer, s_26_0);
        // N s_26_2: branch s_26_1 b30 b27
        if s_26_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#5078 <= s_27_0
        fn_state.gs_5078 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_28_0: read-var gs#5078:u8
        let s_28_0: bool = fn_state.gs_5078;
        // D s_28_1: write-var gs#5079 <= s_28_0
        fn_state.gs_5079 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_29_0: read-var gs#5079:u8
        let s_29_0: bool = fn_state.gs_5079;
        // D s_29_1: write-var gs#5080 <= s_29_0
        fn_state.gs_5080 = s_29_0;
        // N s_29_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
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
    ) -> u8 {
        // C s_31_0: const #90704u : u32
        let s_31_0: u32 = 90704;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_SCR_EL3_Type_EEL2(s_31_1)
        let s_31_2: bool = u_get_SCR_EL3_Type_EEL2(state, tracer, s_31_1);
        // D s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // C s_31_4: const #1u : u8
        let s_31_4: bool = true;
        // C s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 1u16);
        // D s_31_6: cmp-eq s_31_3 s_31_5
        let s_31_6: bool = ((s_31_3) == (s_31_5));
        // D s_31_7: write-var gs#5077 <= s_31_6
        fn_state.gs_5077 = s_31_6;
        // N s_31_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_32_0: read-var gs#5077:u8
        let s_32_0: bool = fn_state.gs_5077;
        // D s_32_1: write-var gs#5078 <= s_32_0
        fn_state.gs_5078 = s_32_0;
        // N s_32_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#5077 <= s_33_0
        fn_state.gs_5077 = s_33_0;
        // N s_33_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#5079 <= s_34_0
        fn_state.gs_5079 = s_34_0;
        // N s_34_2: jump b29
        return block_29(state, tracer, fn_state);
    }
}
