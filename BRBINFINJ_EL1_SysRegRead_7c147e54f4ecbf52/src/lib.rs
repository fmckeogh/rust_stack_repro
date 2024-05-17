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
use EDSCR_read::*;
use X_set::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use IsFeatureImplemented::*;
use Halted::*;
use u_get_MDCR_EL3_Type_SBRBE::*;
use u_get_EDSCR_Type_SDD::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use EL2Enabled::*;
use u_get_HDFGRTR_EL2_Type_nBRBDATA::*;
use u__get_BRBINFINJ_EL1::*;
use u_get_SCR_EL3_Type_NS::*;
use common::*;
pub fn BRBINFINJ_EL1_SysRegRead_7c147e54f4ecbf52<T: Tracer>(
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
        u__SCR_EL3_NS: bool,
        gs_50136: bool,
        gs_50122: bool,
        gs_50123: bool,
        gs_50109: bool,
        ga_45093: ProductType5c790c8ef59cc8b2,
        gs_50084: bool,
        gs_50088: bool,
        gs_50128: bool,
        gs_50093: bool,
        gs_50126: bool,
        gs_50127: bool,
        gs_50111: bool,
        u__PSTATE_EL: u8,
        gs_50134: bool,
        gs_50087: bool,
        u__HDFGRTR_EL2_nBRBDATA: bool,
        gs_50124: bool,
        gs_50086: bool,
        gs_50110: bool,
        gs_50106: bool,
        gs_50114: bool,
        u__EDSCR_SDD: bool,
        gs_50121: bool,
        gs_50105: bool,
        gs_50080: bool,
        gs_50120: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_50095: bool,
        gs_50097: bool,
        gs_50096: bool,
        ga_45055: ProductType5c790c8ef59cc8b2,
        gs_50103: bool,
        gs_50108: bool,
        gs_50081: bool,
        gs_50085: bool,
        gs_50102: bool,
        gs_50107: bool,
        gs_50094: bool,
        u__MDCR_EL3_SBRBE: u8,
        gs_50083: bool,
        gs_50112: bool,
        ga_45089: ProductType5c790c8ef59cc8b2,
        gs_50133: bool,
        gs_50137: bool,
        gs_50125: bool,
        gs_50113: bool,
        gs_50115: bool,
        gs_50082: bool,
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
        // D s_0_9: call _get_MDCR_EL3_Type_SBRBE(s_0_8)
        let s_0_9: u8 = u_get_MDCR_EL3_Type_SBRBE(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_SBRBE <= s_0_9
        fn_state.u__MDCR_EL3_SBRBE = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_NS(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_NS <= s_0_13
        fn_state.u__SCR_EL3_NS = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_FGTEn(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_FGTEn <= s_0_17
        fn_state.u__SCR_EL3_FGTEn = s_0_17;
        // C s_0_19: const #19144u : u32
        let s_0_19: u32 = 19144;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HDFGRTR_EL2_Type_nBRBDATA(s_0_20)
        let s_0_21: bool = u_get_HDFGRTR_EL2_Type_nBRBDATA(state, tracer, s_0_20);
        // D s_0_22: write-var __HDFGRTR_EL2_nBRBDATA <= s_0_21
        fn_state.u__HDFGRTR_EL2_nBRBDATA = s_0_21;
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
        // N s_0_29: branch s_0_28 b154 b1
        if s_0_28 {
            return block_154(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b73 b2
        if s_1_5 {
            return block_73(state, tracer, fn_state);
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
        // C s_5_1: const #10168u : u32
        let s_5_1: u32 = 10168;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_BRBINFINJ_EL1(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_BRBINFINJ_EL1(
            state,
            tracer,
            s_5_2,
        );
        // D s_5_4: write-var ga#45093 <= s_5_3
        fn_state.ga_45093 = s_5_3;
        // D s_5_5: read-var ga#45093.0:struct
        let s_5_5: u64 = fn_state.ga_45093._0;
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
        // N s_6_2: branch s_6_1 b72 b7
        if s_6_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#50080 <= s_7_0
        fn_state.gs_50080 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#50080:u8
        let s_8_0: bool = fn_state.gs_50080;
        // N s_8_1: branch s_8_0 b71 b9
        if s_8_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#50081 <= s_9_0
        fn_state.gs_50081 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#50081:u8
        let s_10_0: bool = fn_state.gs_50081;
        // N s_10_1: branch s_10_0 b70 b11
        if s_10_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#50082 <= s_11_0
        fn_state.gs_50082 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#50082:u8
        let s_12_0: bool = fn_state.gs_50082;
        // N s_12_1: branch s_12_0 b69 b13
        if s_12_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#50083 <= s_13_0
        fn_state.gs_50083 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#50083:u8
        let s_14_0: bool = fn_state.gs_50083;
        // N s_14_1: branch s_14_0 b68 b15
        if s_14_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#50084 <= s_15_0
        fn_state.gs_50084 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#50084:u8
        let s_16_0: bool = fn_state.gs_50084;
        // N s_16_1: branch s_16_0 b67 b17
        if s_16_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call Halted(s_17_0)
        let s_17_1: bool = Halted(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b66 b18
        if s_17_1 {
            return block_66(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#50085 <= s_18_0
        fn_state.gs_50085 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#50085:u8
        let s_19_0: bool = fn_state.gs_50085;
        // N s_19_1: branch s_19_0 b65 b20
        if s_19_0 {
            return block_65(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#50086 <= s_20_0
        fn_state.gs_50086 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#50086:u8
        let s_21_0: bool = fn_state.gs_50086;
        // N s_21_1: branch s_21_0 b64 b22
        if s_21_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#50087 <= s_22_0
        fn_state.gs_50087 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#50087:u8
        let s_23_0: bool = fn_state.gs_50087;
        // N s_23_1: branch s_23_0 b60 b24
        if s_23_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#50093 <= s_24_0
        fn_state.gs_50093 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#50093:u8
        let s_25_0: bool = fn_state.gs_50093;
        // N s_25_1: branch s_25_0 b59 b26
        if s_25_0 {
            return block_59(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#50094 <= s_26_0
        fn_state.gs_50094 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#50094:u8
        let s_27_0: bool = fn_state.gs_50094;
        // N s_27_1: branch s_27_0 b58 b28
        if s_27_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
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
        // C s_28_2: const #2u : u8
        let s_28_2: u8 = 2;
        // D s_28_3: cmp-lt s_28_1 s_28_2
        let s_28_3: bool = ((s_28_1) < (s_28_2));
        // N s_28_4: branch s_28_3 b57 b29
        if s_28_3 {
            return block_57(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#50095 <= s_29_0
        fn_state.gs_50095 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#50095:u8
        let s_30_0: bool = fn_state.gs_50095;
        // N s_30_1: branch s_30_0 b56 b31
        if s_30_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#50096 <= s_31_0
        fn_state.gs_50096 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#50096:u8
        let s_32_0: bool = fn_state.gs_50096;
        // N s_32_1: branch s_32_0 b50 b33
        if s_32_0 {
            return block_50(state, tracer, fn_state);
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
        // N s_33_4: branch s_33_3 b46 b34
        if s_33_3 {
            return block_46(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#50102 <= s_34_0
        fn_state.gs_50102 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#50102:u8
        let s_35_0: bool = fn_state.gs_50102;
        // N s_35_1: branch s_35_0 b45 b36
        if s_35_0 {
            return block_45(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#50103 <= s_36_0
        fn_state.gs_50103 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#50103:u8
        let s_37_0: bool = fn_state.gs_50103;
        // N s_37_1: branch s_37_0 b39 b38
        if s_37_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #64s : i64
        let s_38_0: i64 = 64;
        // C s_38_1: const #10168u : u32
        let s_38_1: u32 = 10168;
        // D s_38_2: read-reg s_38_1:struct
        let s_38_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_1 as isize);
            tracer.read_register(s_38_1 as isize, value);
            value
        };
        // D s_38_3: call __get_BRBINFINJ_EL1(s_38_2)
        let s_38_3: ProductType5c790c8ef59cc8b2 = u__get_BRBINFINJ_EL1(
            state,
            tracer,
            s_38_2,
        );
        // D s_38_4: write-var ga#45089 <= s_38_3
        fn_state.ga_45089 = s_38_3;
        // D s_38_5: read-var ga#45089.0:struct
        let s_38_5: u64 = fn_state.ga_45089._0;
        // D s_38_6: cast zx s_38_5 -> bv
        let s_38_6: Bits = Bits::new(s_38_5 as u128, 64u16);
        // D s_38_7: read-var t:i
        let s_38_7: i128 = fn_state.t;
        // D s_38_8: call X_set(s_38_7, s_38_0, s_38_6)
        let s_38_8: () = X_set(state, tracer, s_38_7, s_38_0, s_38_6);
        // N s_38_9: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call Halted(s_39_0)
        let s_39_1: bool = Halted(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b44 b40
        if s_39_1 {
            return block_44(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#50105 <= s_40_0
        fn_state.gs_50105 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#50105:u8
        let s_41_0: bool = fn_state.gs_50105;
        // N s_41_1: branch s_41_0 b43 b42
        if s_41_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #24u : u8
        let s_42_0: u8 = 24;
        // C s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 8u16);
        // C s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (s_42_1.value() as i128);
        // C s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: cast zx s_42_3 -> i
        let s_42_4: i128 = (i128::try_from(s_42_3).unwrap());
        // C s_42_5: const #424u : u32
        let s_42_5: u32 = 424;
        // D s_42_6: read-reg s_42_5:u8
        let s_42_6: u8 = {
            let value = state.read_register::<u8>(s_42_5 as isize);
            tracer.read_register(s_42_5 as isize, value);
            value
        };
        // D s_42_7: call AArch64_SystemAccessTrap(s_42_6, s_42_4)
        let s_42_7: () = AArch64_SystemAccessTrap(state, tracer, s_42_6, s_42_4);
        // N s_42_8: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: panic
        panic!("{:?}", ());
        // N s_43_1: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __EDSCR_SDD:u8
        let s_44_0: bool = fn_state.u__EDSCR_SDD;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #1u : u8
        let s_44_2: bool = true;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#50105 <= s_44_4
        fn_state.gs_50105 = s_44_4;
        // N s_44_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var __SCR_EL3_NS:u8
        let s_45_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 1u16);
        // C s_45_2: const #1u : u8
        let s_45_2: bool = true;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: write-var gs#50103 <= s_45_4
        fn_state.gs_50103 = s_45_4;
        // N s_45_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __MDCR_EL3_SBRBE:u8
        let s_46_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_46_1: const #0s : i
        let s_46_1: i128 = 0;
        // D s_46_2: cast zx s_46_0 -> bv
        let s_46_2: Bits = Bits::new(s_46_0 as u128, 2u16);
        // C s_46_3: const #1s : i64
        let s_46_3: i64 = 1;
        // C s_46_4: cast zx s_46_3 -> i
        let s_46_4: i128 = (i128::try_from(s_46_3).unwrap());
        // C s_46_5: const #0s : i
        let s_46_5: i128 = 0;
        // C s_46_6: add s_46_5 s_46_4
        let s_46_6: i128 = (s_46_5 + s_46_4);
        // D s_46_7: bit-extract s_46_2 s_46_1 s_46_6
        let s_46_7: Bits = (Bits::new(
            ((s_46_2) >> (s_46_1)).value(),
            u16::try_from(s_46_6).unwrap(),
        ));
        // D s_46_8: cast reint s_46_7 -> u8
        let s_46_8: bool = ((s_46_7.value()) != 0);
        // D s_46_9: cast zx s_46_8 -> bv
        let s_46_9: Bits = Bits::new(s_46_8 as u128, 1u16);
        // C s_46_10: const #0u : u8
        let s_46_10: bool = false;
        // C s_46_11: cast zx s_46_10 -> bv
        let s_46_11: Bits = Bits::new(s_46_10 as u128, 1u16);
        // D s_46_12: cmp-eq s_46_9 s_46_11
        let s_46_12: bool = ((s_46_9) == (s_46_11));
        // D s_46_13: not s_46_12
        let s_46_13: bool = !s_46_12;
        // N s_46_14: branch s_46_13 b49 b47
        if s_46_13 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#50097 <= s_47_0
        fn_state.gs_50097 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#50097:u8
        let s_48_0: bool = fn_state.gs_50097;
        // D s_48_1: write-var gs#50102 <= s_48_0
        fn_state.gs_50102 = s_48_0;
        // N s_48_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#50097 <= s_49_0
        fn_state.gs_50097 = s_49_0;
        // N s_49_2: jump b48
        return block_48(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#50106 <= s_51_0
        fn_state.gs_50106 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#50106:u8
        let s_52_0: bool = fn_state.gs_50106;
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
        // D s_55_5: write-var gs#50106 <= s_55_4
        fn_state.gs_50106 = s_55_4;
        // N s_55_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __SCR_EL3_NS:u8
        let s_56_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #0u : u8
        let s_56_2: bool = false;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#50096 <= s_56_4
        fn_state.gs_50096 = s_56_4;
        // N s_56_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var __MDCR_EL3_SBRBE:u8
        let s_57_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 2u16);
        // C s_57_2: const #3u : u8
        let s_57_2: u8 = 3;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 2u16);
        // D s_57_4: cmp-ne s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) != (s_57_3));
        // D s_57_5: write-var gs#50095 <= s_57_4
        fn_state.gs_50095 = s_57_4;
        // N s_57_6: jump b30
        return block_30(state, tracer, fn_state);
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
        // D s_59_0: read-var __SCR_EL3_NS:u8
        let s_59_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #1u : u8
        let s_59_2: bool = true;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#50094 <= s_59_4
        fn_state.gs_50094 = s_59_4;
        // N s_59_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __MDCR_EL3_SBRBE:u8
        let s_60_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_60_1: const #0s : i
        let s_60_1: i128 = 0;
        // D s_60_2: cast zx s_60_0 -> bv
        let s_60_2: Bits = Bits::new(s_60_0 as u128, 2u16);
        // C s_60_3: const #1s : i64
        let s_60_3: i64 = 1;
        // C s_60_4: cast zx s_60_3 -> i
        let s_60_4: i128 = (i128::try_from(s_60_3).unwrap());
        // C s_60_5: const #0s : i
        let s_60_5: i128 = 0;
        // C s_60_6: add s_60_5 s_60_4
        let s_60_6: i128 = (s_60_5 + s_60_4);
        // D s_60_7: bit-extract s_60_2 s_60_1 s_60_6
        let s_60_7: Bits = (Bits::new(
            ((s_60_2) >> (s_60_1)).value(),
            u16::try_from(s_60_6).unwrap(),
        ));
        // D s_60_8: cast reint s_60_7 -> u8
        let s_60_8: bool = ((s_60_7.value()) != 0);
        // D s_60_9: cast zx s_60_8 -> bv
        let s_60_9: Bits = Bits::new(s_60_8 as u128, 1u16);
        // C s_60_10: const #0u : u8
        let s_60_10: bool = false;
        // C s_60_11: cast zx s_60_10 -> bv
        let s_60_11: Bits = Bits::new(s_60_10 as u128, 1u16);
        // D s_60_12: cmp-eq s_60_9 s_60_11
        let s_60_12: bool = ((s_60_9) == (s_60_11));
        // D s_60_13: not s_60_12
        let s_60_13: bool = !s_60_12;
        // N s_60_14: branch s_60_13 b63 b61
        if s_60_13 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#50088 <= s_61_0
        fn_state.gs_50088 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#50088:u8
        let s_62_0: bool = fn_state.gs_50088;
        // D s_62_1: write-var gs#50093 <= s_62_0
        fn_state.gs_50093 = s_62_0;
        // N s_62_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#50088 <= s_63_0
        fn_state.gs_50088 = s_63_0;
        // N s_63_2: jump b62
        return block_62(state, tracer, fn_state);
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
        // D s_64_2: write-var gs#50087 <= s_64_1
        fn_state.gs_50087 = s_64_1;
        // N s_64_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var __EDSCR_SDD:u8
        let s_65_0: bool = fn_state.u__EDSCR_SDD;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #1u : u8
        let s_65_2: bool = true;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#50086 <= s_65_4
        fn_state.gs_50086 = s_65_4;
        // N s_65_6: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_66_4: write-var gs#50085 <= s_66_3
        fn_state.gs_50085 = s_66_3;
        // N s_66_5: jump b19
        return block_19(state, tracer, fn_state);
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
        // D s_68_0: read-var __SCR_EL3_NS:u8
        let s_68_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #0u : u8
        let s_68_2: bool = false;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#50084 <= s_68_4
        fn_state.gs_50084 = s_68_4;
        // N s_68_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __MDCR_EL3_SBRBE:u8
        let s_69_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 2u16);
        // C s_69_2: const #3u : u8
        let s_69_2: u8 = 3;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 2u16);
        // D s_69_4: cmp-ne s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) != (s_69_3));
        // D s_69_5: write-var gs#50083 <= s_69_4
        fn_state.gs_50083 = s_69_4;
        // N s_69_6: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_70_2: write-var gs#50082 <= s_70_1
        fn_state.gs_50082 = s_70_1;
        // N s_70_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __EDSCR_SDD:u8
        let s_71_0: bool = fn_state.u__EDSCR_SDD;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#50081 <= s_71_4
        fn_state.gs_50081 = s_71_4;
        // N s_71_6: jump b10
        return block_10(state, tracer, fn_state);
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
        // D s_72_4: write-var gs#50080 <= s_72_3
        fn_state.gs_50080 = s_72_3;
        // N s_72_5: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_73_2: branch s_73_1 b153 b74
        if s_73_1 {
            return block_153(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#50107 <= s_74_0
        fn_state.gs_50107 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#50107:u8
        let s_75_0: bool = fn_state.gs_50107;
        // N s_75_1: branch s_75_0 b152 b76
        if s_75_0 {
            return block_152(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#50108 <= s_76_0
        fn_state.gs_50108 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#50108:u8
        let s_77_0: bool = fn_state.gs_50108;
        // N s_77_1: branch s_77_0 b151 b78
        if s_77_0 {
            return block_151(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#50109 <= s_78_0
        fn_state.gs_50109 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#50109:u8
        let s_79_0: bool = fn_state.gs_50109;
        // N s_79_1: branch s_79_0 b150 b80
        if s_79_0 {
            return block_150(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#50110 <= s_80_0
        fn_state.gs_50110 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#50110:u8
        let s_81_0: bool = fn_state.gs_50110;
        // N s_81_1: branch s_81_0 b149 b82
        if s_81_0 {
            return block_149(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#50111 <= s_82_0
        fn_state.gs_50111 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#50111:u8
        let s_83_0: bool = fn_state.gs_50111;
        // N s_83_1: branch s_83_0 b148 b84
        if s_83_0 {
            return block_148(state, tracer, fn_state);
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
        // N s_84_2: branch s_84_1 b147 b85
        if s_84_1 {
            return block_147(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#50112 <= s_85_0
        fn_state.gs_50112 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#50112:u8
        let s_86_0: bool = fn_state.gs_50112;
        // N s_86_1: branch s_86_0 b146 b87
        if s_86_0 {
            return block_146(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#50113 <= s_87_0
        fn_state.gs_50113 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#50113:u8
        let s_88_0: bool = fn_state.gs_50113;
        // N s_88_1: branch s_88_0 b145 b89
        if s_88_0 {
            return block_145(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#50114 <= s_89_0
        fn_state.gs_50114 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#50114:u8
        let s_90_0: bool = fn_state.gs_50114;
        // N s_90_1: branch s_90_0 b141 b91
        if s_90_0 {
            return block_141(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#50120 <= s_91_0
        fn_state.gs_50120 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#50120:u8
        let s_92_0: bool = fn_state.gs_50120;
        // N s_92_1: branch s_92_0 b140 b93
        if s_92_0 {
            return block_140(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#50121 <= s_93_0
        fn_state.gs_50121 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#50121:u8
        let s_94_0: bool = fn_state.gs_50121;
        // N s_94_1: branch s_94_0 b139 b95
        if s_94_0 {
            return block_139(state, tracer, fn_state);
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
        // N s_95_2: branch s_95_1 b138 b96
        if s_95_1 {
            return block_138(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#50122 <= s_96_0
        fn_state.gs_50122 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#50122:u8
        let s_97_0: bool = fn_state.gs_50122;
        // N s_97_1: branch s_97_0 b134 b98
        if s_97_0 {
            return block_134(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#50124 <= s_98_0
        fn_state.gs_50124 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#50124:u8
        let s_99_0: bool = fn_state.gs_50124;
        // N s_99_1: branch s_99_0 b133 b100
        if s_99_0 {
            return block_133(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#50125 <= s_100_0
        fn_state.gs_50125 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#50125:u8
        let s_101_0: bool = fn_state.gs_50125;
        // N s_101_1: branch s_101_0 b132 b102
        if s_101_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #424u : u32
        let s_102_0: u32 = 424;
        // D s_102_1: read-reg s_102_0:u8
        let s_102_1: u8 = {
            let value = state.read_register::<u8>(s_102_0 as isize);
            tracer.read_register(s_102_0 as isize, value);
            value
        };
        // C s_102_2: const #2u : u8
        let s_102_2: u8 = 2;
        // D s_102_3: cmp-lt s_102_1 s_102_2
        let s_102_3: bool = ((s_102_1) < (s_102_2));
        // N s_102_4: branch s_102_3 b131 b103
        if s_102_3 {
            return block_131(state, tracer, fn_state);
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
        // D s_103_1: write-var gs#50126 <= s_103_0
        fn_state.gs_50126 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#50126:u8
        let s_104_0: bool = fn_state.gs_50126;
        // N s_104_1: branch s_104_0 b130 b105
        if s_104_0 {
            return block_130(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#50127 <= s_105_0
        fn_state.gs_50127 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#50127:u8
        let s_106_0: bool = fn_state.gs_50127;
        // N s_106_1: branch s_106_0 b124 b107
        if s_106_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #424u : u32
        let s_107_0: u32 = 424;
        // D s_107_1: read-reg s_107_0:u8
        let s_107_1: u8 = {
            let value = state.read_register::<u8>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // C s_107_2: const #2u : u8
        let s_107_2: u8 = 2;
        // D s_107_3: cmp-lt s_107_1 s_107_2
        let s_107_3: bool = ((s_107_1) < (s_107_2));
        // N s_107_4: branch s_107_3 b120 b108
        if s_107_3 {
            return block_120(state, tracer, fn_state);
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
        // D s_108_1: write-var gs#50133 <= s_108_0
        fn_state.gs_50133 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#50133:u8
        let s_109_0: bool = fn_state.gs_50133;
        // N s_109_1: branch s_109_0 b119 b110
        if s_109_0 {
            return block_119(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#50134 <= s_110_0
        fn_state.gs_50134 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#50134:u8
        let s_111_0: bool = fn_state.gs_50134;
        // N s_111_1: branch s_111_0 b113 b112
        if s_111_0 {
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
        // C s_112_1: const #10168u : u32
        let s_112_1: u32 = 10168;
        // D s_112_2: read-reg s_112_1:struct
        let s_112_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_112_1 as isize);
            tracer.read_register(s_112_1 as isize, value);
            value
        };
        // D s_112_3: call __get_BRBINFINJ_EL1(s_112_2)
        let s_112_3: ProductType5c790c8ef59cc8b2 = u__get_BRBINFINJ_EL1(
            state,
            tracer,
            s_112_2,
        );
        // D s_112_4: write-var ga#45055 <= s_112_3
        fn_state.ga_45055 = s_112_3;
        // D s_112_5: read-var ga#45055.0:struct
        let s_112_5: u64 = fn_state.ga_45055._0;
        // D s_112_6: cast zx s_112_5 -> bv
        let s_112_6: Bits = Bits::new(s_112_5 as u128, 64u16);
        // D s_112_7: read-var t:i
        let s_112_7: i128 = fn_state.t;
        // D s_112_8: call X_set(s_112_7, s_112_0, s_112_6)
        let s_112_8: () = X_set(state, tracer, s_112_7, s_112_0, s_112_6);
        // N s_112_9: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #() : ()
        let s_113_0: () = ();
        // S s_113_1: call Halted(s_113_0)
        let s_113_1: bool = Halted(state, tracer, s_113_0);
        // N s_113_2: branch s_113_1 b118 b114
        if s_113_1 {
            return block_118(state, tracer, fn_state);
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
        // D s_114_1: write-var gs#50136 <= s_114_0
        fn_state.gs_50136 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#50136:u8
        let s_115_0: bool = fn_state.gs_50136;
        // N s_115_1: branch s_115_0 b117 b116
        if s_115_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #24u : u8
        let s_116_0: u8 = 24;
        // C s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 8u16);
        // C s_116_2: cast zx s_116_1 -> i
        let s_116_2: i128 = (s_116_1.value() as i128);
        // C s_116_3: cast reint s_116_2 -> i64
        let s_116_3: i64 = (s_116_2 as i64);
        // C s_116_4: cast zx s_116_3 -> i
        let s_116_4: i128 = (i128::try_from(s_116_3).unwrap());
        // C s_116_5: const #424u : u32
        let s_116_5: u32 = 424;
        // D s_116_6: read-reg s_116_5:u8
        let s_116_6: u8 = {
            let value = state.read_register::<u8>(s_116_5 as isize);
            tracer.read_register(s_116_5 as isize, value);
            value
        };
        // D s_116_7: call AArch64_SystemAccessTrap(s_116_6, s_116_4)
        let s_116_7: () = AArch64_SystemAccessTrap(state, tracer, s_116_6, s_116_4);
        // N s_116_8: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_117_0: panic
        panic!("{:?}", ());
        // N s_117_1: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var __EDSCR_SDD:u8
        let s_118_0: bool = fn_state.u__EDSCR_SDD;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #1u : u8
        let s_118_2: bool = true;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: write-var gs#50136 <= s_118_4
        fn_state.gs_50136 = s_118_4;
        // N s_118_6: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var __SCR_EL3_NS:u8
        let s_119_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 1u16);
        // C s_119_2: const #1u : u8
        let s_119_2: bool = true;
        // C s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 1u16);
        // D s_119_4: cmp-eq s_119_1 s_119_3
        let s_119_4: bool = ((s_119_1) == (s_119_3));
        // D s_119_5: write-var gs#50134 <= s_119_4
        fn_state.gs_50134 = s_119_4;
        // N s_119_6: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var __MDCR_EL3_SBRBE:u8
        let s_120_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_120_1: const #0s : i
        let s_120_1: i128 = 0;
        // D s_120_2: cast zx s_120_0 -> bv
        let s_120_2: Bits = Bits::new(s_120_0 as u128, 2u16);
        // C s_120_3: const #1s : i64
        let s_120_3: i64 = 1;
        // C s_120_4: cast zx s_120_3 -> i
        let s_120_4: i128 = (i128::try_from(s_120_3).unwrap());
        // C s_120_5: const #0s : i
        let s_120_5: i128 = 0;
        // C s_120_6: add s_120_5 s_120_4
        let s_120_6: i128 = (s_120_5 + s_120_4);
        // D s_120_7: bit-extract s_120_2 s_120_1 s_120_6
        let s_120_7: Bits = (Bits::new(
            ((s_120_2) >> (s_120_1)).value(),
            u16::try_from(s_120_6).unwrap(),
        ));
        // D s_120_8: cast reint s_120_7 -> u8
        let s_120_8: bool = ((s_120_7.value()) != 0);
        // D s_120_9: cast zx s_120_8 -> bv
        let s_120_9: Bits = Bits::new(s_120_8 as u128, 1u16);
        // C s_120_10: const #0u : u8
        let s_120_10: bool = false;
        // C s_120_11: cast zx s_120_10 -> bv
        let s_120_11: Bits = Bits::new(s_120_10 as u128, 1u16);
        // D s_120_12: cmp-eq s_120_9 s_120_11
        let s_120_12: bool = ((s_120_9) == (s_120_11));
        // D s_120_13: not s_120_12
        let s_120_13: bool = !s_120_12;
        // N s_120_14: branch s_120_13 b123 b121
        if s_120_13 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #1u : u8
        let s_121_0: bool = true;
        // D s_121_1: write-var gs#50128 <= s_121_0
        fn_state.gs_50128 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#50128:u8
        let s_122_0: bool = fn_state.gs_50128;
        // D s_122_1: write-var gs#50133 <= s_122_0
        fn_state.gs_50133 = s_122_0;
        // N s_122_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #0u : u8
        let s_123_0: bool = false;
        // D s_123_1: write-var gs#50128 <= s_123_0
        fn_state.gs_50128 = s_123_0;
        // N s_123_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call Halted(s_124_0)
        let s_124_1: bool = Halted(state, tracer, s_124_0);
        // N s_124_2: branch s_124_1 b129 b125
        if s_124_1 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #0u : u8
        let s_125_0: bool = false;
        // D s_125_1: write-var gs#50137 <= s_125_0
        fn_state.gs_50137 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#50137:u8
        let s_126_0: bool = fn_state.gs_50137;
        // N s_126_1: branch s_126_0 b128 b127
        if s_126_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #24u : u8
        let s_127_0: u8 = 24;
        // C s_127_1: cast zx s_127_0 -> bv
        let s_127_1: Bits = Bits::new(s_127_0 as u128, 8u16);
        // C s_127_2: cast zx s_127_1 -> i
        let s_127_2: i128 = (s_127_1.value() as i128);
        // C s_127_3: cast reint s_127_2 -> i64
        let s_127_3: i64 = (s_127_2 as i64);
        // C s_127_4: cast zx s_127_3 -> i
        let s_127_4: i128 = (i128::try_from(s_127_3).unwrap());
        // C s_127_5: const #424u : u32
        let s_127_5: u32 = 424;
        // D s_127_6: read-reg s_127_5:u8
        let s_127_6: u8 = {
            let value = state.read_register::<u8>(s_127_5 as isize);
            tracer.read_register(s_127_5 as isize, value);
            value
        };
        // D s_127_7: call AArch64_SystemAccessTrap(s_127_6, s_127_4)
        let s_127_7: () = AArch64_SystemAccessTrap(state, tracer, s_127_6, s_127_4);
        // N s_127_8: return
        return;
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_128_0: panic
        panic!("{:?}", ());
        // N s_128_1: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var __EDSCR_SDD:u8
        let s_129_0: bool = fn_state.u__EDSCR_SDD;
        // D s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 1u16);
        // C s_129_2: const #1u : u8
        let s_129_2: bool = true;
        // C s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 1u16);
        // D s_129_4: cmp-eq s_129_1 s_129_3
        let s_129_4: bool = ((s_129_1) == (s_129_3));
        // D s_129_5: write-var gs#50137 <= s_129_4
        fn_state.gs_50137 = s_129_4;
        // N s_129_6: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var __SCR_EL3_NS:u8
        let s_130_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_130_1: cast zx s_130_0 -> bv
        let s_130_1: Bits = Bits::new(s_130_0 as u128, 1u16);
        // C s_130_2: const #0u : u8
        let s_130_2: bool = false;
        // C s_130_3: cast zx s_130_2 -> bv
        let s_130_3: Bits = Bits::new(s_130_2 as u128, 1u16);
        // D s_130_4: cmp-eq s_130_1 s_130_3
        let s_130_4: bool = ((s_130_1) == (s_130_3));
        // D s_130_5: write-var gs#50127 <= s_130_4
        fn_state.gs_50127 = s_130_4;
        // N s_130_6: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var __MDCR_EL3_SBRBE:u8
        let s_131_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_131_1: cast zx s_131_0 -> bv
        let s_131_1: Bits = Bits::new(s_131_0 as u128, 2u16);
        // C s_131_2: const #3u : u8
        let s_131_2: u8 = 3;
        // C s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 2u16);
        // D s_131_4: cmp-ne s_131_1 s_131_3
        let s_131_4: bool = ((s_131_1) != (s_131_3));
        // D s_131_5: write-var gs#50126 <= s_131_4
        fn_state.gs_50126 = s_131_4;
        // N s_131_6: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #24u : u8
        let s_132_0: u8 = 24;
        // C s_132_1: cast zx s_132_0 -> bv
        let s_132_1: Bits = Bits::new(s_132_0 as u128, 8u16);
        // C s_132_2: cast zx s_132_1 -> i
        let s_132_2: i128 = (s_132_1.value() as i128);
        // C s_132_3: cast reint s_132_2 -> i64
        let s_132_3: i64 = (s_132_2 as i64);
        // C s_132_4: cast zx s_132_3 -> i
        let s_132_4: i128 = (i128::try_from(s_132_3).unwrap());
        // C s_132_5: const #432u : u32
        let s_132_5: u32 = 432;
        // D s_132_6: read-reg s_132_5:u8
        let s_132_6: u8 = {
            let value = state.read_register::<u8>(s_132_5 as isize);
            tracer.read_register(s_132_5 as isize, value);
            value
        };
        // D s_132_7: call AArch64_SystemAccessTrap(s_132_6, s_132_4)
        let s_132_7: () = AArch64_SystemAccessTrap(state, tracer, s_132_6, s_132_4);
        // N s_132_8: return
        return;
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var __HDFGRTR_EL2_nBRBDATA:u8
        let s_133_0: bool = fn_state.u__HDFGRTR_EL2_nBRBDATA;
        // D s_133_1: cast zx s_133_0 -> bv
        let s_133_1: Bits = Bits::new(s_133_0 as u128, 1u16);
        // C s_133_2: const #0u : u8
        let s_133_2: bool = false;
        // C s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 1u16);
        // D s_133_4: cmp-eq s_133_1 s_133_3
        let s_133_4: bool = ((s_133_1) == (s_133_3));
        // D s_133_5: write-var gs#50125 <= s_133_4
        fn_state.gs_50125 = s_133_4;
        // N s_133_6: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #424u : u32
        let s_134_0: u32 = 424;
        // D s_134_1: read-reg s_134_0:u8
        let s_134_1: u8 = {
            let value = state.read_register::<u8>(s_134_0 as isize);
            tracer.read_register(s_134_0 as isize, value);
            value
        };
        // C s_134_2: const #2u : u8
        let s_134_2: u8 = 2;
        // D s_134_3: cmp-lt s_134_1 s_134_2
        let s_134_3: bool = ((s_134_1) < (s_134_2));
        // D s_134_4: not s_134_3
        let s_134_4: bool = !s_134_3;
        // N s_134_5: branch s_134_4 b137 b135
        if s_134_4 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var __SCR_EL3_FGTEn:u8
        let s_135_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_135_1: cast zx s_135_0 -> bv
        let s_135_1: Bits = Bits::new(s_135_0 as u128, 1u16);
        // C s_135_2: const #1u : u8
        let s_135_2: bool = true;
        // C s_135_3: cast zx s_135_2 -> bv
        let s_135_3: Bits = Bits::new(s_135_2 as u128, 1u16);
        // D s_135_4: cmp-eq s_135_1 s_135_3
        let s_135_4: bool = ((s_135_1) == (s_135_3));
        // D s_135_5: write-var gs#50123 <= s_135_4
        fn_state.gs_50123 = s_135_4;
        // N s_135_6: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#50123:u8
        let s_136_0: bool = fn_state.gs_50123;
        // D s_136_1: write-var gs#50124 <= s_136_0
        fn_state.gs_50124 = s_136_0;
        // N s_136_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #1u : u8
        let s_137_0: bool = true;
        // D s_137_1: write-var gs#50123 <= s_137_0
        fn_state.gs_50123 = s_137_0;
        // N s_137_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #146u : u32
        let s_138_0: u32 = 146;
        // S s_138_1: call IsFeatureImplemented(s_138_0)
        let s_138_1: bool = IsFeatureImplemented(state, tracer, s_138_0);
        // D s_138_2: write-var gs#50122 <= s_138_1
        fn_state.gs_50122 = s_138_1;
        // N s_138_3: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_139_0: panic
        panic!("{:?}", ());
        // N s_139_1: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var __SCR_EL3_NS:u8
        let s_140_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 1u16);
        // C s_140_2: const #1u : u8
        let s_140_2: bool = true;
        // C s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 1u16);
        // D s_140_4: cmp-eq s_140_1 s_140_3
        let s_140_4: bool = ((s_140_1) == (s_140_3));
        // D s_140_5: write-var gs#50121 <= s_140_4
        fn_state.gs_50121 = s_140_4;
        // N s_140_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var __MDCR_EL3_SBRBE:u8
        let s_141_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_141_1: const #0s : i
        let s_141_1: i128 = 0;
        // D s_141_2: cast zx s_141_0 -> bv
        let s_141_2: Bits = Bits::new(s_141_0 as u128, 2u16);
        // C s_141_3: const #1s : i64
        let s_141_3: i64 = 1;
        // C s_141_4: cast zx s_141_3 -> i
        let s_141_4: i128 = (i128::try_from(s_141_3).unwrap());
        // C s_141_5: const #0s : i
        let s_141_5: i128 = 0;
        // C s_141_6: add s_141_5 s_141_4
        let s_141_6: i128 = (s_141_5 + s_141_4);
        // D s_141_7: bit-extract s_141_2 s_141_1 s_141_6
        let s_141_7: Bits = (Bits::new(
            ((s_141_2) >> (s_141_1)).value(),
            u16::try_from(s_141_6).unwrap(),
        ));
        // D s_141_8: cast reint s_141_7 -> u8
        let s_141_8: bool = ((s_141_7.value()) != 0);
        // D s_141_9: cast zx s_141_8 -> bv
        let s_141_9: Bits = Bits::new(s_141_8 as u128, 1u16);
        // C s_141_10: const #0u : u8
        let s_141_10: bool = false;
        // C s_141_11: cast zx s_141_10 -> bv
        let s_141_11: Bits = Bits::new(s_141_10 as u128, 1u16);
        // D s_141_12: cmp-eq s_141_9 s_141_11
        let s_141_12: bool = ((s_141_9) == (s_141_11));
        // D s_141_13: not s_141_12
        let s_141_13: bool = !s_141_12;
        // N s_141_14: branch s_141_13 b144 b142
        if s_141_13 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #1u : u8
        let s_142_0: bool = true;
        // D s_142_1: write-var gs#50115 <= s_142_0
        fn_state.gs_50115 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#50115:u8
        let s_143_0: bool = fn_state.gs_50115;
        // D s_143_1: write-var gs#50120 <= s_143_0
        fn_state.gs_50120 = s_143_0;
        // N s_143_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #0u : u8
        let s_144_0: bool = false;
        // D s_144_1: write-var gs#50115 <= s_144_0
        fn_state.gs_50115 = s_144_0;
        // N s_144_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_145_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_145_1: call __IMPDEF_boolean(s_145_0)
        let s_145_1: bool = u__IMPDEF_boolean(state, tracer, s_145_0);
        // D s_145_2: write-var gs#50114 <= s_145_1
        fn_state.gs_50114 = s_145_1;
        // N s_145_3: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var __EDSCR_SDD:u8
        let s_146_0: bool = fn_state.u__EDSCR_SDD;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 1u16);
        // C s_146_2: const #1u : u8
        let s_146_2: bool = true;
        // C s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_4: cmp-eq s_146_1 s_146_3
        let s_146_4: bool = ((s_146_1) == (s_146_3));
        // D s_146_5: write-var gs#50113 <= s_146_4
        fn_state.gs_50113 = s_146_4;
        // N s_146_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #424u : u32
        let s_147_0: u32 = 424;
        // D s_147_1: read-reg s_147_0:u8
        let s_147_1: u8 = {
            let value = state.read_register::<u8>(s_147_0 as isize);
            tracer.read_register(s_147_0 as isize, value);
            value
        };
        // C s_147_2: const #2u : u8
        let s_147_2: u8 = 2;
        // D s_147_3: cmp-lt s_147_1 s_147_2
        let s_147_3: bool = ((s_147_1) < (s_147_2));
        // D s_147_4: write-var gs#50112 <= s_147_3
        fn_state.gs_50112 = s_147_3;
        // N s_147_5: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_148_0: panic
        panic!("{:?}", ());
        // N s_148_1: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var __SCR_EL3_NS:u8
        let s_149_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_149_1: cast zx s_149_0 -> bv
        let s_149_1: Bits = Bits::new(s_149_0 as u128, 1u16);
        // C s_149_2: const #0u : u8
        let s_149_2: bool = false;
        // C s_149_3: cast zx s_149_2 -> bv
        let s_149_3: Bits = Bits::new(s_149_2 as u128, 1u16);
        // D s_149_4: cmp-eq s_149_1 s_149_3
        let s_149_4: bool = ((s_149_1) == (s_149_3));
        // D s_149_5: write-var gs#50111 <= s_149_4
        fn_state.gs_50111 = s_149_4;
        // N s_149_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var __MDCR_EL3_SBRBE:u8
        let s_150_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 2u16);
        // C s_150_2: const #3u : u8
        let s_150_2: u8 = 3;
        // C s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 2u16);
        // D s_150_4: cmp-ne s_150_1 s_150_3
        let s_150_4: bool = ((s_150_1) != (s_150_3));
        // D s_150_5: write-var gs#50110 <= s_150_4
        fn_state.gs_50110 = s_150_4;
        // N s_150_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_151_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_151_1: call __IMPDEF_boolean(s_151_0)
        let s_151_1: bool = u__IMPDEF_boolean(state, tracer, s_151_0);
        // D s_151_2: write-var gs#50109 <= s_151_1
        fn_state.gs_50109 = s_151_1;
        // N s_151_3: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var __EDSCR_SDD:u8
        let s_152_0: bool = fn_state.u__EDSCR_SDD;
        // D s_152_1: cast zx s_152_0 -> bv
        let s_152_1: Bits = Bits::new(s_152_0 as u128, 1u16);
        // C s_152_2: const #1u : u8
        let s_152_2: bool = true;
        // C s_152_3: cast zx s_152_2 -> bv
        let s_152_3: Bits = Bits::new(s_152_2 as u128, 1u16);
        // D s_152_4: cmp-eq s_152_1 s_152_3
        let s_152_4: bool = ((s_152_1) == (s_152_3));
        // D s_152_5: write-var gs#50108 <= s_152_4
        fn_state.gs_50108 = s_152_4;
        // N s_152_6: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #424u : u32
        let s_153_0: u32 = 424;
        // D s_153_1: read-reg s_153_0:u8
        let s_153_1: u8 = {
            let value = state.read_register::<u8>(s_153_0 as isize);
            tracer.read_register(s_153_0 as isize, value);
            value
        };
        // C s_153_2: const #2u : u8
        let s_153_2: u8 = 2;
        // D s_153_3: cmp-lt s_153_1 s_153_2
        let s_153_3: bool = ((s_153_1) < (s_153_2));
        // D s_153_4: write-var gs#50107 <= s_153_3
        fn_state.gs_50107 = s_153_3;
        // N s_153_5: jump b75
        return block_75(state, tracer, fn_state);
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
}
