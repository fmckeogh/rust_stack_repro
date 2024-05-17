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
use X_set::*;
use AArch64_RandomTag::*;
use SP_set::*;
use IsOnes::*;
use AArch64_AllocationTagAccessIsEnabled::*;
use ChooseRandomNonExcludedTag::*;
use SP_read::*;
use X_read::*;
use u_get_GCR_EL1_Type_Exclude::*;
use AArch64_AddressWithAllocationTag::*;
use u_get_RGSR_EL1_Type_TAG::*;
use AArch64_ChooseNonExcludedTag::*;
use u_get_GCR_EL1_Type_RRND::*;
use common::*;
pub fn execute_aarch64_instrs_integer_tags_mcinsertrandomtag<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: u64,
        rtag: u8,
        result: u64,
        exclude: u16,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #31s : i
        let s_0_0: i128 = 31;
        // D s_0_1: read-var n:i64
        let s_0_1: i64 = fn_state.n;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b12 b1
        if s_0_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call X_read(s_1_2, s_1_0)
        let s_1_3: Bits = X_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // D s_1_5: write-var operand <= s_1_4
        fn_state.operand = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var m:i64
        let s_2_1: i64 = fn_state.m;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // C s_2_5: const #0s : i
        let s_2_5: i128 = 0;
        // D s_2_6: cast zx s_2_4 -> bv
        let s_2_6: Bits = Bits::new(s_2_4 as u128, 64u16);
        // C s_2_7: const #1s : i64
        let s_2_7: i64 = 1;
        // C s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // C s_2_9: const #15s : i
        let s_2_9: i128 = 15;
        // C s_2_10: add s_2_9 s_2_8
        let s_2_10: i128 = (s_2_9 + s_2_8);
        // D s_2_11: bit-extract s_2_6 s_2_5 s_2_10
        let s_2_11: Bits = (Bits::new(
            ((s_2_6) >> (s_2_5)).value(),
            u16::try_from(s_2_10).unwrap(),
        ));
        // D s_2_12: cast reint s_2_11 -> u16
        let s_2_12: u16 = (s_2_11.value() as u16);
        // C s_2_13: const #11480u : u32
        let s_2_13: u32 = 11480;
        // D s_2_14: read-reg s_2_13:struct
        let s_2_14: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_13 as isize);
            tracer.read_register(s_2_13 as isize, value);
            value
        };
        // D s_2_15: call _get_GCR_EL1_Type_Exclude(s_2_14)
        let s_2_15: u16 = u_get_GCR_EL1_Type_Exclude(state, tracer, s_2_14);
        // D s_2_16: cast zx s_2_12 -> bv
        let s_2_16: Bits = Bits::new(s_2_12 as u128, 16u16);
        // D s_2_17: cast zx s_2_15 -> bv
        let s_2_17: Bits = Bits::new(s_2_15 as u128, 16u16);
        // D s_2_18: or s_2_16 s_2_17
        let s_2_18: Bits = ((s_2_16) | (s_2_17));
        // D s_2_19: cast reint s_2_18 -> u16
        let s_2_19: u16 = (s_2_18.value() as u16);
        // D s_2_20: write-var exclude <= s_2_19
        fn_state.exclude = s_2_19;
        // C s_2_21: const #16975u : u32
        let s_2_21: u32 = 16975;
        // D s_2_22: read-reg s_2_21:u8
        let s_2_22: u8 = {
            let value = state.read_register::<u8>(s_2_21 as isize);
            tracer.read_register(s_2_21 as isize, value);
            value
        };
        // D s_2_23: call AArch64_AllocationTagAccessIsEnabled(s_2_22)
        let s_2_23: bool = AArch64_AllocationTagAccessIsEnabled(state, tracer, s_2_22);
        // N s_2_24: branch s_2_23 b7 b3
        if s_2_23 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: u8 = 0;
        // D s_3_1: write-var rtag <= s_3_0
        fn_state.rtag = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var operand:u64
        let s_4_0: u64 = fn_state.operand;
        // D s_4_1: read-var rtag:u8
        let s_4_1: u8 = fn_state.rtag;
        // D s_4_2: call AArch64_AddressWithAllocationTag(s_4_0, s_4_1)
        let s_4_2: u64 = AArch64_AddressWithAllocationTag(state, tracer, s_4_0, s_4_1);
        // D s_4_3: write-var result <= s_4_2
        fn_state.result = s_4_2;
        // C s_4_4: const #31s : i
        let s_4_4: i128 = 31;
        // D s_4_5: read-var d:i64
        let s_4_5: i64 = fn_state.d;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: cmp-eq s_4_6 s_4_4
        let s_4_7: bool = ((s_4_6) == (s_4_4));
        // N s_4_8: branch s_4_7 b6 b5
        if s_4_7 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var d:i64
        let s_5_1: i64 = fn_state.d;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: read-var result:u64
        let s_5_3: u64 = fn_state.result;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 64u16);
        // D s_5_5: call X_set(s_5_2, s_5_0, s_5_4)
        let s_5_5: () = X_set(state, tracer, s_5_2, s_5_0, s_5_4);
        // N s_5_6: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var result:u64
        let s_6_0: u64 = fn_state.result;
        // D s_6_1: call SP_set(s_6_0)
        let s_6_1: () = SP_set(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #11480u : u32
        let s_7_0: u32 = 11480;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_GCR_EL1_Type_RRND(s_7_1)
        let s_7_2: bool = u_get_GCR_EL1_Type_RRND(state, tracer, s_7_1);
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // C s_7_4: const #1u : u8
        let s_7_4: bool = true;
        // C s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // D s_7_6: cmp-eq s_7_3 s_7_5
        let s_7_6: bool = ((s_7_3) == (s_7_5));
        // N s_7_7: branch s_7_6 b9 b8
        if s_7_6 {
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
        // C s_8_0: const #20096u : u32
        let s_8_0: u32 = 20096;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_RGSR_EL1_Type_TAG(s_8_1)
        let s_8_2: u8 = u_get_RGSR_EL1_Type_TAG(state, tracer, s_8_1);
        // C s_8_3: const #() : ()
        let s_8_3: () = ();
        // S s_8_4: call AArch64_RandomTag(s_8_3)
        let s_8_4: u8 = AArch64_RandomTag(state, tracer, s_8_3);
        // D s_8_5: read-var exclude:u16
        let s_8_5: u16 = fn_state.exclude;
        // D s_8_6: call AArch64_ChooseNonExcludedTag(s_8_2, s_8_4, s_8_5)
        let s_8_6: u8 = AArch64_ChooseNonExcludedTag(state, tracer, s_8_2, s_8_4, s_8_5);
        // D s_8_7: write-var rtag <= s_8_6
        fn_state.rtag = s_8_6;
        // C s_8_8: const #20096u : u32
        let s_8_8: u32 = 20096;
        // D s_8_9: read-reg s_8_8:struct
        let s_8_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_8 as isize);
            tracer.read_register(s_8_8 as isize, value);
            value
        };
        // C s_8_10: const #20096u : u32
        let s_8_10: u32 = 20096;
        // N s_8_11: write-reg s_8_10 <= s_8_9
        let s_8_11: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_10 as isize, s_8_9);
            tracer.write_register(s_8_10 as isize, s_8_9);
        };
        // N s_8_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var exclude:u16
        let s_9_0: u16 = fn_state.exclude;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 16u16);
        // D s_9_2: call IsOnes(s_9_1)
        let s_9_2: bool = IsOnes(state, tracer, s_9_1);
        // N s_9_3: branch s_9_2 b11 b10
        if s_9_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var exclude:u16
        let s_10_0: u16 = fn_state.exclude;
        // D s_10_1: call ChooseRandomNonExcludedTag(s_10_0)
        let s_10_1: u8 = ChooseRandomNonExcludedTag(state, tracer, s_10_0);
        // D s_10_2: write-var rtag <= s_10_1
        fn_state.rtag = s_10_1;
        // N s_10_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: u8 = 0;
        // D s_11_1: write-var rtag <= s_11_0
        fn_state.rtag = s_11_0;
        // N s_11_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call SP_read(s_12_0)
        let s_12_1: u64 = SP_read(state, tracer, s_12_0);
        // D s_12_2: write-var operand <= s_12_1
        fn_state.operand = s_12_1;
        // N s_12_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
