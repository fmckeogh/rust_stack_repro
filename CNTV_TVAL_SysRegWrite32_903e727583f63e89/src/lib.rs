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
use u_get_CNTKCTL_EL1_Type_EL0VTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_E2H::*;
use Mk_CNTHVS_CVAL_EL2_Type::*;
use AArch32_TakeHypTrapException::*;
use HCR_read::*;
use IsFeatureImplemented::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_HCR_Type_TGE::*;
use u_get_CNTHCTL_EL2_Type_EL1TVT::*;
use CNTV_CVAL_write::*;
use u_get_CNTKCTL_Type_PL0VTEN::*;
use R_read::*;
use ELUsingAArch32::*;
use Mk_CNTHV_CVAL_EL2_Type::*;
use u_get_CNTHCTL_EL2_Type_EL0VTEN::*;
use CNTVOFF_read::*;
use PhysicalCountInt::*;
use EL2Enabled::*;
use CNTKCTL_read__1::*;
use Mk_CNTV_CVAL_Type::*;
use common::*;
pub fn CNTV_TVAL_SysRegWrite32_903e727583f63e89<T: Tracer>(
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
        gs_124385: bool,
        gs_124403: bool,
        gs_124419: bool,
        u__CNTHCTL_EL2_EL1TVT: bool,
        gs_124391: bool,
        gs_124404: bool,
        u__HCR_TGE: bool,
        gs_124416: bool,
        gs_124405: bool,
        gs_124400: bool,
        gs_124406: bool,
        gs_124409: bool,
        gs_124393: bool,
        u__CNTKCTL_PL0VTEN: bool,
        gs_124386: bool,
        u__CNTKCTL_EL1_EL0VTEN: bool,
        u__PSTATE_EL: u8,
        gs_124398: bool,
        gs_124401: bool,
        gs_124397: bool,
        gs_124415: bool,
        gs_124418: bool,
        u__HCR_EL2_TGE: bool,
        gs_124420: bool,
        u__CNTHCTL_EL2_EL0VTEN: bool,
        gs_124392: bool,
        gs_124402: bool,
        gs_124408: bool,
        gs_124399: bool,
        gs_124395: bool,
        gs_124396: bool,
        gs_124417: bool,
        gs_124387: bool,
        gs_124407: bool,
        gs_124394: bool,
        gs_124384: bool,
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
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0VTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0VTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0VTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0VTEN = s_0_5;
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
        // S s_0_13: call _get_CNTKCTL_Type_PL0VTEN(s_0_12)
        let s_0_13: bool = u_get_CNTKCTL_Type_PL0VTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTKCTL_PL0VTEN <= s_0_13
        fn_state.u__CNTKCTL_PL0VTEN = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TGE <= s_0_17
        fn_state.u__HCR_TGE = s_0_17;
        // C s_0_19: const #12808u : u32
        let s_0_19: u32 = 12808;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CNTHCTL_EL2_Type_EL0VTEN(s_0_20)
        let s_0_21: bool = u_get_CNTHCTL_EL2_Type_EL0VTEN(state, tracer, s_0_20);
        // D s_0_22: write-var __CNTHCTL_EL2_EL0VTEN <= s_0_21
        fn_state.u__CNTHCTL_EL2_EL0VTEN = s_0_21;
        // C s_0_23: const #12808u : u32
        let s_0_23: u32 = 12808;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_CNTHCTL_EL2_Type_EL1TVT(s_0_24)
        let s_0_25: bool = u_get_CNTHCTL_EL2_Type_EL1TVT(state, tracer, s_0_24);
        // D s_0_26: write-var __CNTHCTL_EL2_EL1TVT <= s_0_25
        fn_state.u__CNTHCTL_EL2_EL1TVT = s_0_25;
        // D s_0_27: read-var __PSTATE_EL:u8
        let s_0_27: u8 = fn_state.u__PSTATE_EL;
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 2u16);
        // C s_0_29: const #448u : u32
        let s_0_29: u32 = 448;
        // D s_0_30: read-reg s_0_29:u8
        let s_0_30: u8 = {
            let value = state.read_register::<u8>(s_0_29 as isize);
            tracer.read_register(s_0_29 as isize, value);
            value
        };
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 2u16);
        // D s_0_32: cmp-eq s_0_28 s_0_31
        let s_0_32: bool = ((s_0_28) == (s_0_31));
        // N s_0_33: branch s_0_32 b28 b1
        if s_0_32 {
            return block_28(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b9 b2
        if s_1_5 {
            return block_9(state, tracer, fn_state);
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
        // C s_5_0: const #432u : u32
        let s_5_0: u32 = 432;
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
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
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
        // D s_6_12: call Mk_CNTV_CVAL_Type(s_6_11)
        let s_6_12: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_6_11,
        );
        // D s_6_13: call CNTV_CVAL_write(s_6_12)
        let s_6_13: () = CNTV_CVAL_write(state, tracer, s_6_12);
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
        // C s_7_12: const #() : ()
        let s_7_12: () = ();
        // S s_7_13: call CNTVOFF_read(s_7_12)
        let s_7_13: u64 = CNTVOFF_read(state, tracer, s_7_12);
        // D s_7_14: cast zx s_7_11 -> bv
        let s_7_14: Bits = Bits::new(s_7_11 as u128, 64u16);
        // S s_7_15: cast zx s_7_13 -> bv
        let s_7_15: Bits = Bits::new(s_7_13 as u128, 64u16);
        // D s_7_16: sub s_7_14 s_7_15
        let s_7_16: Bits = ((s_7_14) - (s_7_15));
        // D s_7_17: cast reint s_7_16 -> u64
        let s_7_17: u64 = (s_7_16.value() as u64);
        // D s_7_18: call Mk_CNTV_CVAL_Type(s_7_17)
        let s_7_18: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_7_17,
        );
        // D s_7_19: call CNTV_CVAL_write(s_7_18)
        let s_7_19: () = CNTV_CVAL_write(state, tracer, s_7_18);
        // N s_7_20: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var t:i
        let s_8_0: i128 = fn_state.t;
        // D s_8_1: call R_read(s_8_0)
        let s_8_1: u32 = R_read(state, tracer, s_8_0);
        // C s_8_2: const #64s : i
        let s_8_2: i128 = 64;
        // D s_8_3: cast zx s_8_1 -> bv
        let s_8_3: Bits = Bits::new(s_8_1 as u128, 32u16);
        // D s_8_4: bits-cast sx s_8_3 -> bv length s_8_2
        let s_8_4: Bits = s_8_3.sign_extend(s_8_2);
        // D s_8_5: cast reint s_8_4 -> u64
        let s_8_5: u64 = (s_8_4.value() as u64);
        // C s_8_6: const #() : ()
        let s_8_6: () = ();
        // S s_8_7: call PhysicalCountInt(s_8_6)
        let s_8_7: u64 = PhysicalCountInt(state, tracer, s_8_6);
        // D s_8_8: cast zx s_8_5 -> bv
        let s_8_8: Bits = Bits::new(s_8_5 as u128, 64u16);
        // S s_8_9: cast zx s_8_7 -> bv
        let s_8_9: Bits = Bits::new(s_8_7 as u128, 64u16);
        // D s_8_10: add s_8_8 s_8_9
        let s_8_10: Bits = (s_8_8 + s_8_9);
        // D s_8_11: cast reint s_8_10 -> u64
        let s_8_11: u64 = (s_8_10.value() as u64);
        // C s_8_12: const #() : ()
        let s_8_12: () = ();
        // S s_8_13: call CNTVOFF_read(s_8_12)
        let s_8_13: u64 = CNTVOFF_read(state, tracer, s_8_12);
        // D s_8_14: cast zx s_8_11 -> bv
        let s_8_14: Bits = Bits::new(s_8_11 as u128, 64u16);
        // S s_8_15: cast zx s_8_13 -> bv
        let s_8_15: Bits = Bits::new(s_8_13 as u128, 64u16);
        // D s_8_16: sub s_8_14 s_8_15
        let s_8_16: Bits = ((s_8_14) - (s_8_15));
        // D s_8_17: cast reint s_8_16 -> u64
        let s_8_17: u64 = (s_8_16.value() as u64);
        // D s_8_18: call Mk_CNTV_CVAL_Type(s_8_17)
        let s_8_18: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_8_17,
        );
        // D s_8_19: call CNTV_CVAL_write(s_8_18)
        let s_8_19: () = CNTV_CVAL_write(state, tracer, s_8_18);
        // N s_8_20: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call EL2Enabled(s_9_0)
        let s_9_1: bool = EL2Enabled(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b27 b10
        if s_9_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#124384 <= s_10_0
        fn_state.gs_124384 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#124384:u8
        let s_11_0: bool = fn_state.gs_124384;
        // N s_11_1: branch s_11_0 b26 b12
        if s_11_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#124385 <= s_12_0
        fn_state.gs_124385 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#124385:u8
        let s_13_0: bool = fn_state.gs_124385;
        // N s_13_1: branch s_13_0 b25 b14
        if s_13_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #432u : u32
        let s_14_0: u32 = 432;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #2u : u8
        let s_14_2: u8 = 2;
        // D s_14_3: cmp-lt s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) < (s_14_2));
        // N s_14_4: branch s_14_3 b24 b15
        if s_14_3 {
            return block_24(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#124386 <= s_15_0
        fn_state.gs_124386 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#124386:u8
        let s_16_0: bool = fn_state.gs_124386;
        // N s_16_1: branch s_16_0 b23 b17
        if s_16_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #432u : u32
        let s_17_0: u32 = 432;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // N s_17_4: branch s_17_3 b22 b18
        if s_17_3 {
            return block_22(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#124387 <= s_18_0
        fn_state.gs_124387 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#124387:u8
        let s_19_0: bool = fn_state.gs_124387;
        // N s_19_1: branch s_19_0 b21 b20
        if s_19_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var t:i
        let s_20_0: i128 = fn_state.t;
        // D s_20_1: call R_read(s_20_0)
        let s_20_1: u32 = R_read(state, tracer, s_20_0);
        // C s_20_2: const #64s : i
        let s_20_2: i128 = 64;
        // D s_20_3: cast zx s_20_1 -> bv
        let s_20_3: Bits = Bits::new(s_20_1 as u128, 32u16);
        // D s_20_4: bits-cast sx s_20_3 -> bv length s_20_2
        let s_20_4: Bits = s_20_3.sign_extend(s_20_2);
        // D s_20_5: cast reint s_20_4 -> u64
        let s_20_5: u64 = (s_20_4.value() as u64);
        // C s_20_6: const #() : ()
        let s_20_6: () = ();
        // S s_20_7: call PhysicalCountInt(s_20_6)
        let s_20_7: u64 = PhysicalCountInt(state, tracer, s_20_6);
        // D s_20_8: cast zx s_20_5 -> bv
        let s_20_8: Bits = Bits::new(s_20_5 as u128, 64u16);
        // S s_20_9: cast zx s_20_7 -> bv
        let s_20_9: Bits = Bits::new(s_20_7 as u128, 64u16);
        // D s_20_10: add s_20_8 s_20_9
        let s_20_10: Bits = (s_20_8 + s_20_9);
        // D s_20_11: cast reint s_20_10 -> u64
        let s_20_11: u64 = (s_20_10.value() as u64);
        // D s_20_12: call Mk_CNTV_CVAL_Type(s_20_11)
        let s_20_12: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_20_11,
        );
        // D s_20_13: call CNTV_CVAL_write(s_20_12)
        let s_20_13: () = CNTV_CVAL_write(state, tracer, s_20_12);
        // N s_20_14: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var t:i
        let s_21_0: i128 = fn_state.t;
        // D s_21_1: call R_read(s_21_0)
        let s_21_1: u32 = R_read(state, tracer, s_21_0);
        // C s_21_2: const #64s : i
        let s_21_2: i128 = 64;
        // D s_21_3: cast zx s_21_1 -> bv
        let s_21_3: Bits = Bits::new(s_21_1 as u128, 32u16);
        // D s_21_4: bits-cast sx s_21_3 -> bv length s_21_2
        let s_21_4: Bits = s_21_3.sign_extend(s_21_2);
        // D s_21_5: cast reint s_21_4 -> u64
        let s_21_5: u64 = (s_21_4.value() as u64);
        // C s_21_6: const #() : ()
        let s_21_6: () = ();
        // S s_21_7: call PhysicalCountInt(s_21_6)
        let s_21_7: u64 = PhysicalCountInt(state, tracer, s_21_6);
        // D s_21_8: cast zx s_21_5 -> bv
        let s_21_8: Bits = Bits::new(s_21_5 as u128, 64u16);
        // S s_21_9: cast zx s_21_7 -> bv
        let s_21_9: Bits = Bits::new(s_21_7 as u128, 64u16);
        // D s_21_10: add s_21_8 s_21_9
        let s_21_10: Bits = (s_21_8 + s_21_9);
        // D s_21_11: cast reint s_21_10 -> u64
        let s_21_11: u64 = (s_21_10.value() as u64);
        // C s_21_12: const #() : ()
        let s_21_12: () = ();
        // S s_21_13: call CNTVOFF_read(s_21_12)
        let s_21_13: u64 = CNTVOFF_read(state, tracer, s_21_12);
        // D s_21_14: cast zx s_21_11 -> bv
        let s_21_14: Bits = Bits::new(s_21_11 as u128, 64u16);
        // S s_21_15: cast zx s_21_13 -> bv
        let s_21_15: Bits = Bits::new(s_21_13 as u128, 64u16);
        // D s_21_16: sub s_21_14 s_21_15
        let s_21_16: Bits = ((s_21_14) - (s_21_15));
        // D s_21_17: cast reint s_21_16 -> u64
        let s_21_17: u64 = (s_21_16.value() as u64);
        // D s_21_18: call Mk_CNTV_CVAL_Type(s_21_17)
        let s_21_18: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_21_17,
        );
        // D s_21_19: call CNTV_CVAL_write(s_21_18)
        let s_21_19: () = CNTV_CVAL_write(state, tracer, s_21_18);
        // N s_21_20: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #432u : u32
        let s_22_0: u32 = 432;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call ELUsingAArch32(s_22_1)
        let s_22_2: bool = ELUsingAArch32(state, tracer, s_22_1);
        // D s_22_3: write-var gs#124387 <= s_22_2
        fn_state.gs_124387 = s_22_2;
        // N s_22_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var t:i
        let s_23_0: i128 = fn_state.t;
        // D s_23_1: call R_read(s_23_0)
        let s_23_1: u32 = R_read(state, tracer, s_23_0);
        // C s_23_2: const #64s : i
        let s_23_2: i128 = 64;
        // D s_23_3: cast zx s_23_1 -> bv
        let s_23_3: Bits = Bits::new(s_23_1 as u128, 32u16);
        // D s_23_4: bits-cast sx s_23_3 -> bv length s_23_2
        let s_23_4: Bits = s_23_3.sign_extend(s_23_2);
        // D s_23_5: cast reint s_23_4 -> u64
        let s_23_5: u64 = (s_23_4.value() as u64);
        // C s_23_6: const #() : ()
        let s_23_6: () = ();
        // S s_23_7: call PhysicalCountInt(s_23_6)
        let s_23_7: u64 = PhysicalCountInt(state, tracer, s_23_6);
        // D s_23_8: cast zx s_23_5 -> bv
        let s_23_8: Bits = Bits::new(s_23_5 as u128, 64u16);
        // S s_23_9: cast zx s_23_7 -> bv
        let s_23_9: Bits = Bits::new(s_23_7 as u128, 64u16);
        // D s_23_10: add s_23_8 s_23_9
        let s_23_10: Bits = (s_23_8 + s_23_9);
        // D s_23_11: cast reint s_23_10 -> u64
        let s_23_11: u64 = (s_23_10.value() as u64);
        // D s_23_12: cast zx s_23_11 -> bv
        let s_23_12: Bits = Bits::new(s_23_11 as u128, 64u16);
        // C s_23_13: const #22400u : u32
        let s_23_13: u32 = 22400;
        // D s_23_14: read-reg s_23_13:u64
        let s_23_14: u64 = {
            let value = state.read_register::<u64>(s_23_13 as isize);
            tracer.read_register(s_23_13 as isize, value);
            value
        };
        // D s_23_15: cast zx s_23_14 -> bv
        let s_23_15: Bits = Bits::new(s_23_14 as u128, 64u16);
        // D s_23_16: sub s_23_12 s_23_15
        let s_23_16: Bits = ((s_23_12) - (s_23_15));
        // D s_23_17: cast reint s_23_16 -> u64
        let s_23_17: u64 = (s_23_16.value() as u64);
        // D s_23_18: call Mk_CNTV_CVAL_Type(s_23_17)
        let s_23_18: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_23_17,
        );
        // D s_23_19: call CNTV_CVAL_write(s_23_18)
        let s_23_19: () = CNTV_CVAL_write(state, tracer, s_23_18);
        // N s_23_20: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #432u : u32
        let s_24_0: u32 = 432;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call ELUsingAArch32(s_24_1)
        let s_24_2: bool = ELUsingAArch32(state, tracer, s_24_1);
        // D s_24_3: not s_24_2
        let s_24_3: bool = !s_24_2;
        // D s_24_4: write-var gs#124386 <= s_24_3
        fn_state.gs_124386 = s_24_3;
        // N s_24_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #3u : u8
        let s_25_0: u8 = 3;
        // C s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 8u16);
        // C s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (s_25_1.value() as i128);
        // C s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #432u : u32
        let s_25_5: u32 = 432;
        // D s_25_6: read-reg s_25_5:u8
        let s_25_6: u8 = {
            let value = state.read_register::<u8>(s_25_5 as isize);
            tracer.read_register(s_25_5 as isize, value);
            value
        };
        // D s_25_7: call AArch64_AArch32SystemAccessTrap(s_25_6, s_25_4)
        let s_25_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_25_6, s_25_4);
        // N s_25_8: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_26_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#124385 <= s_26_4
        fn_state.gs_124385 = s_26_4;
        // N s_26_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #432u : u32
        let s_27_0: u32 = 432;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call ELUsingAArch32(s_27_1)
        let s_27_2: bool = ELUsingAArch32(state, tracer, s_27_1);
        // D s_27_3: not s_27_2
        let s_27_3: bool = !s_27_2;
        // D s_27_4: write-var gs#124384 <= s_27_3
        fn_state.gs_124384 = s_27_3;
        // N s_27_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #440u : u32
        let s_28_0: u32 = 440;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call ELUsingAArch32(s_28_1)
        let s_28_2: bool = ELUsingAArch32(state, tracer, s_28_1);
        // D s_28_3: not s_28_2
        let s_28_3: bool = !s_28_2;
        // N s_28_4: branch s_28_3 b122 b29
        if s_28_3 {
            return block_122(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#124392 <= s_29_0
        fn_state.gs_124392 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#124392:u8
        let s_30_0: bool = fn_state.gs_124392;
        // N s_30_1: branch s_30_0 b121 b31
        if s_30_0 {
            return block_121(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#124393 <= s_31_0
        fn_state.gs_124393 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#124393:u8
        let s_32_0: bool = fn_state.gs_124393;
        // N s_32_1: branch s_32_0 b112 b33
        if s_32_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #440u : u32
        let s_33_0: u32 = 440;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call ELUsingAArch32(s_33_1)
        let s_33_2: bool = ELUsingAArch32(state, tracer, s_33_1);
        // N s_33_3: branch s_33_2 b111 b34
        if s_33_2 {
            return block_111(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#124394 <= s_34_0
        fn_state.gs_124394 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#124394:u8
        let s_35_0: bool = fn_state.gs_124394;
        // N s_35_1: branch s_35_0 b94 b36
        if s_35_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call EL2Enabled(s_36_0)
        let s_36_1: bool = EL2Enabled(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b93 b37
        if s_36_1 {
            return block_93(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#124395 <= s_37_0
        fn_state.gs_124395 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#124395:u8
        let s_38_0: bool = fn_state.gs_124395;
        // N s_38_1: branch s_38_0 b92 b39
        if s_38_0 {
            return block_92(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#124396 <= s_39_0
        fn_state.gs_124396 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#124396:u8
        let s_40_0: bool = fn_state.gs_124396;
        // N s_40_1: branch s_40_0 b91 b41
        if s_40_0 {
            return block_91(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#124397 <= s_41_0
        fn_state.gs_124397 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#124397:u8
        let s_42_0: bool = fn_state.gs_124397;
        // N s_42_1: branch s_42_0 b90 b43
        if s_42_0 {
            return block_90(state, tracer, fn_state);
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
        // S s_43_1: call EL2Enabled(s_43_0)
        let s_43_1: bool = EL2Enabled(state, tracer, s_43_0);
        // N s_43_2: branch s_43_1 b89 b44
        if s_43_1 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#124398 <= s_44_0
        fn_state.gs_124398 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#124398:u8
        let s_45_0: bool = fn_state.gs_124398;
        // N s_45_1: branch s_45_0 b88 b46
        if s_45_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#124399 <= s_46_0
        fn_state.gs_124399 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#124399:u8
        let s_47_0: bool = fn_state.gs_124399;
        // N s_47_1: branch s_47_0 b87 b48
        if s_47_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#124400 <= s_48_0
        fn_state.gs_124400 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#124400:u8
        let s_49_0: bool = fn_state.gs_124400;
        // N s_49_1: branch s_49_0 b86 b50
        if s_49_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call EL2Enabled(s_50_0)
        let s_50_1: bool = EL2Enabled(state, tracer, s_50_0);
        // N s_50_2: branch s_50_1 b85 b51
        if s_50_1 {
            return block_85(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#124401 <= s_51_0
        fn_state.gs_124401 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#124401:u8
        let s_52_0: bool = fn_state.gs_124401;
        // N s_52_1: branch s_52_0 b84 b53
        if s_52_0 {
            return block_84(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#124402 <= s_53_0
        fn_state.gs_124402 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#124402:u8
        let s_54_0: bool = fn_state.gs_124402;
        // N s_54_1: branch s_54_0 b83 b55
        if s_54_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#124403 <= s_55_0
        fn_state.gs_124403 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#124403:u8
        let s_56_0: bool = fn_state.gs_124403;
        // N s_56_1: branch s_56_0 b82 b57
        if s_56_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#124404 <= s_57_0
        fn_state.gs_124404 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#124404:u8
        let s_58_0: bool = fn_state.gs_124404;
        // N s_58_1: branch s_58_0 b81 b59
        if s_58_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call EL2Enabled(s_59_0)
        let s_59_1: bool = EL2Enabled(state, tracer, s_59_0);
        // N s_59_2: branch s_59_1 b80 b60
        if s_59_1 {
            return block_80(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#124405 <= s_60_0
        fn_state.gs_124405 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#124405:u8
        let s_61_0: bool = fn_state.gs_124405;
        // N s_61_1: branch s_61_0 b79 b62
        if s_61_0 {
            return block_79(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#124406 <= s_62_0
        fn_state.gs_124406 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#124406:u8
        let s_63_0: bool = fn_state.gs_124406;
        // N s_63_1: branch s_63_0 b78 b64
        if s_63_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#124407 <= s_64_0
        fn_state.gs_124407 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#124407:u8
        let s_65_0: bool = fn_state.gs_124407;
        // N s_65_1: branch s_65_0 b77 b66
        if s_65_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #432u : u32
        let s_66_0: u32 = 432;
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
        // N s_66_4: branch s_66_3 b76 b67
        if s_66_3 {
            return block_76(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#124408 <= s_67_0
        fn_state.gs_124408 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#124408:u8
        let s_68_0: bool = fn_state.gs_124408;
        // N s_68_1: branch s_68_0 b75 b69
        if s_68_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #432u : u32
        let s_69_0: u32 = 432;
        // D s_69_1: read-reg s_69_0:u8
        let s_69_1: u8 = {
            let value = state.read_register::<u8>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // C s_69_2: const #2u : u8
        let s_69_2: u8 = 2;
        // D s_69_3: cmp-lt s_69_1 s_69_2
        let s_69_3: bool = ((s_69_1) < (s_69_2));
        // N s_69_4: branch s_69_3 b74 b70
        if s_69_3 {
            return block_74(state, tracer, fn_state);
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
        // D s_70_1: write-var gs#124409 <= s_70_0
        fn_state.gs_124409 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#124409:u8
        let s_71_0: bool = fn_state.gs_124409;
        // N s_71_1: branch s_71_0 b73 b72
        if s_71_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var t:i
        let s_72_0: i128 = fn_state.t;
        // D s_72_1: call R_read(s_72_0)
        let s_72_1: u32 = R_read(state, tracer, s_72_0);
        // C s_72_2: const #64s : i
        let s_72_2: i128 = 64;
        // D s_72_3: cast zx s_72_1 -> bv
        let s_72_3: Bits = Bits::new(s_72_1 as u128, 32u16);
        // D s_72_4: bits-cast sx s_72_3 -> bv length s_72_2
        let s_72_4: Bits = s_72_3.sign_extend(s_72_2);
        // D s_72_5: cast reint s_72_4 -> u64
        let s_72_5: u64 = (s_72_4.value() as u64);
        // C s_72_6: const #() : ()
        let s_72_6: () = ();
        // S s_72_7: call PhysicalCountInt(s_72_6)
        let s_72_7: u64 = PhysicalCountInt(state, tracer, s_72_6);
        // D s_72_8: cast zx s_72_5 -> bv
        let s_72_8: Bits = Bits::new(s_72_5 as u128, 64u16);
        // S s_72_9: cast zx s_72_7 -> bv
        let s_72_9: Bits = Bits::new(s_72_7 as u128, 64u16);
        // D s_72_10: add s_72_8 s_72_9
        let s_72_10: Bits = (s_72_8 + s_72_9);
        // D s_72_11: cast reint s_72_10 -> u64
        let s_72_11: u64 = (s_72_10.value() as u64);
        // D s_72_12: call Mk_CNTV_CVAL_Type(s_72_11)
        let s_72_12: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_72_11,
        );
        // D s_72_13: call CNTV_CVAL_write(s_72_12)
        let s_72_13: () = CNTV_CVAL_write(state, tracer, s_72_12);
        // N s_72_14: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var t:i
        let s_73_0: i128 = fn_state.t;
        // D s_73_1: call R_read(s_73_0)
        let s_73_1: u32 = R_read(state, tracer, s_73_0);
        // C s_73_2: const #64s : i
        let s_73_2: i128 = 64;
        // D s_73_3: cast zx s_73_1 -> bv
        let s_73_3: Bits = Bits::new(s_73_1 as u128, 32u16);
        // D s_73_4: bits-cast sx s_73_3 -> bv length s_73_2
        let s_73_4: Bits = s_73_3.sign_extend(s_73_2);
        // D s_73_5: cast reint s_73_4 -> u64
        let s_73_5: u64 = (s_73_4.value() as u64);
        // C s_73_6: const #() : ()
        let s_73_6: () = ();
        // S s_73_7: call PhysicalCountInt(s_73_6)
        let s_73_7: u64 = PhysicalCountInt(state, tracer, s_73_6);
        // D s_73_8: cast zx s_73_5 -> bv
        let s_73_8: Bits = Bits::new(s_73_5 as u128, 64u16);
        // S s_73_9: cast zx s_73_7 -> bv
        let s_73_9: Bits = Bits::new(s_73_7 as u128, 64u16);
        // D s_73_10: add s_73_8 s_73_9
        let s_73_10: Bits = (s_73_8 + s_73_9);
        // D s_73_11: cast reint s_73_10 -> u64
        let s_73_11: u64 = (s_73_10.value() as u64);
        // C s_73_12: const #() : ()
        let s_73_12: () = ();
        // S s_73_13: call CNTVOFF_read(s_73_12)
        let s_73_13: u64 = CNTVOFF_read(state, tracer, s_73_12);
        // D s_73_14: cast zx s_73_11 -> bv
        let s_73_14: Bits = Bits::new(s_73_11 as u128, 64u16);
        // S s_73_15: cast zx s_73_13 -> bv
        let s_73_15: Bits = Bits::new(s_73_13 as u128, 64u16);
        // D s_73_16: sub s_73_14 s_73_15
        let s_73_16: Bits = ((s_73_14) - (s_73_15));
        // D s_73_17: cast reint s_73_16 -> u64
        let s_73_17: u64 = (s_73_16.value() as u64);
        // D s_73_18: call Mk_CNTV_CVAL_Type(s_73_17)
        let s_73_18: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_73_17,
        );
        // D s_73_19: call CNTV_CVAL_write(s_73_18)
        let s_73_19: () = CNTV_CVAL_write(state, tracer, s_73_18);
        // N s_73_20: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #432u : u32
        let s_74_0: u32 = 432;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: call ELUsingAArch32(s_74_1)
        let s_74_2: bool = ELUsingAArch32(state, tracer, s_74_1);
        // D s_74_3: write-var gs#124409 <= s_74_2
        fn_state.gs_124409 = s_74_2;
        // N s_74_4: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var t:i
        let s_75_0: i128 = fn_state.t;
        // D s_75_1: call R_read(s_75_0)
        let s_75_1: u32 = R_read(state, tracer, s_75_0);
        // C s_75_2: const #64s : i
        let s_75_2: i128 = 64;
        // D s_75_3: cast zx s_75_1 -> bv
        let s_75_3: Bits = Bits::new(s_75_1 as u128, 32u16);
        // D s_75_4: bits-cast sx s_75_3 -> bv length s_75_2
        let s_75_4: Bits = s_75_3.sign_extend(s_75_2);
        // D s_75_5: cast reint s_75_4 -> u64
        let s_75_5: u64 = (s_75_4.value() as u64);
        // C s_75_6: const #() : ()
        let s_75_6: () = ();
        // S s_75_7: call PhysicalCountInt(s_75_6)
        let s_75_7: u64 = PhysicalCountInt(state, tracer, s_75_6);
        // D s_75_8: cast zx s_75_5 -> bv
        let s_75_8: Bits = Bits::new(s_75_5 as u128, 64u16);
        // S s_75_9: cast zx s_75_7 -> bv
        let s_75_9: Bits = Bits::new(s_75_7 as u128, 64u16);
        // D s_75_10: add s_75_8 s_75_9
        let s_75_10: Bits = (s_75_8 + s_75_9);
        // D s_75_11: cast reint s_75_10 -> u64
        let s_75_11: u64 = (s_75_10.value() as u64);
        // D s_75_12: cast zx s_75_11 -> bv
        let s_75_12: Bits = Bits::new(s_75_11 as u128, 64u16);
        // C s_75_13: const #22400u : u32
        let s_75_13: u32 = 22400;
        // D s_75_14: read-reg s_75_13:u64
        let s_75_14: u64 = {
            let value = state.read_register::<u64>(s_75_13 as isize);
            tracer.read_register(s_75_13 as isize, value);
            value
        };
        // D s_75_15: cast zx s_75_14 -> bv
        let s_75_15: Bits = Bits::new(s_75_14 as u128, 64u16);
        // D s_75_16: sub s_75_12 s_75_15
        let s_75_16: Bits = ((s_75_12) - (s_75_15));
        // D s_75_17: cast reint s_75_16 -> u64
        let s_75_17: u64 = (s_75_16.value() as u64);
        // D s_75_18: call Mk_CNTV_CVAL_Type(s_75_17)
        let s_75_18: ProductType5c790c8ef59cc8b2 = Mk_CNTV_CVAL_Type(
            state,
            tracer,
            s_75_17,
        );
        // D s_75_19: call CNTV_CVAL_write(s_75_18)
        let s_75_19: () = CNTV_CVAL_write(state, tracer, s_75_18);
        // N s_75_20: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #432u : u32
        let s_76_0: u32 = 432;
        // D s_76_1: read-reg s_76_0:u8
        let s_76_1: u8 = {
            let value = state.read_register::<u8>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call ELUsingAArch32(s_76_1)
        let s_76_2: bool = ELUsingAArch32(state, tracer, s_76_1);
        // D s_76_3: not s_76_2
        let s_76_3: bool = !s_76_2;
        // D s_76_4: write-var gs#124408 <= s_76_3
        fn_state.gs_124408 = s_76_3;
        // N s_76_5: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var t:i
        let s_77_0: i128 = fn_state.t;
        // D s_77_1: call R_read(s_77_0)
        let s_77_1: u32 = R_read(state, tracer, s_77_0);
        // C s_77_2: const #64s : i
        let s_77_2: i128 = 64;
        // D s_77_3: cast zx s_77_1 -> bv
        let s_77_3: Bits = Bits::new(s_77_1 as u128, 32u16);
        // D s_77_4: bits-cast sx s_77_3 -> bv length s_77_2
        let s_77_4: Bits = s_77_3.sign_extend(s_77_2);
        // D s_77_5: cast reint s_77_4 -> u64
        let s_77_5: u64 = (s_77_4.value() as u64);
        // C s_77_6: const #() : ()
        let s_77_6: () = ();
        // S s_77_7: call PhysicalCountInt(s_77_6)
        let s_77_7: u64 = PhysicalCountInt(state, tracer, s_77_6);
        // D s_77_8: cast zx s_77_5 -> bv
        let s_77_8: Bits = Bits::new(s_77_5 as u128, 64u16);
        // S s_77_9: cast zx s_77_7 -> bv
        let s_77_9: Bits = Bits::new(s_77_7 as u128, 64u16);
        // D s_77_10: add s_77_8 s_77_9
        let s_77_10: Bits = (s_77_8 + s_77_9);
        // D s_77_11: cast reint s_77_10 -> u64
        let s_77_11: u64 = (s_77_10.value() as u64);
        // D s_77_12: call Mk_CNTHV_CVAL_EL2_Type(s_77_11)
        let s_77_12: ProductType5c790c8ef59cc8b2 = Mk_CNTHV_CVAL_EL2_Type(
            state,
            tracer,
            s_77_11,
        );
        // C s_77_13: const #103152u : u32
        let s_77_13: u32 = 103152;
        // N s_77_14: write-reg s_77_13 <= s_77_12
        let s_77_14: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_77_13 as isize, s_77_12);
            tracer.write_register(s_77_13 as isize, s_77_12);
        };
        // N s_77_15: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #90704u : u32
        let s_78_0: u32 = 90704;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_SCR_EL3_Type_NS(s_78_1)
        let s_78_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_78_1);
        // D s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // C s_78_4: const #1u : u8
        let s_78_4: bool = true;
        // C s_78_5: cast zx s_78_4 -> bv
        let s_78_5: Bits = Bits::new(s_78_4 as u128, 1u16);
        // D s_78_6: cmp-eq s_78_3 s_78_5
        let s_78_6: bool = ((s_78_3) == (s_78_5));
        // D s_78_7: write-var gs#124407 <= s_78_6
        fn_state.gs_124407 = s_78_6;
        // N s_78_8: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #102552u : u32
        let s_79_0: u32 = 102552;
        // D s_79_1: read-reg s_79_0:struct
        let s_79_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // D s_79_2: call _get_HCR_EL2_Type_E2H(s_79_1)
        let s_79_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_79_1);
        // C s_79_3: const #102552u : u32
        let s_79_3: u32 = 102552;
        // D s_79_4: read-reg s_79_3:struct
        let s_79_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_3 as isize);
            tracer.read_register(s_79_3 as isize, value);
            value
        };
        // D s_79_5: call _get_HCR_EL2_Type_TGE(s_79_4)
        let s_79_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_79_4);
        // D s_79_6: cast zx s_79_2 -> bv
        let s_79_6: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_7: cast zx s_79_5 -> bv
        let s_79_7: Bits = Bits::new(s_79_5 as u128, 1u16);
        // D s_79_8: cast reint s_79_6 -> u128
        let s_79_8: u128 = (s_79_6.value() as u128);
        // D s_79_9: size-of s_79_6
        let s_79_9: u16 = s_79_6.length();
        // D s_79_10: cast reint s_79_7 -> u128
        let s_79_10: u128 = (s_79_7.value() as u128);
        // D s_79_11: size-of s_79_7
        let s_79_11: u16 = s_79_7.length();
        // D s_79_12: lsl s_79_8 s_79_11
        let s_79_12: u128 = s_79_8 << s_79_11;
        // D s_79_13: or s_79_12 s_79_10
        let s_79_13: u128 = ((s_79_12) | (s_79_10));
        // D s_79_14: add s_79_9 s_79_11
        let s_79_14: u16 = (s_79_9 + s_79_11);
        // D s_79_15: create-bits s_79_13 s_79_14
        let s_79_15: Bits = Bits::new(s_79_13, s_79_14);
        // D s_79_16: cast reint s_79_15 -> u8
        let s_79_16: u8 = (s_79_15.value() as u8);
        // D s_79_17: cast zx s_79_16 -> bv
        let s_79_17: Bits = Bits::new(s_79_16 as u128, 2u16);
        // C s_79_18: const #3u : u8
        let s_79_18: u8 = 3;
        // C s_79_19: cast zx s_79_18 -> bv
        let s_79_19: Bits = Bits::new(s_79_18 as u128, 2u16);
        // D s_79_20: cmp-eq s_79_17 s_79_19
        let s_79_20: bool = ((s_79_17) == (s_79_19));
        // D s_79_21: write-var gs#124406 <= s_79_20
        fn_state.gs_124406 = s_79_20;
        // N s_79_22: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #432u : u32
        let s_80_0: u32 = 432;
        // D s_80_1: read-reg s_80_0:u8
        let s_80_1: u8 = {
            let value = state.read_register::<u8>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: call ELUsingAArch32(s_80_1)
        let s_80_2: bool = ELUsingAArch32(state, tracer, s_80_1);
        // D s_80_3: not s_80_2
        let s_80_3: bool = !s_80_2;
        // D s_80_4: write-var gs#124405 <= s_80_3
        fn_state.gs_124405 = s_80_3;
        // N s_80_5: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var t:i
        let s_81_0: i128 = fn_state.t;
        // D s_81_1: call R_read(s_81_0)
        let s_81_1: u32 = R_read(state, tracer, s_81_0);
        // C s_81_2: const #64s : i
        let s_81_2: i128 = 64;
        // D s_81_3: cast zx s_81_1 -> bv
        let s_81_3: Bits = Bits::new(s_81_1 as u128, 32u16);
        // D s_81_4: bits-cast sx s_81_3 -> bv length s_81_2
        let s_81_4: Bits = s_81_3.sign_extend(s_81_2);
        // D s_81_5: cast reint s_81_4 -> u64
        let s_81_5: u64 = (s_81_4.value() as u64);
        // C s_81_6: const #() : ()
        let s_81_6: () = ();
        // S s_81_7: call PhysicalCountInt(s_81_6)
        let s_81_7: u64 = PhysicalCountInt(state, tracer, s_81_6);
        // D s_81_8: cast zx s_81_5 -> bv
        let s_81_8: Bits = Bits::new(s_81_5 as u128, 64u16);
        // S s_81_9: cast zx s_81_7 -> bv
        let s_81_9: Bits = Bits::new(s_81_7 as u128, 64u16);
        // D s_81_10: add s_81_8 s_81_9
        let s_81_10: Bits = (s_81_8 + s_81_9);
        // D s_81_11: cast reint s_81_10 -> u64
        let s_81_11: u64 = (s_81_10.value() as u64);
        // D s_81_12: call Mk_CNTHVS_CVAL_EL2_Type(s_81_11)
        let s_81_12: ProductType5c790c8ef59cc8b2 = Mk_CNTHVS_CVAL_EL2_Type(
            state,
            tracer,
            s_81_11,
        );
        // C s_81_13: const #10064u : u32
        let s_81_13: u32 = 10064;
        // N s_81_14: write-reg s_81_13 <= s_81_12
        let s_81_14: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_81_13 as isize, s_81_12);
            tracer.write_register(s_81_13 as isize, s_81_12);
        };
        // N s_81_15: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #117u : u32
        let s_82_0: u32 = 117;
        // S s_82_1: call IsFeatureImplemented(s_82_0)
        let s_82_1: bool = IsFeatureImplemented(state, tracer, s_82_0);
        // D s_82_2: write-var gs#124404 <= s_82_1
        fn_state.gs_124404 = s_82_1;
        // N s_82_3: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #90704u : u32
        let s_83_0: u32 = 90704;
        // D s_83_1: read-reg s_83_0:struct
        let s_83_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call _get_SCR_EL3_Type_NS(s_83_1)
        let s_83_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_83_1);
        // D s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // C s_83_4: const #0u : u8
        let s_83_4: bool = false;
        // C s_83_5: cast zx s_83_4 -> bv
        let s_83_5: Bits = Bits::new(s_83_4 as u128, 1u16);
        // D s_83_6: cmp-eq s_83_3 s_83_5
        let s_83_6: bool = ((s_83_3) == (s_83_5));
        // D s_83_7: write-var gs#124403 <= s_83_6
        fn_state.gs_124403 = s_83_6;
        // N s_83_8: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #102552u : u32
        let s_84_0: u32 = 102552;
        // D s_84_1: read-reg s_84_0:struct
        let s_84_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call _get_HCR_EL2_Type_E2H(s_84_1)
        let s_84_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_84_1);
        // C s_84_3: const #102552u : u32
        let s_84_3: u32 = 102552;
        // D s_84_4: read-reg s_84_3:struct
        let s_84_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_3 as isize);
            tracer.read_register(s_84_3 as isize, value);
            value
        };
        // D s_84_5: call _get_HCR_EL2_Type_TGE(s_84_4)
        let s_84_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_84_4);
        // D s_84_6: cast zx s_84_2 -> bv
        let s_84_6: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_7: cast zx s_84_5 -> bv
        let s_84_7: Bits = Bits::new(s_84_5 as u128, 1u16);
        // D s_84_8: cast reint s_84_6 -> u128
        let s_84_8: u128 = (s_84_6.value() as u128);
        // D s_84_9: size-of s_84_6
        let s_84_9: u16 = s_84_6.length();
        // D s_84_10: cast reint s_84_7 -> u128
        let s_84_10: u128 = (s_84_7.value() as u128);
        // D s_84_11: size-of s_84_7
        let s_84_11: u16 = s_84_7.length();
        // D s_84_12: lsl s_84_8 s_84_11
        let s_84_12: u128 = s_84_8 << s_84_11;
        // D s_84_13: or s_84_12 s_84_10
        let s_84_13: u128 = ((s_84_12) | (s_84_10));
        // D s_84_14: add s_84_9 s_84_11
        let s_84_14: u16 = (s_84_9 + s_84_11);
        // D s_84_15: create-bits s_84_13 s_84_14
        let s_84_15: Bits = Bits::new(s_84_13, s_84_14);
        // D s_84_16: cast reint s_84_15 -> u8
        let s_84_16: u8 = (s_84_15.value() as u8);
        // D s_84_17: cast zx s_84_16 -> bv
        let s_84_17: Bits = Bits::new(s_84_16 as u128, 2u16);
        // C s_84_18: const #3u : u8
        let s_84_18: u8 = 3;
        // C s_84_19: cast zx s_84_18 -> bv
        let s_84_19: Bits = Bits::new(s_84_18 as u128, 2u16);
        // D s_84_20: cmp-eq s_84_17 s_84_19
        let s_84_20: bool = ((s_84_17) == (s_84_19));
        // D s_84_21: write-var gs#124402 <= s_84_20
        fn_state.gs_124402 = s_84_20;
        // N s_84_22: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #432u : u32
        let s_85_0: u32 = 432;
        // D s_85_1: read-reg s_85_0:u8
        let s_85_1: u8 = {
            let value = state.read_register::<u8>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call ELUsingAArch32(s_85_1)
        let s_85_2: bool = ELUsingAArch32(state, tracer, s_85_1);
        // D s_85_3: not s_85_2
        let s_85_3: bool = !s_85_2;
        // D s_85_4: write-var gs#124401 <= s_85_3
        fn_state.gs_124401 = s_85_3;
        // N s_85_5: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #3u : u8
        let s_86_0: u8 = 3;
        // C s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 8u16);
        // C s_86_2: cast zx s_86_1 -> i
        let s_86_2: i128 = (s_86_1.value() as i128);
        // C s_86_3: cast reint s_86_2 -> i64
        let s_86_3: i64 = (s_86_2 as i64);
        // C s_86_4: cast zx s_86_3 -> i
        let s_86_4: i128 = (i128::try_from(s_86_3).unwrap());
        // C s_86_5: const #432u : u32
        let s_86_5: u32 = 432;
        // D s_86_6: read-reg s_86_5:u8
        let s_86_6: u8 = {
            let value = state.read_register::<u8>(s_86_5 as isize);
            tracer.read_register(s_86_5 as isize, value);
            value
        };
        // D s_86_7: call AArch64_AArch32SystemAccessTrap(s_86_6, s_86_4)
        let s_86_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_86_6, s_86_4);
        // N s_86_8: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_87_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #1u : u8
        let s_87_2: bool = true;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // D s_87_5: write-var gs#124400 <= s_87_4
        fn_state.gs_124400 = s_87_4;
        // N s_87_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #102552u : u32
        let s_88_0: u32 = 102552;
        // D s_88_1: read-reg s_88_0:struct
        let s_88_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // D s_88_2: call _get_HCR_EL2_Type_E2H(s_88_1)
        let s_88_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_88_1);
        // C s_88_3: const #102552u : u32
        let s_88_3: u32 = 102552;
        // D s_88_4: read-reg s_88_3:struct
        let s_88_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_88_3 as isize);
            tracer.read_register(s_88_3 as isize, value);
            value
        };
        // D s_88_5: call _get_HCR_EL2_Type_TGE(s_88_4)
        let s_88_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_88_4);
        // D s_88_6: cast zx s_88_2 -> bv
        let s_88_6: Bits = Bits::new(s_88_2 as u128, 1u16);
        // D s_88_7: cast zx s_88_5 -> bv
        let s_88_7: Bits = Bits::new(s_88_5 as u128, 1u16);
        // D s_88_8: cast reint s_88_6 -> u128
        let s_88_8: u128 = (s_88_6.value() as u128);
        // D s_88_9: size-of s_88_6
        let s_88_9: u16 = s_88_6.length();
        // D s_88_10: cast reint s_88_7 -> u128
        let s_88_10: u128 = (s_88_7.value() as u128);
        // D s_88_11: size-of s_88_7
        let s_88_11: u16 = s_88_7.length();
        // D s_88_12: lsl s_88_8 s_88_11
        let s_88_12: u128 = s_88_8 << s_88_11;
        // D s_88_13: or s_88_12 s_88_10
        let s_88_13: u128 = ((s_88_12) | (s_88_10));
        // D s_88_14: add s_88_9 s_88_11
        let s_88_14: u16 = (s_88_9 + s_88_11);
        // D s_88_15: create-bits s_88_13 s_88_14
        let s_88_15: Bits = Bits::new(s_88_13, s_88_14);
        // D s_88_16: cast reint s_88_15 -> u8
        let s_88_16: u8 = (s_88_15.value() as u8);
        // D s_88_17: cast zx s_88_16 -> bv
        let s_88_17: Bits = Bits::new(s_88_16 as u128, 2u16);
        // C s_88_18: const #3u : u8
        let s_88_18: u8 = 3;
        // C s_88_19: cast zx s_88_18 -> bv
        let s_88_19: Bits = Bits::new(s_88_18 as u128, 2u16);
        // D s_88_20: cmp-ne s_88_17 s_88_19
        let s_88_20: bool = ((s_88_17) != (s_88_19));
        // D s_88_21: write-var gs#124399 <= s_88_20
        fn_state.gs_124399 = s_88_20;
        // N s_88_22: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #432u : u32
        let s_89_0: u32 = 432;
        // D s_89_1: read-reg s_89_0:u8
        let s_89_1: u8 = {
            let value = state.read_register::<u8>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // D s_89_2: call ELUsingAArch32(s_89_1)
        let s_89_2: bool = ELUsingAArch32(state, tracer, s_89_1);
        // D s_89_3: not s_89_2
        let s_89_3: bool = !s_89_2;
        // D s_89_4: write-var gs#124398 <= s_89_3
        fn_state.gs_124398 = s_89_3;
        // N s_89_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #3u : u8
        let s_90_0: u8 = 3;
        // C s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 8u16);
        // C s_90_2: cast zx s_90_1 -> i
        let s_90_2: i128 = (s_90_1.value() as i128);
        // C s_90_3: cast reint s_90_2 -> i64
        let s_90_3: i64 = (s_90_2 as i64);
        // C s_90_4: cast zx s_90_3 -> i
        let s_90_4: i128 = (i128::try_from(s_90_3).unwrap());
        // C s_90_5: const #432u : u32
        let s_90_5: u32 = 432;
        // D s_90_6: read-reg s_90_5:u8
        let s_90_6: u8 = {
            let value = state.read_register::<u8>(s_90_5 as isize);
            tracer.read_register(s_90_5 as isize, value);
            value
        };
        // D s_90_7: call AArch64_AArch32SystemAccessTrap(s_90_6, s_90_4)
        let s_90_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_90_6, s_90_4);
        // N s_90_8: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var __CNTHCTL_EL2_EL0VTEN:u8
        let s_91_0: bool = fn_state.u__CNTHCTL_EL2_EL0VTEN;
        // D s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 1u16);
        // C s_91_2: const #0u : u8
        let s_91_2: bool = false;
        // C s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_4: cmp-eq s_91_1 s_91_3
        let s_91_4: bool = ((s_91_1) == (s_91_3));
        // D s_91_5: write-var gs#124397 <= s_91_4
        fn_state.gs_124397 = s_91_4;
        // N s_91_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #102552u : u32
        let s_92_0: u32 = 102552;
        // D s_92_1: read-reg s_92_0:struct
        let s_92_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // D s_92_2: call _get_HCR_EL2_Type_E2H(s_92_1)
        let s_92_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_92_1);
        // C s_92_3: const #102552u : u32
        let s_92_3: u32 = 102552;
        // D s_92_4: read-reg s_92_3:struct
        let s_92_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_92_3 as isize);
            tracer.read_register(s_92_3 as isize, value);
            value
        };
        // D s_92_5: call _get_HCR_EL2_Type_TGE(s_92_4)
        let s_92_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_92_4);
        // D s_92_6: cast zx s_92_2 -> bv
        let s_92_6: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_7: cast zx s_92_5 -> bv
        let s_92_7: Bits = Bits::new(s_92_5 as u128, 1u16);
        // D s_92_8: cast reint s_92_6 -> u128
        let s_92_8: u128 = (s_92_6.value() as u128);
        // D s_92_9: size-of s_92_6
        let s_92_9: u16 = s_92_6.length();
        // D s_92_10: cast reint s_92_7 -> u128
        let s_92_10: u128 = (s_92_7.value() as u128);
        // D s_92_11: size-of s_92_7
        let s_92_11: u16 = s_92_7.length();
        // D s_92_12: lsl s_92_8 s_92_11
        let s_92_12: u128 = s_92_8 << s_92_11;
        // D s_92_13: or s_92_12 s_92_10
        let s_92_13: u128 = ((s_92_12) | (s_92_10));
        // D s_92_14: add s_92_9 s_92_11
        let s_92_14: u16 = (s_92_9 + s_92_11);
        // D s_92_15: create-bits s_92_13 s_92_14
        let s_92_15: Bits = Bits::new(s_92_13, s_92_14);
        // D s_92_16: cast reint s_92_15 -> u8
        let s_92_16: u8 = (s_92_15.value() as u8);
        // D s_92_17: cast zx s_92_16 -> bv
        let s_92_17: Bits = Bits::new(s_92_16 as u128, 2u16);
        // C s_92_18: const #3u : u8
        let s_92_18: u8 = 3;
        // C s_92_19: cast zx s_92_18 -> bv
        let s_92_19: Bits = Bits::new(s_92_18 as u128, 2u16);
        // D s_92_20: cmp-eq s_92_17 s_92_19
        let s_92_20: bool = ((s_92_17) == (s_92_19));
        // D s_92_21: write-var gs#124396 <= s_92_20
        fn_state.gs_124396 = s_92_20;
        // N s_92_22: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #432u : u32
        let s_93_0: u32 = 432;
        // D s_93_1: read-reg s_93_0:u8
        let s_93_1: u8 = {
            let value = state.read_register::<u8>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // D s_93_2: call ELUsingAArch32(s_93_1)
        let s_93_2: bool = ELUsingAArch32(state, tracer, s_93_1);
        // D s_93_3: not s_93_2
        let s_93_3: bool = !s_93_2;
        // D s_93_4: write-var gs#124395 <= s_93_3
        fn_state.gs_124395 = s_93_3;
        // N s_93_5: jump b38
        return block_38(state, tracer, fn_state);
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
        // N s_94_2: branch s_94_1 b110 b95
        if s_94_1 {
            return block_110(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#124415 <= s_95_0
        fn_state.gs_124415 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#124415:u8
        let s_96_0: bool = fn_state.gs_124415;
        // N s_96_1: branch s_96_0 b109 b97
        if s_96_0 {
            return block_109(state, tracer, fn_state);
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
        // D s_97_1: write-var gs#124416 <= s_97_0
        fn_state.gs_124416 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#124416:u8
        let s_98_0: bool = fn_state.gs_124416;
        // N s_98_1: branch s_98_0 b108 b99
        if s_98_0 {
            return block_108(state, tracer, fn_state);
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
        // N s_99_2: branch s_99_1 b107 b100
        if s_99_1 {
            return block_107(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#124417 <= s_100_0
        fn_state.gs_124417 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#124417:u8
        let s_101_0: bool = fn_state.gs_124417;
        // N s_101_1: branch s_101_0 b106 b102
        if s_101_0 {
            return block_106(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#124418 <= s_102_0
        fn_state.gs_124418 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#124418:u8
        let s_103_0: bool = fn_state.gs_124418;
        // N s_103_1: branch s_103_0 b105 b104
        if s_103_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_104_0: panic
        panic!("{:?}", ());
        // N s_104_1: return
        return;
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: u8 = 0;
        // C s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 8u16);
        // C s_105_2: cast zx s_105_1 -> i
        let s_105_2: i128 = (s_105_1.value() as i128);
        // C s_105_3: cast reint s_105_2 -> i64
        let s_105_3: i64 = (s_105_2 as i64);
        // C s_105_4: cast zx s_105_3 -> i
        let s_105_4: i128 = (i128::try_from(s_105_3).unwrap());
        // S s_105_5: call AArch32_TakeHypTrapException(s_105_4)
        let s_105_5: () = AArch32_TakeHypTrapException(state, tracer, s_105_4);
        // N s_105_6: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var __HCR_TGE:u8
        let s_106_0: bool = fn_state.u__HCR_TGE;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #1u : u8
        let s_106_2: bool = true;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#124418 <= s_106_4
        fn_state.gs_124418 = s_106_4;
        // N s_106_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #432u : u32
        let s_107_0: u32 = 432;
        // D s_107_1: read-reg s_107_0:u8
        let s_107_1: u8 = {
            let value = state.read_register::<u8>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: call ELUsingAArch32(s_107_1)
        let s_107_2: bool = ELUsingAArch32(state, tracer, s_107_1);
        // D s_107_3: write-var gs#124417 <= s_107_2
        fn_state.gs_124417 = s_107_2;
        // N s_107_4: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #3u : u8
        let s_108_0: u8 = 3;
        // C s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 8u16);
        // C s_108_2: cast zx s_108_1 -> i
        let s_108_2: i128 = (s_108_1.value() as i128);
        // C s_108_3: cast reint s_108_2 -> i64
        let s_108_3: i64 = (s_108_2 as i64);
        // C s_108_4: cast zx s_108_3 -> i
        let s_108_4: i128 = (i128::try_from(s_108_3).unwrap());
        // C s_108_5: const #432u : u32
        let s_108_5: u32 = 432;
        // D s_108_6: read-reg s_108_5:u8
        let s_108_6: u8 = {
            let value = state.read_register::<u8>(s_108_5 as isize);
            tracer.read_register(s_108_5 as isize, value);
            value
        };
        // D s_108_7: call AArch64_AArch32SystemAccessTrap(s_108_6, s_108_4)
        let s_108_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_108_6,
            s_108_4,
        );
        // N s_108_8: return
        return;
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var __HCR_EL2_TGE:u8
        let s_109_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 1u16);
        // C s_109_2: const #1u : u8
        let s_109_2: bool = true;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 1u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // D s_109_5: write-var gs#124416 <= s_109_4
        fn_state.gs_124416 = s_109_4;
        // N s_109_6: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #432u : u32
        let s_110_0: u32 = 432;
        // D s_110_1: read-reg s_110_0:u8
        let s_110_1: u8 = {
            let value = state.read_register::<u8>(s_110_0 as isize);
            tracer.read_register(s_110_0 as isize, value);
            value
        };
        // D s_110_2: call ELUsingAArch32(s_110_1)
        let s_110_2: bool = ELUsingAArch32(state, tracer, s_110_1);
        // D s_110_3: not s_110_2
        let s_110_3: bool = !s_110_2;
        // D s_110_4: write-var gs#124415 <= s_110_3
        fn_state.gs_124415 = s_110_3;
        // N s_110_5: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var __CNTKCTL_PL0VTEN:u8
        let s_111_0: bool = fn_state.u__CNTKCTL_PL0VTEN;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #0u : u8
        let s_111_2: bool = false;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // D s_111_5: write-var gs#124394 <= s_111_4
        fn_state.gs_124394 = s_111_4;
        // N s_111_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #() : ()
        let s_112_0: () = ();
        // S s_112_1: call EL2Enabled(s_112_0)
        let s_112_1: bool = EL2Enabled(state, tracer, s_112_0);
        // N s_112_2: branch s_112_1 b120 b113
        if s_112_1 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #0u : u8
        let s_113_0: bool = false;
        // D s_113_1: write-var gs#124419 <= s_113_0
        fn_state.gs_124419 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#124419:u8
        let s_114_0: bool = fn_state.gs_124419;
        // N s_114_1: branch s_114_0 b119 b115
        if s_114_0 {
            return block_119(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#124420 <= s_115_0
        fn_state.gs_124420 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#124420:u8
        let s_116_0: bool = fn_state.gs_124420;
        // N s_116_1: branch s_116_0 b118 b117
        if s_116_0 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #3u : u8
        let s_117_0: u8 = 3;
        // C s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 8u16);
        // C s_117_2: cast zx s_117_1 -> i
        let s_117_2: i128 = (s_117_1.value() as i128);
        // C s_117_3: cast reint s_117_2 -> i64
        let s_117_3: i64 = (s_117_2 as i64);
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // C s_117_5: const #440u : u32
        let s_117_5: u32 = 440;
        // D s_117_6: read-reg s_117_5:u8
        let s_117_6: u8 = {
            let value = state.read_register::<u8>(s_117_5 as isize);
            tracer.read_register(s_117_5 as isize, value);
            value
        };
        // D s_117_7: call AArch64_AArch32SystemAccessTrap(s_117_6, s_117_4)
        let s_117_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_117_6,
            s_117_4,
        );
        // N s_117_8: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #3u : u8
        let s_118_0: u8 = 3;
        // C s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 8u16);
        // C s_118_2: cast zx s_118_1 -> i
        let s_118_2: i128 = (s_118_1.value() as i128);
        // C s_118_3: cast reint s_118_2 -> i64
        let s_118_3: i64 = (s_118_2 as i64);
        // C s_118_4: cast zx s_118_3 -> i
        let s_118_4: i128 = (i128::try_from(s_118_3).unwrap());
        // C s_118_5: const #432u : u32
        let s_118_5: u32 = 432;
        // D s_118_6: read-reg s_118_5:u8
        let s_118_6: u8 = {
            let value = state.read_register::<u8>(s_118_5 as isize);
            tracer.read_register(s_118_5 as isize, value);
            value
        };
        // D s_118_7: call AArch64_AArch32SystemAccessTrap(s_118_6, s_118_4)
        let s_118_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_118_6,
            s_118_4,
        );
        // N s_118_8: return
        return;
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var __HCR_EL2_TGE:u8
        let s_119_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 1u16);
        // C s_119_2: const #1u : u8
        let s_119_2: bool = true;
        // C s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 1u16);
        // D s_119_4: cmp-eq s_119_1 s_119_3
        let s_119_4: bool = ((s_119_1) == (s_119_3));
        // D s_119_5: write-var gs#124420 <= s_119_4
        fn_state.gs_124420 = s_119_4;
        // N s_119_6: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #432u : u32
        let s_120_0: u32 = 432;
        // D s_120_1: read-reg s_120_0:u8
        let s_120_1: u8 = {
            let value = state.read_register::<u8>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // D s_120_2: call ELUsingAArch32(s_120_1)
        let s_120_2: bool = ELUsingAArch32(state, tracer, s_120_1);
        // D s_120_3: not s_120_2
        let s_120_3: bool = !s_120_2;
        // D s_120_4: write-var gs#124419 <= s_120_3
        fn_state.gs_124419 = s_120_3;
        // N s_120_5: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var __CNTKCTL_EL1_EL0VTEN:u8
        let s_121_0: bool = fn_state.u__CNTKCTL_EL1_EL0VTEN;
        // D s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 1u16);
        // C s_121_2: const #0u : u8
        let s_121_2: bool = false;
        // C s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 1u16);
        // D s_121_4: cmp-eq s_121_1 s_121_3
        let s_121_4: bool = ((s_121_1) == (s_121_3));
        // D s_121_5: write-var gs#124393 <= s_121_4
        fn_state.gs_124393 = s_121_4;
        // N s_121_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #() : ()
        let s_122_0: () = ();
        // S s_122_1: call EL2Enabled(s_122_0)
        let s_122_1: bool = EL2Enabled(state, tracer, s_122_0);
        // N s_122_2: branch s_122_1 b125 b123
        if s_122_1 {
            return block_125(state, tracer, fn_state);
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
        // D s_123_1: write-var gs#124391 <= s_123_0
        fn_state.gs_124391 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#124391:u8
        let s_124_0: bool = fn_state.gs_124391;
        // D s_124_1: not s_124_0
        let s_124_1: bool = !s_124_0;
        // D s_124_2: write-var gs#124392 <= s_124_1
        fn_state.gs_124392 = s_124_1;
        // N s_124_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #102552u : u32
        let s_125_0: u32 = 102552;
        // D s_125_1: read-reg s_125_0:struct
        let s_125_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // D s_125_2: call _get_HCR_EL2_Type_E2H(s_125_1)
        let s_125_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_125_1);
        // C s_125_3: const #102552u : u32
        let s_125_3: u32 = 102552;
        // D s_125_4: read-reg s_125_3:struct
        let s_125_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_3 as isize);
            tracer.read_register(s_125_3 as isize, value);
            value
        };
        // D s_125_5: call _get_HCR_EL2_Type_TGE(s_125_4)
        let s_125_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_125_4);
        // D s_125_6: cast zx s_125_2 -> bv
        let s_125_6: Bits = Bits::new(s_125_2 as u128, 1u16);
        // D s_125_7: cast zx s_125_5 -> bv
        let s_125_7: Bits = Bits::new(s_125_5 as u128, 1u16);
        // D s_125_8: cast reint s_125_6 -> u128
        let s_125_8: u128 = (s_125_6.value() as u128);
        // D s_125_9: size-of s_125_6
        let s_125_9: u16 = s_125_6.length();
        // D s_125_10: cast reint s_125_7 -> u128
        let s_125_10: u128 = (s_125_7.value() as u128);
        // D s_125_11: size-of s_125_7
        let s_125_11: u16 = s_125_7.length();
        // D s_125_12: lsl s_125_8 s_125_11
        let s_125_12: u128 = s_125_8 << s_125_11;
        // D s_125_13: or s_125_12 s_125_10
        let s_125_13: u128 = ((s_125_12) | (s_125_10));
        // D s_125_14: add s_125_9 s_125_11
        let s_125_14: u16 = (s_125_9 + s_125_11);
        // D s_125_15: create-bits s_125_13 s_125_14
        let s_125_15: Bits = Bits::new(s_125_13, s_125_14);
        // D s_125_16: cast reint s_125_15 -> u8
        let s_125_16: u8 = (s_125_15.value() as u8);
        // D s_125_17: cast zx s_125_16 -> bv
        let s_125_17: Bits = Bits::new(s_125_16 as u128, 2u16);
        // C s_125_18: const #3u : u8
        let s_125_18: u8 = 3;
        // C s_125_19: cast zx s_125_18 -> bv
        let s_125_19: Bits = Bits::new(s_125_18 as u128, 2u16);
        // D s_125_20: cmp-eq s_125_17 s_125_19
        let s_125_20: bool = ((s_125_17) == (s_125_19));
        // D s_125_21: write-var gs#124391 <= s_125_20
        fn_state.gs_124391 = s_125_20;
        // N s_125_22: jump b124
        return block_124(state, tracer, fn_state);
    }
}
