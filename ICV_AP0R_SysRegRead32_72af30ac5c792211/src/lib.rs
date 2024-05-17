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
use AArch32_TakeHypTrapException::*;
use u_get_ICC_SRE_Type_SRE::*;
use Halted::*;
use ICC_HSRE_read::*;
use ICC_SRE_read::*;
use u_get_HCR_Type_FMO::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HSTR_Type_T12::*;
use u_get_ICC_MSRE_Type_SRE::*;
use u_get_ICH_HCR_Type_TALL0::*;
use ICV_AP0R_read::*;
use u_get_SCR_Type_FIQ::*;
use HCR_read::*;
use ICC_AP0R_read::*;
use u_get_ICC_HSRE_Type_SRE::*;
use u__IMPDEF_boolean::*;
use u_get_SCR_EL3_Type_FIQ::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use u_get_HCR_EL2_Type_FMO::*;
use AArch64_AArch32SystemAccessTrap::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_ICH_HCR_EL2_Type_TALL0::*;
use ICH_HCR_read::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn ICV_AP0R_SysRegRead32_72af30ac5c792211<T: Tracer>(
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
        u__SCR_FIQ: bool,
        gs_112058: bool,
        gs_112031: bool,
        gs_112021: bool,
        gs_112039: bool,
        gs_112050: bool,
        gs_112022: bool,
        gs_112033: bool,
        gs_112040: bool,
        gs_112044: bool,
        gs_112014: bool,
        gs_112043: bool,
        gs_112012: bool,
        u__HSTR_EL2_T12: bool,
        gs_112032: bool,
        u__HCR_EL2_FMO: bool,
        gs_112013: bool,
        gs_112027: bool,
        u__ICH_HCR_TALL0: bool,
        gs_112038: bool,
        gs_112041: bool,
        gs_112047: bool,
        u__PSTATE_EL: u8,
        gs_112016: bool,
        gs_112036: bool,
        gs_112026: bool,
        gs_112049: bool,
        gs_112037: bool,
        gs_112053: bool,
        gs_112042: bool,
        gs_112045: bool,
        gs_112011: bool,
        gs_112048: bool,
        gs_112028: bool,
        gs_112019: bool,
        gs_112024: bool,
        gs_112023: bool,
        u__ICC_HSRE_SRE: bool,
        gs_112030: bool,
        u__HSTR_T12: bool,
        gs_112015: bool,
        u__ICH_HCR_EL2_TALL0: bool,
        u__SCR_EL3_FIQ: bool,
        gs_112046: bool,
        gs_112020: bool,
        u__ICC_MSRE_SRE: bool,
        gs_112035: bool,
        gs_112054: bool,
        gs_112018: bool,
        gs_112052: bool,
        u__HCR_FMO: bool,
        u__ICC_SRE_SRE: bool,
        u__PSTATE_M: u8,
        gs_112051: bool,
        gs_112034: bool,
        gs_112057: bool,
        gs_112055: bool,
        gs_112029: bool,
        gs_112017: bool,
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
        // D s_0_5: call _get_SCR_EL3_Type_FIQ(s_0_4)
        let s_0_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_0_4);
        // D s_0_6: write-var __SCR_EL3_FIQ <= s_0_5
        fn_state.u__SCR_EL3_FIQ = s_0_5;
        // C s_0_7: const #16983u : u32
        let s_0_7: u32 = 16983;
        // D s_0_8: read-reg s_0_7:u8
        let s_0_8: u8 = {
            let value = state.read_register::<u8>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: write-var __PSTATE_M <= s_0_8
        fn_state.u__PSTATE_M = s_0_8;
        // C s_0_10: const #20920u : u32
        let s_0_10: u32 = 20920;
        // D s_0_11: read-reg s_0_10:struct
        let s_0_11: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: call _get_SCR_Type_FIQ(s_0_11)
        let s_0_12: bool = u_get_SCR_Type_FIQ(state, tracer, s_0_11);
        // D s_0_13: write-var __SCR_FIQ <= s_0_12
        fn_state.u__SCR_FIQ = s_0_12;
        // C s_0_14: const #104936u : u32
        let s_0_14: u32 = 104936;
        // D s_0_15: read-reg s_0_14:struct
        let s_0_15: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // D s_0_16: call _get_HSTR_EL2_Type_T12(s_0_15)
        let s_0_16: bool = u_get_HSTR_EL2_Type_T12(state, tracer, s_0_15);
        // D s_0_17: write-var __HSTR_EL2_T12 <= s_0_16
        fn_state.u__HSTR_EL2_T12 = s_0_16;
        // C s_0_18: const #() : ()
        let s_0_18: () = ();
        // S s_0_19: call HSTR_read(s_0_18)
        let s_0_19: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_18);
        // S s_0_20: call _get_HSTR_Type_T12(s_0_19)
        let s_0_20: bool = u_get_HSTR_Type_T12(state, tracer, s_0_19);
        // D s_0_21: write-var __HSTR_T12 <= s_0_20
        fn_state.u__HSTR_T12 = s_0_20;
        // C s_0_22: const #() : ()
        let s_0_22: () = ();
        // S s_0_23: call ICC_SRE_read(s_0_22)
        let s_0_23: ProductType700c18a878c5601b = ICC_SRE_read(state, tracer, s_0_22);
        // S s_0_24: call _get_ICC_SRE_Type_SRE(s_0_23)
        let s_0_24: bool = u_get_ICC_SRE_Type_SRE(state, tracer, s_0_23);
        // D s_0_25: write-var __ICC_SRE_SRE <= s_0_24
        fn_state.u__ICC_SRE_SRE = s_0_24;
        // C s_0_26: const #20992u : u32
        let s_0_26: u32 = 20992;
        // D s_0_27: read-reg s_0_26:struct
        let s_0_27: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_26 as isize);
            tracer.read_register(s_0_26 as isize, value);
            value
        };
        // D s_0_28: call _get_ICH_HCR_EL2_Type_TALL0(s_0_27)
        let s_0_28: bool = u_get_ICH_HCR_EL2_Type_TALL0(state, tracer, s_0_27);
        // D s_0_29: write-var __ICH_HCR_EL2_TALL0 <= s_0_28
        fn_state.u__ICH_HCR_EL2_TALL0 = s_0_28;
        // C s_0_30: const #() : ()
        let s_0_30: () = ();
        // S s_0_31: call ICH_HCR_read(s_0_30)
        let s_0_31: ProductType700c18a878c5601b = ICH_HCR_read(state, tracer, s_0_30);
        // S s_0_32: call _get_ICH_HCR_Type_TALL0(s_0_31)
        let s_0_32: bool = u_get_ICH_HCR_Type_TALL0(state, tracer, s_0_31);
        // D s_0_33: write-var __ICH_HCR_TALL0 <= s_0_32
        fn_state.u__ICH_HCR_TALL0 = s_0_32;
        // C s_0_34: const #102552u : u32
        let s_0_34: u32 = 102552;
        // D s_0_35: read-reg s_0_34:struct
        let s_0_35: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_34 as isize);
            tracer.read_register(s_0_34 as isize, value);
            value
        };
        // D s_0_36: call _get_HCR_EL2_Type_FMO(s_0_35)
        let s_0_36: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_0_35);
        // D s_0_37: write-var __HCR_EL2_FMO <= s_0_36
        fn_state.u__HCR_EL2_FMO = s_0_36;
        // C s_0_38: const #() : ()
        let s_0_38: () = ();
        // S s_0_39: call HCR_read(s_0_38)
        let s_0_39: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_38);
        // S s_0_40: call _get_HCR_Type_FMO(s_0_39)
        let s_0_40: bool = u_get_HCR_Type_FMO(state, tracer, s_0_39);
        // D s_0_41: write-var __HCR_FMO <= s_0_40
        fn_state.u__HCR_FMO = s_0_40;
        // C s_0_42: const #() : ()
        let s_0_42: () = ();
        // S s_0_43: call ICC_HSRE_read(s_0_42)
        let s_0_43: ProductType700c18a878c5601b = ICC_HSRE_read(state, tracer, s_0_42);
        // S s_0_44: call _get_ICC_HSRE_Type_SRE(s_0_43)
        let s_0_44: bool = u_get_ICC_HSRE_Type_SRE(state, tracer, s_0_43);
        // D s_0_45: write-var __ICC_HSRE_SRE <= s_0_44
        fn_state.u__ICC_HSRE_SRE = s_0_44;
        // C s_0_46: const #19992u : u32
        let s_0_46: u32 = 19992;
        // D s_0_47: read-reg s_0_46:struct
        let s_0_47: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_46 as isize);
            tracer.read_register(s_0_46 as isize, value);
            value
        };
        // D s_0_48: call _get_ICC_MSRE_Type_SRE(s_0_47)
        let s_0_48: bool = u_get_ICC_MSRE_Type_SRE(state, tracer, s_0_47);
        // D s_0_49: write-var __ICC_MSRE_SRE <= s_0_48
        fn_state.u__ICC_MSRE_SRE = s_0_48;
        // N s_0_50: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
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
        // C s_2_2: const #448u : u32
        let s_2_2: u32 = 448;
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
        // N s_2_6: branch s_2_5 b190 b3
        if s_2_5 {
            return block_190(state, tracer, fn_state);
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
        // C s_3_2: const #440u : u32
        let s_3_2: u32 = 440;
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
        // N s_3_6: branch s_3_5 b73 b4
        if s_3_5 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #432u : u32
        let s_4_2: u32 = 432;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b10 b5
        if s_4_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var __PSTATE_EL:u8
        let s_5_0: u8 = fn_state.u__PSTATE_EL;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #424u : u32
        let s_5_2: u32 = 424;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var __ICC_MSRE_SRE:u8
        let s_7_0: bool = fn_state.u__ICC_MSRE_SRE;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #0u : u8
        let s_7_2: bool = false;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
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
        // C s_8_0: const #0s : i64
        let s_8_0: i64 = 0;
        // S s_8_1: call ICC_AP0R_read(s_8_0)
        let s_8_1: u32 = ICC_AP0R_read(state, tracer, s_8_0);
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
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call Halted(s_10_0)
        let s_10_1: bool = Halted(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b72 b11
        if s_10_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#112011 <= s_11_0
        fn_state.gs_112011 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#112011:u8
        let s_12_0: bool = fn_state.gs_112011;
        // N s_12_1: branch s_12_0 b71 b13
        if s_12_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#112012 <= s_13_0
        fn_state.gs_112012 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#112012:u8
        let s_14_0: bool = fn_state.gs_112012;
        // N s_14_1: branch s_14_0 b70 b15
        if s_14_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#112013 <= s_15_0
        fn_state.gs_112013 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#112013:u8
        let s_16_0: bool = fn_state.gs_112013;
        // N s_16_1: branch s_16_0 b69 b17
        if s_16_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#112014 <= s_17_0
        fn_state.gs_112014 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#112014:u8
        let s_18_0: bool = fn_state.gs_112014;
        // N s_18_1: branch s_18_0 b68 b19
        if s_18_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#112015 <= s_19_0
        fn_state.gs_112015 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#112015:u8
        let s_20_0: bool = fn_state.gs_112015;
        // N s_20_1: branch s_20_0 b67 b21
        if s_20_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call Halted(s_21_0)
        let s_21_1: bool = Halted(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b66 b22
        if s_21_1 {
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
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#112016 <= s_22_0
        fn_state.gs_112016 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#112016:u8
        let s_23_0: bool = fn_state.gs_112016;
        // N s_23_1: branch s_23_0 b65 b24
        if s_23_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#112017 <= s_24_0
        fn_state.gs_112017 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#112017:u8
        let s_25_0: bool = fn_state.gs_112017;
        // N s_25_1: branch s_25_0 b64 b26
        if s_25_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#112018 <= s_26_0
        fn_state.gs_112018 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#112018:u8
        let s_27_0: bool = fn_state.gs_112018;
        // N s_27_1: branch s_27_0 b63 b28
        if s_27_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#112019 <= s_28_0
        fn_state.gs_112019 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#112019:u8
        let s_29_0: bool = fn_state.gs_112019;
        // N s_29_1: branch s_29_0 b62 b30
        if s_29_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#112020 <= s_30_0
        fn_state.gs_112020 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#112020:u8
        let s_31_0: bool = fn_state.gs_112020;
        // N s_31_1: branch s_31_0 b61 b32
        if s_31_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __ICC_HSRE_SRE:u8
        let s_32_0: bool = fn_state.u__ICC_HSRE_SRE;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #0u : u8
        let s_32_2: bool = false;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // N s_32_5: branch s_32_4 b60 b33
        if s_32_4 {
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
        // D s_34_1: write-var gs#112021 <= s_34_0
        fn_state.gs_112021 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#112021:u8
        let s_35_0: bool = fn_state.gs_112021;
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
        // D s_36_1: write-var gs#112022 <= s_36_0
        fn_state.gs_112022 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#112022:u8
        let s_37_0: bool = fn_state.gs_112022;
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
        // D s_39_1: write-var gs#112023 <= s_39_0
        fn_state.gs_112023 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#112023:u8
        let s_40_0: bool = fn_state.gs_112023;
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
        // D s_41_1: write-var gs#112024 <= s_41_0
        fn_state.gs_112024 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#112024:u8
        let s_42_0: bool = fn_state.gs_112024;
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
        // C s_43_0: const #0s : i64
        let s_43_0: i64 = 0;
        // S s_43_1: call ICC_AP0R_read(s_43_0)
        let s_43_1: u32 = ICC_AP0R_read(state, tracer, s_43_0);
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
        // D s_45_1: write-var gs#112026 <= s_45_0
        fn_state.gs_112026 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#112026:u8
        let s_46_0: bool = fn_state.gs_112026;
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
        // D s_49_7: write-var gs#112026 <= s_49_6
        fn_state.gs_112026 = s_49_6;
        // N s_49_8: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var __SCR_FIQ:u8
        let s_50_0: bool = fn_state.u__SCR_FIQ;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 1u16);
        // C s_50_2: const #1u : u8
        let s_50_2: bool = true;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_4: cmp-eq s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) == (s_50_3));
        // D s_50_5: write-var gs#112024 <= s_50_4
        fn_state.gs_112024 = s_50_4;
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
        // D s_51_3: write-var gs#112023 <= s_51_2
        fn_state.gs_112023 = s_51_2;
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
        // D s_53_1: write-var gs#112027 <= s_53_0
        fn_state.gs_112027 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#112027:u8
        let s_54_0: bool = fn_state.gs_112027;
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
        // D s_57_7: write-var gs#112027 <= s_57_6
        fn_state.gs_112027 = s_57_6;
        // N s_57_8: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var __SCR_EL3_FIQ:u8
        let s_58_0: bool = fn_state.u__SCR_EL3_FIQ;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 1u16);
        // C s_58_2: const #1u : u8
        let s_58_2: bool = true;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: write-var gs#112022 <= s_58_4
        fn_state.gs_112022 = s_58_4;
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
        // D s_59_4: write-var gs#112021 <= s_59_3
        fn_state.gs_112021 = s_59_3;
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
        // N s_61_0: panic
        panic!("{:?}", ());
        // N s_61_1: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __SCR_FIQ:u8
        let s_62_0: bool = fn_state.u__SCR_FIQ;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#112020 <= s_62_4
        fn_state.gs_112020 = s_62_4;
        // N s_62_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #424u : u32
        let s_63_0: u32 = 424;
        // D s_63_1: read-reg s_63_0:u8
        let s_63_1: u8 = {
            let value = state.read_register::<u8>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call ELUsingAArch32(s_63_1)
        let s_63_2: bool = ELUsingAArch32(state, tracer, s_63_1);
        // D s_63_3: write-var gs#112019 <= s_63_2
        fn_state.gs_112019 = s_63_2;
        // N s_63_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_64_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_64_1: call __IMPDEF_boolean(s_64_0)
        let s_64_1: bool = u__IMPDEF_boolean(state, tracer, s_64_0);
        // D s_64_2: write-var gs#112018 <= s_64_1
        fn_state.gs_112018 = s_64_1;
        // N s_64_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call EDSCR_read(s_65_0)
        let s_65_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_65_0);
        // S s_65_2: call _get_EDSCR_Type_SDD(s_65_1)
        let s_65_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_65_1);
        // S s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // C s_65_4: const #1u : u8
        let s_65_4: bool = true;
        // C s_65_5: cast zx s_65_4 -> bv
        let s_65_5: Bits = Bits::new(s_65_4 as u128, 1u16);
        // S s_65_6: cmp-eq s_65_3 s_65_5
        let s_65_6: bool = ((s_65_3) == (s_65_5));
        // D s_65_7: write-var gs#112017 <= s_65_6
        fn_state.gs_112017 = s_65_6;
        // N s_65_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #424u : u32
        let s_66_0: u32 = 424;
        // D s_66_1: read-reg s_66_0:u8
        let s_66_1: u8 = {
            let value = state.read_register::<u8>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // C s_66_2: const #2u : u8
        let s_66_2: u8 = 2;
        // D s_66_3: cmp-lt s_66_1 s_66_2
        let s_66_3: bool = ((s_66_1) < (s_66_2));
        // D s_66_4: write-var gs#112016 <= s_66_3
        fn_state.gs_112016 = s_66_3;
        // N s_66_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: panic
        panic!("{:?}", ());
        // N s_67_1: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var __SCR_EL3_FIQ:u8
        let s_68_0: bool = fn_state.u__SCR_EL3_FIQ;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #1u : u8
        let s_68_2: bool = true;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#112015 <= s_68_4
        fn_state.gs_112015 = s_68_4;
        // N s_68_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #424u : u32
        let s_69_0: u32 = 424;
        // D s_69_1: read-reg s_69_0:u8
        let s_69_1: u8 = {
            let value = state.read_register::<u8>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call ELUsingAArch32(s_69_1)
        let s_69_2: bool = ELUsingAArch32(state, tracer, s_69_1);
        // D s_69_3: not s_69_2
        let s_69_3: bool = !s_69_2;
        // D s_69_4: write-var gs#112014 <= s_69_3
        fn_state.gs_112014 = s_69_3;
        // N s_69_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_70_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_70_1: call __IMPDEF_boolean(s_70_0)
        let s_70_1: bool = u__IMPDEF_boolean(state, tracer, s_70_0);
        // D s_70_2: write-var gs#112013 <= s_70_1
        fn_state.gs_112013 = s_70_1;
        // N s_70_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call EDSCR_read(s_71_0)
        let s_71_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_71_0);
        // S s_71_2: call _get_EDSCR_Type_SDD(s_71_1)
        let s_71_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_71_1);
        // S s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // C s_71_4: const #1u : u8
        let s_71_4: bool = true;
        // C s_71_5: cast zx s_71_4 -> bv
        let s_71_5: Bits = Bits::new(s_71_4 as u128, 1u16);
        // S s_71_6: cmp-eq s_71_3 s_71_5
        let s_71_6: bool = ((s_71_3) == (s_71_5));
        // D s_71_7: write-var gs#112012 <= s_71_6
        fn_state.gs_112012 = s_71_6;
        // N s_71_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #424u : u32
        let s_72_0: u32 = 424;
        // D s_72_1: read-reg s_72_0:u8
        let s_72_1: u8 = {
            let value = state.read_register::<u8>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // C s_72_2: const #2u : u8
        let s_72_2: u8 = 2;
        // D s_72_3: cmp-lt s_72_1 s_72_2
        let s_72_3: bool = ((s_72_1) < (s_72_2));
        // D s_72_4: write-var gs#112011 <= s_72_3
        fn_state.gs_112011 = s_72_3;
        // N s_72_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call Halted(s_73_0)
        let s_73_1: bool = Halted(state, tracer, s_73_0);
        // N s_73_2: branch s_73_1 b189 b74
        if s_73_1 {
            return block_189(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#112028 <= s_74_0
        fn_state.gs_112028 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#112028:u8
        let s_75_0: bool = fn_state.gs_112028;
        // N s_75_1: branch s_75_0 b188 b76
        if s_75_0 {
            return block_188(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#112029 <= s_76_0
        fn_state.gs_112029 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#112029:u8
        let s_77_0: bool = fn_state.gs_112029;
        // N s_77_1: branch s_77_0 b187 b78
        if s_77_0 {
            return block_187(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#112030 <= s_78_0
        fn_state.gs_112030 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#112030:u8
        let s_79_0: bool = fn_state.gs_112030;
        // N s_79_1: branch s_79_0 b186 b80
        if s_79_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#112031 <= s_80_0
        fn_state.gs_112031 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#112031:u8
        let s_81_0: bool = fn_state.gs_112031;
        // N s_81_1: branch s_81_0 b185 b82
        if s_81_0 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var gs#112032 <= s_82_0
        fn_state.gs_112032 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#112032:u8
        let s_83_0: bool = fn_state.gs_112032;
        // N s_83_1: branch s_83_0 b184 b84
        if s_83_0 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call Halted(s_84_0)
        let s_84_1: bool = Halted(state, tracer, s_84_0);
        // N s_84_2: branch s_84_1 b183 b85
        if s_84_1 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#112033 <= s_85_0
        fn_state.gs_112033 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#112033:u8
        let s_86_0: bool = fn_state.gs_112033;
        // N s_86_1: branch s_86_0 b182 b87
        if s_86_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#112034 <= s_87_0
        fn_state.gs_112034 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#112034:u8
        let s_88_0: bool = fn_state.gs_112034;
        // N s_88_1: branch s_88_0 b181 b89
        if s_88_0 {
            return block_181(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#112035 <= s_89_0
        fn_state.gs_112035 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#112035:u8
        let s_90_0: bool = fn_state.gs_112035;
        // N s_90_1: branch s_90_0 b180 b91
        if s_90_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // D s_91_1: write-var gs#112036 <= s_91_0
        fn_state.gs_112036 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#112036:u8
        let s_92_0: bool = fn_state.gs_112036;
        // N s_92_1: branch s_92_0 b179 b93
        if s_92_0 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#112037 <= s_93_0
        fn_state.gs_112037 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#112037:u8
        let s_94_0: bool = fn_state.gs_112037;
        // N s_94_1: branch s_94_0 b178 b95
        if s_94_0 {
            return block_178(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#112038 <= s_95_0
        fn_state.gs_112038 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#112038:u8
        let s_96_0: bool = fn_state.gs_112038;
        // N s_96_1: branch s_96_0 b177 b97
        if s_96_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #() : ()
        let s_97_0: () = ();
        // S s_97_1: call EL2Enabled(s_97_0)
        let s_97_1: bool = EL2Enabled(state, tracer, s_97_0);
        // N s_97_2: branch s_97_1 b176 b98
        if s_97_1 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#112039 <= s_98_0
        fn_state.gs_112039 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#112039:u8
        let s_99_0: bool = fn_state.gs_112039;
        // N s_99_1: branch s_99_0 b175 b100
        if s_99_0 {
            return block_175(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#112040 <= s_100_0
        fn_state.gs_112040 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#112040:u8
        let s_101_0: bool = fn_state.gs_112040;
        // N s_101_1: branch s_101_0 b174 b102
        if s_101_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #() : ()
        let s_102_0: () = ();
        // S s_102_1: call EL2Enabled(s_102_0)
        let s_102_1: bool = EL2Enabled(state, tracer, s_102_0);
        // N s_102_2: branch s_102_1 b173 b103
        if s_102_1 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#112041 <= s_103_0
        fn_state.gs_112041 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#112041:u8
        let s_104_0: bool = fn_state.gs_112041;
        // N s_104_1: branch s_104_0 b172 b105
        if s_104_0 {
            return block_172(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#112042 <= s_105_0
        fn_state.gs_112042 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#112042:u8
        let s_106_0: bool = fn_state.gs_112042;
        // N s_106_1: branch s_106_0 b171 b107
        if s_106_0 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var __ICC_SRE_SRE:u8
        let s_107_0: bool = fn_state.u__ICC_SRE_SRE;
        // D s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 1u16);
        // C s_107_2: const #0u : u8
        let s_107_2: bool = false;
        // C s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // D s_107_4: cmp-eq s_107_1 s_107_3
        let s_107_4: bool = ((s_107_1) == (s_107_3));
        // N s_107_5: branch s_107_4 b170 b108
        if s_107_4 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #() : ()
        let s_108_0: () = ();
        // S s_108_1: call EL2Enabled(s_108_0)
        let s_108_1: bool = EL2Enabled(state, tracer, s_108_0);
        // N s_108_2: branch s_108_1 b169 b109
        if s_108_1 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#112043 <= s_109_0
        fn_state.gs_112043 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#112043:u8
        let s_110_0: bool = fn_state.gs_112043;
        // N s_110_1: branch s_110_0 b168 b111
        if s_110_0 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #0u : u8
        let s_111_0: bool = false;
        // D s_111_1: write-var gs#112044 <= s_111_0
        fn_state.gs_112044 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#112044:u8
        let s_112_0: bool = fn_state.gs_112044;
        // N s_112_1: branch s_112_0 b167 b113
        if s_112_0 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #() : ()
        let s_113_0: () = ();
        // S s_113_1: call EL2Enabled(s_113_0)
        let s_113_1: bool = EL2Enabled(state, tracer, s_113_0);
        // N s_113_2: branch s_113_1 b166 b114
        if s_113_1 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#112045 <= s_114_0
        fn_state.gs_112045 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#112045:u8
        let s_115_0: bool = fn_state.gs_112045;
        // N s_115_1: branch s_115_0 b165 b116
        if s_115_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#112046 <= s_116_0
        fn_state.gs_112046 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#112046:u8
        let s_117_0: bool = fn_state.gs_112046;
        // N s_117_1: branch s_117_0 b164 b118
        if s_117_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call EL2Enabled(s_118_0)
        let s_118_1: bool = EL2Enabled(state, tracer, s_118_0);
        // N s_118_2: branch s_118_1 b163 b119
        if s_118_1 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#112047 <= s_119_0
        fn_state.gs_112047 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#112047:u8
        let s_120_0: bool = fn_state.gs_112047;
        // N s_120_1: branch s_120_0 b162 b121
        if s_120_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#112048 <= s_121_0
        fn_state.gs_112048 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#112048:u8
        let s_122_0: bool = fn_state.gs_112048;
        // N s_122_1: branch s_122_0 b161 b123
        if s_122_0 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #() : ()
        let s_123_0: () = ();
        // S s_123_1: call EL2Enabled(s_123_0)
        let s_123_1: bool = EL2Enabled(state, tracer, s_123_0);
        // N s_123_2: branch s_123_1 b160 b124
        if s_123_1 {
            return block_160(state, tracer, fn_state);
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
        // D s_124_1: write-var gs#112049 <= s_124_0
        fn_state.gs_112049 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#112049:u8
        let s_125_0: bool = fn_state.gs_112049;
        // N s_125_1: branch s_125_0 b159 b126
        if s_125_0 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #0u : u8
        let s_126_0: bool = false;
        // D s_126_1: write-var gs#112050 <= s_126_0
        fn_state.gs_112050 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#112050:u8
        let s_127_0: bool = fn_state.gs_112050;
        // N s_127_1: branch s_127_0 b158 b128
        if s_127_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #424u : u32
        let s_128_0: u32 = 424;
        // D s_128_1: read-reg s_128_0:u8
        let s_128_1: u8 = {
            let value = state.read_register::<u8>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // C s_128_2: const #2u : u8
        let s_128_2: u8 = 2;
        // D s_128_3: cmp-lt s_128_1 s_128_2
        let s_128_3: bool = ((s_128_1) < (s_128_2));
        // N s_128_4: branch s_128_3 b157 b129
        if s_128_3 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #0u : u8
        let s_129_0: bool = false;
        // D s_129_1: write-var gs#112051 <= s_129_0
        fn_state.gs_112051 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#112051:u8
        let s_130_0: bool = fn_state.gs_112051;
        // N s_130_1: branch s_130_0 b156 b131
        if s_130_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #0u : u8
        let s_131_0: bool = false;
        // D s_131_1: write-var gs#112052 <= s_131_0
        fn_state.gs_112052 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#112052:u8
        let s_132_0: bool = fn_state.gs_112052;
        // N s_132_1: branch s_132_0 b150 b133
        if s_132_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #424u : u32
        let s_133_0: u32 = 424;
        // D s_133_1: read-reg s_133_0:u8
        let s_133_1: u8 = {
            let value = state.read_register::<u8>(s_133_0 as isize);
            tracer.read_register(s_133_0 as isize, value);
            value
        };
        // C s_133_2: const #2u : u8
        let s_133_2: u8 = 2;
        // D s_133_3: cmp-lt s_133_1 s_133_2
        let s_133_3: bool = ((s_133_1) < (s_133_2));
        // N s_133_4: branch s_133_3 b149 b134
        if s_133_3 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#112053 <= s_134_0
        fn_state.gs_112053 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#112053:u8
        let s_135_0: bool = fn_state.gs_112053;
        // N s_135_1: branch s_135_0 b148 b136
        if s_135_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#112054 <= s_136_0
        fn_state.gs_112054 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#112054:u8
        let s_137_0: bool = fn_state.gs_112054;
        // N s_137_1: branch s_137_0 b147 b138
        if s_137_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var gs#112055 <= s_138_0
        fn_state.gs_112055 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#112055:u8
        let s_139_0: bool = fn_state.gs_112055;
        // N s_139_1: branch s_139_0 b141 b140
        if s_139_0 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #0s : i64
        let s_140_0: i64 = 0;
        // S s_140_1: call ICC_AP0R_read(s_140_0)
        let s_140_1: u32 = ICC_AP0R_read(state, tracer, s_140_0);
        // D s_140_2: read-var t:i
        let s_140_2: i128 = fn_state.t;
        // D s_140_3: call R_set(s_140_2, s_140_1)
        let s_140_3: () = R_set(state, tracer, s_140_2, s_140_1);
        // N s_140_4: return
        return;
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #() : ()
        let s_141_0: () = ();
        // S s_141_1: call Halted(s_141_0)
        let s_141_1: bool = Halted(state, tracer, s_141_0);
        // N s_141_2: branch s_141_1 b146 b142
        if s_141_1 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var gs#112057 <= s_142_0
        fn_state.gs_112057 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#112057:u8
        let s_143_0: bool = fn_state.gs_112057;
        // N s_143_1: branch s_143_0 b145 b144
        if s_143_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #() : ()
        let s_144_0: () = ();
        // S s_144_1: call AArch32_TakeMonitorTrapException(s_144_0)
        let s_144_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_144_0);
        // N s_144_2: return
        return;
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_145_0: panic
        panic!("{:?}", ());
        // N s_145_1: return
        return;
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #() : ()
        let s_146_0: () = ();
        // S s_146_1: call EDSCR_read(s_146_0)
        let s_146_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_146_0);
        // S s_146_2: call _get_EDSCR_Type_SDD(s_146_1)
        let s_146_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_146_1);
        // S s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // C s_146_4: const #1u : u8
        let s_146_4: bool = true;
        // C s_146_5: cast zx s_146_4 -> bv
        let s_146_5: Bits = Bits::new(s_146_4 as u128, 1u16);
        // S s_146_6: cmp-eq s_146_3 s_146_5
        let s_146_6: bool = ((s_146_3) == (s_146_5));
        // D s_146_7: write-var gs#112057 <= s_146_6
        fn_state.gs_112057 = s_146_6;
        // N s_146_8: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var __SCR_FIQ:u8
        let s_147_0: bool = fn_state.u__SCR_FIQ;
        // D s_147_1: cast zx s_147_0 -> bv
        let s_147_1: Bits = Bits::new(s_147_0 as u128, 1u16);
        // C s_147_2: const #1u : u8
        let s_147_2: bool = true;
        // C s_147_3: cast zx s_147_2 -> bv
        let s_147_3: Bits = Bits::new(s_147_2 as u128, 1u16);
        // D s_147_4: cmp-eq s_147_1 s_147_3
        let s_147_4: bool = ((s_147_1) == (s_147_3));
        // D s_147_5: write-var gs#112055 <= s_147_4
        fn_state.gs_112055 = s_147_4;
        // N s_147_6: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var __PSTATE_M:u8
        let s_148_0: u8 = fn_state.u__PSTATE_M;
        // D s_148_1: cast zx s_148_0 -> bv
        let s_148_1: Bits = Bits::new(s_148_0 as u128, 5u16);
        // C s_148_2: const #384u : u32
        let s_148_2: u32 = 384;
        // D s_148_3: read-reg s_148_2:u8
        let s_148_3: u8 = {
            let value = state.read_register::<u8>(s_148_2 as isize);
            tracer.read_register(s_148_2 as isize, value);
            value
        };
        // D s_148_4: cast zx s_148_3 -> bv
        let s_148_4: Bits = Bits::new(s_148_3 as u128, 5u16);
        // D s_148_5: cmp-ne s_148_1 s_148_4
        let s_148_5: bool = ((s_148_1) != (s_148_4));
        // D s_148_6: write-var gs#112054 <= s_148_5
        fn_state.gs_112054 = s_148_5;
        // N s_148_7: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #424u : u32
        let s_149_0: u32 = 424;
        // D s_149_1: read-reg s_149_0:u8
        let s_149_1: u8 = {
            let value = state.read_register::<u8>(s_149_0 as isize);
            tracer.read_register(s_149_0 as isize, value);
            value
        };
        // D s_149_2: call ELUsingAArch32(s_149_1)
        let s_149_2: bool = ELUsingAArch32(state, tracer, s_149_1);
        // D s_149_3: write-var gs#112053 <= s_149_2
        fn_state.gs_112053 = s_149_2;
        // N s_149_4: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #() : ()
        let s_150_0: () = ();
        // S s_150_1: call Halted(s_150_0)
        let s_150_1: bool = Halted(state, tracer, s_150_0);
        // N s_150_2: branch s_150_1 b155 b151
        if s_150_1 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: bool = false;
        // D s_151_1: write-var gs#112058 <= s_151_0
        fn_state.gs_112058 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#112058:u8
        let s_152_0: bool = fn_state.gs_112058;
        // N s_152_1: branch s_152_0 b154 b153
        if s_152_0 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
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
        // C s_153_5: const #424u : u32
        let s_153_5: u32 = 424;
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
        // N s_154_0: panic
        panic!("{:?}", ());
        // N s_154_1: return
        return;
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #() : ()
        let s_155_0: () = ();
        // S s_155_1: call EDSCR_read(s_155_0)
        let s_155_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_155_0);
        // S s_155_2: call _get_EDSCR_Type_SDD(s_155_1)
        let s_155_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_155_1);
        // S s_155_3: cast zx s_155_2 -> bv
        let s_155_3: Bits = Bits::new(s_155_2 as u128, 1u16);
        // C s_155_4: const #1u : u8
        let s_155_4: bool = true;
        // C s_155_5: cast zx s_155_4 -> bv
        let s_155_5: Bits = Bits::new(s_155_4 as u128, 1u16);
        // S s_155_6: cmp-eq s_155_3 s_155_5
        let s_155_6: bool = ((s_155_3) == (s_155_5));
        // D s_155_7: write-var gs#112058 <= s_155_6
        fn_state.gs_112058 = s_155_6;
        // N s_155_8: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var __SCR_EL3_FIQ:u8
        let s_156_0: bool = fn_state.u__SCR_EL3_FIQ;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 1u16);
        // C s_156_2: const #1u : u8
        let s_156_2: bool = true;
        // C s_156_3: cast zx s_156_2 -> bv
        let s_156_3: Bits = Bits::new(s_156_2 as u128, 1u16);
        // D s_156_4: cmp-eq s_156_1 s_156_3
        let s_156_4: bool = ((s_156_1) == (s_156_3));
        // D s_156_5: write-var gs#112052 <= s_156_4
        fn_state.gs_112052 = s_156_4;
        // N s_156_6: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #424u : u32
        let s_157_0: u32 = 424;
        // D s_157_1: read-reg s_157_0:u8
        let s_157_1: u8 = {
            let value = state.read_register::<u8>(s_157_0 as isize);
            tracer.read_register(s_157_0 as isize, value);
            value
        };
        // D s_157_2: call ELUsingAArch32(s_157_1)
        let s_157_2: bool = ELUsingAArch32(state, tracer, s_157_1);
        // D s_157_3: not s_157_2
        let s_157_3: bool = !s_157_2;
        // D s_157_4: write-var gs#112051 <= s_157_3
        fn_state.gs_112051 = s_157_3;
        // N s_157_5: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #0s : i64
        let s_158_0: i64 = 0;
        // S s_158_1: call ICV_AP0R_read(s_158_0)
        let s_158_1: u32 = ICV_AP0R_read(state, tracer, s_158_0);
        // D s_158_2: read-var t:i
        let s_158_2: i128 = fn_state.t;
        // D s_158_3: call R_set(s_158_2, s_158_1)
        let s_158_3: () = R_set(state, tracer, s_158_2, s_158_1);
        // N s_158_4: return
        return;
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var __HCR_FMO:u8
        let s_159_0: bool = fn_state.u__HCR_FMO;
        // D s_159_1: cast zx s_159_0 -> bv
        let s_159_1: Bits = Bits::new(s_159_0 as u128, 1u16);
        // C s_159_2: const #1u : u8
        let s_159_2: bool = true;
        // C s_159_3: cast zx s_159_2 -> bv
        let s_159_3: Bits = Bits::new(s_159_2 as u128, 1u16);
        // D s_159_4: cmp-eq s_159_1 s_159_3
        let s_159_4: bool = ((s_159_1) == (s_159_3));
        // D s_159_5: write-var gs#112050 <= s_159_4
        fn_state.gs_112050 = s_159_4;
        // N s_159_6: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #432u : u32
        let s_160_0: u32 = 432;
        // D s_160_1: read-reg s_160_0:u8
        let s_160_1: u8 = {
            let value = state.read_register::<u8>(s_160_0 as isize);
            tracer.read_register(s_160_0 as isize, value);
            value
        };
        // D s_160_2: call ELUsingAArch32(s_160_1)
        let s_160_2: bool = ELUsingAArch32(state, tracer, s_160_1);
        // D s_160_3: write-var gs#112049 <= s_160_2
        fn_state.gs_112049 = s_160_2;
        // N s_160_4: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #0s : i64
        let s_161_0: i64 = 0;
        // S s_161_1: call ICV_AP0R_read(s_161_0)
        let s_161_1: u32 = ICV_AP0R_read(state, tracer, s_161_0);
        // D s_161_2: read-var t:i
        let s_161_2: i128 = fn_state.t;
        // D s_161_3: call R_set(s_161_2, s_161_1)
        let s_161_3: () = R_set(state, tracer, s_161_2, s_161_1);
        // N s_161_4: return
        return;
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var __HCR_EL2_FMO:u8
        let s_162_0: bool = fn_state.u__HCR_EL2_FMO;
        // D s_162_1: cast zx s_162_0 -> bv
        let s_162_1: Bits = Bits::new(s_162_0 as u128, 1u16);
        // C s_162_2: const #1u : u8
        let s_162_2: bool = true;
        // C s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 1u16);
        // D s_162_4: cmp-eq s_162_1 s_162_3
        let s_162_4: bool = ((s_162_1) == (s_162_3));
        // D s_162_5: write-var gs#112048 <= s_162_4
        fn_state.gs_112048 = s_162_4;
        // N s_162_6: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #432u : u32
        let s_163_0: u32 = 432;
        // D s_163_1: read-reg s_163_0:u8
        let s_163_1: u8 = {
            let value = state.read_register::<u8>(s_163_0 as isize);
            tracer.read_register(s_163_0 as isize, value);
            value
        };
        // D s_163_2: call ELUsingAArch32(s_163_1)
        let s_163_2: bool = ELUsingAArch32(state, tracer, s_163_1);
        // D s_163_3: not s_163_2
        let s_163_3: bool = !s_163_2;
        // D s_163_4: write-var gs#112047 <= s_163_3
        fn_state.gs_112047 = s_163_3;
        // N s_163_5: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #3u : u8
        let s_164_0: u8 = 3;
        // C s_164_1: cast zx s_164_0 -> bv
        let s_164_1: Bits = Bits::new(s_164_0 as u128, 8u16);
        // C s_164_2: cast zx s_164_1 -> i
        let s_164_2: i128 = (s_164_1.value() as i128);
        // C s_164_3: cast reint s_164_2 -> i64
        let s_164_3: i64 = (s_164_2 as i64);
        // C s_164_4: cast zx s_164_3 -> i
        let s_164_4: i128 = (i128::try_from(s_164_3).unwrap());
        // S s_164_5: call AArch32_TakeHypTrapException(s_164_4)
        let s_164_5: () = AArch32_TakeHypTrapException(state, tracer, s_164_4);
        // N s_164_6: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var __ICH_HCR_TALL0:u8
        let s_165_0: bool = fn_state.u__ICH_HCR_TALL0;
        // D s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 1u16);
        // C s_165_2: const #1u : u8
        let s_165_2: bool = true;
        // C s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 1u16);
        // D s_165_4: cmp-eq s_165_1 s_165_3
        let s_165_4: bool = ((s_165_1) == (s_165_3));
        // D s_165_5: write-var gs#112046 <= s_165_4
        fn_state.gs_112046 = s_165_4;
        // N s_165_6: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #432u : u32
        let s_166_0: u32 = 432;
        // D s_166_1: read-reg s_166_0:u8
        let s_166_1: u8 = {
            let value = state.read_register::<u8>(s_166_0 as isize);
            tracer.read_register(s_166_0 as isize, value);
            value
        };
        // D s_166_2: call ELUsingAArch32(s_166_1)
        let s_166_2: bool = ELUsingAArch32(state, tracer, s_166_1);
        // D s_166_3: write-var gs#112045 <= s_166_2
        fn_state.gs_112045 = s_166_2;
        // N s_166_4: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #3u : u8
        let s_167_0: u8 = 3;
        // C s_167_1: cast zx s_167_0 -> bv
        let s_167_1: Bits = Bits::new(s_167_0 as u128, 8u16);
        // C s_167_2: cast zx s_167_1 -> i
        let s_167_2: i128 = (s_167_1.value() as i128);
        // C s_167_3: cast reint s_167_2 -> i64
        let s_167_3: i64 = (s_167_2 as i64);
        // C s_167_4: cast zx s_167_3 -> i
        let s_167_4: i128 = (i128::try_from(s_167_3).unwrap());
        // C s_167_5: const #432u : u32
        let s_167_5: u32 = 432;
        // D s_167_6: read-reg s_167_5:u8
        let s_167_6: u8 = {
            let value = state.read_register::<u8>(s_167_5 as isize);
            tracer.read_register(s_167_5 as isize, value);
            value
        };
        // D s_167_7: call AArch64_AArch32SystemAccessTrap(s_167_6, s_167_4)
        let s_167_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_167_6,
            s_167_4,
        );
        // N s_167_8: return
        return;
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var __ICH_HCR_EL2_TALL0:u8
        let s_168_0: bool = fn_state.u__ICH_HCR_EL2_TALL0;
        // D s_168_1: cast zx s_168_0 -> bv
        let s_168_1: Bits = Bits::new(s_168_0 as u128, 1u16);
        // C s_168_2: const #1u : u8
        let s_168_2: bool = true;
        // C s_168_3: cast zx s_168_2 -> bv
        let s_168_3: Bits = Bits::new(s_168_2 as u128, 1u16);
        // D s_168_4: cmp-eq s_168_1 s_168_3
        let s_168_4: bool = ((s_168_1) == (s_168_3));
        // D s_168_5: write-var gs#112044 <= s_168_4
        fn_state.gs_112044 = s_168_4;
        // N s_168_6: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #432u : u32
        let s_169_0: u32 = 432;
        // D s_169_1: read-reg s_169_0:u8
        let s_169_1: u8 = {
            let value = state.read_register::<u8>(s_169_0 as isize);
            tracer.read_register(s_169_0 as isize, value);
            value
        };
        // D s_169_2: call ELUsingAArch32(s_169_1)
        let s_169_2: bool = ELUsingAArch32(state, tracer, s_169_1);
        // D s_169_3: not s_169_2
        let s_169_3: bool = !s_169_2;
        // D s_169_4: write-var gs#112043 <= s_169_3
        fn_state.gs_112043 = s_169_3;
        // N s_169_5: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_170_0: panic
        panic!("{:?}", ());
        // N s_170_1: return
        return;
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #3u : u8
        let s_171_0: u8 = 3;
        // C s_171_1: cast zx s_171_0 -> bv
        let s_171_1: Bits = Bits::new(s_171_0 as u128, 8u16);
        // C s_171_2: cast zx s_171_1 -> i
        let s_171_2: i128 = (s_171_1.value() as i128);
        // C s_171_3: cast reint s_171_2 -> i64
        let s_171_3: i64 = (s_171_2 as i64);
        // C s_171_4: cast zx s_171_3 -> i
        let s_171_4: i128 = (i128::try_from(s_171_3).unwrap());
        // S s_171_5: call AArch32_TakeHypTrapException(s_171_4)
        let s_171_5: () = AArch32_TakeHypTrapException(state, tracer, s_171_4);
        // N s_171_6: return
        return;
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var __HSTR_T12:u8
        let s_172_0: bool = fn_state.u__HSTR_T12;
        // D s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 1u16);
        // C s_172_2: const #1u : u8
        let s_172_2: bool = true;
        // C s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 1u16);
        // D s_172_4: cmp-eq s_172_1 s_172_3
        let s_172_4: bool = ((s_172_1) == (s_172_3));
        // D s_172_5: write-var gs#112042 <= s_172_4
        fn_state.gs_112042 = s_172_4;
        // N s_172_6: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #432u : u32
        let s_173_0: u32 = 432;
        // D s_173_1: read-reg s_173_0:u8
        let s_173_1: u8 = {
            let value = state.read_register::<u8>(s_173_0 as isize);
            tracer.read_register(s_173_0 as isize, value);
            value
        };
        // D s_173_2: call ELUsingAArch32(s_173_1)
        let s_173_2: bool = ELUsingAArch32(state, tracer, s_173_1);
        // D s_173_3: write-var gs#112041 <= s_173_2
        fn_state.gs_112041 = s_173_2;
        // N s_173_4: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #3u : u8
        let s_174_0: u8 = 3;
        // C s_174_1: cast zx s_174_0 -> bv
        let s_174_1: Bits = Bits::new(s_174_0 as u128, 8u16);
        // C s_174_2: cast zx s_174_1 -> i
        let s_174_2: i128 = (s_174_1.value() as i128);
        // C s_174_3: cast reint s_174_2 -> i64
        let s_174_3: i64 = (s_174_2 as i64);
        // C s_174_4: cast zx s_174_3 -> i
        let s_174_4: i128 = (i128::try_from(s_174_3).unwrap());
        // C s_174_5: const #432u : u32
        let s_174_5: u32 = 432;
        // D s_174_6: read-reg s_174_5:u8
        let s_174_6: u8 = {
            let value = state.read_register::<u8>(s_174_5 as isize);
            tracer.read_register(s_174_5 as isize, value);
            value
        };
        // D s_174_7: call AArch64_AArch32SystemAccessTrap(s_174_6, s_174_4)
        let s_174_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_174_6,
            s_174_4,
        );
        // N s_174_8: return
        return;
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var __HSTR_EL2_T12:u8
        let s_175_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 1u16);
        // C s_175_2: const #1u : u8
        let s_175_2: bool = true;
        // C s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // D s_175_4: cmp-eq s_175_1 s_175_3
        let s_175_4: bool = ((s_175_1) == (s_175_3));
        // D s_175_5: write-var gs#112040 <= s_175_4
        fn_state.gs_112040 = s_175_4;
        // N s_175_6: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #432u : u32
        let s_176_0: u32 = 432;
        // D s_176_1: read-reg s_176_0:u8
        let s_176_1: u8 = {
            let value = state.read_register::<u8>(s_176_0 as isize);
            tracer.read_register(s_176_0 as isize, value);
            value
        };
        // D s_176_2: call ELUsingAArch32(s_176_1)
        let s_176_2: bool = ELUsingAArch32(state, tracer, s_176_1);
        // D s_176_3: not s_176_2
        let s_176_3: bool = !s_176_2;
        // D s_176_4: write-var gs#112039 <= s_176_3
        fn_state.gs_112039 = s_176_3;
        // N s_176_5: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_177_0: panic
        panic!("{:?}", ());
        // N s_177_1: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var __SCR_FIQ:u8
        let s_178_0: bool = fn_state.u__SCR_FIQ;
        // D s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 1u16);
        // C s_178_2: const #1u : u8
        let s_178_2: bool = true;
        // C s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // D s_178_4: cmp-eq s_178_1 s_178_3
        let s_178_4: bool = ((s_178_1) == (s_178_3));
        // D s_178_5: write-var gs#112038 <= s_178_4
        fn_state.gs_112038 = s_178_4;
        // N s_178_6: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_179_0: read-var __PSTATE_M:u8
        let s_179_0: u8 = fn_state.u__PSTATE_M;
        // D s_179_1: cast zx s_179_0 -> bv
        let s_179_1: Bits = Bits::new(s_179_0 as u128, 5u16);
        // C s_179_2: const #384u : u32
        let s_179_2: u32 = 384;
        // D s_179_3: read-reg s_179_2:u8
        let s_179_3: u8 = {
            let value = state.read_register::<u8>(s_179_2 as isize);
            tracer.read_register(s_179_2 as isize, value);
            value
        };
        // D s_179_4: cast zx s_179_3 -> bv
        let s_179_4: Bits = Bits::new(s_179_3 as u128, 5u16);
        // D s_179_5: cmp-ne s_179_1 s_179_4
        let s_179_5: bool = ((s_179_1) != (s_179_4));
        // D s_179_6: write-var gs#112037 <= s_179_5
        fn_state.gs_112037 = s_179_5;
        // N s_179_7: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #424u : u32
        let s_180_0: u32 = 424;
        // D s_180_1: read-reg s_180_0:u8
        let s_180_1: u8 = {
            let value = state.read_register::<u8>(s_180_0 as isize);
            tracer.read_register(s_180_0 as isize, value);
            value
        };
        // D s_180_2: call ELUsingAArch32(s_180_1)
        let s_180_2: bool = ELUsingAArch32(state, tracer, s_180_1);
        // D s_180_3: write-var gs#112036 <= s_180_2
        fn_state.gs_112036 = s_180_2;
        // N s_180_4: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_181_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_181_1: call __IMPDEF_boolean(s_181_0)
        let s_181_1: bool = u__IMPDEF_boolean(state, tracer, s_181_0);
        // D s_181_2: write-var gs#112035 <= s_181_1
        fn_state.gs_112035 = s_181_1;
        // N s_181_3: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #() : ()
        let s_182_0: () = ();
        // S s_182_1: call EDSCR_read(s_182_0)
        let s_182_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_182_0);
        // S s_182_2: call _get_EDSCR_Type_SDD(s_182_1)
        let s_182_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_182_1);
        // S s_182_3: cast zx s_182_2 -> bv
        let s_182_3: Bits = Bits::new(s_182_2 as u128, 1u16);
        // C s_182_4: const #1u : u8
        let s_182_4: bool = true;
        // C s_182_5: cast zx s_182_4 -> bv
        let s_182_5: Bits = Bits::new(s_182_4 as u128, 1u16);
        // S s_182_6: cmp-eq s_182_3 s_182_5
        let s_182_6: bool = ((s_182_3) == (s_182_5));
        // D s_182_7: write-var gs#112034 <= s_182_6
        fn_state.gs_112034 = s_182_6;
        // N s_182_8: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #424u : u32
        let s_183_0: u32 = 424;
        // D s_183_1: read-reg s_183_0:u8
        let s_183_1: u8 = {
            let value = state.read_register::<u8>(s_183_0 as isize);
            tracer.read_register(s_183_0 as isize, value);
            value
        };
        // C s_183_2: const #2u : u8
        let s_183_2: u8 = 2;
        // D s_183_3: cmp-lt s_183_1 s_183_2
        let s_183_3: bool = ((s_183_1) < (s_183_2));
        // D s_183_4: write-var gs#112033 <= s_183_3
        fn_state.gs_112033 = s_183_3;
        // N s_183_5: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_184_0: panic
        panic!("{:?}", ());
        // N s_184_1: return
        return;
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_185_0: read-var __SCR_EL3_FIQ:u8
        let s_185_0: bool = fn_state.u__SCR_EL3_FIQ;
        // D s_185_1: cast zx s_185_0 -> bv
        let s_185_1: Bits = Bits::new(s_185_0 as u128, 1u16);
        // C s_185_2: const #1u : u8
        let s_185_2: bool = true;
        // C s_185_3: cast zx s_185_2 -> bv
        let s_185_3: Bits = Bits::new(s_185_2 as u128, 1u16);
        // D s_185_4: cmp-eq s_185_1 s_185_3
        let s_185_4: bool = ((s_185_1) == (s_185_3));
        // D s_185_5: write-var gs#112032 <= s_185_4
        fn_state.gs_112032 = s_185_4;
        // N s_185_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #424u : u32
        let s_186_0: u32 = 424;
        // D s_186_1: read-reg s_186_0:u8
        let s_186_1: u8 = {
            let value = state.read_register::<u8>(s_186_0 as isize);
            tracer.read_register(s_186_0 as isize, value);
            value
        };
        // D s_186_2: call ELUsingAArch32(s_186_1)
        let s_186_2: bool = ELUsingAArch32(state, tracer, s_186_1);
        // D s_186_3: not s_186_2
        let s_186_3: bool = !s_186_2;
        // D s_186_4: write-var gs#112031 <= s_186_3
        fn_state.gs_112031 = s_186_3;
        // N s_186_5: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_187_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_187_1: call __IMPDEF_boolean(s_187_0)
        let s_187_1: bool = u__IMPDEF_boolean(state, tracer, s_187_0);
        // D s_187_2: write-var gs#112030 <= s_187_1
        fn_state.gs_112030 = s_187_1;
        // N s_187_3: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #() : ()
        let s_188_0: () = ();
        // S s_188_1: call EDSCR_read(s_188_0)
        let s_188_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_188_0);
        // S s_188_2: call _get_EDSCR_Type_SDD(s_188_1)
        let s_188_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_188_1);
        // S s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 1u16);
        // C s_188_4: const #1u : u8
        let s_188_4: bool = true;
        // C s_188_5: cast zx s_188_4 -> bv
        let s_188_5: Bits = Bits::new(s_188_4 as u128, 1u16);
        // S s_188_6: cmp-eq s_188_3 s_188_5
        let s_188_6: bool = ((s_188_3) == (s_188_5));
        // D s_188_7: write-var gs#112029 <= s_188_6
        fn_state.gs_112029 = s_188_6;
        // N s_188_8: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #424u : u32
        let s_189_0: u32 = 424;
        // D s_189_1: read-reg s_189_0:u8
        let s_189_1: u8 = {
            let value = state.read_register::<u8>(s_189_0 as isize);
            tracer.read_register(s_189_0 as isize, value);
            value
        };
        // C s_189_2: const #2u : u8
        let s_189_2: u8 = 2;
        // D s_189_3: cmp-lt s_189_1 s_189_2
        let s_189_3: bool = ((s_189_1) < (s_189_2));
        // D s_189_4: write-var gs#112028 <= s_189_3
        fn_state.gs_112028 = s_189_3;
        // N s_189_5: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_190_0: panic
        panic!("{:?}", ());
        // N s_190_1: return
        return;
    }
}
