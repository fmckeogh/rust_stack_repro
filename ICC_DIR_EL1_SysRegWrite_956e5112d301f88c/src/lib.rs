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
use Halted::*;
use u_get_SCR_EL3_Type_IRQ::*;
use Mk_ICV_DIR_EL1_Type::*;
use AArch64_SystemAccessTrap::*;
use X_read::*;
use u_get_SCR_EL3_Type_FIQ::*;
use u__IMPDEF_boolean::*;
use u_get_HCR_EL2_Type_FMO::*;
use ICC_SRE_EL1_read::*;
use Mk_ICC_DIR_EL1_Type::*;
use u_get_ICC_SRE_EL1_Type_SRE::*;
use u_get_ICC_SRE_EL2_Type_SRE::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_ICH_HCR_EL2_Type_TC::*;
use u_get_HCR_EL2_Type_IMO::*;
use u_get_ICC_SRE_EL3_Type_SRE::*;
use EDSCR_read::*;
use u_get_ICH_HCR_EL2_Type_TDIR::*;
use common::*;
pub fn ICC_DIR_EL1_SysRegWrite_956e5112d301f88c<T: Tracer>(
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
        gs_86025: bool,
        gs_86024: bool,
        gs_86026: bool,
        u__ICH_HCR_EL2_TC: bool,
        u__EDSCR_SDD: bool,
        u__HCR_EL2_IMO: bool,
        u__ICC_SRE_EL1_SRE: bool,
        gs_86017: bool,
        u__ICH_HCR_EL2_TDIR: bool,
        gs_86030: bool,
        u__ICC_SRE_EL3_SRE: bool,
        gs_86021: bool,
        gs_86015: bool,
        gs_86028: bool,
        gs_86018: bool,
        gs_86029: bool,
        gs_86016: bool,
        u__ICC_SRE_EL2_SRE: bool,
        gs_86027: bool,
        u__HCR_EL2_FMO: bool,
        gs_86023: bool,
        gs_86022: bool,
        gs_86019: bool,
        u__PSTATE_EL: u8,
        gs_86032: bool,
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
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call ICC_SRE_EL1_read(s_0_7)
        let s_0_8: ProductType5c790c8ef59cc8b2 = ICC_SRE_EL1_read(state, tracer, s_0_7);
        // S s_0_9: call _get_ICC_SRE_EL1_Type_SRE(s_0_8)
        let s_0_9: bool = u_get_ICC_SRE_EL1_Type_SRE(state, tracer, s_0_8);
        // D s_0_10: write-var __ICC_SRE_EL1_SRE <= s_0_9
        fn_state.u__ICC_SRE_EL1_SRE = s_0_9;
        // C s_0_11: const #20992u : u32
        let s_0_11: u32 = 20992;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_ICH_HCR_EL2_Type_TDIR(s_0_12)
        let s_0_13: bool = u_get_ICH_HCR_EL2_Type_TDIR(state, tracer, s_0_12);
        // D s_0_14: write-var __ICH_HCR_EL2_TDIR <= s_0_13
        fn_state.u__ICH_HCR_EL2_TDIR = s_0_13;
        // C s_0_15: const #20992u : u32
        let s_0_15: u32 = 20992;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_ICH_HCR_EL2_Type_TC(s_0_16)
        let s_0_17: bool = u_get_ICH_HCR_EL2_Type_TC(state, tracer, s_0_16);
        // D s_0_18: write-var __ICH_HCR_EL2_TC <= s_0_17
        fn_state.u__ICH_HCR_EL2_TC = s_0_17;
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_FMO(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_FMO <= s_0_21
        fn_state.u__HCR_EL2_FMO = s_0_21;
        // C s_0_23: const #102552u : u32
        let s_0_23: u32 = 102552;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HCR_EL2_Type_IMO(s_0_24)
        let s_0_25: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_0_24);
        // D s_0_26: write-var __HCR_EL2_IMO <= s_0_25
        fn_state.u__HCR_EL2_IMO = s_0_25;
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
        // N s_1_6: branch s_1_5 b35 b2
        if s_1_5 {
            return block_35(state, tracer, fn_state);
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
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: read-var t:i
        let s_6_1: i128 = fn_state.t;
        // D s_6_2: call X_read(s_6_1, s_6_0)
        let s_6_2: Bits = X_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // D s_6_4: call Mk_ICC_DIR_EL1_Type(s_6_3)
        let s_6_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_DIR_EL1_Type(
            state,
            tracer,
            s_6_3,
        );
        // C s_6_5: const #20616u : u32
        let s_6_5: u32 = 20616;
        // N s_6_6: write-reg s_6_5 <= s_6_4
        let s_6_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_5 as isize, s_6_4);
            tracer.write_register(s_6_5 as isize, s_6_4);
        };
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #24u : u8
        let s_7_0: u8 = 24;
        // C s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 8u16);
        // C s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (s_7_1.value() as i128);
        // C s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #424u : u32
        let s_7_5: u32 = 424;
        // D s_7_6: read-reg s_7_5:u8
        let s_7_6: u8 = {
            let value = state.read_register::<u8>(s_7_5 as isize);
            tracer.read_register(s_7_5 as isize, value);
            value
        };
        // D s_7_7: call AArch64_SystemAccessTrap(s_7_6, s_7_4)
        let s_7_7: () = AArch64_SystemAccessTrap(state, tracer, s_7_6, s_7_4);
        // N s_7_8: return
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
        // N s_8_2: branch s_8_1 b34 b9
        if s_8_1 {
            return block_34(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#86015 <= s_9_0
        fn_state.gs_86015 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#86015:u8
        let s_10_0: bool = fn_state.gs_86015;
        // N s_10_1: branch s_10_0 b33 b11
        if s_10_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#86016 <= s_11_0
        fn_state.gs_86016 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#86016:u8
        let s_12_0: bool = fn_state.gs_86016;
        // N s_12_1: branch s_12_0 b32 b13
        if s_12_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#86017 <= s_13_0
        fn_state.gs_86017 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#86017:u8
        let s_14_0: bool = fn_state.gs_86017;
        // N s_14_1: branch s_14_0 b31 b15
        if s_14_0 {
            return block_31(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#86018 <= s_15_0
        fn_state.gs_86018 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#86018:u8
        let s_16_0: bool = fn_state.gs_86018;
        // N s_16_1: branch s_16_0 b30 b17
        if s_16_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var __ICC_SRE_EL2_SRE:u8
        let s_17_0: bool = fn_state.u__ICC_SRE_EL2_SRE;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 1u16);
        // C s_17_2: const #0u : u8
        let s_17_2: bool = false;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // N s_17_5: branch s_17_4 b29 b18
        if s_17_4 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #424u : u32
        let s_18_0: u32 = 424;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // C s_18_2: const #2u : u8
        let s_18_2: u8 = 2;
        // D s_18_3: cmp-lt s_18_1 s_18_2
        let s_18_3: bool = ((s_18_1) < (s_18_2));
        // N s_18_4: branch s_18_3 b28 b19
        if s_18_3 {
            return block_28(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#86019 <= s_19_0
        fn_state.gs_86019 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#86019:u8
        let s_20_0: bool = fn_state.gs_86019;
        // N s_20_1: branch s_20_0 b22 b21
        if s_20_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #64s : i64
        let s_21_0: i64 = 64;
        // D s_21_1: read-var t:i
        let s_21_1: i128 = fn_state.t;
        // D s_21_2: call X_read(s_21_1, s_21_0)
        let s_21_2: Bits = X_read(state, tracer, s_21_1, s_21_0);
        // D s_21_3: cast reint s_21_2 -> u64
        let s_21_3: u64 = (s_21_2.value() as u64);
        // D s_21_4: call Mk_ICC_DIR_EL1_Type(s_21_3)
        let s_21_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_DIR_EL1_Type(
            state,
            tracer,
            s_21_3,
        );
        // C s_21_5: const #20616u : u32
        let s_21_5: u32 = 20616;
        // N s_21_6: write-reg s_21_5 <= s_21_4
        let s_21_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_21_5 as isize, s_21_4);
            tracer.write_register(s_21_5 as isize, s_21_4);
        };
        // N s_21_7: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call Halted(s_22_0)
        let s_22_1: bool = Halted(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b27 b23
        if s_22_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#86021 <= s_23_0
        fn_state.gs_86021 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#86021:u8
        let s_24_0: bool = fn_state.gs_86021;
        // N s_24_1: branch s_24_0 b26 b25
        if s_24_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #24u : u8
        let s_25_0: u8 = 24;
        // C s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 8u16);
        // C s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (s_25_1.value() as i128);
        // C s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #424u : u32
        let s_25_5: u32 = 424;
        // D s_25_6: read-reg s_25_5:u8
        let s_25_6: u8 = {
            let value = state.read_register::<u8>(s_25_5 as isize);
            tracer.read_register(s_25_5 as isize, value);
            value
        };
        // D s_25_7: call AArch64_SystemAccessTrap(s_25_6, s_25_4)
        let s_25_7: () = AArch64_SystemAccessTrap(state, tracer, s_25_6, s_25_4);
        // N s_25_8: return
        return;
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
        // D s_27_0: read-var __EDSCR_SDD:u8
        let s_27_0: bool = fn_state.u__EDSCR_SDD;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#86021 <= s_27_4
        fn_state.gs_86021 = s_27_4;
        // N s_27_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #90704u : u32
        let s_28_0: u32 = 90704;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_SCR_EL3_Type_IRQ(s_28_1)
        let s_28_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_28_1);
        // C s_28_3: const #90704u : u32
        let s_28_3: u32 = 90704;
        // D s_28_4: read-reg s_28_3:struct
        let s_28_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_3 as isize);
            tracer.read_register(s_28_3 as isize, value);
            value
        };
        // D s_28_5: call _get_SCR_EL3_Type_FIQ(s_28_4)
        let s_28_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_28_4);
        // D s_28_6: cast zx s_28_2 -> bv
        let s_28_6: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_7: cast zx s_28_5 -> bv
        let s_28_7: Bits = Bits::new(s_28_5 as u128, 1u16);
        // D s_28_8: cast reint s_28_6 -> u128
        let s_28_8: u128 = (s_28_6.value() as u128);
        // D s_28_9: size-of s_28_6
        let s_28_9: u16 = s_28_6.length();
        // D s_28_10: cast reint s_28_7 -> u128
        let s_28_10: u128 = (s_28_7.value() as u128);
        // D s_28_11: size-of s_28_7
        let s_28_11: u16 = s_28_7.length();
        // D s_28_12: lsl s_28_8 s_28_11
        let s_28_12: u128 = s_28_8 << s_28_11;
        // D s_28_13: or s_28_12 s_28_10
        let s_28_13: u128 = ((s_28_12) | (s_28_10));
        // D s_28_14: add s_28_9 s_28_11
        let s_28_14: u16 = (s_28_9 + s_28_11);
        // D s_28_15: create-bits s_28_13 s_28_14
        let s_28_15: Bits = Bits::new(s_28_13, s_28_14);
        // D s_28_16: cast reint s_28_15 -> u8
        let s_28_16: u8 = (s_28_15.value() as u8);
        // D s_28_17: cast zx s_28_16 -> bv
        let s_28_17: Bits = Bits::new(s_28_16 as u128, 2u16);
        // C s_28_18: const #3u : u8
        let s_28_18: u8 = 3;
        // C s_28_19: cast zx s_28_18 -> bv
        let s_28_19: Bits = Bits::new(s_28_18 as u128, 2u16);
        // D s_28_20: cmp-eq s_28_17 s_28_19
        let s_28_20: bool = ((s_28_17) == (s_28_19));
        // D s_28_21: write-var gs#86019 <= s_28_20
        fn_state.gs_86019 = s_28_20;
        // N s_28_22: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #24u : u8
        let s_29_0: u8 = 24;
        // C s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 8u16);
        // C s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (s_29_1.value() as i128);
        // C s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // C s_29_5: const #432u : u32
        let s_29_5: u32 = 432;
        // D s_29_6: read-reg s_29_5:u8
        let s_29_6: u8 = {
            let value = state.read_register::<u8>(s_29_5 as isize);
            tracer.read_register(s_29_5 as isize, value);
            value
        };
        // D s_29_7: call AArch64_SystemAccessTrap(s_29_6, s_29_4)
        let s_29_7: () = AArch64_SystemAccessTrap(state, tracer, s_29_6, s_29_4);
        // N s_29_8: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: panic
        panic!("{:?}", ());
        // N s_30_1: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #90704u : u32
        let s_31_0: u32 = 90704;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_SCR_EL3_Type_IRQ(s_31_1)
        let s_31_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_31_1);
        // C s_31_3: const #90704u : u32
        let s_31_3: u32 = 90704;
        // D s_31_4: read-reg s_31_3:struct
        let s_31_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_3 as isize);
            tracer.read_register(s_31_3 as isize, value);
            value
        };
        // D s_31_5: call _get_SCR_EL3_Type_FIQ(s_31_4)
        let s_31_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_31_4);
        // D s_31_6: cast zx s_31_2 -> bv
        let s_31_6: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_7: cast zx s_31_5 -> bv
        let s_31_7: Bits = Bits::new(s_31_5 as u128, 1u16);
        // D s_31_8: cast reint s_31_6 -> u128
        let s_31_8: u128 = (s_31_6.value() as u128);
        // D s_31_9: size-of s_31_6
        let s_31_9: u16 = s_31_6.length();
        // D s_31_10: cast reint s_31_7 -> u128
        let s_31_10: u128 = (s_31_7.value() as u128);
        // D s_31_11: size-of s_31_7
        let s_31_11: u16 = s_31_7.length();
        // D s_31_12: lsl s_31_8 s_31_11
        let s_31_12: u128 = s_31_8 << s_31_11;
        // D s_31_13: or s_31_12 s_31_10
        let s_31_13: u128 = ((s_31_12) | (s_31_10));
        // D s_31_14: add s_31_9 s_31_11
        let s_31_14: u16 = (s_31_9 + s_31_11);
        // D s_31_15: create-bits s_31_13 s_31_14
        let s_31_15: Bits = Bits::new(s_31_13, s_31_14);
        // D s_31_16: cast reint s_31_15 -> u8
        let s_31_16: u8 = (s_31_15.value() as u8);
        // D s_31_17: cast zx s_31_16 -> bv
        let s_31_17: Bits = Bits::new(s_31_16 as u128, 2u16);
        // C s_31_18: const #3u : u8
        let s_31_18: u8 = 3;
        // C s_31_19: cast zx s_31_18 -> bv
        let s_31_19: Bits = Bits::new(s_31_18 as u128, 2u16);
        // D s_31_20: cmp-eq s_31_17 s_31_19
        let s_31_20: bool = ((s_31_17) == (s_31_19));
        // D s_31_21: write-var gs#86018 <= s_31_20
        fn_state.gs_86018 = s_31_20;
        // N s_31_22: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_32_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_32_1: call __IMPDEF_boolean(s_32_0)
        let s_32_1: bool = u__IMPDEF_boolean(state, tracer, s_32_0);
        // D s_32_2: write-var gs#86017 <= s_32_1
        fn_state.gs_86017 = s_32_1;
        // N s_32_3: jump b14
        return block_14(state, tracer, fn_state);
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
        // D s_33_5: write-var gs#86016 <= s_33_4
        fn_state.gs_86016 = s_33_4;
        // N s_33_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #424u : u32
        let s_34_0: u32 = 424;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // C s_34_2: const #2u : u8
        let s_34_2: u8 = 2;
        // D s_34_3: cmp-lt s_34_1 s_34_2
        let s_34_3: bool = ((s_34_1) < (s_34_2));
        // D s_34_4: write-var gs#86015 <= s_34_3
        fn_state.gs_86015 = s_34_3;
        // N s_34_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call Halted(s_35_0)
        let s_35_1: bool = Halted(state, tracer, s_35_0);
        // N s_35_2: branch s_35_1 b81 b36
        if s_35_1 {
            return block_81(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#86022 <= s_36_0
        fn_state.gs_86022 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#86022:u8
        let s_37_0: bool = fn_state.gs_86022;
        // N s_37_1: branch s_37_0 b80 b38
        if s_37_0 {
            return block_80(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#86023 <= s_38_0
        fn_state.gs_86023 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#86023:u8
        let s_39_0: bool = fn_state.gs_86023;
        // N s_39_1: branch s_39_0 b79 b40
        if s_39_0 {
            return block_79(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#86024 <= s_40_0
        fn_state.gs_86024 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#86024:u8
        let s_41_0: bool = fn_state.gs_86024;
        // N s_41_1: branch s_41_0 b78 b42
        if s_41_0 {
            return block_78(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#86025 <= s_42_0
        fn_state.gs_86025 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#86025:u8
        let s_43_0: bool = fn_state.gs_86025;
        // N s_43_1: branch s_43_0 b77 b44
        if s_43_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __ICC_SRE_EL1_SRE:u8
        let s_44_0: bool = fn_state.u__ICC_SRE_EL1_SRE;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #0u : u8
        let s_44_2: bool = false;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // N s_44_5: branch s_44_4 b76 b45
        if s_44_4 {
            return block_76(state, tracer, fn_state);
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
        // S s_45_1: call EL2Enabled(s_45_0)
        let s_45_1: bool = EL2Enabled(state, tracer, s_45_0);
        // N s_45_2: branch s_45_1 b75 b46
        if s_45_1 {
            return block_75(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#86026 <= s_46_0
        fn_state.gs_86026 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#86026:u8
        let s_47_0: bool = fn_state.gs_86026;
        // N s_47_1: branch s_47_0 b74 b48
        if s_47_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EL2Enabled(s_48_0)
        let s_48_1: bool = EL2Enabled(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b73 b49
        if s_48_1 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#86027 <= s_49_0
        fn_state.gs_86027 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#86027:u8
        let s_50_0: bool = fn_state.gs_86027;
        // N s_50_1: branch s_50_0 b72 b51
        if s_50_0 {
            return block_72(state, tracer, fn_state);
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
        // N s_51_2: branch s_51_1 b71 b52
        if s_51_1 {
            return block_71(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#86028 <= s_52_0
        fn_state.gs_86028 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#86028:u8
        let s_53_0: bool = fn_state.gs_86028;
        // N s_53_1: branch s_53_0 b70 b54
        if s_53_0 {
            return block_70(state, tracer, fn_state);
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
        // N s_54_2: branch s_54_1 b69 b55
        if s_54_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#86029 <= s_55_0
        fn_state.gs_86029 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#86029:u8
        let s_56_0: bool = fn_state.gs_86029;
        // N s_56_1: branch s_56_0 b68 b57
        if s_56_0 {
            return block_68(state, tracer, fn_state);
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
        // N s_57_4: branch s_57_3 b67 b58
        if s_57_3 {
            return block_67(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#86030 <= s_58_0
        fn_state.gs_86030 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#86030:u8
        let s_59_0: bool = fn_state.gs_86030;
        // N s_59_1: branch s_59_0 b61 b60
        if s_59_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #64s : i64
        let s_60_0: i64 = 64;
        // D s_60_1: read-var t:i
        let s_60_1: i128 = fn_state.t;
        // D s_60_2: call X_read(s_60_1, s_60_0)
        let s_60_2: Bits = X_read(state, tracer, s_60_1, s_60_0);
        // D s_60_3: cast reint s_60_2 -> u64
        let s_60_3: u64 = (s_60_2.value() as u64);
        // D s_60_4: call Mk_ICC_DIR_EL1_Type(s_60_3)
        let s_60_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_DIR_EL1_Type(
            state,
            tracer,
            s_60_3,
        );
        // C s_60_5: const #20616u : u32
        let s_60_5: u32 = 20616;
        // N s_60_6: write-reg s_60_5 <= s_60_4
        let s_60_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_60_5 as isize, s_60_4);
            tracer.write_register(s_60_5 as isize, s_60_4);
        };
        // N s_60_7: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #() : ()
        let s_61_0: () = ();
        // S s_61_1: call Halted(s_61_0)
        let s_61_1: bool = Halted(state, tracer, s_61_0);
        // N s_61_2: branch s_61_1 b66 b62
        if s_61_1 {
            return block_66(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#86032 <= s_62_0
        fn_state.gs_86032 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#86032:u8
        let s_63_0: bool = fn_state.gs_86032;
        // N s_63_1: branch s_63_0 b65 b64
        if s_63_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #24u : u8
        let s_64_0: u8 = 24;
        // C s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 8u16);
        // C s_64_2: cast zx s_64_1 -> i
        let s_64_2: i128 = (s_64_1.value() as i128);
        // C s_64_3: cast reint s_64_2 -> i64
        let s_64_3: i64 = (s_64_2 as i64);
        // C s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // C s_64_5: const #424u : u32
        let s_64_5: u32 = 424;
        // D s_64_6: read-reg s_64_5:u8
        let s_64_6: u8 = {
            let value = state.read_register::<u8>(s_64_5 as isize);
            tracer.read_register(s_64_5 as isize, value);
            value
        };
        // D s_64_7: call AArch64_SystemAccessTrap(s_64_6, s_64_4)
        let s_64_7: () = AArch64_SystemAccessTrap(state, tracer, s_64_6, s_64_4);
        // N s_64_8: return
        return;
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
        // D s_66_0: read-var __EDSCR_SDD:u8
        let s_66_0: bool = fn_state.u__EDSCR_SDD;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#86032 <= s_66_4
        fn_state.gs_86032 = s_66_4;
        // N s_66_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #90704u : u32
        let s_67_0: u32 = 90704;
        // D s_67_1: read-reg s_67_0:struct
        let s_67_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // D s_67_2: call _get_SCR_EL3_Type_IRQ(s_67_1)
        let s_67_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_67_1);
        // C s_67_3: const #90704u : u32
        let s_67_3: u32 = 90704;
        // D s_67_4: read-reg s_67_3:struct
        let s_67_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_67_3 as isize);
            tracer.read_register(s_67_3 as isize, value);
            value
        };
        // D s_67_5: call _get_SCR_EL3_Type_FIQ(s_67_4)
        let s_67_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_67_4);
        // D s_67_6: cast zx s_67_2 -> bv
        let s_67_6: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_7: cast zx s_67_5 -> bv
        let s_67_7: Bits = Bits::new(s_67_5 as u128, 1u16);
        // D s_67_8: cast reint s_67_6 -> u128
        let s_67_8: u128 = (s_67_6.value() as u128);
        // D s_67_9: size-of s_67_6
        let s_67_9: u16 = s_67_6.length();
        // D s_67_10: cast reint s_67_7 -> u128
        let s_67_10: u128 = (s_67_7.value() as u128);
        // D s_67_11: size-of s_67_7
        let s_67_11: u16 = s_67_7.length();
        // D s_67_12: lsl s_67_8 s_67_11
        let s_67_12: u128 = s_67_8 << s_67_11;
        // D s_67_13: or s_67_12 s_67_10
        let s_67_13: u128 = ((s_67_12) | (s_67_10));
        // D s_67_14: add s_67_9 s_67_11
        let s_67_14: u16 = (s_67_9 + s_67_11);
        // D s_67_15: create-bits s_67_13 s_67_14
        let s_67_15: Bits = Bits::new(s_67_13, s_67_14);
        // D s_67_16: cast reint s_67_15 -> u8
        let s_67_16: u8 = (s_67_15.value() as u8);
        // D s_67_17: cast zx s_67_16 -> bv
        let s_67_17: Bits = Bits::new(s_67_16 as u128, 2u16);
        // C s_67_18: const #3u : u8
        let s_67_18: u8 = 3;
        // C s_67_19: cast zx s_67_18 -> bv
        let s_67_19: Bits = Bits::new(s_67_18 as u128, 2u16);
        // D s_67_20: cmp-eq s_67_17 s_67_19
        let s_67_20: bool = ((s_67_17) == (s_67_19));
        // D s_67_21: write-var gs#86030 <= s_67_20
        fn_state.gs_86030 = s_67_20;
        // N s_67_22: jump b59
        return block_59(state, tracer, fn_state);
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
        // D s_68_4: call Mk_ICV_DIR_EL1_Type(s_68_3)
        let s_68_4: ProductType5c790c8ef59cc8b2 = Mk_ICV_DIR_EL1_Type(
            state,
            tracer,
            s_68_3,
        );
        // C s_68_5: const #23448u : u32
        let s_68_5: u32 = 23448;
        // N s_68_6: write-reg s_68_5 <= s_68_4
        let s_68_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_68_5 as isize, s_68_4);
            tracer.write_register(s_68_5 as isize, s_68_4);
        };
        // N s_68_7: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __HCR_EL2_IMO:u8
        let s_69_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#86029 <= s_69_4
        fn_state.gs_86029 = s_69_4;
        // N s_69_6: jump b56
        return block_56(state, tracer, fn_state);
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
        // D s_70_4: call Mk_ICV_DIR_EL1_Type(s_70_3)
        let s_70_4: ProductType5c790c8ef59cc8b2 = Mk_ICV_DIR_EL1_Type(
            state,
            tracer,
            s_70_3,
        );
        // C s_70_5: const #23448u : u32
        let s_70_5: u32 = 23448;
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
        // D s_71_0: read-var __HCR_EL2_FMO:u8
        let s_71_0: bool = fn_state.u__HCR_EL2_FMO;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#86028 <= s_71_4
        fn_state.gs_86028 = s_71_4;
        // N s_71_6: jump b53
        return block_53(state, tracer, fn_state);
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
        // D s_73_0: read-var __ICH_HCR_EL2_TC:u8
        let s_73_0: bool = fn_state.u__ICH_HCR_EL2_TC;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#86027 <= s_73_4
        fn_state.gs_86027 = s_73_4;
        // N s_73_6: jump b50
        return block_50(state, tracer, fn_state);
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
        // D s_75_0: read-var __ICH_HCR_EL2_TDIR:u8
        let s_75_0: bool = fn_state.u__ICH_HCR_EL2_TDIR;
        // D s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 1u16);
        // C s_75_2: const #1u : u8
        let s_75_2: bool = true;
        // C s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_4: cmp-eq s_75_1 s_75_3
        let s_75_4: bool = ((s_75_1) == (s_75_3));
        // D s_75_5: write-var gs#86026 <= s_75_4
        fn_state.gs_86026 = s_75_4;
        // N s_75_6: jump b47
        return block_47(state, tracer, fn_state);
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
        // C s_78_0: const #90704u : u32
        let s_78_0: u32 = 90704;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_SCR_EL3_Type_IRQ(s_78_1)
        let s_78_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_78_1);
        // C s_78_3: const #90704u : u32
        let s_78_3: u32 = 90704;
        // D s_78_4: read-reg s_78_3:struct
        let s_78_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_3 as isize);
            tracer.read_register(s_78_3 as isize, value);
            value
        };
        // D s_78_5: call _get_SCR_EL3_Type_FIQ(s_78_4)
        let s_78_5: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_78_4);
        // D s_78_6: cast zx s_78_2 -> bv
        let s_78_6: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_7: cast zx s_78_5 -> bv
        let s_78_7: Bits = Bits::new(s_78_5 as u128, 1u16);
        // D s_78_8: cast reint s_78_6 -> u128
        let s_78_8: u128 = (s_78_6.value() as u128);
        // D s_78_9: size-of s_78_6
        let s_78_9: u16 = s_78_6.length();
        // D s_78_10: cast reint s_78_7 -> u128
        let s_78_10: u128 = (s_78_7.value() as u128);
        // D s_78_11: size-of s_78_7
        let s_78_11: u16 = s_78_7.length();
        // D s_78_12: lsl s_78_8 s_78_11
        let s_78_12: u128 = s_78_8 << s_78_11;
        // D s_78_13: or s_78_12 s_78_10
        let s_78_13: u128 = ((s_78_12) | (s_78_10));
        // D s_78_14: add s_78_9 s_78_11
        let s_78_14: u16 = (s_78_9 + s_78_11);
        // D s_78_15: create-bits s_78_13 s_78_14
        let s_78_15: Bits = Bits::new(s_78_13, s_78_14);
        // D s_78_16: cast reint s_78_15 -> u8
        let s_78_16: u8 = (s_78_15.value() as u8);
        // D s_78_17: cast zx s_78_16 -> bv
        let s_78_17: Bits = Bits::new(s_78_16 as u128, 2u16);
        // C s_78_18: const #3u : u8
        let s_78_18: u8 = 3;
        // C s_78_19: cast zx s_78_18 -> bv
        let s_78_19: Bits = Bits::new(s_78_18 as u128, 2u16);
        // D s_78_20: cmp-eq s_78_17 s_78_19
        let s_78_20: bool = ((s_78_17) == (s_78_19));
        // D s_78_21: write-var gs#86025 <= s_78_20
        fn_state.gs_86025 = s_78_20;
        // N s_78_22: jump b43
        return block_43(state, tracer, fn_state);
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
        // D s_79_2: write-var gs#86024 <= s_79_1
        fn_state.gs_86024 = s_79_1;
        // N s_79_3: jump b41
        return block_41(state, tracer, fn_state);
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
        // D s_80_5: write-var gs#86023 <= s_80_4
        fn_state.gs_86023 = s_80_4;
        // N s_80_6: jump b39
        return block_39(state, tracer, fn_state);
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
        // D s_81_4: write-var gs#86022 <= s_81_3
        fn_state.gs_86022 = s_81_3;
        // N s_81_5: jump b37
        return block_37(state, tracer, fn_state);
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
