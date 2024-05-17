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
use EL2Enabled::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HFGWTR_EL2_Type_ICC_IGRPENn_EL1::*;
use Halted::*;
use u_get_SCR_EL3_Type_IRQ::*;
use ICC_IGRPEN1_EL1_write::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_SCR_EL3_Type_NS::*;
use Mk_ICV_IGRPEN1_EL1_Type::*;
use ICC_SRE_EL1_read::*;
use IsFeatureImplemented::*;
use u_get_ICH_HCR_EL2_Type_TALL1::*;
use u_get_ICC_SRE_EL1_Type_SRE::*;
use u_get_ICC_SRE_EL2_Type_SRE::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_ICC_SRE_EL3_Type_SRE::*;
use u_get_HCR_EL2_Type_IMO::*;
use EDSCR_read::*;
use Mk_ICC_IGRPEN1_EL1_Type::*;
use common::*;
pub fn ICC_IGRPEN1_EL1_SysRegWrite_a249f92f5cac95a3<T: Tracer>(
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
        u__SCR_EL3_IRQ: bool,
        gs_86067: bool,
        u__SCR_EL3_NS: bool,
        gs_86060: bool,
        gs_86077: bool,
        u__EDSCR_SDD: bool,
        u__HCR_EL2_IMO: bool,
        u__ICC_SRE_EL1_SRE: bool,
        gs_86063: bool,
        gs_86071: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_86078: bool,
        gs_86075: bool,
        gs_86069: bool,
        u__ICC_SRE_EL3_SRE: bool,
        u__ICH_HCR_EL2_TALL1: bool,
        gs_86073: bool,
        gs_86082: bool,
        gs_86059: bool,
        gs_86062: bool,
        u__ICC_SRE_EL2_SRE: bool,
        gs_86068: bool,
        gs_86061: bool,
        u__HFGWTR_EL2_ICC_IGRPENn_EL1: bool,
        gs_86076: bool,
        gs_86074: bool,
        u__PSTATE_EL: u8,
        gs_86072: bool,
        gs_86070: bool,
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
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_IRQ(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_IRQ <= s_0_9
        fn_state.u__SCR_EL3_IRQ = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call ICC_SRE_EL1_read(s_0_11)
        let s_0_12: ProductType5c790c8ef59cc8b2 = ICC_SRE_EL1_read(
            state,
            tracer,
            s_0_11,
        );
        // S s_0_13: call _get_ICC_SRE_EL1_Type_SRE(s_0_12)
        let s_0_13: bool = u_get_ICC_SRE_EL1_Type_SRE(state, tracer, s_0_12);
        // D s_0_14: write-var __ICC_SRE_EL1_SRE <= s_0_13
        fn_state.u__ICC_SRE_EL1_SRE = s_0_13;
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
        // C s_0_19: const #100992u : u32
        let s_0_19: u32 = 100992;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HFGWTR_EL2_Type_ICC_IGRPENn_EL1(s_0_20)
        let s_0_21: bool = u_get_HFGWTR_EL2_Type_ICC_IGRPENn_EL1(state, tracer, s_0_20);
        // D s_0_22: write-var __HFGWTR_EL2_ICC_IGRPENn_EL1 <= s_0_21
        fn_state.u__HFGWTR_EL2_ICC_IGRPENn_EL1 = s_0_21;
        // C s_0_23: const #20992u : u32
        let s_0_23: u32 = 20992;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_ICH_HCR_EL2_Type_TALL1(s_0_24)
        let s_0_25: bool = u_get_ICH_HCR_EL2_Type_TALL1(state, tracer, s_0_24);
        // D s_0_26: write-var __ICH_HCR_EL2_TALL1 <= s_0_25
        fn_state.u__ICH_HCR_EL2_TALL1 = s_0_25;
        // C s_0_27: const #102552u : u32
        let s_0_27: u32 = 102552;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_HCR_EL2_Type_IMO(s_0_28)
        let s_0_29: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_0_28);
        // D s_0_30: write-var __HCR_EL2_IMO <= s_0_29
        fn_state.u__HCR_EL2_IMO = s_0_29;
        // C s_0_31: const #90704u : u32
        let s_0_31: u32 = 90704;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_SCR_EL3_Type_NS(s_0_32)
        let s_0_33: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_0_32);
        // D s_0_34: write-var __SCR_EL3_NS <= s_0_33
        fn_state.u__SCR_EL3_NS = s_0_33;
        // C s_0_35: const #16368u : u32
        let s_0_35: u32 = 16368;
        // D s_0_36: read-reg s_0_35:struct
        let s_0_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_35 as isize);
            tracer.read_register(s_0_35 as isize, value);
            value
        };
        // D s_0_37: call _get_ICC_SRE_EL2_Type_SRE(s_0_36)
        let s_0_37: bool = u_get_ICC_SRE_EL2_Type_SRE(state, tracer, s_0_36);
        // D s_0_38: write-var __ICC_SRE_EL2_SRE <= s_0_37
        fn_state.u__ICC_SRE_EL2_SRE = s_0_37;
        // C s_0_39: const #10200u : u32
        let s_0_39: u32 = 10200;
        // D s_0_40: read-reg s_0_39:struct
        let s_0_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_39 as isize);
            tracer.read_register(s_0_39 as isize, value);
            value
        };
        // D s_0_41: call _get_ICC_SRE_EL3_Type_SRE(s_0_40)
        let s_0_41: bool = u_get_ICC_SRE_EL3_Type_SRE(state, tracer, s_0_40);
        // D s_0_42: write-var __ICC_SRE_EL3_SRE <= s_0_41
        fn_state.u__ICC_SRE_EL3_SRE = s_0_41;
        // D s_0_43: read-var __PSTATE_EL:u8
        let s_0_43: u8 = fn_state.u__PSTATE_EL;
        // D s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 2u16);
        // C s_0_45: const #448u : u32
        let s_0_45: u32 = 448;
        // D s_0_46: read-reg s_0_45:u8
        let s_0_46: u8 = {
            let value = state.read_register::<u8>(s_0_45 as isize);
            tracer.read_register(s_0_45 as isize, value);
            value
        };
        // D s_0_47: cast zx s_0_46 -> bv
        let s_0_47: Bits = Bits::new(s_0_46 as u128, 2u16);
        // D s_0_48: cmp-eq s_0_44 s_0_47
        let s_0_48: bool = ((s_0_44) == (s_0_47));
        // N s_0_49: branch s_0_48 b96 b1
        if s_0_48 {
            return block_96(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b41 b2
        if s_1_5 {
            return block_41(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b10 b3
        if s_2_5 {
            return block_10(state, tracer, fn_state);
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
        // D s_5_0: read-var __ICC_SRE_EL3_SRE:u8
        let s_5_0: bool = fn_state.u__ICC_SRE_EL3_SRE;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b9 b6
        if s_5_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __SCR_EL3_NS:u8
        let s_6_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #0u : u8
        let s_6_2: bool = false;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: read-var t:i
        let s_7_1: i128 = fn_state.t;
        // D s_7_2: call X_read(s_7_1, s_7_0)
        let s_7_2: Bits = X_read(state, tracer, s_7_1, s_7_0);
        // D s_7_3: cast reint s_7_2 -> u64
        let s_7_3: u64 = (s_7_2.value() as u64);
        // D s_7_4: call Mk_ICC_IGRPEN1_EL1_Type(s_7_3)
        let s_7_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_IGRPEN1_EL1_Type(
            state,
            tracer,
            s_7_3,
        );
        // C s_7_5: const #17040u : u32
        let s_7_5: u32 = 17040;
        // N s_7_6: write-reg s_7_5 <= s_7_4
        let s_7_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_7_5 as isize, s_7_4);
            tracer.write_register(s_7_5 as isize, s_7_4);
        };
        // N s_7_7: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // D s_8_1: read-var t:i
        let s_8_1: i128 = fn_state.t;
        // D s_8_2: call X_read(s_8_1, s_8_0)
        let s_8_2: Bits = X_read(state, tracer, s_8_1, s_8_0);
        // D s_8_3: cast reint s_8_2 -> u64
        let s_8_3: u64 = (s_8_2.value() as u64);
        // D s_8_4: call Mk_ICC_IGRPEN1_EL1_Type(s_8_3)
        let s_8_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_IGRPEN1_EL1_Type(
            state,
            tracer,
            s_8_3,
        );
        // C s_8_5: const #101216u : u32
        let s_8_5: u32 = 101216;
        // N s_8_6: write-reg s_8_5 <= s_8_4
        let s_8_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_5 as isize, s_8_4);
            tracer.write_register(s_8_5 as isize, s_8_4);
        };
        // N s_8_7: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #24u : u8
        let s_9_0: u8 = 24;
        // C s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 8u16);
        // C s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (s_9_1.value() as i128);
        // C s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // C s_9_5: const #424u : u32
        let s_9_5: u32 = 424;
        // D s_9_6: read-reg s_9_5:u8
        let s_9_6: u8 = {
            let value = state.read_register::<u8>(s_9_5 as isize);
            tracer.read_register(s_9_5 as isize, value);
            value
        };
        // D s_9_7: call AArch64_SystemAccessTrap(s_9_6, s_9_4)
        let s_9_7: () = AArch64_SystemAccessTrap(state, tracer, s_9_6, s_9_4);
        // N s_9_8: return
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
        // N s_10_2: branch s_10_1 b40 b11
        if s_10_1 {
            return block_40(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#86059 <= s_11_0
        fn_state.gs_86059 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#86059:u8
        let s_12_0: bool = fn_state.gs_86059;
        // N s_12_1: branch s_12_0 b39 b13
        if s_12_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#86060 <= s_13_0
        fn_state.gs_86060 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#86060:u8
        let s_14_0: bool = fn_state.gs_86060;
        // N s_14_1: branch s_14_0 b38 b15
        if s_14_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#86061 <= s_15_0
        fn_state.gs_86061 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#86061:u8
        let s_16_0: bool = fn_state.gs_86061;
        // N s_16_1: branch s_16_0 b37 b17
        if s_16_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#86062 <= s_17_0
        fn_state.gs_86062 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#86062:u8
        let s_18_0: bool = fn_state.gs_86062;
        // N s_18_1: branch s_18_0 b36 b19
        if s_18_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var __ICC_SRE_EL2_SRE:u8
        let s_19_0: bool = fn_state.u__ICC_SRE_EL2_SRE;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #0u : u8
        let s_19_2: bool = false;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // N s_19_5: branch s_19_4 b35 b20
        if s_19_4 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #424u : u32
        let s_20_0: u32 = 424;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // D s_20_3: cmp-lt s_20_1 s_20_2
        let s_20_3: bool = ((s_20_1) < (s_20_2));
        // N s_20_4: branch s_20_3 b34 b21
        if s_20_3 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#86063 <= s_21_0
        fn_state.gs_86063 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#86063:u8
        let s_22_0: bool = fn_state.gs_86063;
        // N s_22_1: branch s_22_0 b28 b23
        if s_22_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #424u : u32
        let s_23_0: u32 = 424;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // C s_23_2: const #2u : u8
        let s_23_2: u8 = 2;
        // D s_23_3: cmp-lt s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) < (s_23_2));
        // N s_23_4: branch s_23_3 b25 b24
        if s_23_3 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // D s_24_1: read-var t:i
        let s_24_1: i128 = fn_state.t;
        // D s_24_2: call X_read(s_24_1, s_24_0)
        let s_24_2: Bits = X_read(state, tracer, s_24_1, s_24_0);
        // D s_24_3: cast reint s_24_2 -> u64
        let s_24_3: u64 = (s_24_2.value() as u64);
        // D s_24_4: call Mk_ICC_IGRPEN1_EL1_Type(s_24_3)
        let s_24_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_IGRPEN1_EL1_Type(
            state,
            tracer,
            s_24_3,
        );
        // D s_24_5: call ICC_IGRPEN1_EL1_write(s_24_4)
        let s_24_5: () = ICC_IGRPEN1_EL1_write(state, tracer, s_24_4);
        // N s_24_6: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __SCR_EL3_NS:u8
        let s_25_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // N s_25_5: branch s_25_4 b27 b26
        if s_25_4 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #64s : i64
        let s_26_0: i64 = 64;
        // D s_26_1: read-var t:i
        let s_26_1: i128 = fn_state.t;
        // D s_26_2: call X_read(s_26_1, s_26_0)
        let s_26_2: Bits = X_read(state, tracer, s_26_1, s_26_0);
        // D s_26_3: cast reint s_26_2 -> u64
        let s_26_3: u64 = (s_26_2.value() as u64);
        // D s_26_4: call Mk_ICC_IGRPEN1_EL1_Type(s_26_3)
        let s_26_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_IGRPEN1_EL1_Type(
            state,
            tracer,
            s_26_3,
        );
        // C s_26_5: const #17040u : u32
        let s_26_5: u32 = 17040;
        // N s_26_6: write-reg s_26_5 <= s_26_4
        let s_26_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_26_5 as isize, s_26_4);
            tracer.write_register(s_26_5 as isize, s_26_4);
        };
        // N s_26_7: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #64s : i64
        let s_27_0: i64 = 64;
        // D s_27_1: read-var t:i
        let s_27_1: i128 = fn_state.t;
        // D s_27_2: call X_read(s_27_1, s_27_0)
        let s_27_2: Bits = X_read(state, tracer, s_27_1, s_27_0);
        // D s_27_3: cast reint s_27_2 -> u64
        let s_27_3: u64 = (s_27_2.value() as u64);
        // D s_27_4: call Mk_ICC_IGRPEN1_EL1_Type(s_27_3)
        let s_27_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_IGRPEN1_EL1_Type(
            state,
            tracer,
            s_27_3,
        );
        // C s_27_5: const #101216u : u32
        let s_27_5: u32 = 101216;
        // N s_27_6: write-reg s_27_5 <= s_27_4
        let s_27_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_27_5 as isize, s_27_4);
            tracer.write_register(s_27_5 as isize, s_27_4);
        };
        // N s_27_7: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call Halted(s_28_0)
        let s_28_1: bool = Halted(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b33 b29
        if s_28_1 {
            return block_33(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#86067 <= s_29_0
        fn_state.gs_86067 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#86067:u8
        let s_30_0: bool = fn_state.gs_86067;
        // N s_30_1: branch s_30_0 b32 b31
        if s_30_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #24u : u8
        let s_31_0: u8 = 24;
        // C s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 8u16);
        // C s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (s_31_1.value() as i128);
        // C s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: cast zx s_31_3 -> i
        let s_31_4: i128 = (i128::try_from(s_31_3).unwrap());
        // C s_31_5: const #424u : u32
        let s_31_5: u32 = 424;
        // D s_31_6: read-reg s_31_5:u8
        let s_31_6: u8 = {
            let value = state.read_register::<u8>(s_31_5 as isize);
            tracer.read_register(s_31_5 as isize, value);
            value
        };
        // D s_31_7: call AArch64_SystemAccessTrap(s_31_6, s_31_4)
        let s_31_7: () = AArch64_SystemAccessTrap(state, tracer, s_31_6, s_31_4);
        // N s_31_8: return
        return;
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
        // D s_33_0: read-var __EDSCR_SDD:u8
        let s_33_0: bool = fn_state.u__EDSCR_SDD;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#86067 <= s_33_4
        fn_state.gs_86067 = s_33_4;
        // N s_33_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var __SCR_EL3_IRQ:u8
        let s_34_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#86063 <= s_34_4
        fn_state.gs_86063 = s_34_4;
        // N s_34_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #24u : u8
        let s_35_0: u8 = 24;
        // C s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 8u16);
        // C s_35_2: cast zx s_35_1 -> i
        let s_35_2: i128 = (s_35_1.value() as i128);
        // C s_35_3: cast reint s_35_2 -> i64
        let s_35_3: i64 = (s_35_2 as i64);
        // C s_35_4: cast zx s_35_3 -> i
        let s_35_4: i128 = (i128::try_from(s_35_3).unwrap());
        // C s_35_5: const #432u : u32
        let s_35_5: u32 = 432;
        // D s_35_6: read-reg s_35_5:u8
        let s_35_6: u8 = {
            let value = state.read_register::<u8>(s_35_5 as isize);
            tracer.read_register(s_35_5 as isize, value);
            value
        };
        // D s_35_7: call AArch64_SystemAccessTrap(s_35_6, s_35_4)
        let s_35_7: () = AArch64_SystemAccessTrap(state, tracer, s_35_6, s_35_4);
        // N s_35_8: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: panic
        panic!("{:?}", ());
        // N s_36_1: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __SCR_EL3_IRQ:u8
        let s_37_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #1u : u8
        let s_37_2: bool = true;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#86062 <= s_37_4
        fn_state.gs_86062 = s_37_4;
        // N s_37_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_38_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_38_1: call __IMPDEF_boolean(s_38_0)
        let s_38_1: bool = u__IMPDEF_boolean(state, tracer, s_38_0);
        // D s_38_2: write-var gs#86061 <= s_38_1
        fn_state.gs_86061 = s_38_1;
        // N s_38_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __EDSCR_SDD:u8
        let s_39_0: bool = fn_state.u__EDSCR_SDD;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#86060 <= s_39_4
        fn_state.gs_86060 = s_39_4;
        // N s_39_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #424u : u32
        let s_40_0: u32 = 424;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // C s_40_2: const #2u : u8
        let s_40_2: u8 = 2;
        // D s_40_3: cmp-lt s_40_1 s_40_2
        let s_40_3: bool = ((s_40_1) < (s_40_2));
        // D s_40_4: write-var gs#86059 <= s_40_3
        fn_state.gs_86059 = s_40_3;
        // N s_40_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call Halted(s_41_0)
        let s_41_1: bool = Halted(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b95 b42
        if s_41_1 {
            return block_95(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#86068 <= s_42_0
        fn_state.gs_86068 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#86068:u8
        let s_43_0: bool = fn_state.gs_86068;
        // N s_43_1: branch s_43_0 b94 b44
        if s_43_0 {
            return block_94(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#86069 <= s_44_0
        fn_state.gs_86069 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#86069:u8
        let s_45_0: bool = fn_state.gs_86069;
        // N s_45_1: branch s_45_0 b93 b46
        if s_45_0 {
            return block_93(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#86070 <= s_46_0
        fn_state.gs_86070 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#86070:u8
        let s_47_0: bool = fn_state.gs_86070;
        // N s_47_1: branch s_47_0 b92 b48
        if s_47_0 {
            return block_92(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#86071 <= s_48_0
        fn_state.gs_86071 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#86071:u8
        let s_49_0: bool = fn_state.gs_86071;
        // N s_49_1: branch s_49_0 b91 b50
        if s_49_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var __ICC_SRE_EL1_SRE:u8
        let s_50_0: bool = fn_state.u__ICC_SRE_EL1_SRE;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 1u16);
        // C s_50_2: const #0u : u8
        let s_50_2: bool = false;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_4: cmp-eq s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) == (s_50_3));
        // N s_50_5: branch s_50_4 b90 b51
        if s_50_4 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call EL2Enabled(s_51_0)
        let s_51_1: bool = EL2Enabled(state, tracer, s_51_0);
        // N s_51_2: branch s_51_1 b89 b52
        if s_51_1 {
            return block_89(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#86072 <= s_52_0
        fn_state.gs_86072 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#86072:u8
        let s_53_0: bool = fn_state.gs_86072;
        // N s_53_1: branch s_53_0 b85 b54
        if s_53_0 {
            return block_85(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#86074 <= s_54_0
        fn_state.gs_86074 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#86074:u8
        let s_55_0: bool = fn_state.gs_86074;
        // N s_55_1: branch s_55_0 b84 b56
        if s_55_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#86075 <= s_56_0
        fn_state.gs_86075 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#86075:u8
        let s_57_0: bool = fn_state.gs_86075;
        // N s_57_1: branch s_57_0 b83 b58
        if s_57_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call EL2Enabled(s_58_0)
        let s_58_1: bool = EL2Enabled(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b82 b59
        if s_58_1 {
            return block_82(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#86076 <= s_59_0
        fn_state.gs_86076 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#86076:u8
        let s_60_0: bool = fn_state.gs_86076;
        // N s_60_1: branch s_60_0 b81 b61
        if s_60_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #() : ()
        let s_61_0: () = ();
        // S s_61_1: call EL2Enabled(s_61_0)
        let s_61_1: bool = EL2Enabled(state, tracer, s_61_0);
        // N s_61_2: branch s_61_1 b80 b62
        if s_61_1 {
            return block_80(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#86077 <= s_62_0
        fn_state.gs_86077 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#86077:u8
        let s_63_0: bool = fn_state.gs_86077;
        // N s_63_1: branch s_63_0 b79 b64
        if s_63_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
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
        // N s_64_4: branch s_64_3 b78 b65
        if s_64_3 {
            return block_78(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#86078 <= s_65_0
        fn_state.gs_86078 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#86078:u8
        let s_66_0: bool = fn_state.gs_86078;
        // N s_66_1: branch s_66_0 b72 b67
        if s_66_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
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
        // C s_67_2: const #2u : u8
        let s_67_2: u8 = 2;
        // D s_67_3: cmp-lt s_67_1 s_67_2
        let s_67_3: bool = ((s_67_1) < (s_67_2));
        // N s_67_4: branch s_67_3 b69 b68
        if s_67_3 {
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
        // C s_68_0: const #64s : i64
        let s_68_0: i64 = 64;
        // D s_68_1: read-var t:i
        let s_68_1: i128 = fn_state.t;
        // D s_68_2: call X_read(s_68_1, s_68_0)
        let s_68_2: Bits = X_read(state, tracer, s_68_1, s_68_0);
        // D s_68_3: cast reint s_68_2 -> u64
        let s_68_3: u64 = (s_68_2.value() as u64);
        // D s_68_4: call Mk_ICC_IGRPEN1_EL1_Type(s_68_3)
        let s_68_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_IGRPEN1_EL1_Type(
            state,
            tracer,
            s_68_3,
        );
        // D s_68_5: call ICC_IGRPEN1_EL1_write(s_68_4)
        let s_68_5: () = ICC_IGRPEN1_EL1_write(state, tracer, s_68_4);
        // N s_68_6: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __SCR_EL3_NS:u8
        let s_69_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #0u : u8
        let s_69_2: bool = false;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // N s_69_5: branch s_69_4 b71 b70
        if s_69_4 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #64s : i64
        let s_70_0: i64 = 64;
        // D s_70_1: read-var t:i
        let s_70_1: i128 = fn_state.t;
        // D s_70_2: call X_read(s_70_1, s_70_0)
        let s_70_2: Bits = X_read(state, tracer, s_70_1, s_70_0);
        // D s_70_3: cast reint s_70_2 -> u64
        let s_70_3: u64 = (s_70_2.value() as u64);
        // D s_70_4: call Mk_ICC_IGRPEN1_EL1_Type(s_70_3)
        let s_70_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_IGRPEN1_EL1_Type(
            state,
            tracer,
            s_70_3,
        );
        // C s_70_5: const #17040u : u32
        let s_70_5: u32 = 17040;
        // N s_70_6: write-reg s_70_5 <= s_70_4
        let s_70_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_70_5 as isize, s_70_4);
            tracer.write_register(s_70_5 as isize, s_70_4);
        };
        // N s_70_7: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #64s : i64
        let s_71_0: i64 = 64;
        // D s_71_1: read-var t:i
        let s_71_1: i128 = fn_state.t;
        // D s_71_2: call X_read(s_71_1, s_71_0)
        let s_71_2: Bits = X_read(state, tracer, s_71_1, s_71_0);
        // D s_71_3: cast reint s_71_2 -> u64
        let s_71_3: u64 = (s_71_2.value() as u64);
        // D s_71_4: call Mk_ICC_IGRPEN1_EL1_Type(s_71_3)
        let s_71_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_IGRPEN1_EL1_Type(
            state,
            tracer,
            s_71_3,
        );
        // C s_71_5: const #101216u : u32
        let s_71_5: u32 = 101216;
        // N s_71_6: write-reg s_71_5 <= s_71_4
        let s_71_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_71_5 as isize, s_71_4);
            tracer.write_register(s_71_5 as isize, s_71_4);
        };
        // N s_71_7: return
        return;
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
        // N s_72_2: branch s_72_1 b77 b73
        if s_72_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#86082 <= s_73_0
        fn_state.gs_86082 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#86082:u8
        let s_74_0: bool = fn_state.gs_86082;
        // N s_74_1: branch s_74_0 b76 b75
        if s_74_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #24u : u8
        let s_75_0: u8 = 24;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 8u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // C s_75_5: const #424u : u32
        let s_75_5: u32 = 424;
        // D s_75_6: read-reg s_75_5:u8
        let s_75_6: u8 = {
            let value = state.read_register::<u8>(s_75_5 as isize);
            tracer.read_register(s_75_5 as isize, value);
            value
        };
        // D s_75_7: call AArch64_SystemAccessTrap(s_75_6, s_75_4)
        let s_75_7: () = AArch64_SystemAccessTrap(state, tracer, s_75_6, s_75_4);
        // N s_75_8: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: panic
        panic!("{:?}", ());
        // N s_76_1: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var __EDSCR_SDD:u8
        let s_77_0: bool = fn_state.u__EDSCR_SDD;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#86082 <= s_77_4
        fn_state.gs_86082 = s_77_4;
        // N s_77_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __SCR_EL3_IRQ:u8
        let s_78_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #1u : u8
        let s_78_2: bool = true;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#86078 <= s_78_4
        fn_state.gs_86078 = s_78_4;
        // N s_78_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #64s : i64
        let s_79_0: i64 = 64;
        // D s_79_1: read-var t:i
        let s_79_1: i128 = fn_state.t;
        // D s_79_2: call X_read(s_79_1, s_79_0)
        let s_79_2: Bits = X_read(state, tracer, s_79_1, s_79_0);
        // D s_79_3: cast reint s_79_2 -> u64
        let s_79_3: u64 = (s_79_2.value() as u64);
        // D s_79_4: call Mk_ICV_IGRPEN1_EL1_Type(s_79_3)
        let s_79_4: ProductType5c790c8ef59cc8b2 = Mk_ICV_IGRPEN1_EL1_Type(
            state,
            tracer,
            s_79_3,
        );
        // C s_79_5: const #10600u : u32
        let s_79_5: u32 = 10600;
        // N s_79_6: write-reg s_79_5 <= s_79_4
        let s_79_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_79_5 as isize, s_79_4);
            tracer.write_register(s_79_5 as isize, s_79_4);
        };
        // N s_79_7: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __HCR_EL2_IMO:u8
        let s_80_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#86077 <= s_80_4
        fn_state.gs_86077 = s_80_4;
        // N s_80_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #24u : u8
        let s_81_0: u8 = 24;
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
        // D s_81_7: call AArch64_SystemAccessTrap(s_81_6, s_81_4)
        let s_81_7: () = AArch64_SystemAccessTrap(state, tracer, s_81_6, s_81_4);
        // N s_81_8: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var __ICH_HCR_EL2_TALL1:u8
        let s_82_0: bool = fn_state.u__ICH_HCR_EL2_TALL1;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 1u16);
        // C s_82_2: const #1u : u8
        let s_82_2: bool = true;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: write-var gs#86076 <= s_82_4
        fn_state.gs_86076 = s_82_4;
        // N s_82_6: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #24u : u8
        let s_83_0: u8 = 24;
        // C s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 8u16);
        // C s_83_2: cast zx s_83_1 -> i
        let s_83_2: i128 = (s_83_1.value() as i128);
        // C s_83_3: cast reint s_83_2 -> i64
        let s_83_3: i64 = (s_83_2 as i64);
        // C s_83_4: cast zx s_83_3 -> i
        let s_83_4: i128 = (i128::try_from(s_83_3).unwrap());
        // C s_83_5: const #432u : u32
        let s_83_5: u32 = 432;
        // D s_83_6: read-reg s_83_5:u8
        let s_83_6: u8 = {
            let value = state.read_register::<u8>(s_83_5 as isize);
            tracer.read_register(s_83_5 as isize, value);
            value
        };
        // D s_83_7: call AArch64_SystemAccessTrap(s_83_6, s_83_4)
        let s_83_7: () = AArch64_SystemAccessTrap(state, tracer, s_83_6, s_83_4);
        // N s_83_8: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __HFGWTR_EL2_ICC_IGRPENn_EL1:u8
        let s_84_0: bool = fn_state.u__HFGWTR_EL2_ICC_IGRPENn_EL1;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#86075 <= s_84_4
        fn_state.gs_86075 = s_84_4;
        // N s_84_6: jump b57
        return block_57(state, tracer, fn_state);
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
        // D s_85_4: not s_85_3
        let s_85_4: bool = !s_85_3;
        // N s_85_5: branch s_85_4 b88 b86
        if s_85_4 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var __SCR_EL3_FGTEn:u8
        let s_86_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #1u : u8
        let s_86_2: bool = true;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#86073 <= s_86_4
        fn_state.gs_86073 = s_86_4;
        // N s_86_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#86073:u8
        let s_87_0: bool = fn_state.gs_86073;
        // D s_87_1: write-var gs#86074 <= s_87_0
        fn_state.gs_86074 = s_87_0;
        // N s_87_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #1u : u8
        let s_88_0: bool = true;
        // D s_88_1: write-var gs#86073 <= s_88_0
        fn_state.gs_86073 = s_88_0;
        // N s_88_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #146u : u32
        let s_89_0: u32 = 146;
        // S s_89_1: call IsFeatureImplemented(s_89_0)
        let s_89_1: bool = IsFeatureImplemented(state, tracer, s_89_0);
        // D s_89_2: write-var gs#86072 <= s_89_1
        fn_state.gs_86072 = s_89_1;
        // N s_89_3: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #24u : u8
        let s_90_0: u8 = 24;
        // C s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 8u16);
        // C s_90_2: cast zx s_90_1 -> i
        let s_90_2: i128 = (s_90_1.value() as i128);
        // C s_90_3: cast reint s_90_2 -> i64
        let s_90_3: i64 = (s_90_2 as i64);
        // C s_90_4: cast zx s_90_3 -> i
        let s_90_4: i128 = (i128::try_from(s_90_3).unwrap());
        // C s_90_5: const #440u : u32
        let s_90_5: u32 = 440;
        // D s_90_6: read-reg s_90_5:u8
        let s_90_6: u8 = {
            let value = state.read_register::<u8>(s_90_5 as isize);
            tracer.read_register(s_90_5 as isize, value);
            value
        };
        // D s_90_7: call AArch64_SystemAccessTrap(s_90_6, s_90_4)
        let s_90_7: () = AArch64_SystemAccessTrap(state, tracer, s_90_6, s_90_4);
        // N s_90_8: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_91_0: panic
        panic!("{:?}", ());
        // N s_91_1: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __SCR_EL3_IRQ:u8
        let s_92_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#86071 <= s_92_4
        fn_state.gs_86071 = s_92_4;
        // N s_92_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_93_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_93_1: call __IMPDEF_boolean(s_93_0)
        let s_93_1: bool = u__IMPDEF_boolean(state, tracer, s_93_0);
        // D s_93_2: write-var gs#86070 <= s_93_1
        fn_state.gs_86070 = s_93_1;
        // N s_93_3: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var __EDSCR_SDD:u8
        let s_94_0: bool = fn_state.u__EDSCR_SDD;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 1u16);
        // C s_94_2: const #1u : u8
        let s_94_2: bool = true;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // D s_94_5: write-var gs#86069 <= s_94_4
        fn_state.gs_86069 = s_94_4;
        // N s_94_6: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #424u : u32
        let s_95_0: u32 = 424;
        // D s_95_1: read-reg s_95_0:u8
        let s_95_1: u8 = {
            let value = state.read_register::<u8>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // C s_95_2: const #2u : u8
        let s_95_2: u8 = 2;
        // D s_95_3: cmp-lt s_95_1 s_95_2
        let s_95_3: bool = ((s_95_1) < (s_95_2));
        // D s_95_4: write-var gs#86068 <= s_95_3
        fn_state.gs_86068 = s_95_3;
        // N s_95_5: jump b43
        return block_43(state, tracer, fn_state);
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
}
