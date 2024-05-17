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
use u_get_HCR_EL2_Type_E2H::*;
use u_get_CPACR_EL1_Type_E0POE::*;
use Halted::*;
use IsFeatureImplemented::*;
use X_read::*;
use Mk_POR_EL0_Type::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_HFGWTR_EL2_Type_nPOR_EL0::*;
use u_get_HCR_EL2_Type_TVM::*;
use u_get_SCR_EL3_Type_PIEn::*;
use u_get_CPTR_EL2_Type_E0POE::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn POR_EL0_SysRegWrite_ebaa371431c9b104<T: Tracer>(
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
        gs_91558: bool,
        gs_91576: bool,
        gs_91564: bool,
        gs_91550: bool,
        gs_91572: bool,
        gs_91580: bool,
        gs_91569: bool,
        gs_91553: bool,
        gs_91579: bool,
        gs_91551: bool,
        gs_91547: bool,
        gs_91566: bool,
        gs_91567: bool,
        gs_91574: bool,
        gs_91549: bool,
        gs_91575: bool,
        u__CPACR_EL1_E0POE: bool,
        u__PSTATE_EL: u8,
        gs_91552: bool,
        gs_91571: bool,
        gs_91577: bool,
        gs_91557: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        u__SCR_EL3_PIEn: bool,
        gs_91556: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_91545: bool,
        gs_91554: bool,
        gs_91563: bool,
        gs_91559: bool,
        gs_91562: bool,
        u__HCR_EL2_TVM: bool,
        u__CPTR_EL2_E0POE: bool,
        gs_91573: bool,
        gs_91555: bool,
        gs_91568: bool,
        gs_91543: bool,
        gs_91544: bool,
        gs_91546: bool,
        gs_91561: bool,
        gs_91570: bool,
        u__HFGWTR_EL2_nPOR_EL0: bool,
        gs_91565: bool,
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
        // D s_0_9: call _get_SCR_EL3_Type_PIEn(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_PIEn(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_PIEn <= s_0_9
        fn_state.u__SCR_EL3_PIEn = s_0_9;
        // C s_0_11: const #12088u : u32
        let s_0_11: u32 = 12088;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CPACR_EL1_Type_E0POE(s_0_12)
        let s_0_13: bool = u_get_CPACR_EL1_Type_E0POE(state, tracer, s_0_12);
        // D s_0_14: write-var __CPACR_EL1_E0POE <= s_0_13
        fn_state.u__CPACR_EL1_E0POE = s_0_13;
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
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_TVM(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_TVM(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_TVM <= s_0_21
        fn_state.u__HCR_EL2_TVM = s_0_21;
        // C s_0_23: const #90704u : u32
        let s_0_23: u32 = 90704;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SCR_EL3_Type_FGTEn(s_0_24)
        let s_0_25: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_24);
        // D s_0_26: write-var __SCR_EL3_FGTEn <= s_0_25
        fn_state.u__SCR_EL3_FGTEn = s_0_25;
        // C s_0_27: const #100992u : u32
        let s_0_27: u32 = 100992;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_HFGWTR_EL2_Type_nPOR_EL0(s_0_28)
        let s_0_29: bool = u_get_HFGWTR_EL2_Type_nPOR_EL0(state, tracer, s_0_28);
        // D s_0_30: write-var __HFGWTR_EL2_nPOR_EL0 <= s_0_29
        fn_state.u__HFGWTR_EL2_nPOR_EL0 = s_0_29;
        // C s_0_31: const #11088u : u32
        let s_0_31: u32 = 11088;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_CPTR_EL2_Type_E0POE(s_0_32)
        let s_0_33: bool = u_get_CPTR_EL2_Type_E0POE(state, tracer, s_0_32);
        // D s_0_34: write-var __CPTR_EL2_E0POE <= s_0_33
        fn_state.u__CPTR_EL2_E0POE = s_0_33;
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
        // N s_0_41: branch s_0_40 b75 b1
        if s_0_40 {
            return block_75(state, tracer, fn_state);
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
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // D s_5_4: call Mk_POR_EL0_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_POR_EL0_Type(state, tracer, s_5_3);
        // C s_5_5: const #102688u : u32
        let s_5_5: u32 = 102688;
        // N s_5_6: write-reg s_5_5 <= s_5_4
        let s_5_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_5 as isize, s_5_4);
            tracer.write_register(s_5_5 as isize, s_5_4);
        };
        // N s_5_7: return
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
        // D s_7_1: write-var gs#91543 <= s_7_0
        fn_state.gs_91543 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#91543:u8
        let s_8_0: bool = fn_state.gs_91543;
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
        // D s_9_1: write-var gs#91544 <= s_9_0
        fn_state.gs_91544 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#91544:u8
        let s_10_0: bool = fn_state.gs_91544;
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
        // D s_11_1: write-var gs#91545 <= s_11_0
        fn_state.gs_91545 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#91545:u8
        let s_12_0: bool = fn_state.gs_91545;
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
        // D s_13_1: write-var gs#91546 <= s_13_0
        fn_state.gs_91546 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#91546:u8
        let s_14_0: bool = fn_state.gs_91546;
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
        // D s_16_1: write-var gs#91547 <= s_16_0
        fn_state.gs_91547 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#91547:u8
        let s_17_0: bool = fn_state.gs_91547;
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
        // D s_18_1: read-var t:i
        let s_18_1: i128 = fn_state.t;
        // D s_18_2: call X_read(s_18_1, s_18_0)
        let s_18_2: Bits = X_read(state, tracer, s_18_1, s_18_0);
        // D s_18_3: cast reint s_18_2 -> u64
        let s_18_3: u64 = (s_18_2.value() as u64);
        // D s_18_4: call Mk_POR_EL0_Type(s_18_3)
        let s_18_4: ProductType5c790c8ef59cc8b2 = Mk_POR_EL0_Type(state, tracer, s_18_3);
        // C s_18_5: const #102688u : u32
        let s_18_5: u32 = 102688;
        // N s_18_6: write-reg s_18_5 <= s_18_4
        let s_18_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_18_5 as isize, s_18_4);
            tracer.write_register(s_18_5 as isize, s_18_4);
        };
        // N s_18_7: return
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
        // D s_20_1: write-var gs#91549 <= s_20_0
        fn_state.gs_91549 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#91549:u8
        let s_21_0: bool = fn_state.gs_91549;
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
        // D s_24_5: write-var gs#91549 <= s_24_4
        fn_state.gs_91549 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __SCR_EL3_PIEn:u8
        let s_25_0: bool = fn_state.u__SCR_EL3_PIEn;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#91547 <= s_25_4
        fn_state.gs_91547 = s_25_4;
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
        // D s_27_0: read-var __SCR_EL3_PIEn:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_PIEn;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#91546 <= s_27_4
        fn_state.gs_91546 = s_27_4;
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
        // D s_28_2: write-var gs#91545 <= s_28_1
        fn_state.gs_91545 = s_28_1;
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
        // D s_29_5: write-var gs#91544 <= s_29_4
        fn_state.gs_91544 = s_29_4;
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
        // D s_30_4: write-var gs#91543 <= s_30_3
        fn_state.gs_91543 = s_30_3;
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
        // N s_31_2: branch s_31_1 b74 b32
        if s_31_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#91550 <= s_32_0
        fn_state.gs_91550 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#91550:u8
        let s_33_0: bool = fn_state.gs_91550;
        // N s_33_1: branch s_33_0 b73 b34
        if s_33_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#91551 <= s_34_0
        fn_state.gs_91551 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#91551:u8
        let s_35_0: bool = fn_state.gs_91551;
        // N s_35_1: branch s_35_0 b72 b36
        if s_35_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#91552 <= s_36_0
        fn_state.gs_91552 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#91552:u8
        let s_37_0: bool = fn_state.gs_91552;
        // N s_37_1: branch s_37_0 b71 b38
        if s_37_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#91553 <= s_38_0
        fn_state.gs_91553 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#91553:u8
        let s_39_0: bool = fn_state.gs_91553;
        // N s_39_1: branch s_39_0 b70 b40
        if s_39_0 {
            return block_70(state, tracer, fn_state);
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
        // N s_40_2: branch s_40_1 b69 b41
        if s_40_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#91554 <= s_41_0
        fn_state.gs_91554 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#91554:u8
        let s_42_0: bool = fn_state.gs_91554;
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
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EL2Enabled(s_43_0)
        let s_43_1: bool = EL2Enabled(state, tracer, s_43_0);
        // N s_43_2: branch s_43_1 b67 b44
        if s_43_1 {
            return block_67(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#91555 <= s_44_0
        fn_state.gs_91555 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#91555:u8
        let s_45_0: bool = fn_state.gs_91555;
        // N s_45_1: branch s_45_0 b63 b46
        if s_45_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#91557 <= s_46_0
        fn_state.gs_91557 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#91557:u8
        let s_47_0: bool = fn_state.gs_91557;
        // N s_47_1: branch s_47_0 b62 b48
        if s_47_0 {
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
        // D s_48_1: write-var gs#91558 <= s_48_0
        fn_state.gs_91558 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#91558:u8
        let s_49_0: bool = fn_state.gs_91558;
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
        // D s_51_1: write-var gs#91559 <= s_51_0
        fn_state.gs_91559 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#91559:u8
        let s_52_0: bool = fn_state.gs_91559;
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
        // C s_53_0: const #64s : i64
        let s_53_0: i64 = 64;
        // D s_53_1: read-var t:i
        let s_53_1: i128 = fn_state.t;
        // D s_53_2: call X_read(s_53_1, s_53_0)
        let s_53_2: Bits = X_read(state, tracer, s_53_1, s_53_0);
        // D s_53_3: cast reint s_53_2 -> u64
        let s_53_3: u64 = (s_53_2.value() as u64);
        // D s_53_4: call Mk_POR_EL0_Type(s_53_3)
        let s_53_4: ProductType5c790c8ef59cc8b2 = Mk_POR_EL0_Type(state, tracer, s_53_3);
        // C s_53_5: const #102688u : u32
        let s_53_5: u32 = 102688;
        // N s_53_6: write-reg s_53_5 <= s_53_4
        let s_53_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_53_5 as isize, s_53_4);
            tracer.write_register(s_53_5 as isize, s_53_4);
        };
        // N s_53_7: return
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
        // D s_55_1: write-var gs#91561 <= s_55_0
        fn_state.gs_91561 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#91561:u8
        let s_56_0: bool = fn_state.gs_91561;
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
        // D s_59_5: write-var gs#91561 <= s_59_4
        fn_state.gs_91561 = s_59_4;
        // N s_59_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __SCR_EL3_PIEn:u8
        let s_60_0: bool = fn_state.u__SCR_EL3_PIEn;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #0u : u8
        let s_60_2: bool = false;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#91559 <= s_60_4
        fn_state.gs_91559 = s_60_4;
        // N s_60_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #24u : u8
        let s_61_0: u8 = 24;
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
        // D s_62_0: read-var __HFGWTR_EL2_nPOR_EL0:u8
        let s_62_0: bool = fn_state.u__HFGWTR_EL2_nPOR_EL0;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #0u : u8
        let s_62_2: bool = false;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#91558 <= s_62_4
        fn_state.gs_91558 = s_62_4;
        // N s_62_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #424u : u32
        let s_63_0: u32 = 424;
        // D s_63_1: read-reg s_63_0:u8
        let s_63_1: u8 = {
            let value = state.read_register::<u8>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // C s_63_2: const #2u : u8
        let s_63_2: u8 = 2;
        // D s_63_3: cmp-lt s_63_1 s_63_2
        let s_63_3: bool = ((s_63_1) < (s_63_2));
        // D s_63_4: not s_63_3
        let s_63_4: bool = !s_63_3;
        // N s_63_5: branch s_63_4 b66 b64
        if s_63_4 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __SCR_EL3_FGTEn:u8
        let s_64_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#91556 <= s_64_4
        fn_state.gs_91556 = s_64_4;
        // N s_64_6: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#91556:u8
        let s_65_0: bool = fn_state.gs_91556;
        // D s_65_1: write-var gs#91557 <= s_65_0
        fn_state.gs_91557 = s_65_0;
        // N s_65_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#91556 <= s_66_0
        fn_state.gs_91556 = s_66_0;
        // N s_66_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #146u : u32
        let s_67_0: u32 = 146;
        // S s_67_1: call IsFeatureImplemented(s_67_0)
        let s_67_1: bool = IsFeatureImplemented(state, tracer, s_67_0);
        // D s_67_2: write-var gs#91555 <= s_67_1
        fn_state.gs_91555 = s_67_1;
        // N s_67_3: jump b45
        return block_45(state, tracer, fn_state);
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
        // C s_68_5: const #432u : u32
        let s_68_5: u32 = 432;
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
        // D s_69_0: read-var __HCR_EL2_TVM:u8
        let s_69_0: bool = fn_state.u__HCR_EL2_TVM;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#91554 <= s_69_4
        fn_state.gs_91554 = s_69_4;
        // N s_69_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_70_0: panic
        panic!("{:?}", ());
        // N s_70_1: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __SCR_EL3_PIEn:u8
        let s_71_0: bool = fn_state.u__SCR_EL3_PIEn;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #0u : u8
        let s_71_2: bool = false;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#91553 <= s_71_4
        fn_state.gs_91553 = s_71_4;
        // N s_71_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_72_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_72_1: call __IMPDEF_boolean(s_72_0)
        let s_72_1: bool = u__IMPDEF_boolean(state, tracer, s_72_0);
        // D s_72_2: write-var gs#91552 <= s_72_1
        fn_state.gs_91552 = s_72_1;
        // N s_72_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __EDSCR_SDD:u8
        let s_73_0: bool = fn_state.u__EDSCR_SDD;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#91551 <= s_73_4
        fn_state.gs_91551 = s_73_4;
        // N s_73_6: jump b35
        return block_35(state, tracer, fn_state);
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
        // D s_74_4: write-var gs#91550 <= s_74_3
        fn_state.gs_91550 = s_74_3;
        // N s_74_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call Halted(s_75_0)
        let s_75_1: bool = Halted(state, tracer, s_75_0);
        // N s_75_2: branch s_75_1 b145 b76
        if s_75_1 {
            return block_145(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#91562 <= s_76_0
        fn_state.gs_91562 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#91562:u8
        let s_77_0: bool = fn_state.gs_91562;
        // N s_77_1: branch s_77_0 b144 b78
        if s_77_0 {
            return block_144(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#91563 <= s_78_0
        fn_state.gs_91563 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#91563:u8
        let s_79_0: bool = fn_state.gs_91563;
        // N s_79_1: branch s_79_0 b143 b80
        if s_79_0 {
            return block_143(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#91564 <= s_80_0
        fn_state.gs_91564 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#91564:u8
        let s_81_0: bool = fn_state.gs_91564;
        // N s_81_1: branch s_81_0 b142 b82
        if s_81_0 {
            return block_142(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#91565 <= s_82_0
        fn_state.gs_91565 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#91565:u8
        let s_83_0: bool = fn_state.gs_91565;
        // N s_83_1: branch s_83_0 b141 b84
        if s_83_0 {
            return block_141(state, tracer, fn_state);
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
        // S s_84_1: call EL2Enabled(s_84_0)
        let s_84_1: bool = EL2Enabled(state, tracer, s_84_0);
        // N s_84_2: branch s_84_1 b140 b85
        if s_84_1 {
            return block_140(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#91566 <= s_85_0
        fn_state.gs_91566 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#91566:u8
        let s_86_0: bool = fn_state.gs_91566;
        // D s_86_1: not s_86_0
        let s_86_1: bool = !s_86_0;
        // N s_86_2: branch s_86_1 b139 b87
        if s_86_1 {
            return block_139(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#91567 <= s_87_0
        fn_state.gs_91567 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#91567:u8
        let s_88_0: bool = fn_state.gs_91567;
        // N s_88_1: branch s_88_0 b133 b89
        if s_88_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call EL2Enabled(s_89_0)
        let s_89_1: bool = EL2Enabled(state, tracer, s_89_0);
        // N s_89_2: branch s_89_1 b132 b90
        if s_89_1 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#91568 <= s_90_0
        fn_state.gs_91568 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#91568:u8
        let s_91_0: bool = fn_state.gs_91568;
        // N s_91_1: branch s_91_0 b131 b92
        if s_91_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#91569 <= s_92_0
        fn_state.gs_91569 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#91569:u8
        let s_93_0: bool = fn_state.gs_91569;
        // N s_93_1: branch s_93_0 b130 b94
        if s_93_0 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
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
        // N s_94_2: branch s_94_1 b129 b95
        if s_94_1 {
            return block_129(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#91570 <= s_95_0
        fn_state.gs_91570 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#91570:u8
        let s_96_0: bool = fn_state.gs_91570;
        // N s_96_1: branch s_96_0 b128 b97
        if s_96_0 {
            return block_128(state, tracer, fn_state);
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
        // D s_97_1: write-var gs#91571 <= s_97_0
        fn_state.gs_91571 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#91571:u8
        let s_98_0: bool = fn_state.gs_91571;
        // N s_98_1: branch s_98_0 b124 b99
        if s_98_0 {
            return block_124(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#91573 <= s_99_0
        fn_state.gs_91573 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#91573:u8
        let s_100_0: bool = fn_state.gs_91573;
        // N s_100_1: branch s_100_0 b123 b101
        if s_100_0 {
            return block_123(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#91574 <= s_101_0
        fn_state.gs_91574 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#91574:u8
        let s_102_0: bool = fn_state.gs_91574;
        // N s_102_1: branch s_102_0 b122 b103
        if s_102_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
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
        // N s_103_2: branch s_103_1 b121 b104
        if s_103_1 {
            return block_121(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#91575 <= s_104_0
        fn_state.gs_91575 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#91575:u8
        let s_105_0: bool = fn_state.gs_91575;
        // N s_105_1: branch s_105_0 b120 b106
        if s_105_0 {
            return block_120(state, tracer, fn_state);
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
        // D s_106_1: write-var gs#91576 <= s_106_0
        fn_state.gs_91576 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#91576:u8
        let s_107_0: bool = fn_state.gs_91576;
        // N s_107_1: branch s_107_0 b119 b108
        if s_107_0 {
            return block_119(state, tracer, fn_state);
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
        // N s_108_4: branch s_108_3 b118 b109
        if s_108_3 {
            return block_118(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#91577 <= s_109_0
        fn_state.gs_91577 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#91577:u8
        let s_110_0: bool = fn_state.gs_91577;
        // N s_110_1: branch s_110_0 b112 b111
        if s_110_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #64s : i64
        let s_111_0: i64 = 64;
        // D s_111_1: read-var t:i
        let s_111_1: i128 = fn_state.t;
        // D s_111_2: call X_read(s_111_1, s_111_0)
        let s_111_2: Bits = X_read(state, tracer, s_111_1, s_111_0);
        // D s_111_3: cast reint s_111_2 -> u64
        let s_111_3: u64 = (s_111_2.value() as u64);
        // D s_111_4: call Mk_POR_EL0_Type(s_111_3)
        let s_111_4: ProductType5c790c8ef59cc8b2 = Mk_POR_EL0_Type(
            state,
            tracer,
            s_111_3,
        );
        // C s_111_5: const #102688u : u32
        let s_111_5: u32 = 102688;
        // N s_111_6: write-reg s_111_5 <= s_111_4
        let s_111_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_111_5 as isize, s_111_4);
            tracer.write_register(s_111_5 as isize, s_111_4);
        };
        // N s_111_7: return
        return;
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #() : ()
        let s_112_0: () = ();
        // S s_112_1: call Halted(s_112_0)
        let s_112_1: bool = Halted(state, tracer, s_112_0);
        // N s_112_2: branch s_112_1 b117 b113
        if s_112_1 {
            return block_117(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#91579 <= s_113_0
        fn_state.gs_91579 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#91579:u8
        let s_114_0: bool = fn_state.gs_91579;
        // N s_114_1: branch s_114_0 b116 b115
        if s_114_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #24u : u8
        let s_115_0: u8 = 24;
        // C s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 8u16);
        // C s_115_2: cast zx s_115_1 -> i
        let s_115_2: i128 = (s_115_1.value() as i128);
        // C s_115_3: cast reint s_115_2 -> i64
        let s_115_3: i64 = (s_115_2 as i64);
        // C s_115_4: cast zx s_115_3 -> i
        let s_115_4: i128 = (i128::try_from(s_115_3).unwrap());
        // C s_115_5: const #424u : u32
        let s_115_5: u32 = 424;
        // D s_115_6: read-reg s_115_5:u8
        let s_115_6: u8 = {
            let value = state.read_register::<u8>(s_115_5 as isize);
            tracer.read_register(s_115_5 as isize, value);
            value
        };
        // D s_115_7: call AArch64_SystemAccessTrap(s_115_6, s_115_4)
        let s_115_7: () = AArch64_SystemAccessTrap(state, tracer, s_115_6, s_115_4);
        // N s_115_8: return
        return;
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_116_0: panic
        panic!("{:?}", ());
        // N s_116_1: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var __EDSCR_SDD:u8
        let s_117_0: bool = fn_state.u__EDSCR_SDD;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 1u16);
        // C s_117_2: const #1u : u8
        let s_117_2: bool = true;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_4: cmp-eq s_117_1 s_117_3
        let s_117_4: bool = ((s_117_1) == (s_117_3));
        // D s_117_5: write-var gs#91579 <= s_117_4
        fn_state.gs_91579 = s_117_4;
        // N s_117_6: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var __SCR_EL3_PIEn:u8
        let s_118_0: bool = fn_state.u__SCR_EL3_PIEn;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #0u : u8
        let s_118_2: bool = false;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: write-var gs#91577 <= s_118_4
        fn_state.gs_91577 = s_118_4;
        // N s_118_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #24u : u8
        let s_119_0: u8 = 24;
        // C s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 8u16);
        // C s_119_2: cast zx s_119_1 -> i
        let s_119_2: i128 = (s_119_1.value() as i128);
        // C s_119_3: cast reint s_119_2 -> i64
        let s_119_3: i64 = (s_119_2 as i64);
        // C s_119_4: cast zx s_119_3 -> i
        let s_119_4: i128 = (i128::try_from(s_119_3).unwrap());
        // C s_119_5: const #432u : u32
        let s_119_5: u32 = 432;
        // D s_119_6: read-reg s_119_5:u8
        let s_119_6: u8 = {
            let value = state.read_register::<u8>(s_119_5 as isize);
            tracer.read_register(s_119_5 as isize, value);
            value
        };
        // D s_119_7: call AArch64_SystemAccessTrap(s_119_6, s_119_4)
        let s_119_7: () = AArch64_SystemAccessTrap(state, tracer, s_119_6, s_119_4);
        // N s_119_8: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var __CPTR_EL2_E0POE:u8
        let s_120_0: bool = fn_state.u__CPTR_EL2_E0POE;
        // D s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 1u16);
        // C s_120_2: const #0u : u8
        let s_120_2: bool = false;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 1u16);
        // D s_120_4: cmp-eq s_120_1 s_120_3
        let s_120_4: bool = ((s_120_1) == (s_120_3));
        // D s_120_5: write-var gs#91576 <= s_120_4
        fn_state.gs_91576 = s_120_4;
        // N s_120_6: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #102552u : u32
        let s_121_0: u32 = 102552;
        // D s_121_1: read-reg s_121_0:struct
        let s_121_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_121_0 as isize);
            tracer.read_register(s_121_0 as isize, value);
            value
        };
        // D s_121_2: call _get_HCR_EL2_Type_E2H(s_121_1)
        let s_121_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_121_1);
        // C s_121_3: const #102552u : u32
        let s_121_3: u32 = 102552;
        // D s_121_4: read-reg s_121_3:struct
        let s_121_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_121_3 as isize);
            tracer.read_register(s_121_3 as isize, value);
            value
        };
        // D s_121_5: call _get_HCR_EL2_Type_TGE(s_121_4)
        let s_121_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_121_4);
        // D s_121_6: cast zx s_121_2 -> bv
        let s_121_6: Bits = Bits::new(s_121_2 as u128, 1u16);
        // D s_121_7: cast zx s_121_5 -> bv
        let s_121_7: Bits = Bits::new(s_121_5 as u128, 1u16);
        // D s_121_8: cast reint s_121_6 -> u128
        let s_121_8: u128 = (s_121_6.value() as u128);
        // D s_121_9: size-of s_121_6
        let s_121_9: u16 = s_121_6.length();
        // D s_121_10: cast reint s_121_7 -> u128
        let s_121_10: u128 = (s_121_7.value() as u128);
        // D s_121_11: size-of s_121_7
        let s_121_11: u16 = s_121_7.length();
        // D s_121_12: lsl s_121_8 s_121_11
        let s_121_12: u128 = s_121_8 << s_121_11;
        // D s_121_13: or s_121_12 s_121_10
        let s_121_13: u128 = ((s_121_12) | (s_121_10));
        // D s_121_14: add s_121_9 s_121_11
        let s_121_14: u16 = (s_121_9 + s_121_11);
        // D s_121_15: create-bits s_121_13 s_121_14
        let s_121_15: Bits = Bits::new(s_121_13, s_121_14);
        // D s_121_16: cast reint s_121_15 -> u8
        let s_121_16: u8 = (s_121_15.value() as u8);
        // D s_121_17: cast zx s_121_16 -> bv
        let s_121_17: Bits = Bits::new(s_121_16 as u128, 2u16);
        // C s_121_18: const #3u : u8
        let s_121_18: u8 = 3;
        // C s_121_19: cast zx s_121_18 -> bv
        let s_121_19: Bits = Bits::new(s_121_18 as u128, 2u16);
        // D s_121_20: cmp-eq s_121_17 s_121_19
        let s_121_20: bool = ((s_121_17) == (s_121_19));
        // D s_121_21: write-var gs#91575 <= s_121_20
        fn_state.gs_91575 = s_121_20;
        // N s_121_22: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #24u : u8
        let s_122_0: u8 = 24;
        // C s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 8u16);
        // C s_122_2: cast zx s_122_1 -> i
        let s_122_2: i128 = (s_122_1.value() as i128);
        // C s_122_3: cast reint s_122_2 -> i64
        let s_122_3: i64 = (s_122_2 as i64);
        // C s_122_4: cast zx s_122_3 -> i
        let s_122_4: i128 = (i128::try_from(s_122_3).unwrap());
        // C s_122_5: const #432u : u32
        let s_122_5: u32 = 432;
        // D s_122_6: read-reg s_122_5:u8
        let s_122_6: u8 = {
            let value = state.read_register::<u8>(s_122_5 as isize);
            tracer.read_register(s_122_5 as isize, value);
            value
        };
        // D s_122_7: call AArch64_SystemAccessTrap(s_122_6, s_122_4)
        let s_122_7: () = AArch64_SystemAccessTrap(state, tracer, s_122_6, s_122_4);
        // N s_122_8: return
        return;
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var __HFGWTR_EL2_nPOR_EL0:u8
        let s_123_0: bool = fn_state.u__HFGWTR_EL2_nPOR_EL0;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #0u : u8
        let s_123_2: bool = false;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // D s_123_5: write-var gs#91574 <= s_123_4
        fn_state.gs_91574 = s_123_4;
        // N s_123_6: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #424u : u32
        let s_124_0: u32 = 424;
        // D s_124_1: read-reg s_124_0:u8
        let s_124_1: u8 = {
            let value = state.read_register::<u8>(s_124_0 as isize);
            tracer.read_register(s_124_0 as isize, value);
            value
        };
        // C s_124_2: const #2u : u8
        let s_124_2: u8 = 2;
        // D s_124_3: cmp-lt s_124_1 s_124_2
        let s_124_3: bool = ((s_124_1) < (s_124_2));
        // D s_124_4: not s_124_3
        let s_124_4: bool = !s_124_3;
        // N s_124_5: branch s_124_4 b127 b125
        if s_124_4 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var __SCR_EL3_FGTEn:u8
        let s_125_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 1u16);
        // C s_125_2: const #1u : u8
        let s_125_2: bool = true;
        // C s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 1u16);
        // D s_125_4: cmp-eq s_125_1 s_125_3
        let s_125_4: bool = ((s_125_1) == (s_125_3));
        // D s_125_5: write-var gs#91572 <= s_125_4
        fn_state.gs_91572 = s_125_4;
        // N s_125_6: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#91572:u8
        let s_126_0: bool = fn_state.gs_91572;
        // D s_126_1: write-var gs#91573 <= s_126_0
        fn_state.gs_91573 = s_126_0;
        // N s_126_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #1u : u8
        let s_127_0: bool = true;
        // D s_127_1: write-var gs#91572 <= s_127_0
        fn_state.gs_91572 = s_127_0;
        // N s_127_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #146u : u32
        let s_128_0: u32 = 146;
        // S s_128_1: call IsFeatureImplemented(s_128_0)
        let s_128_1: bool = IsFeatureImplemented(state, tracer, s_128_0);
        // D s_128_2: write-var gs#91571 <= s_128_1
        fn_state.gs_91571 = s_128_1;
        // N s_128_3: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #102552u : u32
        let s_129_0: u32 = 102552;
        // D s_129_1: read-reg s_129_0:struct
        let s_129_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_129_0 as isize);
            tracer.read_register(s_129_0 as isize, value);
            value
        };
        // D s_129_2: call _get_HCR_EL2_Type_E2H(s_129_1)
        let s_129_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_129_1);
        // C s_129_3: const #102552u : u32
        let s_129_3: u32 = 102552;
        // D s_129_4: read-reg s_129_3:struct
        let s_129_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_129_3 as isize);
            tracer.read_register(s_129_3 as isize, value);
            value
        };
        // D s_129_5: call _get_HCR_EL2_Type_TGE(s_129_4)
        let s_129_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_129_4);
        // D s_129_6: cast zx s_129_2 -> bv
        let s_129_6: Bits = Bits::new(s_129_2 as u128, 1u16);
        // D s_129_7: cast zx s_129_5 -> bv
        let s_129_7: Bits = Bits::new(s_129_5 as u128, 1u16);
        // D s_129_8: cast reint s_129_6 -> u128
        let s_129_8: u128 = (s_129_6.value() as u128);
        // D s_129_9: size-of s_129_6
        let s_129_9: u16 = s_129_6.length();
        // D s_129_10: cast reint s_129_7 -> u128
        let s_129_10: u128 = (s_129_7.value() as u128);
        // D s_129_11: size-of s_129_7
        let s_129_11: u16 = s_129_7.length();
        // D s_129_12: lsl s_129_8 s_129_11
        let s_129_12: u128 = s_129_8 << s_129_11;
        // D s_129_13: or s_129_12 s_129_10
        let s_129_13: u128 = ((s_129_12) | (s_129_10));
        // D s_129_14: add s_129_9 s_129_11
        let s_129_14: u16 = (s_129_9 + s_129_11);
        // D s_129_15: create-bits s_129_13 s_129_14
        let s_129_15: Bits = Bits::new(s_129_13, s_129_14);
        // D s_129_16: cast reint s_129_15 -> u8
        let s_129_16: u8 = (s_129_15.value() as u8);
        // D s_129_17: cast zx s_129_16 -> bv
        let s_129_17: Bits = Bits::new(s_129_16 as u128, 2u16);
        // C s_129_18: const #3u : u8
        let s_129_18: u8 = 3;
        // C s_129_19: cast zx s_129_18 -> bv
        let s_129_19: Bits = Bits::new(s_129_18 as u128, 2u16);
        // D s_129_20: cmp-ne s_129_17 s_129_19
        let s_129_20: bool = ((s_129_17) != (s_129_19));
        // D s_129_21: write-var gs#91570 <= s_129_20
        fn_state.gs_91570 = s_129_20;
        // N s_129_22: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #24u : u8
        let s_130_0: u8 = 24;
        // C s_130_1: cast zx s_130_0 -> bv
        let s_130_1: Bits = Bits::new(s_130_0 as u128, 8u16);
        // C s_130_2: cast zx s_130_1 -> i
        let s_130_2: i128 = (s_130_1.value() as i128);
        // C s_130_3: cast reint s_130_2 -> i64
        let s_130_3: i64 = (s_130_2 as i64);
        // C s_130_4: cast zx s_130_3 -> i
        let s_130_4: i128 = (i128::try_from(s_130_3).unwrap());
        // C s_130_5: const #432u : u32
        let s_130_5: u32 = 432;
        // D s_130_6: read-reg s_130_5:u8
        let s_130_6: u8 = {
            let value = state.read_register::<u8>(s_130_5 as isize);
            tracer.read_register(s_130_5 as isize, value);
            value
        };
        // D s_130_7: call AArch64_SystemAccessTrap(s_130_6, s_130_4)
        let s_130_7: () = AArch64_SystemAccessTrap(state, tracer, s_130_6, s_130_4);
        // N s_130_8: return
        return;
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var __HCR_EL2_TVM:u8
        let s_131_0: bool = fn_state.u__HCR_EL2_TVM;
        // D s_131_1: cast zx s_131_0 -> bv
        let s_131_1: Bits = Bits::new(s_131_0 as u128, 1u16);
        // C s_131_2: const #1u : u8
        let s_131_2: bool = true;
        // C s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 1u16);
        // D s_131_4: cmp-eq s_131_1 s_131_3
        let s_131_4: bool = ((s_131_1) == (s_131_3));
        // D s_131_5: write-var gs#91569 <= s_131_4
        fn_state.gs_91569 = s_131_4;
        // N s_131_6: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #102552u : u32
        let s_132_0: u32 = 102552;
        // D s_132_1: read-reg s_132_0:struct
        let s_132_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // D s_132_2: call _get_HCR_EL2_Type_E2H(s_132_1)
        let s_132_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_132_1);
        // C s_132_3: const #102552u : u32
        let s_132_3: u32 = 102552;
        // D s_132_4: read-reg s_132_3:struct
        let s_132_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_132_3 as isize);
            tracer.read_register(s_132_3 as isize, value);
            value
        };
        // D s_132_5: call _get_HCR_EL2_Type_TGE(s_132_4)
        let s_132_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_132_4);
        // D s_132_6: cast zx s_132_2 -> bv
        let s_132_6: Bits = Bits::new(s_132_2 as u128, 1u16);
        // D s_132_7: cast zx s_132_5 -> bv
        let s_132_7: Bits = Bits::new(s_132_5 as u128, 1u16);
        // D s_132_8: cast reint s_132_6 -> u128
        let s_132_8: u128 = (s_132_6.value() as u128);
        // D s_132_9: size-of s_132_6
        let s_132_9: u16 = s_132_6.length();
        // D s_132_10: cast reint s_132_7 -> u128
        let s_132_10: u128 = (s_132_7.value() as u128);
        // D s_132_11: size-of s_132_7
        let s_132_11: u16 = s_132_7.length();
        // D s_132_12: lsl s_132_8 s_132_11
        let s_132_12: u128 = s_132_8 << s_132_11;
        // D s_132_13: or s_132_12 s_132_10
        let s_132_13: u128 = ((s_132_12) | (s_132_10));
        // D s_132_14: add s_132_9 s_132_11
        let s_132_14: u16 = (s_132_9 + s_132_11);
        // D s_132_15: create-bits s_132_13 s_132_14
        let s_132_15: Bits = Bits::new(s_132_13, s_132_14);
        // D s_132_16: cast reint s_132_15 -> u8
        let s_132_16: u8 = (s_132_15.value() as u8);
        // D s_132_17: cast zx s_132_16 -> bv
        let s_132_17: Bits = Bits::new(s_132_16 as u128, 2u16);
        // C s_132_18: const #3u : u8
        let s_132_18: u8 = 3;
        // C s_132_19: cast zx s_132_18 -> bv
        let s_132_19: Bits = Bits::new(s_132_18 as u128, 2u16);
        // D s_132_20: cmp-ne s_132_17 s_132_19
        let s_132_20: bool = ((s_132_17) != (s_132_19));
        // D s_132_21: write-var gs#91568 <= s_132_20
        fn_state.gs_91568 = s_132_20;
        // N s_132_22: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #() : ()
        let s_133_0: () = ();
        // S s_133_1: call EL2Enabled(s_133_0)
        let s_133_1: bool = EL2Enabled(state, tracer, s_133_0);
        // N s_133_2: branch s_133_1 b138 b134
        if s_133_1 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#91580 <= s_134_0
        fn_state.gs_91580 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#91580:u8
        let s_135_0: bool = fn_state.gs_91580;
        // N s_135_1: branch s_135_0 b137 b136
        if s_135_0 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #24u : u8
        let s_136_0: u8 = 24;
        // C s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 8u16);
        // C s_136_2: cast zx s_136_1 -> i
        let s_136_2: i128 = (s_136_1.value() as i128);
        // C s_136_3: cast reint s_136_2 -> i64
        let s_136_3: i64 = (s_136_2 as i64);
        // C s_136_4: cast zx s_136_3 -> i
        let s_136_4: i128 = (i128::try_from(s_136_3).unwrap());
        // C s_136_5: const #440u : u32
        let s_136_5: u32 = 440;
        // D s_136_6: read-reg s_136_5:u8
        let s_136_6: u8 = {
            let value = state.read_register::<u8>(s_136_5 as isize);
            tracer.read_register(s_136_5 as isize, value);
            value
        };
        // D s_136_7: call AArch64_SystemAccessTrap(s_136_6, s_136_4)
        let s_136_7: () = AArch64_SystemAccessTrap(state, tracer, s_136_6, s_136_4);
        // N s_136_8: return
        return;
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #24u : u8
        let s_137_0: u8 = 24;
        // C s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 8u16);
        // C s_137_2: cast zx s_137_1 -> i
        let s_137_2: i128 = (s_137_1.value() as i128);
        // C s_137_3: cast reint s_137_2 -> i64
        let s_137_3: i64 = (s_137_2 as i64);
        // C s_137_4: cast zx s_137_3 -> i
        let s_137_4: i128 = (i128::try_from(s_137_3).unwrap());
        // C s_137_5: const #432u : u32
        let s_137_5: u32 = 432;
        // D s_137_6: read-reg s_137_5:u8
        let s_137_6: u8 = {
            let value = state.read_register::<u8>(s_137_5 as isize);
            tracer.read_register(s_137_5 as isize, value);
            value
        };
        // D s_137_7: call AArch64_SystemAccessTrap(s_137_6, s_137_4)
        let s_137_7: () = AArch64_SystemAccessTrap(state, tracer, s_137_6, s_137_4);
        // N s_137_8: return
        return;
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var __HCR_EL2_TGE:u8
        let s_138_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_138_1: cast zx s_138_0 -> bv
        let s_138_1: Bits = Bits::new(s_138_0 as u128, 1u16);
        // C s_138_2: const #1u : u8
        let s_138_2: bool = true;
        // C s_138_3: cast zx s_138_2 -> bv
        let s_138_3: Bits = Bits::new(s_138_2 as u128, 1u16);
        // D s_138_4: cmp-eq s_138_1 s_138_3
        let s_138_4: bool = ((s_138_1) == (s_138_3));
        // D s_138_5: write-var gs#91580 <= s_138_4
        fn_state.gs_91580 = s_138_4;
        // N s_138_6: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var __CPACR_EL1_E0POE:u8
        let s_139_0: bool = fn_state.u__CPACR_EL1_E0POE;
        // D s_139_1: cast zx s_139_0 -> bv
        let s_139_1: Bits = Bits::new(s_139_0 as u128, 1u16);
        // C s_139_2: const #0u : u8
        let s_139_2: bool = false;
        // C s_139_3: cast zx s_139_2 -> bv
        let s_139_3: Bits = Bits::new(s_139_2 as u128, 1u16);
        // D s_139_4: cmp-eq s_139_1 s_139_3
        let s_139_4: bool = ((s_139_1) == (s_139_3));
        // D s_139_5: write-var gs#91567 <= s_139_4
        fn_state.gs_91567 = s_139_4;
        // N s_139_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #102552u : u32
        let s_140_0: u32 = 102552;
        // D s_140_1: read-reg s_140_0:struct
        let s_140_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_140_0 as isize);
            tracer.read_register(s_140_0 as isize, value);
            value
        };
        // D s_140_2: call _get_HCR_EL2_Type_E2H(s_140_1)
        let s_140_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_140_1);
        // C s_140_3: const #102552u : u32
        let s_140_3: u32 = 102552;
        // D s_140_4: read-reg s_140_3:struct
        let s_140_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_140_3 as isize);
            tracer.read_register(s_140_3 as isize, value);
            value
        };
        // D s_140_5: call _get_HCR_EL2_Type_TGE(s_140_4)
        let s_140_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_140_4);
        // D s_140_6: cast zx s_140_2 -> bv
        let s_140_6: Bits = Bits::new(s_140_2 as u128, 1u16);
        // D s_140_7: cast zx s_140_5 -> bv
        let s_140_7: Bits = Bits::new(s_140_5 as u128, 1u16);
        // D s_140_8: cast reint s_140_6 -> u128
        let s_140_8: u128 = (s_140_6.value() as u128);
        // D s_140_9: size-of s_140_6
        let s_140_9: u16 = s_140_6.length();
        // D s_140_10: cast reint s_140_7 -> u128
        let s_140_10: u128 = (s_140_7.value() as u128);
        // D s_140_11: size-of s_140_7
        let s_140_11: u16 = s_140_7.length();
        // D s_140_12: lsl s_140_8 s_140_11
        let s_140_12: u128 = s_140_8 << s_140_11;
        // D s_140_13: or s_140_12 s_140_10
        let s_140_13: u128 = ((s_140_12) | (s_140_10));
        // D s_140_14: add s_140_9 s_140_11
        let s_140_14: u16 = (s_140_9 + s_140_11);
        // D s_140_15: create-bits s_140_13 s_140_14
        let s_140_15: Bits = Bits::new(s_140_13, s_140_14);
        // D s_140_16: cast reint s_140_15 -> u8
        let s_140_16: u8 = (s_140_15.value() as u8);
        // D s_140_17: cast zx s_140_16 -> bv
        let s_140_17: Bits = Bits::new(s_140_16 as u128, 2u16);
        // C s_140_18: const #3u : u8
        let s_140_18: u8 = 3;
        // C s_140_19: cast zx s_140_18 -> bv
        let s_140_19: Bits = Bits::new(s_140_18 as u128, 2u16);
        // D s_140_20: cmp-eq s_140_17 s_140_19
        let s_140_20: bool = ((s_140_17) == (s_140_19));
        // D s_140_21: write-var gs#91566 <= s_140_20
        fn_state.gs_91566 = s_140_20;
        // N s_140_22: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_141_0: panic
        panic!("{:?}", ());
        // N s_141_1: return
        return;
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var __SCR_EL3_PIEn:u8
        let s_142_0: bool = fn_state.u__SCR_EL3_PIEn;
        // D s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 1u16);
        // C s_142_2: const #0u : u8
        let s_142_2: bool = false;
        // C s_142_3: cast zx s_142_2 -> bv
        let s_142_3: Bits = Bits::new(s_142_2 as u128, 1u16);
        // D s_142_4: cmp-eq s_142_1 s_142_3
        let s_142_4: bool = ((s_142_1) == (s_142_3));
        // D s_142_5: write-var gs#91565 <= s_142_4
        fn_state.gs_91565 = s_142_4;
        // N s_142_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_143_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_143_1: call __IMPDEF_boolean(s_143_0)
        let s_143_1: bool = u__IMPDEF_boolean(state, tracer, s_143_0);
        // D s_143_2: write-var gs#91564 <= s_143_1
        fn_state.gs_91564 = s_143_1;
        // N s_143_3: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var __EDSCR_SDD:u8
        let s_144_0: bool = fn_state.u__EDSCR_SDD;
        // D s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 1u16);
        // C s_144_2: const #1u : u8
        let s_144_2: bool = true;
        // C s_144_3: cast zx s_144_2 -> bv
        let s_144_3: Bits = Bits::new(s_144_2 as u128, 1u16);
        // D s_144_4: cmp-eq s_144_1 s_144_3
        let s_144_4: bool = ((s_144_1) == (s_144_3));
        // D s_144_5: write-var gs#91563 <= s_144_4
        fn_state.gs_91563 = s_144_4;
        // N s_144_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #424u : u32
        let s_145_0: u32 = 424;
        // D s_145_1: read-reg s_145_0:u8
        let s_145_1: u8 = {
            let value = state.read_register::<u8>(s_145_0 as isize);
            tracer.read_register(s_145_0 as isize, value);
            value
        };
        // C s_145_2: const #2u : u8
        let s_145_2: u8 = 2;
        // D s_145_3: cmp-lt s_145_1 s_145_2
        let s_145_3: bool = ((s_145_1) < (s_145_2));
        // D s_145_4: write-var gs#91562 <= s_145_3
        fn_state.gs_91562 = s_145_3;
        // N s_145_5: jump b77
        return block_77(state, tracer, fn_state);
    }
}
