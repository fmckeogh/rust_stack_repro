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
use Mk_MVBAR_Type::*;
use HaveAArch64::*;
use Mk_RVBAR_EL2_Type::*;
use HighestEL::*;
use Mk_RVBAR_EL1_Type::*;
use Mk_RVBAR_EL3_Type::*;
use common::*;
pub fn SetResetVector<T: Tracer>(state: &mut State, tracer: &T, value_name: u64) -> () {
    #[derive(Default)]
    struct FunctionState {
        value_name: u64,
    }
    let fn_state = FunctionState {
        value_name,
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
        // S s_0_1: call HaveAArch64(s_0_0)
        let s_0_1: bool = HaveAArch64(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b6 b1
        if s_0_2 {
            return block_6(state, tracer, fn_state);
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
        // C s_1_3: const #424u : u32
        let s_1_3: u32 = 424;
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
        // N s_1_7: branch s_1_6 b5 b2
        if s_1_6 {
            return block_5(state, tracer, fn_state);
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
        // C s_2_3: const #432u : u32
        let s_2_3: u32 = 432;
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
        // N s_2_7: branch s_2_6 b4 b3
        if s_2_6 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var value_name:u64
        let s_3_0: u64 = fn_state.value_name;
        // D s_3_1: call Mk_RVBAR_EL1_Type(s_3_0)
        let s_3_1: ProductType5c790c8ef59cc8b2 = Mk_RVBAR_EL1_Type(state, tracer, s_3_0);
        // C s_3_2: const #16264u : u32
        let s_3_2: u32 = 16264;
        // N s_3_3: write-reg s_3_2 <= s_3_1
        let s_3_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_3_2 as isize, s_3_1);
            tracer.write_register(s_3_2 as isize, s_3_1);
        };
        // N s_3_4: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var value_name:u64
        let s_4_0: u64 = fn_state.value_name;
        // D s_4_1: call Mk_RVBAR_EL2_Type(s_4_0)
        let s_4_1: ProductType5c790c8ef59cc8b2 = Mk_RVBAR_EL2_Type(state, tracer, s_4_0);
        // C s_4_2: const #89592u : u32
        let s_4_2: u32 = 89592;
        // N s_4_3: write-reg s_4_2 <= s_4_1
        let s_4_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_2 as isize, s_4_1);
            tracer.write_register(s_4_2 as isize, s_4_1);
        };
        // N s_4_4: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var value_name:u64
        let s_5_0: u64 = fn_state.value_name;
        // D s_5_1: call Mk_RVBAR_EL3_Type(s_5_0)
        let s_5_1: ProductType5c790c8ef59cc8b2 = Mk_RVBAR_EL3_Type(state, tracer, s_5_0);
        // C s_5_2: const #15768u : u32
        let s_5_2: u32 = 15768;
        // N s_5_3: write-reg s_5_2 <= s_5_1
        let s_5_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_2 as isize, s_5_1);
            tracer.write_register(s_5_2 as isize, s_5_1);
        };
        // N s_5_4: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #424u : u32
        let s_6_0: u32 = 424;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // D s_6_3: cmp-lt s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) < (s_6_2));
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
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
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // D s_7_1: read-var value_name:u64
        let s_7_1: u64 = fn_state.value_name;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 64u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #30s : i
        let s_7_5: i128 = 30;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_0 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u31
        let s_7_8: u32 = (s_7_7.value() as u32);
        // D s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 31u16);
        // C s_7_10: const #1u : u8
        let s_7_10: bool = true;
        // C s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 1u16);
        // D s_7_12: cast reint s_7_9 -> u128
        let s_7_12: u128 = (s_7_9.value() as u128);
        // D s_7_13: size-of s_7_9
        let s_7_13: u16 = s_7_9.length();
        // C s_7_14: cast reint s_7_11 -> u128
        let s_7_14: u128 = (s_7_11.value() as u128);
        // D s_7_15: size-of s_7_11
        let s_7_15: u16 = s_7_11.length();
        // D s_7_16: lsl s_7_12 s_7_15
        let s_7_16: u128 = s_7_12 << s_7_15;
        // D s_7_17: or s_7_16 s_7_14
        let s_7_17: u128 = ((s_7_16) | (s_7_14));
        // D s_7_18: add s_7_13 s_7_15
        let s_7_18: u16 = (s_7_13 + s_7_15);
        // D s_7_19: create-bits s_7_17 s_7_18
        let s_7_19: Bits = Bits::new(s_7_17, s_7_18);
        // D s_7_20: cast reint s_7_19 -> u32
        let s_7_20: u32 = (s_7_19.value() as u32);
        // C s_7_21: const #15032u : u32
        let s_7_21: u32 = 15032;
        // N s_7_22: write-reg s_7_21 <= s_7_20
        let s_7_22: () = {
            state.write_register::<u32>(s_7_21 as isize, s_7_20);
            tracer.write_register(s_7_21 as isize, s_7_20);
        };
        // N s_7_23: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1s : i
        let s_8_0: i128 = 1;
        // D s_8_1: read-var value_name:u64
        let s_8_1: u64 = fn_state.value_name;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 64u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #30s : i
        let s_8_5: i128 = 30;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_0 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u31
        let s_8_8: u32 = (s_8_7.value() as u32);
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 31u16);
        // C s_8_10: const #1u : u8
        let s_8_10: bool = true;
        // C s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 1u16);
        // D s_8_12: cast reint s_8_9 -> u128
        let s_8_12: u128 = (s_8_9.value() as u128);
        // D s_8_13: size-of s_8_9
        let s_8_13: u16 = s_8_9.length();
        // C s_8_14: cast reint s_8_11 -> u128
        let s_8_14: u128 = (s_8_11.value() as u128);
        // D s_8_15: size-of s_8_11
        let s_8_15: u16 = s_8_11.length();
        // D s_8_16: lsl s_8_12 s_8_15
        let s_8_16: u128 = s_8_12 << s_8_15;
        // D s_8_17: or s_8_16 s_8_14
        let s_8_17: u128 = ((s_8_16) | (s_8_14));
        // D s_8_18: add s_8_13 s_8_15
        let s_8_18: u16 = (s_8_13 + s_8_15);
        // D s_8_19: create-bits s_8_17 s_8_18
        let s_8_19: Bits = Bits::new(s_8_17, s_8_18);
        // D s_8_20: cast reint s_8_19 -> u32
        let s_8_20: u32 = (s_8_19.value() as u32);
        // D s_8_21: call Mk_MVBAR_Type(s_8_20)
        let s_8_21: ProductType700c18a878c5601b = Mk_MVBAR_Type(state, tracer, s_8_20);
        // C s_8_22: const #100208u : u32
        let s_8_22: u32 = 100208;
        // N s_8_23: write-reg s_8_22 <= s_8_21
        let s_8_23: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_8_22 as isize, s_8_21);
            tracer.write_register(s_8_22 as isize, s_8_21);
        };
        // N s_8_24: return
        return;
    }
}
