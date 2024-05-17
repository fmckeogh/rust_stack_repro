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
use neq_int::*;
use u_get_CPTR_EL3_Type_TAM::*;
use NVMem_set::*;
use u_get_HCR_EL2_Type_NV2::*;
use u_get_HCR_EL2_Type_NV::*;
use Halted::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_SCR_EL3_Type_AMVOFFEN::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use EDSCR_read::*;
use EL2Enabled::*;
use common::*;
pub fn AMEVCNTVOFF0_EL2_SysRegWrite_f6918a05ffc46bd6<T: Tracer>(
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
        u__EDSCR_SDD: bool,
        gs_80418: bool,
        gs_80423: bool,
        gs_80424: bool,
        gs_80422: bool,
        gs_80415: bool,
        u__CPTR_EL3_TAM: bool,
        gs_80416: bool,
        gs_80429: bool,
        gs_80428: bool,
        gs_80419: bool,
        gs_80421: bool,
        gs_80409: bool,
        u__SCR_EL3_AMVOFFEN: bool,
        gs_80430: bool,
        gs_80420: bool,
        u__PSTATE_EL: u8,
        gs_80427: bool,
        gs_80417: bool,
        u__HCR_EL2_NV: bool,
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
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_NV(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_NV <= s_0_5
        fn_state.u__HCR_EL2_NV = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call EDSCR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_EDSCR_Type_SDD(s_0_8)
        let s_0_9: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_8);
        // D s_0_10: write-var __EDSCR_SDD <= s_0_9
        fn_state.u__EDSCR_SDD = s_0_9;
        // C s_0_11: const #16840u : u32
        let s_0_11: u32 = 16840;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CPTR_EL3_Type_TAM(s_0_12)
        let s_0_13: bool = u_get_CPTR_EL3_Type_TAM(state, tracer, s_0_12);
        // D s_0_14: write-var __CPTR_EL3_TAM <= s_0_13
        fn_state.u__CPTR_EL3_TAM = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_AMVOFFEN(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_AMVOFFEN(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_AMVOFFEN <= s_0_17
        fn_state.u__SCR_EL3_AMVOFFEN = s_0_17;
        // C s_0_19: const #0s : i
        let s_0_19: i128 = 0;
        // C s_0_20: const #0s : i
        let s_0_20: i128 = 0;
        // S s_0_21: call neq_int(s_0_19, s_0_20)
        let s_0_21: bool = neq_int(state, tracer, s_0_19, s_0_20);
        // N s_0_22: branch s_0_21 b72 b1
        if s_0_21 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#80409 <= s_1_0
        fn_state.gs_80409 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#80409:u8
        let s_2_0: bool = fn_state.gs_80409;
        // N s_2_1: branch s_2_0 b71 b3
        if s_2_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_4_2: const #448u : u32
        let s_4_2: u32 = 448;
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
        // N s_4_6: branch s_4_5 b70 b5
        if s_4_5 {
            return block_70(state, tracer, fn_state);
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
        // C s_5_2: const #440u : u32
        let s_5_2: u32 = 440;
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
        // N s_5_6: branch s_5_5 b59 b6
        if s_5_5 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __PSTATE_EL:u8
        let s_6_0: u8 = fn_state.u__PSTATE_EL;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #432u : u32
        let s_6_2: u32 = 432;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b10 b7
        if s_6_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var __PSTATE_EL:u8
        let s_7_0: u8 = fn_state.u__PSTATE_EL;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #424u : u32
        let s_7_2: u32 = 424;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
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
        // C s_9_4: const #0s : i
        let s_9_4: i128 = 0;
        // C s_9_5: const #17400u : u32
        let s_9_5: u32 = 17400;
        // D s_9_6: read-reg s_9_5:[u64; 16]
        let s_9_6: [u64; 16usize] = {
            let value = state.read_register::<[u64; 16usize]>(s_9_5 as isize);
            tracer.read_register(s_9_5 as isize, value);
            value
        };
        // D s_9_7: mutate-element s_9_6[s_9_4] <= s_9_3
        let s_9_7: [u64; 16usize] = {
            let mut local = s_9_6.clone();
            local[(s_9_4) as usize] = s_9_3;
            local
        };
        // D s_9_8: cast cvt s_9_7 -> [u64; 0]
        let s_9_8: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_9_7);
        // D s_9_9: cast cvt s_9_8 -> [u64; 16]
        let s_9_9: [u64; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_9_8);
            buf
        };
        // C s_9_10: const #17400u : u32
        let s_9_10: u32 = 17400;
        // N s_9_11: write-reg s_9_10 <= s_9_9
        let s_9_11: () = {
            state.write_register::<[u64; 16usize]>(s_9_10 as isize, s_9_9);
            tracer.write_register(s_9_10 as isize, s_9_9);
        };
        // N s_9_12: return
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
        // N s_10_2: branch s_10_1 b58 b11
        if s_10_1 {
            return block_58(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#80415 <= s_11_0
        fn_state.gs_80415 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#80415:u8
        let s_12_0: bool = fn_state.gs_80415;
        // N s_12_1: branch s_12_0 b57 b13
        if s_12_0 {
            return block_57(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#80416 <= s_13_0
        fn_state.gs_80416 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#80416:u8
        let s_14_0: bool = fn_state.gs_80416;
        // N s_14_1: branch s_14_0 b56 b15
        if s_14_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#80417 <= s_15_0
        fn_state.gs_80417 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#80417:u8
        let s_16_0: bool = fn_state.gs_80417;
        // N s_16_1: branch s_16_0 b55 b17
        if s_16_0 {
            return block_55(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#80418 <= s_17_0
        fn_state.gs_80418 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#80418:u8
        let s_18_0: bool = fn_state.gs_80418;
        // N s_18_1: branch s_18_0 b54 b19
        if s_18_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
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
        // N s_19_2: branch s_19_1 b53 b20
        if s_19_1 {
            return block_53(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#80419 <= s_20_0
        fn_state.gs_80419 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#80419:u8
        let s_21_0: bool = fn_state.gs_80419;
        // N s_21_1: branch s_21_0 b52 b22
        if s_21_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#80420 <= s_22_0
        fn_state.gs_80420 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#80420:u8
        let s_23_0: bool = fn_state.gs_80420;
        // N s_23_1: branch s_23_0 b51 b24
        if s_23_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#80421 <= s_24_0
        fn_state.gs_80421 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#80421:u8
        let s_25_0: bool = fn_state.gs_80421;
        // N s_25_1: branch s_25_0 b50 b26
        if s_25_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#80422 <= s_26_0
        fn_state.gs_80422 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#80422:u8
        let s_27_0: bool = fn_state.gs_80422;
        // N s_27_1: branch s_27_0 b49 b28
        if s_27_0 {
            return block_49(state, tracer, fn_state);
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
        // N s_28_4: branch s_28_3 b48 b29
        if s_28_3 {
            return block_48(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#80423 <= s_29_0
        fn_state.gs_80423 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#80423:u8
        let s_30_0: bool = fn_state.gs_80423;
        // N s_30_1: branch s_30_0 b42 b31
        if s_30_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #424u : u32
        let s_31_0: u32 = 424;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // C s_31_2: const #2u : u8
        let s_31_2: u8 = 2;
        // D s_31_3: cmp-lt s_31_1 s_31_2
        let s_31_3: bool = ((s_31_1) < (s_31_2));
        // N s_31_4: branch s_31_3 b41 b32
        if s_31_3 {
            return block_41(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#80424 <= s_32_0
        fn_state.gs_80424 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#80424:u8
        let s_33_0: bool = fn_state.gs_80424;
        // N s_33_1: branch s_33_0 b35 b34
        if s_33_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #64s : i64
        let s_34_0: i64 = 64;
        // D s_34_1: read-var t:i
        let s_34_1: i128 = fn_state.t;
        // D s_34_2: call X_read(s_34_1, s_34_0)
        let s_34_2: Bits = X_read(state, tracer, s_34_1, s_34_0);
        // D s_34_3: cast reint s_34_2 -> u64
        let s_34_3: u64 = (s_34_2.value() as u64);
        // C s_34_4: const #0s : i
        let s_34_4: i128 = 0;
        // C s_34_5: const #17400u : u32
        let s_34_5: u32 = 17400;
        // D s_34_6: read-reg s_34_5:[u64; 16]
        let s_34_6: [u64; 16usize] = {
            let value = state.read_register::<[u64; 16usize]>(s_34_5 as isize);
            tracer.read_register(s_34_5 as isize, value);
            value
        };
        // D s_34_7: mutate-element s_34_6[s_34_4] <= s_34_3
        let s_34_7: [u64; 16usize] = {
            let mut local = s_34_6.clone();
            local[(s_34_4) as usize] = s_34_3;
            local
        };
        // D s_34_8: cast cvt s_34_7 -> [u64; 0]
        let s_34_8: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_34_7);
        // D s_34_9: cast cvt s_34_8 -> [u64; 16]
        let s_34_9: [u64; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_34_8);
            buf
        };
        // C s_34_10: const #17400u : u32
        let s_34_10: u32 = 17400;
        // N s_34_11: write-reg s_34_10 <= s_34_9
        let s_34_11: () = {
            state.write_register::<[u64; 16usize]>(s_34_10 as isize, s_34_9);
            tracer.write_register(s_34_10 as isize, s_34_9);
        };
        // N s_34_12: return
        return;
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
        // N s_35_2: branch s_35_1 b40 b36
        if s_35_1 {
            return block_40(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#80427 <= s_36_0
        fn_state.gs_80427 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#80427:u8
        let s_37_0: bool = fn_state.gs_80427;
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
        // C s_38_0: const #24u : u8
        let s_38_0: u8 = 24;
        // C s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 8u16);
        // C s_38_2: cast zx s_38_1 -> i
        let s_38_2: i128 = (s_38_1.value() as i128);
        // C s_38_3: cast reint s_38_2 -> i64
        let s_38_3: i64 = (s_38_2 as i64);
        // C s_38_4: cast zx s_38_3 -> i
        let s_38_4: i128 = (i128::try_from(s_38_3).unwrap());
        // C s_38_5: const #424u : u32
        let s_38_5: u32 = 424;
        // D s_38_6: read-reg s_38_5:u8
        let s_38_6: u8 = {
            let value = state.read_register::<u8>(s_38_5 as isize);
            tracer.read_register(s_38_5 as isize, value);
            value
        };
        // D s_38_7: call AArch64_SystemAccessTrap(s_38_6, s_38_4)
        let s_38_7: () = AArch64_SystemAccessTrap(state, tracer, s_38_6, s_38_4);
        // N s_38_8: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: panic
        panic!("{:?}", ());
        // N s_39_1: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __EDSCR_SDD:u8
        let s_40_0: bool = fn_state.u__EDSCR_SDD;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #1u : u8
        let s_40_2: bool = true;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#80427 <= s_40_4
        fn_state.gs_80427 = s_40_4;
        // N s_40_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var __CPTR_EL3_TAM:u8
        let s_41_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 1u16);
        // C s_41_2: const #1u : u8
        let s_41_2: bool = true;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: write-var gs#80424 <= s_41_4
        fn_state.gs_80424 = s_41_4;
        // N s_41_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call Halted(s_42_0)
        let s_42_1: bool = Halted(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b47 b43
        if s_42_1 {
            return block_47(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#80428 <= s_43_0
        fn_state.gs_80428 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#80428:u8
        let s_44_0: bool = fn_state.gs_80428;
        // N s_44_1: branch s_44_0 b46 b45
        if s_44_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #24u : u8
        let s_45_0: u8 = 24;
        // C s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 8u16);
        // C s_45_2: cast zx s_45_1 -> i
        let s_45_2: i128 = (s_45_1.value() as i128);
        // C s_45_3: cast reint s_45_2 -> i64
        let s_45_3: i64 = (s_45_2 as i64);
        // C s_45_4: cast zx s_45_3 -> i
        let s_45_4: i128 = (i128::try_from(s_45_3).unwrap());
        // C s_45_5: const #424u : u32
        let s_45_5: u32 = 424;
        // D s_45_6: read-reg s_45_5:u8
        let s_45_6: u8 = {
            let value = state.read_register::<u8>(s_45_5 as isize);
            tracer.read_register(s_45_5 as isize, value);
            value
        };
        // D s_45_7: call AArch64_SystemAccessTrap(s_45_6, s_45_4)
        let s_45_7: () = AArch64_SystemAccessTrap(state, tracer, s_45_6, s_45_4);
        // N s_45_8: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: panic
        panic!("{:?}", ());
        // N s_46_1: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var __EDSCR_SDD:u8
        let s_47_0: bool = fn_state.u__EDSCR_SDD;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: write-var gs#80428 <= s_47_4
        fn_state.gs_80428 = s_47_4;
        // N s_47_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __SCR_EL3_AMVOFFEN:u8
        let s_48_0: bool = fn_state.u__SCR_EL3_AMVOFFEN;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #0u : u8
        let s_48_2: bool = false;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#80423 <= s_48_4
        fn_state.gs_80423 = s_48_4;
        // N s_48_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_49_0: panic
        panic!("{:?}", ());
        // N s_49_1: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var __SCR_EL3_AMVOFFEN:u8
        let s_50_0: bool = fn_state.u__SCR_EL3_AMVOFFEN;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 1u16);
        // C s_50_2: const #0u : u8
        let s_50_2: bool = false;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_4: cmp-eq s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) == (s_50_3));
        // D s_50_5: write-var gs#80422 <= s_50_4
        fn_state.gs_80422 = s_50_4;
        // N s_50_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_51_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_51_1: call __IMPDEF_boolean(s_51_0)
        let s_51_1: bool = u__IMPDEF_boolean(state, tracer, s_51_0);
        // D s_51_2: write-var gs#80421 <= s_51_1
        fn_state.gs_80421 = s_51_1;
        // N s_51_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var __EDSCR_SDD:u8
        let s_52_0: bool = fn_state.u__EDSCR_SDD;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 1u16);
        // C s_52_2: const #1u : u8
        let s_52_2: bool = true;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#80420 <= s_52_4
        fn_state.gs_80420 = s_52_4;
        // N s_52_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #424u : u32
        let s_53_0: u32 = 424;
        // D s_53_1: read-reg s_53_0:u8
        let s_53_1: u8 = {
            let value = state.read_register::<u8>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // C s_53_2: const #2u : u8
        let s_53_2: u8 = 2;
        // D s_53_3: cmp-lt s_53_1 s_53_2
        let s_53_3: bool = ((s_53_1) < (s_53_2));
        // D s_53_4: write-var gs#80419 <= s_53_3
        fn_state.gs_80419 = s_53_3;
        // N s_53_5: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_55_0: read-var __CPTR_EL3_TAM:u8
        let s_55_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #1u : u8
        let s_55_2: bool = true;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#80418 <= s_55_4
        fn_state.gs_80418 = s_55_4;
        // N s_55_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_56_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_56_1: call __IMPDEF_boolean(s_56_0)
        let s_56_1: bool = u__IMPDEF_boolean(state, tracer, s_56_0);
        // D s_56_2: write-var gs#80417 <= s_56_1
        fn_state.gs_80417 = s_56_1;
        // N s_56_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var __EDSCR_SDD:u8
        let s_57_0: bool = fn_state.u__EDSCR_SDD;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 1u16);
        // C s_57_2: const #1u : u8
        let s_57_2: bool = true;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // D s_57_4: cmp-eq s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) == (s_57_3));
        // D s_57_5: write-var gs#80416 <= s_57_4
        fn_state.gs_80416 = s_57_4;
        // N s_57_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #424u : u32
        let s_58_0: u32 = 424;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // C s_58_2: const #2u : u8
        let s_58_2: u8 = 2;
        // D s_58_3: cmp-lt s_58_1 s_58_2
        let s_58_3: bool = ((s_58_1) < (s_58_2));
        // D s_58_4: write-var gs#80415 <= s_58_3
        fn_state.gs_80415 = s_58_3;
        // N s_58_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call EL2Enabled(s_59_0)
        let s_59_1: bool = EL2Enabled(state, tracer, s_59_0);
        // N s_59_2: branch s_59_1 b69 b60
        if s_59_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#80429 <= s_60_0
        fn_state.gs_80429 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#80429:u8
        let s_61_0: bool = fn_state.gs_80429;
        // N s_61_1: branch s_61_0 b68 b62
        if s_61_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call EL2Enabled(s_62_0)
        let s_62_1: bool = EL2Enabled(state, tracer, s_62_0);
        // N s_62_2: branch s_62_1 b67 b63
        if s_62_1 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#80430 <= s_63_0
        fn_state.gs_80430 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#80430:u8
        let s_64_0: bool = fn_state.gs_80430;
        // N s_64_1: branch s_64_0 b66 b65
        if s_64_0 {
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
        // C s_66_0: const #24u : u8
        let s_66_0: u8 = 24;
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
        // D s_67_0: read-var __HCR_EL2_NV:u8
        let s_67_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#80430 <= s_67_4
        fn_state.gs_80430 = s_67_4;
        // N s_67_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #2560u : u12
        let s_68_0: u16 = 2560;
        // C s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 12u16);
        // C s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (s_68_1.value() as i128);
        // C s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: const #0s : i64
        let s_68_4: i64 = 0;
        // C s_68_5: cast zx s_68_3 -> i
        let s_68_5: i128 = (i128::try_from(s_68_3).unwrap());
        // C s_68_6: cast zx s_68_4 -> i
        let s_68_6: i128 = (i128::try_from(s_68_4).unwrap());
        // C s_68_7: add s_68_5 s_68_6
        let s_68_7: i128 = (s_68_5 + s_68_6);
        // C s_68_8: cast reint s_68_7 -> i64
        let s_68_8: i64 = (s_68_7 as i64);
        // C s_68_9: const #64s : i64
        let s_68_9: i64 = 64;
        // D s_68_10: read-var t:i
        let s_68_10: i128 = fn_state.t;
        // D s_68_11: call X_read(s_68_10, s_68_9)
        let s_68_11: Bits = X_read(state, tracer, s_68_10, s_68_9);
        // D s_68_12: cast reint s_68_11 -> u64
        let s_68_12: u64 = (s_68_11.value() as u64);
        // C s_68_13: cast zx s_68_8 -> i
        let s_68_13: i128 = (i128::try_from(s_68_8).unwrap());
        // D s_68_14: call NVMem_set(s_68_13, s_68_12)
        let s_68_14: () = NVMem_set(state, tracer, s_68_13, s_68_12);
        // N s_68_15: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #102552u : u32
        let s_69_0: u32 = 102552;
        // D s_69_1: read-reg s_69_0:struct
        let s_69_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call _get_HCR_EL2_Type_NV2(s_69_1)
        let s_69_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_69_1);
        // C s_69_3: const #102552u : u32
        let s_69_3: u32 = 102552;
        // D s_69_4: read-reg s_69_3:struct
        let s_69_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_3 as isize);
            tracer.read_register(s_69_3 as isize, value);
            value
        };
        // D s_69_5: call _get_HCR_EL2_Type_NV(s_69_4)
        let s_69_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_69_4);
        // D s_69_6: cast zx s_69_2 -> bv
        let s_69_6: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_7: cast zx s_69_5 -> bv
        let s_69_7: Bits = Bits::new(s_69_5 as u128, 1u16);
        // D s_69_8: cast reint s_69_6 -> u128
        let s_69_8: u128 = (s_69_6.value() as u128);
        // D s_69_9: size-of s_69_6
        let s_69_9: u16 = s_69_6.length();
        // D s_69_10: cast reint s_69_7 -> u128
        let s_69_10: u128 = (s_69_7.value() as u128);
        // D s_69_11: size-of s_69_7
        let s_69_11: u16 = s_69_7.length();
        // D s_69_12: lsl s_69_8 s_69_11
        let s_69_12: u128 = s_69_8 << s_69_11;
        // D s_69_13: or s_69_12 s_69_10
        let s_69_13: u128 = ((s_69_12) | (s_69_10));
        // D s_69_14: add s_69_9 s_69_11
        let s_69_14: u16 = (s_69_9 + s_69_11);
        // D s_69_15: create-bits s_69_13 s_69_14
        let s_69_15: Bits = Bits::new(s_69_13, s_69_14);
        // D s_69_16: cast reint s_69_15 -> u8
        let s_69_16: u8 = (s_69_15.value() as u8);
        // D s_69_17: cast zx s_69_16 -> bv
        let s_69_17: Bits = Bits::new(s_69_16 as u128, 2u16);
        // C s_69_18: const #3u : u8
        let s_69_18: u8 = 3;
        // C s_69_19: cast zx s_69_18 -> bv
        let s_69_19: Bits = Bits::new(s_69_18 as u128, 2u16);
        // D s_69_20: cmp-eq s_69_17 s_69_19
        let s_69_20: bool = ((s_69_17) == (s_69_19));
        // D s_69_21: write-var gs#80429 <= s_69_20
        fn_state.gs_80429 = s_69_20;
        // N s_69_22: jump b61
        return block_61(state, tracer, fn_state);
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
        // C s_71_0: const #0s : i
        let s_71_0: i128 = 0;
        // C s_71_1: const #3s : i
        let s_71_1: i128 = 3;
        // S s_71_2: call neq_int(s_71_0, s_71_1)
        let s_71_2: bool = neq_int(state, tracer, s_71_0, s_71_1);
        // N s_71_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0s : i
        let s_72_0: i128 = 0;
        // C s_72_1: const #2s : i
        let s_72_1: i128 = 2;
        // S s_72_2: call neq_int(s_72_0, s_72_1)
        let s_72_2: bool = neq_int(state, tracer, s_72_0, s_72_1);
        // D s_72_3: write-var gs#80409 <= s_72_2
        fn_state.gs_80409 = s_72_2;
        // N s_72_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
