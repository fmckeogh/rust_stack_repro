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
use ICC_BPR1_EL1_write::*;
use Halted::*;
use u_get_SCR_EL3_Type_IRQ::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_SCR_EL3_Type_NS::*;
use ICC_SRE_EL1_read::*;
use u_get_ICH_HCR_EL2_Type_TALL1::*;
use u_get_ICC_SRE_EL1_Type_SRE::*;
use u_get_ICC_SRE_EL2_Type_SRE::*;
use u_get_EDSCR_Type_SDD::*;
use Mk_ICV_BPR1_EL1_Type::*;
use u_get_ICC_SRE_EL3_Type_SRE::*;
use Mk_ICC_BPR1_EL1_Type::*;
use u_get_HCR_EL2_Type_IMO::*;
use EDSCR_read::*;
use common::*;
pub fn ICV_BPR1_EL1_SysRegWrite_cf350ee97c59e5e0<T: Tracer>(
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
        gs_86437: bool,
        u__SCR_EL3_NS: bool,
        gs_86438: bool,
        gs_86425: bool,
        u__EDSCR_SDD: bool,
        u__HCR_EL2_IMO: bool,
        u__ICC_SRE_EL1_SRE: bool,
        gs_86424: bool,
        u__ICC_SRE_EL3_SRE: bool,
        u__ICH_HCR_EL2_TALL1: bool,
        gs_86435: bool,
        gs_86427: bool,
        gs_86432: bool,
        u__ICC_SRE_EL2_SRE: bool,
        gs_86436: bool,
        gs_86428: bool,
        gs_86426: bool,
        gs_86433: bool,
        gs_86439: bool,
        gs_86443: bool,
        u__PSTATE_EL: u8,
        gs_86434: bool,
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
        // C s_0_15: const #20992u : u32
        let s_0_15: u32 = 20992;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_ICH_HCR_EL2_Type_TALL1(s_0_16)
        let s_0_17: bool = u_get_ICH_HCR_EL2_Type_TALL1(state, tracer, s_0_16);
        // D s_0_18: write-var __ICH_HCR_EL2_TALL1 <= s_0_17
        fn_state.u__ICH_HCR_EL2_TALL1 = s_0_17;
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_IMO(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_IMO <= s_0_21
        fn_state.u__HCR_EL2_IMO = s_0_21;
        // C s_0_23: const #90704u : u32
        let s_0_23: u32 = 90704;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SCR_EL3_Type_NS(s_0_24)
        let s_0_25: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_0_24);
        // D s_0_26: write-var __SCR_EL3_NS <= s_0_25
        fn_state.u__SCR_EL3_NS = s_0_25;
        // C s_0_27: const #16368u : u32
        let s_0_27: u32 = 16368;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_ICC_SRE_EL2_Type_SRE(s_0_28)
        let s_0_29: bool = u_get_ICC_SRE_EL2_Type_SRE(state, tracer, s_0_28);
        // D s_0_30: write-var __ICC_SRE_EL2_SRE <= s_0_29
        fn_state.u__ICC_SRE_EL2_SRE = s_0_29;
        // C s_0_31: const #10200u : u32
        let s_0_31: u32 = 10200;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_ICC_SRE_EL3_Type_SRE(s_0_32)
        let s_0_33: bool = u_get_ICC_SRE_EL3_Type_SRE(state, tracer, s_0_32);
        // D s_0_34: write-var __ICC_SRE_EL3_SRE <= s_0_33
        fn_state.u__ICC_SRE_EL3_SRE = s_0_33;
        // D s_0_35: read-var __PSTATE_EL:u8
        let s_0_35: u8 = fn_state.u__PSTATE_EL;
        // D s_0_36: cast zx s_0_35 -> bv
        let s_0_36: Bits = Bits::new(s_0_35 as u128, 2u16);
        // C s_0_37: const #448u : u32
        let s_0_37: u32 = 448;
        // D s_0_38: read-reg s_0_37:u8
        let s_0_38: u8 = {
            let value = state.read_register::<u8>(s_0_37 as isize);
            tracer.read_register(s_0_37 as isize, value);
            value
        };
        // D s_0_39: cast zx s_0_38 -> bv
        let s_0_39: Bits = Bits::new(s_0_38 as u128, 2u16);
        // D s_0_40: cmp-eq s_0_36 s_0_39
        let s_0_40: bool = ((s_0_36) == (s_0_39));
        // N s_0_41: branch s_0_40 b82 b1
        if s_0_40 {
            return block_82(state, tracer, fn_state);
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
        // D s_7_4: call Mk_ICC_BPR1_EL1_Type(s_7_3)
        let s_7_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_BPR1_EL1_Type(
            state,
            tracer,
            s_7_3,
        );
        // C s_7_5: const #17344u : u32
        let s_7_5: u32 = 17344;
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
        // D s_8_4: call Mk_ICC_BPR1_EL1_Type(s_8_3)
        let s_8_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_BPR1_EL1_Type(
            state,
            tracer,
            s_8_3,
        );
        // C s_8_5: const #102528u : u32
        let s_8_5: u32 = 102528;
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
        // D s_11_1: write-var gs#86424 <= s_11_0
        fn_state.gs_86424 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#86424:u8
        let s_12_0: bool = fn_state.gs_86424;
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
        // D s_13_1: write-var gs#86425 <= s_13_0
        fn_state.gs_86425 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#86425:u8
        let s_14_0: bool = fn_state.gs_86425;
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
        // D s_15_1: write-var gs#86426 <= s_15_0
        fn_state.gs_86426 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#86426:u8
        let s_16_0: bool = fn_state.gs_86426;
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
        // D s_17_1: write-var gs#86427 <= s_17_0
        fn_state.gs_86427 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#86427:u8
        let s_18_0: bool = fn_state.gs_86427;
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
        // D s_21_1: write-var gs#86428 <= s_21_0
        fn_state.gs_86428 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#86428:u8
        let s_22_0: bool = fn_state.gs_86428;
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
        // D s_24_4: call Mk_ICC_BPR1_EL1_Type(s_24_3)
        let s_24_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_BPR1_EL1_Type(
            state,
            tracer,
            s_24_3,
        );
        // D s_24_5: call ICC_BPR1_EL1_write(s_24_4)
        let s_24_5: () = ICC_BPR1_EL1_write(state, tracer, s_24_4);
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
        // D s_26_4: call Mk_ICC_BPR1_EL1_Type(s_26_3)
        let s_26_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_BPR1_EL1_Type(
            state,
            tracer,
            s_26_3,
        );
        // C s_26_5: const #17344u : u32
        let s_26_5: u32 = 17344;
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
        // D s_27_4: call Mk_ICC_BPR1_EL1_Type(s_27_3)
        let s_27_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_BPR1_EL1_Type(
            state,
            tracer,
            s_27_3,
        );
        // C s_27_5: const #102528u : u32
        let s_27_5: u32 = 102528;
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
        // D s_29_1: write-var gs#86432 <= s_29_0
        fn_state.gs_86432 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#86432:u8
        let s_30_0: bool = fn_state.gs_86432;
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
        // D s_33_5: write-var gs#86432 <= s_33_4
        fn_state.gs_86432 = s_33_4;
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
        // D s_34_5: write-var gs#86428 <= s_34_4
        fn_state.gs_86428 = s_34_4;
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
        // D s_37_5: write-var gs#86427 <= s_37_4
        fn_state.gs_86427 = s_37_4;
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
        // D s_38_2: write-var gs#86426 <= s_38_1
        fn_state.gs_86426 = s_38_1;
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
        // D s_39_5: write-var gs#86425 <= s_39_4
        fn_state.gs_86425 = s_39_4;
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
        // D s_40_4: write-var gs#86424 <= s_40_3
        fn_state.gs_86424 = s_40_3;
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
        // N s_41_2: branch s_41_1 b81 b42
        if s_41_1 {
            return block_81(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#86433 <= s_42_0
        fn_state.gs_86433 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#86433:u8
        let s_43_0: bool = fn_state.gs_86433;
        // N s_43_1: branch s_43_0 b80 b44
        if s_43_0 {
            return block_80(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#86434 <= s_44_0
        fn_state.gs_86434 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#86434:u8
        let s_45_0: bool = fn_state.gs_86434;
        // N s_45_1: branch s_45_0 b79 b46
        if s_45_0 {
            return block_79(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#86435 <= s_46_0
        fn_state.gs_86435 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#86435:u8
        let s_47_0: bool = fn_state.gs_86435;
        // N s_47_1: branch s_47_0 b78 b48
        if s_47_0 {
            return block_78(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#86436 <= s_48_0
        fn_state.gs_86436 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#86436:u8
        let s_49_0: bool = fn_state.gs_86436;
        // N s_49_1: branch s_49_0 b77 b50
        if s_49_0 {
            return block_77(state, tracer, fn_state);
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
        // N s_50_5: branch s_50_4 b76 b51
        if s_50_4 {
            return block_76(state, tracer, fn_state);
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
        // N s_51_2: branch s_51_1 b75 b52
        if s_51_1 {
            return block_75(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#86437 <= s_52_0
        fn_state.gs_86437 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#86437:u8
        let s_53_0: bool = fn_state.gs_86437;
        // N s_53_1: branch s_53_0 b74 b54
        if s_53_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call EL2Enabled(s_54_0)
        let s_54_1: bool = EL2Enabled(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b73 b55
        if s_54_1 {
            return block_73(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#86438 <= s_55_0
        fn_state.gs_86438 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#86438:u8
        let s_56_0: bool = fn_state.gs_86438;
        // N s_56_1: branch s_56_0 b72 b57
        if s_56_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
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
        // C s_57_2: const #2u : u8
        let s_57_2: u8 = 2;
        // D s_57_3: cmp-lt s_57_1 s_57_2
        let s_57_3: bool = ((s_57_1) < (s_57_2));
        // N s_57_4: branch s_57_3 b71 b58
        if s_57_3 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#86439 <= s_58_0
        fn_state.gs_86439 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#86439:u8
        let s_59_0: bool = fn_state.gs_86439;
        // N s_59_1: branch s_59_0 b65 b60
        if s_59_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #424u : u32
        let s_60_0: u32 = 424;
        // D s_60_1: read-reg s_60_0:u8
        let s_60_1: u8 = {
            let value = state.read_register::<u8>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // C s_60_2: const #2u : u8
        let s_60_2: u8 = 2;
        // D s_60_3: cmp-lt s_60_1 s_60_2
        let s_60_3: bool = ((s_60_1) < (s_60_2));
        // N s_60_4: branch s_60_3 b62 b61
        if s_60_3 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #64s : i64
        let s_61_0: i64 = 64;
        // D s_61_1: read-var t:i
        let s_61_1: i128 = fn_state.t;
        // D s_61_2: call X_read(s_61_1, s_61_0)
        let s_61_2: Bits = X_read(state, tracer, s_61_1, s_61_0);
        // D s_61_3: cast reint s_61_2 -> u64
        let s_61_3: u64 = (s_61_2.value() as u64);
        // D s_61_4: call Mk_ICC_BPR1_EL1_Type(s_61_3)
        let s_61_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_BPR1_EL1_Type(
            state,
            tracer,
            s_61_3,
        );
        // D s_61_5: call ICC_BPR1_EL1_write(s_61_4)
        let s_61_5: () = ICC_BPR1_EL1_write(state, tracer, s_61_4);
        // N s_61_6: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __SCR_EL3_NS:u8
        let s_62_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #0u : u8
        let s_62_2: bool = false;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // N s_62_5: branch s_62_4 b64 b63
        if s_62_4 {
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
        // C s_63_0: const #64s : i64
        let s_63_0: i64 = 64;
        // D s_63_1: read-var t:i
        let s_63_1: i128 = fn_state.t;
        // D s_63_2: call X_read(s_63_1, s_63_0)
        let s_63_2: Bits = X_read(state, tracer, s_63_1, s_63_0);
        // D s_63_3: cast reint s_63_2 -> u64
        let s_63_3: u64 = (s_63_2.value() as u64);
        // D s_63_4: call Mk_ICC_BPR1_EL1_Type(s_63_3)
        let s_63_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_BPR1_EL1_Type(
            state,
            tracer,
            s_63_3,
        );
        // C s_63_5: const #17344u : u32
        let s_63_5: u32 = 17344;
        // N s_63_6: write-reg s_63_5 <= s_63_4
        let s_63_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_63_5 as isize, s_63_4);
            tracer.write_register(s_63_5 as isize, s_63_4);
        };
        // N s_63_7: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #64s : i64
        let s_64_0: i64 = 64;
        // D s_64_1: read-var t:i
        let s_64_1: i128 = fn_state.t;
        // D s_64_2: call X_read(s_64_1, s_64_0)
        let s_64_2: Bits = X_read(state, tracer, s_64_1, s_64_0);
        // D s_64_3: cast reint s_64_2 -> u64
        let s_64_3: u64 = (s_64_2.value() as u64);
        // D s_64_4: call Mk_ICC_BPR1_EL1_Type(s_64_3)
        let s_64_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_BPR1_EL1_Type(
            state,
            tracer,
            s_64_3,
        );
        // C s_64_5: const #102528u : u32
        let s_64_5: u32 = 102528;
        // N s_64_6: write-reg s_64_5 <= s_64_4
        let s_64_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_64_5 as isize, s_64_4);
            tracer.write_register(s_64_5 as isize, s_64_4);
        };
        // N s_64_7: return
        return;
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
        // D s_66_1: write-var gs#86443 <= s_66_0
        fn_state.gs_86443 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#86443:u8
        let s_67_0: bool = fn_state.gs_86443;
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
        // D s_70_5: write-var gs#86443 <= s_70_4
        fn_state.gs_86443 = s_70_4;
        // N s_70_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __SCR_EL3_IRQ:u8
        let s_71_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#86439 <= s_71_4
        fn_state.gs_86439 = s_71_4;
        // N s_71_6: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #64s : i64
        let s_72_0: i64 = 64;
        // D s_72_1: read-var t:i
        let s_72_1: i128 = fn_state.t;
        // D s_72_2: call X_read(s_72_1, s_72_0)
        let s_72_2: Bits = X_read(state, tracer, s_72_1, s_72_0);
        // D s_72_3: cast reint s_72_2 -> u64
        let s_72_3: u64 = (s_72_2.value() as u64);
        // D s_72_4: call Mk_ICV_BPR1_EL1_Type(s_72_3)
        let s_72_4: ProductType5c790c8ef59cc8b2 = Mk_ICV_BPR1_EL1_Type(
            state,
            tracer,
            s_72_3,
        );
        // C s_72_5: const #102456u : u32
        let s_72_5: u32 = 102456;
        // N s_72_6: write-reg s_72_5 <= s_72_4
        let s_72_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_72_5 as isize, s_72_4);
            tracer.write_register(s_72_5 as isize, s_72_4);
        };
        // N s_72_7: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __HCR_EL2_IMO:u8
        let s_73_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#86438 <= s_73_4
        fn_state.gs_86438 = s_73_4;
        // N s_73_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #24u : u8
        let s_74_0: u8 = 24;
        // C s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 8u16);
        // C s_74_2: cast zx s_74_1 -> i
        let s_74_2: i128 = (s_74_1.value() as i128);
        // C s_74_3: cast reint s_74_2 -> i64
        let s_74_3: i64 = (s_74_2 as i64);
        // C s_74_4: cast zx s_74_3 -> i
        let s_74_4: i128 = (i128::try_from(s_74_3).unwrap());
        // C s_74_5: const #432u : u32
        let s_74_5: u32 = 432;
        // D s_74_6: read-reg s_74_5:u8
        let s_74_6: u8 = {
            let value = state.read_register::<u8>(s_74_5 as isize);
            tracer.read_register(s_74_5 as isize, value);
            value
        };
        // D s_74_7: call AArch64_SystemAccessTrap(s_74_6, s_74_4)
        let s_74_7: () = AArch64_SystemAccessTrap(state, tracer, s_74_6, s_74_4);
        // N s_74_8: return
        return;
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var __ICH_HCR_EL2_TALL1:u8
        let s_75_0: bool = fn_state.u__ICH_HCR_EL2_TALL1;
        // D s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 1u16);
        // C s_75_2: const #1u : u8
        let s_75_2: bool = true;
        // C s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_4: cmp-eq s_75_1 s_75_3
        let s_75_4: bool = ((s_75_1) == (s_75_3));
        // D s_75_5: write-var gs#86437 <= s_75_4
        fn_state.gs_86437 = s_75_4;
        // N s_75_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #24u : u8
        let s_76_0: u8 = 24;
        // C s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 8u16);
        // C s_76_2: cast zx s_76_1 -> i
        let s_76_2: i128 = (s_76_1.value() as i128);
        // C s_76_3: cast reint s_76_2 -> i64
        let s_76_3: i64 = (s_76_2 as i64);
        // C s_76_4: cast zx s_76_3 -> i
        let s_76_4: i128 = (i128::try_from(s_76_3).unwrap());
        // C s_76_5: const #440u : u32
        let s_76_5: u32 = 440;
        // D s_76_6: read-reg s_76_5:u8
        let s_76_6: u8 = {
            let value = state.read_register::<u8>(s_76_5 as isize);
            tracer.read_register(s_76_5 as isize, value);
            value
        };
        // D s_76_7: call AArch64_SystemAccessTrap(s_76_6, s_76_4)
        let s_76_7: () = AArch64_SystemAccessTrap(state, tracer, s_76_6, s_76_4);
        // N s_76_8: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_77_0: panic
        panic!("{:?}", ());
        // N s_77_1: return
        return;
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
        // D s_78_5: write-var gs#86436 <= s_78_4
        fn_state.gs_86436 = s_78_4;
        // N s_78_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_79_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_79_1: call __IMPDEF_boolean(s_79_0)
        let s_79_1: bool = u__IMPDEF_boolean(state, tracer, s_79_0);
        // D s_79_2: write-var gs#86435 <= s_79_1
        fn_state.gs_86435 = s_79_1;
        // N s_79_3: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __EDSCR_SDD:u8
        let s_80_0: bool = fn_state.u__EDSCR_SDD;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#86434 <= s_80_4
        fn_state.gs_86434 = s_80_4;
        // N s_80_6: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #424u : u32
        let s_81_0: u32 = 424;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // C s_81_2: const #2u : u8
        let s_81_2: u8 = 2;
        // D s_81_3: cmp-lt s_81_1 s_81_2
        let s_81_3: bool = ((s_81_1) < (s_81_2));
        // D s_81_4: write-var gs#86433 <= s_81_3
        fn_state.gs_86433 = s_81_3;
        // N s_81_5: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_82_0: panic
        panic!("{:?}", ());
        // N s_82_1: return
        return;
    }
}
