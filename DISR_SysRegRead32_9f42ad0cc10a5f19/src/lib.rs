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
use u_get_HCR_Type_AMO::*;
use EL2Enabled::*;
use u__get_VDISR_EL2::*;
use DISR_read::*;
use u__get_VDISR::*;
use HCR_read::*;
use Halted::*;
use VDISR_read::*;
use AArch32_TakeHypTrapException::*;
use Zeros::*;
use u_get_HSTR_EL2_Type_T12::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_SCR_EL3_Type_EA::*;
use u_get_SCR_Type_EA::*;
use u_get_HCR_EL2_Type_AMO::*;
use u__get_DISR::*;
use u_get_HSTR_Type_T12::*;
use common::*;
pub fn DISR_SysRegRead32_9f42ad0cc10a5f19<T: Tracer>(
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
        gs_110466: bool,
        gs_110467: bool,
        gs_110470: bool,
        gs_110461: bool,
        gs_110465: bool,
        u__HSTR_T12: bool,
        gs_110464: bool,
        ga_176714: ProductType700c18a878c5601b,
        gs_110456: bool,
        ga_176747: ProductType700c18a878c5601b,
        gs_110463: bool,
        ga_176751: ProductType700c18a878c5601b,
        gs_110472: bool,
        gs_110462: bool,
        u__SCR_EA: bool,
        ga_176707: ProductType5c790c8ef59cc8b2,
        u__HCR_EL2_AMO: bool,
        gs_110471: bool,
        ga_176730: ProductType700c18a878c5601b,
        u__SCR_EL3_EA: bool,
        u__HSTR_EL2_T12: bool,
        gs_110454: bool,
        gs_110468: bool,
        gs_110453: bool,
        gs_110452: bool,
        gs_110455: bool,
        gs_110473: bool,
        u__PSTATE_EL: u8,
        u__HCR_AMO: bool,
        gs_110457: bool,
        gs_110460: bool,
        gs_110469: bool,
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
        // C s_0_3: const #104936u : u32
        let s_0_3: u32 = 104936;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HSTR_EL2_Type_T12(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T12(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T12 <= s_0_5
        fn_state.u__HSTR_EL2_T12 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T12(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T12(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T12 <= s_0_9
        fn_state.u__HSTR_T12 = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_AMO(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_AMO <= s_0_13
        fn_state.u__HCR_EL2_AMO = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_AMO(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_AMO(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_AMO <= s_0_17
        fn_state.u__HCR_AMO = s_0_17;
        // C s_0_19: const #90704u : u32
        let s_0_19: u32 = 90704;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCR_EL3_Type_EA(s_0_20)
        let s_0_21: bool = u_get_SCR_EL3_Type_EA(state, tracer, s_0_20);
        // D s_0_22: write-var __SCR_EL3_EA <= s_0_21
        fn_state.u__SCR_EL3_EA = s_0_21;
        // C s_0_23: const #20920u : u32
        let s_0_23: u32 = 20920;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SCR_Type_EA(s_0_24)
        let s_0_25: bool = u_get_SCR_Type_EA(state, tracer, s_0_24);
        // D s_0_26: write-var __SCR_EA <= s_0_25
        fn_state.u__SCR_EA = s_0_25;
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
        // N s_0_33: branch s_0_32 b84 b1
        if s_0_32 {
            return block_84(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b29 b2
        if s_1_5 {
            return block_29(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call DISR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = DISR_read(state, tracer, s_5_0);
        // S s_5_2: call __get_DISR(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = u__get_DISR(state, tracer, s_5_1);
        // D s_5_3: write-var ga#176751 <= s_5_2
        fn_state.ga_176751 = s_5_2;
        // D s_5_4: read-var ga#176751.0:struct
        let s_5_4: u32 = fn_state.ga_176751._0;
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: call R_set(s_5_5, s_5_4)
        let s_5_6: () = R_set(state, tracer, s_5_5, s_5_4);
        // N s_5_7: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #424u : u32
        let s_6_0: u32 = 424;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // D s_6_3: cmp-lt s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) < (s_6_2));
        // N s_6_4: branch s_6_3 b28 b7
        if s_6_3 {
            return block_28(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#110452 <= s_7_0
        fn_state.gs_110452 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#110452:u8
        let s_8_0: bool = fn_state.gs_110452;
        // N s_8_1: branch s_8_0 b27 b9
        if s_8_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#110453 <= s_9_0
        fn_state.gs_110453 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#110453:u8
        let s_10_0: bool = fn_state.gs_110453;
        // N s_10_1: branch s_10_0 b26 b11
        if s_10_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#110454 <= s_11_0
        fn_state.gs_110454 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#110454:u8
        let s_12_0: bool = fn_state.gs_110454;
        // N s_12_1: branch s_12_0 b25 b13
        if s_12_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
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
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // D s_13_3: cmp-lt s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) < (s_13_2));
        // N s_13_4: branch s_13_3 b24 b14
        if s_13_3 {
            return block_24(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#110455 <= s_14_0
        fn_state.gs_110455 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#110455:u8
        let s_15_0: bool = fn_state.gs_110455;
        // N s_15_1: branch s_15_0 b23 b16
        if s_15_0 {
            return block_23(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#110456 <= s_16_0
        fn_state.gs_110456 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#110456:u8
        let s_17_0: bool = fn_state.gs_110456;
        // N s_17_1: branch s_17_0 b22 b18
        if s_17_0 {
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
        // D s_18_1: write-var gs#110457 <= s_18_0
        fn_state.gs_110457 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#110457:u8
        let s_19_0: bool = fn_state.gs_110457;
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
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call DISR_read(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = DISR_read(state, tracer, s_20_0);
        // S s_20_2: call __get_DISR(s_20_1)
        let s_20_2: ProductType700c18a878c5601b = u__get_DISR(state, tracer, s_20_1);
        // D s_20_3: write-var ga#176747 <= s_20_2
        fn_state.ga_176747 = s_20_2;
        // D s_20_4: read-var ga#176747.0:struct
        let s_20_4: u32 = fn_state.ga_176747._0;
        // D s_20_5: read-var t:i
        let s_20_5: i128 = fn_state.t;
        // D s_20_6: call R_set(s_20_5, s_20_4)
        let s_20_6: () = R_set(state, tracer, s_20_5, s_20_4);
        // N s_20_7: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #32s : i
        let s_21_0: i128 = 32;
        // S s_21_1: call Zeros(s_21_0)
        let s_21_1: Bits = Zeros(state, tracer, s_21_0);
        // S s_21_2: cast reint s_21_1 -> u32
        let s_21_2: u32 = (s_21_1.value() as u32);
        // D s_21_3: read-var t:i
        let s_21_3: i128 = fn_state.t;
        // D s_21_4: call R_set(s_21_3, s_21_2)
        let s_21_4: () = R_set(state, tracer, s_21_3, s_21_2);
        // N s_21_5: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var __SCR_EA:u8
        let s_22_0: bool = fn_state.u__SCR_EA;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var gs#110457 <= s_22_4
        fn_state.gs_110457 = s_22_4;
        // N s_22_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call Halted(s_23_0)
        let s_23_1: bool = Halted(state, tracer, s_23_0);
        // S s_23_2: not s_23_1
        let s_23_2: bool = !s_23_1;
        // D s_23_3: write-var gs#110456 <= s_23_2
        fn_state.gs_110456 = s_23_2;
        // N s_23_4: jump b17
        return block_17(state, tracer, fn_state);
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
        // D s_24_2: call ELUsingAArch32(s_24_1)
        let s_24_2: bool = ELUsingAArch32(state, tracer, s_24_1);
        // D s_24_3: write-var gs#110455 <= s_24_2
        fn_state.gs_110455 = s_24_2;
        // N s_24_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #32s : i
        let s_25_0: i128 = 32;
        // S s_25_1: call Zeros(s_25_0)
        let s_25_1: Bits = Zeros(state, tracer, s_25_0);
        // S s_25_2: cast reint s_25_1 -> u32
        let s_25_2: u32 = (s_25_1.value() as u32);
        // D s_25_3: read-var t:i
        let s_25_3: i128 = fn_state.t;
        // D s_25_4: call R_set(s_25_3, s_25_2)
        let s_25_4: () = R_set(state, tracer, s_25_3, s_25_2);
        // N s_25_5: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __SCR_EL3_EA:u8
        let s_26_0: bool = fn_state.u__SCR_EL3_EA;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#110454 <= s_26_4
        fn_state.gs_110454 = s_26_4;
        // N s_26_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call Halted(s_27_0)
        let s_27_1: bool = Halted(state, tracer, s_27_0);
        // S s_27_2: not s_27_1
        let s_27_2: bool = !s_27_1;
        // D s_27_3: write-var gs#110453 <= s_27_2
        fn_state.gs_110453 = s_27_2;
        // N s_27_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #424u : u32
        let s_28_0: u32 = 424;
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
        // D s_28_4: write-var gs#110452 <= s_28_3
        fn_state.gs_110452 = s_28_3;
        // N s_28_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call EL2Enabled(s_29_0)
        let s_29_1: bool = EL2Enabled(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b83 b30
        if s_29_1 {
            return block_83(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#110460 <= s_30_0
        fn_state.gs_110460 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#110460:u8
        let s_31_0: bool = fn_state.gs_110460;
        // N s_31_1: branch s_31_0 b82 b32
        if s_31_0 {
            return block_82(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#110461 <= s_32_0
        fn_state.gs_110461 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#110461:u8
        let s_33_0: bool = fn_state.gs_110461;
        // N s_33_1: branch s_33_0 b81 b34
        if s_33_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call EL2Enabled(s_34_0)
        let s_34_1: bool = EL2Enabled(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b80 b35
        if s_34_1 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#110462 <= s_35_0
        fn_state.gs_110462 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#110462:u8
        let s_36_0: bool = fn_state.gs_110462;
        // N s_36_1: branch s_36_0 b79 b37
        if s_36_0 {
            return block_79(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#110463 <= s_37_0
        fn_state.gs_110463 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#110463:u8
        let s_38_0: bool = fn_state.gs_110463;
        // N s_38_1: branch s_38_0 b78 b39
        if s_38_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call EL2Enabled(s_39_0)
        let s_39_1: bool = EL2Enabled(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b77 b40
        if s_39_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#110464 <= s_40_0
        fn_state.gs_110464 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#110464:u8
        let s_41_0: bool = fn_state.gs_110464;
        // N s_41_1: branch s_41_0 b76 b42
        if s_41_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#110465 <= s_42_0
        fn_state.gs_110465 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#110465:u8
        let s_43_0: bool = fn_state.gs_110465;
        // N s_43_1: branch s_43_0 b75 b44
        if s_43_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call EL2Enabled(s_44_0)
        let s_44_1: bool = EL2Enabled(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b74 b45
        if s_44_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#110466 <= s_45_0
        fn_state.gs_110466 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#110466:u8
        let s_46_0: bool = fn_state.gs_110466;
        // N s_46_1: branch s_46_0 b73 b47
        if s_46_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#110467 <= s_47_0
        fn_state.gs_110467 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#110467:u8
        let s_48_0: bool = fn_state.gs_110467;
        // N s_48_1: branch s_48_0 b72 b49
        if s_48_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
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
        // N s_49_4: branch s_49_3 b71 b50
        if s_49_3 {
            return block_71(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#110468 <= s_50_0
        fn_state.gs_110468 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#110468:u8
        let s_51_0: bool = fn_state.gs_110468;
        // N s_51_1: branch s_51_0 b70 b52
        if s_51_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#110469 <= s_52_0
        fn_state.gs_110469 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#110469:u8
        let s_53_0: bool = fn_state.gs_110469;
        // N s_53_1: branch s_53_0 b69 b54
        if s_53_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#110470 <= s_54_0
        fn_state.gs_110470 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#110470:u8
        let s_55_0: bool = fn_state.gs_110470;
        // N s_55_1: branch s_55_0 b68 b56
        if s_55_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #424u : u32
        let s_56_0: u32 = 424;
        // D s_56_1: read-reg s_56_0:u8
        let s_56_1: u8 = {
            let value = state.read_register::<u8>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // C s_56_2: const #2u : u8
        let s_56_2: u8 = 2;
        // D s_56_3: cmp-lt s_56_1 s_56_2
        let s_56_3: bool = ((s_56_1) < (s_56_2));
        // N s_56_4: branch s_56_3 b67 b57
        if s_56_3 {
            return block_67(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#110471 <= s_57_0
        fn_state.gs_110471 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#110471:u8
        let s_58_0: bool = fn_state.gs_110471;
        // N s_58_1: branch s_58_0 b66 b59
        if s_58_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#110472 <= s_59_0
        fn_state.gs_110472 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#110472:u8
        let s_60_0: bool = fn_state.gs_110472;
        // N s_60_1: branch s_60_0 b65 b61
        if s_60_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#110473 <= s_61_0
        fn_state.gs_110473 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#110473:u8
        let s_62_0: bool = fn_state.gs_110473;
        // N s_62_1: branch s_62_0 b64 b63
        if s_62_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call DISR_read(s_63_0)
        let s_63_1: ProductType700c18a878c5601b = DISR_read(state, tracer, s_63_0);
        // S s_63_2: call __get_DISR(s_63_1)
        let s_63_2: ProductType700c18a878c5601b = u__get_DISR(state, tracer, s_63_1);
        // D s_63_3: write-var ga#176730 <= s_63_2
        fn_state.ga_176730 = s_63_2;
        // D s_63_4: read-var ga#176730.0:struct
        let s_63_4: u32 = fn_state.ga_176730._0;
        // D s_63_5: read-var t:i
        let s_63_5: i128 = fn_state.t;
        // D s_63_6: call R_set(s_63_5, s_63_4)
        let s_63_6: () = R_set(state, tracer, s_63_5, s_63_4);
        // N s_63_7: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #32s : i
        let s_64_0: i128 = 32;
        // S s_64_1: call Zeros(s_64_0)
        let s_64_1: Bits = Zeros(state, tracer, s_64_0);
        // S s_64_2: cast reint s_64_1 -> u32
        let s_64_2: u32 = (s_64_1.value() as u32);
        // D s_64_3: read-var t:i
        let s_64_3: i128 = fn_state.t;
        // D s_64_4: call R_set(s_64_3, s_64_2)
        let s_64_4: () = R_set(state, tracer, s_64_3, s_64_2);
        // N s_64_5: return
        return;
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var __SCR_EA:u8
        let s_65_0: bool = fn_state.u__SCR_EA;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #1u : u8
        let s_65_2: bool = true;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#110473 <= s_65_4
        fn_state.gs_110473 = s_65_4;
        // N s_65_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call Halted(s_66_0)
        let s_66_1: bool = Halted(state, tracer, s_66_0);
        // S s_66_2: not s_66_1
        let s_66_2: bool = !s_66_1;
        // D s_66_3: write-var gs#110472 <= s_66_2
        fn_state.gs_110472 = s_66_2;
        // N s_66_4: jump b60
        return block_60(state, tracer, fn_state);
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
        // D s_67_3: write-var gs#110471 <= s_67_2
        fn_state.gs_110471 = s_67_2;
        // N s_67_4: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #32s : i
        let s_68_0: i128 = 32;
        // S s_68_1: call Zeros(s_68_0)
        let s_68_1: Bits = Zeros(state, tracer, s_68_0);
        // S s_68_2: cast reint s_68_1 -> u32
        let s_68_2: u32 = (s_68_1.value() as u32);
        // D s_68_3: read-var t:i
        let s_68_3: i128 = fn_state.t;
        // D s_68_4: call R_set(s_68_3, s_68_2)
        let s_68_4: () = R_set(state, tracer, s_68_3, s_68_2);
        // N s_68_5: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __SCR_EL3_EA:u8
        let s_69_0: bool = fn_state.u__SCR_EL3_EA;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#110470 <= s_69_4
        fn_state.gs_110470 = s_69_4;
        // N s_69_6: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call Halted(s_70_0)
        let s_70_1: bool = Halted(state, tracer, s_70_0);
        // S s_70_2: not s_70_1
        let s_70_2: bool = !s_70_1;
        // D s_70_3: write-var gs#110469 <= s_70_2
        fn_state.gs_110469 = s_70_2;
        // N s_70_4: jump b53
        return block_53(state, tracer, fn_state);
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
        // D s_71_2: call ELUsingAArch32(s_71_1)
        let s_71_2: bool = ELUsingAArch32(state, tracer, s_71_1);
        // D s_71_3: not s_71_2
        let s_71_3: bool = !s_71_2;
        // D s_71_4: write-var gs#110468 <= s_71_3
        fn_state.gs_110468 = s_71_3;
        // N s_71_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call VDISR_read(s_72_0)
        let s_72_1: ProductType700c18a878c5601b = VDISR_read(state, tracer, s_72_0);
        // S s_72_2: call __get_VDISR(s_72_1)
        let s_72_2: ProductType700c18a878c5601b = u__get_VDISR(state, tracer, s_72_1);
        // D s_72_3: write-var ga#176714 <= s_72_2
        fn_state.ga_176714 = s_72_2;
        // D s_72_4: read-var ga#176714.0:struct
        let s_72_4: u32 = fn_state.ga_176714._0;
        // D s_72_5: read-var t:i
        let s_72_5: i128 = fn_state.t;
        // D s_72_6: call R_set(s_72_5, s_72_4)
        let s_72_6: () = R_set(state, tracer, s_72_5, s_72_4);
        // N s_72_7: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __HCR_AMO:u8
        let s_73_0: bool = fn_state.u__HCR_AMO;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#110467 <= s_73_4
        fn_state.gs_110467 = s_73_4;
        // N s_73_6: jump b48
        return block_48(state, tracer, fn_state);
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
        // D s_74_3: write-var gs#110466 <= s_74_2
        fn_state.gs_110466 = s_74_2;
        // N s_74_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #19328u : u32
        let s_75_0: u32 = 19328;
        // D s_75_1: read-reg s_75_0:struct
        let s_75_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: call __get_VDISR_EL2(s_75_1)
        let s_75_2: ProductType5c790c8ef59cc8b2 = u__get_VDISR_EL2(
            state,
            tracer,
            s_75_1,
        );
        // D s_75_3: write-var ga#176707 <= s_75_2
        fn_state.ga_176707 = s_75_2;
        // D s_75_4: read-var ga#176707.0:struct
        let s_75_4: u64 = fn_state.ga_176707._0;
        // C s_75_5: const #0s : i
        let s_75_5: i128 = 0;
        // D s_75_6: cast zx s_75_4 -> bv
        let s_75_6: Bits = Bits::new(s_75_4 as u128, 64u16);
        // C s_75_7: const #1s : i64
        let s_75_7: i64 = 1;
        // C s_75_8: cast zx s_75_7 -> i
        let s_75_8: i128 = (i128::try_from(s_75_7).unwrap());
        // C s_75_9: const #31s : i
        let s_75_9: i128 = 31;
        // C s_75_10: add s_75_9 s_75_8
        let s_75_10: i128 = (s_75_9 + s_75_8);
        // D s_75_11: bit-extract s_75_6 s_75_5 s_75_10
        let s_75_11: Bits = (Bits::new(
            ((s_75_6) >> (s_75_5)).value(),
            u16::try_from(s_75_10).unwrap(),
        ));
        // D s_75_12: cast reint s_75_11 -> u32
        let s_75_12: u32 = (s_75_11.value() as u32);
        // D s_75_13: read-var t:i
        let s_75_13: i128 = fn_state.t;
        // D s_75_14: call R_set(s_75_13, s_75_12)
        let s_75_14: () = R_set(state, tracer, s_75_13, s_75_12);
        // N s_75_15: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __HCR_EL2_AMO:u8
        let s_76_0: bool = fn_state.u__HCR_EL2_AMO;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #1u : u8
        let s_76_2: bool = true;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#110465 <= s_76_4
        fn_state.gs_110465 = s_76_4;
        // N s_76_6: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #432u : u32
        let s_77_0: u32 = 432;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: u8 = {
            let value = state.read_register::<u8>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call ELUsingAArch32(s_77_1)
        let s_77_2: bool = ELUsingAArch32(state, tracer, s_77_1);
        // D s_77_3: not s_77_2
        let s_77_3: bool = !s_77_2;
        // D s_77_4: write-var gs#110464 <= s_77_3
        fn_state.gs_110464 = s_77_3;
        // N s_77_5: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #3u : u8
        let s_78_0: u8 = 3;
        // C s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 8u16);
        // C s_78_2: cast zx s_78_1 -> i
        let s_78_2: i128 = (s_78_1.value() as i128);
        // C s_78_3: cast reint s_78_2 -> i64
        let s_78_3: i64 = (s_78_2 as i64);
        // C s_78_4: cast zx s_78_3 -> i
        let s_78_4: i128 = (i128::try_from(s_78_3).unwrap());
        // S s_78_5: call AArch32_TakeHypTrapException(s_78_4)
        let s_78_5: () = AArch32_TakeHypTrapException(state, tracer, s_78_4);
        // N s_78_6: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var __HSTR_T12:u8
        let s_79_0: bool = fn_state.u__HSTR_T12;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 1u16);
        // C s_79_2: const #1u : u8
        let s_79_2: bool = true;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: write-var gs#110463 <= s_79_4
        fn_state.gs_110463 = s_79_4;
        // N s_79_6: jump b38
        return block_38(state, tracer, fn_state);
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
        // D s_80_3: write-var gs#110462 <= s_80_2
        fn_state.gs_110462 = s_80_2;
        // N s_80_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #3u : u8
        let s_81_0: u8 = 3;
        // C s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 8u16);
        // C s_81_2: cast zx s_81_1 -> i
        let s_81_2: i128 = (s_81_1.value() as i128);
        // C s_81_3: cast reint s_81_2 -> i64
        let s_81_3: i64 = (s_81_2 as i64);
        // C s_81_4: cast zx s_81_3 -> i
        let s_81_4: i128 = (i128::try_from(s_81_3).unwrap());
        // C s_81_5: const #432u : u32
        let s_81_5: u32 = 432;
        // D s_81_6: read-reg s_81_5:u8
        let s_81_6: u8 = {
            let value = state.read_register::<u8>(s_81_5 as isize);
            tracer.read_register(s_81_5 as isize, value);
            value
        };
        // D s_81_7: call AArch64_AArch32SystemAccessTrap(s_81_6, s_81_4)
        let s_81_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_81_6, s_81_4);
        // N s_81_8: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var __HSTR_EL2_T12:u8
        let s_82_0: bool = fn_state.u__HSTR_EL2_T12;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 1u16);
        // C s_82_2: const #1u : u8
        let s_82_2: bool = true;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: write-var gs#110461 <= s_82_4
        fn_state.gs_110461 = s_82_4;
        // N s_82_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #432u : u32
        let s_83_0: u32 = 432;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call ELUsingAArch32(s_83_1)
        let s_83_2: bool = ELUsingAArch32(state, tracer, s_83_1);
        // D s_83_3: not s_83_2
        let s_83_3: bool = !s_83_2;
        // D s_83_4: write-var gs#110460 <= s_83_3
        fn_state.gs_110460 = s_83_3;
        // N s_83_5: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: panic
        panic!("{:?}", ());
        // N s_84_1: return
        return;
    }
}
