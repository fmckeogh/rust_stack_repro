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
use Halted::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_MDCR_EL3_Type_NSPB::*;
use Mk_PMSCR_EL2_Type::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_SCR_EL3_Type_NSE::*;
use u_get_MDCR_EL3_Type_NSPBE::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn PMSCR_EL2_SysRegWrite_7385521905c06717<T: Tracer>(
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
        gs_90808: bool,
        gs_90826: bool,
        gs_90814: bool,
        gs_90821: bool,
        gs_90807: bool,
        gs_90824: bool,
        u__MDCR_EL3_NSPBE: bool,
        gs_90827: bool,
        gs_90806: bool,
        gs_90813: bool,
        gs_90822: bool,
        gs_90823: bool,
        u__SCR_EL3_NSE: bool,
        u__PSTATE_EL: u8,
        gs_90815: bool,
        gs_90816: bool,
        u__HCR_EL2_NV: bool,
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
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_NV(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_NV <= s_0_5
        fn_state.u__HCR_EL2_NV = s_0_5;
        // C s_0_7: const #22712u : u32
        let s_0_7: u32 = 22712;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL3_Type_NSPBE(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL3_Type_NSPBE(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_NSPBE <= s_0_9
        fn_state.u__MDCR_EL3_NSPBE = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_NSE(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_NSE(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_NSE <= s_0_13
        fn_state.u__SCR_EL3_NSE = s_0_13;
        // D s_0_15: read-var __PSTATE_EL:u8
        let s_0_15: u8 = fn_state.u__PSTATE_EL;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 2u16);
        // C s_0_17: const #448u : u32
        let s_0_17: u32 = 448;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: u8 = {
            let value = state.read_register::<u8>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 2u16);
        // D s_0_20: cmp-eq s_0_16 s_0_19
        let s_0_20: bool = ((s_0_16) == (s_0_19));
        // N s_0_21: branch s_0_20 b55 b1
        if s_0_20 {
            return block_55(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b49 b2
        if s_1_5 {
            return block_49(state, tracer, fn_state);
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
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // D s_5_4: call Mk_PMSCR_EL2_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_PMSCR_EL2_Type(state, tracer, s_5_3);
        // C s_5_5: const #104928u : u32
        let s_5_5: u32 = 104928;
        // N s_5_6: write-reg s_5_5 <= s_5_4
        let s_5_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_5 as isize, s_5_4);
            tracer.write_register(s_5_5 as isize, s_5_4);
        };
        // N s_5_7: return
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
        // N s_6_2: branch s_6_1 b48 b7
        if s_6_1 {
            return block_48(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#90806 <= s_7_0
        fn_state.gs_90806 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#90806:u8
        let s_8_0: bool = fn_state.gs_90806;
        // N s_8_1: branch s_8_0 b47 b9
        if s_8_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#90807 <= s_9_0
        fn_state.gs_90807 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#90807:u8
        let s_10_0: bool = fn_state.gs_90807;
        // N s_10_1: branch s_10_0 b46 b11
        if s_10_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#90808 <= s_11_0
        fn_state.gs_90808 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#90808:u8
        let s_12_0: bool = fn_state.gs_90808;
        // N s_12_1: branch s_12_0 b36 b13
        if s_12_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#90816 <= s_13_0
        fn_state.gs_90816 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#90816:u8
        let s_14_0: bool = fn_state.gs_90816;
        // N s_14_1: branch s_14_0 b35 b15
        if s_14_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #424u : u32
        let s_15_0: u32 = 424;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // D s_15_3: cmp-lt s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) < (s_15_2));
        // N s_15_4: branch s_15_3 b25 b16
        if s_15_3 {
            return block_25(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#90824 <= s_16_0
        fn_state.gs_90824 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#90824:u8
        let s_17_0: bool = fn_state.gs_90824;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #64s : i64
        let s_18_0: i64 = 64;
        // D s_18_1: read-var t:i
        let s_18_1: i128 = fn_state.t;
        // D s_18_2: call X_read(s_18_1, s_18_0)
        let s_18_2: Bits = X_read(state, tracer, s_18_1, s_18_0);
        // D s_18_3: cast reint s_18_2 -> u64
        let s_18_3: u64 = (s_18_2.value() as u64);
        // D s_18_4: call Mk_PMSCR_EL2_Type(s_18_3)
        let s_18_4: ProductType5c790c8ef59cc8b2 = Mk_PMSCR_EL2_Type(
            state,
            tracer,
            s_18_3,
        );
        // C s_18_5: const #104928u : u32
        let s_18_5: u32 = 104928;
        // N s_18_6: write-reg s_18_5 <= s_18_4
        let s_18_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_18_5 as isize, s_18_4);
            tracer.write_register(s_18_5 as isize, s_18_4);
        };
        // N s_18_7: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call Halted(s_19_0)
        let s_19_1: bool = Halted(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b24 b20
        if s_19_1 {
            return block_24(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#90826 <= s_20_0
        fn_state.gs_90826 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#90826:u8
        let s_21_0: bool = fn_state.gs_90826;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #24u : u8
        let s_22_0: u8 = 24;
        // C s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 8u16);
        // C s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (s_22_1.value() as i128);
        // C s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #424u : u32
        let s_22_5: u32 = 424;
        // D s_22_6: read-reg s_22_5:u8
        let s_22_6: u8 = {
            let value = state.read_register::<u8>(s_22_5 as isize);
            tracer.read_register(s_22_5 as isize, value);
            value
        };
        // D s_22_7: call AArch64_SystemAccessTrap(s_22_6, s_22_4)
        let s_22_7: () = AArch64_SystemAccessTrap(state, tracer, s_22_6, s_22_4);
        // N s_22_8: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call EDSCR_read(s_24_0)
        let s_24_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_24_0);
        // S s_24_2: call _get_EDSCR_Type_SDD(s_24_1)
        let s_24_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_24_1);
        // S s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // S s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var gs#90826 <= s_24_6
        fn_state.gs_90826 = s_24_6;
        // N s_24_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #22712u : u32
        let s_25_0: u32 = 22712;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_MDCR_EL3_Type_NSPB(s_25_1)
        let s_25_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_25_1);
        // C s_25_3: const #0s : i
        let s_25_3: i128 = 0;
        // D s_25_4: cast zx s_25_2 -> bv
        let s_25_4: Bits = Bits::new(s_25_2 as u128, 2u16);
        // C s_25_5: const #1u : u64
        let s_25_5: u64 = 1;
        // D s_25_6: bit-extract s_25_4 s_25_3 s_25_5
        let s_25_6: Bits = (Bits::new(
            ((s_25_4) >> (s_25_3)).value(),
            u16::try_from(s_25_5).unwrap(),
        ));
        // D s_25_7: cast reint s_25_6 -> u8
        let s_25_7: bool = ((s_25_6.value()) != 0);
        // C s_25_8: const #0s : i
        let s_25_8: i128 = 0;
        // C s_25_9: const #0u : u64
        let s_25_9: u64 = 0;
        // D s_25_10: cast zx s_25_7 -> u64
        let s_25_10: u64 = (s_25_7 as u64);
        // C s_25_11: const #1u : u64
        let s_25_11: u64 = 1;
        // D s_25_12: and s_25_10 s_25_11
        let s_25_12: u64 = ((s_25_10) & (s_25_11));
        // D s_25_13: cmp-eq s_25_12 s_25_11
        let s_25_13: bool = ((s_25_12) == (s_25_11));
        // D s_25_14: lsl s_25_10 s_25_8
        let s_25_14: u64 = s_25_10 << s_25_8;
        // D s_25_15: or s_25_9 s_25_14
        let s_25_15: u64 = ((s_25_9) | (s_25_14));
        // D s_25_16: cmpl s_25_14
        let s_25_16: u64 = !s_25_14;
        // D s_25_17: and s_25_9 s_25_16
        let s_25_17: u64 = ((s_25_9) & (s_25_16));
        // D s_25_18: select s_25_13 s_25_15 s_25_17
        let s_25_18: u64 = if s_25_13 { s_25_15 } else { s_25_17 };
        // D s_25_19: cast trunc s_25_18 -> u8
        let s_25_19: bool = ((s_25_18) != 0);
        // D s_25_20: cast zx s_25_19 -> bv
        let s_25_20: Bits = Bits::new(s_25_19 as u128, 1u16);
        // C s_25_21: const #0u : u8
        let s_25_21: bool = false;
        // C s_25_22: cast zx s_25_21 -> bv
        let s_25_22: Bits = Bits::new(s_25_21 as u128, 1u16);
        // D s_25_23: cmp-eq s_25_20 s_25_22
        let s_25_23: bool = ((s_25_20) == (s_25_22));
        // N s_25_24: branch s_25_23 b34 b26
        if s_25_23 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #22712u : u32
        let s_26_0: u32 = 22712;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_MDCR_EL3_Type_NSPB(s_26_1)
        let s_26_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_26_1);
        // C s_26_3: const #1s : i
        let s_26_3: i128 = 1;
        // D s_26_4: cast zx s_26_2 -> bv
        let s_26_4: Bits = Bits::new(s_26_2 as u128, 2u16);
        // C s_26_5: const #1u : u64
        let s_26_5: u64 = 1;
        // D s_26_6: bit-extract s_26_4 s_26_3 s_26_5
        let s_26_6: Bits = (Bits::new(
            ((s_26_4) >> (s_26_3)).value(),
            u16::try_from(s_26_5).unwrap(),
        ));
        // D s_26_7: cast reint s_26_6 -> u8
        let s_26_7: bool = ((s_26_6.value()) != 0);
        // C s_26_8: const #0s : i
        let s_26_8: i128 = 0;
        // C s_26_9: const #0u : u64
        let s_26_9: u64 = 0;
        // D s_26_10: cast zx s_26_7 -> u64
        let s_26_10: u64 = (s_26_7 as u64);
        // C s_26_11: const #1u : u64
        let s_26_11: u64 = 1;
        // D s_26_12: and s_26_10 s_26_11
        let s_26_12: u64 = ((s_26_10) & (s_26_11));
        // D s_26_13: cmp-eq s_26_12 s_26_11
        let s_26_13: bool = ((s_26_12) == (s_26_11));
        // D s_26_14: lsl s_26_10 s_26_8
        let s_26_14: u64 = s_26_10 << s_26_8;
        // D s_26_15: or s_26_9 s_26_14
        let s_26_15: u64 = ((s_26_9) | (s_26_14));
        // D s_26_16: cmpl s_26_14
        let s_26_16: u64 = !s_26_14;
        // D s_26_17: and s_26_9 s_26_16
        let s_26_17: u64 = ((s_26_9) & (s_26_16));
        // D s_26_18: select s_26_13 s_26_15 s_26_17
        let s_26_18: u64 = if s_26_13 { s_26_15 } else { s_26_17 };
        // D s_26_19: cast trunc s_26_18 -> u8
        let s_26_19: bool = ((s_26_18) != 0);
        // C s_26_20: const #90704u : u32
        let s_26_20: u32 = 90704;
        // D s_26_21: read-reg s_26_20:struct
        let s_26_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_20 as isize);
            tracer.read_register(s_26_20 as isize, value);
            value
        };
        // D s_26_22: call _get_SCR_EL3_Type_NS(s_26_21)
        let s_26_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_26_21);
        // D s_26_23: cast zx s_26_19 -> bv
        let s_26_23: Bits = Bits::new(s_26_19 as u128, 1u16);
        // D s_26_24: cast zx s_26_22 -> bv
        let s_26_24: Bits = Bits::new(s_26_22 as u128, 1u16);
        // D s_26_25: cmp-ne s_26_23 s_26_24
        let s_26_25: bool = ((s_26_23) != (s_26_24));
        // D s_26_26: write-var gs#90821 <= s_26_25
        fn_state.gs_90821 = s_26_25;
        // N s_26_27: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#90821:u8
        let s_27_0: bool = fn_state.gs_90821;
        // N s_27_1: branch s_27_0 b33 b28
        if s_27_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #232u : u32
        let s_28_0: u32 = 232;
        // S s_28_1: call IsFeatureImplemented(s_28_0)
        let s_28_1: bool = IsFeatureImplemented(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b32 b29
        if s_28_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#90822 <= s_29_0
        fn_state.gs_90822 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#90822:u8
        let s_30_0: bool = fn_state.gs_90822;
        // D s_30_1: write-var gs#90823 <= s_30_0
        fn_state.gs_90823 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#90823:u8
        let s_31_0: bool = fn_state.gs_90823;
        // D s_31_1: write-var gs#90824 <= s_31_0
        fn_state.gs_90824 = s_31_0;
        // N s_31_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __MDCR_EL3_NSPBE:u8
        let s_32_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // D s_32_2: read-var __SCR_EL3_NSE:u8
        let s_32_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-ne s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) != (s_32_3));
        // D s_32_5: write-var gs#90822 <= s_32_4
        fn_state.gs_90822 = s_32_4;
        // N s_32_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#90823 <= s_33_0
        fn_state.gs_90823 = s_33_0;
        // N s_33_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#90821 <= s_34_0
        fn_state.gs_90821 = s_34_0;
        // N s_34_2: jump b27
        return block_27(state, tracer, fn_state);
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
        // C s_36_0: const #22712u : u32
        let s_36_0: u32 = 22712;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_MDCR_EL3_Type_NSPB(s_36_1)
        let s_36_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_36_1);
        // C s_36_3: const #0s : i
        let s_36_3: i128 = 0;
        // D s_36_4: cast zx s_36_2 -> bv
        let s_36_4: Bits = Bits::new(s_36_2 as u128, 2u16);
        // C s_36_5: const #1u : u64
        let s_36_5: u64 = 1;
        // D s_36_6: bit-extract s_36_4 s_36_3 s_36_5
        let s_36_6: Bits = (Bits::new(
            ((s_36_4) >> (s_36_3)).value(),
            u16::try_from(s_36_5).unwrap(),
        ));
        // D s_36_7: cast reint s_36_6 -> u8
        let s_36_7: bool = ((s_36_6.value()) != 0);
        // C s_36_8: const #0s : i
        let s_36_8: i128 = 0;
        // C s_36_9: const #0u : u64
        let s_36_9: u64 = 0;
        // D s_36_10: cast zx s_36_7 -> u64
        let s_36_10: u64 = (s_36_7 as u64);
        // C s_36_11: const #1u : u64
        let s_36_11: u64 = 1;
        // D s_36_12: and s_36_10 s_36_11
        let s_36_12: u64 = ((s_36_10) & (s_36_11));
        // D s_36_13: cmp-eq s_36_12 s_36_11
        let s_36_13: bool = ((s_36_12) == (s_36_11));
        // D s_36_14: lsl s_36_10 s_36_8
        let s_36_14: u64 = s_36_10 << s_36_8;
        // D s_36_15: or s_36_9 s_36_14
        let s_36_15: u64 = ((s_36_9) | (s_36_14));
        // D s_36_16: cmpl s_36_14
        let s_36_16: u64 = !s_36_14;
        // D s_36_17: and s_36_9 s_36_16
        let s_36_17: u64 = ((s_36_9) & (s_36_16));
        // D s_36_18: select s_36_13 s_36_15 s_36_17
        let s_36_18: u64 = if s_36_13 { s_36_15 } else { s_36_17 };
        // D s_36_19: cast trunc s_36_18 -> u8
        let s_36_19: bool = ((s_36_18) != 0);
        // D s_36_20: cast zx s_36_19 -> bv
        let s_36_20: Bits = Bits::new(s_36_19 as u128, 1u16);
        // C s_36_21: const #0u : u8
        let s_36_21: bool = false;
        // C s_36_22: cast zx s_36_21 -> bv
        let s_36_22: Bits = Bits::new(s_36_21 as u128, 1u16);
        // D s_36_23: cmp-eq s_36_20 s_36_22
        let s_36_23: bool = ((s_36_20) == (s_36_22));
        // N s_36_24: branch s_36_23 b45 b37
        if s_36_23 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #22712u : u32
        let s_37_0: u32 = 22712;
        // D s_37_1: read-reg s_37_0:struct
        let s_37_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call _get_MDCR_EL3_Type_NSPB(s_37_1)
        let s_37_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_37_1);
        // C s_37_3: const #1s : i
        let s_37_3: i128 = 1;
        // D s_37_4: cast zx s_37_2 -> bv
        let s_37_4: Bits = Bits::new(s_37_2 as u128, 2u16);
        // C s_37_5: const #1u : u64
        let s_37_5: u64 = 1;
        // D s_37_6: bit-extract s_37_4 s_37_3 s_37_5
        let s_37_6: Bits = (Bits::new(
            ((s_37_4) >> (s_37_3)).value(),
            u16::try_from(s_37_5).unwrap(),
        ));
        // D s_37_7: cast reint s_37_6 -> u8
        let s_37_7: bool = ((s_37_6.value()) != 0);
        // C s_37_8: const #0s : i
        let s_37_8: i128 = 0;
        // C s_37_9: const #0u : u64
        let s_37_9: u64 = 0;
        // D s_37_10: cast zx s_37_7 -> u64
        let s_37_10: u64 = (s_37_7 as u64);
        // C s_37_11: const #1u : u64
        let s_37_11: u64 = 1;
        // D s_37_12: and s_37_10 s_37_11
        let s_37_12: u64 = ((s_37_10) & (s_37_11));
        // D s_37_13: cmp-eq s_37_12 s_37_11
        let s_37_13: bool = ((s_37_12) == (s_37_11));
        // D s_37_14: lsl s_37_10 s_37_8
        let s_37_14: u64 = s_37_10 << s_37_8;
        // D s_37_15: or s_37_9 s_37_14
        let s_37_15: u64 = ((s_37_9) | (s_37_14));
        // D s_37_16: cmpl s_37_14
        let s_37_16: u64 = !s_37_14;
        // D s_37_17: and s_37_9 s_37_16
        let s_37_17: u64 = ((s_37_9) & (s_37_16));
        // D s_37_18: select s_37_13 s_37_15 s_37_17
        let s_37_18: u64 = if s_37_13 { s_37_15 } else { s_37_17 };
        // D s_37_19: cast trunc s_37_18 -> u8
        let s_37_19: bool = ((s_37_18) != 0);
        // C s_37_20: const #90704u : u32
        let s_37_20: u32 = 90704;
        // D s_37_21: read-reg s_37_20:struct
        let s_37_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_37_20 as isize);
            tracer.read_register(s_37_20 as isize, value);
            value
        };
        // D s_37_22: call _get_SCR_EL3_Type_NS(s_37_21)
        let s_37_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_37_21);
        // D s_37_23: cast zx s_37_19 -> bv
        let s_37_23: Bits = Bits::new(s_37_19 as u128, 1u16);
        // D s_37_24: cast zx s_37_22 -> bv
        let s_37_24: Bits = Bits::new(s_37_22 as u128, 1u16);
        // D s_37_25: cmp-ne s_37_23 s_37_24
        let s_37_25: bool = ((s_37_23) != (s_37_24));
        // D s_37_26: write-var gs#90813 <= s_37_25
        fn_state.gs_90813 = s_37_25;
        // N s_37_27: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#90813:u8
        let s_38_0: bool = fn_state.gs_90813;
        // N s_38_1: branch s_38_0 b44 b39
        if s_38_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #232u : u32
        let s_39_0: u32 = 232;
        // S s_39_1: call IsFeatureImplemented(s_39_0)
        let s_39_1: bool = IsFeatureImplemented(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b43 b40
        if s_39_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#90814 <= s_40_0
        fn_state.gs_90814 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#90814:u8
        let s_41_0: bool = fn_state.gs_90814;
        // D s_41_1: write-var gs#90815 <= s_41_0
        fn_state.gs_90815 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#90815:u8
        let s_42_0: bool = fn_state.gs_90815;
        // D s_42_1: write-var gs#90816 <= s_42_0
        fn_state.gs_90816 = s_42_0;
        // N s_42_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var __MDCR_EL3_NSPBE:u8
        let s_43_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // D s_43_2: read-var __SCR_EL3_NSE:u8
        let s_43_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-ne s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) != (s_43_3));
        // D s_43_5: write-var gs#90814 <= s_43_4
        fn_state.gs_90814 = s_43_4;
        // N s_43_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#90815 <= s_44_0
        fn_state.gs_90815 = s_44_0;
        // N s_44_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#90813 <= s_45_0
        fn_state.gs_90813 = s_45_0;
        // N s_45_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_46_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_46_1: call __IMPDEF_boolean(s_46_0)
        let s_46_1: bool = u__IMPDEF_boolean(state, tracer, s_46_0);
        // D s_46_2: write-var gs#90808 <= s_46_1
        fn_state.gs_90808 = s_46_1;
        // N s_46_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EDSCR_read(s_47_0)
        let s_47_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_47_0);
        // S s_47_2: call _get_EDSCR_Type_SDD(s_47_1)
        let s_47_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_47_1);
        // S s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // C s_47_4: const #1u : u8
        let s_47_4: bool = true;
        // C s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 1u16);
        // S s_47_6: cmp-eq s_47_3 s_47_5
        let s_47_6: bool = ((s_47_3) == (s_47_5));
        // D s_47_7: write-var gs#90807 <= s_47_6
        fn_state.gs_90807 = s_47_6;
        // N s_47_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #424u : u32
        let s_48_0: u32 = 424;
        // D s_48_1: read-reg s_48_0:u8
        let s_48_1: u8 = {
            let value = state.read_register::<u8>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // C s_48_2: const #2u : u8
        let s_48_2: u8 = 2;
        // D s_48_3: cmp-lt s_48_1 s_48_2
        let s_48_3: bool = ((s_48_1) < (s_48_2));
        // D s_48_4: write-var gs#90806 <= s_48_3
        fn_state.gs_90806 = s_48_3;
        // N s_48_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call EL2Enabled(s_49_0)
        let s_49_1: bool = EL2Enabled(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b54 b50
        if s_49_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#90827 <= s_50_0
        fn_state.gs_90827 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#90827:u8
        let s_51_0: bool = fn_state.gs_90827;
        // N s_51_1: branch s_51_0 b53 b52
        if s_51_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: panic
        panic!("{:?}", ());
        // N s_52_1: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #24u : u8
        let s_53_0: u8 = 24;
        // C s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 8u16);
        // C s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (s_53_1.value() as i128);
        // C s_53_3: cast reint s_53_2 -> i64
        let s_53_3: i64 = (s_53_2 as i64);
        // C s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // C s_53_5: const #432u : u32
        let s_53_5: u32 = 432;
        // D s_53_6: read-reg s_53_5:u8
        let s_53_6: u8 = {
            let value = state.read_register::<u8>(s_53_5 as isize);
            tracer.read_register(s_53_5 as isize, value);
            value
        };
        // D s_53_7: call AArch64_SystemAccessTrap(s_53_6, s_53_4)
        let s_53_7: () = AArch64_SystemAccessTrap(state, tracer, s_53_6, s_53_4);
        // N s_53_8: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var __HCR_EL2_NV:u8
        let s_54_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 1u16);
        // C s_54_2: const #1u : u8
        let s_54_2: bool = true;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // D s_54_5: write-var gs#90827 <= s_54_4
        fn_state.gs_90827 = s_54_4;
        // N s_54_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: panic
        panic!("{:?}", ());
        // N s_55_1: return
        return;
    }
}
