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
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use Halted::*;
use u__get_PMCR_EL0::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u_get_MDCR_EL2_Type_TPMCR::*;
use u__IMPDEF_boolean::*;
use u_get_MDCR_EL3_Type_TPM::*;
use X_set::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use EDSCR_read::*;
use common::*;
pub fn PMCR_EL0_SysRegRead_e26571c3d63ff1ee<T: Tracer>(
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
        ga_73441: ProductType5c790c8ef59cc8b2,
        gs_64171: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        u__MDCR_EL2_TPMCR: bool,
        gs_64192: bool,
        gs_64186: bool,
        gs_64181: bool,
        gs_64188: bool,
        u__PMUSERENR_EL0_EN: bool,
        gs_64187: bool,
        u__MDCR_EL3_TPM: bool,
        gs_64183: bool,
        gs_64172: bool,
        gs_64189: bool,
        gs_64179: bool,
        gs_64173: bool,
        gs_64164: bool,
        gs_64167: bool,
        gs_64166: bool,
        ga_73425: ProductType5c790c8ef59cc8b2,
        gs_64191: bool,
        ga_73403: ProductType5c790c8ef59cc8b2,
        gs_64175: bool,
        gs_64182: bool,
        gs_64176: bool,
        gs_64165: bool,
        gs_64185: bool,
        ga_73445: ProductType5c790c8ef59cc8b2,
        gs_64174: bool,
        gs_64184: bool,
        u__PSTATE_EL: u8,
        gs_64180: bool,
        gs_64168: bool,
        gs_64177: bool,
        u__MDCR_EL2_TPM: bool,
        gs_64170: bool,
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
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call EDSCR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_EDSCR_Type_SDD(s_0_4)
        let s_0_5: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_4);
        // D s_0_6: write-var __EDSCR_SDD <= s_0_5
        fn_state.u__EDSCR_SDD = s_0_5;
        // C s_0_7: const #22712u : u32
        let s_0_7: u32 = 22712;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL3_Type_TPM(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL3_Type_TPM(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_TPM <= s_0_9
        fn_state.u__MDCR_EL3_TPM = s_0_9;
        // C s_0_11: const #102624u : u32
        let s_0_11: u32 = 102624;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_PMUSERENR_EL0_Type_EN(s_0_12)
        let s_0_13: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_0_12);
        // D s_0_14: write-var __PMUSERENR_EL0_EN <= s_0_13
        fn_state.u__PMUSERENR_EL0_EN = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_TGE <= s_0_17
        fn_state.u__HCR_EL2_TGE = s_0_17;
        // C s_0_19: const #104880u : u32
        let s_0_19: u32 = 104880;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_MDCR_EL2_Type_TPM(s_0_20)
        let s_0_21: bool = u_get_MDCR_EL2_Type_TPM(state, tracer, s_0_20);
        // D s_0_22: write-var __MDCR_EL2_TPM <= s_0_21
        fn_state.u__MDCR_EL2_TPM = s_0_21;
        // C s_0_23: const #104880u : u32
        let s_0_23: u32 = 104880;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_MDCR_EL2_Type_TPMCR(s_0_24)
        let s_0_25: bool = u_get_MDCR_EL2_Type_TPMCR(state, tracer, s_0_24);
        // D s_0_26: write-var __MDCR_EL2_TPMCR <= s_0_25
        fn_state.u__MDCR_EL2_TPMCR = s_0_25;
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
        // N s_0_33: branch s_0_32 b66 b1
        if s_0_32 {
            return block_66(state, tracer, fn_state);
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
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // C s_5_1: const #21016u : u32
        let s_5_1: u32 = 21016;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_PMCR_EL0(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_PMCR_EL0(state, tracer, s_5_2);
        // D s_5_4: write-var ga#73445 <= s_5_3
        fn_state.ga_73445 = s_5_3;
        // D s_5_5: read-var ga#73445.0:struct
        let s_5_5: u64 = fn_state.ga_73445._0;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 64u16);
        // D s_5_7: read-var t:i
        let s_5_7: i128 = fn_state.t;
        // D s_5_8: call X_set(s_5_7, s_5_0, s_5_6)
        let s_5_8: () = X_set(state, tracer, s_5_7, s_5_0, s_5_6);
        // N s_5_9: return
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
        // D s_7_1: write-var gs#64164 <= s_7_0
        fn_state.gs_64164 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#64164:u8
        let s_8_0: bool = fn_state.gs_64164;
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
        // D s_9_1: write-var gs#64165 <= s_9_0
        fn_state.gs_64165 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#64165:u8
        let s_10_0: bool = fn_state.gs_64165;
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
        // D s_11_1: write-var gs#64166 <= s_11_0
        fn_state.gs_64166 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#64166:u8
        let s_12_0: bool = fn_state.gs_64166;
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
        // D s_13_1: write-var gs#64167 <= s_13_0
        fn_state.gs_64167 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#64167:u8
        let s_14_0: bool = fn_state.gs_64167;
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
        // D s_16_1: write-var gs#64168 <= s_16_0
        fn_state.gs_64168 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#64168:u8
        let s_17_0: bool = fn_state.gs_64168;
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
        // C s_18_1: const #21016u : u32
        let s_18_1: u32 = 21016;
        // D s_18_2: read-reg s_18_1:struct
        let s_18_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_1 as isize);
            tracer.read_register(s_18_1 as isize, value);
            value
        };
        // D s_18_3: call __get_PMCR_EL0(s_18_2)
        let s_18_3: ProductType5c790c8ef59cc8b2 = u__get_PMCR_EL0(state, tracer, s_18_2);
        // D s_18_4: write-var ga#73441 <= s_18_3
        fn_state.ga_73441 = s_18_3;
        // D s_18_5: read-var ga#73441.0:struct
        let s_18_5: u64 = fn_state.ga_73441._0;
        // D s_18_6: cast zx s_18_5 -> bv
        let s_18_6: Bits = Bits::new(s_18_5 as u128, 64u16);
        // D s_18_7: read-var t:i
        let s_18_7: i128 = fn_state.t;
        // D s_18_8: call X_set(s_18_7, s_18_0, s_18_6)
        let s_18_8: () = X_set(state, tracer, s_18_7, s_18_0, s_18_6);
        // N s_18_9: return
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
        // D s_20_1: write-var gs#64170 <= s_20_0
        fn_state.gs_64170 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#64170:u8
        let s_21_0: bool = fn_state.gs_64170;
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
        // D s_24_5: write-var gs#64170 <= s_24_4
        fn_state.gs_64170 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __MDCR_EL3_TPM:u8
        let s_25_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#64168 <= s_25_4
        fn_state.gs_64168 = s_25_4;
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
        // D s_27_0: read-var __MDCR_EL3_TPM:u8
        let s_27_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#64167 <= s_27_4
        fn_state.gs_64167 = s_27_4;
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
        // D s_28_2: write-var gs#64166 <= s_28_1
        fn_state.gs_64166 = s_28_1;
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
        // D s_29_5: write-var gs#64165 <= s_29_4
        fn_state.gs_64165 = s_29_4;
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
        // D s_30_4: write-var gs#64164 <= s_30_3
        fn_state.gs_64164 = s_30_3;
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
        // N s_31_2: branch s_31_1 b65 b32
        if s_31_1 {
            return block_65(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#64171 <= s_32_0
        fn_state.gs_64171 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#64171:u8
        let s_33_0: bool = fn_state.gs_64171;
        // N s_33_1: branch s_33_0 b64 b34
        if s_33_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#64172 <= s_34_0
        fn_state.gs_64172 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#64172:u8
        let s_35_0: bool = fn_state.gs_64172;
        // N s_35_1: branch s_35_0 b63 b36
        if s_35_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#64173 <= s_36_0
        fn_state.gs_64173 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#64173:u8
        let s_37_0: bool = fn_state.gs_64173;
        // N s_37_1: branch s_37_0 b62 b38
        if s_37_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#64174 <= s_38_0
        fn_state.gs_64174 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#64174:u8
        let s_39_0: bool = fn_state.gs_64174;
        // N s_39_1: branch s_39_0 b61 b40
        if s_39_0 {
            return block_61(state, tracer, fn_state);
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
        // N s_40_2: branch s_40_1 b60 b41
        if s_40_1 {
            return block_60(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#64175 <= s_41_0
        fn_state.gs_64175 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#64175:u8
        let s_42_0: bool = fn_state.gs_64175;
        // N s_42_1: branch s_42_0 b59 b43
        if s_42_0 {
            return block_59(state, tracer, fn_state);
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
        // N s_43_2: branch s_43_1 b58 b44
        if s_43_1 {
            return block_58(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#64176 <= s_44_0
        fn_state.gs_64176 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#64176:u8
        let s_45_0: bool = fn_state.gs_64176;
        // N s_45_1: branch s_45_0 b57 b46
        if s_45_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #424u : u32
        let s_46_0: u32 = 424;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // C s_46_2: const #2u : u8
        let s_46_2: u8 = 2;
        // D s_46_3: cmp-lt s_46_1 s_46_2
        let s_46_3: bool = ((s_46_1) < (s_46_2));
        // N s_46_4: branch s_46_3 b56 b47
        if s_46_3 {
            return block_56(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#64177 <= s_47_0
        fn_state.gs_64177 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#64177:u8
        let s_48_0: bool = fn_state.gs_64177;
        // N s_48_1: branch s_48_0 b50 b49
        if s_48_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #64s : i64
        let s_49_0: i64 = 64;
        // C s_49_1: const #21016u : u32
        let s_49_1: u32 = 21016;
        // D s_49_2: read-reg s_49_1:struct
        let s_49_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_49_1 as isize);
            tracer.read_register(s_49_1 as isize, value);
            value
        };
        // D s_49_3: call __get_PMCR_EL0(s_49_2)
        let s_49_3: ProductType5c790c8ef59cc8b2 = u__get_PMCR_EL0(state, tracer, s_49_2);
        // D s_49_4: write-var ga#73425 <= s_49_3
        fn_state.ga_73425 = s_49_3;
        // D s_49_5: read-var ga#73425.0:struct
        let s_49_5: u64 = fn_state.ga_73425._0;
        // D s_49_6: cast zx s_49_5 -> bv
        let s_49_6: Bits = Bits::new(s_49_5 as u128, 64u16);
        // D s_49_7: read-var t:i
        let s_49_7: i128 = fn_state.t;
        // D s_49_8: call X_set(s_49_7, s_49_0, s_49_6)
        let s_49_8: () = X_set(state, tracer, s_49_7, s_49_0, s_49_6);
        // N s_49_9: return
        return;
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
        // D s_51_1: write-var gs#64179 <= s_51_0
        fn_state.gs_64179 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#64179:u8
        let s_52_0: bool = fn_state.gs_64179;
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
        // C s_53_5: const #424u : u32
        let s_53_5: u32 = 424;
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
        // D s_55_0: read-var __EDSCR_SDD:u8
        let s_55_0: bool = fn_state.u__EDSCR_SDD;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #1u : u8
        let s_55_2: bool = true;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#64179 <= s_55_4
        fn_state.gs_64179 = s_55_4;
        // N s_55_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __MDCR_EL3_TPM:u8
        let s_56_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#64177 <= s_56_4
        fn_state.gs_64177 = s_56_4;
        // N s_56_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #24u : u8
        let s_57_0: u8 = 24;
        // C s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 8u16);
        // C s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (s_57_1.value() as i128);
        // C s_57_3: cast reint s_57_2 -> i64
        let s_57_3: i64 = (s_57_2 as i64);
        // C s_57_4: cast zx s_57_3 -> i
        let s_57_4: i128 = (i128::try_from(s_57_3).unwrap());
        // C s_57_5: const #432u : u32
        let s_57_5: u32 = 432;
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
        // D s_58_0: read-var __MDCR_EL2_TPMCR:u8
        let s_58_0: bool = fn_state.u__MDCR_EL2_TPMCR;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 1u16);
        // C s_58_2: const #1u : u8
        let s_58_2: bool = true;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: write-var gs#64176 <= s_58_4
        fn_state.gs_64176 = s_58_4;
        // N s_58_6: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #24u : u8
        let s_59_0: u8 = 24;
        // C s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 8u16);
        // C s_59_2: cast zx s_59_1 -> i
        let s_59_2: i128 = (s_59_1.value() as i128);
        // C s_59_3: cast reint s_59_2 -> i64
        let s_59_3: i64 = (s_59_2 as i64);
        // C s_59_4: cast zx s_59_3 -> i
        let s_59_4: i128 = (i128::try_from(s_59_3).unwrap());
        // C s_59_5: const #432u : u32
        let s_59_5: u32 = 432;
        // D s_59_6: read-reg s_59_5:u8
        let s_59_6: u8 = {
            let value = state.read_register::<u8>(s_59_5 as isize);
            tracer.read_register(s_59_5 as isize, value);
            value
        };
        // D s_59_7: call AArch64_SystemAccessTrap(s_59_6, s_59_4)
        let s_59_7: () = AArch64_SystemAccessTrap(state, tracer, s_59_6, s_59_4);
        // N s_59_8: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __MDCR_EL2_TPM:u8
        let s_60_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#64175 <= s_60_4
        fn_state.gs_64175 = s_60_4;
        // N s_60_6: jump b42
        return block_42(state, tracer, fn_state);
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
        // D s_62_0: read-var __MDCR_EL3_TPM:u8
        let s_62_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#64174 <= s_62_4
        fn_state.gs_64174 = s_62_4;
        // N s_62_6: jump b39
        return block_39(state, tracer, fn_state);
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
        // D s_63_2: write-var gs#64173 <= s_63_1
        fn_state.gs_64173 = s_63_1;
        // N s_63_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __EDSCR_SDD:u8
        let s_64_0: bool = fn_state.u__EDSCR_SDD;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#64172 <= s_64_4
        fn_state.gs_64172 = s_64_4;
        // N s_64_6: jump b35
        return block_35(state, tracer, fn_state);
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
        // D s_65_4: write-var gs#64171 <= s_65_3
        fn_state.gs_64171 = s_65_3;
        // N s_65_5: jump b33
        return block_33(state, tracer, fn_state);
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
        // N s_66_2: branch s_66_1 b116 b67
        if s_66_1 {
            return block_116(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#64180 <= s_67_0
        fn_state.gs_64180 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#64180:u8
        let s_68_0: bool = fn_state.gs_64180;
        // N s_68_1: branch s_68_0 b115 b69
        if s_68_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#64181 <= s_69_0
        fn_state.gs_64181 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#64181:u8
        let s_70_0: bool = fn_state.gs_64181;
        // N s_70_1: branch s_70_0 b114 b71
        if s_70_0 {
            return block_114(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#64182 <= s_71_0
        fn_state.gs_64182 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#64182:u8
        let s_72_0: bool = fn_state.gs_64182;
        // N s_72_1: branch s_72_0 b113 b73
        if s_72_0 {
            return block_113(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#64183 <= s_73_0
        fn_state.gs_64183 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#64183:u8
        let s_74_0: bool = fn_state.gs_64183;
        // N s_74_1: branch s_74_0 b112 b75
        if s_74_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #204u : u32
        let s_75_0: u32 = 204;
        // S s_75_1: call IsFeatureImplemented(s_75_0)
        let s_75_1: bool = IsFeatureImplemented(state, tracer, s_75_0);
        // N s_75_2: branch s_75_1 b111 b76
        if s_75_1 {
            return block_111(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#64184 <= s_76_0
        fn_state.gs_64184 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#64184:u8
        let s_77_0: bool = fn_state.gs_64184;
        // N s_77_1: branch s_77_0 b110 b78
        if s_77_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #204u : u32
        let s_78_0: u32 = 204;
        // S s_78_1: call IsFeatureImplemented(s_78_0)
        let s_78_1: bool = IsFeatureImplemented(state, tracer, s_78_0);
        // S s_78_2: not s_78_1
        let s_78_2: bool = !s_78_1;
        // N s_78_3: branch s_78_2 b109 b79
        if s_78_2 {
            return block_109(state, tracer, fn_state);
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
        // D s_79_1: write-var gs#64185 <= s_79_0
        fn_state.gs_64185 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#64185:u8
        let s_80_0: bool = fn_state.gs_64185;
        // D s_80_1: write-var gs#64186 <= s_80_0
        fn_state.gs_64186 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#64186:u8
        let s_81_0: bool = fn_state.gs_64186;
        // N s_81_1: branch s_81_0 b103 b82
        if s_81_0 {
            return block_103(state, tracer, fn_state);
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
        // S s_82_1: call EL2Enabled(s_82_0)
        let s_82_1: bool = EL2Enabled(state, tracer, s_82_0);
        // N s_82_2: branch s_82_1 b102 b83
        if s_82_1 {
            return block_102(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#64187 <= s_83_0
        fn_state.gs_64187 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#64187:u8
        let s_84_0: bool = fn_state.gs_64187;
        // N s_84_1: branch s_84_0 b101 b85
        if s_84_0 {
            return block_101(state, tracer, fn_state);
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
        // N s_85_2: branch s_85_1 b100 b86
        if s_85_1 {
            return block_100(state, tracer, fn_state);
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
        // D s_86_1: write-var gs#64188 <= s_86_0
        fn_state.gs_64188 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#64188:u8
        let s_87_0: bool = fn_state.gs_64188;
        // N s_87_1: branch s_87_0 b99 b88
        if s_87_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #424u : u32
        let s_88_0: u32 = 424;
        // D s_88_1: read-reg s_88_0:u8
        let s_88_1: u8 = {
            let value = state.read_register::<u8>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // C s_88_2: const #2u : u8
        let s_88_2: u8 = 2;
        // D s_88_3: cmp-lt s_88_1 s_88_2
        let s_88_3: bool = ((s_88_1) < (s_88_2));
        // N s_88_4: branch s_88_3 b98 b89
        if s_88_3 {
            return block_98(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#64189 <= s_89_0
        fn_state.gs_64189 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#64189:u8
        let s_90_0: bool = fn_state.gs_64189;
        // N s_90_1: branch s_90_0 b92 b91
        if s_90_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #64s : i64
        let s_91_0: i64 = 64;
        // C s_91_1: const #21016u : u32
        let s_91_1: u32 = 21016;
        // D s_91_2: read-reg s_91_1:struct
        let s_91_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_91_1 as isize);
            tracer.read_register(s_91_1 as isize, value);
            value
        };
        // D s_91_3: call __get_PMCR_EL0(s_91_2)
        let s_91_3: ProductType5c790c8ef59cc8b2 = u__get_PMCR_EL0(state, tracer, s_91_2);
        // D s_91_4: write-var ga#73403 <= s_91_3
        fn_state.ga_73403 = s_91_3;
        // D s_91_5: read-var ga#73403.0:struct
        let s_91_5: u64 = fn_state.ga_73403._0;
        // D s_91_6: cast zx s_91_5 -> bv
        let s_91_6: Bits = Bits::new(s_91_5 as u128, 64u16);
        // D s_91_7: read-var t:i
        let s_91_7: i128 = fn_state.t;
        // D s_91_8: call X_set(s_91_7, s_91_0, s_91_6)
        let s_91_8: () = X_set(state, tracer, s_91_7, s_91_0, s_91_6);
        // N s_91_9: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #() : ()
        let s_92_0: () = ();
        // S s_92_1: call Halted(s_92_0)
        let s_92_1: bool = Halted(state, tracer, s_92_0);
        // N s_92_2: branch s_92_1 b97 b93
        if s_92_1 {
            return block_97(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#64191 <= s_93_0
        fn_state.gs_64191 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#64191:u8
        let s_94_0: bool = fn_state.gs_64191;
        // N s_94_1: branch s_94_0 b96 b95
        if s_94_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #24u : u8
        let s_95_0: u8 = 24;
        // C s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 8u16);
        // C s_95_2: cast zx s_95_1 -> i
        let s_95_2: i128 = (s_95_1.value() as i128);
        // C s_95_3: cast reint s_95_2 -> i64
        let s_95_3: i64 = (s_95_2 as i64);
        // C s_95_4: cast zx s_95_3 -> i
        let s_95_4: i128 = (i128::try_from(s_95_3).unwrap());
        // C s_95_5: const #424u : u32
        let s_95_5: u32 = 424;
        // D s_95_6: read-reg s_95_5:u8
        let s_95_6: u8 = {
            let value = state.read_register::<u8>(s_95_5 as isize);
            tracer.read_register(s_95_5 as isize, value);
            value
        };
        // D s_95_7: call AArch64_SystemAccessTrap(s_95_6, s_95_4)
        let s_95_7: () = AArch64_SystemAccessTrap(state, tracer, s_95_6, s_95_4);
        // N s_95_8: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_96_0: panic
        panic!("{:?}", ());
        // N s_96_1: return
        return;
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var __EDSCR_SDD:u8
        let s_97_0: bool = fn_state.u__EDSCR_SDD;
        // D s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 1u16);
        // C s_97_2: const #1u : u8
        let s_97_2: bool = true;
        // C s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_4: cmp-eq s_97_1 s_97_3
        let s_97_4: bool = ((s_97_1) == (s_97_3));
        // D s_97_5: write-var gs#64191 <= s_97_4
        fn_state.gs_64191 = s_97_4;
        // N s_97_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var __MDCR_EL3_TPM:u8
        let s_98_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 1u16);
        // C s_98_2: const #1u : u8
        let s_98_2: bool = true;
        // C s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 1u16);
        // D s_98_4: cmp-eq s_98_1 s_98_3
        let s_98_4: bool = ((s_98_1) == (s_98_3));
        // D s_98_5: write-var gs#64189 <= s_98_4
        fn_state.gs_64189 = s_98_4;
        // N s_98_6: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #24u : u8
        let s_99_0: u8 = 24;
        // C s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 8u16);
        // C s_99_2: cast zx s_99_1 -> i
        let s_99_2: i128 = (s_99_1.value() as i128);
        // C s_99_3: cast reint s_99_2 -> i64
        let s_99_3: i64 = (s_99_2 as i64);
        // C s_99_4: cast zx s_99_3 -> i
        let s_99_4: i128 = (i128::try_from(s_99_3).unwrap());
        // C s_99_5: const #432u : u32
        let s_99_5: u32 = 432;
        // D s_99_6: read-reg s_99_5:u8
        let s_99_6: u8 = {
            let value = state.read_register::<u8>(s_99_5 as isize);
            tracer.read_register(s_99_5 as isize, value);
            value
        };
        // D s_99_7: call AArch64_SystemAccessTrap(s_99_6, s_99_4)
        let s_99_7: () = AArch64_SystemAccessTrap(state, tracer, s_99_6, s_99_4);
        // N s_99_8: return
        return;
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var __MDCR_EL2_TPMCR:u8
        let s_100_0: bool = fn_state.u__MDCR_EL2_TPMCR;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 1u16);
        // C s_100_2: const #1u : u8
        let s_100_2: bool = true;
        // C s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // D s_100_4: cmp-eq s_100_1 s_100_3
        let s_100_4: bool = ((s_100_1) == (s_100_3));
        // D s_100_5: write-var gs#64188 <= s_100_4
        fn_state.gs_64188 = s_100_4;
        // N s_100_6: jump b87
        return block_87(state, tracer, fn_state);
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
        // C s_101_5: const #432u : u32
        let s_101_5: u32 = 432;
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
        // D s_102_0: read-var __MDCR_EL2_TPM:u8
        let s_102_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #1u : u8
        let s_102_2: bool = true;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#64187 <= s_102_4
        fn_state.gs_64187 = s_102_4;
        // N s_102_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #() : ()
        let s_103_0: () = ();
        // S s_103_1: call EL2Enabled(s_103_0)
        let s_103_1: bool = EL2Enabled(state, tracer, s_103_0);
        // N s_103_2: branch s_103_1 b108 b104
        if s_103_1 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #0u : u8
        let s_104_0: bool = false;
        // D s_104_1: write-var gs#64192 <= s_104_0
        fn_state.gs_64192 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#64192:u8
        let s_105_0: bool = fn_state.gs_64192;
        // N s_105_1: branch s_105_0 b107 b106
        if s_105_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #24u : u8
        let s_106_0: u8 = 24;
        // C s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 8u16);
        // C s_106_2: cast zx s_106_1 -> i
        let s_106_2: i128 = (s_106_1.value() as i128);
        // C s_106_3: cast reint s_106_2 -> i64
        let s_106_3: i64 = (s_106_2 as i64);
        // C s_106_4: cast zx s_106_3 -> i
        let s_106_4: i128 = (i128::try_from(s_106_3).unwrap());
        // C s_106_5: const #440u : u32
        let s_106_5: u32 = 440;
        // D s_106_6: read-reg s_106_5:u8
        let s_106_6: u8 = {
            let value = state.read_register::<u8>(s_106_5 as isize);
            tracer.read_register(s_106_5 as isize, value);
            value
        };
        // D s_106_7: call AArch64_SystemAccessTrap(s_106_6, s_106_4)
        let s_106_7: () = AArch64_SystemAccessTrap(state, tracer, s_106_6, s_106_4);
        // N s_106_8: return
        return;
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #24u : u8
        let s_107_0: u8 = 24;
        // C s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 8u16);
        // C s_107_2: cast zx s_107_1 -> i
        let s_107_2: i128 = (s_107_1.value() as i128);
        // C s_107_3: cast reint s_107_2 -> i64
        let s_107_3: i64 = (s_107_2 as i64);
        // C s_107_4: cast zx s_107_3 -> i
        let s_107_4: i128 = (i128::try_from(s_107_3).unwrap());
        // C s_107_5: const #432u : u32
        let s_107_5: u32 = 432;
        // D s_107_6: read-reg s_107_5:u8
        let s_107_6: u8 = {
            let value = state.read_register::<u8>(s_107_5 as isize);
            tracer.read_register(s_107_5 as isize, value);
            value
        };
        // D s_107_7: call AArch64_SystemAccessTrap(s_107_6, s_107_4)
        let s_107_7: () = AArch64_SystemAccessTrap(state, tracer, s_107_6, s_107_4);
        // N s_107_8: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var __HCR_EL2_TGE:u8
        let s_108_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #1u : u8
        let s_108_2: bool = true;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#64192 <= s_108_4
        fn_state.gs_64192 = s_108_4;
        // N s_108_6: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var __PMUSERENR_EL0_EN:u8
        let s_109_0: bool = fn_state.u__PMUSERENR_EL0_EN;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 1u16);
        // C s_109_2: const #0u : u8
        let s_109_2: bool = false;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 1u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // D s_109_5: write-var gs#64185 <= s_109_4
        fn_state.gs_64185 = s_109_4;
        // N s_109_6: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #1u : u8
        let s_110_0: bool = true;
        // D s_110_1: write-var gs#64186 <= s_110_0
        fn_state.gs_64186 = s_110_0;
        // N s_110_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #102624u : u32
        let s_111_0: u32 = 102624;
        // D s_111_1: read-reg s_111_0:struct
        let s_111_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_111_0 as isize);
            tracer.read_register(s_111_0 as isize, value);
            value
        };
        // D s_111_2: call _get_PMUSERENR_EL0_Type_UEN(s_111_1)
        let s_111_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_111_1);
        // C s_111_3: const #102624u : u32
        let s_111_3: u32 = 102624;
        // D s_111_4: read-reg s_111_3:struct
        let s_111_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_111_3 as isize);
            tracer.read_register(s_111_3 as isize, value);
            value
        };
        // D s_111_5: call _get_PMUSERENR_EL0_Type_EN(s_111_4)
        let s_111_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_111_4);
        // D s_111_6: cast zx s_111_2 -> bv
        let s_111_6: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_7: cast zx s_111_5 -> bv
        let s_111_7: Bits = Bits::new(s_111_5 as u128, 1u16);
        // D s_111_8: cast reint s_111_6 -> u128
        let s_111_8: u128 = (s_111_6.value() as u128);
        // D s_111_9: size-of s_111_6
        let s_111_9: u16 = s_111_6.length();
        // D s_111_10: cast reint s_111_7 -> u128
        let s_111_10: u128 = (s_111_7.value() as u128);
        // D s_111_11: size-of s_111_7
        let s_111_11: u16 = s_111_7.length();
        // D s_111_12: lsl s_111_8 s_111_11
        let s_111_12: u128 = s_111_8 << s_111_11;
        // D s_111_13: or s_111_12 s_111_10
        let s_111_13: u128 = ((s_111_12) | (s_111_10));
        // D s_111_14: add s_111_9 s_111_11
        let s_111_14: u16 = (s_111_9 + s_111_11);
        // D s_111_15: create-bits s_111_13 s_111_14
        let s_111_15: Bits = Bits::new(s_111_13, s_111_14);
        // D s_111_16: cast reint s_111_15 -> u8
        let s_111_16: u8 = (s_111_15.value() as u8);
        // D s_111_17: cast zx s_111_16 -> bv
        let s_111_17: Bits = Bits::new(s_111_16 as u128, 2u16);
        // C s_111_18: const #1u : u8
        let s_111_18: u8 = 1;
        // C s_111_19: cast zx s_111_18 -> bv
        let s_111_19: Bits = Bits::new(s_111_18 as u128, 2u16);
        // D s_111_20: cmp-ne s_111_17 s_111_19
        let s_111_20: bool = ((s_111_17) != (s_111_19));
        // D s_111_21: write-var gs#64184 <= s_111_20
        fn_state.gs_64184 = s_111_20;
        // N s_111_22: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_112_0: panic
        panic!("{:?}", ());
        // N s_112_1: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var __MDCR_EL3_TPM:u8
        let s_113_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 1u16);
        // C s_113_2: const #1u : u8
        let s_113_2: bool = true;
        // C s_113_3: cast zx s_113_2 -> bv
        let s_113_3: Bits = Bits::new(s_113_2 as u128, 1u16);
        // D s_113_4: cmp-eq s_113_1 s_113_3
        let s_113_4: bool = ((s_113_1) == (s_113_3));
        // D s_113_5: write-var gs#64183 <= s_113_4
        fn_state.gs_64183 = s_113_4;
        // N s_113_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_114_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_114_1: call __IMPDEF_boolean(s_114_0)
        let s_114_1: bool = u__IMPDEF_boolean(state, tracer, s_114_0);
        // D s_114_2: write-var gs#64182 <= s_114_1
        fn_state.gs_64182 = s_114_1;
        // N s_114_3: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var __EDSCR_SDD:u8
        let s_115_0: bool = fn_state.u__EDSCR_SDD;
        // D s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 1u16);
        // C s_115_2: const #1u : u8
        let s_115_2: bool = true;
        // C s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_4: cmp-eq s_115_1 s_115_3
        let s_115_4: bool = ((s_115_1) == (s_115_3));
        // D s_115_5: write-var gs#64181 <= s_115_4
        fn_state.gs_64181 = s_115_4;
        // N s_115_6: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #424u : u32
        let s_116_0: u32 = 424;
        // D s_116_1: read-reg s_116_0:u8
        let s_116_1: u8 = {
            let value = state.read_register::<u8>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // C s_116_2: const #2u : u8
        let s_116_2: u8 = 2;
        // D s_116_3: cmp-lt s_116_1 s_116_2
        let s_116_3: bool = ((s_116_1) < (s_116_2));
        // D s_116_4: write-var gs#64180 <= s_116_3
        fn_state.gs_64180 = s_116_3;
        // N s_116_5: jump b68
        return block_68(state, tracer, fn_state);
    }
}
