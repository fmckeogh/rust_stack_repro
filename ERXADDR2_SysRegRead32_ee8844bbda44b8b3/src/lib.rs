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
use HCR2_read::*;
use EL2Enabled::*;
use AArch32_TakeHypTrapException::*;
use u_get_SCR_Type_TERR::*;
use Halted::*;
use u_get_HSTR_EL2_Type_T5::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HSTR_Type_T5::*;
use u_get_SCR_EL3_Type_TERR::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_HCR_EL2_Type_TERR::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR2_Type_TERR::*;
use ERXADDR2_read::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn ERXADDR2_SysRegRead32_ee8844bbda44b8b3<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRn: u8,
    opc2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_110591: bool,
        gs_110577: bool,
        u__HSTR_EL2_T5: bool,
        gs_110571: bool,
        gs_110601: bool,
        gs_110580: bool,
        gs_110563: bool,
        gs_110583: bool,
        gs_110564: bool,
        gs_110593: bool,
        gs_110586: bool,
        u__SCR_TERR: bool,
        gs_110603: bool,
        gs_110602: bool,
        gs_110565: bool,
        gs_110600: bool,
        u__HSTR_T5: bool,
        gs_110576: bool,
        gs_110562: bool,
        gs_110587: bool,
        gs_110599: bool,
        gs_110578: bool,
        gs_110589: bool,
        gs_110590: bool,
        gs_110575: bool,
        u__SCR_EL3_TERR: bool,
        gs_110598: bool,
        gs_110566: bool,
        gs_110572: bool,
        gs_110569: bool,
        gs_110573: bool,
        gs_110588: bool,
        u__PSTATE_M: u8,
        gs_110567: bool,
        gs_110579: bool,
        gs_110582: bool,
        gs_110570: bool,
        gs_110568: bool,
        gs_110581: bool,
        gs_110594: bool,
        gs_110574: bool,
        u__PSTATE_EL: u8,
        gs_110597: bool,
        gs_110584: bool,
        gs_110596: bool,
        u__HCR_EL2_TERR: bool,
        gs_110585: bool,
        gs_110595: bool,
        gs_110592: bool,
        u__HCR2_TERR: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRn: u8,
        opc2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRn,
        opc2,
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
        // C s_0_3: const #90704u : u32
        let s_0_3: u32 = 90704;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCR_EL3_Type_TERR(s_0_4)
        let s_0_5: bool = u_get_SCR_EL3_Type_TERR(state, tracer, s_0_4);
        // D s_0_6: write-var __SCR_EL3_TERR <= s_0_5
        fn_state.u__SCR_EL3_TERR = s_0_5;
        // C s_0_7: const #20920u : u32
        let s_0_7: u32 = 20920;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_Type_TERR(s_0_8)
        let s_0_9: bool = u_get_SCR_Type_TERR(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_TERR <= s_0_9
        fn_state.u__SCR_TERR = s_0_9;
        // C s_0_11: const #104936u : u32
        let s_0_11: u32 = 104936;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HSTR_EL2_Type_T5(s_0_12)
        let s_0_13: bool = u_get_HSTR_EL2_Type_T5(state, tracer, s_0_12);
        // D s_0_14: write-var __HSTR_EL2_T5 <= s_0_13
        fn_state.u__HSTR_EL2_T5 = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HSTR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HSTR_Type_T5(s_0_16)
        let s_0_17: bool = u_get_HSTR_Type_T5(state, tracer, s_0_16);
        // D s_0_18: write-var __HSTR_T5 <= s_0_17
        fn_state.u__HSTR_T5 = s_0_17;
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_TERR(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_TERR(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_TERR <= s_0_21
        fn_state.u__HCR_EL2_TERR = s_0_21;
        // C s_0_23: const #() : ()
        let s_0_23: () = ();
        // S s_0_24: call HCR2_read(s_0_23)
        let s_0_24: ProductType700c18a878c5601b = HCR2_read(state, tracer, s_0_23);
        // S s_0_25: call _get_HCR2_Type_TERR(s_0_24)
        let s_0_25: bool = u_get_HCR2_Type_TERR(state, tracer, s_0_24);
        // D s_0_26: write-var __HCR2_TERR <= s_0_25
        fn_state.u__HCR2_TERR = s_0_25;
        // C s_0_27: const #16983u : u32
        let s_0_27: u32 = 16983;
        // D s_0_28: read-reg s_0_27:u8
        let s_0_28: u8 = {
            let value = state.read_register::<u8>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: write-var __PSTATE_M <= s_0_28
        fn_state.u__PSTATE_M = s_0_28;
        // D s_0_30: read-var __PSTATE_EL:u8
        let s_0_30: u8 = fn_state.u__PSTATE_EL;
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 2u16);
        // C s_0_32: const #448u : u32
        let s_0_32: u32 = 448;
        // D s_0_33: read-reg s_0_32:u8
        let s_0_33: u8 = {
            let value = state.read_register::<u8>(s_0_32 as isize);
            tracer.read_register(s_0_32 as isize, value);
            value
        };
        // D s_0_34: cast zx s_0_33 -> bv
        let s_0_34: Bits = Bits::new(s_0_33 as u128, 2u16);
        // D s_0_35: cmp-eq s_0_31 s_0_34
        let s_0_35: bool = ((s_0_31) == (s_0_34));
        // N s_0_36: branch s_0_35 b168 b1
        if s_0_35 {
            return block_168(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b72 b2
        if s_1_5 {
            return block_72(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b11 b3
        if s_2_5 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_0: read-var __PSTATE_M:u8
        let s_5_0: u8 = fn_state.u__PSTATE_M;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 5u16);
        // C s_5_2: const #384u : u32
        let s_5_2: u32 = 384;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 5u16);
        // D s_5_5: cmp-ne s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) != (s_5_4));
        // N s_5_6: branch s_5_5 b10 b6
        if s_5_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#110562 <= s_6_0
        fn_state.gs_110562 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#110562:u8
        let s_7_0: bool = fn_state.gs_110562;
        // N s_7_1: branch s_7_0 b9 b8
        if s_7_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call ERXADDR2_read(s_8_0)
        let s_8_1: u32 = ERXADDR2_read(state, tracer, s_8_0);
        // D s_8_2: read-var t:i
        let s_8_2: i128 = fn_state.t;
        // D s_8_3: call R_set(s_8_2, s_8_1)
        let s_8_3: () = R_set(state, tracer, s_8_2, s_8_1);
        // N s_8_4: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call AArch32_TakeMonitorTrapException(s_9_0)
        let s_9_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_9_0);
        // N s_9_2: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var __SCR_TERR:u8
        let s_10_0: bool = fn_state.u__SCR_TERR;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: write-var gs#110562 <= s_10_4
        fn_state.gs_110562 = s_10_4;
        // N s_10_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call Halted(s_11_0)
        let s_11_1: bool = Halted(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b71 b12
        if s_11_1 {
            return block_71(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#110563 <= s_12_0
        fn_state.gs_110563 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#110563:u8
        let s_13_0: bool = fn_state.gs_110563;
        // N s_13_1: branch s_13_0 b70 b14
        if s_13_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#110564 <= s_14_0
        fn_state.gs_110564 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#110564:u8
        let s_15_0: bool = fn_state.gs_110564;
        // N s_15_1: branch s_15_0 b69 b16
        if s_15_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#110565 <= s_16_0
        fn_state.gs_110565 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#110565:u8
        let s_17_0: bool = fn_state.gs_110565;
        // N s_17_1: branch s_17_0 b68 b18
        if s_17_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#110566 <= s_18_0
        fn_state.gs_110566 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#110566:u8
        let s_19_0: bool = fn_state.gs_110566;
        // N s_19_1: branch s_19_0 b67 b20
        if s_19_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#110567 <= s_20_0
        fn_state.gs_110567 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#110567:u8
        let s_21_0: bool = fn_state.gs_110567;
        // N s_21_1: branch s_21_0 b66 b22
        if s_21_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call Halted(s_22_0)
        let s_22_1: bool = Halted(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b65 b23
        if s_22_1 {
            return block_65(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#110568 <= s_23_0
        fn_state.gs_110568 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#110568:u8
        let s_24_0: bool = fn_state.gs_110568;
        // N s_24_1: branch s_24_0 b64 b25
        if s_24_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#110569 <= s_25_0
        fn_state.gs_110569 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#110569:u8
        let s_26_0: bool = fn_state.gs_110569;
        // N s_26_1: branch s_26_0 b63 b27
        if s_26_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#110570 <= s_27_0
        fn_state.gs_110570 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#110570:u8
        let s_28_0: bool = fn_state.gs_110570;
        // N s_28_1: branch s_28_0 b62 b29
        if s_28_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#110571 <= s_29_0
        fn_state.gs_110571 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#110571:u8
        let s_30_0: bool = fn_state.gs_110571;
        // N s_30_1: branch s_30_0 b61 b31
        if s_30_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#110572 <= s_31_0
        fn_state.gs_110572 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#110572:u8
        let s_32_0: bool = fn_state.gs_110572;
        // N s_32_1: branch s_32_0 b60 b33
        if s_32_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #424u : u32
        let s_33_0: u32 = 424;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // C s_33_2: const #2u : u8
        let s_33_2: u8 = 2;
        // D s_33_3: cmp-lt s_33_1 s_33_2
        let s_33_3: bool = ((s_33_1) < (s_33_2));
        // N s_33_4: branch s_33_3 b59 b34
        if s_33_3 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#110573 <= s_34_0
        fn_state.gs_110573 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#110573:u8
        let s_35_0: bool = fn_state.gs_110573;
        // N s_35_1: branch s_35_0 b58 b36
        if s_35_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#110574 <= s_36_0
        fn_state.gs_110574 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#110574:u8
        let s_37_0: bool = fn_state.gs_110574;
        // N s_37_1: branch s_37_0 b52 b38
        if s_37_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #424u : u32
        let s_38_0: u32 = 424;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // C s_38_2: const #2u : u8
        let s_38_2: u8 = 2;
        // D s_38_3: cmp-lt s_38_1 s_38_2
        let s_38_3: bool = ((s_38_1) < (s_38_2));
        // N s_38_4: branch s_38_3 b51 b39
        if s_38_3 {
            return block_51(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#110575 <= s_39_0
        fn_state.gs_110575 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#110575:u8
        let s_40_0: bool = fn_state.gs_110575;
        // N s_40_1: branch s_40_0 b50 b41
        if s_40_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#110576 <= s_41_0
        fn_state.gs_110576 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#110576:u8
        let s_42_0: bool = fn_state.gs_110576;
        // N s_42_1: branch s_42_0 b44 b43
        if s_42_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call ERXADDR2_read(s_43_0)
        let s_43_1: u32 = ERXADDR2_read(state, tracer, s_43_0);
        // D s_43_2: read-var t:i
        let s_43_2: i128 = fn_state.t;
        // D s_43_3: call R_set(s_43_2, s_43_1)
        let s_43_3: () = R_set(state, tracer, s_43_2, s_43_1);
        // N s_43_4: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call Halted(s_44_0)
        let s_44_1: bool = Halted(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b49 b45
        if s_44_1 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#110577 <= s_45_0
        fn_state.gs_110577 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#110577:u8
        let s_46_0: bool = fn_state.gs_110577;
        // N s_46_1: branch s_46_0 b48 b47
        if s_46_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call AArch32_TakeMonitorTrapException(s_47_0)
        let s_47_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_47_0);
        // N s_47_2: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: panic
        panic!("{:?}", ());
        // N s_48_1: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call EDSCR_read(s_49_0)
        let s_49_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_49_0);
        // S s_49_2: call _get_EDSCR_Type_SDD(s_49_1)
        let s_49_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_49_1);
        // S s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // C s_49_4: const #1u : u8
        let s_49_4: bool = true;
        // C s_49_5: cast zx s_49_4 -> bv
        let s_49_5: Bits = Bits::new(s_49_4 as u128, 1u16);
        // S s_49_6: cmp-eq s_49_3 s_49_5
        let s_49_6: bool = ((s_49_3) == (s_49_5));
        // D s_49_7: write-var gs#110577 <= s_49_6
        fn_state.gs_110577 = s_49_6;
        // N s_49_8: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var __SCR_TERR:u8
        let s_50_0: bool = fn_state.u__SCR_TERR;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 1u16);
        // C s_50_2: const #1u : u8
        let s_50_2: bool = true;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_4: cmp-eq s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) == (s_50_3));
        // D s_50_5: write-var gs#110576 <= s_50_4
        fn_state.gs_110576 = s_50_4;
        // N s_50_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #424u : u32
        let s_51_0: u32 = 424;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call ELUsingAArch32(s_51_1)
        let s_51_2: bool = ELUsingAArch32(state, tracer, s_51_1);
        // D s_51_3: write-var gs#110575 <= s_51_2
        fn_state.gs_110575 = s_51_2;
        // N s_51_4: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call Halted(s_52_0)
        let s_52_1: bool = Halted(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b57 b53
        if s_52_1 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#110578 <= s_53_0
        fn_state.gs_110578 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#110578:u8
        let s_54_0: bool = fn_state.gs_110578;
        // N s_54_1: branch s_54_0 b56 b55
        if s_54_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #3u : u8
        let s_55_0: u8 = 3;
        // C s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 8u16);
        // C s_55_2: cast zx s_55_1 -> i
        let s_55_2: i128 = (s_55_1.value() as i128);
        // C s_55_3: cast reint s_55_2 -> i64
        let s_55_3: i64 = (s_55_2 as i64);
        // C s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // C s_55_5: const #424u : u32
        let s_55_5: u32 = 424;
        // D s_55_6: read-reg s_55_5:u8
        let s_55_6: u8 = {
            let value = state.read_register::<u8>(s_55_5 as isize);
            tracer.read_register(s_55_5 as isize, value);
            value
        };
        // D s_55_7: call AArch64_AArch32SystemAccessTrap(s_55_6, s_55_4)
        let s_55_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_55_6, s_55_4);
        // N s_55_8: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: panic
        panic!("{:?}", ());
        // N s_56_1: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EDSCR_read(s_57_0)
        let s_57_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_57_0);
        // S s_57_2: call _get_EDSCR_Type_SDD(s_57_1)
        let s_57_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_57_1);
        // S s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // C s_57_4: const #1u : u8
        let s_57_4: bool = true;
        // C s_57_5: cast zx s_57_4 -> bv
        let s_57_5: Bits = Bits::new(s_57_4 as u128, 1u16);
        // S s_57_6: cmp-eq s_57_3 s_57_5
        let s_57_6: bool = ((s_57_3) == (s_57_5));
        // D s_57_7: write-var gs#110578 <= s_57_6
        fn_state.gs_110578 = s_57_6;
        // N s_57_8: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var __SCR_EL3_TERR:u8
        let s_58_0: bool = fn_state.u__SCR_EL3_TERR;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 1u16);
        // C s_58_2: const #1u : u8
        let s_58_2: bool = true;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: write-var gs#110574 <= s_58_4
        fn_state.gs_110574 = s_58_4;
        // N s_58_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #424u : u32
        let s_59_0: u32 = 424;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: u8 = {
            let value = state.read_register::<u8>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call ELUsingAArch32(s_59_1)
        let s_59_2: bool = ELUsingAArch32(state, tracer, s_59_1);
        // D s_59_3: not s_59_2
        let s_59_3: bool = !s_59_2;
        // D s_59_4: write-var gs#110573 <= s_59_3
        fn_state.gs_110573 = s_59_3;
        // N s_59_5: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: panic
        panic!("{:?}", ());
        // N s_60_1: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var __SCR_TERR:u8
        let s_61_0: bool = fn_state.u__SCR_TERR;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 1u16);
        // C s_61_2: const #1u : u8
        let s_61_2: bool = true;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: write-var gs#110572 <= s_61_4
        fn_state.gs_110572 = s_61_4;
        // N s_61_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #424u : u32
        let s_62_0: u32 = 424;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call ELUsingAArch32(s_62_1)
        let s_62_2: bool = ELUsingAArch32(state, tracer, s_62_1);
        // D s_62_3: write-var gs#110571 <= s_62_2
        fn_state.gs_110571 = s_62_2;
        // N s_62_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_63_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_63_1: call __IMPDEF_boolean(s_63_0)
        let s_63_1: bool = u__IMPDEF_boolean(state, tracer, s_63_0);
        // D s_63_2: write-var gs#110570 <= s_63_1
        fn_state.gs_110570 = s_63_1;
        // N s_63_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call EDSCR_read(s_64_0)
        let s_64_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_64_0);
        // S s_64_2: call _get_EDSCR_Type_SDD(s_64_1)
        let s_64_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_64_1);
        // S s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // C s_64_4: const #1u : u8
        let s_64_4: bool = true;
        // C s_64_5: cast zx s_64_4 -> bv
        let s_64_5: Bits = Bits::new(s_64_4 as u128, 1u16);
        // S s_64_6: cmp-eq s_64_3 s_64_5
        let s_64_6: bool = ((s_64_3) == (s_64_5));
        // D s_64_7: write-var gs#110569 <= s_64_6
        fn_state.gs_110569 = s_64_6;
        // N s_64_8: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #424u : u32
        let s_65_0: u32 = 424;
        // D s_65_1: read-reg s_65_0:u8
        let s_65_1: u8 = {
            let value = state.read_register::<u8>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // C s_65_2: const #2u : u8
        let s_65_2: u8 = 2;
        // D s_65_3: cmp-lt s_65_1 s_65_2
        let s_65_3: bool = ((s_65_1) < (s_65_2));
        // D s_65_4: write-var gs#110568 <= s_65_3
        fn_state.gs_110568 = s_65_3;
        // N s_65_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_66_0: panic
        panic!("{:?}", ());
        // N s_66_1: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __SCR_EL3_TERR:u8
        let s_67_0: bool = fn_state.u__SCR_EL3_TERR;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#110567 <= s_67_4
        fn_state.gs_110567 = s_67_4;
        // N s_67_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #424u : u32
        let s_68_0: u32 = 424;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call ELUsingAArch32(s_68_1)
        let s_68_2: bool = ELUsingAArch32(state, tracer, s_68_1);
        // D s_68_3: not s_68_2
        let s_68_3: bool = !s_68_2;
        // D s_68_4: write-var gs#110566 <= s_68_3
        fn_state.gs_110566 = s_68_3;
        // N s_68_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_69_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_69_1: call __IMPDEF_boolean(s_69_0)
        let s_69_1: bool = u__IMPDEF_boolean(state, tracer, s_69_0);
        // D s_69_2: write-var gs#110565 <= s_69_1
        fn_state.gs_110565 = s_69_1;
        // N s_69_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call EDSCR_read(s_70_0)
        let s_70_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_70_0);
        // S s_70_2: call _get_EDSCR_Type_SDD(s_70_1)
        let s_70_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_70_1);
        // S s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // C s_70_4: const #1u : u8
        let s_70_4: bool = true;
        // C s_70_5: cast zx s_70_4 -> bv
        let s_70_5: Bits = Bits::new(s_70_4 as u128, 1u16);
        // S s_70_6: cmp-eq s_70_3 s_70_5
        let s_70_6: bool = ((s_70_3) == (s_70_5));
        // D s_70_7: write-var gs#110564 <= s_70_6
        fn_state.gs_110564 = s_70_6;
        // N s_70_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #424u : u32
        let s_71_0: u32 = 424;
        // D s_71_1: read-reg s_71_0:u8
        let s_71_1: u8 = {
            let value = state.read_register::<u8>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // C s_71_2: const #2u : u8
        let s_71_2: u8 = 2;
        // D s_71_3: cmp-lt s_71_1 s_71_2
        let s_71_3: bool = ((s_71_1) < (s_71_2));
        // D s_71_4: write-var gs#110563 <= s_71_3
        fn_state.gs_110563 = s_71_3;
        // N s_71_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call Halted(s_72_0)
        let s_72_1: bool = Halted(state, tracer, s_72_0);
        // N s_72_2: branch s_72_1 b167 b73
        if s_72_1 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#110579 <= s_73_0
        fn_state.gs_110579 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#110579:u8
        let s_74_0: bool = fn_state.gs_110579;
        // N s_74_1: branch s_74_0 b166 b75
        if s_74_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#110580 <= s_75_0
        fn_state.gs_110580 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#110580:u8
        let s_76_0: bool = fn_state.gs_110580;
        // N s_76_1: branch s_76_0 b165 b77
        if s_76_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#110581 <= s_77_0
        fn_state.gs_110581 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#110581:u8
        let s_78_0: bool = fn_state.gs_110581;
        // N s_78_1: branch s_78_0 b164 b79
        if s_78_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#110582 <= s_79_0
        fn_state.gs_110582 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#110582:u8
        let s_80_0: bool = fn_state.gs_110582;
        // N s_80_1: branch s_80_0 b163 b81
        if s_80_0 {
            return block_163(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#110583 <= s_81_0
        fn_state.gs_110583 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#110583:u8
        let s_82_0: bool = fn_state.gs_110583;
        // N s_82_1: branch s_82_0 b162 b83
        if s_82_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call Halted(s_83_0)
        let s_83_1: bool = Halted(state, tracer, s_83_0);
        // N s_83_2: branch s_83_1 b161 b84
        if s_83_1 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var gs#110584 <= s_84_0
        fn_state.gs_110584 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#110584:u8
        let s_85_0: bool = fn_state.gs_110584;
        // N s_85_1: branch s_85_0 b160 b86
        if s_85_0 {
            return block_160(state, tracer, fn_state);
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
        // D s_86_1: write-var gs#110585 <= s_86_0
        fn_state.gs_110585 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#110585:u8
        let s_87_0: bool = fn_state.gs_110585;
        // N s_87_1: branch s_87_0 b159 b88
        if s_87_0 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#110586 <= s_88_0
        fn_state.gs_110586 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#110586:u8
        let s_89_0: bool = fn_state.gs_110586;
        // N s_89_1: branch s_89_0 b158 b90
        if s_89_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#110587 <= s_90_0
        fn_state.gs_110587 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#110587:u8
        let s_91_0: bool = fn_state.gs_110587;
        // N s_91_1: branch s_91_0 b157 b92
        if s_91_0 {
            return block_157(state, tracer, fn_state);
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
        // D s_92_1: write-var gs#110588 <= s_92_0
        fn_state.gs_110588 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#110588:u8
        let s_93_0: bool = fn_state.gs_110588;
        // N s_93_1: branch s_93_0 b156 b94
        if s_93_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #() : ()
        let s_94_0: () = ();
        // S s_94_1: call EL2Enabled(s_94_0)
        let s_94_1: bool = EL2Enabled(state, tracer, s_94_0);
        // N s_94_2: branch s_94_1 b155 b95
        if s_94_1 {
            return block_155(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#110589 <= s_95_0
        fn_state.gs_110589 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#110589:u8
        let s_96_0: bool = fn_state.gs_110589;
        // N s_96_1: branch s_96_0 b154 b97
        if s_96_0 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#110590 <= s_97_0
        fn_state.gs_110590 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#110590:u8
        let s_98_0: bool = fn_state.gs_110590;
        // N s_98_1: branch s_98_0 b153 b99
        if s_98_0 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #() : ()
        let s_99_0: () = ();
        // S s_99_1: call EL2Enabled(s_99_0)
        let s_99_1: bool = EL2Enabled(state, tracer, s_99_0);
        // N s_99_2: branch s_99_1 b152 b100
        if s_99_1 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #0u : u8
        let s_100_0: bool = false;
        // D s_100_1: write-var gs#110591 <= s_100_0
        fn_state.gs_110591 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#110591:u8
        let s_101_0: bool = fn_state.gs_110591;
        // N s_101_1: branch s_101_0 b151 b102
        if s_101_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // D s_102_1: write-var gs#110592 <= s_102_0
        fn_state.gs_110592 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#110592:u8
        let s_103_0: bool = fn_state.gs_110592;
        // N s_103_1: branch s_103_0 b150 b104
        if s_103_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #() : ()
        let s_104_0: () = ();
        // S s_104_1: call EL2Enabled(s_104_0)
        let s_104_1: bool = EL2Enabled(state, tracer, s_104_0);
        // N s_104_2: branch s_104_1 b149 b105
        if s_104_1 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#110593 <= s_105_0
        fn_state.gs_110593 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#110593:u8
        let s_106_0: bool = fn_state.gs_110593;
        // N s_106_1: branch s_106_0 b148 b107
        if s_106_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#110594 <= s_107_0
        fn_state.gs_110594 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#110594:u8
        let s_108_0: bool = fn_state.gs_110594;
        // N s_108_1: branch s_108_0 b147 b109
        if s_108_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call EL2Enabled(s_109_0)
        let s_109_1: bool = EL2Enabled(state, tracer, s_109_0);
        // N s_109_2: branch s_109_1 b146 b110
        if s_109_1 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #0u : u8
        let s_110_0: bool = false;
        // D s_110_1: write-var gs#110595 <= s_110_0
        fn_state.gs_110595 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#110595:u8
        let s_111_0: bool = fn_state.gs_110595;
        // N s_111_1: branch s_111_0 b145 b112
        if s_111_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#110596 <= s_112_0
        fn_state.gs_110596 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#110596:u8
        let s_113_0: bool = fn_state.gs_110596;
        // N s_113_1: branch s_113_0 b144 b114
        if s_113_0 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #424u : u32
        let s_114_0: u32 = 424;
        // D s_114_1: read-reg s_114_0:u8
        let s_114_1: u8 = {
            let value = state.read_register::<u8>(s_114_0 as isize);
            tracer.read_register(s_114_0 as isize, value);
            value
        };
        // C s_114_2: const #2u : u8
        let s_114_2: u8 = 2;
        // D s_114_3: cmp-lt s_114_1 s_114_2
        let s_114_3: bool = ((s_114_1) < (s_114_2));
        // N s_114_4: branch s_114_3 b143 b115
        if s_114_3 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#110597 <= s_115_0
        fn_state.gs_110597 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#110597:u8
        let s_116_0: bool = fn_state.gs_110597;
        // N s_116_1: branch s_116_0 b142 b117
        if s_116_0 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0u : u8
        let s_117_0: bool = false;
        // D s_117_1: write-var gs#110598 <= s_117_0
        fn_state.gs_110598 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#110598:u8
        let s_118_0: bool = fn_state.gs_110598;
        // N s_118_1: branch s_118_0 b136 b119
        if s_118_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #424u : u32
        let s_119_0: u32 = 424;
        // D s_119_1: read-reg s_119_0:u8
        let s_119_1: u8 = {
            let value = state.read_register::<u8>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // C s_119_2: const #2u : u8
        let s_119_2: u8 = 2;
        // D s_119_3: cmp-lt s_119_1 s_119_2
        let s_119_3: bool = ((s_119_1) < (s_119_2));
        // N s_119_4: branch s_119_3 b135 b120
        if s_119_3 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #0u : u8
        let s_120_0: bool = false;
        // D s_120_1: write-var gs#110599 <= s_120_0
        fn_state.gs_110599 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#110599:u8
        let s_121_0: bool = fn_state.gs_110599;
        // N s_121_1: branch s_121_0 b134 b122
        if s_121_0 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#110600 <= s_122_0
        fn_state.gs_110600 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#110600:u8
        let s_123_0: bool = fn_state.gs_110600;
        // N s_123_1: branch s_123_0 b133 b124
        if s_123_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var gs#110601 <= s_124_0
        fn_state.gs_110601 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#110601:u8
        let s_125_0: bool = fn_state.gs_110601;
        // N s_125_1: branch s_125_0 b127 b126
        if s_125_0 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #() : ()
        let s_126_0: () = ();
        // S s_126_1: call ERXADDR2_read(s_126_0)
        let s_126_1: u32 = ERXADDR2_read(state, tracer, s_126_0);
        // D s_126_2: read-var t:i
        let s_126_2: i128 = fn_state.t;
        // D s_126_3: call R_set(s_126_2, s_126_1)
        let s_126_3: () = R_set(state, tracer, s_126_2, s_126_1);
        // N s_126_4: return
        return;
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #() : ()
        let s_127_0: () = ();
        // S s_127_1: call Halted(s_127_0)
        let s_127_1: bool = Halted(state, tracer, s_127_0);
        // N s_127_2: branch s_127_1 b132 b128
        if s_127_1 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // D s_128_1: write-var gs#110602 <= s_128_0
        fn_state.gs_110602 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#110602:u8
        let s_129_0: bool = fn_state.gs_110602;
        // N s_129_1: branch s_129_0 b131 b130
        if s_129_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #() : ()
        let s_130_0: () = ();
        // S s_130_1: call AArch32_TakeMonitorTrapException(s_130_0)
        let s_130_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_130_0);
        // N s_130_2: return
        return;
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_131_0: panic
        panic!("{:?}", ());
        // N s_131_1: return
        return;
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #() : ()
        let s_132_0: () = ();
        // S s_132_1: call EDSCR_read(s_132_0)
        let s_132_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_132_0);
        // S s_132_2: call _get_EDSCR_Type_SDD(s_132_1)
        let s_132_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_132_1);
        // S s_132_3: cast zx s_132_2 -> bv
        let s_132_3: Bits = Bits::new(s_132_2 as u128, 1u16);
        // C s_132_4: const #1u : u8
        let s_132_4: bool = true;
        // C s_132_5: cast zx s_132_4 -> bv
        let s_132_5: Bits = Bits::new(s_132_4 as u128, 1u16);
        // S s_132_6: cmp-eq s_132_3 s_132_5
        let s_132_6: bool = ((s_132_3) == (s_132_5));
        // D s_132_7: write-var gs#110602 <= s_132_6
        fn_state.gs_110602 = s_132_6;
        // N s_132_8: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var __SCR_TERR:u8
        let s_133_0: bool = fn_state.u__SCR_TERR;
        // D s_133_1: cast zx s_133_0 -> bv
        let s_133_1: Bits = Bits::new(s_133_0 as u128, 1u16);
        // C s_133_2: const #1u : u8
        let s_133_2: bool = true;
        // C s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 1u16);
        // D s_133_4: cmp-eq s_133_1 s_133_3
        let s_133_4: bool = ((s_133_1) == (s_133_3));
        // D s_133_5: write-var gs#110601 <= s_133_4
        fn_state.gs_110601 = s_133_4;
        // N s_133_6: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var __PSTATE_M:u8
        let s_134_0: u8 = fn_state.u__PSTATE_M;
        // D s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 5u16);
        // C s_134_2: const #384u : u32
        let s_134_2: u32 = 384;
        // D s_134_3: read-reg s_134_2:u8
        let s_134_3: u8 = {
            let value = state.read_register::<u8>(s_134_2 as isize);
            tracer.read_register(s_134_2 as isize, value);
            value
        };
        // D s_134_4: cast zx s_134_3 -> bv
        let s_134_4: Bits = Bits::new(s_134_3 as u128, 5u16);
        // D s_134_5: cmp-ne s_134_1 s_134_4
        let s_134_5: bool = ((s_134_1) != (s_134_4));
        // D s_134_6: write-var gs#110600 <= s_134_5
        fn_state.gs_110600 = s_134_5;
        // N s_134_7: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #424u : u32
        let s_135_0: u32 = 424;
        // D s_135_1: read-reg s_135_0:u8
        let s_135_1: u8 = {
            let value = state.read_register::<u8>(s_135_0 as isize);
            tracer.read_register(s_135_0 as isize, value);
            value
        };
        // D s_135_2: call ELUsingAArch32(s_135_1)
        let s_135_2: bool = ELUsingAArch32(state, tracer, s_135_1);
        // D s_135_3: write-var gs#110599 <= s_135_2
        fn_state.gs_110599 = s_135_2;
        // N s_135_4: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #() : ()
        let s_136_0: () = ();
        // S s_136_1: call Halted(s_136_0)
        let s_136_1: bool = Halted(state, tracer, s_136_0);
        // N s_136_2: branch s_136_1 b141 b137
        if s_136_1 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #0u : u8
        let s_137_0: bool = false;
        // D s_137_1: write-var gs#110603 <= s_137_0
        fn_state.gs_110603 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#110603:u8
        let s_138_0: bool = fn_state.gs_110603;
        // N s_138_1: branch s_138_0 b140 b139
        if s_138_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #3u : u8
        let s_139_0: u8 = 3;
        // C s_139_1: cast zx s_139_0 -> bv
        let s_139_1: Bits = Bits::new(s_139_0 as u128, 8u16);
        // C s_139_2: cast zx s_139_1 -> i
        let s_139_2: i128 = (s_139_1.value() as i128);
        // C s_139_3: cast reint s_139_2 -> i64
        let s_139_3: i64 = (s_139_2 as i64);
        // C s_139_4: cast zx s_139_3 -> i
        let s_139_4: i128 = (i128::try_from(s_139_3).unwrap());
        // C s_139_5: const #424u : u32
        let s_139_5: u32 = 424;
        // D s_139_6: read-reg s_139_5:u8
        let s_139_6: u8 = {
            let value = state.read_register::<u8>(s_139_5 as isize);
            tracer.read_register(s_139_5 as isize, value);
            value
        };
        // D s_139_7: call AArch64_AArch32SystemAccessTrap(s_139_6, s_139_4)
        let s_139_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_139_6,
            s_139_4,
        );
        // N s_139_8: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_140_0: panic
        panic!("{:?}", ());
        // N s_140_1: return
        return;
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #() : ()
        let s_141_0: () = ();
        // S s_141_1: call EDSCR_read(s_141_0)
        let s_141_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_141_0);
        // S s_141_2: call _get_EDSCR_Type_SDD(s_141_1)
        let s_141_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_141_1);
        // S s_141_3: cast zx s_141_2 -> bv
        let s_141_3: Bits = Bits::new(s_141_2 as u128, 1u16);
        // C s_141_4: const #1u : u8
        let s_141_4: bool = true;
        // C s_141_5: cast zx s_141_4 -> bv
        let s_141_5: Bits = Bits::new(s_141_4 as u128, 1u16);
        // S s_141_6: cmp-eq s_141_3 s_141_5
        let s_141_6: bool = ((s_141_3) == (s_141_5));
        // D s_141_7: write-var gs#110603 <= s_141_6
        fn_state.gs_110603 = s_141_6;
        // N s_141_8: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var __SCR_EL3_TERR:u8
        let s_142_0: bool = fn_state.u__SCR_EL3_TERR;
        // D s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 1u16);
        // C s_142_2: const #1u : u8
        let s_142_2: bool = true;
        // C s_142_3: cast zx s_142_2 -> bv
        let s_142_3: Bits = Bits::new(s_142_2 as u128, 1u16);
        // D s_142_4: cmp-eq s_142_1 s_142_3
        let s_142_4: bool = ((s_142_1) == (s_142_3));
        // D s_142_5: write-var gs#110598 <= s_142_4
        fn_state.gs_110598 = s_142_4;
        // N s_142_6: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #424u : u32
        let s_143_0: u32 = 424;
        // D s_143_1: read-reg s_143_0:u8
        let s_143_1: u8 = {
            let value = state.read_register::<u8>(s_143_0 as isize);
            tracer.read_register(s_143_0 as isize, value);
            value
        };
        // D s_143_2: call ELUsingAArch32(s_143_1)
        let s_143_2: bool = ELUsingAArch32(state, tracer, s_143_1);
        // D s_143_3: not s_143_2
        let s_143_3: bool = !s_143_2;
        // D s_143_4: write-var gs#110597 <= s_143_3
        fn_state.gs_110597 = s_143_3;
        // N s_143_5: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #3u : u8
        let s_144_0: u8 = 3;
        // C s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 8u16);
        // C s_144_2: cast zx s_144_1 -> i
        let s_144_2: i128 = (s_144_1.value() as i128);
        // C s_144_3: cast reint s_144_2 -> i64
        let s_144_3: i64 = (s_144_2 as i64);
        // C s_144_4: cast zx s_144_3 -> i
        let s_144_4: i128 = (i128::try_from(s_144_3).unwrap());
        // S s_144_5: call AArch32_TakeHypTrapException(s_144_4)
        let s_144_5: () = AArch32_TakeHypTrapException(state, tracer, s_144_4);
        // N s_144_6: return
        return;
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var __HCR2_TERR:u8
        let s_145_0: bool = fn_state.u__HCR2_TERR;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 1u16);
        // C s_145_2: const #1u : u8
        let s_145_2: bool = true;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // D s_145_5: write-var gs#110596 <= s_145_4
        fn_state.gs_110596 = s_145_4;
        // N s_145_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #432u : u32
        let s_146_0: u32 = 432;
        // D s_146_1: read-reg s_146_0:u8
        let s_146_1: u8 = {
            let value = state.read_register::<u8>(s_146_0 as isize);
            tracer.read_register(s_146_0 as isize, value);
            value
        };
        // D s_146_2: call ELUsingAArch32(s_146_1)
        let s_146_2: bool = ELUsingAArch32(state, tracer, s_146_1);
        // D s_146_3: write-var gs#110595 <= s_146_2
        fn_state.gs_110595 = s_146_2;
        // N s_146_4: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #3u : u8
        let s_147_0: u8 = 3;
        // C s_147_1: cast zx s_147_0 -> bv
        let s_147_1: Bits = Bits::new(s_147_0 as u128, 8u16);
        // C s_147_2: cast zx s_147_1 -> i
        let s_147_2: i128 = (s_147_1.value() as i128);
        // C s_147_3: cast reint s_147_2 -> i64
        let s_147_3: i64 = (s_147_2 as i64);
        // C s_147_4: cast zx s_147_3 -> i
        let s_147_4: i128 = (i128::try_from(s_147_3).unwrap());
        // C s_147_5: const #432u : u32
        let s_147_5: u32 = 432;
        // D s_147_6: read-reg s_147_5:u8
        let s_147_6: u8 = {
            let value = state.read_register::<u8>(s_147_5 as isize);
            tracer.read_register(s_147_5 as isize, value);
            value
        };
        // D s_147_7: call AArch64_AArch32SystemAccessTrap(s_147_6, s_147_4)
        let s_147_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_147_6,
            s_147_4,
        );
        // N s_147_8: return
        return;
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var __HCR_EL2_TERR:u8
        let s_148_0: bool = fn_state.u__HCR_EL2_TERR;
        // D s_148_1: cast zx s_148_0 -> bv
        let s_148_1: Bits = Bits::new(s_148_0 as u128, 1u16);
        // C s_148_2: const #1u : u8
        let s_148_2: bool = true;
        // C s_148_3: cast zx s_148_2 -> bv
        let s_148_3: Bits = Bits::new(s_148_2 as u128, 1u16);
        // D s_148_4: cmp-eq s_148_1 s_148_3
        let s_148_4: bool = ((s_148_1) == (s_148_3));
        // D s_148_5: write-var gs#110594 <= s_148_4
        fn_state.gs_110594 = s_148_4;
        // N s_148_6: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #432u : u32
        let s_149_0: u32 = 432;
        // D s_149_1: read-reg s_149_0:u8
        let s_149_1: u8 = {
            let value = state.read_register::<u8>(s_149_0 as isize);
            tracer.read_register(s_149_0 as isize, value);
            value
        };
        // D s_149_2: call ELUsingAArch32(s_149_1)
        let s_149_2: bool = ELUsingAArch32(state, tracer, s_149_1);
        // D s_149_3: not s_149_2
        let s_149_3: bool = !s_149_2;
        // D s_149_4: write-var gs#110593 <= s_149_3
        fn_state.gs_110593 = s_149_3;
        // N s_149_5: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #3u : u8
        let s_150_0: u8 = 3;
        // C s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 8u16);
        // C s_150_2: cast zx s_150_1 -> i
        let s_150_2: i128 = (s_150_1.value() as i128);
        // C s_150_3: cast reint s_150_2 -> i64
        let s_150_3: i64 = (s_150_2 as i64);
        // C s_150_4: cast zx s_150_3 -> i
        let s_150_4: i128 = (i128::try_from(s_150_3).unwrap());
        // S s_150_5: call AArch32_TakeHypTrapException(s_150_4)
        let s_150_5: () = AArch32_TakeHypTrapException(state, tracer, s_150_4);
        // N s_150_6: return
        return;
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var __HSTR_T5:u8
        let s_151_0: bool = fn_state.u__HSTR_T5;
        // D s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 1u16);
        // C s_151_2: const #1u : u8
        let s_151_2: bool = true;
        // C s_151_3: cast zx s_151_2 -> bv
        let s_151_3: Bits = Bits::new(s_151_2 as u128, 1u16);
        // D s_151_4: cmp-eq s_151_1 s_151_3
        let s_151_4: bool = ((s_151_1) == (s_151_3));
        // D s_151_5: write-var gs#110592 <= s_151_4
        fn_state.gs_110592 = s_151_4;
        // N s_151_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #432u : u32
        let s_152_0: u32 = 432;
        // D s_152_1: read-reg s_152_0:u8
        let s_152_1: u8 = {
            let value = state.read_register::<u8>(s_152_0 as isize);
            tracer.read_register(s_152_0 as isize, value);
            value
        };
        // D s_152_2: call ELUsingAArch32(s_152_1)
        let s_152_2: bool = ELUsingAArch32(state, tracer, s_152_1);
        // D s_152_3: write-var gs#110591 <= s_152_2
        fn_state.gs_110591 = s_152_2;
        // N s_152_4: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #3u : u8
        let s_153_0: u8 = 3;
        // C s_153_1: cast zx s_153_0 -> bv
        let s_153_1: Bits = Bits::new(s_153_0 as u128, 8u16);
        // C s_153_2: cast zx s_153_1 -> i
        let s_153_2: i128 = (s_153_1.value() as i128);
        // C s_153_3: cast reint s_153_2 -> i64
        let s_153_3: i64 = (s_153_2 as i64);
        // C s_153_4: cast zx s_153_3 -> i
        let s_153_4: i128 = (i128::try_from(s_153_3).unwrap());
        // C s_153_5: const #432u : u32
        let s_153_5: u32 = 432;
        // D s_153_6: read-reg s_153_5:u8
        let s_153_6: u8 = {
            let value = state.read_register::<u8>(s_153_5 as isize);
            tracer.read_register(s_153_5 as isize, value);
            value
        };
        // D s_153_7: call AArch64_AArch32SystemAccessTrap(s_153_6, s_153_4)
        let s_153_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_153_6,
            s_153_4,
        );
        // N s_153_8: return
        return;
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var __HSTR_EL2_T5:u8
        let s_154_0: bool = fn_state.u__HSTR_EL2_T5;
        // D s_154_1: cast zx s_154_0 -> bv
        let s_154_1: Bits = Bits::new(s_154_0 as u128, 1u16);
        // C s_154_2: const #1u : u8
        let s_154_2: bool = true;
        // C s_154_3: cast zx s_154_2 -> bv
        let s_154_3: Bits = Bits::new(s_154_2 as u128, 1u16);
        // D s_154_4: cmp-eq s_154_1 s_154_3
        let s_154_4: bool = ((s_154_1) == (s_154_3));
        // D s_154_5: write-var gs#110590 <= s_154_4
        fn_state.gs_110590 = s_154_4;
        // N s_154_6: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #432u : u32
        let s_155_0: u32 = 432;
        // D s_155_1: read-reg s_155_0:u8
        let s_155_1: u8 = {
            let value = state.read_register::<u8>(s_155_0 as isize);
            tracer.read_register(s_155_0 as isize, value);
            value
        };
        // D s_155_2: call ELUsingAArch32(s_155_1)
        let s_155_2: bool = ELUsingAArch32(state, tracer, s_155_1);
        // D s_155_3: not s_155_2
        let s_155_3: bool = !s_155_2;
        // D s_155_4: write-var gs#110589 <= s_155_3
        fn_state.gs_110589 = s_155_3;
        // N s_155_5: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_156_0: panic
        panic!("{:?}", ());
        // N s_156_1: return
        return;
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var __SCR_TERR:u8
        let s_157_0: bool = fn_state.u__SCR_TERR;
        // D s_157_1: cast zx s_157_0 -> bv
        let s_157_1: Bits = Bits::new(s_157_0 as u128, 1u16);
        // C s_157_2: const #1u : u8
        let s_157_2: bool = true;
        // C s_157_3: cast zx s_157_2 -> bv
        let s_157_3: Bits = Bits::new(s_157_2 as u128, 1u16);
        // D s_157_4: cmp-eq s_157_1 s_157_3
        let s_157_4: bool = ((s_157_1) == (s_157_3));
        // D s_157_5: write-var gs#110588 <= s_157_4
        fn_state.gs_110588 = s_157_4;
        // N s_157_6: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #424u : u32
        let s_158_0: u32 = 424;
        // D s_158_1: read-reg s_158_0:u8
        let s_158_1: u8 = {
            let value = state.read_register::<u8>(s_158_0 as isize);
            tracer.read_register(s_158_0 as isize, value);
            value
        };
        // D s_158_2: call ELUsingAArch32(s_158_1)
        let s_158_2: bool = ELUsingAArch32(state, tracer, s_158_1);
        // D s_158_3: write-var gs#110587 <= s_158_2
        fn_state.gs_110587 = s_158_2;
        // N s_158_4: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_159_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_159_1: call __IMPDEF_boolean(s_159_0)
        let s_159_1: bool = u__IMPDEF_boolean(state, tracer, s_159_0);
        // D s_159_2: write-var gs#110586 <= s_159_1
        fn_state.gs_110586 = s_159_1;
        // N s_159_3: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #() : ()
        let s_160_0: () = ();
        // S s_160_1: call EDSCR_read(s_160_0)
        let s_160_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_160_0);
        // S s_160_2: call _get_EDSCR_Type_SDD(s_160_1)
        let s_160_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_160_1);
        // S s_160_3: cast zx s_160_2 -> bv
        let s_160_3: Bits = Bits::new(s_160_2 as u128, 1u16);
        // C s_160_4: const #1u : u8
        let s_160_4: bool = true;
        // C s_160_5: cast zx s_160_4 -> bv
        let s_160_5: Bits = Bits::new(s_160_4 as u128, 1u16);
        // S s_160_6: cmp-eq s_160_3 s_160_5
        let s_160_6: bool = ((s_160_3) == (s_160_5));
        // D s_160_7: write-var gs#110585 <= s_160_6
        fn_state.gs_110585 = s_160_6;
        // N s_160_8: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #424u : u32
        let s_161_0: u32 = 424;
        // D s_161_1: read-reg s_161_0:u8
        let s_161_1: u8 = {
            let value = state.read_register::<u8>(s_161_0 as isize);
            tracer.read_register(s_161_0 as isize, value);
            value
        };
        // C s_161_2: const #2u : u8
        let s_161_2: u8 = 2;
        // D s_161_3: cmp-lt s_161_1 s_161_2
        let s_161_3: bool = ((s_161_1) < (s_161_2));
        // D s_161_4: write-var gs#110584 <= s_161_3
        fn_state.gs_110584 = s_161_3;
        // N s_161_5: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_162_0: panic
        panic!("{:?}", ());
        // N s_162_1: return
        return;
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var __SCR_EL3_TERR:u8
        let s_163_0: bool = fn_state.u__SCR_EL3_TERR;
        // D s_163_1: cast zx s_163_0 -> bv
        let s_163_1: Bits = Bits::new(s_163_0 as u128, 1u16);
        // C s_163_2: const #1u : u8
        let s_163_2: bool = true;
        // C s_163_3: cast zx s_163_2 -> bv
        let s_163_3: Bits = Bits::new(s_163_2 as u128, 1u16);
        // D s_163_4: cmp-eq s_163_1 s_163_3
        let s_163_4: bool = ((s_163_1) == (s_163_3));
        // D s_163_5: write-var gs#110583 <= s_163_4
        fn_state.gs_110583 = s_163_4;
        // N s_163_6: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #424u : u32
        let s_164_0: u32 = 424;
        // D s_164_1: read-reg s_164_0:u8
        let s_164_1: u8 = {
            let value = state.read_register::<u8>(s_164_0 as isize);
            tracer.read_register(s_164_0 as isize, value);
            value
        };
        // D s_164_2: call ELUsingAArch32(s_164_1)
        let s_164_2: bool = ELUsingAArch32(state, tracer, s_164_1);
        // D s_164_3: not s_164_2
        let s_164_3: bool = !s_164_2;
        // D s_164_4: write-var gs#110582 <= s_164_3
        fn_state.gs_110582 = s_164_3;
        // N s_164_5: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_165_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_165_1: call __IMPDEF_boolean(s_165_0)
        let s_165_1: bool = u__IMPDEF_boolean(state, tracer, s_165_0);
        // D s_165_2: write-var gs#110581 <= s_165_1
        fn_state.gs_110581 = s_165_1;
        // N s_165_3: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #() : ()
        let s_166_0: () = ();
        // S s_166_1: call EDSCR_read(s_166_0)
        let s_166_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_166_0);
        // S s_166_2: call _get_EDSCR_Type_SDD(s_166_1)
        let s_166_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_166_1);
        // S s_166_3: cast zx s_166_2 -> bv
        let s_166_3: Bits = Bits::new(s_166_2 as u128, 1u16);
        // C s_166_4: const #1u : u8
        let s_166_4: bool = true;
        // C s_166_5: cast zx s_166_4 -> bv
        let s_166_5: Bits = Bits::new(s_166_4 as u128, 1u16);
        // S s_166_6: cmp-eq s_166_3 s_166_5
        let s_166_6: bool = ((s_166_3) == (s_166_5));
        // D s_166_7: write-var gs#110580 <= s_166_6
        fn_state.gs_110580 = s_166_6;
        // N s_166_8: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #424u : u32
        let s_167_0: u32 = 424;
        // D s_167_1: read-reg s_167_0:u8
        let s_167_1: u8 = {
            let value = state.read_register::<u8>(s_167_0 as isize);
            tracer.read_register(s_167_0 as isize, value);
            value
        };
        // C s_167_2: const #2u : u8
        let s_167_2: u8 = 2;
        // D s_167_3: cmp-lt s_167_1 s_167_2
        let s_167_3: bool = ((s_167_1) < (s_167_2));
        // D s_167_4: write-var gs#110579 <= s_167_3
        fn_state.gs_110579 = s_167_3;
        // N s_167_5: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_168_0: panic
        panic!("{:?}", ());
        // N s_168_1: return
        return;
    }
}
