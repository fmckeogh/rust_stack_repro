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
use num_of_Feature::*;
use InitFeatureImpl::*;
use neq_int::*;
use InitVariantImplemented::*;
use u_get_RMR_EL3_Type_AA64::*;
use u_get_ID_AA64PFR0_EL1_Type_EL1::*;
use integer_subrange::*;
use u_get_ID_AA64PFR0_EL1_Type_EL2::*;
use AArch64_TakeReset::*;
use u_get_ID_AA64PFR0_EL1_Type_EL3::*;
use SetResetVector::*;
use HighestEL::*;
use AArch32_TakeReset::*;
use integer_access::*;
use common::*;
pub fn TakeReset<T: Tracer>(state: &mut State, tracer: &T, cold: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_330932: bool,
        gs_330933: bool,
        ga_370163: u8,
        cold: bool,
    }
    let fn_state = FunctionState {
        cold,
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
        // S s_0_1: call HighestEL(s_0_0)
        let s_0_1: u8 = HighestEL(state, tracer, s_0_0);
        // S s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #440u : u32
        let s_0_3: u32 = 440;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b36 b1
        if s_0_6 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HighestEL(s_1_0)
        let s_1_1: u8 = HighestEL(state, tracer, s_1_0);
        // S s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #432u : u32
        let s_1_3: u32 = 432;
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
        // N s_1_7: branch s_1_6 b35 b2
        if s_1_6 {
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
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HighestEL(s_2_0)
        let s_2_1: u8 = HighestEL(state, tracer, s_2_0);
        // S s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #424u : u32
        let s_2_3: u32 = 424;
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
        // D s_2_7: write-var gs#330932 <= s_2_6
        fn_state.gs_330932 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#330932:u8
        let s_3_0: bool = fn_state.gs_330932;
        // D s_3_1: write-var gs#330933 <= s_3_0
        fn_state.gs_330933 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#330933:u8
        let s_4_0: bool = fn_state.gs_330933;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #0u : u8
        let s_4_2: bool = false;
        // C s_4_3: const #102312u : u32
        let s_4_3: u32 = 102312;
        // N s_4_4: write-reg s_4_3 <= s_4_2
        let s_4_4: () = {
            state.write_register::<bool>(s_4_3 as isize, s_4_2);
            tracer.write_register(s_4_3 as isize, s_4_2);
        };
        // C s_4_5: const #0u : u8
        let s_4_5: bool = false;
        // C s_4_6: const #23288u : u32
        let s_4_6: u32 = 23288;
        // N s_4_7: write-reg s_4_6 <= s_4_5
        let s_4_7: () = {
            state.write_register::<bool>(s_4_6 as isize, s_4_5);
            tracer.write_register(s_4_6 as isize, s_4_5);
        };
        // C s_4_8: const #() : ()
        let s_4_8: () = ();
        // S s_4_9: call InitVariantImplemented(s_4_8)
        let s_4_9: () = InitVariantImplemented(state, tracer, s_4_8);
        // C s_4_10: const #() : ()
        let s_4_10: () = ();
        // S s_4_11: call InitFeatureImpl(s_4_10)
        let s_4_11: () = InitFeatureImpl(state, tracer, s_4_10);
        // D s_4_12: read-var cold:u8
        let s_4_12: bool = fn_state.cold;
        // N s_4_13: branch s_4_12 b34 b5
        if s_4_12 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #101912u : u32
        let s_6_0: u32 = 101912;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_RMR_EL3_Type_AA64(s_6_1)
        let s_6_2: bool = u_get_RMR_EL3_Type_AA64(state, tracer, s_6_1);
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #1u : u8
        let s_6_4: bool = true;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b33 b7
        if s_6_6 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HighestEL(s_7_0)
        let s_7_1: u8 = HighestEL(state, tracer, s_7_0);
        // D s_7_2: write-var ga#370163 <= s_7_1
        fn_state.ga_370163 = s_7_1;
        // D s_7_3: read-var ga#370163:u8
        let s_7_3: u8 = fn_state.ga_370163;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // C s_7_5: const #424u : u32
        let s_7_5: u32 = 424;
        // D s_7_6: read-reg s_7_5:u8
        let s_7_6: u8 = {
            let value = state.read_register::<u8>(s_7_5 as isize);
            tracer.read_register(s_7_5 as isize, value);
            value
        };
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 2u16);
        // D s_7_8: cmp-eq s_7_4 s_7_7
        let s_7_8: bool = ((s_7_4) == (s_7_7));
        // D s_7_9: not s_7_8
        let s_7_9: bool = !s_7_8;
        // N s_7_10: branch s_7_9 b28 b8
        if s_7_9 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #101824u : u32
        let s_8_0: u32 = 101824;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_ID_AA64PFR0_EL1_Type_EL3(s_8_1)
        let s_8_2: u8 = u_get_ID_AA64PFR0_EL1_Type_EL3(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 4u16);
        // C s_8_4: const #2u : u8
        let s_8_4: u8 = 2;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 4u16);
        // D s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // N s_8_7: assert s_8_6
        let s_8_7: () = assert!(s_8_6);
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // C s_9_1: const #10368u : u32
        let s_9_1: u32 = 10368;
        // N s_9_2: write-reg s_9_1 <= s_9_0
        let s_9_2: () = {
            state.write_register::<bool>(s_9_1 as isize, s_9_0);
            tracer.write_register(s_9_1 as isize, s_9_0);
        };
        // C s_9_3: const #4u : u32
        let s_9_3: u32 = 4;
        // S s_9_4: call num_of_Feature(s_9_3)
        let s_9_4: i64 = num_of_Feature(state, tracer, s_9_3);
        // C s_9_5: const #102872u : u32
        let s_9_5: u32 = 102872;
        // D s_9_6: read-reg s_9_5:[u8; 259]
        let s_9_6: [bool; 259usize] = {
            let value = state.read_register::<[bool; 259usize]>(s_9_5 as isize);
            tracer.read_register(s_9_5 as isize, value);
            value
        };
        // S s_9_7: cast zx s_9_4 -> i
        let s_9_7: i128 = (i128::try_from(s_9_4).unwrap());
        // C s_9_8: const #0u : u8
        let s_9_8: bool = false;
        // D s_9_9: mutate-element s_9_6[s_9_7] <= s_9_8
        let s_9_9: [bool; 259usize] = {
            let mut local = s_9_6.clone();
            local[(s_9_7) as usize] = s_9_8;
            local
        };
        // D s_9_10: cast cvt s_9_9 -> [u8; 0]
        let s_9_10: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_9_9);
        // D s_9_11: cast cvt s_9_10 -> [u8; 259]
        let s_9_11: [bool; 259usize] = {
            let mut buf = [Default::default(); 259usize];
            buf.copy_from_slice(&s_9_10);
            buf
        };
        // C s_9_12: const #102872u : u32
        let s_9_12: u32 = 102872;
        // N s_9_13: write-reg s_9_12 <= s_9_11
        let s_9_13: () = {
            state.write_register::<[bool; 259usize]>(s_9_12 as isize, s_9_11);
            tracer.write_register(s_9_12 as isize, s_9_11);
        };
        // C s_9_14: const #5u : u32
        let s_9_14: u32 = 5;
        // S s_9_15: call num_of_Feature(s_9_14)
        let s_9_15: i64 = num_of_Feature(state, tracer, s_9_14);
        // C s_9_16: const #102872u : u32
        let s_9_16: u32 = 102872;
        // D s_9_17: read-reg s_9_16:[u8; 259]
        let s_9_17: [bool; 259usize] = {
            let value = state.read_register::<[bool; 259usize]>(s_9_16 as isize);
            tracer.read_register(s_9_16 as isize, value);
            value
        };
        // S s_9_18: cast zx s_9_15 -> i
        let s_9_18: i128 = (i128::try_from(s_9_15).unwrap());
        // C s_9_19: const #0u : u8
        let s_9_19: bool = false;
        // D s_9_20: mutate-element s_9_17[s_9_18] <= s_9_19
        let s_9_20: [bool; 259usize] = {
            let mut local = s_9_17.clone();
            local[(s_9_18) as usize] = s_9_19;
            local
        };
        // D s_9_21: cast cvt s_9_20 -> [u8; 0]
        let s_9_21: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_9_20);
        // D s_9_22: cast cvt s_9_21 -> [u8; 259]
        let s_9_22: [bool; 259usize] = {
            let mut buf = [Default::default(); 259usize];
            buf.copy_from_slice(&s_9_21);
            buf
        };
        // C s_9_23: const #102872u : u32
        let s_9_23: u32 = 102872;
        // N s_9_24: write-reg s_9_23 <= s_9_22
        let s_9_24: () = {
            state.write_register::<[bool; 259usize]>(s_9_23 as isize, s_9_22);
            tracer.write_register(s_9_23 as isize, s_9_22);
        };
        // C s_9_25: const #6u : u32
        let s_9_25: u32 = 6;
        // S s_9_26: call num_of_Feature(s_9_25)
        let s_9_26: i64 = num_of_Feature(state, tracer, s_9_25);
        // C s_9_27: const #102872u : u32
        let s_9_27: u32 = 102872;
        // D s_9_28: read-reg s_9_27:[u8; 259]
        let s_9_28: [bool; 259usize] = {
            let value = state.read_register::<[bool; 259usize]>(s_9_27 as isize);
            tracer.read_register(s_9_27 as isize, value);
            value
        };
        // S s_9_29: cast zx s_9_26 -> i
        let s_9_29: i128 = (i128::try_from(s_9_26).unwrap());
        // C s_9_30: const #0u : u8
        let s_9_30: bool = false;
        // D s_9_31: mutate-element s_9_28[s_9_29] <= s_9_30
        let s_9_31: [bool; 259usize] = {
            let mut local = s_9_28.clone();
            local[(s_9_29) as usize] = s_9_30;
            local
        };
        // D s_9_32: cast cvt s_9_31 -> [u8; 0]
        let s_9_32: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_9_31);
        // D s_9_33: cast cvt s_9_32 -> [u8; 259]
        let s_9_33: [bool; 259usize] = {
            let mut buf = [Default::default(); 259usize];
            buf.copy_from_slice(&s_9_32);
            buf
        };
        // C s_9_34: const #102872u : u32
        let s_9_34: u32 = 102872;
        // N s_9_35: write-reg s_9_34 <= s_9_33
        let s_9_35: () = {
            state.write_register::<[bool; 259usize]>(s_9_34 as isize, s_9_33);
            tracer.write_register(s_9_34 as isize, s_9_33);
        };
        // C s_9_36: const #7u : u32
        let s_9_36: u32 = 7;
        // S s_9_37: call num_of_Feature(s_9_36)
        let s_9_37: i64 = num_of_Feature(state, tracer, s_9_36);
        // C s_9_38: const #102872u : u32
        let s_9_38: u32 = 102872;
        // D s_9_39: read-reg s_9_38:[u8; 259]
        let s_9_39: [bool; 259usize] = {
            let value = state.read_register::<[bool; 259usize]>(s_9_38 as isize);
            tracer.read_register(s_9_38 as isize, value);
            value
        };
        // S s_9_40: cast zx s_9_37 -> i
        let s_9_40: i128 = (i128::try_from(s_9_37).unwrap());
        // C s_9_41: const #0u : u8
        let s_9_41: bool = false;
        // D s_9_42: mutate-element s_9_39[s_9_40] <= s_9_41
        let s_9_42: [bool; 259usize] = {
            let mut local = s_9_39.clone();
            local[(s_9_40) as usize] = s_9_41;
            local
        };
        // D s_9_43: cast cvt s_9_42 -> [u8; 0]
        let s_9_43: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_9_42);
        // D s_9_44: cast cvt s_9_43 -> [u8; 259]
        let s_9_44: [bool; 259usize] = {
            let mut buf = [Default::default(); 259usize];
            buf.copy_from_slice(&s_9_43);
            buf
        };
        // C s_9_45: const #102872u : u32
        let s_9_45: u32 = 102872;
        // N s_9_46: write-reg s_9_45 <= s_9_44
        let s_9_46: () = {
            state.write_register::<[bool; 259usize]>(s_9_45 as isize, s_9_44);
            tracer.write_register(s_9_45 as isize, s_9_44);
        };
        // C s_9_47: const #21880u : u32
        let s_9_47: u32 = 21880;
        // D s_9_48: read-reg s_9_47:u8
        let s_9_48: bool = {
            let value = state.read_register::<bool>(s_9_47 as isize);
            tracer.read_register(s_9_47 as isize, value);
            value
        };
        // N s_9_49: branch s_9_48 b27 b10
        if s_9_48 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #102440u : u32
        let s_10_0: u32 = 102440;
        // D s_10_1: read-reg s_10_0:u64
        let s_10_1: u64 = {
            let value = state.read_register::<u64>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call SetResetVector(s_10_1)
        let s_10_2: () = SetResetVector(state, tracer, s_10_1);
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #424u : u32
        let s_11_0: u32 = 424;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // C s_11_2: const #2u : u8
        let s_11_2: u8 = 2;
        // D s_11_3: cmp-lt s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) < (s_11_2));
        // N s_11_4: branch s_11_3 b26 b12
        if s_11_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var cold:u8
        let s_13_0: bool = fn_state.cold;
        // D s_13_1: call AArch32_TakeReset(s_13_0)
        let s_13_1: () = AArch32_TakeReset(state, tracer, s_13_0);
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1s : i
        let s_14_0: i128 = 1;
        // C s_14_1: neg s_14_0
        let s_14_1: i128 = -s_14_0;
        // C s_14_2: cast reint s_14_1 -> i64
        let s_14_2: i64 = (s_14_1 as i64);
        // C s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // C s_14_4: const #14800u : u32
        let s_14_4: u32 = 14800;
        // D s_14_5: read-reg s_14_4:i
        let s_14_5: i128 = {
            let value = state.read_register::<i128>(s_14_4 as isize);
            tracer.read_register(s_14_4 as isize, value);
            value
        };
        // D s_14_6: call neq_int(s_14_5, s_14_3)
        let s_14_6: bool = neq_int(state, tracer, s_14_5, s_14_3);
        // N s_14_7: branch s_14_6 b25 b15
        if s_14_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1s : i
        let s_16_0: i128 = 1;
        // C s_16_1: neg s_16_0
        let s_16_1: i128 = -s_16_0;
        // C s_16_2: cast reint s_16_1 -> i64
        let s_16_2: i64 = (s_16_1 as i64);
        // C s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // C s_16_4: const #15656u : u32
        let s_16_4: u32 = 15656;
        // D s_16_5: read-reg s_16_4:i
        let s_16_5: i128 = {
            let value = state.read_register::<i128>(s_16_4 as isize);
            tracer.read_register(s_16_4 as isize, value);
            value
        };
        // D s_16_6: call neq_int(s_16_5, s_16_3)
        let s_16_6: bool = neq_int(state, tracer, s_16_5, s_16_3);
        // N s_16_7: branch s_16_6 b24 b17
        if s_16_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1s : i
        let s_18_0: i128 = 1;
        // C s_18_1: neg s_18_0
        let s_18_1: i128 = -s_18_0;
        // C s_18_2: cast reint s_18_1 -> i64
        let s_18_2: i64 = (s_18_1 as i64);
        // C s_18_3: cast zx s_18_2 -> i
        let s_18_3: i128 = (i128::try_from(s_18_2).unwrap());
        // C s_18_4: const #90352u : u32
        let s_18_4: u32 = 90352;
        // D s_18_5: read-reg s_18_4:i
        let s_18_5: i128 = {
            let value = state.read_register::<i128>(s_18_4 as isize);
            tracer.read_register(s_18_4 as isize, value);
            value
        };
        // D s_18_6: call neq_int(s_18_5, s_18_3)
        let s_18_6: bool = neq_int(state, tracer, s_18_5, s_18_3);
        // N s_18_7: branch s_18_6 b23 b19
        if s_18_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1s : i
        let s_20_0: i128 = 1;
        // C s_20_1: neg s_20_0
        let s_20_1: i128 = -s_20_0;
        // C s_20_2: cast reint s_20_1 -> i64
        let s_20_2: i64 = (s_20_1 as i64);
        // C s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // C s_20_4: const #90928u : u32
        let s_20_4: u32 = 90928;
        // D s_20_5: read-reg s_20_4:i
        let s_20_5: i128 = {
            let value = state.read_register::<i128>(s_20_4 as isize);
            tracer.read_register(s_20_4 as isize, value);
            value
        };
        // D s_20_6: call neq_int(s_20_5, s_20_3)
        let s_20_6: bool = neq_int(state, tracer, s_20_5, s_20_3);
        // N s_20_7: branch s_20_6 b22 b21
        if s_20_6 {
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
        // N s_21_0: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #3s : i
        let s_22_0: i128 = 3;
        // C s_22_1: const #0s : i
        let s_22_1: i128 = 0;
        // C s_22_2: const #90928u : u32
        let s_22_2: u32 = 90928;
        // D s_22_3: read-reg s_22_2:i
        let s_22_3: i128 = {
            let value = state.read_register::<i128>(s_22_2 as isize);
            tracer.read_register(s_22_2 as isize, value);
            value
        };
        // D s_22_4: call integer_subrange(s_22_3, s_22_0, s_22_1)
        let s_22_4: Bits = integer_subrange(state, tracer, s_22_3, s_22_0, s_22_1);
        // C s_22_5: const #90960u : u32
        let s_22_5: u32 = 90960;
        // D s_22_6: read-reg s_22_5:struct
        let s_22_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_5 as isize);
            tracer.read_register(s_22_5 as isize, value);
            value
        };
        // C s_22_7: const #90960u : u32
        let s_22_7: u32 = 90960;
        // N s_22_8: write-reg s_22_7 <= s_22_6
        let s_22_8: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_22_7 as isize, s_22_6);
            tracer.write_register(s_22_7 as isize, s_22_6);
        };
        // N s_22_9: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0s : i
        let s_23_0: i128 = 0;
        // C s_23_1: const #90352u : u32
        let s_23_1: u32 = 90352;
        // D s_23_2: read-reg s_23_1:i
        let s_23_2: i128 = {
            let value = state.read_register::<i128>(s_23_1 as isize);
            tracer.read_register(s_23_1 as isize, value);
            value
        };
        // D s_23_3: call integer_access(s_23_2, s_23_0)
        let s_23_3: bool = integer_access(state, tracer, s_23_2, s_23_0);
        // C s_23_4: const #16840u : u32
        let s_23_4: u32 = 16840;
        // D s_23_5: read-reg s_23_4:struct
        let s_23_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_4 as isize);
            tracer.read_register(s_23_4 as isize, value);
            value
        };
        // C s_23_6: const #16840u : u32
        let s_23_6: u32 = 16840;
        // N s_23_7: write-reg s_23_6 <= s_23_5
        let s_23_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_23_6 as isize, s_23_5);
            tracer.write_register(s_23_6 as isize, s_23_5);
        };
        // N s_23_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0s : i
        let s_24_0: i128 = 0;
        // C s_24_1: const #15656u : u32
        let s_24_1: u32 = 15656;
        // D s_24_2: read-reg s_24_1:i
        let s_24_2: i128 = {
            let value = state.read_register::<i128>(s_24_1 as isize);
            tracer.read_register(s_24_1 as isize, value);
            value
        };
        // D s_24_3: call integer_access(s_24_2, s_24_0)
        let s_24_3: bool = integer_access(state, tracer, s_24_2, s_24_0);
        // C s_24_4: const #16840u : u32
        let s_24_4: u32 = 16840;
        // D s_24_5: read-reg s_24_4:struct
        let s_24_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_4 as isize);
            tracer.read_register(s_24_4 as isize, value);
            value
        };
        // C s_24_6: const #16840u : u32
        let s_24_6: u32 = 16840;
        // N s_24_7: write-reg s_24_6 <= s_24_5
        let s_24_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_24_6 as isize, s_24_5);
            tracer.write_register(s_24_6 as isize, s_24_5);
        };
        // N s_24_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #3s : i
        let s_25_0: i128 = 3;
        // C s_25_1: const #0s : i
        let s_25_1: i128 = 0;
        // C s_25_2: const #14800u : u32
        let s_25_2: u32 = 14800;
        // D s_25_3: read-reg s_25_2:i
        let s_25_3: i128 = {
            let value = state.read_register::<i128>(s_25_2 as isize);
            tracer.read_register(s_25_2 as isize, value);
            value
        };
        // D s_25_4: call integer_subrange(s_25_3, s_25_0, s_25_1)
        let s_25_4: Bits = integer_subrange(state, tracer, s_25_3, s_25_0, s_25_1);
        // C s_25_5: const #11064u : u32
        let s_25_5: u32 = 11064;
        // D s_25_6: read-reg s_25_5:struct
        let s_25_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_5 as isize);
            tracer.read_register(s_25_5 as isize, value);
            value
        };
        // C s_25_7: const #11064u : u32
        let s_25_7: u32 = 11064;
        // N s_25_8: write-reg s_25_7 <= s_25_6
        let s_25_8: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_25_7 as isize, s_25_6);
            tracer.write_register(s_25_7 as isize, s_25_6);
        };
        // N s_25_9: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #20920u : u32
        let s_26_0: u32 = 20920;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // C s_26_2: const #20920u : u32
        let s_26_2: u32 = 20920;
        // N s_26_3: write-reg s_26_2 <= s_26_1
        let s_26_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_26_2 as isize, s_26_1);
            tracer.write_register(s_26_2 as isize, s_26_1);
        };
        // N s_26_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #64s : i
        let s_27_0: i128 = 64;
        // C s_27_1: const #0u : u8
        let s_27_1: u8 = 0;
        // C s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 4u16);
        // D s_27_3: bits-cast zx s_27_2 -> bv length s_27_0
        let s_27_3: Bits = s_27_2.zero_extend(s_27_0);
        // D s_27_4: cast reint s_27_3 -> u64
        let s_27_4: u64 = (s_27_3.value() as u64);
        // D s_27_5: call SetResetVector(s_27_4)
        let s_27_5: () = SetResetVector(state, tracer, s_27_4);
        // N s_27_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var ga#370163:u8
        let s_28_0: u8 = fn_state.ga_370163;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 2u16);
        // C s_28_2: const #432u : u32
        let s_28_2: u32 = 432;
        // D s_28_3: read-reg s_28_2:u8
        let s_28_3: u8 = {
            let value = state.read_register::<u8>(s_28_2 as isize);
            tracer.read_register(s_28_2 as isize, value);
            value
        };
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 2u16);
        // D s_28_5: cmp-eq s_28_1 s_28_4
        let s_28_5: bool = ((s_28_1) == (s_28_4));
        // D s_28_6: not s_28_5
        let s_28_6: bool = !s_28_5;
        // N s_28_7: branch s_28_6 b30 b29
        if s_28_6 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #101824u : u32
        let s_29_0: u32 = 101824;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_ID_AA64PFR0_EL1_Type_EL2(s_29_1)
        let s_29_2: u8 = u_get_ID_AA64PFR0_EL1_Type_EL2(state, tracer, s_29_1);
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 4u16);
        // C s_29_4: const #2u : u8
        let s_29_4: u8 = 2;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 4u16);
        // D s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // N s_29_7: assert s_29_6
        let s_29_7: () = assert!(s_29_6);
        // N s_29_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ga#370163:u8
        let s_30_0: u8 = fn_state.ga_370163;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 2u16);
        // C s_30_2: const #440u : u32
        let s_30_2: u32 = 440;
        // D s_30_3: read-reg s_30_2:u8
        let s_30_3: u8 = {
            let value = state.read_register::<u8>(s_30_2 as isize);
            tracer.read_register(s_30_2 as isize, value);
            value
        };
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 2u16);
        // D s_30_5: cmp-eq s_30_1 s_30_4
        let s_30_5: bool = ((s_30_1) == (s_30_4));
        // D s_30_6: not s_30_5
        let s_30_6: bool = !s_30_5;
        // N s_30_7: branch s_30_6 b32 b31
        if s_30_6 {
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
        // C s_31_0: const #101824u : u32
        let s_31_0: u32 = 101824;
        // D s_31_1: read-reg s_31_0:struct
        let s_31_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call _get_ID_AA64PFR0_EL1_Type_EL1(s_31_1)
        let s_31_2: u8 = u_get_ID_AA64PFR0_EL1_Type_EL1(state, tracer, s_31_1);
        // D s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 4u16);
        // C s_31_4: const #2u : u8
        let s_31_4: u8 = 2;
        // C s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 4u16);
        // D s_31_6: cmp-eq s_31_3 s_31_5
        let s_31_6: bool = ((s_31_3) == (s_31_5));
        // N s_31_7: assert s_31_6
        let s_31_7: () = assert!(s_31_6);
        // N s_31_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // C s_33_1: const #10368u : u32
        let s_33_1: u32 = 10368;
        // N s_33_2: write-reg s_33_1 <= s_33_0
        let s_33_2: () = {
            state.write_register::<bool>(s_33_1 as isize, s_33_0);
            tracer.write_register(s_33_1 as isize, s_33_0);
        };
        // C s_33_3: const #102440u : u32
        let s_33_3: u32 = 102440;
        // D s_33_4: read-reg s_33_3:u64
        let s_33_4: u64 = {
            let value = state.read_register::<u64>(s_33_3 as isize);
            tracer.read_register(s_33_3 as isize, value);
            value
        };
        // D s_33_5: call SetResetVector(s_33_4)
        let s_33_5: () = SetResetVector(state, tracer, s_33_4);
        // D s_33_6: read-var cold:u8
        let s_33_6: bool = fn_state.cold;
        // D s_33_7: call AArch64_TakeReset(s_33_6)
        let s_33_7: () = AArch64_TakeReset(state, tracer, s_33_6);
        // N s_33_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #101824u : u32
        let s_34_0: u32 = 101824;
        // D s_34_1: read-reg s_34_0:struct
        let s_34_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // C s_34_2: const #101824u : u32
        let s_34_2: u32 = 101824;
        // N s_34_3: write-reg s_34_2 <= s_34_1
        let s_34_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_34_2 as isize, s_34_1);
            tracer.write_register(s_34_2 as isize, s_34_1);
        };
        // C s_34_4: const #101824u : u32
        let s_34_4: u32 = 101824;
        // D s_34_5: read-reg s_34_4:struct
        let s_34_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_4 as isize);
            tracer.read_register(s_34_4 as isize, value);
            value
        };
        // C s_34_6: const #101824u : u32
        let s_34_6: u32 = 101824;
        // N s_34_7: write-reg s_34_6 <= s_34_5
        let s_34_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_34_6 as isize, s_34_5);
            tracer.write_register(s_34_6 as isize, s_34_5);
        };
        // C s_34_8: const #101824u : u32
        let s_34_8: u32 = 101824;
        // D s_34_9: read-reg s_34_8:struct
        let s_34_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_8 as isize);
            tracer.read_register(s_34_8 as isize, value);
            value
        };
        // C s_34_10: const #101824u : u32
        let s_34_10: u32 = 101824;
        // N s_34_11: write-reg s_34_10 <= s_34_9
        let s_34_11: () = {
            state
                .write_register::<ProductType5c790c8ef59cc8b2>(s_34_10 as isize, s_34_9);
            tracer.write_register(s_34_10 as isize, s_34_9);
        };
        // C s_34_12: const #101824u : u32
        let s_34_12: u32 = 101824;
        // D s_34_13: read-reg s_34_12:struct
        let s_34_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_12 as isize);
            tracer.read_register(s_34_12 as isize, value);
            value
        };
        // C s_34_14: const #101824u : u32
        let s_34_14: u32 = 101824;
        // N s_34_15: write-reg s_34_14 <= s_34_13
        let s_34_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_34_14 as isize, s_34_13);
            tracer.write_register(s_34_14 as isize, s_34_13);
        };
        // C s_34_16: const #10128u : u32
        let s_34_16: u32 = 10128;
        // D s_34_17: read-reg s_34_16:struct
        let s_34_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_16 as isize);
            tracer.read_register(s_34_16 as isize, value);
            value
        };
        // C s_34_18: const #10128u : u32
        let s_34_18: u32 = 10128;
        // N s_34_19: write-reg s_34_18 <= s_34_17
        let s_34_19: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_34_18 as isize, s_34_17);
            tracer.write_register(s_34_18 as isize, s_34_17);
        };
        // C s_34_20: const #101912u : u32
        let s_34_20: u32 = 101912;
        // D s_34_21: read-reg s_34_20:struct
        let s_34_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_20 as isize);
            tracer.read_register(s_34_20 as isize, value);
            value
        };
        // C s_34_22: const #101912u : u32
        let s_34_22: u32 = 101912;
        // N s_34_23: write-reg s_34_22 <= s_34_21
        let s_34_23: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_34_22 as isize, s_34_21);
            tracer.write_register(s_34_22 as isize, s_34_21);
        };
        // N s_34_24: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#330932 <= s_35_0
        fn_state.gs_330932 = s_35_0;
        // N s_35_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#330933 <= s_36_0
        fn_state.gs_330933 = s_36_0;
        // N s_36_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
