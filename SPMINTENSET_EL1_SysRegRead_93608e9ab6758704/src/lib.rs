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
use SPMINTENSET_EL1_read::*;
use Halted::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use u_get_MDCR_EL2_Type_EnSPM::*;
use AArch64_SystemAccessTrap::*;
use u_get_SPMSELR_EL0_Type_SYSPMUSEL::*;
use u__get_selected_SPMACCESSR_EL2_field::*;
use X_set::*;
use u_get_HDFGRTR2_EL2_Type_nSPMINTEN::*;
use u_get_SCR_EL3_Type_FGTEn2::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_MDCR_EL3_Type_EnPM2::*;
use EDSCR_read::*;
use u__get_selected_SPMACCESSR_EL3_field::*;
use common::*;
pub fn SPMINTENSET_EL1_SysRegRead_93608e9ab6758704<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_73832: bool,
        u__MDCR_EL3_EnPM2: bool,
        gs_73853: bool,
        gs_73847: bool,
        gs_73834: bool,
        gs_73844: bool,
        u__HDFGRTR2_EL2_nSPMINTEN: bool,
        gs_73849: bool,
        gs_73843: bool,
        gs_73846: bool,
        gs_73837: bool,
        gs_73845: bool,
        u__SCR_EL3_FGTEn2: bool,
        u__MDCR_EL2_EnSPM: bool,
        gs_73830: bool,
        gs_73831: bool,
        gs_73852: bool,
        gs_73839: bool,
        gs_73842: bool,
        gs_73828: bool,
        gs_73829: bool,
        gs_73860: bool,
        gs_73850: bool,
        gs_73854: bool,
        gs_73836: bool,
        gs_73851: bool,
        gs_73840: bool,
        gs_73848: bool,
        u__PSTATE_EL: u8,
        gs_73833: bool,
        gs_73859: bool,
        gs_73835: bool,
        gs_73841: bool,
        gs_73855: bool,
        gs_73856: bool,
        gs_73857: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
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
        // D s_0_2: write-var __PSTATE_EL <= s_0_1
        fn_state.u__PSTATE_EL = s_0_1;
        // C s_0_3: const #22712u : u32
        let s_0_3: u32 = 22712;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL3_Type_EnPM2(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_EnPM2(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_EnPM2 <= s_0_5
        fn_state.u__MDCR_EL3_EnPM2 = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_FGTEn2(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_FGTEn2(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_FGTEn2 <= s_0_9
        fn_state.u__SCR_EL3_FGTEn2 = s_0_9;
        // C s_0_11: const #101224u : u32
        let s_0_11: u32 = 101224;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HDFGRTR2_EL2_Type_nSPMINTEN(s_0_12)
        let s_0_13: bool = u_get_HDFGRTR2_EL2_Type_nSPMINTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __HDFGRTR2_EL2_nSPMINTEN <= s_0_13
        fn_state.u__HDFGRTR2_EL2_nSPMINTEN = s_0_13;
        // C s_0_15: const #104880u : u32
        let s_0_15: u32 = 104880;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_MDCR_EL2_Type_EnSPM(s_0_16)
        let s_0_17: bool = u_get_MDCR_EL2_Type_EnSPM(state, tracer, s_0_16);
        // D s_0_18: write-var __MDCR_EL2_EnSPM <= s_0_17
        fn_state.u__MDCR_EL2_EnSPM = s_0_17;
        // D s_0_19: read-var __PSTATE_EL:u8
        let s_0_19: u8 = fn_state.u__PSTATE_EL;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // C s_0_21: const #448u : u32
        let s_0_21: u32 = 448;
        // D s_0_22: read-reg s_0_21:u8
        let s_0_22: u8 = {
            let value = state.read_register::<u8>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 2u16);
        // D s_0_24: cmp-eq s_0_20 s_0_23
        let s_0_24: bool = ((s_0_20) == (s_0_23));
        // N s_0_25: branch s_0_24 b133 b1
        if s_0_24 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var __PSTATE_EL:u8
        let s_1_0: u8 = fn_state.u__PSTATE_EL;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #440u : u32
        let s_1_2: u32 = 440;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // N s_1_6: branch s_1_5 b55 b2
        if s_1_5 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var __PSTATE_EL:u8
        let s_2_0: u8 = fn_state.u__PSTATE_EL;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #432u : u32
        let s_2_2: u32 = 432;
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
        // N s_2_6: branch s_2_5 b6 b3
        if s_2_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var __PSTATE_EL:u8
        let s_3_0: u8 = fn_state.u__PSTATE_EL;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #424u : u32
        let s_3_2: u32 = 424;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // C s_5_1: const #16552u : u32
        let s_5_1: u32 = 16552;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_5_2)
        let s_5_3: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_5_2);
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 6u16);
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (s_5_4.value() as i128);
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: call SPMINTENSET_EL1_read(s_5_7)
        let s_5_8: u64 = SPMINTENSET_EL1_read(state, tracer, s_5_7);
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 64u16);
        // D s_5_10: read-var t:i
        let s_5_10: i128 = fn_state.t;
        // D s_5_11: call X_set(s_5_10, s_5_0, s_5_9)
        let s_5_11: () = X_set(state, tracer, s_5_10, s_5_0, s_5_9);
        // N s_5_12: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Halted(s_6_0)
        let s_6_1: bool = Halted(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b54 b7
        if s_6_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#73828 <= s_7_0
        fn_state.gs_73828 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#73828:u8
        let s_8_0: bool = fn_state.gs_73828;
        // N s_8_1: branch s_8_0 b53 b9
        if s_8_0 {
            return block_53(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#73829 <= s_9_0
        fn_state.gs_73829 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#73829:u8
        let s_10_0: bool = fn_state.gs_73829;
        // N s_10_1: branch s_10_0 b52 b11
        if s_10_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#73830 <= s_11_0
        fn_state.gs_73830 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#73830:u8
        let s_12_0: bool = fn_state.gs_73830;
        // N s_12_1: branch s_12_0 b51 b13
        if s_12_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#73831 <= s_13_0
        fn_state.gs_73831 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#73831:u8
        let s_14_0: bool = fn_state.gs_73831;
        // N s_14_1: branch s_14_0 b50 b15
        if s_14_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call Halted(s_15_0)
        let s_15_1: bool = Halted(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b49 b16
        if s_15_1 {
            return block_49(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#73832 <= s_16_0
        fn_state.gs_73832 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#73832:u8
        let s_17_0: bool = fn_state.gs_73832;
        // N s_17_1: branch s_17_0 b48 b18
        if s_17_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#73833 <= s_18_0
        fn_state.gs_73833 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#73833:u8
        let s_19_0: bool = fn_state.gs_73833;
        // N s_19_1: branch s_19_0 b47 b20
        if s_19_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#73834 <= s_20_0
        fn_state.gs_73834 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#73834:u8
        let s_21_0: bool = fn_state.gs_73834;
        // N s_21_1: branch s_21_0 b46 b22
        if s_21_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#73835 <= s_22_0
        fn_state.gs_73835 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#73835:u8
        let s_23_0: bool = fn_state.gs_73835;
        // N s_23_1: branch s_23_0 b45 b24
        if s_23_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #424u : u32
        let s_24_0: u32 = 424;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // C s_24_2: const #2u : u8
        let s_24_2: u8 = 2;
        // D s_24_3: cmp-lt s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) < (s_24_2));
        // N s_24_4: branch s_24_3 b44 b25
        if s_24_3 {
            return block_44(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#73836 <= s_25_0
        fn_state.gs_73836 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#73836:u8
        let s_26_0: bool = fn_state.gs_73836;
        // N s_26_1: branch s_26_0 b38 b27
        if s_26_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #424u : u32
        let s_27_0: u32 = 424;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // C s_27_2: const #2u : u8
        let s_27_2: u8 = 2;
        // D s_27_3: cmp-lt s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) < (s_27_2));
        // N s_27_4: branch s_27_3 b37 b28
        if s_27_3 {
            return block_37(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#73837 <= s_28_0
        fn_state.gs_73837 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#73837:u8
        let s_29_0: bool = fn_state.gs_73837;
        // N s_29_1: branch s_29_0 b31 b30
        if s_29_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #64s : i64
        let s_30_0: i64 = 64;
        // C s_30_1: const #16552u : u32
        let s_30_1: u32 = 16552;
        // D s_30_2: read-reg s_30_1:struct
        let s_30_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_1 as isize);
            tracer.read_register(s_30_1 as isize, value);
            value
        };
        // D s_30_3: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_30_2)
        let s_30_3: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_30_2);
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 6u16);
        // D s_30_5: cast zx s_30_4 -> i
        let s_30_5: i128 = (s_30_4.value() as i128);
        // D s_30_6: cast reint s_30_5 -> i64
        let s_30_6: i64 = (s_30_5 as i64);
        // D s_30_7: cast zx s_30_6 -> i
        let s_30_7: i128 = (i128::try_from(s_30_6).unwrap());
        // D s_30_8: call SPMINTENSET_EL1_read(s_30_7)
        let s_30_8: u64 = SPMINTENSET_EL1_read(state, tracer, s_30_7);
        // D s_30_9: cast zx s_30_8 -> bv
        let s_30_9: Bits = Bits::new(s_30_8 as u128, 64u16);
        // D s_30_10: read-var t:i
        let s_30_10: i128 = fn_state.t;
        // D s_30_11: call X_set(s_30_10, s_30_0, s_30_9)
        let s_30_11: () = X_set(state, tracer, s_30_10, s_30_0, s_30_9);
        // N s_30_12: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call Halted(s_31_0)
        let s_31_1: bool = Halted(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b36 b32
        if s_31_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#73839 <= s_32_0
        fn_state.gs_73839 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#73839:u8
        let s_33_0: bool = fn_state.gs_73839;
        // N s_33_1: branch s_33_0 b35 b34
        if s_33_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #24u : u8
        let s_34_0: u8 = 24;
        // C s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 8u16);
        // C s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (s_34_1.value() as i128);
        // C s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: cast zx s_34_3 -> i
        let s_34_4: i128 = (i128::try_from(s_34_3).unwrap());
        // C s_34_5: const #424u : u32
        let s_34_5: u32 = 424;
        // D s_34_6: read-reg s_34_5:u8
        let s_34_6: u8 = {
            let value = state.read_register::<u8>(s_34_5 as isize);
            tracer.read_register(s_34_5 as isize, value);
            value
        };
        // D s_34_7: call AArch64_SystemAccessTrap(s_34_6, s_34_4)
        let s_34_7: () = AArch64_SystemAccessTrap(state, tracer, s_34_6, s_34_4);
        // N s_34_8: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call EDSCR_read(s_36_0)
        let s_36_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_36_0);
        // S s_36_2: call _get_EDSCR_Type_SDD(s_36_1)
        let s_36_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_36_1);
        // S s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // S s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // D s_36_7: write-var gs#73839 <= s_36_6
        fn_state.gs_73839 = s_36_6;
        // N s_36_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call __get_selected_SPMACCESSR_EL3_field(s_37_0)
        let s_37_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_37_0);
        // S s_37_2: cast zx s_37_1 -> bv
        let s_37_2: Bits = Bits::new(s_37_1 as u128, 2u16);
        // C s_37_3: const #0u : u8
        let s_37_3: u8 = 0;
        // C s_37_4: cast zx s_37_3 -> bv
        let s_37_4: Bits = Bits::new(s_37_3 as u128, 2u16);
        // S s_37_5: cmp-eq s_37_2 s_37_4
        let s_37_5: bool = ((s_37_2) == (s_37_4));
        // D s_37_6: write-var gs#73837 <= s_37_5
        fn_state.gs_73837 = s_37_5;
        // N s_37_7: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call Halted(s_38_0)
        let s_38_1: bool = Halted(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b43 b39
        if s_38_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#73840 <= s_39_0
        fn_state.gs_73840 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#73840:u8
        let s_40_0: bool = fn_state.gs_73840;
        // N s_40_1: branch s_40_0 b42 b41
        if s_40_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #24u : u8
        let s_41_0: u8 = 24;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 8u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // C s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // C s_41_5: const #424u : u32
        let s_41_5: u32 = 424;
        // D s_41_6: read-reg s_41_5:u8
        let s_41_6: u8 = {
            let value = state.read_register::<u8>(s_41_5 as isize);
            tracer.read_register(s_41_5 as isize, value);
            value
        };
        // D s_41_7: call AArch64_SystemAccessTrap(s_41_6, s_41_4)
        let s_41_7: () = AArch64_SystemAccessTrap(state, tracer, s_41_6, s_41_4);
        // N s_41_8: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: panic
        panic!("{:?}", ());
        // N s_42_1: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EDSCR_read(s_43_0)
        let s_43_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_43_0);
        // S s_43_2: call _get_EDSCR_Type_SDD(s_43_1)
        let s_43_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_43_1);
        // S s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // C s_43_4: const #1u : u8
        let s_43_4: bool = true;
        // C s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // S s_43_6: cmp-eq s_43_3 s_43_5
        let s_43_6: bool = ((s_43_3) == (s_43_5));
        // D s_43_7: write-var gs#73840 <= s_43_6
        fn_state.gs_73840 = s_43_6;
        // N s_43_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __MDCR_EL3_EnPM2:u8
        let s_44_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #0u : u8
        let s_44_2: bool = false;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#73836 <= s_44_4
        fn_state.gs_73836 = s_44_4;
        // N s_44_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: panic
        panic!("{:?}", ());
        // N s_45_1: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call __get_selected_SPMACCESSR_EL3_field(s_46_0)
        let s_46_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_46_0);
        // S s_46_2: cast zx s_46_1 -> bv
        let s_46_2: Bits = Bits::new(s_46_1 as u128, 2u16);
        // C s_46_3: const #0u : u8
        let s_46_3: u8 = 0;
        // C s_46_4: cast zx s_46_3 -> bv
        let s_46_4: Bits = Bits::new(s_46_3 as u128, 2u16);
        // S s_46_5: cmp-eq s_46_2 s_46_4
        let s_46_5: bool = ((s_46_2) == (s_46_4));
        // D s_46_6: write-var gs#73835 <= s_46_5
        fn_state.gs_73835 = s_46_5;
        // N s_46_7: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_47_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_47_1: call __IMPDEF_boolean(s_47_0)
        let s_47_1: bool = u__IMPDEF_boolean(state, tracer, s_47_0);
        // D s_47_2: write-var gs#73834 <= s_47_1
        fn_state.gs_73834 = s_47_1;
        // N s_47_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EDSCR_read(s_48_0)
        let s_48_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_48_0);
        // S s_48_2: call _get_EDSCR_Type_SDD(s_48_1)
        let s_48_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_48_1);
        // S s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // C s_48_4: const #1u : u8
        let s_48_4: bool = true;
        // C s_48_5: cast zx s_48_4 -> bv
        let s_48_5: Bits = Bits::new(s_48_4 as u128, 1u16);
        // S s_48_6: cmp-eq s_48_3 s_48_5
        let s_48_6: bool = ((s_48_3) == (s_48_5));
        // D s_48_7: write-var gs#73833 <= s_48_6
        fn_state.gs_73833 = s_48_6;
        // N s_48_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #424u : u32
        let s_49_0: u32 = 424;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // C s_49_2: const #2u : u8
        let s_49_2: u8 = 2;
        // D s_49_3: cmp-lt s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) < (s_49_2));
        // D s_49_4: write-var gs#73832 <= s_49_3
        fn_state.gs_73832 = s_49_3;
        // N s_49_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: panic
        panic!("{:?}", ());
        // N s_50_1: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __MDCR_EL3_EnPM2:u8
        let s_51_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #0u : u8
        let s_51_2: bool = false;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#73831 <= s_51_4
        fn_state.gs_73831 = s_51_4;
        // N s_51_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_52_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_52_1: call __IMPDEF_boolean(s_52_0)
        let s_52_1: bool = u__IMPDEF_boolean(state, tracer, s_52_0);
        // D s_52_2: write-var gs#73830 <= s_52_1
        fn_state.gs_73830 = s_52_1;
        // N s_52_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EDSCR_read(s_53_0)
        let s_53_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_53_0);
        // S s_53_2: call _get_EDSCR_Type_SDD(s_53_1)
        let s_53_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_53_1);
        // S s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // C s_53_4: const #1u : u8
        let s_53_4: bool = true;
        // C s_53_5: cast zx s_53_4 -> bv
        let s_53_5: Bits = Bits::new(s_53_4 as u128, 1u16);
        // S s_53_6: cmp-eq s_53_3 s_53_5
        let s_53_6: bool = ((s_53_3) == (s_53_5));
        // D s_53_7: write-var gs#73829 <= s_53_6
        fn_state.gs_73829 = s_53_6;
        // N s_53_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #424u : u32
        let s_54_0: u32 = 424;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // C s_54_2: const #2u : u8
        let s_54_2: u8 = 2;
        // D s_54_3: cmp-lt s_54_1 s_54_2
        let s_54_3: bool = ((s_54_1) < (s_54_2));
        // D s_54_4: write-var gs#73828 <= s_54_3
        fn_state.gs_73828 = s_54_3;
        // N s_54_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call Halted(s_55_0)
        let s_55_1: bool = Halted(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b132 b56
        if s_55_1 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#73841 <= s_56_0
        fn_state.gs_73841 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#73841:u8
        let s_57_0: bool = fn_state.gs_73841;
        // N s_57_1: branch s_57_0 b131 b58
        if s_57_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#73842 <= s_58_0
        fn_state.gs_73842 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#73842:u8
        let s_59_0: bool = fn_state.gs_73842;
        // N s_59_1: branch s_59_0 b130 b60
        if s_59_0 {
            return block_130(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#73843 <= s_60_0
        fn_state.gs_73843 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#73843:u8
        let s_61_0: bool = fn_state.gs_73843;
        // N s_61_1: branch s_61_0 b129 b62
        if s_61_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#73844 <= s_62_0
        fn_state.gs_73844 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#73844:u8
        let s_63_0: bool = fn_state.gs_73844;
        // N s_63_1: branch s_63_0 b128 b64
        if s_63_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call Halted(s_64_0)
        let s_64_1: bool = Halted(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b127 b65
        if s_64_1 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#73845 <= s_65_0
        fn_state.gs_73845 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#73845:u8
        let s_66_0: bool = fn_state.gs_73845;
        // N s_66_1: branch s_66_0 b126 b67
        if s_66_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#73846 <= s_67_0
        fn_state.gs_73846 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#73846:u8
        let s_68_0: bool = fn_state.gs_73846;
        // N s_68_1: branch s_68_0 b125 b69
        if s_68_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#73847 <= s_69_0
        fn_state.gs_73847 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#73847:u8
        let s_70_0: bool = fn_state.gs_73847;
        // N s_70_1: branch s_70_0 b124 b71
        if s_70_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#73848 <= s_71_0
        fn_state.gs_73848 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#73848:u8
        let s_72_0: bool = fn_state.gs_73848;
        // N s_72_1: branch s_72_0 b123 b73
        if s_72_0 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call EL2Enabled(s_73_0)
        let s_73_1: bool = EL2Enabled(state, tracer, s_73_0);
        // N s_73_2: branch s_73_1 b122 b74
        if s_73_1 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#73849 <= s_74_0
        fn_state.gs_73849 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#73849:u8
        let s_75_0: bool = fn_state.gs_73849;
        // N s_75_1: branch s_75_0 b121 b76
        if s_75_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#73850 <= s_76_0
        fn_state.gs_73850 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#73850:u8
        let s_77_0: bool = fn_state.gs_73850;
        // N s_77_1: branch s_77_0 b120 b78
        if s_77_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var gs#73851 <= s_78_0
        fn_state.gs_73851 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#73851:u8
        let s_79_0: bool = fn_state.gs_73851;
        // N s_79_1: branch s_79_0 b119 b80
        if s_79_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call EL2Enabled(s_80_0)
        let s_80_1: bool = EL2Enabled(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b118 b81
        if s_80_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#73852 <= s_81_0
        fn_state.gs_73852 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#73852:u8
        let s_82_0: bool = fn_state.gs_73852;
        // N s_82_1: branch s_82_0 b117 b83
        if s_82_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#73853 <= s_83_0
        fn_state.gs_73853 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#73853:u8
        let s_84_0: bool = fn_state.gs_73853;
        // N s_84_1: branch s_84_0 b116 b85
        if s_84_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #() : ()
        let s_85_0: () = ();
        // S s_85_1: call EL2Enabled(s_85_0)
        let s_85_1: bool = EL2Enabled(state, tracer, s_85_0);
        // N s_85_2: branch s_85_1 b115 b86
        if s_85_1 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#73854 <= s_86_0
        fn_state.gs_73854 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#73854:u8
        let s_87_0: bool = fn_state.gs_73854;
        // N s_87_1: branch s_87_0 b114 b88
        if s_87_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call EL2Enabled(s_88_0)
        let s_88_1: bool = EL2Enabled(state, tracer, s_88_0);
        // N s_88_2: branch s_88_1 b113 b89
        if s_88_1 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#73855 <= s_89_0
        fn_state.gs_73855 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#73855:u8
        let s_90_0: bool = fn_state.gs_73855;
        // N s_90_1: branch s_90_0 b112 b91
        if s_90_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #424u : u32
        let s_91_0: u32 = 424;
        // D s_91_1: read-reg s_91_0:u8
        let s_91_1: u8 = {
            let value = state.read_register::<u8>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // C s_91_2: const #2u : u8
        let s_91_2: u8 = 2;
        // D s_91_3: cmp-lt s_91_1 s_91_2
        let s_91_3: bool = ((s_91_1) < (s_91_2));
        // N s_91_4: branch s_91_3 b111 b92
        if s_91_3 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#73856 <= s_92_0
        fn_state.gs_73856 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#73856:u8
        let s_93_0: bool = fn_state.gs_73856;
        // N s_93_1: branch s_93_0 b105 b94
        if s_93_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #424u : u32
        let s_94_0: u32 = 424;
        // D s_94_1: read-reg s_94_0:u8
        let s_94_1: u8 = {
            let value = state.read_register::<u8>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // C s_94_2: const #2u : u8
        let s_94_2: u8 = 2;
        // D s_94_3: cmp-lt s_94_1 s_94_2
        let s_94_3: bool = ((s_94_1) < (s_94_2));
        // N s_94_4: branch s_94_3 b104 b95
        if s_94_3 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#73857 <= s_95_0
        fn_state.gs_73857 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#73857:u8
        let s_96_0: bool = fn_state.gs_73857;
        // N s_96_1: branch s_96_0 b98 b97
        if s_96_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #64s : i64
        let s_97_0: i64 = 64;
        // C s_97_1: const #16552u : u32
        let s_97_1: u32 = 16552;
        // D s_97_2: read-reg s_97_1:struct
        let s_97_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_97_1 as isize);
            tracer.read_register(s_97_1 as isize, value);
            value
        };
        // D s_97_3: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_97_2)
        let s_97_3: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_97_2);
        // D s_97_4: cast zx s_97_3 -> bv
        let s_97_4: Bits = Bits::new(s_97_3 as u128, 6u16);
        // D s_97_5: cast zx s_97_4 -> i
        let s_97_5: i128 = (s_97_4.value() as i128);
        // D s_97_6: cast reint s_97_5 -> i64
        let s_97_6: i64 = (s_97_5 as i64);
        // D s_97_7: cast zx s_97_6 -> i
        let s_97_7: i128 = (i128::try_from(s_97_6).unwrap());
        // D s_97_8: call SPMINTENSET_EL1_read(s_97_7)
        let s_97_8: u64 = SPMINTENSET_EL1_read(state, tracer, s_97_7);
        // D s_97_9: cast zx s_97_8 -> bv
        let s_97_9: Bits = Bits::new(s_97_8 as u128, 64u16);
        // D s_97_10: read-var t:i
        let s_97_10: i128 = fn_state.t;
        // D s_97_11: call X_set(s_97_10, s_97_0, s_97_9)
        let s_97_11: () = X_set(state, tracer, s_97_10, s_97_0, s_97_9);
        // N s_97_12: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call Halted(s_98_0)
        let s_98_1: bool = Halted(state, tracer, s_98_0);
        // N s_98_2: branch s_98_1 b103 b99
        if s_98_1 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#73859 <= s_99_0
        fn_state.gs_73859 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#73859:u8
        let s_100_0: bool = fn_state.gs_73859;
        // N s_100_1: branch s_100_0 b102 b101
        if s_100_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #24u : u8
        let s_101_0: u8 = 24;
        // C s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 8u16);
        // C s_101_2: cast zx s_101_1 -> i
        let s_101_2: i128 = (s_101_1.value() as i128);
        // C s_101_3: cast reint s_101_2 -> i64
        let s_101_3: i64 = (s_101_2 as i64);
        // C s_101_4: cast zx s_101_3 -> i
        let s_101_4: i128 = (i128::try_from(s_101_3).unwrap());
        // C s_101_5: const #424u : u32
        let s_101_5: u32 = 424;
        // D s_101_6: read-reg s_101_5:u8
        let s_101_6: u8 = {
            let value = state.read_register::<u8>(s_101_5 as isize);
            tracer.read_register(s_101_5 as isize, value);
            value
        };
        // D s_101_7: call AArch64_SystemAccessTrap(s_101_6, s_101_4)
        let s_101_7: () = AArch64_SystemAccessTrap(state, tracer, s_101_6, s_101_4);
        // N s_101_8: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_102_0: panic
        panic!("{:?}", ());
        // N s_102_1: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #() : ()
        let s_103_0: () = ();
        // S s_103_1: call EDSCR_read(s_103_0)
        let s_103_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_103_0);
        // S s_103_2: call _get_EDSCR_Type_SDD(s_103_1)
        let s_103_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_103_1);
        // S s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 1u16);
        // C s_103_4: const #1u : u8
        let s_103_4: bool = true;
        // C s_103_5: cast zx s_103_4 -> bv
        let s_103_5: Bits = Bits::new(s_103_4 as u128, 1u16);
        // S s_103_6: cmp-eq s_103_3 s_103_5
        let s_103_6: bool = ((s_103_3) == (s_103_5));
        // D s_103_7: write-var gs#73859 <= s_103_6
        fn_state.gs_73859 = s_103_6;
        // N s_103_8: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #() : ()
        let s_104_0: () = ();
        // S s_104_1: call __get_selected_SPMACCESSR_EL3_field(s_104_0)
        let s_104_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_104_0);
        // S s_104_2: cast zx s_104_1 -> bv
        let s_104_2: Bits = Bits::new(s_104_1 as u128, 2u16);
        // C s_104_3: const #0u : u8
        let s_104_3: u8 = 0;
        // C s_104_4: cast zx s_104_3 -> bv
        let s_104_4: Bits = Bits::new(s_104_3 as u128, 2u16);
        // S s_104_5: cmp-eq s_104_2 s_104_4
        let s_104_5: bool = ((s_104_2) == (s_104_4));
        // D s_104_6: write-var gs#73857 <= s_104_5
        fn_state.gs_73857 = s_104_5;
        // N s_104_7: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #() : ()
        let s_105_0: () = ();
        // S s_105_1: call Halted(s_105_0)
        let s_105_1: bool = Halted(state, tracer, s_105_0);
        // N s_105_2: branch s_105_1 b110 b106
        if s_105_1 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #0u : u8
        let s_106_0: bool = false;
        // D s_106_1: write-var gs#73860 <= s_106_0
        fn_state.gs_73860 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#73860:u8
        let s_107_0: bool = fn_state.gs_73860;
        // N s_107_1: branch s_107_0 b109 b108
        if s_107_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #24u : u8
        let s_108_0: u8 = 24;
        // C s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 8u16);
        // C s_108_2: cast zx s_108_1 -> i
        let s_108_2: i128 = (s_108_1.value() as i128);
        // C s_108_3: cast reint s_108_2 -> i64
        let s_108_3: i64 = (s_108_2 as i64);
        // C s_108_4: cast zx s_108_3 -> i
        let s_108_4: i128 = (i128::try_from(s_108_3).unwrap());
        // C s_108_5: const #424u : u32
        let s_108_5: u32 = 424;
        // D s_108_6: read-reg s_108_5:u8
        let s_108_6: u8 = {
            let value = state.read_register::<u8>(s_108_5 as isize);
            tracer.read_register(s_108_5 as isize, value);
            value
        };
        // D s_108_7: call AArch64_SystemAccessTrap(s_108_6, s_108_4)
        let s_108_7: () = AArch64_SystemAccessTrap(state, tracer, s_108_6, s_108_4);
        // N s_108_8: return
        return;
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_109_0: panic
        panic!("{:?}", ());
        // N s_109_1: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #() : ()
        let s_110_0: () = ();
        // S s_110_1: call EDSCR_read(s_110_0)
        let s_110_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_110_0);
        // S s_110_2: call _get_EDSCR_Type_SDD(s_110_1)
        let s_110_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_110_1);
        // S s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // C s_110_4: const #1u : u8
        let s_110_4: bool = true;
        // C s_110_5: cast zx s_110_4 -> bv
        let s_110_5: Bits = Bits::new(s_110_4 as u128, 1u16);
        // S s_110_6: cmp-eq s_110_3 s_110_5
        let s_110_6: bool = ((s_110_3) == (s_110_5));
        // D s_110_7: write-var gs#73860 <= s_110_6
        fn_state.gs_73860 = s_110_6;
        // N s_110_8: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var __MDCR_EL3_EnPM2:u8
        let s_111_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #0u : u8
        let s_111_2: bool = false;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // D s_111_5: write-var gs#73856 <= s_111_4
        fn_state.gs_73856 = s_111_4;
        // N s_111_6: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #24u : u8
        let s_112_0: u8 = 24;
        // C s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 8u16);
        // C s_112_2: cast zx s_112_1 -> i
        let s_112_2: i128 = (s_112_1.value() as i128);
        // C s_112_3: cast reint s_112_2 -> i64
        let s_112_3: i64 = (s_112_2 as i64);
        // C s_112_4: cast zx s_112_3 -> i
        let s_112_4: i128 = (i128::try_from(s_112_3).unwrap());
        // C s_112_5: const #432u : u32
        let s_112_5: u32 = 432;
        // D s_112_6: read-reg s_112_5:u8
        let s_112_6: u8 = {
            let value = state.read_register::<u8>(s_112_5 as isize);
            tracer.read_register(s_112_5 as isize, value);
            value
        };
        // D s_112_7: call AArch64_SystemAccessTrap(s_112_6, s_112_4)
        let s_112_7: () = AArch64_SystemAccessTrap(state, tracer, s_112_6, s_112_4);
        // N s_112_8: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #() : ()
        let s_113_0: () = ();
        // S s_113_1: call __get_selected_SPMACCESSR_EL2_field(s_113_0)
        let s_113_1: u8 = u__get_selected_SPMACCESSR_EL2_field(state, tracer, s_113_0);
        // S s_113_2: cast zx s_113_1 -> bv
        let s_113_2: Bits = Bits::new(s_113_1 as u128, 2u16);
        // C s_113_3: const #0u : u8
        let s_113_3: u8 = 0;
        // C s_113_4: cast zx s_113_3 -> bv
        let s_113_4: Bits = Bits::new(s_113_3 as u128, 2u16);
        // S s_113_5: cmp-eq s_113_2 s_113_4
        let s_113_5: bool = ((s_113_2) == (s_113_4));
        // D s_113_6: write-var gs#73855 <= s_113_5
        fn_state.gs_73855 = s_113_5;
        // N s_113_7: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #24u : u8
        let s_114_0: u8 = 24;
        // C s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 8u16);
        // C s_114_2: cast zx s_114_1 -> i
        let s_114_2: i128 = (s_114_1.value() as i128);
        // C s_114_3: cast reint s_114_2 -> i64
        let s_114_3: i64 = (s_114_2 as i64);
        // C s_114_4: cast zx s_114_3 -> i
        let s_114_4: i128 = (i128::try_from(s_114_3).unwrap());
        // C s_114_5: const #432u : u32
        let s_114_5: u32 = 432;
        // D s_114_6: read-reg s_114_5:u8
        let s_114_6: u8 = {
            let value = state.read_register::<u8>(s_114_5 as isize);
            tracer.read_register(s_114_5 as isize, value);
            value
        };
        // D s_114_7: call AArch64_SystemAccessTrap(s_114_6, s_114_4)
        let s_114_7: () = AArch64_SystemAccessTrap(state, tracer, s_114_6, s_114_4);
        // N s_114_8: return
        return;
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var __MDCR_EL2_EnSPM:u8
        let s_115_0: bool = fn_state.u__MDCR_EL2_EnSPM;
        // D s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 1u16);
        // C s_115_2: const #0u : u8
        let s_115_2: bool = false;
        // C s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_4: cmp-eq s_115_1 s_115_3
        let s_115_4: bool = ((s_115_1) == (s_115_3));
        // D s_115_5: write-var gs#73854 <= s_115_4
        fn_state.gs_73854 = s_115_4;
        // N s_115_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #24u : u8
        let s_116_0: u8 = 24;
        // C s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 8u16);
        // C s_116_2: cast zx s_116_1 -> i
        let s_116_2: i128 = (s_116_1.value() as i128);
        // C s_116_3: cast reint s_116_2 -> i64
        let s_116_3: i64 = (s_116_2 as i64);
        // C s_116_4: cast zx s_116_3 -> i
        let s_116_4: i128 = (i128::try_from(s_116_3).unwrap());
        // C s_116_5: const #432u : u32
        let s_116_5: u32 = 432;
        // D s_116_6: read-reg s_116_5:u8
        let s_116_6: u8 = {
            let value = state.read_register::<u8>(s_116_5 as isize);
            tracer.read_register(s_116_5 as isize, value);
            value
        };
        // D s_116_7: call AArch64_SystemAccessTrap(s_116_6, s_116_4)
        let s_116_7: () = AArch64_SystemAccessTrap(state, tracer, s_116_6, s_116_4);
        // N s_116_8: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var __HDFGRTR2_EL2_nSPMINTEN:u8
        let s_117_0: bool = fn_state.u__HDFGRTR2_EL2_nSPMINTEN;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 1u16);
        // C s_117_2: const #0u : u8
        let s_117_2: bool = false;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_4: cmp-eq s_117_1 s_117_3
        let s_117_4: bool = ((s_117_1) == (s_117_3));
        // D s_117_5: write-var gs#73853 <= s_117_4
        fn_state.gs_73853 = s_117_4;
        // N s_117_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #188u : u32
        let s_118_0: u32 = 188;
        // S s_118_1: call IsFeatureImplemented(s_118_0)
        let s_118_1: bool = IsFeatureImplemented(state, tracer, s_118_0);
        // D s_118_2: write-var gs#73852 <= s_118_1
        fn_state.gs_73852 = s_118_1;
        // N s_118_3: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #24u : u8
        let s_119_0: u8 = 24;
        // C s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 8u16);
        // C s_119_2: cast zx s_119_1 -> i
        let s_119_2: i128 = (s_119_1.value() as i128);
        // C s_119_3: cast reint s_119_2 -> i64
        let s_119_3: i64 = (s_119_2 as i64);
        // C s_119_4: cast zx s_119_3 -> i
        let s_119_4: i128 = (i128::try_from(s_119_3).unwrap());
        // C s_119_5: const #432u : u32
        let s_119_5: u32 = 432;
        // D s_119_6: read-reg s_119_5:u8
        let s_119_6: u8 = {
            let value = state.read_register::<u8>(s_119_5 as isize);
            tracer.read_register(s_119_5 as isize, value);
            value
        };
        // D s_119_7: call AArch64_SystemAccessTrap(s_119_6, s_119_4)
        let s_119_7: () = AArch64_SystemAccessTrap(state, tracer, s_119_6, s_119_4);
        // N s_119_8: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var __SCR_EL3_FGTEn2:u8
        let s_120_0: bool = fn_state.u__SCR_EL3_FGTEn2;
        // D s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 1u16);
        // C s_120_2: const #0u : u8
        let s_120_2: bool = false;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 1u16);
        // D s_120_4: cmp-eq s_120_1 s_120_3
        let s_120_4: bool = ((s_120_1) == (s_120_3));
        // D s_120_5: write-var gs#73851 <= s_120_4
        fn_state.gs_73851 = s_120_4;
        // N s_120_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #424u : u32
        let s_121_0: u32 = 424;
        // D s_121_1: read-reg s_121_0:u8
        let s_121_1: u8 = {
            let value = state.read_register::<u8>(s_121_0 as isize);
            tracer.read_register(s_121_0 as isize, value);
            value
        };
        // C s_121_2: const #2u : u8
        let s_121_2: u8 = 2;
        // D s_121_3: cmp-lt s_121_1 s_121_2
        let s_121_3: bool = ((s_121_1) < (s_121_2));
        // D s_121_4: write-var gs#73850 <= s_121_3
        fn_state.gs_73850 = s_121_3;
        // N s_121_5: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #188u : u32
        let s_122_0: u32 = 188;
        // S s_122_1: call IsFeatureImplemented(s_122_0)
        let s_122_1: bool = IsFeatureImplemented(state, tracer, s_122_0);
        // D s_122_2: write-var gs#73849 <= s_122_1
        fn_state.gs_73849 = s_122_1;
        // N s_122_3: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_123_0: panic
        panic!("{:?}", ());
        // N s_123_1: return
        return;
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call __get_selected_SPMACCESSR_EL3_field(s_124_0)
        let s_124_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_124_0);
        // S s_124_2: cast zx s_124_1 -> bv
        let s_124_2: Bits = Bits::new(s_124_1 as u128, 2u16);
        // C s_124_3: const #0u : u8
        let s_124_3: u8 = 0;
        // C s_124_4: cast zx s_124_3 -> bv
        let s_124_4: Bits = Bits::new(s_124_3 as u128, 2u16);
        // S s_124_5: cmp-eq s_124_2 s_124_4
        let s_124_5: bool = ((s_124_2) == (s_124_4));
        // D s_124_6: write-var gs#73848 <= s_124_5
        fn_state.gs_73848 = s_124_5;
        // N s_124_7: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_125_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_125_1: call __IMPDEF_boolean(s_125_0)
        let s_125_1: bool = u__IMPDEF_boolean(state, tracer, s_125_0);
        // D s_125_2: write-var gs#73847 <= s_125_1
        fn_state.gs_73847 = s_125_1;
        // N s_125_3: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #() : ()
        let s_126_0: () = ();
        // S s_126_1: call EDSCR_read(s_126_0)
        let s_126_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_126_0);
        // S s_126_2: call _get_EDSCR_Type_SDD(s_126_1)
        let s_126_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_126_1);
        // S s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 1u16);
        // C s_126_4: const #1u : u8
        let s_126_4: bool = true;
        // C s_126_5: cast zx s_126_4 -> bv
        let s_126_5: Bits = Bits::new(s_126_4 as u128, 1u16);
        // S s_126_6: cmp-eq s_126_3 s_126_5
        let s_126_6: bool = ((s_126_3) == (s_126_5));
        // D s_126_7: write-var gs#73846 <= s_126_6
        fn_state.gs_73846 = s_126_6;
        // N s_126_8: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #424u : u32
        let s_127_0: u32 = 424;
        // D s_127_1: read-reg s_127_0:u8
        let s_127_1: u8 = {
            let value = state.read_register::<u8>(s_127_0 as isize);
            tracer.read_register(s_127_0 as isize, value);
            value
        };
        // C s_127_2: const #2u : u8
        let s_127_2: u8 = 2;
        // D s_127_3: cmp-lt s_127_1 s_127_2
        let s_127_3: bool = ((s_127_1) < (s_127_2));
        // D s_127_4: write-var gs#73845 <= s_127_3
        fn_state.gs_73845 = s_127_3;
        // N s_127_5: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_128_0: panic
        panic!("{:?}", ());
        // N s_128_1: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var __MDCR_EL3_EnPM2:u8
        let s_129_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 1u16);
        // C s_129_2: const #0u : u8
        let s_129_2: bool = false;
        // C s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 1u16);
        // D s_129_4: cmp-eq s_129_1 s_129_3
        let s_129_4: bool = ((s_129_1) == (s_129_3));
        // D s_129_5: write-var gs#73844 <= s_129_4
        fn_state.gs_73844 = s_129_4;
        // N s_129_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_130_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_130_1: call __IMPDEF_boolean(s_130_0)
        let s_130_1: bool = u__IMPDEF_boolean(state, tracer, s_130_0);
        // D s_130_2: write-var gs#73843 <= s_130_1
        fn_state.gs_73843 = s_130_1;
        // N s_130_3: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #() : ()
        let s_131_0: () = ();
        // S s_131_1: call EDSCR_read(s_131_0)
        let s_131_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_131_0);
        // S s_131_2: call _get_EDSCR_Type_SDD(s_131_1)
        let s_131_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_131_1);
        // S s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 1u16);
        // C s_131_4: const #1u : u8
        let s_131_4: bool = true;
        // C s_131_5: cast zx s_131_4 -> bv
        let s_131_5: Bits = Bits::new(s_131_4 as u128, 1u16);
        // S s_131_6: cmp-eq s_131_3 s_131_5
        let s_131_6: bool = ((s_131_3) == (s_131_5));
        // D s_131_7: write-var gs#73842 <= s_131_6
        fn_state.gs_73842 = s_131_6;
        // N s_131_8: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #424u : u32
        let s_132_0: u32 = 424;
        // D s_132_1: read-reg s_132_0:u8
        let s_132_1: u8 = {
            let value = state.read_register::<u8>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // C s_132_2: const #2u : u8
        let s_132_2: u8 = 2;
        // D s_132_3: cmp-lt s_132_1 s_132_2
        let s_132_3: bool = ((s_132_1) < (s_132_2));
        // D s_132_4: write-var gs#73841 <= s_132_3
        fn_state.gs_73841 = s_132_3;
        // N s_132_5: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_133_0: panic
        panic!("{:?}", ());
        // N s_133_1: return
        return;
    }
}
