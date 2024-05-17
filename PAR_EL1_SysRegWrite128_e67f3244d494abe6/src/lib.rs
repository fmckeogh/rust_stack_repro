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
use u_get_HFGWTR_EL2_Type_PAR_EL1::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use PAR_EL1_write::*;
use u_get_SCR_EL3_Type_D128En::*;
use Halted::*;
use IsFeatureImplemented::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use Mk_PAR_EL1_Type::*;
use u__IMPDEF_boolean::*;
use u_get_HCRX_EL2_Type_D128En::*;
use PAR_EL1_read::*;
use IsHCRXEL2Enabled::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn PAR_EL1_SysRegWrite128_e67f3244d494abe6<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_239694: ProductType782ac6922b48c20d,
        gs_137624: bool,
        gs_137615: bool,
        u__HCRX_EL2_D128En: bool,
        gs_137619: bool,
        u__SCR_EL3_D128En: bool,
        ga_239725: ProductType782ac6922b48c20d,
        u__EDSCR_SDD: bool,
        ga_239693: ProductTypeb78df3ce1505b121,
        u__SCR_EL3_FGTEn: bool,
        ga_239735: ProductTypeb78df3ce1505b121,
        gs_137617: bool,
        gs_137637: bool,
        gs_137623: bool,
        gs_137618: bool,
        gs_137620: bool,
        gs_137599: bool,
        gs_137598: bool,
        gs_137600: bool,
        u__HFGWTR_EL2_PAR_EL1: bool,
        gs_137601: bool,
        ga_239736: ProductType782ac6922b48c20d,
        gs_137602: bool,
        gs_137625: bool,
        ga_239721: ProductType782ac6922b48c20d,
        ga_239698: ProductType782ac6922b48c20d,
        gs_137616: bool,
        u__PSTATE_EL: u8,
        gs_137621: bool,
        gs_137622: bool,
        ga_239740: ProductType782ac6922b48c20d,
        ga_239720: ProductTypeb78df3ce1505b121,
        gs_137614: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
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
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call EDSCR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_EDSCR_Type_SDD(s_0_4)
        let s_0_5: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_4);
        // D s_0_6: write-var __EDSCR_SDD <= s_0_5
        fn_state.u__EDSCR_SDD = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_D128En(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_D128En(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_D128En <= s_0_9
        fn_state.u__SCR_EL3_D128En = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_FGTEn(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_FGTEn <= s_0_13
        fn_state.u__SCR_EL3_FGTEn = s_0_13;
        // C s_0_15: const #100992u : u32
        let s_0_15: u32 = 100992;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HFGWTR_EL2_Type_PAR_EL1(s_0_16)
        let s_0_17: bool = u_get_HFGWTR_EL2_Type_PAR_EL1(state, tracer, s_0_16);
        // D s_0_18: write-var __HFGWTR_EL2_PAR_EL1 <= s_0_17
        fn_state.u__HFGWTR_EL2_PAR_EL1 = s_0_17;
        // C s_0_19: const #22528u : u32
        let s_0_19: u32 = 22528;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCRX_EL2_Type_D128En(s_0_20)
        let s_0_21: bool = u_get_HCRX_EL2_Type_D128En(state, tracer, s_0_20);
        // D s_0_22: write-var __HCRX_EL2_D128En <= s_0_21
        fn_state.u__HCRX_EL2_D128En = s_0_21;
        // D s_0_23: read-var __PSTATE_EL:u8
        let s_0_23: u8 = fn_state.u__PSTATE_EL;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 2u16);
        // C s_0_25: const #448u : u32
        let s_0_25: u32 = 448;
        // D s_0_26: read-reg s_0_25:u8
        let s_0_26: u8 = {
            let value = state.read_register::<u8>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // D s_0_28: cmp-eq s_0_24 s_0_27
        let s_0_28: bool = ((s_0_24) == (s_0_27));
        // N s_0_29: branch s_0_28 b78 b1
        if s_0_28 {
            return block_78(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b31 b2
        if s_1_5 {
            return block_31(state, tracer, fn_state);
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
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: add s_5_1 s_5_0
        let s_5_2: i128 = (s_5_1 + s_5_0);
        // C s_5_3: const #64s : i64
        let s_5_3: i64 = 64;
        // D s_5_4: call X_read(s_5_2, s_5_3)
        let s_5_4: Bits = X_read(state, tracer, s_5_2, s_5_3);
        // D s_5_5: cast reint s_5_4 -> u64
        let s_5_5: u64 = (s_5_4.value() as u64);
        // C s_5_6: const #64s : i64
        let s_5_6: i64 = 64;
        // D s_5_7: read-var t:i
        let s_5_7: i128 = fn_state.t;
        // D s_5_8: call X_read(s_5_7, s_5_6)
        let s_5_8: Bits = X_read(state, tracer, s_5_7, s_5_6);
        // D s_5_9: cast reint s_5_8 -> u64
        let s_5_9: u64 = (s_5_8.value() as u64);
        // D s_5_10: create-product struct = ["s_5_5", "s_5_9"]
        let s_5_10: ProductTypeb78df3ce1505b121 = ProductTypeb78df3ce1505b121 {
            _0: s_5_5,
            _1: s_5_9,
        };
        // D s_5_11: write-var ga#239735 <= s_5_10
        fn_state.ga_239735 = s_5_10;
        // D s_5_12: read-var ga#239735.0:struct
        let s_5_12: u64 = fn_state.ga_239735._0;
        // D s_5_13: read-var ga#239735.1:struct
        let s_5_13: u64 = fn_state.ga_239735._1;
        // C s_5_14: const #() : ()
        let s_5_14: () = ();
        // S s_5_15: call PAR_EL1_read(s_5_14)
        let s_5_15: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_5_14);
        // D s_5_16: write-var ga#239736 <= s_5_15
        fn_state.ga_239736 = s_5_15;
        // D s_5_17: read-var ga#239736.0:struct
        let s_5_17: u128 = fn_state.ga_239736._0;
        // C s_5_18: const #64s : i
        let s_5_18: i128 = 64;
        // D s_5_19: cast zx s_5_17 -> bv
        let s_5_19: Bits = Bits::new(s_5_17 as u128, 128u16);
        // D s_5_20: cast zx s_5_12 -> bv
        let s_5_20: Bits = Bits::new(s_5_12 as u128, 64u16);
        // C s_5_21: const #63s : i
        let s_5_21: i128 = 63;
        // C s_5_22: const #1u : u64
        let s_5_22: u64 = 1;
        // C s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 64u16);
        // C s_5_24: lsl s_5_23 s_5_21
        let s_5_24: Bits = s_5_23 << s_5_21;
        // C s_5_25: sub s_5_24 s_5_23
        let s_5_25: Bits = ((s_5_24) - (s_5_23));
        // D s_5_26: and s_5_20 s_5_25
        let s_5_26: Bits = ((s_5_20) & (s_5_25));
        // D s_5_27: lsl s_5_26 s_5_18
        let s_5_27: Bits = s_5_26 << s_5_18;
        // C s_5_28: lsl s_5_25 s_5_18
        let s_5_28: Bits = s_5_25 << s_5_18;
        // C s_5_29: cmpl s_5_28
        let s_5_29: Bits = !s_5_28;
        // D s_5_30: and s_5_19 s_5_29
        let s_5_30: Bits = ((s_5_19) & (s_5_29));
        // D s_5_31: or s_5_30 s_5_27
        let s_5_31: Bits = ((s_5_30) | (s_5_27));
        // D s_5_32: cast reint s_5_31 -> u128
        let s_5_32: u128 = (s_5_31.value() as u128);
        // D s_5_33: call Mk_PAR_EL1_Type(s_5_32)
        let s_5_33: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(state, tracer, s_5_32);
        // D s_5_34: call PAR_EL1_write(s_5_33)
        let s_5_34: () = PAR_EL1_write(state, tracer, s_5_33);
        // C s_5_35: const #() : ()
        let s_5_35: () = ();
        // S s_5_36: call PAR_EL1_read(s_5_35)
        let s_5_36: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_5_35);
        // D s_5_37: write-var ga#239740 <= s_5_36
        fn_state.ga_239740 = s_5_36;
        // D s_5_38: read-var ga#239740.0:struct
        let s_5_38: u128 = fn_state.ga_239740._0;
        // C s_5_39: const #0s : i
        let s_5_39: i128 = 0;
        // D s_5_40: cast zx s_5_38 -> bv
        let s_5_40: Bits = Bits::new(s_5_38 as u128, 128u16);
        // D s_5_41: cast zx s_5_13 -> bv
        let s_5_41: Bits = Bits::new(s_5_13 as u128, 64u16);
        // C s_5_42: const #63s : i
        let s_5_42: i128 = 63;
        // C s_5_43: const #1u : u64
        let s_5_43: u64 = 1;
        // C s_5_44: cast zx s_5_43 -> bv
        let s_5_44: Bits = Bits::new(s_5_43 as u128, 64u16);
        // C s_5_45: lsl s_5_44 s_5_42
        let s_5_45: Bits = s_5_44 << s_5_42;
        // C s_5_46: sub s_5_45 s_5_44
        let s_5_46: Bits = ((s_5_45) - (s_5_44));
        // D s_5_47: and s_5_41 s_5_46
        let s_5_47: Bits = ((s_5_41) & (s_5_46));
        // D s_5_48: lsl s_5_47 s_5_39
        let s_5_48: Bits = s_5_47 << s_5_39;
        // C s_5_49: lsl s_5_46 s_5_39
        let s_5_49: Bits = s_5_46 << s_5_39;
        // C s_5_50: cmpl s_5_49
        let s_5_50: Bits = !s_5_49;
        // D s_5_51: and s_5_40 s_5_50
        let s_5_51: Bits = ((s_5_40) & (s_5_50));
        // D s_5_52: or s_5_51 s_5_48
        let s_5_52: Bits = ((s_5_51) | (s_5_48));
        // D s_5_53: cast reint s_5_52 -> u128
        let s_5_53: u128 = (s_5_52.value() as u128);
        // D s_5_54: call Mk_PAR_EL1_Type(s_5_53)
        let s_5_54: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(state, tracer, s_5_53);
        // D s_5_55: call PAR_EL1_write(s_5_54)
        let s_5_55: () = PAR_EL1_write(state, tracer, s_5_54);
        // N s_5_56: return
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
        // N s_6_2: branch s_6_1 b30 b7
        if s_6_1 {
            return block_30(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#137598 <= s_7_0
        fn_state.gs_137598 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#137598:u8
        let s_8_0: bool = fn_state.gs_137598;
        // N s_8_1: branch s_8_0 b29 b9
        if s_8_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#137599 <= s_9_0
        fn_state.gs_137599 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#137599:u8
        let s_10_0: bool = fn_state.gs_137599;
        // N s_10_1: branch s_10_0 b28 b11
        if s_10_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#137600 <= s_11_0
        fn_state.gs_137600 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#137600:u8
        let s_12_0: bool = fn_state.gs_137600;
        // N s_12_1: branch s_12_0 b27 b13
        if s_12_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#137601 <= s_13_0
        fn_state.gs_137601 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#137601:u8
        let s_14_0: bool = fn_state.gs_137601;
        // N s_14_1: branch s_14_0 b26 b15
        if s_14_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#137602 <= s_16_0
        fn_state.gs_137602 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#137602:u8
        let s_17_0: bool = fn_state.gs_137602;
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
        // C s_18_0: const #1s : i
        let s_18_0: i128 = 1;
        // D s_18_1: read-var t:i
        let s_18_1: i128 = fn_state.t;
        // D s_18_2: add s_18_1 s_18_0
        let s_18_2: i128 = (s_18_1 + s_18_0);
        // C s_18_3: const #64s : i64
        let s_18_3: i64 = 64;
        // D s_18_4: call X_read(s_18_2, s_18_3)
        let s_18_4: Bits = X_read(state, tracer, s_18_2, s_18_3);
        // D s_18_5: cast reint s_18_4 -> u64
        let s_18_5: u64 = (s_18_4.value() as u64);
        // C s_18_6: const #64s : i64
        let s_18_6: i64 = 64;
        // D s_18_7: read-var t:i
        let s_18_7: i128 = fn_state.t;
        // D s_18_8: call X_read(s_18_7, s_18_6)
        let s_18_8: Bits = X_read(state, tracer, s_18_7, s_18_6);
        // D s_18_9: cast reint s_18_8 -> u64
        let s_18_9: u64 = (s_18_8.value() as u64);
        // D s_18_10: create-product struct = ["s_18_5", "s_18_9"]
        let s_18_10: ProductTypeb78df3ce1505b121 = ProductTypeb78df3ce1505b121 {
            _0: s_18_5,
            _1: s_18_9,
        };
        // D s_18_11: write-var ga#239720 <= s_18_10
        fn_state.ga_239720 = s_18_10;
        // D s_18_12: read-var ga#239720.0:struct
        let s_18_12: u64 = fn_state.ga_239720._0;
        // D s_18_13: read-var ga#239720.1:struct
        let s_18_13: u64 = fn_state.ga_239720._1;
        // C s_18_14: const #() : ()
        let s_18_14: () = ();
        // S s_18_15: call PAR_EL1_read(s_18_14)
        let s_18_15: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_18_14);
        // D s_18_16: write-var ga#239721 <= s_18_15
        fn_state.ga_239721 = s_18_15;
        // D s_18_17: read-var ga#239721.0:struct
        let s_18_17: u128 = fn_state.ga_239721._0;
        // C s_18_18: const #64s : i
        let s_18_18: i128 = 64;
        // D s_18_19: cast zx s_18_17 -> bv
        let s_18_19: Bits = Bits::new(s_18_17 as u128, 128u16);
        // D s_18_20: cast zx s_18_12 -> bv
        let s_18_20: Bits = Bits::new(s_18_12 as u128, 64u16);
        // C s_18_21: const #63s : i
        let s_18_21: i128 = 63;
        // C s_18_22: const #1u : u64
        let s_18_22: u64 = 1;
        // C s_18_23: cast zx s_18_22 -> bv
        let s_18_23: Bits = Bits::new(s_18_22 as u128, 64u16);
        // C s_18_24: lsl s_18_23 s_18_21
        let s_18_24: Bits = s_18_23 << s_18_21;
        // C s_18_25: sub s_18_24 s_18_23
        let s_18_25: Bits = ((s_18_24) - (s_18_23));
        // D s_18_26: and s_18_20 s_18_25
        let s_18_26: Bits = ((s_18_20) & (s_18_25));
        // D s_18_27: lsl s_18_26 s_18_18
        let s_18_27: Bits = s_18_26 << s_18_18;
        // C s_18_28: lsl s_18_25 s_18_18
        let s_18_28: Bits = s_18_25 << s_18_18;
        // C s_18_29: cmpl s_18_28
        let s_18_29: Bits = !s_18_28;
        // D s_18_30: and s_18_19 s_18_29
        let s_18_30: Bits = ((s_18_19) & (s_18_29));
        // D s_18_31: or s_18_30 s_18_27
        let s_18_31: Bits = ((s_18_30) | (s_18_27));
        // D s_18_32: cast reint s_18_31 -> u128
        let s_18_32: u128 = (s_18_31.value() as u128);
        // D s_18_33: call Mk_PAR_EL1_Type(s_18_32)
        let s_18_33: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_18_32,
        );
        // D s_18_34: call PAR_EL1_write(s_18_33)
        let s_18_34: () = PAR_EL1_write(state, tracer, s_18_33);
        // C s_18_35: const #() : ()
        let s_18_35: () = ();
        // S s_18_36: call PAR_EL1_read(s_18_35)
        let s_18_36: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_18_35);
        // D s_18_37: write-var ga#239725 <= s_18_36
        fn_state.ga_239725 = s_18_36;
        // D s_18_38: read-var ga#239725.0:struct
        let s_18_38: u128 = fn_state.ga_239725._0;
        // C s_18_39: const #0s : i
        let s_18_39: i128 = 0;
        // D s_18_40: cast zx s_18_38 -> bv
        let s_18_40: Bits = Bits::new(s_18_38 as u128, 128u16);
        // D s_18_41: cast zx s_18_13 -> bv
        let s_18_41: Bits = Bits::new(s_18_13 as u128, 64u16);
        // C s_18_42: const #63s : i
        let s_18_42: i128 = 63;
        // C s_18_43: const #1u : u64
        let s_18_43: u64 = 1;
        // C s_18_44: cast zx s_18_43 -> bv
        let s_18_44: Bits = Bits::new(s_18_43 as u128, 64u16);
        // C s_18_45: lsl s_18_44 s_18_42
        let s_18_45: Bits = s_18_44 << s_18_42;
        // C s_18_46: sub s_18_45 s_18_44
        let s_18_46: Bits = ((s_18_45) - (s_18_44));
        // D s_18_47: and s_18_41 s_18_46
        let s_18_47: Bits = ((s_18_41) & (s_18_46));
        // D s_18_48: lsl s_18_47 s_18_39
        let s_18_48: Bits = s_18_47 << s_18_39;
        // C s_18_49: lsl s_18_46 s_18_39
        let s_18_49: Bits = s_18_46 << s_18_39;
        // C s_18_50: cmpl s_18_49
        let s_18_50: Bits = !s_18_49;
        // D s_18_51: and s_18_40 s_18_50
        let s_18_51: Bits = ((s_18_40) & (s_18_50));
        // D s_18_52: or s_18_51 s_18_48
        let s_18_52: Bits = ((s_18_51) | (s_18_48));
        // D s_18_53: cast reint s_18_52 -> u128
        let s_18_53: u128 = (s_18_52.value() as u128);
        // D s_18_54: call Mk_PAR_EL1_Type(s_18_53)
        let s_18_54: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_18_53,
        );
        // D s_18_55: call PAR_EL1_write(s_18_54)
        let s_18_55: () = PAR_EL1_write(state, tracer, s_18_54);
        // N s_18_56: return
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
        // D s_20_1: write-var gs#137614 <= s_20_0
        fn_state.gs_137614 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#137614:u8
        let s_21_0: bool = fn_state.gs_137614;
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
        // C s_22_0: const #20u : u8
        let s_22_0: u8 = 20;
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
        // D s_24_0: read-var __EDSCR_SDD:u8
        let s_24_0: bool = fn_state.u__EDSCR_SDD;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#137614 <= s_24_4
        fn_state.gs_137614 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __SCR_EL3_D128En:u8
        let s_25_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#137602 <= s_25_4
        fn_state.gs_137602 = s_25_4;
        // N s_25_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __SCR_EL3_D128En:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#137601 <= s_27_4
        fn_state.gs_137601 = s_27_4;
        // N s_27_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_28_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_28_1: call __IMPDEF_boolean(s_28_0)
        let s_28_1: bool = u__IMPDEF_boolean(state, tracer, s_28_0);
        // D s_28_2: write-var gs#137600 <= s_28_1
        fn_state.gs_137600 = s_28_1;
        // N s_28_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __EDSCR_SDD:u8
        let s_29_0: bool = fn_state.u__EDSCR_SDD;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#137599 <= s_29_4
        fn_state.gs_137599 = s_29_4;
        // N s_29_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // D s_30_4: write-var gs#137598 <= s_30_3
        fn_state.gs_137598 = s_30_3;
        // N s_30_5: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_31_2: branch s_31_1 b77 b32
        if s_31_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#137615 <= s_32_0
        fn_state.gs_137615 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#137615:u8
        let s_33_0: bool = fn_state.gs_137615;
        // N s_33_1: branch s_33_0 b76 b34
        if s_33_0 {
            return block_76(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#137616 <= s_34_0
        fn_state.gs_137616 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#137616:u8
        let s_35_0: bool = fn_state.gs_137616;
        // N s_35_1: branch s_35_0 b75 b36
        if s_35_0 {
            return block_75(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#137617 <= s_36_0
        fn_state.gs_137617 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#137617:u8
        let s_37_0: bool = fn_state.gs_137617;
        // N s_37_1: branch s_37_0 b74 b38
        if s_37_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#137618 <= s_38_0
        fn_state.gs_137618 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#137618:u8
        let s_39_0: bool = fn_state.gs_137618;
        // N s_39_1: branch s_39_0 b73 b40
        if s_39_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EL2Enabled(s_40_0)
        let s_40_1: bool = EL2Enabled(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b72 b41
        if s_40_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#137619 <= s_41_0
        fn_state.gs_137619 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#137619:u8
        let s_42_0: bool = fn_state.gs_137619;
        // N s_42_1: branch s_42_0 b68 b43
        if s_42_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#137621 <= s_43_0
        fn_state.gs_137621 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#137621:u8
        let s_44_0: bool = fn_state.gs_137621;
        // N s_44_1: branch s_44_0 b67 b45
        if s_44_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#137622 <= s_45_0
        fn_state.gs_137622 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#137622:u8
        let s_46_0: bool = fn_state.gs_137622;
        // N s_46_1: branch s_46_0 b66 b47
        if s_46_0 {
            return block_66(state, tracer, fn_state);
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
        // S s_47_1: call EL2Enabled(s_47_0)
        let s_47_1: bool = EL2Enabled(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b62 b48
        if s_47_1 {
            return block_62(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#137624 <= s_48_0
        fn_state.gs_137624 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#137624:u8
        let s_49_0: bool = fn_state.gs_137624;
        // N s_49_1: branch s_49_0 b61 b50
        if s_49_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #424u : u32
        let s_50_0: u32 = 424;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // C s_50_2: const #2u : u8
        let s_50_2: u8 = 2;
        // D s_50_3: cmp-lt s_50_1 s_50_2
        let s_50_3: bool = ((s_50_1) < (s_50_2));
        // N s_50_4: branch s_50_3 b60 b51
        if s_50_3 {
            return block_60(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#137625 <= s_51_0
        fn_state.gs_137625 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#137625:u8
        let s_52_0: bool = fn_state.gs_137625;
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
        // C s_53_0: const #1s : i
        let s_53_0: i128 = 1;
        // D s_53_1: read-var t:i
        let s_53_1: i128 = fn_state.t;
        // D s_53_2: add s_53_1 s_53_0
        let s_53_2: i128 = (s_53_1 + s_53_0);
        // C s_53_3: const #64s : i64
        let s_53_3: i64 = 64;
        // D s_53_4: call X_read(s_53_2, s_53_3)
        let s_53_4: Bits = X_read(state, tracer, s_53_2, s_53_3);
        // D s_53_5: cast reint s_53_4 -> u64
        let s_53_5: u64 = (s_53_4.value() as u64);
        // C s_53_6: const #64s : i64
        let s_53_6: i64 = 64;
        // D s_53_7: read-var t:i
        let s_53_7: i128 = fn_state.t;
        // D s_53_8: call X_read(s_53_7, s_53_6)
        let s_53_8: Bits = X_read(state, tracer, s_53_7, s_53_6);
        // D s_53_9: cast reint s_53_8 -> u64
        let s_53_9: u64 = (s_53_8.value() as u64);
        // D s_53_10: create-product struct = ["s_53_5", "s_53_9"]
        let s_53_10: ProductTypeb78df3ce1505b121 = ProductTypeb78df3ce1505b121 {
            _0: s_53_5,
            _1: s_53_9,
        };
        // D s_53_11: write-var ga#239693 <= s_53_10
        fn_state.ga_239693 = s_53_10;
        // D s_53_12: read-var ga#239693.0:struct
        let s_53_12: u64 = fn_state.ga_239693._0;
        // D s_53_13: read-var ga#239693.1:struct
        let s_53_13: u64 = fn_state.ga_239693._1;
        // C s_53_14: const #() : ()
        let s_53_14: () = ();
        // S s_53_15: call PAR_EL1_read(s_53_14)
        let s_53_15: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_53_14);
        // D s_53_16: write-var ga#239694 <= s_53_15
        fn_state.ga_239694 = s_53_15;
        // D s_53_17: read-var ga#239694.0:struct
        let s_53_17: u128 = fn_state.ga_239694._0;
        // C s_53_18: const #64s : i
        let s_53_18: i128 = 64;
        // D s_53_19: cast zx s_53_17 -> bv
        let s_53_19: Bits = Bits::new(s_53_17 as u128, 128u16);
        // D s_53_20: cast zx s_53_12 -> bv
        let s_53_20: Bits = Bits::new(s_53_12 as u128, 64u16);
        // C s_53_21: const #63s : i
        let s_53_21: i128 = 63;
        // C s_53_22: const #1u : u64
        let s_53_22: u64 = 1;
        // C s_53_23: cast zx s_53_22 -> bv
        let s_53_23: Bits = Bits::new(s_53_22 as u128, 64u16);
        // C s_53_24: lsl s_53_23 s_53_21
        let s_53_24: Bits = s_53_23 << s_53_21;
        // C s_53_25: sub s_53_24 s_53_23
        let s_53_25: Bits = ((s_53_24) - (s_53_23));
        // D s_53_26: and s_53_20 s_53_25
        let s_53_26: Bits = ((s_53_20) & (s_53_25));
        // D s_53_27: lsl s_53_26 s_53_18
        let s_53_27: Bits = s_53_26 << s_53_18;
        // C s_53_28: lsl s_53_25 s_53_18
        let s_53_28: Bits = s_53_25 << s_53_18;
        // C s_53_29: cmpl s_53_28
        let s_53_29: Bits = !s_53_28;
        // D s_53_30: and s_53_19 s_53_29
        let s_53_30: Bits = ((s_53_19) & (s_53_29));
        // D s_53_31: or s_53_30 s_53_27
        let s_53_31: Bits = ((s_53_30) | (s_53_27));
        // D s_53_32: cast reint s_53_31 -> u128
        let s_53_32: u128 = (s_53_31.value() as u128);
        // D s_53_33: call Mk_PAR_EL1_Type(s_53_32)
        let s_53_33: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_53_32,
        );
        // D s_53_34: call PAR_EL1_write(s_53_33)
        let s_53_34: () = PAR_EL1_write(state, tracer, s_53_33);
        // C s_53_35: const #() : ()
        let s_53_35: () = ();
        // S s_53_36: call PAR_EL1_read(s_53_35)
        let s_53_36: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_53_35);
        // D s_53_37: write-var ga#239698 <= s_53_36
        fn_state.ga_239698 = s_53_36;
        // D s_53_38: read-var ga#239698.0:struct
        let s_53_38: u128 = fn_state.ga_239698._0;
        // C s_53_39: const #0s : i
        let s_53_39: i128 = 0;
        // D s_53_40: cast zx s_53_38 -> bv
        let s_53_40: Bits = Bits::new(s_53_38 as u128, 128u16);
        // D s_53_41: cast zx s_53_13 -> bv
        let s_53_41: Bits = Bits::new(s_53_13 as u128, 64u16);
        // C s_53_42: const #63s : i
        let s_53_42: i128 = 63;
        // C s_53_43: const #1u : u64
        let s_53_43: u64 = 1;
        // C s_53_44: cast zx s_53_43 -> bv
        let s_53_44: Bits = Bits::new(s_53_43 as u128, 64u16);
        // C s_53_45: lsl s_53_44 s_53_42
        let s_53_45: Bits = s_53_44 << s_53_42;
        // C s_53_46: sub s_53_45 s_53_44
        let s_53_46: Bits = ((s_53_45) - (s_53_44));
        // D s_53_47: and s_53_41 s_53_46
        let s_53_47: Bits = ((s_53_41) & (s_53_46));
        // D s_53_48: lsl s_53_47 s_53_39
        let s_53_48: Bits = s_53_47 << s_53_39;
        // C s_53_49: lsl s_53_46 s_53_39
        let s_53_49: Bits = s_53_46 << s_53_39;
        // C s_53_50: cmpl s_53_49
        let s_53_50: Bits = !s_53_49;
        // D s_53_51: and s_53_40 s_53_50
        let s_53_51: Bits = ((s_53_40) & (s_53_50));
        // D s_53_52: or s_53_51 s_53_48
        let s_53_52: Bits = ((s_53_51) | (s_53_48));
        // D s_53_53: cast reint s_53_52 -> u128
        let s_53_53: u128 = (s_53_52.value() as u128);
        // D s_53_54: call Mk_PAR_EL1_Type(s_53_53)
        let s_53_54: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_53_53,
        );
        // D s_53_55: call PAR_EL1_write(s_53_54)
        let s_53_55: () = PAR_EL1_write(state, tracer, s_53_54);
        // N s_53_56: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call Halted(s_54_0)
        let s_54_1: bool = Halted(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b59 b55
        if s_54_1 {
            return block_59(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#137637 <= s_55_0
        fn_state.gs_137637 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#137637:u8
        let s_56_0: bool = fn_state.gs_137637;
        // N s_56_1: branch s_56_0 b58 b57
        if s_56_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #20u : u8
        let s_57_0: u8 = 20;
        // C s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 8u16);
        // C s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (s_57_1.value() as i128);
        // C s_57_3: cast reint s_57_2 -> i64
        let s_57_3: i64 = (s_57_2 as i64);
        // C s_57_4: cast zx s_57_3 -> i
        let s_57_4: i128 = (i128::try_from(s_57_3).unwrap());
        // C s_57_5: const #424u : u32
        let s_57_5: u32 = 424;
        // D s_57_6: read-reg s_57_5:u8
        let s_57_6: u8 = {
            let value = state.read_register::<u8>(s_57_5 as isize);
            tracer.read_register(s_57_5 as isize, value);
            value
        };
        // D s_57_7: call AArch64_SystemAccessTrap(s_57_6, s_57_4)
        let s_57_7: () = AArch64_SystemAccessTrap(state, tracer, s_57_6, s_57_4);
        // N s_57_8: return
        return;
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
        // D s_59_0: read-var __EDSCR_SDD:u8
        let s_59_0: bool = fn_state.u__EDSCR_SDD;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #1u : u8
        let s_59_2: bool = true;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#137637 <= s_59_4
        fn_state.gs_137637 = s_59_4;
        // N s_59_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __SCR_EL3_D128En:u8
        let s_60_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #0u : u8
        let s_60_2: bool = false;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#137625 <= s_60_4
        fn_state.gs_137625 = s_60_4;
        // N s_60_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #20u : u8
        let s_61_0: u8 = 20;
        // C s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 8u16);
        // C s_61_2: cast zx s_61_1 -> i
        let s_61_2: i128 = (s_61_1.value() as i128);
        // C s_61_3: cast reint s_61_2 -> i64
        let s_61_3: i64 = (s_61_2 as i64);
        // C s_61_4: cast zx s_61_3 -> i
        let s_61_4: i128 = (i128::try_from(s_61_3).unwrap());
        // C s_61_5: const #432u : u32
        let s_61_5: u32 = 432;
        // D s_61_6: read-reg s_61_5:u8
        let s_61_6: u8 = {
            let value = state.read_register::<u8>(s_61_5 as isize);
            tracer.read_register(s_61_5 as isize, value);
            value
        };
        // D s_61_7: call AArch64_SystemAccessTrap(s_61_6, s_61_4)
        let s_61_7: () = AArch64_SystemAccessTrap(state, tracer, s_61_6, s_61_4);
        // N s_61_8: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call IsHCRXEL2Enabled(s_62_0)
        let s_62_1: bool = IsHCRXEL2Enabled(state, tracer, s_62_0);
        // S s_62_2: not s_62_1
        let s_62_2: bool = !s_62_1;
        // N s_62_3: branch s_62_2 b65 b63
        if s_62_2 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __HCRX_EL2_D128En:u8
        let s_63_0: bool = fn_state.u__HCRX_EL2_D128En;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #0u : u8
        let s_63_2: bool = false;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#137623 <= s_63_4
        fn_state.gs_137623 = s_63_4;
        // N s_63_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#137623:u8
        let s_64_0: bool = fn_state.gs_137623;
        // D s_64_1: write-var gs#137624 <= s_64_0
        fn_state.gs_137624 = s_64_0;
        // N s_64_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#137623 <= s_65_0
        fn_state.gs_137623 = s_65_0;
        // N s_65_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #20u : u8
        let s_66_0: u8 = 20;
        // C s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 8u16);
        // C s_66_2: cast zx s_66_1 -> i
        let s_66_2: i128 = (s_66_1.value() as i128);
        // C s_66_3: cast reint s_66_2 -> i64
        let s_66_3: i64 = (s_66_2 as i64);
        // C s_66_4: cast zx s_66_3 -> i
        let s_66_4: i128 = (i128::try_from(s_66_3).unwrap());
        // C s_66_5: const #432u : u32
        let s_66_5: u32 = 432;
        // D s_66_6: read-reg s_66_5:u8
        let s_66_6: u8 = {
            let value = state.read_register::<u8>(s_66_5 as isize);
            tracer.read_register(s_66_5 as isize, value);
            value
        };
        // D s_66_7: call AArch64_SystemAccessTrap(s_66_6, s_66_4)
        let s_66_7: () = AArch64_SystemAccessTrap(state, tracer, s_66_6, s_66_4);
        // N s_66_8: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __HFGWTR_EL2_PAR_EL1:u8
        let s_67_0: bool = fn_state.u__HFGWTR_EL2_PAR_EL1;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#137622 <= s_67_4
        fn_state.gs_137622 = s_67_4;
        // N s_67_6: jump b46
        return block_46(state, tracer, fn_state);
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
        // C s_68_2: const #2u : u8
        let s_68_2: u8 = 2;
        // D s_68_3: cmp-lt s_68_1 s_68_2
        let s_68_3: bool = ((s_68_1) < (s_68_2));
        // D s_68_4: not s_68_3
        let s_68_4: bool = !s_68_3;
        // N s_68_5: branch s_68_4 b71 b69
        if s_68_4 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __SCR_EL3_FGTEn:u8
        let s_69_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#137620 <= s_69_4
        fn_state.gs_137620 = s_69_4;
        // N s_69_6: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#137620:u8
        let s_70_0: bool = fn_state.gs_137620;
        // D s_70_1: write-var gs#137621 <= s_70_0
        fn_state.gs_137621 = s_70_0;
        // N s_70_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#137620 <= s_71_0
        fn_state.gs_137620 = s_71_0;
        // N s_71_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #146u : u32
        let s_72_0: u32 = 146;
        // S s_72_1: call IsFeatureImplemented(s_72_0)
        let s_72_1: bool = IsFeatureImplemented(state, tracer, s_72_0);
        // D s_72_2: write-var gs#137619 <= s_72_1
        fn_state.gs_137619 = s_72_1;
        // N s_72_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_73_0: panic
        panic!("{:?}", ());
        // N s_73_1: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var __SCR_EL3_D128En:u8
        let s_74_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #0u : u8
        let s_74_2: bool = false;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#137618 <= s_74_4
        fn_state.gs_137618 = s_74_4;
        // N s_74_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_75_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_75_1: call __IMPDEF_boolean(s_75_0)
        let s_75_1: bool = u__IMPDEF_boolean(state, tracer, s_75_0);
        // D s_75_2: write-var gs#137617 <= s_75_1
        fn_state.gs_137617 = s_75_1;
        // N s_75_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __EDSCR_SDD:u8
        let s_76_0: bool = fn_state.u__EDSCR_SDD;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #1u : u8
        let s_76_2: bool = true;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#137616 <= s_76_4
        fn_state.gs_137616 = s_76_4;
        // N s_76_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #424u : u32
        let s_77_0: u32 = 424;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: u8 = {
            let value = state.read_register::<u8>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // C s_77_2: const #2u : u8
        let s_77_2: u8 = 2;
        // D s_77_3: cmp-lt s_77_1 s_77_2
        let s_77_3: bool = ((s_77_1) < (s_77_2));
        // D s_77_4: write-var gs#137615 <= s_77_3
        fn_state.gs_137615 = s_77_3;
        // N s_77_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_78_0: panic
        panic!("{:?}", ());
        // N s_78_1: return
        return;
    }
}
