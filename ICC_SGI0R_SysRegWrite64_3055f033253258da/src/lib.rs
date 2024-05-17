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
use u_get_SCR_Type_IRQ::*;
use u_get_ICC_SRE_Type_SRE::*;
use Halted::*;
use ICC_SGI0R_write::*;
use ICC_HSRE_read::*;
use ICC_SRE_read::*;
use Mk_ICC_SGI0R_Type::*;
use R_read::*;
use u_get_HCR_Type_FMO::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_Type_IMO::*;
use EL2Enabled::*;
use u_get_HSTR_Type_T12::*;
use u_get_ICC_MSRE_Type_SRE::*;
use u_get_SCR_Type_FIQ::*;
use HCR_read::*;
use u_get_ICH_HCR_Type_TC::*;
use u_get_ICC_HSRE_Type_SRE::*;
use u_get_SCR_EL3_Type_IRQ::*;
use u__IMPDEF_boolean::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HCR_EL2_Type_FMO::*;
use u_get_SCR_EL3_Type_FIQ::*;
use ELUsingAArch32::*;
use u_get_ICH_HCR_EL2_Type_TC::*;
use u_get_HCR_EL2_Type_IMO::*;
use ICH_HCR_read::*;
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use common::*;
pub fn ICC_SGI0R_SysRegWrite64_3055f033253258da<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_104887: bool,
        u__HCR_EL2_IMO: bool,
        gs_104889: bool,
        gs_104878: bool,
        gs_104899: bool,
        gs_104914: bool,
        gs_104875: bool,
        gs_104877: bool,
        gs_104886: bool,
        gs_104896: bool,
        gs_104903: bool,
        gs_104920: bool,
        gs_104911: bool,
        gs_104880: bool,
        gs_104894: bool,
        gs_104921: bool,
        gs_104876: bool,
        u__HSTR_EL2_T12: bool,
        gs_104885: bool,
        gs_104891: bool,
        u__HCR_EL2_FMO: bool,
        gs_104874: bool,
        gs_104881: bool,
        gs_104906: bool,
        gs_104913: bool,
        gs_104873: bool,
        u__PSTATE_EL: u8,
        gs_104908: bool,
        gs_104912: bool,
        gs_104909: bool,
        gs_104901: bool,
        gs_104919: bool,
        gs_104915: bool,
        u__ICH_HCR_EL2_TC: bool,
        gs_104890: bool,
        gs_104882: bool,
        u__ICC_HSRE_SRE: bool,
        u__HSTR_T12: bool,
        u__ICH_HCR_TC: bool,
        gs_104893: bool,
        u__HCR_IMO: bool,
        gs_104902: bool,
        gs_104910: bool,
        gs_104883: bool,
        gs_104904: bool,
        u__ICC_MSRE_SRE: bool,
        gs_104905: bool,
        gs_104872: bool,
        gs_104907: bool,
        gs_104892: bool,
        u__HCR_FMO: bool,
        gs_104897: bool,
        u__ICC_SRE_SRE: bool,
        u__PSTATE_M: u8,
        gs_104900: bool,
        gs_104879: bool,
        gs_104884: bool,
        gs_104917: bool,
        gs_104916: bool,
        gs_104898: bool,
        gs_104918: bool,
        gs_104888: bool,
        gs_104895: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRm,
        t,
        t2,
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
        // C s_0_3: const #16983u : u32
        let s_0_3: u32 = 16983;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: write-var __PSTATE_M <= s_0_4
        fn_state.u__PSTATE_M = s_0_4;
        // C s_0_6: const #104936u : u32
        let s_0_6: u32 = 104936;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // D s_0_8: call _get_HSTR_EL2_Type_T12(s_0_7)
        let s_0_8: bool = u_get_HSTR_EL2_Type_T12(state, tracer, s_0_7);
        // D s_0_9: write-var __HSTR_EL2_T12 <= s_0_8
        fn_state.u__HSTR_EL2_T12 = s_0_8;
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call HSTR_read(s_0_10)
        let s_0_11: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_10);
        // S s_0_12: call _get_HSTR_Type_T12(s_0_11)
        let s_0_12: bool = u_get_HSTR_Type_T12(state, tracer, s_0_11);
        // D s_0_13: write-var __HSTR_T12 <= s_0_12
        fn_state.u__HSTR_T12 = s_0_12;
        // C s_0_14: const #() : ()
        let s_0_14: () = ();
        // S s_0_15: call ICC_SRE_read(s_0_14)
        let s_0_15: ProductType700c18a878c5601b = ICC_SRE_read(state, tracer, s_0_14);
        // S s_0_16: call _get_ICC_SRE_Type_SRE(s_0_15)
        let s_0_16: bool = u_get_ICC_SRE_Type_SRE(state, tracer, s_0_15);
        // D s_0_17: write-var __ICC_SRE_SRE <= s_0_16
        fn_state.u__ICC_SRE_SRE = s_0_16;
        // C s_0_18: const #20992u : u32
        let s_0_18: u32 = 20992;
        // D s_0_19: read-reg s_0_18:struct
        let s_0_19: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_18 as isize);
            tracer.read_register(s_0_18 as isize, value);
            value
        };
        // D s_0_20: call _get_ICH_HCR_EL2_Type_TC(s_0_19)
        let s_0_20: bool = u_get_ICH_HCR_EL2_Type_TC(state, tracer, s_0_19);
        // D s_0_21: write-var __ICH_HCR_EL2_TC <= s_0_20
        fn_state.u__ICH_HCR_EL2_TC = s_0_20;
        // C s_0_22: const #() : ()
        let s_0_22: () = ();
        // S s_0_23: call ICH_HCR_read(s_0_22)
        let s_0_23: ProductType700c18a878c5601b = ICH_HCR_read(state, tracer, s_0_22);
        // S s_0_24: call _get_ICH_HCR_Type_TC(s_0_23)
        let s_0_24: bool = u_get_ICH_HCR_Type_TC(state, tracer, s_0_23);
        // D s_0_25: write-var __ICH_HCR_TC <= s_0_24
        fn_state.u__ICH_HCR_TC = s_0_24;
        // C s_0_26: const #102552u : u32
        let s_0_26: u32 = 102552;
        // D s_0_27: read-reg s_0_26:struct
        let s_0_27: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_26 as isize);
            tracer.read_register(s_0_26 as isize, value);
            value
        };
        // D s_0_28: call _get_HCR_EL2_Type_FMO(s_0_27)
        let s_0_28: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_0_27);
        // D s_0_29: write-var __HCR_EL2_FMO <= s_0_28
        fn_state.u__HCR_EL2_FMO = s_0_28;
        // C s_0_30: const #102552u : u32
        let s_0_30: u32 = 102552;
        // D s_0_31: read-reg s_0_30:struct
        let s_0_31: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_30 as isize);
            tracer.read_register(s_0_30 as isize, value);
            value
        };
        // D s_0_32: call _get_HCR_EL2_Type_IMO(s_0_31)
        let s_0_32: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_0_31);
        // D s_0_33: write-var __HCR_EL2_IMO <= s_0_32
        fn_state.u__HCR_EL2_IMO = s_0_32;
        // C s_0_34: const #() : ()
        let s_0_34: () = ();
        // S s_0_35: call HCR_read(s_0_34)
        let s_0_35: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_34);
        // S s_0_36: call _get_HCR_Type_FMO(s_0_35)
        let s_0_36: bool = u_get_HCR_Type_FMO(state, tracer, s_0_35);
        // D s_0_37: write-var __HCR_FMO <= s_0_36
        fn_state.u__HCR_FMO = s_0_36;
        // C s_0_38: const #() : ()
        let s_0_38: () = ();
        // S s_0_39: call HCR_read(s_0_38)
        let s_0_39: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_38);
        // S s_0_40: call _get_HCR_Type_IMO(s_0_39)
        let s_0_40: bool = u_get_HCR_Type_IMO(state, tracer, s_0_39);
        // D s_0_41: write-var __HCR_IMO <= s_0_40
        fn_state.u__HCR_IMO = s_0_40;
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
        // D s_0_50: read-var __PSTATE_EL:u8
        let s_0_50: u8 = fn_state.u__PSTATE_EL;
        // D s_0_51: cast zx s_0_50 -> bv
        let s_0_51: Bits = Bits::new(s_0_50 as u128, 2u16);
        // C s_0_52: const #448u : u32
        let s_0_52: u32 = 448;
        // D s_0_53: read-reg s_0_52:u8
        let s_0_53: u8 = {
            let value = state.read_register::<u8>(s_0_52 as isize);
            tracer.read_register(s_0_52 as isize, value);
            value
        };
        // D s_0_54: cast zx s_0_53 -> bv
        let s_0_54: Bits = Bits::new(s_0_53 as u128, 2u16);
        // D s_0_55: cmp-eq s_0_51 s_0_54
        let s_0_55: bool = ((s_0_51) == (s_0_54));
        // N s_0_56: branch s_0_55 b204 b1
        if s_0_55 {
            return block_204(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b71 b2
        if s_1_5 {
            return block_71(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b8 b3
        if s_2_5 {
            return block_8(state, tracer, fn_state);
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
        // D s_5_0: read-var __ICC_MSRE_SRE:u8
        let s_5_0: bool = fn_state.u__ICC_MSRE_SRE;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
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
        // D s_6_0: read-var t2:i
        let s_6_0: i128 = fn_state.t2;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // D s_6_2: read-var t:i
        let s_6_2: i128 = fn_state.t;
        // D s_6_3: call R_read(s_6_2)
        let s_6_3: u32 = R_read(state, tracer, s_6_2);
        // D s_6_4: cast zx s_6_1 -> bv
        let s_6_4: Bits = Bits::new(s_6_1 as u128, 32u16);
        // D s_6_5: cast zx s_6_3 -> bv
        let s_6_5: Bits = Bits::new(s_6_3 as u128, 32u16);
        // D s_6_6: cast reint s_6_4 -> u128
        let s_6_6: u128 = (s_6_4.value() as u128);
        // D s_6_7: size-of s_6_4
        let s_6_7: u16 = s_6_4.length();
        // D s_6_8: cast reint s_6_5 -> u128
        let s_6_8: u128 = (s_6_5.value() as u128);
        // D s_6_9: size-of s_6_5
        let s_6_9: u16 = s_6_5.length();
        // D s_6_10: lsl s_6_6 s_6_9
        let s_6_10: u128 = s_6_6 << s_6_9;
        // D s_6_11: or s_6_10 s_6_8
        let s_6_11: u128 = ((s_6_10) | (s_6_8));
        // D s_6_12: add s_6_7 s_6_9
        let s_6_12: u16 = (s_6_7 + s_6_9);
        // D s_6_13: create-bits s_6_11 s_6_12
        let s_6_13: Bits = Bits::new(s_6_11, s_6_12);
        // D s_6_14: cast reint s_6_13 -> u64
        let s_6_14: u64 = (s_6_13.value() as u64);
        // D s_6_15: call Mk_ICC_SGI0R_Type(s_6_14)
        let s_6_15: ProductType5c790c8ef59cc8b2 = Mk_ICC_SGI0R_Type(
            state,
            tracer,
            s_6_14,
        );
        // D s_6_16: call ICC_SGI0R_write(s_6_15)
        let s_6_16: () = ICC_SGI0R_write(state, tracer, s_6_15);
        // N s_6_17: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call Halted(s_8_0)
        let s_8_1: bool = Halted(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b70 b9
        if s_8_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#104872 <= s_9_0
        fn_state.gs_104872 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#104872:u8
        let s_10_0: bool = fn_state.gs_104872;
        // N s_10_1: branch s_10_0 b69 b11
        if s_10_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#104873 <= s_11_0
        fn_state.gs_104873 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#104873:u8
        let s_12_0: bool = fn_state.gs_104873;
        // N s_12_1: branch s_12_0 b68 b13
        if s_12_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#104874 <= s_13_0
        fn_state.gs_104874 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#104874:u8
        let s_14_0: bool = fn_state.gs_104874;
        // N s_14_1: branch s_14_0 b67 b15
        if s_14_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#104875 <= s_15_0
        fn_state.gs_104875 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#104875:u8
        let s_16_0: bool = fn_state.gs_104875;
        // N s_16_1: branch s_16_0 b66 b17
        if s_16_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#104876 <= s_17_0
        fn_state.gs_104876 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#104876:u8
        let s_18_0: bool = fn_state.gs_104876;
        // N s_18_1: branch s_18_0 b65 b19
        if s_18_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
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
        // N s_19_2: branch s_19_1 b64 b20
        if s_19_1 {
            return block_64(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#104877 <= s_20_0
        fn_state.gs_104877 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#104877:u8
        let s_21_0: bool = fn_state.gs_104877;
        // N s_21_1: branch s_21_0 b63 b22
        if s_21_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#104878 <= s_22_0
        fn_state.gs_104878 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#104878:u8
        let s_23_0: bool = fn_state.gs_104878;
        // N s_23_1: branch s_23_0 b62 b24
        if s_23_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#104879 <= s_24_0
        fn_state.gs_104879 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#104879:u8
        let s_25_0: bool = fn_state.gs_104879;
        // N s_25_1: branch s_25_0 b61 b26
        if s_25_0 {
            return block_61(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#104880 <= s_26_0
        fn_state.gs_104880 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#104880:u8
        let s_27_0: bool = fn_state.gs_104880;
        // N s_27_1: branch s_27_0 b60 b28
        if s_27_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#104881 <= s_28_0
        fn_state.gs_104881 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#104881:u8
        let s_29_0: bool = fn_state.gs_104881;
        // N s_29_1: branch s_29_0 b59 b30
        if s_29_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __ICC_HSRE_SRE:u8
        let s_30_0: bool = fn_state.u__ICC_HSRE_SRE;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #0u : u8
        let s_30_2: bool = false;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // N s_30_5: branch s_30_4 b58 b31
        if s_30_4 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #424u : u32
        let s_31_0: u32 = 424;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // C s_31_2: const #2u : u8
        let s_31_2: u8 = 2;
        // D s_31_3: cmp-lt s_31_1 s_31_2
        let s_31_3: bool = ((s_31_1) < (s_31_2));
        // N s_31_4: branch s_31_3 b57 b32
        if s_31_3 {
            return block_57(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#104882 <= s_32_0
        fn_state.gs_104882 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#104882:u8
        let s_33_0: bool = fn_state.gs_104882;
        // N s_33_1: branch s_33_0 b56 b34
        if s_33_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#104883 <= s_34_0
        fn_state.gs_104883 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#104883:u8
        let s_35_0: bool = fn_state.gs_104883;
        // N s_35_1: branch s_35_0 b50 b36
        if s_35_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #424u : u32
        let s_36_0: u32 = 424;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // C s_36_2: const #2u : u8
        let s_36_2: u8 = 2;
        // D s_36_3: cmp-lt s_36_1 s_36_2
        let s_36_3: bool = ((s_36_1) < (s_36_2));
        // N s_36_4: branch s_36_3 b49 b37
        if s_36_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#104884 <= s_37_0
        fn_state.gs_104884 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#104884:u8
        let s_38_0: bool = fn_state.gs_104884;
        // N s_38_1: branch s_38_0 b48 b39
        if s_38_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#104885 <= s_39_0
        fn_state.gs_104885 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#104885:u8
        let s_40_0: bool = fn_state.gs_104885;
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
        // D s_41_0: read-var t2:i
        let s_41_0: i128 = fn_state.t2;
        // D s_41_1: call R_read(s_41_0)
        let s_41_1: u32 = R_read(state, tracer, s_41_0);
        // D s_41_2: read-var t:i
        let s_41_2: i128 = fn_state.t;
        // D s_41_3: call R_read(s_41_2)
        let s_41_3: u32 = R_read(state, tracer, s_41_2);
        // D s_41_4: cast zx s_41_1 -> bv
        let s_41_4: Bits = Bits::new(s_41_1 as u128, 32u16);
        // D s_41_5: cast zx s_41_3 -> bv
        let s_41_5: Bits = Bits::new(s_41_3 as u128, 32u16);
        // D s_41_6: cast reint s_41_4 -> u128
        let s_41_6: u128 = (s_41_4.value() as u128);
        // D s_41_7: size-of s_41_4
        let s_41_7: u16 = s_41_4.length();
        // D s_41_8: cast reint s_41_5 -> u128
        let s_41_8: u128 = (s_41_5.value() as u128);
        // D s_41_9: size-of s_41_5
        let s_41_9: u16 = s_41_5.length();
        // D s_41_10: lsl s_41_6 s_41_9
        let s_41_10: u128 = s_41_6 << s_41_9;
        // D s_41_11: or s_41_10 s_41_8
        let s_41_11: u128 = ((s_41_10) | (s_41_8));
        // D s_41_12: add s_41_7 s_41_9
        let s_41_12: u16 = (s_41_7 + s_41_9);
        // D s_41_13: create-bits s_41_11 s_41_12
        let s_41_13: Bits = Bits::new(s_41_11, s_41_12);
        // D s_41_14: cast reint s_41_13 -> u64
        let s_41_14: u64 = (s_41_13.value() as u64);
        // D s_41_15: call Mk_ICC_SGI0R_Type(s_41_14)
        let s_41_15: ProductType5c790c8ef59cc8b2 = Mk_ICC_SGI0R_Type(
            state,
            tracer,
            s_41_14,
        );
        // D s_41_16: call ICC_SGI0R_write(s_41_15)
        let s_41_16: () = ICC_SGI0R_write(state, tracer, s_41_15);
        // N s_41_17: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call Halted(s_42_0)
        let s_42_1: bool = Halted(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b47 b43
        if s_42_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#104886 <= s_43_0
        fn_state.gs_104886 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#104886:u8
        let s_44_0: bool = fn_state.gs_104886;
        // N s_44_1: branch s_44_0 b46 b45
        if s_44_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call AArch32_TakeMonitorTrapException(s_45_0)
        let s_45_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_45_0);
        // N s_45_2: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: panic
        panic!("{:?}", ());
        // N s_46_1: return
        return;
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
        // D s_47_7: write-var gs#104886 <= s_47_6
        fn_state.gs_104886 = s_47_6;
        // N s_47_8: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #20920u : u32
        let s_48_0: u32 = 20920;
        // D s_48_1: read-reg s_48_0:struct
        let s_48_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call _get_SCR_Type_IRQ(s_48_1)
        let s_48_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_48_1);
        // C s_48_3: const #20920u : u32
        let s_48_3: u32 = 20920;
        // D s_48_4: read-reg s_48_3:struct
        let s_48_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_48_3 as isize);
            tracer.read_register(s_48_3 as isize, value);
            value
        };
        // D s_48_5: call _get_SCR_Type_FIQ(s_48_4)
        let s_48_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_48_4);
        // D s_48_6: cast zx s_48_2 -> bv
        let s_48_6: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_7: cast zx s_48_5 -> bv
        let s_48_7: Bits = Bits::new(s_48_5 as u128, 1u16);
        // D s_48_8: cast reint s_48_6 -> u128
        let s_48_8: u128 = (s_48_6.value() as u128);
        // D s_48_9: size-of s_48_6
        let s_48_9: u16 = s_48_6.length();
        // D s_48_10: cast reint s_48_7 -> u128
        let s_48_10: u128 = (s_48_7.value() as u128);
        // D s_48_11: size-of s_48_7
        let s_48_11: u16 = s_48_7.length();
        // D s_48_12: lsl s_48_8 s_48_11
        let s_48_12: u128 = s_48_8 << s_48_11;
        // D s_48_13: or s_48_12 s_48_10
        let s_48_13: u128 = ((s_48_12) | (s_48_10));
        // D s_48_14: add s_48_9 s_48_11
        let s_48_14: u16 = (s_48_9 + s_48_11);
        // D s_48_15: create-bits s_48_13 s_48_14
        let s_48_15: Bits = Bits::new(s_48_13, s_48_14);
        // D s_48_16: cast reint s_48_15 -> u8
        let s_48_16: u8 = (s_48_15.value() as u8);
        // D s_48_17: cast zx s_48_16 -> bv
        let s_48_17: Bits = Bits::new(s_48_16 as u128, 2u16);
        // C s_48_18: const #3u : u8
        let s_48_18: u8 = 3;
        // C s_48_19: cast zx s_48_18 -> bv
        let s_48_19: Bits = Bits::new(s_48_18 as u128, 2u16);
        // D s_48_20: cmp-eq s_48_17 s_48_19
        let s_48_20: bool = ((s_48_17) == (s_48_19));
        // D s_48_21: write-var gs#104885 <= s_48_20
        fn_state.gs_104885 = s_48_20;
        // N s_48_22: jump b40
        return block_40(state, tracer, fn_state);
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
        // D s_49_2: call ELUsingAArch32(s_49_1)
        let s_49_2: bool = ELUsingAArch32(state, tracer, s_49_1);
        // D s_49_3: write-var gs#104884 <= s_49_2
        fn_state.gs_104884 = s_49_2;
        // N s_49_4: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call Halted(s_50_0)
        let s_50_1: bool = Halted(state, tracer, s_50_0);
        // N s_50_2: branch s_50_1 b55 b51
        if s_50_1 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#104887 <= s_51_0
        fn_state.gs_104887 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#104887:u8
        let s_52_0: bool = fn_state.gs_104887;
        // N s_52_1: branch s_52_0 b54 b53
        if s_52_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #3u : u8
        let s_53_0: u8 = 3;
        // C s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 8u16);
        // C s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (s_53_1.value() as i128);
        // C s_53_3: cast reint s_53_2 -> i64
        let s_53_3: i64 = (s_53_2 as i64);
        // C s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // C s_53_5: const #424u : u32
        let s_53_5: u32 = 424;
        // D s_53_6: read-reg s_53_5:u8
        let s_53_6: u8 = {
            let value = state.read_register::<u8>(s_53_5 as isize);
            tracer.read_register(s_53_5 as isize, value);
            value
        };
        // D s_53_7: call AArch64_AArch32SystemAccessTrap(s_53_6, s_53_4)
        let s_53_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_53_6, s_53_4);
        // N s_53_8: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: panic
        panic!("{:?}", ());
        // N s_54_1: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call EDSCR_read(s_55_0)
        let s_55_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_55_0);
        // S s_55_2: call _get_EDSCR_Type_SDD(s_55_1)
        let s_55_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_55_1);
        // S s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // C s_55_4: const #1u : u8
        let s_55_4: bool = true;
        // C s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 1u16);
        // S s_55_6: cmp-eq s_55_3 s_55_5
        let s_55_6: bool = ((s_55_3) == (s_55_5));
        // D s_55_7: write-var gs#104887 <= s_55_6
        fn_state.gs_104887 = s_55_6;
        // N s_55_8: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #90704u : u32
        let s_56_0: u32 = 90704;
        // D s_56_1: read-reg s_56_0:struct
        let s_56_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: call _get_SCR_EL3_Type_IRQ(s_56_1)
        let s_56_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_56_1);
        // C s_56_3: const #90704u : u32
        let s_56_3: u32 = 90704;
        // D s_56_4: read-reg s_56_3:struct
        let s_56_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_3 as isize);
            tracer.read_register(s_56_3 as isize, value);
            value
        };
        // D s_56_5: call _get_SCR_EL3_Type_FIQ(s_56_4)
        let s_56_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_56_4);
        // D s_56_6: cast zx s_56_2 -> bv
        let s_56_6: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_7: cast zx s_56_5 -> bv
        let s_56_7: Bits = Bits::new(s_56_5 as u128, 1u16);
        // D s_56_8: cast reint s_56_6 -> u128
        let s_56_8: u128 = (s_56_6.value() as u128);
        // D s_56_9: size-of s_56_6
        let s_56_9: u16 = s_56_6.length();
        // D s_56_10: cast reint s_56_7 -> u128
        let s_56_10: u128 = (s_56_7.value() as u128);
        // D s_56_11: size-of s_56_7
        let s_56_11: u16 = s_56_7.length();
        // D s_56_12: lsl s_56_8 s_56_11
        let s_56_12: u128 = s_56_8 << s_56_11;
        // D s_56_13: or s_56_12 s_56_10
        let s_56_13: u128 = ((s_56_12) | (s_56_10));
        // D s_56_14: add s_56_9 s_56_11
        let s_56_14: u16 = (s_56_9 + s_56_11);
        // D s_56_15: create-bits s_56_13 s_56_14
        let s_56_15: Bits = Bits::new(s_56_13, s_56_14);
        // D s_56_16: cast reint s_56_15 -> u8
        let s_56_16: u8 = (s_56_15.value() as u8);
        // D s_56_17: cast zx s_56_16 -> bv
        let s_56_17: Bits = Bits::new(s_56_16 as u128, 2u16);
        // C s_56_18: const #3u : u8
        let s_56_18: u8 = 3;
        // C s_56_19: cast zx s_56_18 -> bv
        let s_56_19: Bits = Bits::new(s_56_18 as u128, 2u16);
        // D s_56_20: cmp-eq s_56_17 s_56_19
        let s_56_20: bool = ((s_56_17) == (s_56_19));
        // D s_56_21: write-var gs#104883 <= s_56_20
        fn_state.gs_104883 = s_56_20;
        // N s_56_22: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #424u : u32
        let s_57_0: u32 = 424;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call ELUsingAArch32(s_57_1)
        let s_57_2: bool = ELUsingAArch32(state, tracer, s_57_1);
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // D s_57_4: write-var gs#104882 <= s_57_3
        fn_state.gs_104882 = s_57_3;
        // N s_57_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: panic
        panic!("{:?}", ());
        // N s_58_1: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: panic
        panic!("{:?}", ());
        // N s_59_1: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #20920u : u32
        let s_60_0: u32 = 20920;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_SCR_Type_IRQ(s_60_1)
        let s_60_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_60_1);
        // C s_60_3: const #20920u : u32
        let s_60_3: u32 = 20920;
        // D s_60_4: read-reg s_60_3:struct
        let s_60_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_60_3 as isize);
            tracer.read_register(s_60_3 as isize, value);
            value
        };
        // D s_60_5: call _get_SCR_Type_FIQ(s_60_4)
        let s_60_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_60_4);
        // D s_60_6: cast zx s_60_2 -> bv
        let s_60_6: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_7: cast zx s_60_5 -> bv
        let s_60_7: Bits = Bits::new(s_60_5 as u128, 1u16);
        // D s_60_8: cast reint s_60_6 -> u128
        let s_60_8: u128 = (s_60_6.value() as u128);
        // D s_60_9: size-of s_60_6
        let s_60_9: u16 = s_60_6.length();
        // D s_60_10: cast reint s_60_7 -> u128
        let s_60_10: u128 = (s_60_7.value() as u128);
        // D s_60_11: size-of s_60_7
        let s_60_11: u16 = s_60_7.length();
        // D s_60_12: lsl s_60_8 s_60_11
        let s_60_12: u128 = s_60_8 << s_60_11;
        // D s_60_13: or s_60_12 s_60_10
        let s_60_13: u128 = ((s_60_12) | (s_60_10));
        // D s_60_14: add s_60_9 s_60_11
        let s_60_14: u16 = (s_60_9 + s_60_11);
        // D s_60_15: create-bits s_60_13 s_60_14
        let s_60_15: Bits = Bits::new(s_60_13, s_60_14);
        // D s_60_16: cast reint s_60_15 -> u8
        let s_60_16: u8 = (s_60_15.value() as u8);
        // D s_60_17: cast zx s_60_16 -> bv
        let s_60_17: Bits = Bits::new(s_60_16 as u128, 2u16);
        // C s_60_18: const #3u : u8
        let s_60_18: u8 = 3;
        // C s_60_19: cast zx s_60_18 -> bv
        let s_60_19: Bits = Bits::new(s_60_18 as u128, 2u16);
        // D s_60_20: cmp-eq s_60_17 s_60_19
        let s_60_20: bool = ((s_60_17) == (s_60_19));
        // D s_60_21: write-var gs#104881 <= s_60_20
        fn_state.gs_104881 = s_60_20;
        // N s_60_22: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #424u : u32
        let s_61_0: u32 = 424;
        // D s_61_1: read-reg s_61_0:u8
        let s_61_1: u8 = {
            let value = state.read_register::<u8>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call ELUsingAArch32(s_61_1)
        let s_61_2: bool = ELUsingAArch32(state, tracer, s_61_1);
        // D s_61_3: write-var gs#104880 <= s_61_2
        fn_state.gs_104880 = s_61_2;
        // N s_61_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_62_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_62_1: call __IMPDEF_boolean(s_62_0)
        let s_62_1: bool = u__IMPDEF_boolean(state, tracer, s_62_0);
        // D s_62_2: write-var gs#104879 <= s_62_1
        fn_state.gs_104879 = s_62_1;
        // N s_62_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call EDSCR_read(s_63_0)
        let s_63_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_63_0);
        // S s_63_2: call _get_EDSCR_Type_SDD(s_63_1)
        let s_63_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_63_1);
        // S s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // C s_63_4: const #1u : u8
        let s_63_4: bool = true;
        // C s_63_5: cast zx s_63_4 -> bv
        let s_63_5: Bits = Bits::new(s_63_4 as u128, 1u16);
        // S s_63_6: cmp-eq s_63_3 s_63_5
        let s_63_6: bool = ((s_63_3) == (s_63_5));
        // D s_63_7: write-var gs#104878 <= s_63_6
        fn_state.gs_104878 = s_63_6;
        // N s_63_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #424u : u32
        let s_64_0: u32 = 424;
        // D s_64_1: read-reg s_64_0:u8
        let s_64_1: u8 = {
            let value = state.read_register::<u8>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // C s_64_2: const #2u : u8
        let s_64_2: u8 = 2;
        // D s_64_3: cmp-lt s_64_1 s_64_2
        let s_64_3: bool = ((s_64_1) < (s_64_2));
        // D s_64_4: write-var gs#104877 <= s_64_3
        fn_state.gs_104877 = s_64_3;
        // N s_64_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: panic
        panic!("{:?}", ());
        // N s_65_1: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #90704u : u32
        let s_66_0: u32 = 90704;
        // D s_66_1: read-reg s_66_0:struct
        let s_66_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // D s_66_2: call _get_SCR_EL3_Type_IRQ(s_66_1)
        let s_66_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_66_1);
        // C s_66_3: const #90704u : u32
        let s_66_3: u32 = 90704;
        // D s_66_4: read-reg s_66_3:struct
        let s_66_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_3 as isize);
            tracer.read_register(s_66_3 as isize, value);
            value
        };
        // D s_66_5: call _get_SCR_EL3_Type_FIQ(s_66_4)
        let s_66_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_66_4);
        // D s_66_6: cast zx s_66_2 -> bv
        let s_66_6: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_7: cast zx s_66_5 -> bv
        let s_66_7: Bits = Bits::new(s_66_5 as u128, 1u16);
        // D s_66_8: cast reint s_66_6 -> u128
        let s_66_8: u128 = (s_66_6.value() as u128);
        // D s_66_9: size-of s_66_6
        let s_66_9: u16 = s_66_6.length();
        // D s_66_10: cast reint s_66_7 -> u128
        let s_66_10: u128 = (s_66_7.value() as u128);
        // D s_66_11: size-of s_66_7
        let s_66_11: u16 = s_66_7.length();
        // D s_66_12: lsl s_66_8 s_66_11
        let s_66_12: u128 = s_66_8 << s_66_11;
        // D s_66_13: or s_66_12 s_66_10
        let s_66_13: u128 = ((s_66_12) | (s_66_10));
        // D s_66_14: add s_66_9 s_66_11
        let s_66_14: u16 = (s_66_9 + s_66_11);
        // D s_66_15: create-bits s_66_13 s_66_14
        let s_66_15: Bits = Bits::new(s_66_13, s_66_14);
        // D s_66_16: cast reint s_66_15 -> u8
        let s_66_16: u8 = (s_66_15.value() as u8);
        // D s_66_17: cast zx s_66_16 -> bv
        let s_66_17: Bits = Bits::new(s_66_16 as u128, 2u16);
        // C s_66_18: const #3u : u8
        let s_66_18: u8 = 3;
        // C s_66_19: cast zx s_66_18 -> bv
        let s_66_19: Bits = Bits::new(s_66_18 as u128, 2u16);
        // D s_66_20: cmp-eq s_66_17 s_66_19
        let s_66_20: bool = ((s_66_17) == (s_66_19));
        // D s_66_21: write-var gs#104876 <= s_66_20
        fn_state.gs_104876 = s_66_20;
        // N s_66_22: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #424u : u32
        let s_67_0: u32 = 424;
        // D s_67_1: read-reg s_67_0:u8
        let s_67_1: u8 = {
            let value = state.read_register::<u8>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // D s_67_2: call ELUsingAArch32(s_67_1)
        let s_67_2: bool = ELUsingAArch32(state, tracer, s_67_1);
        // D s_67_3: not s_67_2
        let s_67_3: bool = !s_67_2;
        // D s_67_4: write-var gs#104875 <= s_67_3
        fn_state.gs_104875 = s_67_3;
        // N s_67_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_68_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_68_1: call __IMPDEF_boolean(s_68_0)
        let s_68_1: bool = u__IMPDEF_boolean(state, tracer, s_68_0);
        // D s_68_2: write-var gs#104874 <= s_68_1
        fn_state.gs_104874 = s_68_1;
        // N s_68_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call EDSCR_read(s_69_0)
        let s_69_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_69_0);
        // S s_69_2: call _get_EDSCR_Type_SDD(s_69_1)
        let s_69_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_69_1);
        // S s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // C s_69_4: const #1u : u8
        let s_69_4: bool = true;
        // C s_69_5: cast zx s_69_4 -> bv
        let s_69_5: Bits = Bits::new(s_69_4 as u128, 1u16);
        // S s_69_6: cmp-eq s_69_3 s_69_5
        let s_69_6: bool = ((s_69_3) == (s_69_5));
        // D s_69_7: write-var gs#104873 <= s_69_6
        fn_state.gs_104873 = s_69_6;
        // N s_69_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #424u : u32
        let s_70_0: u32 = 424;
        // D s_70_1: read-reg s_70_0:u8
        let s_70_1: u8 = {
            let value = state.read_register::<u8>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // C s_70_2: const #2u : u8
        let s_70_2: u8 = 2;
        // D s_70_3: cmp-lt s_70_1 s_70_2
        let s_70_3: bool = ((s_70_1) < (s_70_2));
        // D s_70_4: write-var gs#104872 <= s_70_3
        fn_state.gs_104872 = s_70_3;
        // N s_70_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call Halted(s_71_0)
        let s_71_1: bool = Halted(state, tracer, s_71_0);
        // N s_71_2: branch s_71_1 b203 b72
        if s_71_1 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#104888 <= s_72_0
        fn_state.gs_104888 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#104888:u8
        let s_73_0: bool = fn_state.gs_104888;
        // N s_73_1: branch s_73_0 b202 b74
        if s_73_0 {
            return block_202(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#104889 <= s_74_0
        fn_state.gs_104889 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#104889:u8
        let s_75_0: bool = fn_state.gs_104889;
        // N s_75_1: branch s_75_0 b201 b76
        if s_75_0 {
            return block_201(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#104890 <= s_76_0
        fn_state.gs_104890 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#104890:u8
        let s_77_0: bool = fn_state.gs_104890;
        // N s_77_1: branch s_77_0 b200 b78
        if s_77_0 {
            return block_200(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#104891 <= s_78_0
        fn_state.gs_104891 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#104891:u8
        let s_79_0: bool = fn_state.gs_104891;
        // N s_79_1: branch s_79_0 b199 b80
        if s_79_0 {
            return block_199(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#104892 <= s_80_0
        fn_state.gs_104892 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#104892:u8
        let s_81_0: bool = fn_state.gs_104892;
        // N s_81_1: branch s_81_0 b198 b82
        if s_81_0 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call Halted(s_82_0)
        let s_82_1: bool = Halted(state, tracer, s_82_0);
        // N s_82_2: branch s_82_1 b197 b83
        if s_82_1 {
            return block_197(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#104893 <= s_83_0
        fn_state.gs_104893 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#104893:u8
        let s_84_0: bool = fn_state.gs_104893;
        // N s_84_1: branch s_84_0 b196 b85
        if s_84_0 {
            return block_196(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#104894 <= s_85_0
        fn_state.gs_104894 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#104894:u8
        let s_86_0: bool = fn_state.gs_104894;
        // N s_86_1: branch s_86_0 b195 b87
        if s_86_0 {
            return block_195(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#104895 <= s_87_0
        fn_state.gs_104895 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#104895:u8
        let s_88_0: bool = fn_state.gs_104895;
        // N s_88_1: branch s_88_0 b194 b89
        if s_88_0 {
            return block_194(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#104896 <= s_89_0
        fn_state.gs_104896 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#104896:u8
        let s_90_0: bool = fn_state.gs_104896;
        // N s_90_1: branch s_90_0 b193 b91
        if s_90_0 {
            return block_193(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#104897 <= s_91_0
        fn_state.gs_104897 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#104897:u8
        let s_92_0: bool = fn_state.gs_104897;
        // N s_92_1: branch s_92_0 b192 b93
        if s_92_0 {
            return block_192(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#104898 <= s_93_0
        fn_state.gs_104898 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#104898:u8
        let s_94_0: bool = fn_state.gs_104898;
        // N s_94_1: branch s_94_0 b191 b95
        if s_94_0 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #() : ()
        let s_95_0: () = ();
        // S s_95_1: call EL2Enabled(s_95_0)
        let s_95_1: bool = EL2Enabled(state, tracer, s_95_0);
        // N s_95_2: branch s_95_1 b190 b96
        if s_95_1 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#104899 <= s_96_0
        fn_state.gs_104899 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#104899:u8
        let s_97_0: bool = fn_state.gs_104899;
        // N s_97_1: branch s_97_0 b189 b98
        if s_97_0 {
            return block_189(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#104900 <= s_98_0
        fn_state.gs_104900 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#104900:u8
        let s_99_0: bool = fn_state.gs_104900;
        // N s_99_1: branch s_99_0 b188 b100
        if s_99_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #() : ()
        let s_100_0: () = ();
        // S s_100_1: call EL2Enabled(s_100_0)
        let s_100_1: bool = EL2Enabled(state, tracer, s_100_0);
        // N s_100_2: branch s_100_1 b187 b101
        if s_100_1 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#104901 <= s_101_0
        fn_state.gs_104901 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#104901:u8
        let s_102_0: bool = fn_state.gs_104901;
        // N s_102_1: branch s_102_0 b186 b103
        if s_102_0 {
            return block_186(state, tracer, fn_state);
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
        // D s_103_1: write-var gs#104902 <= s_103_0
        fn_state.gs_104902 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#104902:u8
        let s_104_0: bool = fn_state.gs_104902;
        // N s_104_1: branch s_104_0 b185 b105
        if s_104_0 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var __ICC_SRE_SRE:u8
        let s_105_0: bool = fn_state.u__ICC_SRE_SRE;
        // D s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 1u16);
        // C s_105_2: const #0u : u8
        let s_105_2: bool = false;
        // C s_105_3: cast zx s_105_2 -> bv
        let s_105_3: Bits = Bits::new(s_105_2 as u128, 1u16);
        // D s_105_4: cmp-eq s_105_1 s_105_3
        let s_105_4: bool = ((s_105_1) == (s_105_3));
        // N s_105_5: branch s_105_4 b184 b106
        if s_105_4 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #() : ()
        let s_106_0: () = ();
        // S s_106_1: call EL2Enabled(s_106_0)
        let s_106_1: bool = EL2Enabled(state, tracer, s_106_0);
        // N s_106_2: branch s_106_1 b183 b107
        if s_106_1 {
            return block_183(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#104903 <= s_107_0
        fn_state.gs_104903 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#104903:u8
        let s_108_0: bool = fn_state.gs_104903;
        // N s_108_1: branch s_108_0 b182 b109
        if s_108_0 {
            return block_182(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#104904 <= s_109_0
        fn_state.gs_104904 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#104904:u8
        let s_110_0: bool = fn_state.gs_104904;
        // N s_110_1: branch s_110_0 b181 b111
        if s_110_0 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #() : ()
        let s_111_0: () = ();
        // S s_111_1: call EL2Enabled(s_111_0)
        let s_111_1: bool = EL2Enabled(state, tracer, s_111_0);
        // N s_111_2: branch s_111_1 b180 b112
        if s_111_1 {
            return block_180(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#104905 <= s_112_0
        fn_state.gs_104905 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#104905:u8
        let s_113_0: bool = fn_state.gs_104905;
        // N s_113_1: branch s_113_0 b179 b114
        if s_113_0 {
            return block_179(state, tracer, fn_state);
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
        // D s_114_1: write-var gs#104906 <= s_114_0
        fn_state.gs_104906 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#104906:u8
        let s_115_0: bool = fn_state.gs_104906;
        // N s_115_1: branch s_115_0 b178 b116
        if s_115_0 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #() : ()
        let s_116_0: () = ();
        // S s_116_1: call EL2Enabled(s_116_0)
        let s_116_1: bool = EL2Enabled(state, tracer, s_116_0);
        // N s_116_2: branch s_116_1 b177 b117
        if s_116_1 {
            return block_177(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#104907 <= s_117_0
        fn_state.gs_104907 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#104907:u8
        let s_118_0: bool = fn_state.gs_104907;
        // N s_118_1: branch s_118_0 b176 b119
        if s_118_0 {
            return block_176(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#104908 <= s_119_0
        fn_state.gs_104908 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#104908:u8
        let s_120_0: bool = fn_state.gs_104908;
        // N s_120_1: branch s_120_0 b175 b121
        if s_120_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #() : ()
        let s_121_0: () = ();
        // S s_121_1: call EL2Enabled(s_121_0)
        let s_121_1: bool = EL2Enabled(state, tracer, s_121_0);
        // N s_121_2: branch s_121_1 b174 b122
        if s_121_1 {
            return block_174(state, tracer, fn_state);
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
        // D s_122_1: write-var gs#104909 <= s_122_0
        fn_state.gs_104909 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#104909:u8
        let s_123_0: bool = fn_state.gs_104909;
        // N s_123_1: branch s_123_0 b173 b124
        if s_123_0 {
            return block_173(state, tracer, fn_state);
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
        // D s_124_1: write-var gs#104910 <= s_124_0
        fn_state.gs_104910 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#104910:u8
        let s_125_0: bool = fn_state.gs_104910;
        // N s_125_1: branch s_125_0 b172 b126
        if s_125_0 {
            return block_172(state, tracer, fn_state);
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
        // S s_126_1: call EL2Enabled(s_126_0)
        let s_126_1: bool = EL2Enabled(state, tracer, s_126_0);
        // N s_126_2: branch s_126_1 b171 b127
        if s_126_1 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #0u : u8
        let s_127_0: bool = false;
        // D s_127_1: write-var gs#104911 <= s_127_0
        fn_state.gs_104911 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#104911:u8
        let s_128_0: bool = fn_state.gs_104911;
        // N s_128_1: branch s_128_0 b170 b129
        if s_128_0 {
            return block_170(state, tracer, fn_state);
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
        // D s_129_1: write-var gs#104912 <= s_129_0
        fn_state.gs_104912 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#104912:u8
        let s_130_0: bool = fn_state.gs_104912;
        // N s_130_1: branch s_130_0 b169 b131
        if s_130_0 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #() : ()
        let s_131_0: () = ();
        // S s_131_1: call EL2Enabled(s_131_0)
        let s_131_1: bool = EL2Enabled(state, tracer, s_131_0);
        // N s_131_2: branch s_131_1 b168 b132
        if s_131_1 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #0u : u8
        let s_132_0: bool = false;
        // D s_132_1: write-var gs#104913 <= s_132_0
        fn_state.gs_104913 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#104913:u8
        let s_133_0: bool = fn_state.gs_104913;
        // N s_133_1: branch s_133_0 b167 b134
        if s_133_0 {
            return block_167(state, tracer, fn_state);
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
        // D s_134_1: write-var gs#104914 <= s_134_0
        fn_state.gs_104914 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#104914:u8
        let s_135_0: bool = fn_state.gs_104914;
        // N s_135_1: branch s_135_0 b166 b136
        if s_135_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #424u : u32
        let s_136_0: u32 = 424;
        // D s_136_1: read-reg s_136_0:u8
        let s_136_1: u8 = {
            let value = state.read_register::<u8>(s_136_0 as isize);
            tracer.read_register(s_136_0 as isize, value);
            value
        };
        // C s_136_2: const #2u : u8
        let s_136_2: u8 = 2;
        // D s_136_3: cmp-lt s_136_1 s_136_2
        let s_136_3: bool = ((s_136_1) < (s_136_2));
        // N s_136_4: branch s_136_3 b165 b137
        if s_136_3 {
            return block_165(state, tracer, fn_state);
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
        // D s_137_1: write-var gs#104915 <= s_137_0
        fn_state.gs_104915 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#104915:u8
        let s_138_0: bool = fn_state.gs_104915;
        // N s_138_1: branch s_138_0 b164 b139
        if s_138_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #0u : u8
        let s_139_0: bool = false;
        // D s_139_1: write-var gs#104916 <= s_139_0
        fn_state.gs_104916 = s_139_0;
        // N s_139_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var gs#104916:u8
        let s_140_0: bool = fn_state.gs_104916;
        // N s_140_1: branch s_140_0 b158 b141
        if s_140_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #424u : u32
        let s_141_0: u32 = 424;
        // D s_141_1: read-reg s_141_0:u8
        let s_141_1: u8 = {
            let value = state.read_register::<u8>(s_141_0 as isize);
            tracer.read_register(s_141_0 as isize, value);
            value
        };
        // C s_141_2: const #2u : u8
        let s_141_2: u8 = 2;
        // D s_141_3: cmp-lt s_141_1 s_141_2
        let s_141_3: bool = ((s_141_1) < (s_141_2));
        // N s_141_4: branch s_141_3 b157 b142
        if s_141_3 {
            return block_157(state, tracer, fn_state);
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
        // D s_142_1: write-var gs#104917 <= s_142_0
        fn_state.gs_104917 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#104917:u8
        let s_143_0: bool = fn_state.gs_104917;
        // N s_143_1: branch s_143_0 b156 b144
        if s_143_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #0u : u8
        let s_144_0: bool = false;
        // D s_144_1: write-var gs#104918 <= s_144_0
        fn_state.gs_104918 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var gs#104918:u8
        let s_145_0: bool = fn_state.gs_104918;
        // N s_145_1: branch s_145_0 b155 b146
        if s_145_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #0u : u8
        let s_146_0: bool = false;
        // D s_146_1: write-var gs#104919 <= s_146_0
        fn_state.gs_104919 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#104919:u8
        let s_147_0: bool = fn_state.gs_104919;
        // N s_147_1: branch s_147_0 b149 b148
        if s_147_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var t2:i
        let s_148_0: i128 = fn_state.t2;
        // D s_148_1: call R_read(s_148_0)
        let s_148_1: u32 = R_read(state, tracer, s_148_0);
        // D s_148_2: read-var t:i
        let s_148_2: i128 = fn_state.t;
        // D s_148_3: call R_read(s_148_2)
        let s_148_3: u32 = R_read(state, tracer, s_148_2);
        // D s_148_4: cast zx s_148_1 -> bv
        let s_148_4: Bits = Bits::new(s_148_1 as u128, 32u16);
        // D s_148_5: cast zx s_148_3 -> bv
        let s_148_5: Bits = Bits::new(s_148_3 as u128, 32u16);
        // D s_148_6: cast reint s_148_4 -> u128
        let s_148_6: u128 = (s_148_4.value() as u128);
        // D s_148_7: size-of s_148_4
        let s_148_7: u16 = s_148_4.length();
        // D s_148_8: cast reint s_148_5 -> u128
        let s_148_8: u128 = (s_148_5.value() as u128);
        // D s_148_9: size-of s_148_5
        let s_148_9: u16 = s_148_5.length();
        // D s_148_10: lsl s_148_6 s_148_9
        let s_148_10: u128 = s_148_6 << s_148_9;
        // D s_148_11: or s_148_10 s_148_8
        let s_148_11: u128 = ((s_148_10) | (s_148_8));
        // D s_148_12: add s_148_7 s_148_9
        let s_148_12: u16 = (s_148_7 + s_148_9);
        // D s_148_13: create-bits s_148_11 s_148_12
        let s_148_13: Bits = Bits::new(s_148_11, s_148_12);
        // D s_148_14: cast reint s_148_13 -> u64
        let s_148_14: u64 = (s_148_13.value() as u64);
        // D s_148_15: call Mk_ICC_SGI0R_Type(s_148_14)
        let s_148_15: ProductType5c790c8ef59cc8b2 = Mk_ICC_SGI0R_Type(
            state,
            tracer,
            s_148_14,
        );
        // D s_148_16: call ICC_SGI0R_write(s_148_15)
        let s_148_16: () = ICC_SGI0R_write(state, tracer, s_148_15);
        // N s_148_17: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #() : ()
        let s_149_0: () = ();
        // S s_149_1: call Halted(s_149_0)
        let s_149_1: bool = Halted(state, tracer, s_149_0);
        // N s_149_2: branch s_149_1 b154 b150
        if s_149_1 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #0u : u8
        let s_150_0: bool = false;
        // D s_150_1: write-var gs#104920 <= s_150_0
        fn_state.gs_104920 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#104920:u8
        let s_151_0: bool = fn_state.gs_104920;
        // N s_151_1: branch s_151_0 b153 b152
        if s_151_0 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #() : ()
        let s_152_0: () = ();
        // S s_152_1: call AArch32_TakeMonitorTrapException(s_152_0)
        let s_152_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_152_0);
        // N s_152_2: return
        return;
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_153_0: panic
        panic!("{:?}", ());
        // N s_153_1: return
        return;
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #() : ()
        let s_154_0: () = ();
        // S s_154_1: call EDSCR_read(s_154_0)
        let s_154_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_154_0);
        // S s_154_2: call _get_EDSCR_Type_SDD(s_154_1)
        let s_154_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_154_1);
        // S s_154_3: cast zx s_154_2 -> bv
        let s_154_3: Bits = Bits::new(s_154_2 as u128, 1u16);
        // C s_154_4: const #1u : u8
        let s_154_4: bool = true;
        // C s_154_5: cast zx s_154_4 -> bv
        let s_154_5: Bits = Bits::new(s_154_4 as u128, 1u16);
        // S s_154_6: cmp-eq s_154_3 s_154_5
        let s_154_6: bool = ((s_154_3) == (s_154_5));
        // D s_154_7: write-var gs#104920 <= s_154_6
        fn_state.gs_104920 = s_154_6;
        // N s_154_8: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #20920u : u32
        let s_155_0: u32 = 20920;
        // D s_155_1: read-reg s_155_0:struct
        let s_155_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_155_0 as isize);
            tracer.read_register(s_155_0 as isize, value);
            value
        };
        // D s_155_2: call _get_SCR_Type_IRQ(s_155_1)
        let s_155_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_155_1);
        // C s_155_3: const #20920u : u32
        let s_155_3: u32 = 20920;
        // D s_155_4: read-reg s_155_3:struct
        let s_155_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_155_3 as isize);
            tracer.read_register(s_155_3 as isize, value);
            value
        };
        // D s_155_5: call _get_SCR_Type_FIQ(s_155_4)
        let s_155_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_155_4);
        // D s_155_6: cast zx s_155_2 -> bv
        let s_155_6: Bits = Bits::new(s_155_2 as u128, 1u16);
        // D s_155_7: cast zx s_155_5 -> bv
        let s_155_7: Bits = Bits::new(s_155_5 as u128, 1u16);
        // D s_155_8: cast reint s_155_6 -> u128
        let s_155_8: u128 = (s_155_6.value() as u128);
        // D s_155_9: size-of s_155_6
        let s_155_9: u16 = s_155_6.length();
        // D s_155_10: cast reint s_155_7 -> u128
        let s_155_10: u128 = (s_155_7.value() as u128);
        // D s_155_11: size-of s_155_7
        let s_155_11: u16 = s_155_7.length();
        // D s_155_12: lsl s_155_8 s_155_11
        let s_155_12: u128 = s_155_8 << s_155_11;
        // D s_155_13: or s_155_12 s_155_10
        let s_155_13: u128 = ((s_155_12) | (s_155_10));
        // D s_155_14: add s_155_9 s_155_11
        let s_155_14: u16 = (s_155_9 + s_155_11);
        // D s_155_15: create-bits s_155_13 s_155_14
        let s_155_15: Bits = Bits::new(s_155_13, s_155_14);
        // D s_155_16: cast reint s_155_15 -> u8
        let s_155_16: u8 = (s_155_15.value() as u8);
        // D s_155_17: cast zx s_155_16 -> bv
        let s_155_17: Bits = Bits::new(s_155_16 as u128, 2u16);
        // C s_155_18: const #3u : u8
        let s_155_18: u8 = 3;
        // C s_155_19: cast zx s_155_18 -> bv
        let s_155_19: Bits = Bits::new(s_155_18 as u128, 2u16);
        // D s_155_20: cmp-eq s_155_17 s_155_19
        let s_155_20: bool = ((s_155_17) == (s_155_19));
        // D s_155_21: write-var gs#104919 <= s_155_20
        fn_state.gs_104919 = s_155_20;
        // N s_155_22: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var __PSTATE_M:u8
        let s_156_0: u8 = fn_state.u__PSTATE_M;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 5u16);
        // C s_156_2: const #384u : u32
        let s_156_2: u32 = 384;
        // D s_156_3: read-reg s_156_2:u8
        let s_156_3: u8 = {
            let value = state.read_register::<u8>(s_156_2 as isize);
            tracer.read_register(s_156_2 as isize, value);
            value
        };
        // D s_156_4: cast zx s_156_3 -> bv
        let s_156_4: Bits = Bits::new(s_156_3 as u128, 5u16);
        // D s_156_5: cmp-ne s_156_1 s_156_4
        let s_156_5: bool = ((s_156_1) != (s_156_4));
        // D s_156_6: write-var gs#104918 <= s_156_5
        fn_state.gs_104918 = s_156_5;
        // N s_156_7: jump b145
        return block_145(state, tracer, fn_state);
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
        // D s_157_3: write-var gs#104917 <= s_157_2
        fn_state.gs_104917 = s_157_2;
        // N s_157_4: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #() : ()
        let s_158_0: () = ();
        // S s_158_1: call Halted(s_158_0)
        let s_158_1: bool = Halted(state, tracer, s_158_0);
        // N s_158_2: branch s_158_1 b163 b159
        if s_158_1 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #0u : u8
        let s_159_0: bool = false;
        // D s_159_1: write-var gs#104921 <= s_159_0
        fn_state.gs_104921 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#104921:u8
        let s_160_0: bool = fn_state.gs_104921;
        // N s_160_1: branch s_160_0 b162 b161
        if s_160_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #3u : u8
        let s_161_0: u8 = 3;
        // C s_161_1: cast zx s_161_0 -> bv
        let s_161_1: Bits = Bits::new(s_161_0 as u128, 8u16);
        // C s_161_2: cast zx s_161_1 -> i
        let s_161_2: i128 = (s_161_1.value() as i128);
        // C s_161_3: cast reint s_161_2 -> i64
        let s_161_3: i64 = (s_161_2 as i64);
        // C s_161_4: cast zx s_161_3 -> i
        let s_161_4: i128 = (i128::try_from(s_161_3).unwrap());
        // C s_161_5: const #424u : u32
        let s_161_5: u32 = 424;
        // D s_161_6: read-reg s_161_5:u8
        let s_161_6: u8 = {
            let value = state.read_register::<u8>(s_161_5 as isize);
            tracer.read_register(s_161_5 as isize, value);
            value
        };
        // D s_161_7: call AArch64_AArch32SystemAccessTrap(s_161_6, s_161_4)
        let s_161_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_161_6,
            s_161_4,
        );
        // N s_161_8: return
        return;
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
        // C s_163_0: const #() : ()
        let s_163_0: () = ();
        // S s_163_1: call EDSCR_read(s_163_0)
        let s_163_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_163_0);
        // S s_163_2: call _get_EDSCR_Type_SDD(s_163_1)
        let s_163_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_163_1);
        // S s_163_3: cast zx s_163_2 -> bv
        let s_163_3: Bits = Bits::new(s_163_2 as u128, 1u16);
        // C s_163_4: const #1u : u8
        let s_163_4: bool = true;
        // C s_163_5: cast zx s_163_4 -> bv
        let s_163_5: Bits = Bits::new(s_163_4 as u128, 1u16);
        // S s_163_6: cmp-eq s_163_3 s_163_5
        let s_163_6: bool = ((s_163_3) == (s_163_5));
        // D s_163_7: write-var gs#104921 <= s_163_6
        fn_state.gs_104921 = s_163_6;
        // N s_163_8: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #90704u : u32
        let s_164_0: u32 = 90704;
        // D s_164_1: read-reg s_164_0:struct
        let s_164_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_164_0 as isize);
            tracer.read_register(s_164_0 as isize, value);
            value
        };
        // D s_164_2: call _get_SCR_EL3_Type_IRQ(s_164_1)
        let s_164_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_164_1);
        // C s_164_3: const #90704u : u32
        let s_164_3: u32 = 90704;
        // D s_164_4: read-reg s_164_3:struct
        let s_164_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_164_3 as isize);
            tracer.read_register(s_164_3 as isize, value);
            value
        };
        // D s_164_5: call _get_SCR_EL3_Type_FIQ(s_164_4)
        let s_164_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_164_4);
        // D s_164_6: cast zx s_164_2 -> bv
        let s_164_6: Bits = Bits::new(s_164_2 as u128, 1u16);
        // D s_164_7: cast zx s_164_5 -> bv
        let s_164_7: Bits = Bits::new(s_164_5 as u128, 1u16);
        // D s_164_8: cast reint s_164_6 -> u128
        let s_164_8: u128 = (s_164_6.value() as u128);
        // D s_164_9: size-of s_164_6
        let s_164_9: u16 = s_164_6.length();
        // D s_164_10: cast reint s_164_7 -> u128
        let s_164_10: u128 = (s_164_7.value() as u128);
        // D s_164_11: size-of s_164_7
        let s_164_11: u16 = s_164_7.length();
        // D s_164_12: lsl s_164_8 s_164_11
        let s_164_12: u128 = s_164_8 << s_164_11;
        // D s_164_13: or s_164_12 s_164_10
        let s_164_13: u128 = ((s_164_12) | (s_164_10));
        // D s_164_14: add s_164_9 s_164_11
        let s_164_14: u16 = (s_164_9 + s_164_11);
        // D s_164_15: create-bits s_164_13 s_164_14
        let s_164_15: Bits = Bits::new(s_164_13, s_164_14);
        // D s_164_16: cast reint s_164_15 -> u8
        let s_164_16: u8 = (s_164_15.value() as u8);
        // D s_164_17: cast zx s_164_16 -> bv
        let s_164_17: Bits = Bits::new(s_164_16 as u128, 2u16);
        // C s_164_18: const #3u : u8
        let s_164_18: u8 = 3;
        // C s_164_19: cast zx s_164_18 -> bv
        let s_164_19: Bits = Bits::new(s_164_18 as u128, 2u16);
        // D s_164_20: cmp-eq s_164_17 s_164_19
        let s_164_20: bool = ((s_164_17) == (s_164_19));
        // D s_164_21: write-var gs#104916 <= s_164_20
        fn_state.gs_104916 = s_164_20;
        // N s_164_22: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #424u : u32
        let s_165_0: u32 = 424;
        // D s_165_1: read-reg s_165_0:u8
        let s_165_1: u8 = {
            let value = state.read_register::<u8>(s_165_0 as isize);
            tracer.read_register(s_165_0 as isize, value);
            value
        };
        // D s_165_2: call ELUsingAArch32(s_165_1)
        let s_165_2: bool = ELUsingAArch32(state, tracer, s_165_1);
        // D s_165_3: not s_165_2
        let s_165_3: bool = !s_165_2;
        // D s_165_4: write-var gs#104915 <= s_165_3
        fn_state.gs_104915 = s_165_3;
        // N s_165_5: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #3u : u8
        let s_166_0: u8 = 3;
        // C s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 8u16);
        // C s_166_2: cast zx s_166_1 -> i
        let s_166_2: i128 = (s_166_1.value() as i128);
        // C s_166_3: cast reint s_166_2 -> i64
        let s_166_3: i64 = (s_166_2 as i64);
        // C s_166_4: cast zx s_166_3 -> i
        let s_166_4: i128 = (i128::try_from(s_166_3).unwrap());
        // S s_166_5: call AArch32_TakeHypTrapException(s_166_4)
        let s_166_5: () = AArch32_TakeHypTrapException(state, tracer, s_166_4);
        // N s_166_6: return
        return;
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var __HCR_IMO:u8
        let s_167_0: bool = fn_state.u__HCR_IMO;
        // D s_167_1: cast zx s_167_0 -> bv
        let s_167_1: Bits = Bits::new(s_167_0 as u128, 1u16);
        // C s_167_2: const #1u : u8
        let s_167_2: bool = true;
        // C s_167_3: cast zx s_167_2 -> bv
        let s_167_3: Bits = Bits::new(s_167_2 as u128, 1u16);
        // D s_167_4: cmp-eq s_167_1 s_167_3
        let s_167_4: bool = ((s_167_1) == (s_167_3));
        // D s_167_5: write-var gs#104914 <= s_167_4
        fn_state.gs_104914 = s_167_4;
        // N s_167_6: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #432u : u32
        let s_168_0: u32 = 432;
        // D s_168_1: read-reg s_168_0:u8
        let s_168_1: u8 = {
            let value = state.read_register::<u8>(s_168_0 as isize);
            tracer.read_register(s_168_0 as isize, value);
            value
        };
        // D s_168_2: call ELUsingAArch32(s_168_1)
        let s_168_2: bool = ELUsingAArch32(state, tracer, s_168_1);
        // D s_168_3: write-var gs#104913 <= s_168_2
        fn_state.gs_104913 = s_168_2;
        // N s_168_4: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #3u : u8
        let s_169_0: u8 = 3;
        // C s_169_1: cast zx s_169_0 -> bv
        let s_169_1: Bits = Bits::new(s_169_0 as u128, 8u16);
        // C s_169_2: cast zx s_169_1 -> i
        let s_169_2: i128 = (s_169_1.value() as i128);
        // C s_169_3: cast reint s_169_2 -> i64
        let s_169_3: i64 = (s_169_2 as i64);
        // C s_169_4: cast zx s_169_3 -> i
        let s_169_4: i128 = (i128::try_from(s_169_3).unwrap());
        // S s_169_5: call AArch32_TakeHypTrapException(s_169_4)
        let s_169_5: () = AArch32_TakeHypTrapException(state, tracer, s_169_4);
        // N s_169_6: return
        return;
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var __HCR_FMO:u8
        let s_170_0: bool = fn_state.u__HCR_FMO;
        // D s_170_1: cast zx s_170_0 -> bv
        let s_170_1: Bits = Bits::new(s_170_0 as u128, 1u16);
        // C s_170_2: const #1u : u8
        let s_170_2: bool = true;
        // C s_170_3: cast zx s_170_2 -> bv
        let s_170_3: Bits = Bits::new(s_170_2 as u128, 1u16);
        // D s_170_4: cmp-eq s_170_1 s_170_3
        let s_170_4: bool = ((s_170_1) == (s_170_3));
        // D s_170_5: write-var gs#104912 <= s_170_4
        fn_state.gs_104912 = s_170_4;
        // N s_170_6: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #432u : u32
        let s_171_0: u32 = 432;
        // D s_171_1: read-reg s_171_0:u8
        let s_171_1: u8 = {
            let value = state.read_register::<u8>(s_171_0 as isize);
            tracer.read_register(s_171_0 as isize, value);
            value
        };
        // D s_171_2: call ELUsingAArch32(s_171_1)
        let s_171_2: bool = ELUsingAArch32(state, tracer, s_171_1);
        // D s_171_3: write-var gs#104911 <= s_171_2
        fn_state.gs_104911 = s_171_2;
        // N s_171_4: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #3u : u8
        let s_172_0: u8 = 3;
        // C s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 8u16);
        // C s_172_2: cast zx s_172_1 -> i
        let s_172_2: i128 = (s_172_1.value() as i128);
        // C s_172_3: cast reint s_172_2 -> i64
        let s_172_3: i64 = (s_172_2 as i64);
        // C s_172_4: cast zx s_172_3 -> i
        let s_172_4: i128 = (i128::try_from(s_172_3).unwrap());
        // C s_172_5: const #432u : u32
        let s_172_5: u32 = 432;
        // D s_172_6: read-reg s_172_5:u8
        let s_172_6: u8 = {
            let value = state.read_register::<u8>(s_172_5 as isize);
            tracer.read_register(s_172_5 as isize, value);
            value
        };
        // D s_172_7: call AArch64_AArch32SystemAccessTrap(s_172_6, s_172_4)
        let s_172_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_172_6,
            s_172_4,
        );
        // N s_172_8: return
        return;
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var __HCR_EL2_IMO:u8
        let s_173_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_173_1: cast zx s_173_0 -> bv
        let s_173_1: Bits = Bits::new(s_173_0 as u128, 1u16);
        // C s_173_2: const #1u : u8
        let s_173_2: bool = true;
        // C s_173_3: cast zx s_173_2 -> bv
        let s_173_3: Bits = Bits::new(s_173_2 as u128, 1u16);
        // D s_173_4: cmp-eq s_173_1 s_173_3
        let s_173_4: bool = ((s_173_1) == (s_173_3));
        // D s_173_5: write-var gs#104910 <= s_173_4
        fn_state.gs_104910 = s_173_4;
        // N s_173_6: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #432u : u32
        let s_174_0: u32 = 432;
        // D s_174_1: read-reg s_174_0:u8
        let s_174_1: u8 = {
            let value = state.read_register::<u8>(s_174_0 as isize);
            tracer.read_register(s_174_0 as isize, value);
            value
        };
        // D s_174_2: call ELUsingAArch32(s_174_1)
        let s_174_2: bool = ELUsingAArch32(state, tracer, s_174_1);
        // D s_174_3: not s_174_2
        let s_174_3: bool = !s_174_2;
        // D s_174_4: write-var gs#104909 <= s_174_3
        fn_state.gs_104909 = s_174_3;
        // N s_174_5: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #3u : u8
        let s_175_0: u8 = 3;
        // C s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 8u16);
        // C s_175_2: cast zx s_175_1 -> i
        let s_175_2: i128 = (s_175_1.value() as i128);
        // C s_175_3: cast reint s_175_2 -> i64
        let s_175_3: i64 = (s_175_2 as i64);
        // C s_175_4: cast zx s_175_3 -> i
        let s_175_4: i128 = (i128::try_from(s_175_3).unwrap());
        // C s_175_5: const #432u : u32
        let s_175_5: u32 = 432;
        // D s_175_6: read-reg s_175_5:u8
        let s_175_6: u8 = {
            let value = state.read_register::<u8>(s_175_5 as isize);
            tracer.read_register(s_175_5 as isize, value);
            value
        };
        // D s_175_7: call AArch64_AArch32SystemAccessTrap(s_175_6, s_175_4)
        let s_175_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_175_6,
            s_175_4,
        );
        // N s_175_8: return
        return;
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var __HCR_EL2_FMO:u8
        let s_176_0: bool = fn_state.u__HCR_EL2_FMO;
        // D s_176_1: cast zx s_176_0 -> bv
        let s_176_1: Bits = Bits::new(s_176_0 as u128, 1u16);
        // C s_176_2: const #1u : u8
        let s_176_2: bool = true;
        // C s_176_3: cast zx s_176_2 -> bv
        let s_176_3: Bits = Bits::new(s_176_2 as u128, 1u16);
        // D s_176_4: cmp-eq s_176_1 s_176_3
        let s_176_4: bool = ((s_176_1) == (s_176_3));
        // D s_176_5: write-var gs#104908 <= s_176_4
        fn_state.gs_104908 = s_176_4;
        // N s_176_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #432u : u32
        let s_177_0: u32 = 432;
        // D s_177_1: read-reg s_177_0:u8
        let s_177_1: u8 = {
            let value = state.read_register::<u8>(s_177_0 as isize);
            tracer.read_register(s_177_0 as isize, value);
            value
        };
        // D s_177_2: call ELUsingAArch32(s_177_1)
        let s_177_2: bool = ELUsingAArch32(state, tracer, s_177_1);
        // D s_177_3: not s_177_2
        let s_177_3: bool = !s_177_2;
        // D s_177_4: write-var gs#104907 <= s_177_3
        fn_state.gs_104907 = s_177_3;
        // N s_177_5: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #3u : u8
        let s_178_0: u8 = 3;
        // C s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 8u16);
        // C s_178_2: cast zx s_178_1 -> i
        let s_178_2: i128 = (s_178_1.value() as i128);
        // C s_178_3: cast reint s_178_2 -> i64
        let s_178_3: i64 = (s_178_2 as i64);
        // C s_178_4: cast zx s_178_3 -> i
        let s_178_4: i128 = (i128::try_from(s_178_3).unwrap());
        // S s_178_5: call AArch32_TakeHypTrapException(s_178_4)
        let s_178_5: () = AArch32_TakeHypTrapException(state, tracer, s_178_4);
        // N s_178_6: return
        return;
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_179_0: read-var __ICH_HCR_TC:u8
        let s_179_0: bool = fn_state.u__ICH_HCR_TC;
        // D s_179_1: cast zx s_179_0 -> bv
        let s_179_1: Bits = Bits::new(s_179_0 as u128, 1u16);
        // C s_179_2: const #1u : u8
        let s_179_2: bool = true;
        // C s_179_3: cast zx s_179_2 -> bv
        let s_179_3: Bits = Bits::new(s_179_2 as u128, 1u16);
        // D s_179_4: cmp-eq s_179_1 s_179_3
        let s_179_4: bool = ((s_179_1) == (s_179_3));
        // D s_179_5: write-var gs#104906 <= s_179_4
        fn_state.gs_104906 = s_179_4;
        // N s_179_6: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #432u : u32
        let s_180_0: u32 = 432;
        // D s_180_1: read-reg s_180_0:u8
        let s_180_1: u8 = {
            let value = state.read_register::<u8>(s_180_0 as isize);
            tracer.read_register(s_180_0 as isize, value);
            value
        };
        // D s_180_2: call ELUsingAArch32(s_180_1)
        let s_180_2: bool = ELUsingAArch32(state, tracer, s_180_1);
        // D s_180_3: write-var gs#104905 <= s_180_2
        fn_state.gs_104905 = s_180_2;
        // N s_180_4: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #3u : u8
        let s_181_0: u8 = 3;
        // C s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 8u16);
        // C s_181_2: cast zx s_181_1 -> i
        let s_181_2: i128 = (s_181_1.value() as i128);
        // C s_181_3: cast reint s_181_2 -> i64
        let s_181_3: i64 = (s_181_2 as i64);
        // C s_181_4: cast zx s_181_3 -> i
        let s_181_4: i128 = (i128::try_from(s_181_3).unwrap());
        // C s_181_5: const #432u : u32
        let s_181_5: u32 = 432;
        // D s_181_6: read-reg s_181_5:u8
        let s_181_6: u8 = {
            let value = state.read_register::<u8>(s_181_5 as isize);
            tracer.read_register(s_181_5 as isize, value);
            value
        };
        // D s_181_7: call AArch64_AArch32SystemAccessTrap(s_181_6, s_181_4)
        let s_181_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_181_6,
            s_181_4,
        );
        // N s_181_8: return
        return;
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var __ICH_HCR_EL2_TC:u8
        let s_182_0: bool = fn_state.u__ICH_HCR_EL2_TC;
        // D s_182_1: cast zx s_182_0 -> bv
        let s_182_1: Bits = Bits::new(s_182_0 as u128, 1u16);
        // C s_182_2: const #1u : u8
        let s_182_2: bool = true;
        // C s_182_3: cast zx s_182_2 -> bv
        let s_182_3: Bits = Bits::new(s_182_2 as u128, 1u16);
        // D s_182_4: cmp-eq s_182_1 s_182_3
        let s_182_4: bool = ((s_182_1) == (s_182_3));
        // D s_182_5: write-var gs#104904 <= s_182_4
        fn_state.gs_104904 = s_182_4;
        // N s_182_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #432u : u32
        let s_183_0: u32 = 432;
        // D s_183_1: read-reg s_183_0:u8
        let s_183_1: u8 = {
            let value = state.read_register::<u8>(s_183_0 as isize);
            tracer.read_register(s_183_0 as isize, value);
            value
        };
        // D s_183_2: call ELUsingAArch32(s_183_1)
        let s_183_2: bool = ELUsingAArch32(state, tracer, s_183_1);
        // D s_183_3: not s_183_2
        let s_183_3: bool = !s_183_2;
        // D s_183_4: write-var gs#104903 <= s_183_3
        fn_state.gs_104903 = s_183_3;
        // N s_183_5: jump b108
        return block_108(state, tracer, fn_state);
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
        // C s_185_0: const #3u : u8
        let s_185_0: u8 = 3;
        // C s_185_1: cast zx s_185_0 -> bv
        let s_185_1: Bits = Bits::new(s_185_0 as u128, 8u16);
        // C s_185_2: cast zx s_185_1 -> i
        let s_185_2: i128 = (s_185_1.value() as i128);
        // C s_185_3: cast reint s_185_2 -> i64
        let s_185_3: i64 = (s_185_2 as i64);
        // C s_185_4: cast zx s_185_3 -> i
        let s_185_4: i128 = (i128::try_from(s_185_3).unwrap());
        // S s_185_5: call AArch32_TakeHypTrapException(s_185_4)
        let s_185_5: () = AArch32_TakeHypTrapException(state, tracer, s_185_4);
        // N s_185_6: return
        return;
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var __HSTR_T12:u8
        let s_186_0: bool = fn_state.u__HSTR_T12;
        // D s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 1u16);
        // C s_186_2: const #1u : u8
        let s_186_2: bool = true;
        // C s_186_3: cast zx s_186_2 -> bv
        let s_186_3: Bits = Bits::new(s_186_2 as u128, 1u16);
        // D s_186_4: cmp-eq s_186_1 s_186_3
        let s_186_4: bool = ((s_186_1) == (s_186_3));
        // D s_186_5: write-var gs#104902 <= s_186_4
        fn_state.gs_104902 = s_186_4;
        // N s_186_6: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #432u : u32
        let s_187_0: u32 = 432;
        // D s_187_1: read-reg s_187_0:u8
        let s_187_1: u8 = {
            let value = state.read_register::<u8>(s_187_0 as isize);
            tracer.read_register(s_187_0 as isize, value);
            value
        };
        // D s_187_2: call ELUsingAArch32(s_187_1)
        let s_187_2: bool = ELUsingAArch32(state, tracer, s_187_1);
        // D s_187_3: write-var gs#104901 <= s_187_2
        fn_state.gs_104901 = s_187_2;
        // N s_187_4: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #3u : u8
        let s_188_0: u8 = 3;
        // C s_188_1: cast zx s_188_0 -> bv
        let s_188_1: Bits = Bits::new(s_188_0 as u128, 8u16);
        // C s_188_2: cast zx s_188_1 -> i
        let s_188_2: i128 = (s_188_1.value() as i128);
        // C s_188_3: cast reint s_188_2 -> i64
        let s_188_3: i64 = (s_188_2 as i64);
        // C s_188_4: cast zx s_188_3 -> i
        let s_188_4: i128 = (i128::try_from(s_188_3).unwrap());
        // C s_188_5: const #432u : u32
        let s_188_5: u32 = 432;
        // D s_188_6: read-reg s_188_5:u8
        let s_188_6: u8 = {
            let value = state.read_register::<u8>(s_188_5 as isize);
            tracer.read_register(s_188_5 as isize, value);
            value
        };
        // D s_188_7: call AArch64_AArch32SystemAccessTrap(s_188_6, s_188_4)
        let s_188_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_188_6,
            s_188_4,
        );
        // N s_188_8: return
        return;
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_189_0: read-var __HSTR_EL2_T12:u8
        let s_189_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 1u16);
        // C s_189_2: const #1u : u8
        let s_189_2: bool = true;
        // C s_189_3: cast zx s_189_2 -> bv
        let s_189_3: Bits = Bits::new(s_189_2 as u128, 1u16);
        // D s_189_4: cmp-eq s_189_1 s_189_3
        let s_189_4: bool = ((s_189_1) == (s_189_3));
        // D s_189_5: write-var gs#104900 <= s_189_4
        fn_state.gs_104900 = s_189_4;
        // N s_189_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #432u : u32
        let s_190_0: u32 = 432;
        // D s_190_1: read-reg s_190_0:u8
        let s_190_1: u8 = {
            let value = state.read_register::<u8>(s_190_0 as isize);
            tracer.read_register(s_190_0 as isize, value);
            value
        };
        // D s_190_2: call ELUsingAArch32(s_190_1)
        let s_190_2: bool = ELUsingAArch32(state, tracer, s_190_1);
        // D s_190_3: not s_190_2
        let s_190_3: bool = !s_190_2;
        // D s_190_4: write-var gs#104899 <= s_190_3
        fn_state.gs_104899 = s_190_3;
        // N s_190_5: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_191_0: panic
        panic!("{:?}", ());
        // N s_191_1: return
        return;
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #20920u : u32
        let s_192_0: u32 = 20920;
        // D s_192_1: read-reg s_192_0:struct
        let s_192_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_192_0 as isize);
            tracer.read_register(s_192_0 as isize, value);
            value
        };
        // D s_192_2: call _get_SCR_Type_IRQ(s_192_1)
        let s_192_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_192_1);
        // C s_192_3: const #20920u : u32
        let s_192_3: u32 = 20920;
        // D s_192_4: read-reg s_192_3:struct
        let s_192_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_192_3 as isize);
            tracer.read_register(s_192_3 as isize, value);
            value
        };
        // D s_192_5: call _get_SCR_Type_FIQ(s_192_4)
        let s_192_5: bool = u_get_SCR_Type_FIQ(state, tracer, s_192_4);
        // D s_192_6: cast zx s_192_2 -> bv
        let s_192_6: Bits = Bits::new(s_192_2 as u128, 1u16);
        // D s_192_7: cast zx s_192_5 -> bv
        let s_192_7: Bits = Bits::new(s_192_5 as u128, 1u16);
        // D s_192_8: cast reint s_192_6 -> u128
        let s_192_8: u128 = (s_192_6.value() as u128);
        // D s_192_9: size-of s_192_6
        let s_192_9: u16 = s_192_6.length();
        // D s_192_10: cast reint s_192_7 -> u128
        let s_192_10: u128 = (s_192_7.value() as u128);
        // D s_192_11: size-of s_192_7
        let s_192_11: u16 = s_192_7.length();
        // D s_192_12: lsl s_192_8 s_192_11
        let s_192_12: u128 = s_192_8 << s_192_11;
        // D s_192_13: or s_192_12 s_192_10
        let s_192_13: u128 = ((s_192_12) | (s_192_10));
        // D s_192_14: add s_192_9 s_192_11
        let s_192_14: u16 = (s_192_9 + s_192_11);
        // D s_192_15: create-bits s_192_13 s_192_14
        let s_192_15: Bits = Bits::new(s_192_13, s_192_14);
        // D s_192_16: cast reint s_192_15 -> u8
        let s_192_16: u8 = (s_192_15.value() as u8);
        // D s_192_17: cast zx s_192_16 -> bv
        let s_192_17: Bits = Bits::new(s_192_16 as u128, 2u16);
        // C s_192_18: const #3u : u8
        let s_192_18: u8 = 3;
        // C s_192_19: cast zx s_192_18 -> bv
        let s_192_19: Bits = Bits::new(s_192_18 as u128, 2u16);
        // D s_192_20: cmp-eq s_192_17 s_192_19
        let s_192_20: bool = ((s_192_17) == (s_192_19));
        // D s_192_21: write-var gs#104898 <= s_192_20
        fn_state.gs_104898 = s_192_20;
        // N s_192_22: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var __PSTATE_M:u8
        let s_193_0: u8 = fn_state.u__PSTATE_M;
        // D s_193_1: cast zx s_193_0 -> bv
        let s_193_1: Bits = Bits::new(s_193_0 as u128, 5u16);
        // C s_193_2: const #384u : u32
        let s_193_2: u32 = 384;
        // D s_193_3: read-reg s_193_2:u8
        let s_193_3: u8 = {
            let value = state.read_register::<u8>(s_193_2 as isize);
            tracer.read_register(s_193_2 as isize, value);
            value
        };
        // D s_193_4: cast zx s_193_3 -> bv
        let s_193_4: Bits = Bits::new(s_193_3 as u128, 5u16);
        // D s_193_5: cmp-ne s_193_1 s_193_4
        let s_193_5: bool = ((s_193_1) != (s_193_4));
        // D s_193_6: write-var gs#104897 <= s_193_5
        fn_state.gs_104897 = s_193_5;
        // N s_193_7: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #424u : u32
        let s_194_0: u32 = 424;
        // D s_194_1: read-reg s_194_0:u8
        let s_194_1: u8 = {
            let value = state.read_register::<u8>(s_194_0 as isize);
            tracer.read_register(s_194_0 as isize, value);
            value
        };
        // D s_194_2: call ELUsingAArch32(s_194_1)
        let s_194_2: bool = ELUsingAArch32(state, tracer, s_194_1);
        // D s_194_3: write-var gs#104896 <= s_194_2
        fn_state.gs_104896 = s_194_2;
        // N s_194_4: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_195_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_195_1: call __IMPDEF_boolean(s_195_0)
        let s_195_1: bool = u__IMPDEF_boolean(state, tracer, s_195_0);
        // D s_195_2: write-var gs#104895 <= s_195_1
        fn_state.gs_104895 = s_195_1;
        // N s_195_3: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #() : ()
        let s_196_0: () = ();
        // S s_196_1: call EDSCR_read(s_196_0)
        let s_196_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_196_0);
        // S s_196_2: call _get_EDSCR_Type_SDD(s_196_1)
        let s_196_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_196_1);
        // S s_196_3: cast zx s_196_2 -> bv
        let s_196_3: Bits = Bits::new(s_196_2 as u128, 1u16);
        // C s_196_4: const #1u : u8
        let s_196_4: bool = true;
        // C s_196_5: cast zx s_196_4 -> bv
        let s_196_5: Bits = Bits::new(s_196_4 as u128, 1u16);
        // S s_196_6: cmp-eq s_196_3 s_196_5
        let s_196_6: bool = ((s_196_3) == (s_196_5));
        // D s_196_7: write-var gs#104894 <= s_196_6
        fn_state.gs_104894 = s_196_6;
        // N s_196_8: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #424u : u32
        let s_197_0: u32 = 424;
        // D s_197_1: read-reg s_197_0:u8
        let s_197_1: u8 = {
            let value = state.read_register::<u8>(s_197_0 as isize);
            tracer.read_register(s_197_0 as isize, value);
            value
        };
        // C s_197_2: const #2u : u8
        let s_197_2: u8 = 2;
        // D s_197_3: cmp-lt s_197_1 s_197_2
        let s_197_3: bool = ((s_197_1) < (s_197_2));
        // D s_197_4: write-var gs#104893 <= s_197_3
        fn_state.gs_104893 = s_197_3;
        // N s_197_5: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_198_0: panic
        panic!("{:?}", ());
        // N s_198_1: return
        return;
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #90704u : u32
        let s_199_0: u32 = 90704;
        // D s_199_1: read-reg s_199_0:struct
        let s_199_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_199_0 as isize);
            tracer.read_register(s_199_0 as isize, value);
            value
        };
        // D s_199_2: call _get_SCR_EL3_Type_IRQ(s_199_1)
        let s_199_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_199_1);
        // C s_199_3: const #90704u : u32
        let s_199_3: u32 = 90704;
        // D s_199_4: read-reg s_199_3:struct
        let s_199_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_199_3 as isize);
            tracer.read_register(s_199_3 as isize, value);
            value
        };
        // D s_199_5: call _get_SCR_EL3_Type_FIQ(s_199_4)
        let s_199_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_199_4);
        // D s_199_6: cast zx s_199_2 -> bv
        let s_199_6: Bits = Bits::new(s_199_2 as u128, 1u16);
        // D s_199_7: cast zx s_199_5 -> bv
        let s_199_7: Bits = Bits::new(s_199_5 as u128, 1u16);
        // D s_199_8: cast reint s_199_6 -> u128
        let s_199_8: u128 = (s_199_6.value() as u128);
        // D s_199_9: size-of s_199_6
        let s_199_9: u16 = s_199_6.length();
        // D s_199_10: cast reint s_199_7 -> u128
        let s_199_10: u128 = (s_199_7.value() as u128);
        // D s_199_11: size-of s_199_7
        let s_199_11: u16 = s_199_7.length();
        // D s_199_12: lsl s_199_8 s_199_11
        let s_199_12: u128 = s_199_8 << s_199_11;
        // D s_199_13: or s_199_12 s_199_10
        let s_199_13: u128 = ((s_199_12) | (s_199_10));
        // D s_199_14: add s_199_9 s_199_11
        let s_199_14: u16 = (s_199_9 + s_199_11);
        // D s_199_15: create-bits s_199_13 s_199_14
        let s_199_15: Bits = Bits::new(s_199_13, s_199_14);
        // D s_199_16: cast reint s_199_15 -> u8
        let s_199_16: u8 = (s_199_15.value() as u8);
        // D s_199_17: cast zx s_199_16 -> bv
        let s_199_17: Bits = Bits::new(s_199_16 as u128, 2u16);
        // C s_199_18: const #3u : u8
        let s_199_18: u8 = 3;
        // C s_199_19: cast zx s_199_18 -> bv
        let s_199_19: Bits = Bits::new(s_199_18 as u128, 2u16);
        // D s_199_20: cmp-eq s_199_17 s_199_19
        let s_199_20: bool = ((s_199_17) == (s_199_19));
        // D s_199_21: write-var gs#104892 <= s_199_20
        fn_state.gs_104892 = s_199_20;
        // N s_199_22: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #424u : u32
        let s_200_0: u32 = 424;
        // D s_200_1: read-reg s_200_0:u8
        let s_200_1: u8 = {
            let value = state.read_register::<u8>(s_200_0 as isize);
            tracer.read_register(s_200_0 as isize, value);
            value
        };
        // D s_200_2: call ELUsingAArch32(s_200_1)
        let s_200_2: bool = ELUsingAArch32(state, tracer, s_200_1);
        // D s_200_3: not s_200_2
        let s_200_3: bool = !s_200_2;
        // D s_200_4: write-var gs#104891 <= s_200_3
        fn_state.gs_104891 = s_200_3;
        // N s_200_5: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_201_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_201_1: call __IMPDEF_boolean(s_201_0)
        let s_201_1: bool = u__IMPDEF_boolean(state, tracer, s_201_0);
        // D s_201_2: write-var gs#104890 <= s_201_1
        fn_state.gs_104890 = s_201_1;
        // N s_201_3: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #() : ()
        let s_202_0: () = ();
        // S s_202_1: call EDSCR_read(s_202_0)
        let s_202_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_202_0);
        // S s_202_2: call _get_EDSCR_Type_SDD(s_202_1)
        let s_202_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_202_1);
        // S s_202_3: cast zx s_202_2 -> bv
        let s_202_3: Bits = Bits::new(s_202_2 as u128, 1u16);
        // C s_202_4: const #1u : u8
        let s_202_4: bool = true;
        // C s_202_5: cast zx s_202_4 -> bv
        let s_202_5: Bits = Bits::new(s_202_4 as u128, 1u16);
        // S s_202_6: cmp-eq s_202_3 s_202_5
        let s_202_6: bool = ((s_202_3) == (s_202_5));
        // D s_202_7: write-var gs#104889 <= s_202_6
        fn_state.gs_104889 = s_202_6;
        // N s_202_8: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #424u : u32
        let s_203_0: u32 = 424;
        // D s_203_1: read-reg s_203_0:u8
        let s_203_1: u8 = {
            let value = state.read_register::<u8>(s_203_0 as isize);
            tracer.read_register(s_203_0 as isize, value);
            value
        };
        // C s_203_2: const #2u : u8
        let s_203_2: u8 = 2;
        // D s_203_3: cmp-lt s_203_1 s_203_2
        let s_203_3: bool = ((s_203_1) < (s_203_2));
        // D s_203_4: write-var gs#104888 <= s_203_3
        fn_state.gs_104888 = s_203_3;
        // N s_203_5: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_204_0: panic
        panic!("{:?}", ());
        // N s_204_1: return
        return;
    }
}
