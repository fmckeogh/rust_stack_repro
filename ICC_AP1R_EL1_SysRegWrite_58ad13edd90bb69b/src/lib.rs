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
use ICC_AP1R_EL1_set::*;
use Halted::*;
use u_get_SCR_EL3_Type_IRQ::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use Mk_ICC_AP1R_EL1_Type::*;
use u_get_SCR_EL3_Type_NS::*;
use u__IMPDEF_boolean::*;
use ICC_SRE_EL1_read::*;
use u_get_ICH_HCR_EL2_Type_TALL1::*;
use u_get_ICC_SRE_EL1_Type_SRE::*;
use u_get_ICC_SRE_EL2_Type_SRE::*;
use u_get_EDSCR_Type_SDD::*;
use Mk_ICV_AP1R_EL1_Type::*;
use u_get_ICC_SRE_EL3_Type_SRE::*;
use u_get_HCR_EL2_Type_IMO::*;
use EDSCR_read::*;
use EL2Enabled::*;
use common::*;
pub fn ICC_AP1R_EL1_SysRegWrite_58ad13edd90bb69b<T: Tracer>(
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
        u__SCR_EL3_NS: bool,
        gs_85928: bool,
        gs_85935: bool,
        u__EDSCR_SDD: bool,
        gs_85929: bool,
        u__HCR_EL2_IMO: bool,
        u__ICC_SRE_EL1_SRE: bool,
        gs_85920: bool,
        gs_85921: bool,
        u__ICC_SRE_EL3_SRE: bool,
        gs_85933: bool,
        u__ICH_HCR_EL2_TALL1: bool,
        gs_85932: bool,
        u__ICC_SRE_EL2_SRE: bool,
        gs_85942: bool,
        gs_85934: bool,
        gs_85919: bool,
        gs_85931: bool,
        gs_85917: bool,
        u__PSTATE_EL: u8,
        gs_85930: bool,
        gs_85918: bool,
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
        // N s_0_35: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
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
        // C s_2_2: const #448u : u32
        let s_2_2: u32 = 448;
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
        // N s_2_6: branch s_2_5 b84 b3
        if s_2_5 {
            return block_84(state, tracer, fn_state);
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
        // C s_3_2: const #440u : u32
        let s_3_2: u32 = 440;
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
        // N s_3_6: branch s_3_5 b43 b4
        if s_3_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #432u : u32
        let s_4_2: u32 = 432;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b12 b5
        if s_4_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var __PSTATE_EL:u8
        let s_5_0: u8 = fn_state.u__PSTATE_EL;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #424u : u32
        let s_5_2: u32 = 424;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
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
        // N s_6_0: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var __ICC_SRE_EL3_SRE:u8
        let s_7_0: bool = fn_state.u__ICC_SRE_EL3_SRE;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #0u : u8
        let s_7_2: bool = false;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b11 b8
        if s_7_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var __SCR_EL3_NS:u8
        let s_8_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // D s_9_1: read-var t:i
        let s_9_1: i128 = fn_state.t;
        // D s_9_2: call X_read(s_9_1, s_9_0)
        let s_9_2: Bits = X_read(state, tracer, s_9_1, s_9_0);
        // D s_9_3: cast reint s_9_2 -> u64
        let s_9_3: u64 = (s_9_2.value() as u64);
        // D s_9_4: call Mk_ICC_AP1R_EL1_Type(s_9_3)
        let s_9_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_AP1R_EL1_Type(
            state,
            tracer,
            s_9_3,
        );
        // C s_9_5: const #0s : i
        let s_9_5: i128 = 0;
        // C s_9_6: const #11536u : u32
        let s_9_6: u32 = 11536;
        // D s_9_7: read-reg s_9_6:[struct; 4]
        let s_9_7: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_9_6 as isize);
            tracer.read_register(s_9_6 as isize, value);
            value
        };
        // D s_9_8: mutate-element s_9_7[s_9_5] <= s_9_4
        let s_9_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_9_7.clone();
            local[(s_9_5) as usize] = s_9_4;
            local
        };
        // D s_9_9: cast cvt s_9_8 -> [struct; 0]
        let s_9_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_9_8,
        );
        // D s_9_10: cast cvt s_9_9 -> [struct; 4]
        let s_9_10: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_9_9);
            buf
        };
        // C s_9_11: const #11536u : u32
        let s_9_11: u32 = 11536;
        // N s_9_12: write-reg s_9_11 <= s_9_10
        let s_9_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_9_11 as isize, s_9_10);
            tracer.write_register(s_9_11 as isize, s_9_10);
        };
        // N s_9_13: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // D s_10_1: read-var t:i
        let s_10_1: i128 = fn_state.t;
        // D s_10_2: call X_read(s_10_1, s_10_0)
        let s_10_2: Bits = X_read(state, tracer, s_10_1, s_10_0);
        // D s_10_3: cast reint s_10_2 -> u64
        let s_10_3: u64 = (s_10_2.value() as u64);
        // D s_10_4: call Mk_ICC_AP1R_EL1_Type(s_10_3)
        let s_10_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_AP1R_EL1_Type(
            state,
            tracer,
            s_10_3,
        );
        // C s_10_5: const #0s : i
        let s_10_5: i128 = 0;
        // C s_10_6: const #1600u : u32
        let s_10_6: u32 = 1600;
        // D s_10_7: read-reg s_10_6:[struct; 4]
        let s_10_7: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_10_6 as isize);
            tracer.read_register(s_10_6 as isize, value);
            value
        };
        // D s_10_8: mutate-element s_10_7[s_10_5] <= s_10_4
        let s_10_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_10_7.clone();
            local[(s_10_5) as usize] = s_10_4;
            local
        };
        // D s_10_9: cast cvt s_10_8 -> [struct; 0]
        let s_10_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_10_8,
        );
        // D s_10_10: cast cvt s_10_9 -> [struct; 4]
        let s_10_10: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_10_9);
            buf
        };
        // C s_10_11: const #1600u : u32
        let s_10_11: u32 = 1600;
        // N s_10_12: write-reg s_10_11 <= s_10_10
        let s_10_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_10_11 as isize, s_10_10);
            tracer.write_register(s_10_11 as isize, s_10_10);
        };
        // N s_10_13: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #24u : u8
        let s_11_0: u8 = 24;
        // C s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 8u16);
        // C s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (s_11_1.value() as i128);
        // C s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // C s_11_5: const #424u : u32
        let s_11_5: u32 = 424;
        // D s_11_6: read-reg s_11_5:u8
        let s_11_6: u8 = {
            let value = state.read_register::<u8>(s_11_5 as isize);
            tracer.read_register(s_11_5 as isize, value);
            value
        };
        // D s_11_7: call AArch64_SystemAccessTrap(s_11_6, s_11_4)
        let s_11_7: () = AArch64_SystemAccessTrap(state, tracer, s_11_6, s_11_4);
        // N s_11_8: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call Halted(s_12_0)
        let s_12_1: bool = Halted(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b42 b13
        if s_12_1 {
            return block_42(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#85917 <= s_13_0
        fn_state.gs_85917 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#85917:u8
        let s_14_0: bool = fn_state.gs_85917;
        // N s_14_1: branch s_14_0 b41 b15
        if s_14_0 {
            return block_41(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#85918 <= s_15_0
        fn_state.gs_85918 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#85918:u8
        let s_16_0: bool = fn_state.gs_85918;
        // N s_16_1: branch s_16_0 b40 b17
        if s_16_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#85919 <= s_17_0
        fn_state.gs_85919 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#85919:u8
        let s_18_0: bool = fn_state.gs_85919;
        // N s_18_1: branch s_18_0 b39 b19
        if s_18_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#85920 <= s_19_0
        fn_state.gs_85920 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#85920:u8
        let s_20_0: bool = fn_state.gs_85920;
        // N s_20_1: branch s_20_0 b38 b21
        if s_20_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var __ICC_SRE_EL2_SRE:u8
        let s_21_0: bool = fn_state.u__ICC_SRE_EL2_SRE;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 1u16);
        // C s_21_2: const #0u : u8
        let s_21_2: bool = false;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // N s_21_5: branch s_21_4 b37 b22
        if s_21_4 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #424u : u32
        let s_22_0: u32 = 424;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // C s_22_2: const #2u : u8
        let s_22_2: u8 = 2;
        // D s_22_3: cmp-lt s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) < (s_22_2));
        // N s_22_4: branch s_22_3 b36 b23
        if s_22_3 {
            return block_36(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#85921 <= s_23_0
        fn_state.gs_85921 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#85921:u8
        let s_24_0: bool = fn_state.gs_85921;
        // N s_24_1: branch s_24_0 b30 b25
        if s_24_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #424u : u32
        let s_25_0: u32 = 424;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // C s_25_2: const #2u : u8
        let s_25_2: u8 = 2;
        // D s_25_3: cmp-lt s_25_1 s_25_2
        let s_25_3: bool = ((s_25_1) < (s_25_2));
        // N s_25_4: branch s_25_3 b27 b26
        if s_25_3 {
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
        // D s_26_4: call Mk_ICC_AP1R_EL1_Type(s_26_3)
        let s_26_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_AP1R_EL1_Type(
            state,
            tracer,
            s_26_3,
        );
        // C s_26_5: const #0s : i
        let s_26_5: i128 = 0;
        // D s_26_6: call ICC_AP1R_EL1_set(s_26_5, s_26_4)
        let s_26_6: () = ICC_AP1R_EL1_set(state, tracer, s_26_5, s_26_4);
        // N s_26_7: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __SCR_EL3_NS:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // N s_27_5: branch s_27_4 b29 b28
        if s_27_4 {
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
        // C s_28_0: const #64s : i64
        let s_28_0: i64 = 64;
        // D s_28_1: read-var t:i
        let s_28_1: i128 = fn_state.t;
        // D s_28_2: call X_read(s_28_1, s_28_0)
        let s_28_2: Bits = X_read(state, tracer, s_28_1, s_28_0);
        // D s_28_3: cast reint s_28_2 -> u64
        let s_28_3: u64 = (s_28_2.value() as u64);
        // D s_28_4: call Mk_ICC_AP1R_EL1_Type(s_28_3)
        let s_28_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_AP1R_EL1_Type(
            state,
            tracer,
            s_28_3,
        );
        // C s_28_5: const #0s : i
        let s_28_5: i128 = 0;
        // C s_28_6: const #11536u : u32
        let s_28_6: u32 = 11536;
        // D s_28_7: read-reg s_28_6:[struct; 4]
        let s_28_7: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_28_6 as isize);
            tracer.read_register(s_28_6 as isize, value);
            value
        };
        // D s_28_8: mutate-element s_28_7[s_28_5] <= s_28_4
        let s_28_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_28_7.clone();
            local[(s_28_5) as usize] = s_28_4;
            local
        };
        // D s_28_9: cast cvt s_28_8 -> [struct; 0]
        let s_28_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_28_8,
        );
        // D s_28_10: cast cvt s_28_9 -> [struct; 4]
        let s_28_10: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_28_9);
            buf
        };
        // C s_28_11: const #11536u : u32
        let s_28_11: u32 = 11536;
        // N s_28_12: write-reg s_28_11 <= s_28_10
        let s_28_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_28_11 as isize, s_28_10);
            tracer.write_register(s_28_11 as isize, s_28_10);
        };
        // N s_28_13: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #64s : i64
        let s_29_0: i64 = 64;
        // D s_29_1: read-var t:i
        let s_29_1: i128 = fn_state.t;
        // D s_29_2: call X_read(s_29_1, s_29_0)
        let s_29_2: Bits = X_read(state, tracer, s_29_1, s_29_0);
        // D s_29_3: cast reint s_29_2 -> u64
        let s_29_3: u64 = (s_29_2.value() as u64);
        // D s_29_4: call Mk_ICC_AP1R_EL1_Type(s_29_3)
        let s_29_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_AP1R_EL1_Type(
            state,
            tracer,
            s_29_3,
        );
        // C s_29_5: const #0s : i
        let s_29_5: i128 = 0;
        // C s_29_6: const #1600u : u32
        let s_29_6: u32 = 1600;
        // D s_29_7: read-reg s_29_6:[struct; 4]
        let s_29_7: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_29_6 as isize);
            tracer.read_register(s_29_6 as isize, value);
            value
        };
        // D s_29_8: mutate-element s_29_7[s_29_5] <= s_29_4
        let s_29_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_29_7.clone();
            local[(s_29_5) as usize] = s_29_4;
            local
        };
        // D s_29_9: cast cvt s_29_8 -> [struct; 0]
        let s_29_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_29_8,
        );
        // D s_29_10: cast cvt s_29_9 -> [struct; 4]
        let s_29_10: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_29_9);
            buf
        };
        // C s_29_11: const #1600u : u32
        let s_29_11: u32 = 1600;
        // N s_29_12: write-reg s_29_11 <= s_29_10
        let s_29_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_29_11 as isize, s_29_10);
            tracer.write_register(s_29_11 as isize, s_29_10);
        };
        // N s_29_13: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call Halted(s_30_0)
        let s_30_1: bool = Halted(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b35 b31
        if s_30_1 {
            return block_35(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#85928 <= s_31_0
        fn_state.gs_85928 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#85928:u8
        let s_32_0: bool = fn_state.gs_85928;
        // N s_32_1: branch s_32_0 b34 b33
        if s_32_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #24u : u8
        let s_33_0: u8 = 24;
        // C s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 8u16);
        // C s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (s_33_1.value() as i128);
        // C s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: cast zx s_33_3 -> i
        let s_33_4: i128 = (i128::try_from(s_33_3).unwrap());
        // C s_33_5: const #424u : u32
        let s_33_5: u32 = 424;
        // D s_33_6: read-reg s_33_5:u8
        let s_33_6: u8 = {
            let value = state.read_register::<u8>(s_33_5 as isize);
            tracer.read_register(s_33_5 as isize, value);
            value
        };
        // D s_33_7: call AArch64_SystemAccessTrap(s_33_6, s_33_4)
        let s_33_7: () = AArch64_SystemAccessTrap(state, tracer, s_33_6, s_33_4);
        // N s_33_8: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: panic
        panic!("{:?}", ());
        // N s_34_1: return
        return;
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
        // D s_35_5: write-var gs#85928 <= s_35_4
        fn_state.gs_85928 = s_35_4;
        // N s_35_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var __SCR_EL3_IRQ:u8
        let s_36_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 1u16);
        // C s_36_2: const #1u : u8
        let s_36_2: bool = true;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#85921 <= s_36_4
        fn_state.gs_85921 = s_36_4;
        // N s_36_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #24u : u8
        let s_37_0: u8 = 24;
        // C s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 8u16);
        // C s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (s_37_1.value() as i128);
        // C s_37_3: cast reint s_37_2 -> i64
        let s_37_3: i64 = (s_37_2 as i64);
        // C s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // C s_37_5: const #432u : u32
        let s_37_5: u32 = 432;
        // D s_37_6: read-reg s_37_5:u8
        let s_37_6: u8 = {
            let value = state.read_register::<u8>(s_37_5 as isize);
            tracer.read_register(s_37_5 as isize, value);
            value
        };
        // D s_37_7: call AArch64_SystemAccessTrap(s_37_6, s_37_4)
        let s_37_7: () = AArch64_SystemAccessTrap(state, tracer, s_37_6, s_37_4);
        // N s_37_8: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: panic
        panic!("{:?}", ());
        // N s_38_1: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __SCR_EL3_IRQ:u8
        let s_39_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#85920 <= s_39_4
        fn_state.gs_85920 = s_39_4;
        // N s_39_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_40_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_40_1: call __IMPDEF_boolean(s_40_0)
        let s_40_1: bool = u__IMPDEF_boolean(state, tracer, s_40_0);
        // D s_40_2: write-var gs#85919 <= s_40_1
        fn_state.gs_85919 = s_40_1;
        // N s_40_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var __EDSCR_SDD:u8
        let s_41_0: bool = fn_state.u__EDSCR_SDD;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 1u16);
        // C s_41_2: const #1u : u8
        let s_41_2: bool = true;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: write-var gs#85918 <= s_41_4
        fn_state.gs_85918 = s_41_4;
        // N s_41_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #424u : u32
        let s_42_0: u32 = 424;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // C s_42_2: const #2u : u8
        let s_42_2: u8 = 2;
        // D s_42_3: cmp-lt s_42_1 s_42_2
        let s_42_3: bool = ((s_42_1) < (s_42_2));
        // D s_42_4: write-var gs#85917 <= s_42_3
        fn_state.gs_85917 = s_42_3;
        // N s_42_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call Halted(s_43_0)
        let s_43_1: bool = Halted(state, tracer, s_43_0);
        // N s_43_2: branch s_43_1 b83 b44
        if s_43_1 {
            return block_83(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#85929 <= s_44_0
        fn_state.gs_85929 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#85929:u8
        let s_45_0: bool = fn_state.gs_85929;
        // N s_45_1: branch s_45_0 b82 b46
        if s_45_0 {
            return block_82(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#85930 <= s_46_0
        fn_state.gs_85930 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#85930:u8
        let s_47_0: bool = fn_state.gs_85930;
        // N s_47_1: branch s_47_0 b81 b48
        if s_47_0 {
            return block_81(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#85931 <= s_48_0
        fn_state.gs_85931 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#85931:u8
        let s_49_0: bool = fn_state.gs_85931;
        // N s_49_1: branch s_49_0 b80 b50
        if s_49_0 {
            return block_80(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#85932 <= s_50_0
        fn_state.gs_85932 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#85932:u8
        let s_51_0: bool = fn_state.gs_85932;
        // N s_51_1: branch s_51_0 b79 b52
        if s_51_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var __ICC_SRE_EL1_SRE:u8
        let s_52_0: bool = fn_state.u__ICC_SRE_EL1_SRE;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 1u16);
        // C s_52_2: const #0u : u8
        let s_52_2: bool = false;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // N s_52_5: branch s_52_4 b78 b53
        if s_52_4 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EL2Enabled(s_53_0)
        let s_53_1: bool = EL2Enabled(state, tracer, s_53_0);
        // N s_53_2: branch s_53_1 b77 b54
        if s_53_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#85933 <= s_54_0
        fn_state.gs_85933 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#85933:u8
        let s_55_0: bool = fn_state.gs_85933;
        // N s_55_1: branch s_55_0 b76 b56
        if s_55_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call EL2Enabled(s_56_0)
        let s_56_1: bool = EL2Enabled(state, tracer, s_56_0);
        // N s_56_2: branch s_56_1 b75 b57
        if s_56_1 {
            return block_75(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#85934 <= s_57_0
        fn_state.gs_85934 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#85934:u8
        let s_58_0: bool = fn_state.gs_85934;
        // N s_58_1: branch s_58_0 b74 b59
        if s_58_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #424u : u32
        let s_59_0: u32 = 424;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: u8 = {
            let value = state.read_register::<u8>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // C s_59_2: const #2u : u8
        let s_59_2: u8 = 2;
        // D s_59_3: cmp-lt s_59_1 s_59_2
        let s_59_3: bool = ((s_59_1) < (s_59_2));
        // N s_59_4: branch s_59_3 b73 b60
        if s_59_3 {
            return block_73(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#85935 <= s_60_0
        fn_state.gs_85935 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#85935:u8
        let s_61_0: bool = fn_state.gs_85935;
        // N s_61_1: branch s_61_0 b67 b62
        if s_61_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #424u : u32
        let s_62_0: u32 = 424;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // C s_62_2: const #2u : u8
        let s_62_2: u8 = 2;
        // D s_62_3: cmp-lt s_62_1 s_62_2
        let s_62_3: bool = ((s_62_1) < (s_62_2));
        // N s_62_4: branch s_62_3 b64 b63
        if s_62_3 {
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
        // D s_63_4: call Mk_ICC_AP1R_EL1_Type(s_63_3)
        let s_63_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_AP1R_EL1_Type(
            state,
            tracer,
            s_63_3,
        );
        // C s_63_5: const #0s : i
        let s_63_5: i128 = 0;
        // D s_63_6: call ICC_AP1R_EL1_set(s_63_5, s_63_4)
        let s_63_6: () = ICC_AP1R_EL1_set(state, tracer, s_63_5, s_63_4);
        // N s_63_7: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __SCR_EL3_NS:u8
        let s_64_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #0u : u8
        let s_64_2: bool = false;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // N s_64_5: branch s_64_4 b66 b65
        if s_64_4 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #64s : i64
        let s_65_0: i64 = 64;
        // D s_65_1: read-var t:i
        let s_65_1: i128 = fn_state.t;
        // D s_65_2: call X_read(s_65_1, s_65_0)
        let s_65_2: Bits = X_read(state, tracer, s_65_1, s_65_0);
        // D s_65_3: cast reint s_65_2 -> u64
        let s_65_3: u64 = (s_65_2.value() as u64);
        // D s_65_4: call Mk_ICC_AP1R_EL1_Type(s_65_3)
        let s_65_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_AP1R_EL1_Type(
            state,
            tracer,
            s_65_3,
        );
        // C s_65_5: const #0s : i
        let s_65_5: i128 = 0;
        // C s_65_6: const #11536u : u32
        let s_65_6: u32 = 11536;
        // D s_65_7: read-reg s_65_6:[struct; 4]
        let s_65_7: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_65_6 as isize);
            tracer.read_register(s_65_6 as isize, value);
            value
        };
        // D s_65_8: mutate-element s_65_7[s_65_5] <= s_65_4
        let s_65_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_65_7.clone();
            local[(s_65_5) as usize] = s_65_4;
            local
        };
        // D s_65_9: cast cvt s_65_8 -> [struct; 0]
        let s_65_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_65_8,
        );
        // D s_65_10: cast cvt s_65_9 -> [struct; 4]
        let s_65_10: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_65_9);
            buf
        };
        // C s_65_11: const #11536u : u32
        let s_65_11: u32 = 11536;
        // N s_65_12: write-reg s_65_11 <= s_65_10
        let s_65_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_65_11 as isize, s_65_10);
            tracer.write_register(s_65_11 as isize, s_65_10);
        };
        // N s_65_13: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #64s : i64
        let s_66_0: i64 = 64;
        // D s_66_1: read-var t:i
        let s_66_1: i128 = fn_state.t;
        // D s_66_2: call X_read(s_66_1, s_66_0)
        let s_66_2: Bits = X_read(state, tracer, s_66_1, s_66_0);
        // D s_66_3: cast reint s_66_2 -> u64
        let s_66_3: u64 = (s_66_2.value() as u64);
        // D s_66_4: call Mk_ICC_AP1R_EL1_Type(s_66_3)
        let s_66_4: ProductType5c790c8ef59cc8b2 = Mk_ICC_AP1R_EL1_Type(
            state,
            tracer,
            s_66_3,
        );
        // C s_66_5: const #0s : i
        let s_66_5: i128 = 0;
        // C s_66_6: const #1600u : u32
        let s_66_6: u32 = 1600;
        // D s_66_7: read-reg s_66_6:[struct; 4]
        let s_66_7: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_66_6 as isize);
            tracer.read_register(s_66_6 as isize, value);
            value
        };
        // D s_66_8: mutate-element s_66_7[s_66_5] <= s_66_4
        let s_66_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_66_7.clone();
            local[(s_66_5) as usize] = s_66_4;
            local
        };
        // D s_66_9: cast cvt s_66_8 -> [struct; 0]
        let s_66_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_66_8,
        );
        // D s_66_10: cast cvt s_66_9 -> [struct; 4]
        let s_66_10: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_66_9);
            buf
        };
        // C s_66_11: const #1600u : u32
        let s_66_11: u32 = 1600;
        // N s_66_12: write-reg s_66_11 <= s_66_10
        let s_66_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_66_11 as isize, s_66_10);
            tracer.write_register(s_66_11 as isize, s_66_10);
        };
        // N s_66_13: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call Halted(s_67_0)
        let s_67_1: bool = Halted(state, tracer, s_67_0);
        // N s_67_2: branch s_67_1 b72 b68
        if s_67_1 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#85942 <= s_68_0
        fn_state.gs_85942 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#85942:u8
        let s_69_0: bool = fn_state.gs_85942;
        // N s_69_1: branch s_69_0 b71 b70
        if s_69_0 {
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
        // C s_70_0: const #24u : u8
        let s_70_0: u8 = 24;
        // C s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 8u16);
        // C s_70_2: cast zx s_70_1 -> i
        let s_70_2: i128 = (s_70_1.value() as i128);
        // C s_70_3: cast reint s_70_2 -> i64
        let s_70_3: i64 = (s_70_2 as i64);
        // C s_70_4: cast zx s_70_3 -> i
        let s_70_4: i128 = (i128::try_from(s_70_3).unwrap());
        // C s_70_5: const #424u : u32
        let s_70_5: u32 = 424;
        // D s_70_6: read-reg s_70_5:u8
        let s_70_6: u8 = {
            let value = state.read_register::<u8>(s_70_5 as isize);
            tracer.read_register(s_70_5 as isize, value);
            value
        };
        // D s_70_7: call AArch64_SystemAccessTrap(s_70_6, s_70_4)
        let s_70_7: () = AArch64_SystemAccessTrap(state, tracer, s_70_6, s_70_4);
        // N s_70_8: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_71_0: panic
        panic!("{:?}", ());
        // N s_71_1: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __EDSCR_SDD:u8
        let s_72_0: bool = fn_state.u__EDSCR_SDD;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #1u : u8
        let s_72_2: bool = true;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#85942 <= s_72_4
        fn_state.gs_85942 = s_72_4;
        // N s_72_6: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __SCR_EL3_IRQ:u8
        let s_73_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#85935 <= s_73_4
        fn_state.gs_85935 = s_73_4;
        // N s_73_6: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #64s : i64
        let s_74_0: i64 = 64;
        // D s_74_1: read-var t:i
        let s_74_1: i128 = fn_state.t;
        // D s_74_2: call X_read(s_74_1, s_74_0)
        let s_74_2: Bits = X_read(state, tracer, s_74_1, s_74_0);
        // D s_74_3: cast reint s_74_2 -> u64
        let s_74_3: u64 = (s_74_2.value() as u64);
        // D s_74_4: call Mk_ICV_AP1R_EL1_Type(s_74_3)
        let s_74_4: ProductType5c790c8ef59cc8b2 = Mk_ICV_AP1R_EL1_Type(
            state,
            tracer,
            s_74_3,
        );
        // C s_74_5: const #0s : i
        let s_74_5: i128 = 0;
        // C s_74_6: const #20744u : u32
        let s_74_6: u32 = 20744;
        // D s_74_7: read-reg s_74_6:[struct; 4]
        let s_74_7: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_74_6 as isize);
            tracer.read_register(s_74_6 as isize, value);
            value
        };
        // D s_74_8: mutate-element s_74_7[s_74_5] <= s_74_4
        let s_74_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_74_7.clone();
            local[(s_74_5) as usize] = s_74_4;
            local
        };
        // D s_74_9: cast cvt s_74_8 -> [struct; 0]
        let s_74_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_74_8,
        );
        // D s_74_10: cast cvt s_74_9 -> [struct; 4]
        let s_74_10: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_74_9);
            buf
        };
        // C s_74_11: const #20744u : u32
        let s_74_11: u32 = 20744;
        // N s_74_12: write-reg s_74_11 <= s_74_10
        let s_74_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_74_11 as isize, s_74_10);
            tracer.write_register(s_74_11 as isize, s_74_10);
        };
        // N s_74_13: return
        return;
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var __HCR_EL2_IMO:u8
        let s_75_0: bool = fn_state.u__HCR_EL2_IMO;
        // D s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 1u16);
        // C s_75_2: const #1u : u8
        let s_75_2: bool = true;
        // C s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_4: cmp-eq s_75_1 s_75_3
        let s_75_4: bool = ((s_75_1) == (s_75_3));
        // D s_75_5: write-var gs#85934 <= s_75_4
        fn_state.gs_85934 = s_75_4;
        // N s_75_6: jump b58
        return block_58(state, tracer, fn_state);
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
        // C s_76_5: const #432u : u32
        let s_76_5: u32 = 432;
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
        // D s_77_0: read-var __ICH_HCR_EL2_TALL1:u8
        let s_77_0: bool = fn_state.u__ICH_HCR_EL2_TALL1;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#85933 <= s_77_4
        fn_state.gs_85933 = s_77_4;
        // N s_77_6: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #24u : u8
        let s_78_0: u8 = 24;
        // C s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 8u16);
        // C s_78_2: cast zx s_78_1 -> i
        let s_78_2: i128 = (s_78_1.value() as i128);
        // C s_78_3: cast reint s_78_2 -> i64
        let s_78_3: i64 = (s_78_2 as i64);
        // C s_78_4: cast zx s_78_3 -> i
        let s_78_4: i128 = (i128::try_from(s_78_3).unwrap());
        // C s_78_5: const #440u : u32
        let s_78_5: u32 = 440;
        // D s_78_6: read-reg s_78_5:u8
        let s_78_6: u8 = {
            let value = state.read_register::<u8>(s_78_5 as isize);
            tracer.read_register(s_78_5 as isize, value);
            value
        };
        // D s_78_7: call AArch64_SystemAccessTrap(s_78_6, s_78_4)
        let s_78_7: () = AArch64_SystemAccessTrap(state, tracer, s_78_6, s_78_4);
        // N s_78_8: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_79_0: panic
        panic!("{:?}", ());
        // N s_79_1: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __SCR_EL3_IRQ:u8
        let s_80_0: bool = fn_state.u__SCR_EL3_IRQ;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#85932 <= s_80_4
        fn_state.gs_85932 = s_80_4;
        // N s_80_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_81_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_81_1: call __IMPDEF_boolean(s_81_0)
        let s_81_1: bool = u__IMPDEF_boolean(state, tracer, s_81_0);
        // D s_81_2: write-var gs#85931 <= s_81_1
        fn_state.gs_85931 = s_81_1;
        // N s_81_3: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var __EDSCR_SDD:u8
        let s_82_0: bool = fn_state.u__EDSCR_SDD;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 1u16);
        // C s_82_2: const #1u : u8
        let s_82_2: bool = true;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: write-var gs#85930 <= s_82_4
        fn_state.gs_85930 = s_82_4;
        // N s_82_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #424u : u32
        let s_83_0: u32 = 424;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // C s_83_2: const #2u : u8
        let s_83_2: u8 = 2;
        // D s_83_3: cmp-lt s_83_1 s_83_2
        let s_83_3: bool = ((s_83_1) < (s_83_2));
        // D s_83_4: write-var gs#85929 <= s_83_3
        fn_state.gs_85929 = s_83_3;
        // N s_83_5: jump b45
        return block_45(state, tracer, fn_state);
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
