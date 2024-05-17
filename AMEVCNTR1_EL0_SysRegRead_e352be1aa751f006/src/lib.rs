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
use u_get_SCR_EL3_Type_FGTEn::*;
use IsHighestEL::*;
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use u_get_AMCR_EL0_Type_CG1RZ::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use Zeros::*;
use u__IMPDEF_boolean::*;
use IsG1ActivityMonitorImplemented::*;
use X_set::*;
use u_get_CPTR_EL3_Type_TAM::*;
use u_get_HAFGRTR_EL2_Type_AMEVCNTR16_EL0::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_AMUSERENR_EL0_Type_EN::*;
use u_get_CPTR_EL2_Type_TAM::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn AMEVCNTR1_EL0_SysRegRead_e352be1aa751f006<T: Tracer>(
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
        gs_48241: bool,
        gs_48234: bool,
        u__AMCR_EL0_CG1RZ: bool,
        gs_48262: bool,
        gs_48258: bool,
        gs_48230: bool,
        gs_48255: bool,
        gs_48264: bool,
        gs_48260: bool,
        u__CPTR_EL3_TAM: bool,
        gs_48265: bool,
        u__CPTR_EL2_TAM: bool,
        gs_48232: bool,
        gs_48244: bool,
        gs_48246: bool,
        gs_48245: bool,
        gs_48233: bool,
        ga_40538: ProductType5c790c8ef59cc8b2,
        gs_48229: bool,
        gs_48243: bool,
        gs_48272: bool,
        gs_48239: bool,
        u__HAFGRTR_EL2_AMEVCNTR16_EL0: bool,
        gs_48266: bool,
        gs_48242: bool,
        gs_48249: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        ga_40594: ProductType5c790c8ef59cc8b2,
        gs_48259: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_48240: bool,
        ga_40599: ProductType5c790c8ef59cc8b2,
        u__AMUSERENR_EL0_EN: bool,
        gs_48247: bool,
        gs_48257: bool,
        gs_48271: bool,
        ga_40571: ProductType5c790c8ef59cc8b2,
        gs_48256: bool,
        gs_48250: bool,
        gs_48231: bool,
        gs_48248: bool,
        gs_48263: bool,
        gs_48261: bool,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call EDSCR_read(s_0_0)
        let s_0_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_0);
        // S s_0_2: call _get_EDSCR_Type_SDD(s_0_1)
        let s_0_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_1);
        // D s_0_3: write-var __EDSCR_SDD <= s_0_2
        fn_state.u__EDSCR_SDD = s_0_2;
        // C s_0_4: const #16840u : u32
        let s_0_4: u32 = 16840;
        // D s_0_5: read-reg s_0_4:struct
        let s_0_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: call _get_CPTR_EL3_Type_TAM(s_0_5)
        let s_0_6: bool = u_get_CPTR_EL3_Type_TAM(state, tracer, s_0_5);
        // D s_0_7: write-var __CPTR_EL3_TAM <= s_0_6
        fn_state.u__CPTR_EL3_TAM = s_0_6;
        // C s_0_8: const #90496u : u32
        let s_0_8: u32 = 90496;
        // D s_0_9: read-reg s_0_8:struct
        let s_0_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: call _get_AMUSERENR_EL0_Type_EN(s_0_9)
        let s_0_10: bool = u_get_AMUSERENR_EL0_Type_EN(state, tracer, s_0_9);
        // D s_0_11: write-var __AMUSERENR_EL0_EN <= s_0_10
        fn_state.u__AMUSERENR_EL0_EN = s_0_10;
        // C s_0_12: const #102552u : u32
        let s_0_12: u32 = 102552;
        // D s_0_13: read-reg s_0_12:struct
        let s_0_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_12 as isize);
            tracer.read_register(s_0_12 as isize, value);
            value
        };
        // D s_0_14: call _get_HCR_EL2_Type_TGE(s_0_13)
        let s_0_14: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_13);
        // D s_0_15: write-var __HCR_EL2_TGE <= s_0_14
        fn_state.u__HCR_EL2_TGE = s_0_14;
        // C s_0_16: const #11088u : u32
        let s_0_16: u32 = 11088;
        // D s_0_17: read-reg s_0_16:struct
        let s_0_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_16 as isize);
            tracer.read_register(s_0_16 as isize, value);
            value
        };
        // D s_0_18: call _get_CPTR_EL2_Type_TAM(s_0_17)
        let s_0_18: bool = u_get_CPTR_EL2_Type_TAM(state, tracer, s_0_17);
        // D s_0_19: write-var __CPTR_EL2_TAM <= s_0_18
        fn_state.u__CPTR_EL2_TAM = s_0_18;
        // C s_0_20: const #90704u : u32
        let s_0_20: u32 = 90704;
        // D s_0_21: read-reg s_0_20:struct
        let s_0_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_20 as isize);
            tracer.read_register(s_0_20 as isize, value);
            value
        };
        // D s_0_22: call _get_SCR_EL3_Type_FGTEn(s_0_21)
        let s_0_22: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_21);
        // D s_0_23: write-var __SCR_EL3_FGTEn <= s_0_22
        fn_state.u__SCR_EL3_FGTEn = s_0_22;
        // C s_0_24: const #21760u : u32
        let s_0_24: u32 = 21760;
        // D s_0_25: read-reg s_0_24:struct
        let s_0_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_24 as isize);
            tracer.read_register(s_0_24 as isize, value);
            value
        };
        // D s_0_26: call _get_HAFGRTR_EL2_Type_AMEVCNTR16_EL0(s_0_25)
        let s_0_26: bool = u_get_HAFGRTR_EL2_Type_AMEVCNTR16_EL0(state, tracer, s_0_25);
        // D s_0_27: write-var __HAFGRTR_EL2_AMEVCNTR16_EL0 <= s_0_26
        fn_state.u__HAFGRTR_EL2_AMEVCNTR16_EL0 = s_0_26;
        // C s_0_28: const #15544u : u32
        let s_0_28: u32 = 15544;
        // D s_0_29: read-reg s_0_28:struct
        let s_0_29: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_28 as isize);
            tracer.read_register(s_0_28 as isize, value);
            value
        };
        // D s_0_30: call _get_AMCR_EL0_Type_CG1RZ(s_0_29)
        let s_0_30: bool = u_get_AMCR_EL0_Type_CG1RZ(state, tracer, s_0_29);
        // D s_0_31: write-var __AMCR_EL0_CG1RZ <= s_0_30
        fn_state.u__AMCR_EL0_CG1RZ = s_0_30;
        // C s_0_32: const #6s : i
        let s_0_32: i128 = 6;
        // S s_0_33: call IsG1ActivityMonitorImplemented(s_0_32)
        let s_0_33: bool = IsG1ActivityMonitorImplemented(state, tracer, s_0_32);
        // S s_0_34: not s_0_33
        let s_0_34: bool = !s_0_33;
        // N s_0_35: branch s_0_34 b142 b1
        if s_0_34 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // N s_1_7: branch s_1_6 b86 b2
        if s_1_6 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16975u : u32
        let s_2_0: u32 = 16975;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #440u : u32
        let s_2_3: u32 = 440;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // N s_2_7: branch s_2_6 b37 b3
        if s_2_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #432u : u32
        let s_3_3: u32 = 432;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) == (s_3_5));
        // N s_3_7: branch s_3_6 b7 b4
        if s_3_6 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #16975u : u32
        let s_4_0: u32 = 16975;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 2u16);
        // C s_4_3: const #424u : u32
        let s_4_3: u32 = 424;
        // D s_4_4: read-reg s_4_3:u8
        let s_4_4: u8 = {
            let value = state.read_register::<u8>(s_4_3 as isize);
            tracer.read_register(s_4_3 as isize, value);
            value
        };
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 2u16);
        // D s_4_6: cmp-eq s_4_2 s_4_5
        let s_4_6: bool = ((s_4_2) == (s_4_5));
        // N s_4_7: branch s_4_6 b6 b5
        if s_4_6 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #6s : i
        let s_6_1: i128 = 6;
        // C s_6_2: const #103240u : u32
        let s_6_2: u32 = 103240;
        // D s_6_3: read-reg s_6_2:[struct; 16]
        let s_6_3: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 16usize]>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: read-element s_6_3[s_6_1]
        let s_6_4: ProductType5c790c8ef59cc8b2 = s_6_3[(s_6_1) as usize];
        // D s_6_5: write-var ga#40599 <= s_6_4
        fn_state.ga_40599 = s_6_4;
        // D s_6_6: read-var ga#40599.0:struct
        let s_6_6: u64 = fn_state.ga_40599._0;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 64u16);
        // D s_6_8: read-var t:i
        let s_6_8: i128 = fn_state.t;
        // D s_6_9: call X_set(s_6_8, s_6_0, s_6_7)
        let s_6_9: () = X_set(state, tracer, s_6_8, s_6_0, s_6_7);
        // N s_6_10: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call Halted(s_7_0)
        let s_7_1: bool = Halted(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b36 b8
        if s_7_1 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#48229 <= s_8_0
        fn_state.gs_48229 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#48229:u8
        let s_9_0: bool = fn_state.gs_48229;
        // N s_9_1: branch s_9_0 b35 b10
        if s_9_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#48230 <= s_10_0
        fn_state.gs_48230 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#48230:u8
        let s_11_0: bool = fn_state.gs_48230;
        // N s_11_1: branch s_11_0 b34 b12
        if s_11_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#48231 <= s_12_0
        fn_state.gs_48231 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#48231:u8
        let s_13_0: bool = fn_state.gs_48231;
        // N s_13_1: branch s_13_0 b33 b14
        if s_13_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#48232 <= s_14_0
        fn_state.gs_48232 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#48232:u8
        let s_15_0: bool = fn_state.gs_48232;
        // N s_15_1: branch s_15_0 b32 b16
        if s_15_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // C s_16_2: const #2u : u8
        let s_16_2: u8 = 2;
        // D s_16_3: cmp-lt s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) < (s_16_2));
        // N s_16_4: branch s_16_3 b31 b17
        if s_16_3 {
            return block_31(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#48233 <= s_17_0
        fn_state.gs_48233 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#48233:u8
        let s_18_0: bool = fn_state.gs_48233;
        // N s_18_1: branch s_18_0 b25 b19
        if s_18_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #16975u : u32
        let s_19_0: u32 = 16975;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call IsHighestEL(s_19_1)
        let s_19_2: bool = IsHighestEL(state, tracer, s_19_1);
        // D s_19_3: not s_19_2
        let s_19_3: bool = !s_19_2;
        // N s_19_4: branch s_19_3 b24 b20
        if s_19_3 {
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
        // D s_20_1: write-var gs#48234 <= s_20_0
        fn_state.gs_48234 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#48234:u8
        let s_21_0: bool = fn_state.gs_48234;
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
        // C s_22_0: const #64s : i64
        let s_22_0: i64 = 64;
        // C s_22_1: const #6s : i
        let s_22_1: i128 = 6;
        // C s_22_2: const #103240u : u32
        let s_22_2: u32 = 103240;
        // D s_22_3: read-reg s_22_2:[struct; 16]
        let s_22_3: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_22_2 as isize);
            tracer.read_register(s_22_2 as isize, value);
            value
        };
        // D s_22_4: read-element s_22_3[s_22_1]
        let s_22_4: ProductType5c790c8ef59cc8b2 = s_22_3[(s_22_1) as usize];
        // D s_22_5: write-var ga#40594 <= s_22_4
        fn_state.ga_40594 = s_22_4;
        // D s_22_6: read-var ga#40594.0:struct
        let s_22_6: u64 = fn_state.ga_40594._0;
        // D s_22_7: cast zx s_22_6 -> bv
        let s_22_7: Bits = Bits::new(s_22_6 as u128, 64u16);
        // D s_22_8: read-var t:i
        let s_22_8: i128 = fn_state.t;
        // D s_22_9: call X_set(s_22_8, s_22_0, s_22_7)
        let s_22_9: () = X_set(state, tracer, s_22_8, s_22_0, s_22_7);
        // N s_22_10: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // C s_23_1: const #64s : i
        let s_23_1: i128 = 64;
        // S s_23_2: call Zeros(s_23_1)
        let s_23_2: Bits = Zeros(state, tracer, s_23_1);
        // S s_23_3: cast reint s_23_2 -> u64
        let s_23_3: u64 = (s_23_2.value() as u64);
        // S s_23_4: cast zx s_23_3 -> bv
        let s_23_4: Bits = Bits::new(s_23_3 as u128, 64u16);
        // D s_23_5: read-var t:i
        let s_23_5: i128 = fn_state.t;
        // D s_23_6: call X_set(s_23_5, s_23_0, s_23_4)
        let s_23_6: () = X_set(state, tracer, s_23_5, s_23_0, s_23_4);
        // N s_23_7: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __AMCR_EL0_CG1RZ:u8
        let s_24_0: bool = fn_state.u__AMCR_EL0_CG1RZ;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#48234 <= s_24_4
        fn_state.gs_48234 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call Halted(s_25_0)
        let s_25_1: bool = Halted(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b30 b26
        if s_25_1 {
            return block_30(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#48239 <= s_26_0
        fn_state.gs_48239 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#48239:u8
        let s_27_0: bool = fn_state.gs_48239;
        // N s_27_1: branch s_27_0 b29 b28
        if s_27_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #24u : u8
        let s_28_0: u8 = 24;
        // C s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 8u16);
        // C s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (s_28_1.value() as i128);
        // C s_28_3: cast reint s_28_2 -> i64
        let s_28_3: i64 = (s_28_2 as i64);
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // C s_28_5: const #424u : u32
        let s_28_5: u32 = 424;
        // D s_28_6: read-reg s_28_5:u8
        let s_28_6: u8 = {
            let value = state.read_register::<u8>(s_28_5 as isize);
            tracer.read_register(s_28_5 as isize, value);
            value
        };
        // D s_28_7: call AArch64_SystemAccessTrap(s_28_6, s_28_4)
        let s_28_7: () = AArch64_SystemAccessTrap(state, tracer, s_28_6, s_28_4);
        // N s_28_8: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: panic
        panic!("{:?}", ());
        // N s_29_1: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __EDSCR_SDD:u8
        let s_30_0: bool = fn_state.u__EDSCR_SDD;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#48239 <= s_30_4
        fn_state.gs_48239 = s_30_4;
        // N s_30_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __CPTR_EL3_TAM:u8
        let s_31_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#48233 <= s_31_4
        fn_state.gs_48233 = s_31_4;
        // N s_31_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: panic
        panic!("{:?}", ());
        // N s_32_1: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __CPTR_EL3_TAM:u8
        let s_33_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#48232 <= s_33_4
        fn_state.gs_48232 = s_33_4;
        // N s_33_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_34_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_34_1: call __IMPDEF_boolean(s_34_0)
        let s_34_1: bool = u__IMPDEF_boolean(state, tracer, s_34_0);
        // D s_34_2: write-var gs#48231 <= s_34_1
        fn_state.gs_48231 = s_34_1;
        // N s_34_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var __EDSCR_SDD:u8
        let s_35_0: bool = fn_state.u__EDSCR_SDD;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 1u16);
        // C s_35_2: const #1u : u8
        let s_35_2: bool = true;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: write-var gs#48230 <= s_35_4
        fn_state.gs_48230 = s_35_4;
        // N s_35_6: jump b11
        return block_11(state, tracer, fn_state);
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
        // D s_36_4: write-var gs#48229 <= s_36_3
        fn_state.gs_48229 = s_36_3;
        // N s_36_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call Halted(s_37_0)
        let s_37_1: bool = Halted(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b85 b38
        if s_37_1 {
            return block_85(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#48240 <= s_38_0
        fn_state.gs_48240 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#48240:u8
        let s_39_0: bool = fn_state.gs_48240;
        // N s_39_1: branch s_39_0 b84 b40
        if s_39_0 {
            return block_84(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#48241 <= s_40_0
        fn_state.gs_48241 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#48241:u8
        let s_41_0: bool = fn_state.gs_48241;
        // N s_41_1: branch s_41_0 b83 b42
        if s_41_0 {
            return block_83(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#48242 <= s_42_0
        fn_state.gs_48242 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#48242:u8
        let s_43_0: bool = fn_state.gs_48242;
        // N s_43_1: branch s_43_0 b82 b44
        if s_43_0 {
            return block_82(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#48243 <= s_44_0
        fn_state.gs_48243 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#48243:u8
        let s_45_0: bool = fn_state.gs_48243;
        // N s_45_1: branch s_45_0 b81 b46
        if s_45_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call EL2Enabled(s_46_0)
        let s_46_1: bool = EL2Enabled(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b80 b47
        if s_46_1 {
            return block_80(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#48244 <= s_47_0
        fn_state.gs_48244 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#48244:u8
        let s_48_0: bool = fn_state.gs_48244;
        // N s_48_1: branch s_48_0 b79 b49
        if s_48_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
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
        // N s_49_2: branch s_49_1 b78 b50
        if s_49_1 {
            return block_78(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#48245 <= s_50_0
        fn_state.gs_48245 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#48245:u8
        let s_51_0: bool = fn_state.gs_48245;
        // N s_51_1: branch s_51_0 b74 b52
        if s_51_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#48247 <= s_52_0
        fn_state.gs_48247 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#48247:u8
        let s_53_0: bool = fn_state.gs_48247;
        // N s_53_1: branch s_53_0 b73 b54
        if s_53_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#48248 <= s_54_0
        fn_state.gs_48248 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#48248:u8
        let s_55_0: bool = fn_state.gs_48248;
        // N s_55_1: branch s_55_0 b72 b56
        if s_55_0 {
            return block_72(state, tracer, fn_state);
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
        // N s_56_4: branch s_56_3 b71 b57
        if s_56_3 {
            return block_71(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#48249 <= s_57_0
        fn_state.gs_48249 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#48249:u8
        let s_58_0: bool = fn_state.gs_48249;
        // N s_58_1: branch s_58_0 b65 b59
        if s_58_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #16975u : u32
        let s_59_0: u32 = 16975;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: u8 = {
            let value = state.read_register::<u8>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call IsHighestEL(s_59_1)
        let s_59_2: bool = IsHighestEL(state, tracer, s_59_1);
        // D s_59_3: not s_59_2
        let s_59_3: bool = !s_59_2;
        // N s_59_4: branch s_59_3 b64 b60
        if s_59_3 {
            return block_64(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#48250 <= s_60_0
        fn_state.gs_48250 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#48250:u8
        let s_61_0: bool = fn_state.gs_48250;
        // N s_61_1: branch s_61_0 b63 b62
        if s_61_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #64s : i64
        let s_62_0: i64 = 64;
        // C s_62_1: const #6s : i
        let s_62_1: i128 = 6;
        // C s_62_2: const #103240u : u32
        let s_62_2: u32 = 103240;
        // D s_62_3: read-reg s_62_2:[struct; 16]
        let s_62_3: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_62_2 as isize);
            tracer.read_register(s_62_2 as isize, value);
            value
        };
        // D s_62_4: read-element s_62_3[s_62_1]
        let s_62_4: ProductType5c790c8ef59cc8b2 = s_62_3[(s_62_1) as usize];
        // D s_62_5: write-var ga#40571 <= s_62_4
        fn_state.ga_40571 = s_62_4;
        // D s_62_6: read-var ga#40571.0:struct
        let s_62_6: u64 = fn_state.ga_40571._0;
        // D s_62_7: cast zx s_62_6 -> bv
        let s_62_7: Bits = Bits::new(s_62_6 as u128, 64u16);
        // D s_62_8: read-var t:i
        let s_62_8: i128 = fn_state.t;
        // D s_62_9: call X_set(s_62_8, s_62_0, s_62_7)
        let s_62_9: () = X_set(state, tracer, s_62_8, s_62_0, s_62_7);
        // N s_62_10: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #64s : i64
        let s_63_0: i64 = 64;
        // C s_63_1: const #64s : i
        let s_63_1: i128 = 64;
        // S s_63_2: call Zeros(s_63_1)
        let s_63_2: Bits = Zeros(state, tracer, s_63_1);
        // S s_63_3: cast reint s_63_2 -> u64
        let s_63_3: u64 = (s_63_2.value() as u64);
        // S s_63_4: cast zx s_63_3 -> bv
        let s_63_4: Bits = Bits::new(s_63_3 as u128, 64u16);
        // D s_63_5: read-var t:i
        let s_63_5: i128 = fn_state.t;
        // D s_63_6: call X_set(s_63_5, s_63_0, s_63_4)
        let s_63_6: () = X_set(state, tracer, s_63_5, s_63_0, s_63_4);
        // N s_63_7: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __AMCR_EL0_CG1RZ:u8
        let s_64_0: bool = fn_state.u__AMCR_EL0_CG1RZ;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#48250 <= s_64_4
        fn_state.gs_48250 = s_64_4;
        // N s_64_6: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call Halted(s_65_0)
        let s_65_1: bool = Halted(state, tracer, s_65_0);
        // N s_65_2: branch s_65_1 b70 b66
        if s_65_1 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#48255 <= s_66_0
        fn_state.gs_48255 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#48255:u8
        let s_67_0: bool = fn_state.gs_48255;
        // N s_67_1: branch s_67_0 b69 b68
        if s_67_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #24u : u8
        let s_68_0: u8 = 24;
        // C s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 8u16);
        // C s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (s_68_1.value() as i128);
        // C s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // C s_68_5: const #424u : u32
        let s_68_5: u32 = 424;
        // D s_68_6: read-reg s_68_5:u8
        let s_68_6: u8 = {
            let value = state.read_register::<u8>(s_68_5 as isize);
            tracer.read_register(s_68_5 as isize, value);
            value
        };
        // D s_68_7: call AArch64_SystemAccessTrap(s_68_6, s_68_4)
        let s_68_7: () = AArch64_SystemAccessTrap(state, tracer, s_68_6, s_68_4);
        // N s_68_8: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_69_0: panic
        panic!("{:?}", ());
        // N s_69_1: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __EDSCR_SDD:u8
        let s_70_0: bool = fn_state.u__EDSCR_SDD;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#48255 <= s_70_4
        fn_state.gs_48255 = s_70_4;
        // N s_70_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __CPTR_EL3_TAM:u8
        let s_71_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#48249 <= s_71_4
        fn_state.gs_48249 = s_71_4;
        // N s_71_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #24u : u8
        let s_72_0: u8 = 24;
        // C s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 8u16);
        // C s_72_2: cast zx s_72_1 -> i
        let s_72_2: i128 = (s_72_1.value() as i128);
        // C s_72_3: cast reint s_72_2 -> i64
        let s_72_3: i64 = (s_72_2 as i64);
        // C s_72_4: cast zx s_72_3 -> i
        let s_72_4: i128 = (i128::try_from(s_72_3).unwrap());
        // C s_72_5: const #432u : u32
        let s_72_5: u32 = 432;
        // D s_72_6: read-reg s_72_5:u8
        let s_72_6: u8 = {
            let value = state.read_register::<u8>(s_72_5 as isize);
            tracer.read_register(s_72_5 as isize, value);
            value
        };
        // D s_72_7: call AArch64_SystemAccessTrap(s_72_6, s_72_4)
        let s_72_7: () = AArch64_SystemAccessTrap(state, tracer, s_72_6, s_72_4);
        // N s_72_8: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __HAFGRTR_EL2_AMEVCNTR16_EL0:u8
        let s_73_0: bool = fn_state.u__HAFGRTR_EL2_AMEVCNTR16_EL0;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#48248 <= s_73_4
        fn_state.gs_48248 = s_73_4;
        // N s_73_6: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #424u : u32
        let s_74_0: u32 = 424;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // C s_74_2: const #2u : u8
        let s_74_2: u8 = 2;
        // D s_74_3: cmp-lt s_74_1 s_74_2
        let s_74_3: bool = ((s_74_1) < (s_74_2));
        // D s_74_4: not s_74_3
        let s_74_4: bool = !s_74_3;
        // N s_74_5: branch s_74_4 b77 b75
        if s_74_4 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var __SCR_EL3_FGTEn:u8
        let s_75_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 1u16);
        // C s_75_2: const #1u : u8
        let s_75_2: bool = true;
        // C s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_4: cmp-eq s_75_1 s_75_3
        let s_75_4: bool = ((s_75_1) == (s_75_3));
        // D s_75_5: write-var gs#48246 <= s_75_4
        fn_state.gs_48246 = s_75_4;
        // N s_75_6: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#48246:u8
        let s_76_0: bool = fn_state.gs_48246;
        // D s_76_1: write-var gs#48247 <= s_76_0
        fn_state.gs_48247 = s_76_0;
        // N s_76_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #1u : u8
        let s_77_0: bool = true;
        // D s_77_1: write-var gs#48246 <= s_77_0
        fn_state.gs_48246 = s_77_0;
        // N s_77_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #146u : u32
        let s_78_0: u32 = 146;
        // S s_78_1: call IsFeatureImplemented(s_78_0)
        let s_78_1: bool = IsFeatureImplemented(state, tracer, s_78_0);
        // D s_78_2: write-var gs#48245 <= s_78_1
        fn_state.gs_48245 = s_78_1;
        // N s_78_3: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #24u : u8
        let s_79_0: u8 = 24;
        // C s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 8u16);
        // C s_79_2: cast zx s_79_1 -> i
        let s_79_2: i128 = (s_79_1.value() as i128);
        // C s_79_3: cast reint s_79_2 -> i64
        let s_79_3: i64 = (s_79_2 as i64);
        // C s_79_4: cast zx s_79_3 -> i
        let s_79_4: i128 = (i128::try_from(s_79_3).unwrap());
        // C s_79_5: const #432u : u32
        let s_79_5: u32 = 432;
        // D s_79_6: read-reg s_79_5:u8
        let s_79_6: u8 = {
            let value = state.read_register::<u8>(s_79_5 as isize);
            tracer.read_register(s_79_5 as isize, value);
            value
        };
        // D s_79_7: call AArch64_SystemAccessTrap(s_79_6, s_79_4)
        let s_79_7: () = AArch64_SystemAccessTrap(state, tracer, s_79_6, s_79_4);
        // N s_79_8: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __CPTR_EL2_TAM:u8
        let s_80_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#48244 <= s_80_4
        fn_state.gs_48244 = s_80_4;
        // N s_80_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_81_0: panic
        panic!("{:?}", ());
        // N s_81_1: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var __CPTR_EL3_TAM:u8
        let s_82_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 1u16);
        // C s_82_2: const #1u : u8
        let s_82_2: bool = true;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: write-var gs#48243 <= s_82_4
        fn_state.gs_48243 = s_82_4;
        // N s_82_6: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_83_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_83_1: call __IMPDEF_boolean(s_83_0)
        let s_83_1: bool = u__IMPDEF_boolean(state, tracer, s_83_0);
        // D s_83_2: write-var gs#48242 <= s_83_1
        fn_state.gs_48242 = s_83_1;
        // N s_83_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __EDSCR_SDD:u8
        let s_84_0: bool = fn_state.u__EDSCR_SDD;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#48241 <= s_84_4
        fn_state.gs_48241 = s_84_4;
        // N s_84_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #424u : u32
        let s_85_0: u32 = 424;
        // D s_85_1: read-reg s_85_0:u8
        let s_85_1: u8 = {
            let value = state.read_register::<u8>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // C s_85_2: const #2u : u8
        let s_85_2: u8 = 2;
        // D s_85_3: cmp-lt s_85_1 s_85_2
        let s_85_3: bool = ((s_85_1) < (s_85_2));
        // D s_85_4: write-var gs#48240 <= s_85_3
        fn_state.gs_48240 = s_85_3;
        // N s_85_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call Halted(s_86_0)
        let s_86_1: bool = Halted(state, tracer, s_86_0);
        // N s_86_2: branch s_86_1 b141 b87
        if s_86_1 {
            return block_141(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#48256 <= s_87_0
        fn_state.gs_48256 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#48256:u8
        let s_88_0: bool = fn_state.gs_48256;
        // N s_88_1: branch s_88_0 b140 b89
        if s_88_0 {
            return block_140(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#48257 <= s_89_0
        fn_state.gs_48257 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#48257:u8
        let s_90_0: bool = fn_state.gs_48257;
        // N s_90_1: branch s_90_0 b139 b91
        if s_90_0 {
            return block_139(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#48258 <= s_91_0
        fn_state.gs_48258 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#48258:u8
        let s_92_0: bool = fn_state.gs_48258;
        // N s_92_1: branch s_92_0 b138 b93
        if s_92_0 {
            return block_138(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#48259 <= s_93_0
        fn_state.gs_48259 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#48259:u8
        let s_94_0: bool = fn_state.gs_48259;
        // N s_94_1: branch s_94_0 b137 b95
        if s_94_0 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var __AMUSERENR_EL0_EN:u8
        let s_95_0: bool = fn_state.u__AMUSERENR_EL0_EN;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 1u16);
        // C s_95_2: const #0u : u8
        let s_95_2: bool = false;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // N s_95_5: branch s_95_4 b131 b96
        if s_95_4 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #() : ()
        let s_96_0: () = ();
        // S s_96_1: call EL2Enabled(s_96_0)
        let s_96_1: bool = EL2Enabled(state, tracer, s_96_0);
        // N s_96_2: branch s_96_1 b130 b97
        if s_96_1 {
            return block_130(state, tracer, fn_state);
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
        // D s_97_1: write-var gs#48260 <= s_97_0
        fn_state.gs_48260 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#48260:u8
        let s_98_0: bool = fn_state.gs_48260;
        // N s_98_1: branch s_98_0 b129 b99
        if s_98_0 {
            return block_129(state, tracer, fn_state);
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
        // N s_99_2: branch s_99_1 b128 b100
        if s_99_1 {
            return block_128(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#48261 <= s_100_0
        fn_state.gs_48261 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#48261:u8
        let s_101_0: bool = fn_state.gs_48261;
        // N s_101_1: branch s_101_0 b127 b102
        if s_101_0 {
            return block_127(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#48262 <= s_102_0
        fn_state.gs_48262 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#48262:u8
        let s_103_0: bool = fn_state.gs_48262;
        // N s_103_1: branch s_103_0 b123 b104
        if s_103_0 {
            return block_123(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#48264 <= s_104_0
        fn_state.gs_48264 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#48264:u8
        let s_105_0: bool = fn_state.gs_48264;
        // N s_105_1: branch s_105_0 b122 b106
        if s_105_0 {
            return block_122(state, tracer, fn_state);
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
        // D s_106_1: write-var gs#48265 <= s_106_0
        fn_state.gs_48265 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#48265:u8
        let s_107_0: bool = fn_state.gs_48265;
        // N s_107_1: branch s_107_0 b121 b108
        if s_107_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #424u : u32
        let s_108_0: u32 = 424;
        // D s_108_1: read-reg s_108_0:u8
        let s_108_1: u8 = {
            let value = state.read_register::<u8>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // C s_108_2: const #2u : u8
        let s_108_2: u8 = 2;
        // D s_108_3: cmp-lt s_108_1 s_108_2
        let s_108_3: bool = ((s_108_1) < (s_108_2));
        // N s_108_4: branch s_108_3 b120 b109
        if s_108_3 {
            return block_120(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#48266 <= s_109_0
        fn_state.gs_48266 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#48266:u8
        let s_110_0: bool = fn_state.gs_48266;
        // N s_110_1: branch s_110_0 b114 b111
        if s_110_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var __AMCR_EL0_CG1RZ:u8
        let s_111_0: bool = fn_state.u__AMCR_EL0_CG1RZ;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #1u : u8
        let s_111_2: bool = true;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // N s_111_5: branch s_111_4 b113 b112
        if s_111_4 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #64s : i64
        let s_112_0: i64 = 64;
        // C s_112_1: const #6s : i
        let s_112_1: i128 = 6;
        // C s_112_2: const #103240u : u32
        let s_112_2: u32 = 103240;
        // D s_112_3: read-reg s_112_2:[struct; 16]
        let s_112_3: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_112_2 as isize);
            tracer.read_register(s_112_2 as isize, value);
            value
        };
        // D s_112_4: read-element s_112_3[s_112_1]
        let s_112_4: ProductType5c790c8ef59cc8b2 = s_112_3[(s_112_1) as usize];
        // D s_112_5: write-var ga#40538 <= s_112_4
        fn_state.ga_40538 = s_112_4;
        // D s_112_6: read-var ga#40538.0:struct
        let s_112_6: u64 = fn_state.ga_40538._0;
        // D s_112_7: cast zx s_112_6 -> bv
        let s_112_7: Bits = Bits::new(s_112_6 as u128, 64u16);
        // D s_112_8: read-var t:i
        let s_112_8: i128 = fn_state.t;
        // D s_112_9: call X_set(s_112_8, s_112_0, s_112_7)
        let s_112_9: () = X_set(state, tracer, s_112_8, s_112_0, s_112_7);
        // N s_112_10: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #64s : i64
        let s_113_0: i64 = 64;
        // C s_113_1: const #64s : i
        let s_113_1: i128 = 64;
        // S s_113_2: call Zeros(s_113_1)
        let s_113_2: Bits = Zeros(state, tracer, s_113_1);
        // S s_113_3: cast reint s_113_2 -> u64
        let s_113_3: u64 = (s_113_2.value() as u64);
        // S s_113_4: cast zx s_113_3 -> bv
        let s_113_4: Bits = Bits::new(s_113_3 as u128, 64u16);
        // D s_113_5: read-var t:i
        let s_113_5: i128 = fn_state.t;
        // D s_113_6: call X_set(s_113_5, s_113_0, s_113_4)
        let s_113_6: () = X_set(state, tracer, s_113_5, s_113_0, s_113_4);
        // N s_113_7: return
        return;
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #() : ()
        let s_114_0: () = ();
        // S s_114_1: call Halted(s_114_0)
        let s_114_1: bool = Halted(state, tracer, s_114_0);
        // N s_114_2: branch s_114_1 b119 b115
        if s_114_1 {
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
        // D s_115_1: write-var gs#48271 <= s_115_0
        fn_state.gs_48271 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#48271:u8
        let s_116_0: bool = fn_state.gs_48271;
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
        // C s_117_0: const #24u : u8
        let s_117_0: u8 = 24;
        // C s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 8u16);
        // C s_117_2: cast zx s_117_1 -> i
        let s_117_2: i128 = (s_117_1.value() as i128);
        // C s_117_3: cast reint s_117_2 -> i64
        let s_117_3: i64 = (s_117_2 as i64);
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // C s_117_5: const #424u : u32
        let s_117_5: u32 = 424;
        // D s_117_6: read-reg s_117_5:u8
        let s_117_6: u8 = {
            let value = state.read_register::<u8>(s_117_5 as isize);
            tracer.read_register(s_117_5 as isize, value);
            value
        };
        // D s_117_7: call AArch64_SystemAccessTrap(s_117_6, s_117_4)
        let s_117_7: () = AArch64_SystemAccessTrap(state, tracer, s_117_6, s_117_4);
        // N s_117_8: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_118_0: panic
        panic!("{:?}", ());
        // N s_118_1: return
        return;
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var __EDSCR_SDD:u8
        let s_119_0: bool = fn_state.u__EDSCR_SDD;
        // D s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 1u16);
        // C s_119_2: const #1u : u8
        let s_119_2: bool = true;
        // C s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 1u16);
        // D s_119_4: cmp-eq s_119_1 s_119_3
        let s_119_4: bool = ((s_119_1) == (s_119_3));
        // D s_119_5: write-var gs#48271 <= s_119_4
        fn_state.gs_48271 = s_119_4;
        // N s_119_6: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var __CPTR_EL3_TAM:u8
        let s_120_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 1u16);
        // C s_120_2: const #1u : u8
        let s_120_2: bool = true;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 1u16);
        // D s_120_4: cmp-eq s_120_1 s_120_3
        let s_120_4: bool = ((s_120_1) == (s_120_3));
        // D s_120_5: write-var gs#48266 <= s_120_4
        fn_state.gs_48266 = s_120_4;
        // N s_120_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #24u : u8
        let s_121_0: u8 = 24;
        // C s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 8u16);
        // C s_121_2: cast zx s_121_1 -> i
        let s_121_2: i128 = (s_121_1.value() as i128);
        // C s_121_3: cast reint s_121_2 -> i64
        let s_121_3: i64 = (s_121_2 as i64);
        // C s_121_4: cast zx s_121_3 -> i
        let s_121_4: i128 = (i128::try_from(s_121_3).unwrap());
        // C s_121_5: const #432u : u32
        let s_121_5: u32 = 432;
        // D s_121_6: read-reg s_121_5:u8
        let s_121_6: u8 = {
            let value = state.read_register::<u8>(s_121_5 as isize);
            tracer.read_register(s_121_5 as isize, value);
            value
        };
        // D s_121_7: call AArch64_SystemAccessTrap(s_121_6, s_121_4)
        let s_121_7: () = AArch64_SystemAccessTrap(state, tracer, s_121_6, s_121_4);
        // N s_121_8: return
        return;
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var __HAFGRTR_EL2_AMEVCNTR16_EL0:u8
        let s_122_0: bool = fn_state.u__HAFGRTR_EL2_AMEVCNTR16_EL0;
        // D s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 1u16);
        // C s_122_2: const #1u : u8
        let s_122_2: bool = true;
        // C s_122_3: cast zx s_122_2 -> bv
        let s_122_3: Bits = Bits::new(s_122_2 as u128, 1u16);
        // D s_122_4: cmp-eq s_122_1 s_122_3
        let s_122_4: bool = ((s_122_1) == (s_122_3));
        // D s_122_5: write-var gs#48265 <= s_122_4
        fn_state.gs_48265 = s_122_4;
        // N s_122_6: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #424u : u32
        let s_123_0: u32 = 424;
        // D s_123_1: read-reg s_123_0:u8
        let s_123_1: u8 = {
            let value = state.read_register::<u8>(s_123_0 as isize);
            tracer.read_register(s_123_0 as isize, value);
            value
        };
        // C s_123_2: const #2u : u8
        let s_123_2: u8 = 2;
        // D s_123_3: cmp-lt s_123_1 s_123_2
        let s_123_3: bool = ((s_123_1) < (s_123_2));
        // D s_123_4: not s_123_3
        let s_123_4: bool = !s_123_3;
        // N s_123_5: branch s_123_4 b126 b124
        if s_123_4 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var __SCR_EL3_FGTEn:u8
        let s_124_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 1u16);
        // C s_124_2: const #1u : u8
        let s_124_2: bool = true;
        // C s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 1u16);
        // D s_124_4: cmp-eq s_124_1 s_124_3
        let s_124_4: bool = ((s_124_1) == (s_124_3));
        // D s_124_5: write-var gs#48263 <= s_124_4
        fn_state.gs_48263 = s_124_4;
        // N s_124_6: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#48263:u8
        let s_125_0: bool = fn_state.gs_48263;
        // D s_125_1: write-var gs#48264 <= s_125_0
        fn_state.gs_48264 = s_125_0;
        // N s_125_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #1u : u8
        let s_126_0: bool = true;
        // D s_126_1: write-var gs#48263 <= s_126_0
        fn_state.gs_48263 = s_126_0;
        // N s_126_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #146u : u32
        let s_127_0: u32 = 146;
        // S s_127_1: call IsFeatureImplemented(s_127_0)
        let s_127_1: bool = IsFeatureImplemented(state, tracer, s_127_0);
        // D s_127_2: write-var gs#48262 <= s_127_1
        fn_state.gs_48262 = s_127_1;
        // N s_127_3: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #102552u : u32
        let s_128_0: u32 = 102552;
        // D s_128_1: read-reg s_128_0:struct
        let s_128_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // D s_128_2: call _get_HCR_EL2_Type_E2H(s_128_1)
        let s_128_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_128_1);
        // C s_128_3: const #102552u : u32
        let s_128_3: u32 = 102552;
        // D s_128_4: read-reg s_128_3:struct
        let s_128_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_128_3 as isize);
            tracer.read_register(s_128_3 as isize, value);
            value
        };
        // D s_128_5: call _get_HCR_EL2_Type_TGE(s_128_4)
        let s_128_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_128_4);
        // D s_128_6: cast zx s_128_2 -> bv
        let s_128_6: Bits = Bits::new(s_128_2 as u128, 1u16);
        // D s_128_7: cast zx s_128_5 -> bv
        let s_128_7: Bits = Bits::new(s_128_5 as u128, 1u16);
        // D s_128_8: cast reint s_128_6 -> u128
        let s_128_8: u128 = (s_128_6.value() as u128);
        // D s_128_9: size-of s_128_6
        let s_128_9: u16 = s_128_6.length();
        // D s_128_10: cast reint s_128_7 -> u128
        let s_128_10: u128 = (s_128_7.value() as u128);
        // D s_128_11: size-of s_128_7
        let s_128_11: u16 = s_128_7.length();
        // D s_128_12: lsl s_128_8 s_128_11
        let s_128_12: u128 = s_128_8 << s_128_11;
        // D s_128_13: or s_128_12 s_128_10
        let s_128_13: u128 = ((s_128_12) | (s_128_10));
        // D s_128_14: add s_128_9 s_128_11
        let s_128_14: u16 = (s_128_9 + s_128_11);
        // D s_128_15: create-bits s_128_13 s_128_14
        let s_128_15: Bits = Bits::new(s_128_13, s_128_14);
        // D s_128_16: cast reint s_128_15 -> u8
        let s_128_16: u8 = (s_128_15.value() as u8);
        // D s_128_17: cast zx s_128_16 -> bv
        let s_128_17: Bits = Bits::new(s_128_16 as u128, 2u16);
        // C s_128_18: const #3u : u8
        let s_128_18: u8 = 3;
        // C s_128_19: cast zx s_128_18 -> bv
        let s_128_19: Bits = Bits::new(s_128_18 as u128, 2u16);
        // D s_128_20: cmp-ne s_128_17 s_128_19
        let s_128_20: bool = ((s_128_17) != (s_128_19));
        // D s_128_21: write-var gs#48261 <= s_128_20
        fn_state.gs_48261 = s_128_20;
        // N s_128_22: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #24u : u8
        let s_129_0: u8 = 24;
        // C s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 8u16);
        // C s_129_2: cast zx s_129_1 -> i
        let s_129_2: i128 = (s_129_1.value() as i128);
        // C s_129_3: cast reint s_129_2 -> i64
        let s_129_3: i64 = (s_129_2 as i64);
        // C s_129_4: cast zx s_129_3 -> i
        let s_129_4: i128 = (i128::try_from(s_129_3).unwrap());
        // C s_129_5: const #432u : u32
        let s_129_5: u32 = 432;
        // D s_129_6: read-reg s_129_5:u8
        let s_129_6: u8 = {
            let value = state.read_register::<u8>(s_129_5 as isize);
            tracer.read_register(s_129_5 as isize, value);
            value
        };
        // D s_129_7: call AArch64_SystemAccessTrap(s_129_6, s_129_4)
        let s_129_7: () = AArch64_SystemAccessTrap(state, tracer, s_129_6, s_129_4);
        // N s_129_8: return
        return;
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var __CPTR_EL2_TAM:u8
        let s_130_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_130_1: cast zx s_130_0 -> bv
        let s_130_1: Bits = Bits::new(s_130_0 as u128, 1u16);
        // C s_130_2: const #1u : u8
        let s_130_2: bool = true;
        // C s_130_3: cast zx s_130_2 -> bv
        let s_130_3: Bits = Bits::new(s_130_2 as u128, 1u16);
        // D s_130_4: cmp-eq s_130_1 s_130_3
        let s_130_4: bool = ((s_130_1) == (s_130_3));
        // D s_130_5: write-var gs#48260 <= s_130_4
        fn_state.gs_48260 = s_130_4;
        // N s_130_6: jump b98
        return block_98(state, tracer, fn_state);
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
        // N s_131_2: branch s_131_1 b136 b132
        if s_131_1 {
            return block_136(state, tracer, fn_state);
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
        // D s_132_1: write-var gs#48272 <= s_132_0
        fn_state.gs_48272 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#48272:u8
        let s_133_0: bool = fn_state.gs_48272;
        // N s_133_1: branch s_133_0 b135 b134
        if s_133_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #24u : u8
        let s_134_0: u8 = 24;
        // C s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 8u16);
        // C s_134_2: cast zx s_134_1 -> i
        let s_134_2: i128 = (s_134_1.value() as i128);
        // C s_134_3: cast reint s_134_2 -> i64
        let s_134_3: i64 = (s_134_2 as i64);
        // C s_134_4: cast zx s_134_3 -> i
        let s_134_4: i128 = (i128::try_from(s_134_3).unwrap());
        // C s_134_5: const #440u : u32
        let s_134_5: u32 = 440;
        // D s_134_6: read-reg s_134_5:u8
        let s_134_6: u8 = {
            let value = state.read_register::<u8>(s_134_5 as isize);
            tracer.read_register(s_134_5 as isize, value);
            value
        };
        // D s_134_7: call AArch64_SystemAccessTrap(s_134_6, s_134_4)
        let s_134_7: () = AArch64_SystemAccessTrap(state, tracer, s_134_6, s_134_4);
        // N s_134_8: return
        return;
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #24u : u8
        let s_135_0: u8 = 24;
        // C s_135_1: cast zx s_135_0 -> bv
        let s_135_1: Bits = Bits::new(s_135_0 as u128, 8u16);
        // C s_135_2: cast zx s_135_1 -> i
        let s_135_2: i128 = (s_135_1.value() as i128);
        // C s_135_3: cast reint s_135_2 -> i64
        let s_135_3: i64 = (s_135_2 as i64);
        // C s_135_4: cast zx s_135_3 -> i
        let s_135_4: i128 = (i128::try_from(s_135_3).unwrap());
        // C s_135_5: const #432u : u32
        let s_135_5: u32 = 432;
        // D s_135_6: read-reg s_135_5:u8
        let s_135_6: u8 = {
            let value = state.read_register::<u8>(s_135_5 as isize);
            tracer.read_register(s_135_5 as isize, value);
            value
        };
        // D s_135_7: call AArch64_SystemAccessTrap(s_135_6, s_135_4)
        let s_135_7: () = AArch64_SystemAccessTrap(state, tracer, s_135_6, s_135_4);
        // N s_135_8: return
        return;
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var __HCR_EL2_TGE:u8
        let s_136_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 1u16);
        // C s_136_2: const #1u : u8
        let s_136_2: bool = true;
        // C s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 1u16);
        // D s_136_4: cmp-eq s_136_1 s_136_3
        let s_136_4: bool = ((s_136_1) == (s_136_3));
        // D s_136_5: write-var gs#48272 <= s_136_4
        fn_state.gs_48272 = s_136_4;
        // N s_136_6: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_137_0: panic
        panic!("{:?}", ());
        // N s_137_1: return
        return;
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var __CPTR_EL3_TAM:u8
        let s_138_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_138_1: cast zx s_138_0 -> bv
        let s_138_1: Bits = Bits::new(s_138_0 as u128, 1u16);
        // C s_138_2: const #1u : u8
        let s_138_2: bool = true;
        // C s_138_3: cast zx s_138_2 -> bv
        let s_138_3: Bits = Bits::new(s_138_2 as u128, 1u16);
        // D s_138_4: cmp-eq s_138_1 s_138_3
        let s_138_4: bool = ((s_138_1) == (s_138_3));
        // D s_138_5: write-var gs#48259 <= s_138_4
        fn_state.gs_48259 = s_138_4;
        // N s_138_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_139_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_139_1: call __IMPDEF_boolean(s_139_0)
        let s_139_1: bool = u__IMPDEF_boolean(state, tracer, s_139_0);
        // D s_139_2: write-var gs#48258 <= s_139_1
        fn_state.gs_48258 = s_139_1;
        // N s_139_3: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var __EDSCR_SDD:u8
        let s_140_0: bool = fn_state.u__EDSCR_SDD;
        // D s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 1u16);
        // C s_140_2: const #1u : u8
        let s_140_2: bool = true;
        // C s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 1u16);
        // D s_140_4: cmp-eq s_140_1 s_140_3
        let s_140_4: bool = ((s_140_1) == (s_140_3));
        // D s_140_5: write-var gs#48257 <= s_140_4
        fn_state.gs_48257 = s_140_4;
        // N s_140_6: jump b90
        return block_90(state, tracer, fn_state);
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
        // D s_141_4: write-var gs#48256 <= s_141_3
        fn_state.gs_48256 = s_141_3;
        // N s_141_5: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_142_0: panic
        panic!("{:?}", ());
        // N s_142_1: return
        return;
    }
}
