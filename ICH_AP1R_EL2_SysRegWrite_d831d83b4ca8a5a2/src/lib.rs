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
use u_get_HCR_EL2_Type_NV::*;
use u_get_ICC_SRE_EL2_Type_SRE::*;
use Mk_ICH_AP1R_EL2_Type::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use EL2Enabled::*;
use u_get_ICC_SRE_EL3_Type_SRE::*;
use NVMem_set::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn ICH_AP1R_EL2_SysRegWrite_d831d83b4ca8a5a2<T: Tracer>(
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
        u__ICC_SRE_EL2_SRE: bool,
        gs_86231: bool,
        u__ICC_SRE_EL3_SRE: bool,
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
        gs_86232: bool,
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
        // C s_0_7: const #16368u : u32
        let s_0_7: u32 = 16368;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_ICC_SRE_EL2_Type_SRE(s_0_8)
        let s_0_9: bool = u_get_ICC_SRE_EL2_Type_SRE(state, tracer, s_0_8);
        // D s_0_10: write-var __ICC_SRE_EL2_SRE <= s_0_9
        fn_state.u__ICC_SRE_EL2_SRE = s_0_9;
        // C s_0_11: const #10200u : u32
        let s_0_11: u32 = 10200;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_ICC_SRE_EL3_Type_SRE(s_0_12)
        let s_0_13: bool = u_get_ICC_SRE_EL3_Type_SRE(state, tracer, s_0_12);
        // D s_0_14: write-var __ICC_SRE_EL3_SRE <= s_0_13
        fn_state.u__ICC_SRE_EL3_SRE = s_0_13;
        // N s_0_15: jump b1
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
        // N s_2_6: branch s_2_5 b24 b3
        if s_2_5 {
            return block_24(state, tracer, fn_state);
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
        // N s_3_6: branch s_3_5 b13 b4
        if s_3_5 {
            return block_13(state, tracer, fn_state);
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
        // N s_4_6: branch s_4_5 b10 b5
        if s_4_5 {
            return block_10(state, tracer, fn_state);
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
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
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
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // D s_8_1: read-var t:i
        let s_8_1: i128 = fn_state.t;
        // D s_8_2: call X_read(s_8_1, s_8_0)
        let s_8_2: Bits = X_read(state, tracer, s_8_1, s_8_0);
        // D s_8_3: cast reint s_8_2 -> u64
        let s_8_3: u64 = (s_8_2.value() as u64);
        // D s_8_4: call Mk_ICH_AP1R_EL2_Type(s_8_3)
        let s_8_4: ProductType5c790c8ef59cc8b2 = Mk_ICH_AP1R_EL2_Type(
            state,
            tracer,
            s_8_3,
        );
        // C s_8_5: const #0s : i
        let s_8_5: i128 = 0;
        // C s_8_6: const #19904u : u32
        let s_8_6: u32 = 19904;
        // D s_8_7: read-reg s_8_6:[struct; 4]
        let s_8_7: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_8_6 as isize);
            tracer.read_register(s_8_6 as isize, value);
            value
        };
        // D s_8_8: mutate-element s_8_7[s_8_5] <= s_8_4
        let s_8_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_8_7.clone();
            local[(s_8_5) as usize] = s_8_4;
            local
        };
        // D s_8_9: cast cvt s_8_8 -> [struct; 0]
        let s_8_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_8_8,
        );
        // D s_8_10: cast cvt s_8_9 -> [struct; 4]
        let s_8_10: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_8_9);
            buf
        };
        // C s_8_11: const #19904u : u32
        let s_8_11: u32 = 19904;
        // N s_8_12: write-reg s_8_11 <= s_8_10
        let s_8_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_8_11 as isize, s_8_10);
            tracer.write_register(s_8_11 as isize, s_8_10);
        };
        // N s_8_13: return
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
        // D s_10_0: read-var __ICC_SRE_EL2_SRE:u8
        let s_10_0: bool = fn_state.u__ICC_SRE_EL2_SRE;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // D s_11_1: read-var t:i
        let s_11_1: i128 = fn_state.t;
        // D s_11_2: call X_read(s_11_1, s_11_0)
        let s_11_2: Bits = X_read(state, tracer, s_11_1, s_11_0);
        // D s_11_3: cast reint s_11_2 -> u64
        let s_11_3: u64 = (s_11_2.value() as u64);
        // D s_11_4: call Mk_ICH_AP1R_EL2_Type(s_11_3)
        let s_11_4: ProductType5c790c8ef59cc8b2 = Mk_ICH_AP1R_EL2_Type(
            state,
            tracer,
            s_11_3,
        );
        // C s_11_5: const #0s : i
        let s_11_5: i128 = 0;
        // C s_11_6: const #19904u : u32
        let s_11_6: u32 = 19904;
        // D s_11_7: read-reg s_11_6:[struct; 4]
        let s_11_7: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_11_6 as isize);
            tracer.read_register(s_11_6 as isize, value);
            value
        };
        // D s_11_8: mutate-element s_11_7[s_11_5] <= s_11_4
        let s_11_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_11_7.clone();
            local[(s_11_5) as usize] = s_11_4;
            local
        };
        // D s_11_9: cast cvt s_11_8 -> [struct; 0]
        let s_11_9: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_11_8,
        );
        // D s_11_10: cast cvt s_11_9 -> [struct; 4]
        let s_11_10: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_11_9);
            buf
        };
        // C s_11_11: const #19904u : u32
        let s_11_11: u32 = 19904;
        // N s_11_12: write-reg s_11_11 <= s_11_10
        let s_11_12: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_11_11 as isize, s_11_10);
            tracer.write_register(s_11_11 as isize, s_11_10);
        };
        // N s_11_13: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #24u : u8
        let s_12_0: u8 = 24;
        // C s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 8u16);
        // C s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (s_12_1.value() as i128);
        // C s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // C s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // C s_12_5: const #432u : u32
        let s_12_5: u32 = 432;
        // D s_12_6: read-reg s_12_5:u8
        let s_12_6: u8 = {
            let value = state.read_register::<u8>(s_12_5 as isize);
            tracer.read_register(s_12_5 as isize, value);
            value
        };
        // D s_12_7: call AArch64_SystemAccessTrap(s_12_6, s_12_4)
        let s_12_7: () = AArch64_SystemAccessTrap(state, tracer, s_12_6, s_12_4);
        // N s_12_8: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call EL2Enabled(s_13_0)
        let s_13_1: bool = EL2Enabled(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b23 b14
        if s_13_1 {
            return block_23(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#86231 <= s_14_0
        fn_state.gs_86231 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#86231:u8
        let s_15_0: bool = fn_state.gs_86231;
        // N s_15_1: branch s_15_0 b22 b16
        if s_15_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call EL2Enabled(s_16_0)
        let s_16_1: bool = EL2Enabled(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b21 b17
        if s_16_1 {
            return block_21(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#86232 <= s_17_0
        fn_state.gs_86232 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#86232:u8
        let s_18_0: bool = fn_state.gs_86232;
        // N s_18_1: branch s_18_0 b20 b19
        if s_18_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #24u : u8
        let s_20_0: u8 = 24;
        // C s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 8u16);
        // C s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (s_20_1.value() as i128);
        // C s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #432u : u32
        let s_20_5: u32 = 432;
        // D s_20_6: read-reg s_20_5:u8
        let s_20_6: u8 = {
            let value = state.read_register::<u8>(s_20_5 as isize);
            tracer.read_register(s_20_5 as isize, value);
            value
        };
        // D s_20_7: call AArch64_SystemAccessTrap(s_20_6, s_20_4)
        let s_20_7: () = AArch64_SystemAccessTrap(state, tracer, s_20_6, s_20_4);
        // N s_20_8: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var __HCR_EL2_NV:u8
        let s_21_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 1u16);
        // C s_21_2: const #1u : u8
        let s_21_2: bool = true;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: write-var gs#86232 <= s_21_4
        fn_state.gs_86232 = s_21_4;
        // N s_21_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1184u : u12
        let s_22_0: u16 = 1184;
        // C s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 12u16);
        // C s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (s_22_1.value() as i128);
        // C s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: const #0s : i64
        let s_22_4: i64 = 0;
        // C s_22_5: cast zx s_22_3 -> i
        let s_22_5: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_6: cast zx s_22_4 -> i
        let s_22_6: i128 = (i128::try_from(s_22_4).unwrap());
        // C s_22_7: add s_22_5 s_22_6
        let s_22_7: i128 = (s_22_5 + s_22_6);
        // C s_22_8: cast reint s_22_7 -> i64
        let s_22_8: i64 = (s_22_7 as i64);
        // C s_22_9: const #64s : i64
        let s_22_9: i64 = 64;
        // D s_22_10: read-var t:i
        let s_22_10: i128 = fn_state.t;
        // D s_22_11: call X_read(s_22_10, s_22_9)
        let s_22_11: Bits = X_read(state, tracer, s_22_10, s_22_9);
        // D s_22_12: cast reint s_22_11 -> u64
        let s_22_12: u64 = (s_22_11.value() as u64);
        // C s_22_13: cast zx s_22_8 -> i
        let s_22_13: i128 = (i128::try_from(s_22_8).unwrap());
        // D s_22_14: call NVMem_set(s_22_13, s_22_12)
        let s_22_14: () = NVMem_set(state, tracer, s_22_13, s_22_12);
        // N s_22_15: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #102552u : u32
        let s_23_0: u32 = 102552;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_HCR_EL2_Type_NV2(s_23_1)
        let s_23_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_23_1);
        // C s_23_3: const #102552u : u32
        let s_23_3: u32 = 102552;
        // D s_23_4: read-reg s_23_3:struct
        let s_23_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_3 as isize);
            tracer.read_register(s_23_3 as isize, value);
            value
        };
        // D s_23_5: call _get_HCR_EL2_Type_NV(s_23_4)
        let s_23_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_23_4);
        // D s_23_6: cast zx s_23_2 -> bv
        let s_23_6: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_7: cast zx s_23_5 -> bv
        let s_23_7: Bits = Bits::new(s_23_5 as u128, 1u16);
        // D s_23_8: cast reint s_23_6 -> u128
        let s_23_8: u128 = (s_23_6.value() as u128);
        // D s_23_9: size-of s_23_6
        let s_23_9: u16 = s_23_6.length();
        // D s_23_10: cast reint s_23_7 -> u128
        let s_23_10: u128 = (s_23_7.value() as u128);
        // D s_23_11: size-of s_23_7
        let s_23_11: u16 = s_23_7.length();
        // D s_23_12: lsl s_23_8 s_23_11
        let s_23_12: u128 = s_23_8 << s_23_11;
        // D s_23_13: or s_23_12 s_23_10
        let s_23_13: u128 = ((s_23_12) | (s_23_10));
        // D s_23_14: add s_23_9 s_23_11
        let s_23_14: u16 = (s_23_9 + s_23_11);
        // D s_23_15: create-bits s_23_13 s_23_14
        let s_23_15: Bits = Bits::new(s_23_13, s_23_14);
        // D s_23_16: cast reint s_23_15 -> u8
        let s_23_16: u8 = (s_23_15.value() as u8);
        // D s_23_17: cast zx s_23_16 -> bv
        let s_23_17: Bits = Bits::new(s_23_16 as u128, 2u16);
        // C s_23_18: const #3u : u8
        let s_23_18: u8 = 3;
        // C s_23_19: cast zx s_23_18 -> bv
        let s_23_19: Bits = Bits::new(s_23_18 as u128, 2u16);
        // D s_23_20: cmp-eq s_23_17 s_23_19
        let s_23_20: bool = ((s_23_17) == (s_23_19));
        // D s_23_21: write-var gs#86231 <= s_23_20
        fn_state.gs_86231 = s_23_20;
        // N s_23_22: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
}
