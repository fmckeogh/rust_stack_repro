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
use u_get_CNTHCTL_EL2_Type_ECV::*;
use u_get_CNTKCTL_Type_PL0PTEN::*;
use u_get_CNTHCTL_EL2_Type_EL1PTEN::*;
use CNTHCTL_read::*;
use u_get_SCR_EL3_Type_NS::*;
use CNTP_CVAL_NS_write::*;
use u_get_CNTHCTL_EL2_Type_EL1PCEN::*;
use u_get_CNTHCTL_EL2_Type_EL0PTEN::*;
use u_get_SCR_EL3_Type_ECVEn::*;
use R_read::*;
use EL2Enabled::*;
use Mk_CNTHP_CVAL_EL2_Type::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_CNTHCTL_Type_PL1PCEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use Mk_CNTHPS_CVAL_EL2_Type::*;
use HCR_read::*;
use IsFeatureImplemented::*;
use CNTP_CVAL_write::*;
use u_get_SCR_Type_NS::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HCR_Type_TGE::*;
use Mk_CNTP_CVAL_Type::*;
use ELUsingAArch32::*;
use CNTKCTL_read__1::*;
use PhysicalCountInt::*;
use u_get_CNTKCTL_EL1_Type_EL0PTEN::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn CNTHP_TVAL_SysRegWrite32_8e96a3fad1864250<T: Tracer>(
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
        gs_124336: bool,
        gs_124308: bool,
        u__HCR_EL2_E2H: bool,
        gs_124325: bool,
        gs_124316: bool,
        gs_124324: bool,
        gs_124340: bool,
        gs_124304: bool,
        gs_124299: bool,
        gs_124301: bool,
        gs_124332: bool,
        gs_124310: bool,
        gs_124333: bool,
        gs_124306: bool,
        gs_124309: bool,
        u__SCR_EL3_ECVEn: bool,
        u__HCR_TGE: bool,
        gs_124352: bool,
        gs_124351: bool,
        gs_124339: bool,
        gs_124317: bool,
        gs_124348: bool,
        gs_124298: bool,
        gs_124295: bool,
        gs_124329: bool,
        u__CNTHCTL_EL2_ECV: bool,
        u__PSTATE_EL: u8,
        gs_124330: bool,
        gs_124337: bool,
        u__CNTHCTL_EL2_EL1PCEN: bool,
        gs_124353: bool,
        gs_124323: bool,
        gs_124327: bool,
        gs_124341: bool,
        gs_124314: bool,
        gs_124319: bool,
        gs_124321: bool,
        gs_124307: bool,
        u__HCR_EL2_TGE: bool,
        gs_124335: bool,
        u__CNTKCTL_PL0PTEN: bool,
        u__CNTHCTL_EL2_EL0PTEN: bool,
        gs_124338: bool,
        gs_124300: bool,
        gs_124322: bool,
        gs_124302: bool,
        gs_124303: bool,
        gs_124334: bool,
        u__SCR_NS: bool,
        u__CNTHCTL_PL1PCEN: bool,
        gs_124328: bool,
        gs_124349: bool,
        gs_124326: bool,
        gs_124331: bool,
        gs_124320: bool,
        u__CNTKCTL_EL1_EL0PTEN: bool,
        gs_124305: bool,
        gs_124350: bool,
        u__CNTHCTL_EL2_EL1PTEN: bool,
        gs_124315: bool,
        gs_124318: bool,
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
        // C s_0_3: const #22056u : u32
        let s_0_3: u32 = 22056;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0PTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0PTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0PTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0PTEN = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_TGE(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_TGE <= s_0_9
        fn_state.u__HCR_EL2_TGE = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call CNTKCTL_read__1(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = CNTKCTL_read__1(state, tracer, s_0_11);
        // S s_0_13: call _get_CNTKCTL_Type_PL0PTEN(s_0_12)
        let s_0_13: bool = u_get_CNTKCTL_Type_PL0PTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTKCTL_PL0PTEN <= s_0_13
        fn_state.u__CNTKCTL_PL0PTEN = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TGE <= s_0_17
        fn_state.u__HCR_TGE = s_0_17;
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_E2H(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_E2H <= s_0_21
        fn_state.u__HCR_EL2_E2H = s_0_21;
        // C s_0_23: const #12808u : u32
        let s_0_23: u32 = 12808;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_CNTHCTL_EL2_Type_EL1PCEN(s_0_24)
        let s_0_25: bool = u_get_CNTHCTL_EL2_Type_EL1PCEN(state, tracer, s_0_24);
        // D s_0_26: write-var __CNTHCTL_EL2_EL1PCEN <= s_0_25
        fn_state.u__CNTHCTL_EL2_EL1PCEN = s_0_25;
        // C s_0_27: const #12808u : u32
        let s_0_27: u32 = 12808;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_CNTHCTL_EL2_Type_EL1PTEN(s_0_28)
        let s_0_29: bool = u_get_CNTHCTL_EL2_Type_EL1PTEN(state, tracer, s_0_28);
        // D s_0_30: write-var __CNTHCTL_EL2_EL1PTEN <= s_0_29
        fn_state.u__CNTHCTL_EL2_EL1PTEN = s_0_29;
        // C s_0_31: const #12808u : u32
        let s_0_31: u32 = 12808;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_CNTHCTL_EL2_Type_EL0PTEN(s_0_32)
        let s_0_33: bool = u_get_CNTHCTL_EL2_Type_EL0PTEN(state, tracer, s_0_32);
        // D s_0_34: write-var __CNTHCTL_EL2_EL0PTEN <= s_0_33
        fn_state.u__CNTHCTL_EL2_EL0PTEN = s_0_33;
        // C s_0_35: const #() : ()
        let s_0_35: () = ();
        // S s_0_36: call CNTHCTL_read(s_0_35)
        let s_0_36: ProductType700c18a878c5601b = CNTHCTL_read(state, tracer, s_0_35);
        // S s_0_37: call _get_CNTHCTL_Type_PL1PCEN(s_0_36)
        let s_0_37: bool = u_get_CNTHCTL_Type_PL1PCEN(state, tracer, s_0_36);
        // D s_0_38: write-var __CNTHCTL_PL1PCEN <= s_0_37
        fn_state.u__CNTHCTL_PL1PCEN = s_0_37;
        // C s_0_39: const #90704u : u32
        let s_0_39: u32 = 90704;
        // D s_0_40: read-reg s_0_39:struct
        let s_0_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_39 as isize);
            tracer.read_register(s_0_39 as isize, value);
            value
        };
        // D s_0_41: call _get_SCR_EL3_Type_ECVEn(s_0_40)
        let s_0_41: bool = u_get_SCR_EL3_Type_ECVEn(state, tracer, s_0_40);
        // D s_0_42: write-var __SCR_EL3_ECVEn <= s_0_41
        fn_state.u__SCR_EL3_ECVEn = s_0_41;
        // C s_0_43: const #12808u : u32
        let s_0_43: u32 = 12808;
        // D s_0_44: read-reg s_0_43:struct
        let s_0_44: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_43 as isize);
            tracer.read_register(s_0_43 as isize, value);
            value
        };
        // D s_0_45: call _get_CNTHCTL_EL2_Type_ECV(s_0_44)
        let s_0_45: bool = u_get_CNTHCTL_EL2_Type_ECV(state, tracer, s_0_44);
        // D s_0_46: write-var __CNTHCTL_EL2_ECV <= s_0_45
        fn_state.u__CNTHCTL_EL2_ECV = s_0_45;
        // C s_0_47: const #20920u : u32
        let s_0_47: u32 = 20920;
        // D s_0_48: read-reg s_0_47:struct
        let s_0_48: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_47 as isize);
            tracer.read_register(s_0_47 as isize, value);
            value
        };
        // D s_0_49: call _get_SCR_Type_NS(s_0_48)
        let s_0_49: bool = u_get_SCR_Type_NS(state, tracer, s_0_48);
        // D s_0_50: write-var __SCR_NS <= s_0_49
        fn_state.u__SCR_NS = s_0_49;
        // D s_0_51: read-var __PSTATE_EL:u8
        let s_0_51: u8 = fn_state.u__PSTATE_EL;
        // D s_0_52: cast zx s_0_51 -> bv
        let s_0_52: Bits = Bits::new(s_0_51 as u128, 2u16);
        // C s_0_53: const #448u : u32
        let s_0_53: u32 = 448;
        // D s_0_54: read-reg s_0_53:u8
        let s_0_54: u8 = {
            let value = state.read_register::<u8>(s_0_53 as isize);
            tracer.read_register(s_0_53 as isize, value);
            value
        };
        // D s_0_55: cast zx s_0_54 -> bv
        let s_0_55: Bits = Bits::new(s_0_54 as u128, 2u16);
        // D s_0_56: cmp-eq s_0_52 s_0_55
        let s_0_56: bool = ((s_0_52) == (s_0_55));
        // N s_0_57: branch s_0_56 b64 b1
        if s_0_56 {
            return block_64(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b14 b2
        if s_1_5 {
            return block_14(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCR_NS:u8
        let s_5_0: bool = fn_state.u__SCR_NS;
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
        // D s_6_0: read-var t:i
        let s_6_0: i128 = fn_state.t;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // C s_6_2: const #64s : i
        let s_6_2: i128 = 64;
        // D s_6_3: cast zx s_6_1 -> bv
        let s_6_3: Bits = Bits::new(s_6_1 as u128, 32u16);
        // D s_6_4: bits-cast sx s_6_3 -> bv length s_6_2
        let s_6_4: Bits = s_6_3.sign_extend(s_6_2);
        // D s_6_5: cast reint s_6_4 -> u64
        let s_6_5: u64 = (s_6_4.value() as u64);
        // C s_6_6: const #() : ()
        let s_6_6: () = ();
        // S s_6_7: call PhysicalCountInt(s_6_6)
        let s_6_7: u64 = PhysicalCountInt(state, tracer, s_6_6);
        // D s_6_8: cast zx s_6_5 -> bv
        let s_6_8: Bits = Bits::new(s_6_5 as u128, 64u16);
        // S s_6_9: cast zx s_6_7 -> bv
        let s_6_9: Bits = Bits::new(s_6_7 as u128, 64u16);
        // D s_6_10: add s_6_8 s_6_9
        let s_6_10: Bits = (s_6_8 + s_6_9);
        // D s_6_11: cast reint s_6_10 -> u64
        let s_6_11: u64 = (s_6_10.value() as u64);
        // D s_6_12: call Mk_CNTP_CVAL_Type(s_6_11)
        let s_6_12: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_6_11,
        );
        // D s_6_13: call CNTP_CVAL_NS_write(s_6_12)
        let s_6_13: () = CNTP_CVAL_NS_write(state, tracer, s_6_12);
        // N s_6_14: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var t:i
        let s_7_0: i128 = fn_state.t;
        // D s_7_1: call R_read(s_7_0)
        let s_7_1: u32 = R_read(state, tracer, s_7_0);
        // C s_7_2: const #64s : i
        let s_7_2: i128 = 64;
        // D s_7_3: cast zx s_7_1 -> bv
        let s_7_3: Bits = Bits::new(s_7_1 as u128, 32u16);
        // D s_7_4: bits-cast sx s_7_3 -> bv length s_7_2
        let s_7_4: Bits = s_7_3.sign_extend(s_7_2);
        // D s_7_5: cast reint s_7_4 -> u64
        let s_7_5: u64 = (s_7_4.value() as u64);
        // C s_7_6: const #() : ()
        let s_7_6: () = ();
        // S s_7_7: call PhysicalCountInt(s_7_6)
        let s_7_7: u64 = PhysicalCountInt(state, tracer, s_7_6);
        // D s_7_8: cast zx s_7_5 -> bv
        let s_7_8: Bits = Bits::new(s_7_5 as u128, 64u16);
        // S s_7_9: cast zx s_7_7 -> bv
        let s_7_9: Bits = Bits::new(s_7_7 as u128, 64u16);
        // D s_7_10: add s_7_8 s_7_9
        let s_7_10: Bits = (s_7_8 + s_7_9);
        // D s_7_11: cast reint s_7_10 -> u64
        let s_7_11: u64 = (s_7_10.value() as u64);
        // D s_7_12: call Mk_CNTP_CVAL_Type(s_7_11)
        let s_7_12: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_7_11,
        );
        // C s_7_13: const #16848u : u32
        let s_7_13: u32 = 16848;
        // N s_7_14: write-reg s_7_13 <= s_7_12
        let s_7_14: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_7_13 as isize, s_7_12);
            tracer.write_register(s_7_13 as isize, s_7_12);
        };
        // N s_7_15: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #424u : u32
        let s_8_0: u32 = 424;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // D s_8_3: cmp-lt s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) < (s_8_2));
        // N s_8_4: branch s_8_3 b13 b9
        if s_8_3 {
            return block_13(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#124295 <= s_9_0
        fn_state.gs_124295 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#124295:u8
        let s_10_0: bool = fn_state.gs_124295;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var t:i
        let s_11_0: i128 = fn_state.t;
        // D s_11_1: call R_read(s_11_0)
        let s_11_1: u32 = R_read(state, tracer, s_11_0);
        // C s_11_2: const #64s : i
        let s_11_2: i128 = 64;
        // D s_11_3: cast zx s_11_1 -> bv
        let s_11_3: Bits = Bits::new(s_11_1 as u128, 32u16);
        // D s_11_4: bits-cast sx s_11_3 -> bv length s_11_2
        let s_11_4: Bits = s_11_3.sign_extend(s_11_2);
        // D s_11_5: cast reint s_11_4 -> u64
        let s_11_5: u64 = (s_11_4.value() as u64);
        // C s_11_6: const #() : ()
        let s_11_6: () = ();
        // S s_11_7: call PhysicalCountInt(s_11_6)
        let s_11_7: u64 = PhysicalCountInt(state, tracer, s_11_6);
        // D s_11_8: cast zx s_11_5 -> bv
        let s_11_8: Bits = Bits::new(s_11_5 as u128, 64u16);
        // S s_11_9: cast zx s_11_7 -> bv
        let s_11_9: Bits = Bits::new(s_11_7 as u128, 64u16);
        // D s_11_10: add s_11_8 s_11_9
        let s_11_10: Bits = (s_11_8 + s_11_9);
        // D s_11_11: cast reint s_11_10 -> u64
        let s_11_11: u64 = (s_11_10.value() as u64);
        // D s_11_12: call Mk_CNTP_CVAL_Type(s_11_11)
        let s_11_12: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_11_11,
        );
        // D s_11_13: call CNTP_CVAL_write(s_11_12)
        let s_11_13: () = CNTP_CVAL_write(state, tracer, s_11_12);
        // N s_11_14: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var t:i
        let s_12_0: i128 = fn_state.t;
        // D s_12_1: call R_read(s_12_0)
        let s_12_1: u32 = R_read(state, tracer, s_12_0);
        // C s_12_2: const #64s : i
        let s_12_2: i128 = 64;
        // D s_12_3: cast zx s_12_1 -> bv
        let s_12_3: Bits = Bits::new(s_12_1 as u128, 32u16);
        // D s_12_4: bits-cast sx s_12_3 -> bv length s_12_2
        let s_12_4: Bits = s_12_3.sign_extend(s_12_2);
        // D s_12_5: cast reint s_12_4 -> u64
        let s_12_5: u64 = (s_12_4.value() as u64);
        // C s_12_6: const #() : ()
        let s_12_6: () = ();
        // S s_12_7: call PhysicalCountInt(s_12_6)
        let s_12_7: u64 = PhysicalCountInt(state, tracer, s_12_6);
        // D s_12_8: cast zx s_12_5 -> bv
        let s_12_8: Bits = Bits::new(s_12_5 as u128, 64u16);
        // S s_12_9: cast zx s_12_7 -> bv
        let s_12_9: Bits = Bits::new(s_12_7 as u128, 64u16);
        // D s_12_10: add s_12_8 s_12_9
        let s_12_10: Bits = (s_12_8 + s_12_9);
        // D s_12_11: cast reint s_12_10 -> u64
        let s_12_11: u64 = (s_12_10.value() as u64);
        // D s_12_12: call Mk_CNTP_CVAL_Type(s_12_11)
        let s_12_12: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_12_11,
        );
        // D s_12_13: call CNTP_CVAL_NS_write(s_12_12)
        let s_12_13: () = CNTP_CVAL_NS_write(state, tracer, s_12_12);
        // N s_12_14: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #424u : u32
        let s_13_0: u32 = 424;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // D s_13_3: write-var gs#124295 <= s_13_2
        fn_state.gs_124295 = s_13_2;
        // N s_13_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EL2Enabled(s_14_0)
        let s_14_1: bool = EL2Enabled(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b63 b15
        if s_14_1 {
            return block_63(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#124298 <= s_15_0
        fn_state.gs_124298 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#124298:u8
        let s_16_0: bool = fn_state.gs_124298;
        // N s_16_1: branch s_16_0 b62 b17
        if s_16_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#124299 <= s_17_0
        fn_state.gs_124299 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#124299:u8
        let s_18_0: bool = fn_state.gs_124299;
        // N s_18_1: branch s_18_0 b61 b19
        if s_18_0 {
            return block_61(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#124300 <= s_19_0
        fn_state.gs_124300 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#124300:u8
        let s_20_0: bool = fn_state.gs_124300;
        // N s_20_1: branch s_20_0 b60 b21
        if s_20_0 {
            return block_60(state, tracer, fn_state);
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
        // S s_21_1: call EL2Enabled(s_21_0)
        let s_21_1: bool = EL2Enabled(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b59 b22
        if s_21_1 {
            return block_59(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#124301 <= s_22_0
        fn_state.gs_124301 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#124301:u8
        let s_23_0: bool = fn_state.gs_124301;
        // N s_23_1: branch s_23_0 b58 b24
        if s_23_0 {
            return block_58(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#124302 <= s_24_0
        fn_state.gs_124302 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#124302:u8
        let s_25_0: bool = fn_state.gs_124302;
        // N s_25_1: branch s_25_0 b57 b26
        if s_25_0 {
            return block_57(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#124303 <= s_26_0
        fn_state.gs_124303 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#124303:u8
        let s_27_0: bool = fn_state.gs_124303;
        // N s_27_1: branch s_27_0 b56 b28
        if s_27_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EL2Enabled(s_28_0)
        let s_28_1: bool = EL2Enabled(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b55 b29
        if s_28_1 {
            return block_55(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#124304 <= s_29_0
        fn_state.gs_124304 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#124304:u8
        let s_30_0: bool = fn_state.gs_124304;
        // N s_30_1: branch s_30_0 b54 b31
        if s_30_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#124305 <= s_31_0
        fn_state.gs_124305 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#124305:u8
        let s_32_0: bool = fn_state.gs_124305;
        // N s_32_1: branch s_32_0 b53 b33
        if s_32_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #145u : u32
        let s_33_0: u32 = 145;
        // S s_33_1: call IsFeatureImplemented(s_33_0)
        let s_33_1: bool = IsFeatureImplemented(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b52 b34
        if s_33_1 {
            return block_52(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#124306 <= s_34_0
        fn_state.gs_124306 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#124306:u8
        let s_35_0: bool = fn_state.gs_124306;
        // N s_35_1: branch s_35_0 b51 b36
        if s_35_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#124307 <= s_36_0
        fn_state.gs_124307 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#124307:u8
        let s_37_0: bool = fn_state.gs_124307;
        // N s_37_1: branch s_37_0 b50 b38
        if s_37_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#124308 <= s_38_0
        fn_state.gs_124308 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#124308:u8
        let s_39_0: bool = fn_state.gs_124308;
        // N s_39_1: branch s_39_0 b49 b40
        if s_39_0 {
            return block_49(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#124309 <= s_40_0
        fn_state.gs_124309 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#124309:u8
        let s_41_0: bool = fn_state.gs_124309;
        // N s_41_1: branch s_41_0 b48 b42
        if s_41_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #424u : u32
        let s_42_0: u32 = 424;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // C s_42_2: const #2u : u8
        let s_42_2: u8 = 2;
        // D s_42_3: cmp-lt s_42_1 s_42_2
        let s_42_3: bool = ((s_42_1) < (s_42_2));
        // N s_42_4: branch s_42_3 b47 b43
        if s_42_3 {
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
        // D s_43_1: write-var gs#124310 <= s_43_0
        fn_state.gs_124310 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#124310:u8
        let s_44_0: bool = fn_state.gs_124310;
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
        // D s_45_0: read-var t:i
        let s_45_0: i128 = fn_state.t;
        // D s_45_1: call R_read(s_45_0)
        let s_45_1: u32 = R_read(state, tracer, s_45_0);
        // C s_45_2: const #64s : i
        let s_45_2: i128 = 64;
        // D s_45_3: cast zx s_45_1 -> bv
        let s_45_3: Bits = Bits::new(s_45_1 as u128, 32u16);
        // D s_45_4: bits-cast sx s_45_3 -> bv length s_45_2
        let s_45_4: Bits = s_45_3.sign_extend(s_45_2);
        // D s_45_5: cast reint s_45_4 -> u64
        let s_45_5: u64 = (s_45_4.value() as u64);
        // C s_45_6: const #() : ()
        let s_45_6: () = ();
        // S s_45_7: call PhysicalCountInt(s_45_6)
        let s_45_7: u64 = PhysicalCountInt(state, tracer, s_45_6);
        // D s_45_8: cast zx s_45_5 -> bv
        let s_45_8: Bits = Bits::new(s_45_5 as u128, 64u16);
        // S s_45_9: cast zx s_45_7 -> bv
        let s_45_9: Bits = Bits::new(s_45_7 as u128, 64u16);
        // D s_45_10: add s_45_8 s_45_9
        let s_45_10: Bits = (s_45_8 + s_45_9);
        // D s_45_11: cast reint s_45_10 -> u64
        let s_45_11: u64 = (s_45_10.value() as u64);
        // D s_45_12: call Mk_CNTP_CVAL_Type(s_45_11)
        let s_45_12: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_45_11,
        );
        // D s_45_13: call CNTP_CVAL_write(s_45_12)
        let s_45_13: () = CNTP_CVAL_write(state, tracer, s_45_12);
        // N s_45_14: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var t:i
        let s_46_0: i128 = fn_state.t;
        // D s_46_1: call R_read(s_46_0)
        let s_46_1: u32 = R_read(state, tracer, s_46_0);
        // C s_46_2: const #64s : i
        let s_46_2: i128 = 64;
        // D s_46_3: cast zx s_46_1 -> bv
        let s_46_3: Bits = Bits::new(s_46_1 as u128, 32u16);
        // D s_46_4: bits-cast sx s_46_3 -> bv length s_46_2
        let s_46_4: Bits = s_46_3.sign_extend(s_46_2);
        // D s_46_5: cast reint s_46_4 -> u64
        let s_46_5: u64 = (s_46_4.value() as u64);
        // C s_46_6: const #() : ()
        let s_46_6: () = ();
        // S s_46_7: call PhysicalCountInt(s_46_6)
        let s_46_7: u64 = PhysicalCountInt(state, tracer, s_46_6);
        // D s_46_8: cast zx s_46_5 -> bv
        let s_46_8: Bits = Bits::new(s_46_5 as u128, 64u16);
        // S s_46_9: cast zx s_46_7 -> bv
        let s_46_9: Bits = Bits::new(s_46_7 as u128, 64u16);
        // D s_46_10: add s_46_8 s_46_9
        let s_46_10: Bits = (s_46_8 + s_46_9);
        // D s_46_11: cast reint s_46_10 -> u64
        let s_46_11: u64 = (s_46_10.value() as u64);
        // D s_46_12: call Mk_CNTP_CVAL_Type(s_46_11)
        let s_46_12: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_46_11,
        );
        // D s_46_13: call CNTP_CVAL_NS_write(s_46_12)
        let s_46_13: () = CNTP_CVAL_NS_write(state, tracer, s_46_12);
        // N s_46_14: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #424u : u32
        let s_47_0: u32 = 424;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call ELUsingAArch32(s_47_1)
        let s_47_2: bool = ELUsingAArch32(state, tracer, s_47_1);
        // D s_47_3: write-var gs#124310 <= s_47_2
        fn_state.gs_124310 = s_47_2;
        // N s_47_4: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var t:i
        let s_48_0: i128 = fn_state.t;
        // D s_48_1: call R_read(s_48_0)
        let s_48_1: u32 = R_read(state, tracer, s_48_0);
        // C s_48_2: const #64s : i
        let s_48_2: i128 = 64;
        // D s_48_3: cast zx s_48_1 -> bv
        let s_48_3: Bits = Bits::new(s_48_1 as u128, 32u16);
        // D s_48_4: bits-cast sx s_48_3 -> bv length s_48_2
        let s_48_4: Bits = s_48_3.sign_extend(s_48_2);
        // D s_48_5: cast reint s_48_4 -> u64
        let s_48_5: u64 = (s_48_4.value() as u64);
        // C s_48_6: const #() : ()
        let s_48_6: () = ();
        // S s_48_7: call PhysicalCountInt(s_48_6)
        let s_48_7: u64 = PhysicalCountInt(state, tracer, s_48_6);
        // D s_48_8: cast zx s_48_5 -> bv
        let s_48_8: Bits = Bits::new(s_48_5 as u128, 64u16);
        // S s_48_9: cast zx s_48_7 -> bv
        let s_48_9: Bits = Bits::new(s_48_7 as u128, 64u16);
        // D s_48_10: add s_48_8 s_48_9
        let s_48_10: Bits = (s_48_8 + s_48_9);
        // D s_48_11: cast reint s_48_10 -> u64
        let s_48_11: u64 = (s_48_10.value() as u64);
        // D s_48_12: cast zx s_48_11 -> bv
        let s_48_12: Bits = Bits::new(s_48_11 as u128, 64u16);
        // C s_48_13: const #14584u : u32
        let s_48_13: u32 = 14584;
        // D s_48_14: read-reg s_48_13:u64
        let s_48_14: u64 = {
            let value = state.read_register::<u64>(s_48_13 as isize);
            tracer.read_register(s_48_13 as isize, value);
            value
        };
        // D s_48_15: cast zx s_48_14 -> bv
        let s_48_15: Bits = Bits::new(s_48_14 as u128, 64u16);
        // D s_48_16: sub s_48_12 s_48_15
        let s_48_16: Bits = ((s_48_12) - (s_48_15));
        // D s_48_17: cast reint s_48_16 -> u64
        let s_48_17: u64 = (s_48_16.value() as u64);
        // D s_48_18: call Mk_CNTP_CVAL_Type(s_48_17)
        let s_48_18: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_48_17,
        );
        // D s_48_19: call CNTP_CVAL_write(s_48_18)
        let s_48_19: () = CNTP_CVAL_write(state, tracer, s_48_18);
        // N s_48_20: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_49_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #1u : u8
        let s_49_2: bool = true;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: write-var gs#124309 <= s_49_4
        fn_state.gs_124309 = s_49_4;
        // N s_49_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var __SCR_EL3_ECVEn:u8
        let s_50_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 1u16);
        // C s_50_2: const #1u : u8
        let s_50_2: bool = true;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_4: cmp-eq s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) == (s_50_3));
        // D s_50_5: write-var gs#124308 <= s_50_4
        fn_state.gs_124308 = s_50_4;
        // N s_50_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #432u : u32
        let s_51_0: u32 = 432;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call ELUsingAArch32(s_51_1)
        let s_51_2: bool = ELUsingAArch32(state, tracer, s_51_1);
        // D s_51_3: not s_51_2
        let s_51_3: bool = !s_51_2;
        // D s_51_4: write-var gs#124307 <= s_51_3
        fn_state.gs_124307 = s_51_3;
        // N s_51_5: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EL2Enabled(s_52_0)
        let s_52_1: bool = EL2Enabled(state, tracer, s_52_0);
        // D s_52_2: write-var gs#124306 <= s_52_1
        fn_state.gs_124306 = s_52_1;
        // N s_52_3: jump b35
        return block_35(state, tracer, fn_state);
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
        // S s_53_5: call AArch32_TakeHypTrapException(s_53_4)
        let s_53_5: () = AArch32_TakeHypTrapException(state, tracer, s_53_4);
        // N s_53_6: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var __CNTHCTL_PL1PCEN:u8
        let s_54_0: bool = fn_state.u__CNTHCTL_PL1PCEN;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 1u16);
        // C s_54_2: const #0u : u8
        let s_54_2: bool = false;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // D s_54_5: write-var gs#124305 <= s_54_4
        fn_state.gs_124305 = s_54_4;
        // N s_54_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #432u : u32
        let s_55_0: u32 = 432;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call ELUsingAArch32(s_55_1)
        let s_55_2: bool = ELUsingAArch32(state, tracer, s_55_1);
        // D s_55_3: write-var gs#124304 <= s_55_2
        fn_state.gs_124304 = s_55_2;
        // N s_55_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #3u : u8
        let s_56_0: u8 = 3;
        // C s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 8u16);
        // C s_56_2: cast zx s_56_1 -> i
        let s_56_2: i128 = (s_56_1.value() as i128);
        // C s_56_3: cast reint s_56_2 -> i64
        let s_56_3: i64 = (s_56_2 as i64);
        // C s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // C s_56_5: const #432u : u32
        let s_56_5: u32 = 432;
        // D s_56_6: read-reg s_56_5:u8
        let s_56_6: u8 = {
            let value = state.read_register::<u8>(s_56_5 as isize);
            tracer.read_register(s_56_5 as isize, value);
            value
        };
        // D s_56_7: call AArch64_AArch32SystemAccessTrap(s_56_6, s_56_4)
        let s_56_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_56_6, s_56_4);
        // N s_56_8: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_57_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 1u16);
        // C s_57_2: const #0u : u8
        let s_57_2: bool = false;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // D s_57_4: cmp-eq s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) == (s_57_3));
        // D s_57_5: write-var gs#124303 <= s_57_4
        fn_state.gs_124303 = s_57_4;
        // N s_57_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var __HCR_EL2_E2H:u8
        let s_58_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 1u16);
        // C s_58_2: const #1u : u8
        let s_58_2: bool = true;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: write-var gs#124302 <= s_58_4
        fn_state.gs_124302 = s_58_4;
        // N s_58_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #432u : u32
        let s_59_0: u32 = 432;
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
        // D s_59_4: write-var gs#124301 <= s_59_3
        fn_state.gs_124301 = s_59_3;
        // N s_59_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #3u : u8
        let s_60_0: u8 = 3;
        // C s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 8u16);
        // C s_60_2: cast zx s_60_1 -> i
        let s_60_2: i128 = (s_60_1.value() as i128);
        // C s_60_3: cast reint s_60_2 -> i64
        let s_60_3: i64 = (s_60_2 as i64);
        // C s_60_4: cast zx s_60_3 -> i
        let s_60_4: i128 = (i128::try_from(s_60_3).unwrap());
        // C s_60_5: const #432u : u32
        let s_60_5: u32 = 432;
        // D s_60_6: read-reg s_60_5:u8
        let s_60_6: u8 = {
            let value = state.read_register::<u8>(s_60_5 as isize);
            tracer.read_register(s_60_5 as isize, value);
            value
        };
        // D s_60_7: call AArch64_AArch32SystemAccessTrap(s_60_6, s_60_4)
        let s_60_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_60_6, s_60_4);
        // N s_60_8: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_61_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 1u16);
        // C s_61_2: const #0u : u8
        let s_61_2: bool = false;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: write-var gs#124300 <= s_61_4
        fn_state.gs_124300 = s_61_4;
        // N s_61_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __HCR_EL2_E2H:u8
        let s_62_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #0u : u8
        let s_62_2: bool = false;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#124299 <= s_62_4
        fn_state.gs_124299 = s_62_4;
        // N s_62_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #432u : u32
        let s_63_0: u32 = 432;
        // D s_63_1: read-reg s_63_0:u8
        let s_63_1: u8 = {
            let value = state.read_register::<u8>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call ELUsingAArch32(s_63_1)
        let s_63_2: bool = ELUsingAArch32(state, tracer, s_63_1);
        // D s_63_3: not s_63_2
        let s_63_3: bool = !s_63_2;
        // D s_63_4: write-var gs#124298 <= s_63_3
        fn_state.gs_124298 = s_63_3;
        // N s_63_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #440u : u32
        let s_64_0: u32 = 440;
        // D s_64_1: read-reg s_64_0:u8
        let s_64_1: u8 = {
            let value = state.read_register::<u8>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call ELUsingAArch32(s_64_1)
        let s_64_2: bool = ELUsingAArch32(state, tracer, s_64_1);
        // D s_64_3: not s_64_2
        let s_64_3: bool = !s_64_2;
        // N s_64_4: branch s_64_3 b191 b65
        if s_64_3 {
            return block_191(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#124315 <= s_65_0
        fn_state.gs_124315 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#124315:u8
        let s_66_0: bool = fn_state.gs_124315;
        // N s_66_1: branch s_66_0 b190 b67
        if s_66_0 {
            return block_190(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#124316 <= s_67_0
        fn_state.gs_124316 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#124316:u8
        let s_68_0: bool = fn_state.gs_124316;
        // N s_68_1: branch s_68_0 b181 b69
        if s_68_0 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #440u : u32
        let s_69_0: u32 = 440;
        // D s_69_1: read-reg s_69_0:u8
        let s_69_1: u8 = {
            let value = state.read_register::<u8>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call ELUsingAArch32(s_69_1)
        let s_69_2: bool = ELUsingAArch32(state, tracer, s_69_1);
        // N s_69_3: branch s_69_2 b180 b70
        if s_69_2 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#124317 <= s_70_0
        fn_state.gs_124317 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#124317:u8
        let s_71_0: bool = fn_state.gs_124317;
        // N s_71_1: branch s_71_0 b163 b72
        if s_71_0 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call EL2Enabled(s_72_0)
        let s_72_1: bool = EL2Enabled(state, tracer, s_72_0);
        // N s_72_2: branch s_72_1 b162 b73
        if s_72_1 {
            return block_162(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#124318 <= s_73_0
        fn_state.gs_124318 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#124318:u8
        let s_74_0: bool = fn_state.gs_124318;
        // N s_74_1: branch s_74_0 b161 b75
        if s_74_0 {
            return block_161(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#124319 <= s_75_0
        fn_state.gs_124319 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#124319:u8
        let s_76_0: bool = fn_state.gs_124319;
        // N s_76_1: branch s_76_0 b160 b77
        if s_76_0 {
            return block_160(state, tracer, fn_state);
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
        // D s_77_1: write-var gs#124320 <= s_77_0
        fn_state.gs_124320 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#124320:u8
        let s_78_0: bool = fn_state.gs_124320;
        // N s_78_1: branch s_78_0 b159 b79
        if s_78_0 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call EL2Enabled(s_79_0)
        let s_79_1: bool = EL2Enabled(state, tracer, s_79_0);
        // N s_79_2: branch s_79_1 b158 b80
        if s_79_1 {
            return block_158(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#124321 <= s_80_0
        fn_state.gs_124321 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#124321:u8
        let s_81_0: bool = fn_state.gs_124321;
        // N s_81_1: branch s_81_0 b157 b82
        if s_81_0 {
            return block_157(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#124322 <= s_82_0
        fn_state.gs_124322 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#124322:u8
        let s_83_0: bool = fn_state.gs_124322;
        // N s_83_1: branch s_83_0 b156 b84
        if s_83_0 {
            return block_156(state, tracer, fn_state);
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
        // D s_84_1: write-var gs#124323 <= s_84_0
        fn_state.gs_124323 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#124323:u8
        let s_85_0: bool = fn_state.gs_124323;
        // N s_85_1: branch s_85_0 b155 b86
        if s_85_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call EL2Enabled(s_86_0)
        let s_86_1: bool = EL2Enabled(state, tracer, s_86_0);
        // N s_86_2: branch s_86_1 b154 b87
        if s_86_1 {
            return block_154(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#124324 <= s_87_0
        fn_state.gs_124324 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#124324:u8
        let s_88_0: bool = fn_state.gs_124324;
        // N s_88_1: branch s_88_0 b153 b89
        if s_88_0 {
            return block_153(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#124325 <= s_89_0
        fn_state.gs_124325 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#124325:u8
        let s_90_0: bool = fn_state.gs_124325;
        // N s_90_1: branch s_90_0 b152 b91
        if s_90_0 {
            return block_152(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#124326 <= s_91_0
        fn_state.gs_124326 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#124326:u8
        let s_92_0: bool = fn_state.gs_124326;
        // N s_92_1: branch s_92_0 b151 b93
        if s_92_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #() : ()
        let s_93_0: () = ();
        // S s_93_1: call EL2Enabled(s_93_0)
        let s_93_1: bool = EL2Enabled(state, tracer, s_93_0);
        // N s_93_2: branch s_93_1 b150 b94
        if s_93_1 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#124327 <= s_94_0
        fn_state.gs_124327 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#124327:u8
        let s_95_0: bool = fn_state.gs_124327;
        // N s_95_1: branch s_95_0 b149 b96
        if s_95_0 {
            return block_149(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#124328 <= s_96_0
        fn_state.gs_124328 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#124328:u8
        let s_97_0: bool = fn_state.gs_124328;
        // N s_97_1: branch s_97_0 b148 b98
        if s_97_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call EL2Enabled(s_98_0)
        let s_98_1: bool = EL2Enabled(state, tracer, s_98_0);
        // N s_98_2: branch s_98_1 b147 b99
        if s_98_1 {
            return block_147(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#124329 <= s_99_0
        fn_state.gs_124329 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#124329:u8
        let s_100_0: bool = fn_state.gs_124329;
        // N s_100_1: branch s_100_0 b146 b101
        if s_100_0 {
            return block_146(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#124330 <= s_101_0
        fn_state.gs_124330 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#124330:u8
        let s_102_0: bool = fn_state.gs_124330;
        // N s_102_1: branch s_102_0 b145 b103
        if s_102_0 {
            return block_145(state, tracer, fn_state);
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
        // D s_103_1: write-var gs#124331 <= s_103_0
        fn_state.gs_124331 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#124331:u8
        let s_104_0: bool = fn_state.gs_124331;
        // N s_104_1: branch s_104_0 b144 b105
        if s_104_0 {
            return block_144(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#124332 <= s_105_0
        fn_state.gs_124332 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#124332:u8
        let s_106_0: bool = fn_state.gs_124332;
        // N s_106_1: branch s_106_0 b143 b107
        if s_106_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #() : ()
        let s_107_0: () = ();
        // S s_107_1: call EL2Enabled(s_107_0)
        let s_107_1: bool = EL2Enabled(state, tracer, s_107_0);
        // N s_107_2: branch s_107_1 b142 b108
        if s_107_1 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #0u : u8
        let s_108_0: bool = false;
        // D s_108_1: write-var gs#124333 <= s_108_0
        fn_state.gs_124333 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#124333:u8
        let s_109_0: bool = fn_state.gs_124333;
        // N s_109_1: branch s_109_0 b141 b110
        if s_109_0 {
            return block_141(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#124334 <= s_110_0
        fn_state.gs_124334 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#124334:u8
        let s_111_0: bool = fn_state.gs_124334;
        // N s_111_1: branch s_111_0 b140 b112
        if s_111_0 {
            return block_140(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#124335 <= s_112_0
        fn_state.gs_124335 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#124335:u8
        let s_113_0: bool = fn_state.gs_124335;
        // N s_113_1: branch s_113_0 b139 b114
        if s_113_0 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #145u : u32
        let s_114_0: u32 = 145;
        // S s_114_1: call IsFeatureImplemented(s_114_0)
        let s_114_1: bool = IsFeatureImplemented(state, tracer, s_114_0);
        // N s_114_2: branch s_114_1 b138 b115
        if s_114_1 {
            return block_138(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#124336 <= s_115_0
        fn_state.gs_124336 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#124336:u8
        let s_116_0: bool = fn_state.gs_124336;
        // N s_116_1: branch s_116_0 b137 b117
        if s_116_0 {
            return block_137(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#124337 <= s_117_0
        fn_state.gs_124337 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#124337:u8
        let s_118_0: bool = fn_state.gs_124337;
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
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#124338 <= s_119_0
        fn_state.gs_124338 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#124338:u8
        let s_120_0: bool = fn_state.gs_124338;
        // N s_120_1: branch s_120_0 b135 b121
        if s_120_0 {
            return block_135(state, tracer, fn_state);
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
        // D s_121_1: write-var gs#124339 <= s_121_0
        fn_state.gs_124339 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#124339:u8
        let s_122_0: bool = fn_state.gs_124339;
        // N s_122_1: branch s_122_0 b134 b123
        if s_122_0 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #0u : u8
        let s_123_0: bool = false;
        // D s_123_1: write-var gs#124340 <= s_123_0
        fn_state.gs_124340 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#124340:u8
        let s_124_0: bool = fn_state.gs_124340;
        // N s_124_1: branch s_124_0 b133 b125
        if s_124_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #424u : u32
        let s_125_0: u32 = 424;
        // D s_125_1: read-reg s_125_0:u8
        let s_125_1: u8 = {
            let value = state.read_register::<u8>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // C s_125_2: const #2u : u8
        let s_125_2: u8 = 2;
        // D s_125_3: cmp-lt s_125_1 s_125_2
        let s_125_3: bool = ((s_125_1) < (s_125_2));
        // N s_125_4: branch s_125_3 b132 b126
        if s_125_3 {
            return block_132(state, tracer, fn_state);
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
        // D s_126_1: write-var gs#124341 <= s_126_0
        fn_state.gs_124341 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#124341:u8
        let s_127_0: bool = fn_state.gs_124341;
        // N s_127_1: branch s_127_0 b129 b128
        if s_127_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var t:i
        let s_128_0: i128 = fn_state.t;
        // D s_128_1: call R_read(s_128_0)
        let s_128_1: u32 = R_read(state, tracer, s_128_0);
        // C s_128_2: const #64s : i
        let s_128_2: i128 = 64;
        // D s_128_3: cast zx s_128_1 -> bv
        let s_128_3: Bits = Bits::new(s_128_1 as u128, 32u16);
        // D s_128_4: bits-cast sx s_128_3 -> bv length s_128_2
        let s_128_4: Bits = s_128_3.sign_extend(s_128_2);
        // D s_128_5: cast reint s_128_4 -> u64
        let s_128_5: u64 = (s_128_4.value() as u64);
        // C s_128_6: const #() : ()
        let s_128_6: () = ();
        // S s_128_7: call PhysicalCountInt(s_128_6)
        let s_128_7: u64 = PhysicalCountInt(state, tracer, s_128_6);
        // D s_128_8: cast zx s_128_5 -> bv
        let s_128_8: Bits = Bits::new(s_128_5 as u128, 64u16);
        // S s_128_9: cast zx s_128_7 -> bv
        let s_128_9: Bits = Bits::new(s_128_7 as u128, 64u16);
        // D s_128_10: add s_128_8 s_128_9
        let s_128_10: Bits = (s_128_8 + s_128_9);
        // D s_128_11: cast reint s_128_10 -> u64
        let s_128_11: u64 = (s_128_10.value() as u64);
        // D s_128_12: call Mk_CNTP_CVAL_Type(s_128_11)
        let s_128_12: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_128_11,
        );
        // D s_128_13: call CNTP_CVAL_write(s_128_12)
        let s_128_13: () = CNTP_CVAL_write(state, tracer, s_128_12);
        // N s_128_14: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var __SCR_NS:u8
        let s_129_0: bool = fn_state.u__SCR_NS;
        // D s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 1u16);
        // C s_129_2: const #1u : u8
        let s_129_2: bool = true;
        // C s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 1u16);
        // D s_129_4: cmp-eq s_129_1 s_129_3
        let s_129_4: bool = ((s_129_1) == (s_129_3));
        // N s_129_5: branch s_129_4 b131 b130
        if s_129_4 {
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
        // D s_130_0: read-var t:i
        let s_130_0: i128 = fn_state.t;
        // D s_130_1: call R_read(s_130_0)
        let s_130_1: u32 = R_read(state, tracer, s_130_0);
        // C s_130_2: const #64s : i
        let s_130_2: i128 = 64;
        // D s_130_3: cast zx s_130_1 -> bv
        let s_130_3: Bits = Bits::new(s_130_1 as u128, 32u16);
        // D s_130_4: bits-cast sx s_130_3 -> bv length s_130_2
        let s_130_4: Bits = s_130_3.sign_extend(s_130_2);
        // D s_130_5: cast reint s_130_4 -> u64
        let s_130_5: u64 = (s_130_4.value() as u64);
        // C s_130_6: const #() : ()
        let s_130_6: () = ();
        // S s_130_7: call PhysicalCountInt(s_130_6)
        let s_130_7: u64 = PhysicalCountInt(state, tracer, s_130_6);
        // D s_130_8: cast zx s_130_5 -> bv
        let s_130_8: Bits = Bits::new(s_130_5 as u128, 64u16);
        // S s_130_9: cast zx s_130_7 -> bv
        let s_130_9: Bits = Bits::new(s_130_7 as u128, 64u16);
        // D s_130_10: add s_130_8 s_130_9
        let s_130_10: Bits = (s_130_8 + s_130_9);
        // D s_130_11: cast reint s_130_10 -> u64
        let s_130_11: u64 = (s_130_10.value() as u64);
        // D s_130_12: call Mk_CNTP_CVAL_Type(s_130_11)
        let s_130_12: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_130_11,
        );
        // C s_130_13: const #16848u : u32
        let s_130_13: u32 = 16848;
        // N s_130_14: write-reg s_130_13 <= s_130_12
        let s_130_14: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_130_13 as isize, s_130_12);
            tracer.write_register(s_130_13 as isize, s_130_12);
        };
        // N s_130_15: return
        return;
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var t:i
        let s_131_0: i128 = fn_state.t;
        // D s_131_1: call R_read(s_131_0)
        let s_131_1: u32 = R_read(state, tracer, s_131_0);
        // C s_131_2: const #64s : i
        let s_131_2: i128 = 64;
        // D s_131_3: cast zx s_131_1 -> bv
        let s_131_3: Bits = Bits::new(s_131_1 as u128, 32u16);
        // D s_131_4: bits-cast sx s_131_3 -> bv length s_131_2
        let s_131_4: Bits = s_131_3.sign_extend(s_131_2);
        // D s_131_5: cast reint s_131_4 -> u64
        let s_131_5: u64 = (s_131_4.value() as u64);
        // C s_131_6: const #() : ()
        let s_131_6: () = ();
        // S s_131_7: call PhysicalCountInt(s_131_6)
        let s_131_7: u64 = PhysicalCountInt(state, tracer, s_131_6);
        // D s_131_8: cast zx s_131_5 -> bv
        let s_131_8: Bits = Bits::new(s_131_5 as u128, 64u16);
        // S s_131_9: cast zx s_131_7 -> bv
        let s_131_9: Bits = Bits::new(s_131_7 as u128, 64u16);
        // D s_131_10: add s_131_8 s_131_9
        let s_131_10: Bits = (s_131_8 + s_131_9);
        // D s_131_11: cast reint s_131_10 -> u64
        let s_131_11: u64 = (s_131_10.value() as u64);
        // D s_131_12: call Mk_CNTP_CVAL_Type(s_131_11)
        let s_131_12: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_131_11,
        );
        // D s_131_13: call CNTP_CVAL_NS_write(s_131_12)
        let s_131_13: () = CNTP_CVAL_NS_write(state, tracer, s_131_12);
        // N s_131_14: return
        return;
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
        // D s_132_2: call ELUsingAArch32(s_132_1)
        let s_132_2: bool = ELUsingAArch32(state, tracer, s_132_1);
        // D s_132_3: write-var gs#124341 <= s_132_2
        fn_state.gs_124341 = s_132_2;
        // N s_132_4: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var t:i
        let s_133_0: i128 = fn_state.t;
        // D s_133_1: call R_read(s_133_0)
        let s_133_1: u32 = R_read(state, tracer, s_133_0);
        // C s_133_2: const #64s : i
        let s_133_2: i128 = 64;
        // D s_133_3: cast zx s_133_1 -> bv
        let s_133_3: Bits = Bits::new(s_133_1 as u128, 32u16);
        // D s_133_4: bits-cast sx s_133_3 -> bv length s_133_2
        let s_133_4: Bits = s_133_3.sign_extend(s_133_2);
        // D s_133_5: cast reint s_133_4 -> u64
        let s_133_5: u64 = (s_133_4.value() as u64);
        // C s_133_6: const #() : ()
        let s_133_6: () = ();
        // S s_133_7: call PhysicalCountInt(s_133_6)
        let s_133_7: u64 = PhysicalCountInt(state, tracer, s_133_6);
        // D s_133_8: cast zx s_133_5 -> bv
        let s_133_8: Bits = Bits::new(s_133_5 as u128, 64u16);
        // S s_133_9: cast zx s_133_7 -> bv
        let s_133_9: Bits = Bits::new(s_133_7 as u128, 64u16);
        // D s_133_10: add s_133_8 s_133_9
        let s_133_10: Bits = (s_133_8 + s_133_9);
        // D s_133_11: cast reint s_133_10 -> u64
        let s_133_11: u64 = (s_133_10.value() as u64);
        // D s_133_12: cast zx s_133_11 -> bv
        let s_133_12: Bits = Bits::new(s_133_11 as u128, 64u16);
        // C s_133_13: const #14584u : u32
        let s_133_13: u32 = 14584;
        // D s_133_14: read-reg s_133_13:u64
        let s_133_14: u64 = {
            let value = state.read_register::<u64>(s_133_13 as isize);
            tracer.read_register(s_133_13 as isize, value);
            value
        };
        // D s_133_15: cast zx s_133_14 -> bv
        let s_133_15: Bits = Bits::new(s_133_14 as u128, 64u16);
        // D s_133_16: sub s_133_12 s_133_15
        let s_133_16: Bits = ((s_133_12) - (s_133_15));
        // D s_133_17: cast reint s_133_16 -> u64
        let s_133_17: u64 = (s_133_16.value() as u64);
        // D s_133_18: call Mk_CNTP_CVAL_Type(s_133_17)
        let s_133_18: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_Type(
            state,
            tracer,
            s_133_17,
        );
        // D s_133_19: call CNTP_CVAL_write(s_133_18)
        let s_133_19: () = CNTP_CVAL_write(state, tracer, s_133_18);
        // N s_133_20: return
        return;
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #102552u : u32
        let s_134_0: u32 = 102552;
        // D s_134_1: read-reg s_134_0:struct
        let s_134_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_134_0 as isize);
            tracer.read_register(s_134_0 as isize, value);
            value
        };
        // D s_134_2: call _get_HCR_EL2_Type_E2H(s_134_1)
        let s_134_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_134_1);
        // C s_134_3: const #102552u : u32
        let s_134_3: u32 = 102552;
        // D s_134_4: read-reg s_134_3:struct
        let s_134_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_134_3 as isize);
            tracer.read_register(s_134_3 as isize, value);
            value
        };
        // D s_134_5: call _get_HCR_EL2_Type_TGE(s_134_4)
        let s_134_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_134_4);
        // D s_134_6: cast zx s_134_2 -> bv
        let s_134_6: Bits = Bits::new(s_134_2 as u128, 1u16);
        // D s_134_7: cast zx s_134_5 -> bv
        let s_134_7: Bits = Bits::new(s_134_5 as u128, 1u16);
        // D s_134_8: cast reint s_134_6 -> u128
        let s_134_8: u128 = (s_134_6.value() as u128);
        // D s_134_9: size-of s_134_6
        let s_134_9: u16 = s_134_6.length();
        // D s_134_10: cast reint s_134_7 -> u128
        let s_134_10: u128 = (s_134_7.value() as u128);
        // D s_134_11: size-of s_134_7
        let s_134_11: u16 = s_134_7.length();
        // D s_134_12: lsl s_134_8 s_134_11
        let s_134_12: u128 = s_134_8 << s_134_11;
        // D s_134_13: or s_134_12 s_134_10
        let s_134_13: u128 = ((s_134_12) | (s_134_10));
        // D s_134_14: add s_134_9 s_134_11
        let s_134_14: u16 = (s_134_9 + s_134_11);
        // D s_134_15: create-bits s_134_13 s_134_14
        let s_134_15: Bits = Bits::new(s_134_13, s_134_14);
        // D s_134_16: cast reint s_134_15 -> u8
        let s_134_16: u8 = (s_134_15.value() as u8);
        // D s_134_17: cast zx s_134_16 -> bv
        let s_134_17: Bits = Bits::new(s_134_16 as u128, 2u16);
        // C s_134_18: const #3u : u8
        let s_134_18: u8 = 3;
        // C s_134_19: cast zx s_134_18 -> bv
        let s_134_19: Bits = Bits::new(s_134_18 as u128, 2u16);
        // D s_134_20: cmp-ne s_134_17 s_134_19
        let s_134_20: bool = ((s_134_17) != (s_134_19));
        // D s_134_21: write-var gs#124340 <= s_134_20
        fn_state.gs_124340 = s_134_20;
        // N s_134_22: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_135_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_135_1: cast zx s_135_0 -> bv
        let s_135_1: Bits = Bits::new(s_135_0 as u128, 1u16);
        // C s_135_2: const #1u : u8
        let s_135_2: bool = true;
        // C s_135_3: cast zx s_135_2 -> bv
        let s_135_3: Bits = Bits::new(s_135_2 as u128, 1u16);
        // D s_135_4: cmp-eq s_135_1 s_135_3
        let s_135_4: bool = ((s_135_1) == (s_135_3));
        // D s_135_5: write-var gs#124339 <= s_135_4
        fn_state.gs_124339 = s_135_4;
        // N s_135_6: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var __SCR_EL3_ECVEn:u8
        let s_136_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 1u16);
        // C s_136_2: const #1u : u8
        let s_136_2: bool = true;
        // C s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 1u16);
        // D s_136_4: cmp-eq s_136_1 s_136_3
        let s_136_4: bool = ((s_136_1) == (s_136_3));
        // D s_136_5: write-var gs#124338 <= s_136_4
        fn_state.gs_124338 = s_136_4;
        // N s_136_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #432u : u32
        let s_137_0: u32 = 432;
        // D s_137_1: read-reg s_137_0:u8
        let s_137_1: u8 = {
            let value = state.read_register::<u8>(s_137_0 as isize);
            tracer.read_register(s_137_0 as isize, value);
            value
        };
        // D s_137_2: call ELUsingAArch32(s_137_1)
        let s_137_2: bool = ELUsingAArch32(state, tracer, s_137_1);
        // D s_137_3: not s_137_2
        let s_137_3: bool = !s_137_2;
        // D s_137_4: write-var gs#124337 <= s_137_3
        fn_state.gs_124337 = s_137_3;
        // N s_137_5: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #() : ()
        let s_138_0: () = ();
        // S s_138_1: call EL2Enabled(s_138_0)
        let s_138_1: bool = EL2Enabled(state, tracer, s_138_0);
        // D s_138_2: write-var gs#124336 <= s_138_1
        fn_state.gs_124336 = s_138_1;
        // N s_138_3: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var t:i
        let s_139_0: i128 = fn_state.t;
        // D s_139_1: call R_read(s_139_0)
        let s_139_1: u32 = R_read(state, tracer, s_139_0);
        // C s_139_2: const #64s : i
        let s_139_2: i128 = 64;
        // D s_139_3: cast zx s_139_1 -> bv
        let s_139_3: Bits = Bits::new(s_139_1 as u128, 32u16);
        // D s_139_4: bits-cast sx s_139_3 -> bv length s_139_2
        let s_139_4: Bits = s_139_3.sign_extend(s_139_2);
        // D s_139_5: cast reint s_139_4 -> u64
        let s_139_5: u64 = (s_139_4.value() as u64);
        // C s_139_6: const #() : ()
        let s_139_6: () = ();
        // S s_139_7: call PhysicalCountInt(s_139_6)
        let s_139_7: u64 = PhysicalCountInt(state, tracer, s_139_6);
        // D s_139_8: cast zx s_139_5 -> bv
        let s_139_8: Bits = Bits::new(s_139_5 as u128, 64u16);
        // S s_139_9: cast zx s_139_7 -> bv
        let s_139_9: Bits = Bits::new(s_139_7 as u128, 64u16);
        // D s_139_10: add s_139_8 s_139_9
        let s_139_10: Bits = (s_139_8 + s_139_9);
        // D s_139_11: cast reint s_139_10 -> u64
        let s_139_11: u64 = (s_139_10.value() as u64);
        // D s_139_12: call Mk_CNTHP_CVAL_EL2_Type(s_139_11)
        let s_139_12: ProductType5c790c8ef59cc8b2 = Mk_CNTHP_CVAL_EL2_Type(
            state,
            tracer,
            s_139_11,
        );
        // C s_139_13: const #16640u : u32
        let s_139_13: u32 = 16640;
        // N s_139_14: write-reg s_139_13 <= s_139_12
        let s_139_14: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_139_13 as isize, s_139_12);
            tracer.write_register(s_139_13 as isize, s_139_12);
        };
        // N s_139_15: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #90704u : u32
        let s_140_0: u32 = 90704;
        // D s_140_1: read-reg s_140_0:struct
        let s_140_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_140_0 as isize);
            tracer.read_register(s_140_0 as isize, value);
            value
        };
        // D s_140_2: call _get_SCR_EL3_Type_NS(s_140_1)
        let s_140_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_140_1);
        // D s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 1u16);
        // C s_140_4: const #1u : u8
        let s_140_4: bool = true;
        // C s_140_5: cast zx s_140_4 -> bv
        let s_140_5: Bits = Bits::new(s_140_4 as u128, 1u16);
        // D s_140_6: cmp-eq s_140_3 s_140_5
        let s_140_6: bool = ((s_140_3) == (s_140_5));
        // D s_140_7: write-var gs#124335 <= s_140_6
        fn_state.gs_124335 = s_140_6;
        // N s_140_8: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #102552u : u32
        let s_141_0: u32 = 102552;
        // D s_141_1: read-reg s_141_0:struct
        let s_141_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_141_0 as isize);
            tracer.read_register(s_141_0 as isize, value);
            value
        };
        // D s_141_2: call _get_HCR_EL2_Type_E2H(s_141_1)
        let s_141_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_141_1);
        // C s_141_3: const #102552u : u32
        let s_141_3: u32 = 102552;
        // D s_141_4: read-reg s_141_3:struct
        let s_141_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_141_3 as isize);
            tracer.read_register(s_141_3 as isize, value);
            value
        };
        // D s_141_5: call _get_HCR_EL2_Type_TGE(s_141_4)
        let s_141_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_141_4);
        // D s_141_6: cast zx s_141_2 -> bv
        let s_141_6: Bits = Bits::new(s_141_2 as u128, 1u16);
        // D s_141_7: cast zx s_141_5 -> bv
        let s_141_7: Bits = Bits::new(s_141_5 as u128, 1u16);
        // D s_141_8: cast reint s_141_6 -> u128
        let s_141_8: u128 = (s_141_6.value() as u128);
        // D s_141_9: size-of s_141_6
        let s_141_9: u16 = s_141_6.length();
        // D s_141_10: cast reint s_141_7 -> u128
        let s_141_10: u128 = (s_141_7.value() as u128);
        // D s_141_11: size-of s_141_7
        let s_141_11: u16 = s_141_7.length();
        // D s_141_12: lsl s_141_8 s_141_11
        let s_141_12: u128 = s_141_8 << s_141_11;
        // D s_141_13: or s_141_12 s_141_10
        let s_141_13: u128 = ((s_141_12) | (s_141_10));
        // D s_141_14: add s_141_9 s_141_11
        let s_141_14: u16 = (s_141_9 + s_141_11);
        // D s_141_15: create-bits s_141_13 s_141_14
        let s_141_15: Bits = Bits::new(s_141_13, s_141_14);
        // D s_141_16: cast reint s_141_15 -> u8
        let s_141_16: u8 = (s_141_15.value() as u8);
        // D s_141_17: cast zx s_141_16 -> bv
        let s_141_17: Bits = Bits::new(s_141_16 as u128, 2u16);
        // C s_141_18: const #3u : u8
        let s_141_18: u8 = 3;
        // C s_141_19: cast zx s_141_18 -> bv
        let s_141_19: Bits = Bits::new(s_141_18 as u128, 2u16);
        // D s_141_20: cmp-eq s_141_17 s_141_19
        let s_141_20: bool = ((s_141_17) == (s_141_19));
        // D s_141_21: write-var gs#124334 <= s_141_20
        fn_state.gs_124334 = s_141_20;
        // N s_141_22: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #432u : u32
        let s_142_0: u32 = 432;
        // D s_142_1: read-reg s_142_0:u8
        let s_142_1: u8 = {
            let value = state.read_register::<u8>(s_142_0 as isize);
            tracer.read_register(s_142_0 as isize, value);
            value
        };
        // D s_142_2: call ELUsingAArch32(s_142_1)
        let s_142_2: bool = ELUsingAArch32(state, tracer, s_142_1);
        // D s_142_3: not s_142_2
        let s_142_3: bool = !s_142_2;
        // D s_142_4: write-var gs#124333 <= s_142_3
        fn_state.gs_124333 = s_142_3;
        // N s_142_5: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var t:i
        let s_143_0: i128 = fn_state.t;
        // D s_143_1: call R_read(s_143_0)
        let s_143_1: u32 = R_read(state, tracer, s_143_0);
        // C s_143_2: const #64s : i
        let s_143_2: i128 = 64;
        // D s_143_3: cast zx s_143_1 -> bv
        let s_143_3: Bits = Bits::new(s_143_1 as u128, 32u16);
        // D s_143_4: bits-cast sx s_143_3 -> bv length s_143_2
        let s_143_4: Bits = s_143_3.sign_extend(s_143_2);
        // D s_143_5: cast reint s_143_4 -> u64
        let s_143_5: u64 = (s_143_4.value() as u64);
        // C s_143_6: const #() : ()
        let s_143_6: () = ();
        // S s_143_7: call PhysicalCountInt(s_143_6)
        let s_143_7: u64 = PhysicalCountInt(state, tracer, s_143_6);
        // D s_143_8: cast zx s_143_5 -> bv
        let s_143_8: Bits = Bits::new(s_143_5 as u128, 64u16);
        // S s_143_9: cast zx s_143_7 -> bv
        let s_143_9: Bits = Bits::new(s_143_7 as u128, 64u16);
        // D s_143_10: add s_143_8 s_143_9
        let s_143_10: Bits = (s_143_8 + s_143_9);
        // D s_143_11: cast reint s_143_10 -> u64
        let s_143_11: u64 = (s_143_10.value() as u64);
        // D s_143_12: call Mk_CNTHPS_CVAL_EL2_Type(s_143_11)
        let s_143_12: ProductType5c790c8ef59cc8b2 = Mk_CNTHPS_CVAL_EL2_Type(
            state,
            tracer,
            s_143_11,
        );
        // C s_143_13: const #22672u : u32
        let s_143_13: u32 = 22672;
        // N s_143_14: write-reg s_143_13 <= s_143_12
        let s_143_14: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_143_13 as isize, s_143_12);
            tracer.write_register(s_143_13 as isize, s_143_12);
        };
        // N s_143_15: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #117u : u32
        let s_144_0: u32 = 117;
        // S s_144_1: call IsFeatureImplemented(s_144_0)
        let s_144_1: bool = IsFeatureImplemented(state, tracer, s_144_0);
        // D s_144_2: write-var gs#124332 <= s_144_1
        fn_state.gs_124332 = s_144_1;
        // N s_144_3: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #90704u : u32
        let s_145_0: u32 = 90704;
        // D s_145_1: read-reg s_145_0:struct
        let s_145_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_145_0 as isize);
            tracer.read_register(s_145_0 as isize, value);
            value
        };
        // D s_145_2: call _get_SCR_EL3_Type_NS(s_145_1)
        let s_145_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_145_1);
        // D s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // C s_145_4: const #0u : u8
        let s_145_4: bool = false;
        // C s_145_5: cast zx s_145_4 -> bv
        let s_145_5: Bits = Bits::new(s_145_4 as u128, 1u16);
        // D s_145_6: cmp-eq s_145_3 s_145_5
        let s_145_6: bool = ((s_145_3) == (s_145_5));
        // D s_145_7: write-var gs#124331 <= s_145_6
        fn_state.gs_124331 = s_145_6;
        // N s_145_8: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #102552u : u32
        let s_146_0: u32 = 102552;
        // D s_146_1: read-reg s_146_0:struct
        let s_146_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_146_0 as isize);
            tracer.read_register(s_146_0 as isize, value);
            value
        };
        // D s_146_2: call _get_HCR_EL2_Type_E2H(s_146_1)
        let s_146_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_146_1);
        // C s_146_3: const #102552u : u32
        let s_146_3: u32 = 102552;
        // D s_146_4: read-reg s_146_3:struct
        let s_146_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_146_3 as isize);
            tracer.read_register(s_146_3 as isize, value);
            value
        };
        // D s_146_5: call _get_HCR_EL2_Type_TGE(s_146_4)
        let s_146_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_146_4);
        // D s_146_6: cast zx s_146_2 -> bv
        let s_146_6: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_7: cast zx s_146_5 -> bv
        let s_146_7: Bits = Bits::new(s_146_5 as u128, 1u16);
        // D s_146_8: cast reint s_146_6 -> u128
        let s_146_8: u128 = (s_146_6.value() as u128);
        // D s_146_9: size-of s_146_6
        let s_146_9: u16 = s_146_6.length();
        // D s_146_10: cast reint s_146_7 -> u128
        let s_146_10: u128 = (s_146_7.value() as u128);
        // D s_146_11: size-of s_146_7
        let s_146_11: u16 = s_146_7.length();
        // D s_146_12: lsl s_146_8 s_146_11
        let s_146_12: u128 = s_146_8 << s_146_11;
        // D s_146_13: or s_146_12 s_146_10
        let s_146_13: u128 = ((s_146_12) | (s_146_10));
        // D s_146_14: add s_146_9 s_146_11
        let s_146_14: u16 = (s_146_9 + s_146_11);
        // D s_146_15: create-bits s_146_13 s_146_14
        let s_146_15: Bits = Bits::new(s_146_13, s_146_14);
        // D s_146_16: cast reint s_146_15 -> u8
        let s_146_16: u8 = (s_146_15.value() as u8);
        // D s_146_17: cast zx s_146_16 -> bv
        let s_146_17: Bits = Bits::new(s_146_16 as u128, 2u16);
        // C s_146_18: const #3u : u8
        let s_146_18: u8 = 3;
        // C s_146_19: cast zx s_146_18 -> bv
        let s_146_19: Bits = Bits::new(s_146_18 as u128, 2u16);
        // D s_146_20: cmp-eq s_146_17 s_146_19
        let s_146_20: bool = ((s_146_17) == (s_146_19));
        // D s_146_21: write-var gs#124330 <= s_146_20
        fn_state.gs_124330 = s_146_20;
        // N s_146_22: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #432u : u32
        let s_147_0: u32 = 432;
        // D s_147_1: read-reg s_147_0:u8
        let s_147_1: u8 = {
            let value = state.read_register::<u8>(s_147_0 as isize);
            tracer.read_register(s_147_0 as isize, value);
            value
        };
        // D s_147_2: call ELUsingAArch32(s_147_1)
        let s_147_2: bool = ELUsingAArch32(state, tracer, s_147_1);
        // D s_147_3: not s_147_2
        let s_147_3: bool = !s_147_2;
        // D s_147_4: write-var gs#124329 <= s_147_3
        fn_state.gs_124329 = s_147_3;
        // N s_147_5: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #3u : u8
        let s_148_0: u8 = 3;
        // C s_148_1: cast zx s_148_0 -> bv
        let s_148_1: Bits = Bits::new(s_148_0 as u128, 8u16);
        // C s_148_2: cast zx s_148_1 -> i
        let s_148_2: i128 = (s_148_1.value() as i128);
        // C s_148_3: cast reint s_148_2 -> i64
        let s_148_3: i64 = (s_148_2 as i64);
        // C s_148_4: cast zx s_148_3 -> i
        let s_148_4: i128 = (i128::try_from(s_148_3).unwrap());
        // S s_148_5: call AArch32_TakeHypTrapException(s_148_4)
        let s_148_5: () = AArch32_TakeHypTrapException(state, tracer, s_148_4);
        // N s_148_6: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var __CNTHCTL_PL1PCEN:u8
        let s_149_0: bool = fn_state.u__CNTHCTL_PL1PCEN;
        // D s_149_1: cast zx s_149_0 -> bv
        let s_149_1: Bits = Bits::new(s_149_0 as u128, 1u16);
        // C s_149_2: const #0u : u8
        let s_149_2: bool = false;
        // C s_149_3: cast zx s_149_2 -> bv
        let s_149_3: Bits = Bits::new(s_149_2 as u128, 1u16);
        // D s_149_4: cmp-eq s_149_1 s_149_3
        let s_149_4: bool = ((s_149_1) == (s_149_3));
        // D s_149_5: write-var gs#124328 <= s_149_4
        fn_state.gs_124328 = s_149_4;
        // N s_149_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #432u : u32
        let s_150_0: u32 = 432;
        // D s_150_1: read-reg s_150_0:u8
        let s_150_1: u8 = {
            let value = state.read_register::<u8>(s_150_0 as isize);
            tracer.read_register(s_150_0 as isize, value);
            value
        };
        // D s_150_2: call ELUsingAArch32(s_150_1)
        let s_150_2: bool = ELUsingAArch32(state, tracer, s_150_1);
        // D s_150_3: write-var gs#124327 <= s_150_2
        fn_state.gs_124327 = s_150_2;
        // N s_150_4: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #3u : u8
        let s_151_0: u8 = 3;
        // C s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 8u16);
        // C s_151_2: cast zx s_151_1 -> i
        let s_151_2: i128 = (s_151_1.value() as i128);
        // C s_151_3: cast reint s_151_2 -> i64
        let s_151_3: i64 = (s_151_2 as i64);
        // C s_151_4: cast zx s_151_3 -> i
        let s_151_4: i128 = (i128::try_from(s_151_3).unwrap());
        // C s_151_5: const #432u : u32
        let s_151_5: u32 = 432;
        // D s_151_6: read-reg s_151_5:u8
        let s_151_6: u8 = {
            let value = state.read_register::<u8>(s_151_5 as isize);
            tracer.read_register(s_151_5 as isize, value);
            value
        };
        // D s_151_7: call AArch64_AArch32SystemAccessTrap(s_151_6, s_151_4)
        let s_151_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_151_6,
            s_151_4,
        );
        // N s_151_8: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var __CNTHCTL_EL2_EL0PTEN:u8
        let s_152_0: bool = fn_state.u__CNTHCTL_EL2_EL0PTEN;
        // D s_152_1: cast zx s_152_0 -> bv
        let s_152_1: Bits = Bits::new(s_152_0 as u128, 1u16);
        // C s_152_2: const #0u : u8
        let s_152_2: bool = false;
        // C s_152_3: cast zx s_152_2 -> bv
        let s_152_3: Bits = Bits::new(s_152_2 as u128, 1u16);
        // D s_152_4: cmp-eq s_152_1 s_152_3
        let s_152_4: bool = ((s_152_1) == (s_152_3));
        // D s_152_5: write-var gs#124326 <= s_152_4
        fn_state.gs_124326 = s_152_4;
        // N s_152_6: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #102552u : u32
        let s_153_0: u32 = 102552;
        // D s_153_1: read-reg s_153_0:struct
        let s_153_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_153_0 as isize);
            tracer.read_register(s_153_0 as isize, value);
            value
        };
        // D s_153_2: call _get_HCR_EL2_Type_E2H(s_153_1)
        let s_153_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_153_1);
        // C s_153_3: const #102552u : u32
        let s_153_3: u32 = 102552;
        // D s_153_4: read-reg s_153_3:struct
        let s_153_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_153_3 as isize);
            tracer.read_register(s_153_3 as isize, value);
            value
        };
        // D s_153_5: call _get_HCR_EL2_Type_TGE(s_153_4)
        let s_153_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_153_4);
        // D s_153_6: cast zx s_153_2 -> bv
        let s_153_6: Bits = Bits::new(s_153_2 as u128, 1u16);
        // D s_153_7: cast zx s_153_5 -> bv
        let s_153_7: Bits = Bits::new(s_153_5 as u128, 1u16);
        // D s_153_8: cast reint s_153_6 -> u128
        let s_153_8: u128 = (s_153_6.value() as u128);
        // D s_153_9: size-of s_153_6
        let s_153_9: u16 = s_153_6.length();
        // D s_153_10: cast reint s_153_7 -> u128
        let s_153_10: u128 = (s_153_7.value() as u128);
        // D s_153_11: size-of s_153_7
        let s_153_11: u16 = s_153_7.length();
        // D s_153_12: lsl s_153_8 s_153_11
        let s_153_12: u128 = s_153_8 << s_153_11;
        // D s_153_13: or s_153_12 s_153_10
        let s_153_13: u128 = ((s_153_12) | (s_153_10));
        // D s_153_14: add s_153_9 s_153_11
        let s_153_14: u16 = (s_153_9 + s_153_11);
        // D s_153_15: create-bits s_153_13 s_153_14
        let s_153_15: Bits = Bits::new(s_153_13, s_153_14);
        // D s_153_16: cast reint s_153_15 -> u8
        let s_153_16: u8 = (s_153_15.value() as u8);
        // D s_153_17: cast zx s_153_16 -> bv
        let s_153_17: Bits = Bits::new(s_153_16 as u128, 2u16);
        // C s_153_18: const #3u : u8
        let s_153_18: u8 = 3;
        // C s_153_19: cast zx s_153_18 -> bv
        let s_153_19: Bits = Bits::new(s_153_18 as u128, 2u16);
        // D s_153_20: cmp-eq s_153_17 s_153_19
        let s_153_20: bool = ((s_153_17) == (s_153_19));
        // D s_153_21: write-var gs#124325 <= s_153_20
        fn_state.gs_124325 = s_153_20;
        // N s_153_22: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #432u : u32
        let s_154_0: u32 = 432;
        // D s_154_1: read-reg s_154_0:u8
        let s_154_1: u8 = {
            let value = state.read_register::<u8>(s_154_0 as isize);
            tracer.read_register(s_154_0 as isize, value);
            value
        };
        // D s_154_2: call ELUsingAArch32(s_154_1)
        let s_154_2: bool = ELUsingAArch32(state, tracer, s_154_1);
        // D s_154_3: not s_154_2
        let s_154_3: bool = !s_154_2;
        // D s_154_4: write-var gs#124324 <= s_154_3
        fn_state.gs_124324 = s_154_3;
        // N s_154_5: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #3u : u8
        let s_155_0: u8 = 3;
        // C s_155_1: cast zx s_155_0 -> bv
        let s_155_1: Bits = Bits::new(s_155_0 as u128, 8u16);
        // C s_155_2: cast zx s_155_1 -> i
        let s_155_2: i128 = (s_155_1.value() as i128);
        // C s_155_3: cast reint s_155_2 -> i64
        let s_155_3: i64 = (s_155_2 as i64);
        // C s_155_4: cast zx s_155_3 -> i
        let s_155_4: i128 = (i128::try_from(s_155_3).unwrap());
        // C s_155_5: const #432u : u32
        let s_155_5: u32 = 432;
        // D s_155_6: read-reg s_155_5:u8
        let s_155_6: u8 = {
            let value = state.read_register::<u8>(s_155_5 as isize);
            tracer.read_register(s_155_5 as isize, value);
            value
        };
        // D s_155_7: call AArch64_AArch32SystemAccessTrap(s_155_6, s_155_4)
        let s_155_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_155_6,
            s_155_4,
        );
        // N s_155_8: return
        return;
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_156_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 1u16);
        // C s_156_2: const #0u : u8
        let s_156_2: bool = false;
        // C s_156_3: cast zx s_156_2 -> bv
        let s_156_3: Bits = Bits::new(s_156_2 as u128, 1u16);
        // D s_156_4: cmp-eq s_156_1 s_156_3
        let s_156_4: bool = ((s_156_1) == (s_156_3));
        // D s_156_5: write-var gs#124323 <= s_156_4
        fn_state.gs_124323 = s_156_4;
        // N s_156_6: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #102552u : u32
        let s_157_0: u32 = 102552;
        // D s_157_1: read-reg s_157_0:struct
        let s_157_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_157_0 as isize);
            tracer.read_register(s_157_0 as isize, value);
            value
        };
        // D s_157_2: call _get_HCR_EL2_Type_E2H(s_157_1)
        let s_157_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_157_1);
        // C s_157_3: const #102552u : u32
        let s_157_3: u32 = 102552;
        // D s_157_4: read-reg s_157_3:struct
        let s_157_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_157_3 as isize);
            tracer.read_register(s_157_3 as isize, value);
            value
        };
        // D s_157_5: call _get_HCR_EL2_Type_TGE(s_157_4)
        let s_157_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_157_4);
        // D s_157_6: cast zx s_157_2 -> bv
        let s_157_6: Bits = Bits::new(s_157_2 as u128, 1u16);
        // D s_157_7: cast zx s_157_5 -> bv
        let s_157_7: Bits = Bits::new(s_157_5 as u128, 1u16);
        // D s_157_8: cast reint s_157_6 -> u128
        let s_157_8: u128 = (s_157_6.value() as u128);
        // D s_157_9: size-of s_157_6
        let s_157_9: u16 = s_157_6.length();
        // D s_157_10: cast reint s_157_7 -> u128
        let s_157_10: u128 = (s_157_7.value() as u128);
        // D s_157_11: size-of s_157_7
        let s_157_11: u16 = s_157_7.length();
        // D s_157_12: lsl s_157_8 s_157_11
        let s_157_12: u128 = s_157_8 << s_157_11;
        // D s_157_13: or s_157_12 s_157_10
        let s_157_13: u128 = ((s_157_12) | (s_157_10));
        // D s_157_14: add s_157_9 s_157_11
        let s_157_14: u16 = (s_157_9 + s_157_11);
        // D s_157_15: create-bits s_157_13 s_157_14
        let s_157_15: Bits = Bits::new(s_157_13, s_157_14);
        // D s_157_16: cast reint s_157_15 -> u8
        let s_157_16: u8 = (s_157_15.value() as u8);
        // D s_157_17: cast zx s_157_16 -> bv
        let s_157_17: Bits = Bits::new(s_157_16 as u128, 2u16);
        // C s_157_18: const #2u : u8
        let s_157_18: u8 = 2;
        // C s_157_19: cast zx s_157_18 -> bv
        let s_157_19: Bits = Bits::new(s_157_18 as u128, 2u16);
        // D s_157_20: cmp-eq s_157_17 s_157_19
        let s_157_20: bool = ((s_157_17) == (s_157_19));
        // D s_157_21: write-var gs#124322 <= s_157_20
        fn_state.gs_124322 = s_157_20;
        // N s_157_22: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #432u : u32
        let s_158_0: u32 = 432;
        // D s_158_1: read-reg s_158_0:u8
        let s_158_1: u8 = {
            let value = state.read_register::<u8>(s_158_0 as isize);
            tracer.read_register(s_158_0 as isize, value);
            value
        };
        // D s_158_2: call ELUsingAArch32(s_158_1)
        let s_158_2: bool = ELUsingAArch32(state, tracer, s_158_1);
        // D s_158_3: not s_158_2
        let s_158_3: bool = !s_158_2;
        // D s_158_4: write-var gs#124321 <= s_158_3
        fn_state.gs_124321 = s_158_3;
        // N s_158_5: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #3u : u8
        let s_159_0: u8 = 3;
        // C s_159_1: cast zx s_159_0 -> bv
        let s_159_1: Bits = Bits::new(s_159_0 as u128, 8u16);
        // C s_159_2: cast zx s_159_1 -> i
        let s_159_2: i128 = (s_159_1.value() as i128);
        // C s_159_3: cast reint s_159_2 -> i64
        let s_159_3: i64 = (s_159_2 as i64);
        // C s_159_4: cast zx s_159_3 -> i
        let s_159_4: i128 = (i128::try_from(s_159_3).unwrap());
        // C s_159_5: const #432u : u32
        let s_159_5: u32 = 432;
        // D s_159_6: read-reg s_159_5:u8
        let s_159_6: u8 = {
            let value = state.read_register::<u8>(s_159_5 as isize);
            tracer.read_register(s_159_5 as isize, value);
            value
        };
        // D s_159_7: call AArch64_AArch32SystemAccessTrap(s_159_6, s_159_4)
        let s_159_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_159_6,
            s_159_4,
        );
        // N s_159_8: return
        return;
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_160_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 1u16);
        // C s_160_2: const #0u : u8
        let s_160_2: bool = false;
        // C s_160_3: cast zx s_160_2 -> bv
        let s_160_3: Bits = Bits::new(s_160_2 as u128, 1u16);
        // D s_160_4: cmp-eq s_160_1 s_160_3
        let s_160_4: bool = ((s_160_1) == (s_160_3));
        // D s_160_5: write-var gs#124320 <= s_160_4
        fn_state.gs_124320 = s_160_4;
        // N s_160_6: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var __HCR_EL2_E2H:u8
        let s_161_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_161_1: cast zx s_161_0 -> bv
        let s_161_1: Bits = Bits::new(s_161_0 as u128, 1u16);
        // C s_161_2: const #0u : u8
        let s_161_2: bool = false;
        // C s_161_3: cast zx s_161_2 -> bv
        let s_161_3: Bits = Bits::new(s_161_2 as u128, 1u16);
        // D s_161_4: cmp-eq s_161_1 s_161_3
        let s_161_4: bool = ((s_161_1) == (s_161_3));
        // D s_161_5: write-var gs#124319 <= s_161_4
        fn_state.gs_124319 = s_161_4;
        // N s_161_6: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #432u : u32
        let s_162_0: u32 = 432;
        // D s_162_1: read-reg s_162_0:u8
        let s_162_1: u8 = {
            let value = state.read_register::<u8>(s_162_0 as isize);
            tracer.read_register(s_162_0 as isize, value);
            value
        };
        // D s_162_2: call ELUsingAArch32(s_162_1)
        let s_162_2: bool = ELUsingAArch32(state, tracer, s_162_1);
        // D s_162_3: not s_162_2
        let s_162_3: bool = !s_162_2;
        // D s_162_4: write-var gs#124318 <= s_162_3
        fn_state.gs_124318 = s_162_3;
        // N s_162_5: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #() : ()
        let s_163_0: () = ();
        // S s_163_1: call EL2Enabled(s_163_0)
        let s_163_1: bool = EL2Enabled(state, tracer, s_163_0);
        // N s_163_2: branch s_163_1 b179 b164
        if s_163_1 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #0u : u8
        let s_164_0: bool = false;
        // D s_164_1: write-var gs#124348 <= s_164_0
        fn_state.gs_124348 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#124348:u8
        let s_165_0: bool = fn_state.gs_124348;
        // N s_165_1: branch s_165_0 b178 b166
        if s_165_0 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #0u : u8
        let s_166_0: bool = false;
        // D s_166_1: write-var gs#124349 <= s_166_0
        fn_state.gs_124349 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#124349:u8
        let s_167_0: bool = fn_state.gs_124349;
        // N s_167_1: branch s_167_0 b177 b168
        if s_167_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #() : ()
        let s_168_0: () = ();
        // S s_168_1: call EL2Enabled(s_168_0)
        let s_168_1: bool = EL2Enabled(state, tracer, s_168_0);
        // N s_168_2: branch s_168_1 b176 b169
        if s_168_1 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #0u : u8
        let s_169_0: bool = false;
        // D s_169_1: write-var gs#124350 <= s_169_0
        fn_state.gs_124350 = s_169_0;
        // N s_169_2: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var gs#124350:u8
        let s_170_0: bool = fn_state.gs_124350;
        // N s_170_1: branch s_170_0 b175 b171
        if s_170_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #0u : u8
        let s_171_0: bool = false;
        // D s_171_1: write-var gs#124351 <= s_171_0
        fn_state.gs_124351 = s_171_0;
        // N s_171_2: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var gs#124351:u8
        let s_172_0: bool = fn_state.gs_124351;
        // N s_172_1: branch s_172_0 b174 b173
        if s_172_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_173_0: panic
        panic!("{:?}", ());
        // N s_173_1: return
        return;
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #0u : u8
        let s_174_0: u8 = 0;
        // C s_174_1: cast zx s_174_0 -> bv
        let s_174_1: Bits = Bits::new(s_174_0 as u128, 8u16);
        // C s_174_2: cast zx s_174_1 -> i
        let s_174_2: i128 = (s_174_1.value() as i128);
        // C s_174_3: cast reint s_174_2 -> i64
        let s_174_3: i64 = (s_174_2 as i64);
        // C s_174_4: cast zx s_174_3 -> i
        let s_174_4: i128 = (i128::try_from(s_174_3).unwrap());
        // S s_174_5: call AArch32_TakeHypTrapException(s_174_4)
        let s_174_5: () = AArch32_TakeHypTrapException(state, tracer, s_174_4);
        // N s_174_6: return
        return;
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var __HCR_TGE:u8
        let s_175_0: bool = fn_state.u__HCR_TGE;
        // D s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 1u16);
        // C s_175_2: const #1u : u8
        let s_175_2: bool = true;
        // C s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // D s_175_4: cmp-eq s_175_1 s_175_3
        let s_175_4: bool = ((s_175_1) == (s_175_3));
        // D s_175_5: write-var gs#124351 <= s_175_4
        fn_state.gs_124351 = s_175_4;
        // N s_175_6: jump b172
        return block_172(state, tracer, fn_state);
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
        // D s_176_3: write-var gs#124350 <= s_176_2
        fn_state.gs_124350 = s_176_2;
        // N s_176_4: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #3u : u8
        let s_177_0: u8 = 3;
        // C s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 8u16);
        // C s_177_2: cast zx s_177_1 -> i
        let s_177_2: i128 = (s_177_1.value() as i128);
        // C s_177_3: cast reint s_177_2 -> i64
        let s_177_3: i64 = (s_177_2 as i64);
        // C s_177_4: cast zx s_177_3 -> i
        let s_177_4: i128 = (i128::try_from(s_177_3).unwrap());
        // C s_177_5: const #432u : u32
        let s_177_5: u32 = 432;
        // D s_177_6: read-reg s_177_5:u8
        let s_177_6: u8 = {
            let value = state.read_register::<u8>(s_177_5 as isize);
            tracer.read_register(s_177_5 as isize, value);
            value
        };
        // D s_177_7: call AArch64_AArch32SystemAccessTrap(s_177_6, s_177_4)
        let s_177_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_177_6,
            s_177_4,
        );
        // N s_177_8: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var __HCR_EL2_TGE:u8
        let s_178_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 1u16);
        // C s_178_2: const #1u : u8
        let s_178_2: bool = true;
        // C s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // D s_178_4: cmp-eq s_178_1 s_178_3
        let s_178_4: bool = ((s_178_1) == (s_178_3));
        // D s_178_5: write-var gs#124349 <= s_178_4
        fn_state.gs_124349 = s_178_4;
        // N s_178_6: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #432u : u32
        let s_179_0: u32 = 432;
        // D s_179_1: read-reg s_179_0:u8
        let s_179_1: u8 = {
            let value = state.read_register::<u8>(s_179_0 as isize);
            tracer.read_register(s_179_0 as isize, value);
            value
        };
        // D s_179_2: call ELUsingAArch32(s_179_1)
        let s_179_2: bool = ELUsingAArch32(state, tracer, s_179_1);
        // D s_179_3: not s_179_2
        let s_179_3: bool = !s_179_2;
        // D s_179_4: write-var gs#124348 <= s_179_3
        fn_state.gs_124348 = s_179_3;
        // N s_179_5: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var __CNTKCTL_PL0PTEN:u8
        let s_180_0: bool = fn_state.u__CNTKCTL_PL0PTEN;
        // D s_180_1: cast zx s_180_0 -> bv
        let s_180_1: Bits = Bits::new(s_180_0 as u128, 1u16);
        // C s_180_2: const #0u : u8
        let s_180_2: bool = false;
        // C s_180_3: cast zx s_180_2 -> bv
        let s_180_3: Bits = Bits::new(s_180_2 as u128, 1u16);
        // D s_180_4: cmp-eq s_180_1 s_180_3
        let s_180_4: bool = ((s_180_1) == (s_180_3));
        // D s_180_5: write-var gs#124317 <= s_180_4
        fn_state.gs_124317 = s_180_4;
        // N s_180_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #() : ()
        let s_181_0: () = ();
        // S s_181_1: call EL2Enabled(s_181_0)
        let s_181_1: bool = EL2Enabled(state, tracer, s_181_0);
        // N s_181_2: branch s_181_1 b189 b182
        if s_181_1 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_182(state, tracer, fn_state);
        };
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #0u : u8
        let s_182_0: bool = false;
        // D s_182_1: write-var gs#124352 <= s_182_0
        fn_state.gs_124352 = s_182_0;
        // N s_182_2: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var gs#124352:u8
        let s_183_0: bool = fn_state.gs_124352;
        // N s_183_1: branch s_183_0 b188 b184
        if s_183_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_184(state, tracer, fn_state);
        };
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #0u : u8
        let s_184_0: bool = false;
        // D s_184_1: write-var gs#124353 <= s_184_0
        fn_state.gs_124353 = s_184_0;
        // N s_184_2: jump b185
        return block_185(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_185_0: read-var gs#124353:u8
        let s_185_0: bool = fn_state.gs_124353;
        // N s_185_1: branch s_185_0 b187 b186
        if s_185_0 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_186(state, tracer, fn_state);
        };
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #3u : u8
        let s_186_0: u8 = 3;
        // C s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 8u16);
        // C s_186_2: cast zx s_186_1 -> i
        let s_186_2: i128 = (s_186_1.value() as i128);
        // C s_186_3: cast reint s_186_2 -> i64
        let s_186_3: i64 = (s_186_2 as i64);
        // C s_186_4: cast zx s_186_3 -> i
        let s_186_4: i128 = (i128::try_from(s_186_3).unwrap());
        // C s_186_5: const #440u : u32
        let s_186_5: u32 = 440;
        // D s_186_6: read-reg s_186_5:u8
        let s_186_6: u8 = {
            let value = state.read_register::<u8>(s_186_5 as isize);
            tracer.read_register(s_186_5 as isize, value);
            value
        };
        // D s_186_7: call AArch64_AArch32SystemAccessTrap(s_186_6, s_186_4)
        let s_186_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_186_6,
            s_186_4,
        );
        // N s_186_8: return
        return;
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #3u : u8
        let s_187_0: u8 = 3;
        // C s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 8u16);
        // C s_187_2: cast zx s_187_1 -> i
        let s_187_2: i128 = (s_187_1.value() as i128);
        // C s_187_3: cast reint s_187_2 -> i64
        let s_187_3: i64 = (s_187_2 as i64);
        // C s_187_4: cast zx s_187_3 -> i
        let s_187_4: i128 = (i128::try_from(s_187_3).unwrap());
        // C s_187_5: const #432u : u32
        let s_187_5: u32 = 432;
        // D s_187_6: read-reg s_187_5:u8
        let s_187_6: u8 = {
            let value = state.read_register::<u8>(s_187_5 as isize);
            tracer.read_register(s_187_5 as isize, value);
            value
        };
        // D s_187_7: call AArch64_AArch32SystemAccessTrap(s_187_6, s_187_4)
        let s_187_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_187_6,
            s_187_4,
        );
        // N s_187_8: return
        return;
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var __HCR_EL2_TGE:u8
        let s_188_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_188_1: cast zx s_188_0 -> bv
        let s_188_1: Bits = Bits::new(s_188_0 as u128, 1u16);
        // C s_188_2: const #1u : u8
        let s_188_2: bool = true;
        // C s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 1u16);
        // D s_188_4: cmp-eq s_188_1 s_188_3
        let s_188_4: bool = ((s_188_1) == (s_188_3));
        // D s_188_5: write-var gs#124353 <= s_188_4
        fn_state.gs_124353 = s_188_4;
        // N s_188_6: jump b185
        return block_185(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #432u : u32
        let s_189_0: u32 = 432;
        // D s_189_1: read-reg s_189_0:u8
        let s_189_1: u8 = {
            let value = state.read_register::<u8>(s_189_0 as isize);
            tracer.read_register(s_189_0 as isize, value);
            value
        };
        // D s_189_2: call ELUsingAArch32(s_189_1)
        let s_189_2: bool = ELUsingAArch32(state, tracer, s_189_1);
        // D s_189_3: not s_189_2
        let s_189_3: bool = !s_189_2;
        // D s_189_4: write-var gs#124352 <= s_189_3
        fn_state.gs_124352 = s_189_3;
        // N s_189_5: jump b183
        return block_183(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var __CNTKCTL_EL1_EL0PTEN:u8
        let s_190_0: bool = fn_state.u__CNTKCTL_EL1_EL0PTEN;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 1u16);
        // C s_190_2: const #0u : u8
        let s_190_2: bool = false;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 1u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#124316 <= s_190_4
        fn_state.gs_124316 = s_190_4;
        // N s_190_6: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #() : ()
        let s_191_0: () = ();
        // S s_191_1: call EL2Enabled(s_191_0)
        let s_191_1: bool = EL2Enabled(state, tracer, s_191_0);
        // N s_191_2: branch s_191_1 b194 b192
        if s_191_1 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_192(state, tracer, fn_state);
        };
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #0u : u8
        let s_192_0: bool = false;
        // D s_192_1: write-var gs#124314 <= s_192_0
        fn_state.gs_124314 = s_192_0;
        // N s_192_2: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var gs#124314:u8
        let s_193_0: bool = fn_state.gs_124314;
        // D s_193_1: not s_193_0
        let s_193_1: bool = !s_193_0;
        // D s_193_2: write-var gs#124315 <= s_193_1
        fn_state.gs_124315 = s_193_1;
        // N s_193_3: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #102552u : u32
        let s_194_0: u32 = 102552;
        // D s_194_1: read-reg s_194_0:struct
        let s_194_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_194_0 as isize);
            tracer.read_register(s_194_0 as isize, value);
            value
        };
        // D s_194_2: call _get_HCR_EL2_Type_E2H(s_194_1)
        let s_194_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_194_1);
        // C s_194_3: const #102552u : u32
        let s_194_3: u32 = 102552;
        // D s_194_4: read-reg s_194_3:struct
        let s_194_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_194_3 as isize);
            tracer.read_register(s_194_3 as isize, value);
            value
        };
        // D s_194_5: call _get_HCR_EL2_Type_TGE(s_194_4)
        let s_194_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_194_4);
        // D s_194_6: cast zx s_194_2 -> bv
        let s_194_6: Bits = Bits::new(s_194_2 as u128, 1u16);
        // D s_194_7: cast zx s_194_5 -> bv
        let s_194_7: Bits = Bits::new(s_194_5 as u128, 1u16);
        // D s_194_8: cast reint s_194_6 -> u128
        let s_194_8: u128 = (s_194_6.value() as u128);
        // D s_194_9: size-of s_194_6
        let s_194_9: u16 = s_194_6.length();
        // D s_194_10: cast reint s_194_7 -> u128
        let s_194_10: u128 = (s_194_7.value() as u128);
        // D s_194_11: size-of s_194_7
        let s_194_11: u16 = s_194_7.length();
        // D s_194_12: lsl s_194_8 s_194_11
        let s_194_12: u128 = s_194_8 << s_194_11;
        // D s_194_13: or s_194_12 s_194_10
        let s_194_13: u128 = ((s_194_12) | (s_194_10));
        // D s_194_14: add s_194_9 s_194_11
        let s_194_14: u16 = (s_194_9 + s_194_11);
        // D s_194_15: create-bits s_194_13 s_194_14
        let s_194_15: Bits = Bits::new(s_194_13, s_194_14);
        // D s_194_16: cast reint s_194_15 -> u8
        let s_194_16: u8 = (s_194_15.value() as u8);
        // D s_194_17: cast zx s_194_16 -> bv
        let s_194_17: Bits = Bits::new(s_194_16 as u128, 2u16);
        // C s_194_18: const #3u : u8
        let s_194_18: u8 = 3;
        // C s_194_19: cast zx s_194_18 -> bv
        let s_194_19: Bits = Bits::new(s_194_18 as u128, 2u16);
        // D s_194_20: cmp-eq s_194_17 s_194_19
        let s_194_20: bool = ((s_194_17) == (s_194_19));
        // D s_194_21: write-var gs#124314 <= s_194_20
        fn_state.gs_124314 = s_194_20;
        // N s_194_22: jump b193
        return block_193(state, tracer, fn_state);
    }
}
